use crate::{bearer_token, deadline_response, json_error, AnswerInput, ApiResponse, ApiService};
use brain_contracts::bot_tasks::{
    BotCapability, BotIdentity, BotPlatform, BotTask, BotTaskRequest, BotTaskResponse,
    BotTaskResult, CapabilityAvailability, TaskFailure, TaskUsage, TwitchTitleInput,
    TwitchTitlePort,
};
use brain_contracts::discord_task::DiscordAnswerTask;
use brain_contracts::{AnswerProfile, Principal, PublicAnswerResponse, Query, RequestDeadline};
use brain_kernel::AnswerKernelPort;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Instant,
};

pub(crate) type TaskConsumers = BTreeMap<(String, String), BTreeSet<BotCapability>>;

impl<K> ApiService<K>
where
    K: AnswerKernelPort,
{
    pub fn with_bot_task_consumers(
        mut self,
        consumers: BTreeMap<(String, String), BTreeSet<BotCapability>>,
    ) -> Self {
        self.bot_task_consumers = consumers;
        self
    }

    pub fn with_twitch_title(mut self, port: Arc<dyn TwitchTitlePort>) -> Self {
        self.twitch_title = Some(port);
        self
    }

    pub(crate) fn handle_task(
        &self,
        authorization: Option<&str>,
        body: &[u8],
        deadline: RequestDeadline,
    ) -> (ApiResponse, Option<Value>) {
        if deadline.check().is_err() {
            return (deadline_response(), None);
        }
        if body.len() > 64 * 1024 {
            return (
                json_error(413, "payload_too_large", "Request ist zu groß"),
                None,
            );
        }
        let Some(token) = bearer_token(authorization) else {
            return (
                json_error(401, "unauthorized", "Bearer Token fehlt oder ist ungültig"),
                None,
            );
        };
        let principal = match self.policy.authenticate(token) {
            Ok(principal)
                if principal.actor_id != "second-brain"
                    && !principal.scopes.contains("second_brain.internal") =>
            {
                principal
            }
            _ => {
                return (
                    json_error(401, "unauthorized", "Zugangsdaten sind ungültig"),
                    None,
                )
            }
        };
        let request = match serde_json::from_slice::<Value>(body)
            .and_then(serde_json::from_value::<BotTaskRequest>)
        {
            Ok(request) if request.validate().is_ok() => request,
            _ => return (invalid_task(), None),
        };
        let allowed = self.task_allowed(&principal, request.task.capability());
        let started = Instant::now();
        let result = match &request.task {
            BotTask::CapabilityStatus { capability } => BotTaskResult::CapabilityStatus {
                capability: *capability,
                availability: if allowed && self.capability_ready(*capability) {
                    CapabilityAvailability::Available
                } else {
                    CapabilityAvailability::Unavailable
                },
                capabilities: Vec::new(),
            },
            _ if !allowed => {
                return (
                    json_error(403, "forbidden", "Bot-Aufgabe ist nicht freigegeben"),
                    None,
                )
            }
            BotTask::DiscordAnswer { .. } => match self.discord_answer(token, &request, deadline) {
                Ok(answer) => BotTaskResult::DiscordAnswer { answer },
                Err(response) => return (response, None),
            },
            BotTask::TwitchTitle { .. } => match self.twitch_title(&request, &deadline) {
                Ok(result) => result,
                Err(response) => return (response, None),
            },
            BotTask::Conversation(task) => BotTaskResult::Failed {
                capability: task.capability(),
                failure: TaskFailure::ProviderUnavailable,
            },
        };
        let event = task_event(&request, &result, started);
        let response = BotTaskResponse::new(&request, result);
        if response.validate_for(&request).is_err() {
            return (
                json_error(502, "invalid_task_response", "Ungültige Aufgabenantwort"),
                Some(event),
            );
        }
        match serde_json::to_string(&response) {
            Ok(body) => (
                ApiResponse {
                    status: 200,
                    content_type: "application/json",
                    body,
                },
                Some(event),
            ),
            Err(_) => (
                json_error(
                    500,
                    "serialization_error",
                    "Antwort konnte nicht serialisiert werden",
                ),
                Some(event),
            ),
        }
    }

    fn task_allowed(&self, principal: &Principal, capability: BotCapability) -> bool {
        let key = (principal.actor_id.clone(), principal.channel.clone());
        principal.channel == capability.platform().channel()
            && self
                .bot_task_consumers
                .get(&key)
                .is_some_and(|capabilities| capabilities.contains(&capability))
            && match capability.platform() {
                BotPlatform::Discord => self.discord_consumers.contains(&key),
                BotPlatform::Twitch => principal.provider_egress.contains("public"),
            }
    }

    fn capability_ready(&self, capability: BotCapability) -> bool {
        match capability.platform() {
            BotPlatform::Discord => self.core_not_ready().is_none(),
            BotPlatform::Twitch => self
                .twitch_title
                .as_ref()
                .is_some_and(|port| port.available()),
        }
    }

    fn discord_answer(
        &self,
        token: &str,
        request: &BotTaskRequest,
        deadline: RequestDeadline,
    ) -> Result<PublicAnswerResponse, ApiResponse> {
        let (
            BotIdentity::Discord {
                channel_id,
                user_id,
                ..
            },
            BotTask::DiscordAnswer {
                capability,
                question,
                user_questions,
            },
        ) = (&request.identity, &request.task)
        else {
            return Err(invalid_task());
        };
        if let Some(response) = self.core_not_ready() {
            return Err(response);
        }
        let query = Query {
            answer_context: None,
            domain: None,
            request_id: request.request_id.clone(),
            conversation_id: request.request_id.clone(),
            text: question.clone(),
            requested_scopes: BTreeSet::from(["bot.public".into()]),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        };
        let response = self.answer_input(
            token,
            AnswerInput {
                query,
                user_questions: user_questions.clone(),
                local_context: !user_questions.is_empty(),
                claimed_user: Some(*user_id),
                allow_discord_reads: false,
                task: Some(DiscordAnswerTask {
                    capability: *capability,
                    channel_id: *channel_id,
                }),
            },
            deadline,
        );
        if response.status != 200 {
            return Err(response);
        }
        serde_json::from_str(&response.body).map_err(|_| {
            json_error(
                502,
                "invalid_kernel_response",
                "Ungültige öffentliche Antwort",
            )
        })
    }

    fn twitch_title(
        &self,
        request: &BotTaskRequest,
        deadline: &RequestDeadline,
    ) -> Result<BotTaskResult, ApiResponse> {
        let BotTask::TwitchTitle {
            purpose,
            prompt,
            temperature,
            max_tokens,
        } = &request.task
        else {
            return Err(invalid_task());
        };
        let failed = |failure| BotTaskResult::Failed {
            capability: BotCapability::TwitchTitle,
            failure,
        };
        let Some(port) = &self.twitch_title else {
            return Ok(failed(TaskFailure::ProviderUnavailable));
        };
        if *max_tokens > port.max_output_tokens() {
            return Err(invalid_task());
        }
        let input = TwitchTitleInput {
            purpose: *purpose,
            prompt,
            temperature: *temperature,
            max_tokens: *max_tokens,
        };
        Ok(match port.generate(&input, deadline) {
            Ok(title) => BotTaskResult::TwitchTitle {
                text: title.text,
                model: title.model,
                usage: title.usage,
            },
            Err(failure) => failed(failure),
        })
    }
}

fn invalid_task() -> ApiResponse {
    json_error(400, "invalid_request", "Bot-Aufgabe ist ungültig")
}

fn task_event(request: &BotTaskRequest, result: &BotTaskResult, started: Instant) -> Value {
    let (model, usage) = match result {
        BotTaskResult::TwitchTitle { model, usage, .. } => (Some(model.as_str()), *usage),
        BotTaskResult::Conversation { model, usage, .. } => (model.as_deref(), *usage),
        _ => (None, TaskUsage::default()),
    };
    let outcome = match result {
        BotTaskResult::DiscordAnswer { answer } => {
            serde_json::to_value(answer.status).unwrap_or(Value::Null)
        }
        BotTaskResult::TwitchTitle { .. } => json!("succeeded"),
        BotTaskResult::CapabilityStatus { availability, .. } => {
            serde_json::to_value(availability).unwrap_or(Value::Null)
        }
        BotTaskResult::Conversation { reply, .. } => json!(reply.outcome()),
        BotTaskResult::Failed { failure, .. } => json!(failure.as_str()),
    };
    let purpose = match &request.task {
        BotTask::TwitchTitle { purpose, .. } => serde_json::to_value(purpose).ok(),
        _ => None,
    };
    json!({
        "event": "bot_task",
        "request_id": brain_contracts::response_audit::diagnostic_text(&request.request_id, None),
        "kind": request.task.kind(),
        "capability": request.task.capability(),
        "purpose": purpose,
        "model": model,
        "input_tokens": usage.input_tokens,
        "output_tokens": usage.output_tokens,
        "latency_ms": u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        "outcome": outcome,
    })
}
