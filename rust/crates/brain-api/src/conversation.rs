use crate::persona::{self, DashboardInput, Finish};
use crate::task::KernelAsk;
use crate::{ApiResponse, ApiService};
use brain_contracts::bot_conversation::{
    BotReply, ChatLine, ConversationTask, DialogRole, DialogTurn, PitchOccasion, PromoPitchRequest,
    ReplySignal, SilentReason, StreamContext, MAX_REPLY_BYTES,
};
use brain_contracts::bot_tasks::{
    BotCapability, BotIdentity, BotTaskRequest, BotTaskResult, ConversationPort,
    ConversationPrompt, TaskFailure, TaskUsage,
};
use brain_contracts::discord_task::{bounded_user_questions, DiscordAnswerCapability};
use brain_contracts::{AnswerStatus, PublicAnswerResponse, RequestDeadline};
use brain_kernel::AnswerKernelPort;
use serde_json::Value;
use std::collections::BTreeSet;

enum Stop {
    Failed(TaskFailure),
    Http(ApiResponse),
}

impl From<TaskFailure> for Stop {
    fn from(failure: TaskFailure) -> Self {
        Self::Failed(failure)
    }
}

struct Run<'a> {
    port: Option<&'a dyn ConversationPort>,
    deadline: &'a RequestDeadline,
    model: Option<String>,
    usage: TaskUsage,
    kernel_final: bool,
}

fn summed(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.saturating_add(right)),
        (left, right) => left.or(right),
    }
}

impl Run<'_> {
    fn ask(&mut self, prompt: &ConversationPrompt) -> Result<String, TaskFailure> {
        let port = self.port.ok_or(TaskFailure::ProviderUnavailable)?;
        let generated = port.generate(prompt, self.deadline)?;
        self.model = Some(generated.model);
        self.usage.input_tokens = summed(self.usage.input_tokens, generated.usage.input_tokens);
        self.usage.output_tokens = summed(self.usage.output_tokens, generated.usage.output_tokens);
        Ok(generated.text)
    }

    fn judge(&mut self, prompt: &ConversationPrompt) -> Result<Value, TaskFailure> {
        let text = self.ask(prompt)?;
        persona::parse_json(&text).ok_or(TaskFailure::InvalidOutput)
    }
}

#[derive(PartialEq)]
enum Verdict {
    Yes,
    Unsure,
    No,
}

fn confident(value: &Value) -> bool {
    value
        .get("confidence")
        .and_then(Value::as_f64)
        .is_some_and(|confidence| confidence >= persona::JUDGE_MIN_CONFIDENCE)
}

fn verdict(value: &Value) -> Verdict {
    match value.get("verdict").and_then(Value::as_str) {
        Some("yes") if confident(value) => Verdict::Yes,
        Some("unsure") => Verdict::Unsure,
        _ => Verdict::No,
    }
}

fn no_occasion() -> BotReply {
    BotReply::Silent {
        reason: SilentReason::NoOccasion,
    }
}

fn insufficient() -> BotReply {
    BotReply::Silent {
        reason: SilentReason::InsufficientEvidence,
    }
}

fn twitch_finish() -> Finish<'static> {
    Finish {
        silent: SilentReason::NoOccasion,
        allowed_urls: &[],
        max_chars: persona::TWITCH_LINE_CHARS,
    }
}

fn dialog_finish(allowed_urls: &[String]) -> Finish<'_> {
    Finish {
        silent: SilentReason::InsufficientEvidence,
        allowed_urls,
        max_chars: MAX_REPLY_BYTES,
    }
}

fn user_questions(history: &[DialogTurn], question: &str) -> Vec<String> {
    let asked: Vec<String> = history
        .iter()
        .filter(|turn| turn.role == DialogRole::User)
        .map(|turn| turn.text.clone())
        .collect();
    bounded_user_questions(&asked, question)
}

fn kernel_reply(answer: PublicAnswerResponse, run: &mut Run<'_>) -> BotReply {
    run.kernel_final = true;
    if answer.text.len() > MAX_REPLY_BYTES {
        return BotReply::Silent {
            reason: SilentReason::OutputRejected,
        };
    }
    BotReply::Answered {
        text: answer.text,
        signals: Vec::new(),
        citations: answer.citations,
        artifacts: answer.artifacts,
    }
}

fn answer_is_final(status: AnswerStatus) -> Result<bool, Stop> {
    match status {
        AnswerStatus::Answered | AnswerStatus::BuildRejected => Ok(true),
        AnswerStatus::InsufficientEvidence
        | AnswerStatus::UnauthorizedEvidence
        | AnswerStatus::Unverified => Ok(false),
        AnswerStatus::Unavailable | AnswerStatus::ProviderError | AnswerStatus::BudgetExceeded => {
            Err(TaskFailure::ProviderUnavailable.into())
        }
    }
}

fn pate_signal(run: &mut Run<'_>, question: &str) -> Vec<ReplySignal> {
    let requested = persona::pate_gate(question)
        && run
            .judge(&persona::pate(question))
            .is_ok_and(|value| value.get("pate_request").and_then(Value::as_bool) == Some(true));
    if requested {
        vec![ReplySignal::PateRequest]
    } else {
        Vec::new()
    }
}

fn with_signals(reply: BotReply, added: Vec<ReplySignal>) -> BotReply {
    match reply {
        BotReply::Answered {
            text,
            signals,
            citations,
            artifacts,
        } => BotReply::Answered {
            text,
            signals: signals.into_iter().chain(added).collect(),
            citations,
            artifacts,
        },
        silent => silent,
    }
}

struct Dialogue<'a> {
    capability: DiscordAnswerCapability,
    trace: BotCapability,
    question: &'a str,
    history: &'a [DialogTurn],
    member_facts: &'a [String],
    readable: Option<BTreeSet<u64>>,
}

impl<K> ApiService<K>
where
    K: AnswerKernelPort,
{
    pub(crate) fn conversation(
        &self,
        token: &str,
        request: &BotTaskRequest,
        task: &ConversationTask,
        deadline: &RequestDeadline,
    ) -> (Result<BotTaskResult, ApiResponse>, bool) {
        let mut run = Run {
            port: self.conversation.as_deref(),
            deadline,
            model: None,
            usage: TaskUsage::default(),
            kernel_final: false,
        };
        let capability = task.capability();
        let outcome = self.reply(token, request, task, &mut run);
        let kernel_final = run.kernel_final;
        let result = match outcome {
            Ok(reply) => Ok(BotTaskResult::Conversation {
                capability,
                reply,
                model: run.model,
                usage: run.usage,
            }),
            Err(Stop::Failed(failure)) => Ok(BotTaskResult::Failed {
                capability,
                failure,
            }),
            Err(Stop::Http(response)) => Err(response),
        };
        (result, kernel_final)
    }

    fn reply(
        &self,
        token: &str,
        request: &BotTaskRequest,
        task: &ConversationTask,
        run: &mut Run<'_>,
    ) -> Result<BotReply, Stop> {
        match task {
            ConversationTask::DiscordConcierge {
                question,
                history,
                readable_channel_ids,
                member_facts,
            } => {
                let dialogue = Dialogue {
                    capability: DiscordAnswerCapability::Concierge,
                    trace: BotCapability::DiscordConcierge,
                    question,
                    history,
                    member_facts,
                    readable: (!readable_channel_ids.is_empty())
                        .then(|| readable_channel_ids.clone()),
                };
                let reply = self.dialogue(token, request, dialogue, run)?;
                let signals = match &reply {
                    BotReply::Answered { .. } => pate_signal(run, question),
                    BotReply::Silent { .. } => Vec::new(),
                };
                Ok(with_signals(reply, signals))
            }
            ConversationTask::DiscordFaq { question, history } => self.dialogue(
                token,
                request,
                Dialogue {
                    capability: DiscordAnswerCapability::Faq,
                    trace: BotCapability::DiscordFaq,
                    question,
                    history,
                    member_facts: &[],
                    readable: None,
                },
                run,
            ),
            ConversationTask::DiscordPassiveHelp {
                message,
                recent_messages,
            } => self.passive_help(token, request, message, recent_messages, run),
            _ => Ok(generated_reply(&request.request_id, task, run)?),
        }
    }

    fn dialogue(
        &self,
        token: &str,
        request: &BotTaskRequest,
        dialogue: Dialogue<'_>,
        run: &mut Run<'_>,
    ) -> Result<BotReply, Stop> {
        let answer = self.ask_kernel(
            token,
            request,
            KernelAsk {
                capability: dialogue.capability,
                trace: dialogue.trace,
                question: dialogue.question,
                user_questions: user_questions(dialogue.history, dialogue.question),
                readable: dialogue.readable,
            },
            run.deadline,
        )?;
        if answer_is_final(answer.status)? {
            return Ok(kernel_reply(answer, run));
        }
        if run.port.is_none() {
            return Ok(insufficient());
        }
        let text = run.ask(&persona::concierge(
            dialogue.member_facts,
            dialogue.history,
            dialogue.question,
        ))?;
        Ok(persona::finish(&text, &dialog_finish(&[])))
    }

    fn passive_help(
        &self,
        token: &str,
        request: &BotTaskRequest,
        message: &str,
        recent: &[ChatLine],
        run: &mut Run<'_>,
    ) -> Result<BotReply, Stop> {
        let judged = run.judge(&persona::passive_help(message, recent))?;
        let question = judged
            .get("question")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|question| !question.is_empty());
        let help = judged.get("help").and_then(Value::as_bool) == Some(true);
        let (true, Some(question)) = (help, question) else {
            return Ok(no_occasion());
        };
        let answer = self.ask_kernel(
            token,
            request,
            KernelAsk {
                capability: DiscordAnswerCapability::Faq,
                trace: BotCapability::DiscordPassiveHelp,
                question,
                user_questions: Vec::new(),
                readable: None,
            },
            run.deadline,
        )?;
        if answer_is_final(answer.status)? {
            Ok(kernel_reply(answer, run))
        } else {
            run.kernel_final = true;
            Ok(insufficient())
        }
    }

    fn ask_kernel(
        &self,
        token: &str,
        request: &BotTaskRequest,
        ask: KernelAsk<'_>,
        deadline: &RequestDeadline,
    ) -> Result<PublicAnswerResponse, Stop> {
        let BotIdentity::Discord {
            channel_id,
            user_id,
            ..
        } = &request.identity
        else {
            return Err(TaskFailure::ProviderUnavailable.into());
        };
        self.kernel_answer(
            token,
            &request.request_id,
            *channel_id,
            *user_id,
            ask,
            deadline.clone(),
        )
        .map_err(Stop::Http)
    }
}

fn generated_reply(
    seed: &str,
    task: &ConversationTask,
    run: &mut Run<'_>,
) -> Result<BotReply, TaskFailure> {
    match task {
        ConversationTask::DiscordVoiceHint { message } => {
            let text = run.ask(&persona::voice_hint(message))?;
            Ok(if text.trim().eq_ignore_ascii_case("ja") {
                BotReply::answered(persona::VOICE_REPLY)
            } else {
                no_occasion()
            })
        }
        ConversationTask::DiscordInviteLounge { .. } => {
            Ok(BotReply::answered(persona::INVITE_LOUNGE))
        }
        ConversationTask::TwitchChat {
            stream,
            author,
            message,
            recent_messages,
            persona: chat_persona,
            talkative,
        } => {
            let text = run.ask(&persona::twitch_chat(
                stream,
                author,
                message,
                recent_messages,
                *chat_persona,
                *talkative,
            ))?;
            Ok(persona::finish(&text, &twitch_finish()))
        }
        ConversationTask::TwitchPromoPitch {
            stream,
            pitch,
            recent_messages,
            examples,
            invite_url,
        } => {
            let prompt = persona::promo_pitch(stream, pitch, recent_messages, examples);
            promo_pitch(run, &prompt, pitch, invite_url.as_deref())
        }
        ConversationTask::TwitchInviteReply {
            stream,
            author,
            message,
            recent_messages,
            is_newcomer,
            invite_url,
        } => {
            let judged = run.judge(&persona::invite_judge(
                stream,
                author,
                message,
                recent_messages,
                *is_newcomer,
            ))?;
            Ok(match verdict(&judged) {
                Verdict::Yes => {
                    BotReply::answered(persona::invite_go(seed, author, invite_url.as_deref()))
                }
                Verdict::Unsure => BotReply::answered(persona::invite_confirm(seed, author)),
                Verdict::No => no_occasion(),
            })
        }
        ConversationTask::TwitchLfgReply {
            stream,
            author,
            message,
            recent_messages,
            invite_url,
        } => lfg_reply(
            run,
            seed,
            stream,
            author,
            message,
            recent_messages,
            invite_url.as_deref(),
        ),
        ConversationTask::TwitchStandardReply { reply, author, .. } => Ok(BotReply::answered(
            persona::standard_reply(*reply, seed, author),
        )),
        ConversationTask::TwitchTip {
            source_text,
            language,
            ..
        } => {
            let allowed = persona::urls_in(source_text);
            let text = run.ask(&persona::tip(source_text, *language))?;
            Ok(persona::finish(
                &text,
                &Finish {
                    allowed_urls: &allowed,
                    ..twitch_finish()
                },
            ))
        }
        ConversationTask::TwitchDashboardAssistant {
            streamer_name,
            page,
            question,
            history,
            knowledge,
            streamer_data,
            language,
        } => {
            let mut allowed = persona::urls_in(knowledge);
            allowed.extend(persona::urls_in(streamer_data));
            let text = run.ask(&persona::dashboard(&DashboardInput {
                name: streamer_name,
                page,
                knowledge,
                data: streamer_data,
                history,
                question,
                language: *language,
            }))?;
            Ok(persona::finish(&text, &dialog_finish(&allowed)))
        }
        ConversationTask::TwitchSelfExplainer {
            question,
            history,
            documents,
            language,
        } => {
            let mut allowed = persona::urls_in(documents);
            allowed.extend([persona::SELF_EXPLAINER_URL.to_owned()]);
            let text = run.ask(&persona::self_explainer(
                documents, history, question, *language,
            ))?;
            Ok(persona::finish(&text, &dialog_finish(&allowed)))
        }
        ConversationTask::TwitchAnalysisChat {
            streamer_name,
            question,
            history,
            analysis,
            language,
        } => {
            let allowed = persona::urls_in(&analysis.to_string());
            let text = run.ask(&persona::analysis(
                streamer_name,
                analysis,
                history,
                question,
                *language,
            ))?;
            Ok(persona::finish(&text, &dialog_finish(&allowed)))
        }
        ConversationTask::DiscordConcierge { .. }
        | ConversationTask::DiscordFaq { .. }
        | ConversationTask::DiscordPassiveHelp { .. } => Err(TaskFailure::ProviderUnavailable),
    }
}

fn lfg_reply(
    run: &mut Run<'_>,
    seed: &str,
    stream: &StreamContext,
    author: &str,
    message: &str,
    recent: &[ChatLine],
    invite_url: Option<&str>,
) -> Result<BotReply, TaskFailure> {
    let judged = run.judge(&persona::lfg_judge(stream, author, message, recent))?;
    Ok(if verdict(&judged) == Verdict::Yes {
        BotReply::answered(persona::lfg_go(seed, author, invite_url))
    } else {
        no_occasion()
    })
}

fn promo_pitch(
    run: &mut Run<'_>,
    prompt: &ConversationPrompt,
    pitch: &PromoPitchRequest,
    invite_url: Option<&str>,
) -> Result<BotReply, TaskFailure> {
    match pitch {
        PromoPitchRequest::Reactive { .. } => Ok(reactive_pitch(&run.judge(prompt)?)),
        PromoPitchRequest::ChannelPromo => {
            let text = run.ask(prompt)?;
            let allowed: Vec<String> = invite_url.map(str::to_owned).into_iter().collect();
            let line = match invite_url {
                Some(url) if !text.contains("<silent>") && !text.trim().is_empty() => {
                    format!("{} {url}", text.trim())
                }
                _ => text,
            };
            Ok(persona::finish(
                &line,
                &Finish {
                    allowed_urls: &allowed,
                    ..twitch_finish()
                },
            ))
        }
        PromoPitchRequest::Targeted { .. } | PromoPitchRequest::Partner { .. } => {
            let text = run.ask(prompt)?;
            Ok(persona::finish(&text, &twitch_finish()))
        }
    }
}

fn reactive_pitch(judged: &Value) -> BotReply {
    let serious = judged.get("ernst_gemeint").and_then(Value::as_bool) == Some(true);
    let occasion = judged
        .get("occasion")
        .cloned()
        .and_then(|value| serde_json::from_value::<PitchOccasion>(value).ok());
    let text = judged.get("reply").and_then(Value::as_str);
    let (true, true, Some(occasion), Some(text)) = (serious, confident(judged), occasion, text)
    else {
        return no_occasion();
    };
    let text = if occasion == PitchOccasion::Coaching {
        persona::with_coaching_cta(text)
    } else {
        text.to_owned()
    };
    with_signals(
        persona::finish(&text, &twitch_finish()),
        vec![ReplySignal::PitchOccasion { occasion }],
    )
}
