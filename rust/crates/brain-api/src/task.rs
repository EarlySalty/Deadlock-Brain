use crate::{bearer_token, deadline_response, json_error, AnswerInput, ApiResponse, ApiService};
use brain_contracts::bot_conversation::{
    BotReply, ConversationAudit, ConversationAuditPort, ConversationTask,
};
use brain_contracts::bot_tasks::{
    BotCapability, BotIdentity, BotPlatform, BotTask, BotTaskRequest, BotTaskResponse,
    BotTaskResult, CapabilityAvailability, CapabilityState, ConversationPort, TaskFailure,
    TaskUsage, TwitchTitleInput, TwitchTitlePort,
};
use brain_contracts::discord_task::{DiscordAnswerCapability, DiscordAnswerTask};
use brain_contracts::{AnswerProfile, Principal, PublicAnswerResponse, Query, RequestDeadline};
use brain_kernel::AnswerKernelPort;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Instant,
};

pub(crate) type TaskConsumers = BTreeMap<(String, String), BTreeSet<BotCapability>>;

pub(crate) struct KernelAsk<'a> {
    pub capability: DiscordAnswerCapability,
    pub trace: BotCapability,
    pub question: &'a str,
    pub user_questions: Vec<String>,
    pub readable: Option<BTreeSet<u64>>,
}

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

    pub fn with_conversation(mut self, port: Arc<dyn ConversationPort>) -> Self {
        self.conversation = Some(port);
        self
    }

    pub fn with_conversation_audit(mut self, port: Arc<dyn ConversationAuditPort>) -> Self {
        self.conversation_audit = Some(port);
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
                availability: self.availability(&principal, *capability),
                capabilities: self.capability_states(&principal, request.identity.platform()),
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
            BotTask::Conversation(task) => {
                let (result, kernel_final) = self.conversation(token, &request, task, &deadline);
                match result {
                    Ok(result) => {
                        if !kernel_final {
                            self.audit_conversation(&principal, task, &result, started);
                        }
                        result
                    }
                    Err(response) => return (response, None),
                }
            }
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

    fn availability(
        &self,
        principal: &Principal,
        capability: BotCapability,
    ) -> CapabilityAvailability {
        if self.task_allowed(principal, capability) && self.capability_ready(capability) {
            CapabilityAvailability::Available
        } else {
            CapabilityAvailability::Unavailable
        }
    }

    fn capability_states(
        &self,
        principal: &Principal,
        platform: BotPlatform,
    ) -> Vec<CapabilityState> {
        BotCapability::ALL
            .iter()
            .filter(|capability| capability.platform() == platform)
            .map(|capability| CapabilityState {
                capability: *capability,
                availability: self.availability(principal, *capability),
            })
            .collect()
    }

    fn conversation_ready(&self) -> bool {
        self.conversation
            .as_ref()
            .is_some_and(|port| port.available())
    }

    fn capability_ready(&self, capability: BotCapability) -> bool {
        let core = || self.core_not_ready().is_none();
        match capability {
            BotCapability::DiscordConcierge | BotCapability::DiscordFaq => core(),
            BotCapability::DiscordPassiveHelp => core() && self.conversation_ready(),
            BotCapability::DiscordInviteLounge | BotCapability::TwitchStandardReply => true,
            BotCapability::TwitchTitle => self
                .twitch_title
                .as_ref()
                .is_some_and(|port| port.available()),
            _ => self.conversation_ready(),
        }
    }

    fn audit_conversation(
        &self,
        principal: &Principal,
        task: &ConversationTask,
        result: &BotTaskResult,
        started: Instant,
    ) {
        let Some(port) = &self.conversation_audit else {
            return;
        };
        let (status, answer_text, reason, usage) = match result {
            BotTaskResult::Conversation { reply, usage, .. } => match reply {
                BotReply::Answered { text, .. } => ("answered", Some(text.clone()), None, *usage),
                BotReply::Silent { reason } => {
                    ("insufficient_evidence", None, Some(reason.as_str()), *usage)
                }
            },
            BotTaskResult::Failed { failure, .. } => (
                "provider_error",
                None,
                Some(failure.as_str()),
                TaskUsage::default(),
            ),
            _ => return,
        };
        port.record(ConversationAudit {
            capability: task.capability(),
            actor_id: principal.actor_id.clone(),
            channel: principal.channel.clone(),
            question: task.audit_question().map(str::to_owned),
            status,
            answer_text,
            reason,
            usage,
            latency_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        });
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
        self.kernel_answer(
            token,
            &request.request_id,
            *channel_id,
            *user_id,
            KernelAsk {
                capability: *capability,
                trace: (*capability).into(),
                question,
                user_questions: user_questions.clone(),
                readable: None,
            },
            deadline,
        )
    }

    pub(crate) fn kernel_answer(
        &self,
        token: &str,
        request_id: &str,
        channel_id: u64,
        user_id: u64,
        ask: KernelAsk<'_>,
        deadline: RequestDeadline,
    ) -> Result<PublicAnswerResponse, ApiResponse> {
        if let Some(response) = self.core_not_ready() {
            return Err(response);
        }
        let query = Query {
            answer_context: None,
            domain: None,
            request_id: request_id.to_owned(),
            conversation_id: request_id.to_owned(),
            text: ask.question.to_owned(),
            requested_scopes: BTreeSet::from(["bot.public".into()]),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        };
        let response = self.answer_input(
            token,
            AnswerInput {
                query,
                local_context: !ask.user_questions.is_empty(),
                user_questions: ask.user_questions,
                claimed_user: Some(user_id),
                allow_discord_reads: ask.readable.is_some(),
                readable_channels: ask.readable,
                trace_capability: Some(format!("bot_task:{}", ask.trace.as_str())),
                task: Some(DiscordAnswerTask {
                    capability: ask.capability,
                    channel_id,
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
