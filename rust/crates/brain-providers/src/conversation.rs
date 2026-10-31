use super::generation::{
    GenerationError, GenerationMessage, TextGenerationProvider, TextGenerationRequest,
};
use super::model_selection::selected_model;
use brain_contracts::bot_conversation::GeneratedReply;
use brain_contracts::bot_tasks::{ConversationPort, ConversationPrompt, TaskFailure, TaskUsage};
use brain_contracts::RequestDeadline;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ConversationGenerator {
    provider: TextGenerationProvider,
    model_selection_path: PathBuf,
    max_output_tokens: u32,
}

impl ConversationGenerator {
    pub fn new(
        provider: TextGenerationProvider,
        model_selection_path: PathBuf,
        max_output_tokens: u32,
    ) -> Self {
        Self {
            provider,
            model_selection_path,
            max_output_tokens,
        }
    }
}

fn task_failure(error: GenerationError) -> TaskFailure {
    match error {
        GenerationError::Unavailable => TaskFailure::ProviderUnavailable,
        GenerationError::RateLimited => TaskFailure::ProviderRateLimited,
        GenerationError::Timeout => TaskFailure::Timeout,
        GenerationError::InvalidOutput => TaskFailure::InvalidOutput,
    }
}

impl ConversationPort for ConversationGenerator {
    fn available(&self) -> bool {
        selected_model(&self.model_selection_path).is_ok()
    }

    fn generate(
        &self,
        prompt: &ConversationPrompt,
        deadline: &RequestDeadline,
    ) -> Result<GeneratedReply, TaskFailure> {
        let model = selected_model(&self.model_selection_path)
            .map_err(|_| TaskFailure::ProviderUnavailable)?;
        let messages = [
            GenerationMessage {
                role: "system",
                content: &prompt.system,
            },
            GenerationMessage {
                role: "user",
                content: &prompt.user,
            },
        ];
        let generated = self
            .provider
            .generate(
                &TextGenerationRequest {
                    model: &model,
                    messages: &messages,
                    max_tokens: prompt.max_tokens.min(self.max_output_tokens),
                    temperature: prompt.temperature,
                },
                deadline,
            )
            .map_err(task_failure)?;
        Ok(GeneratedReply {
            text: generated.content,
            model: generated.model,
            usage: TaskUsage {
                input_tokens: generated.prompt_tokens,
                output_tokens: generated.completion_tokens,
            },
        })
    }
}
