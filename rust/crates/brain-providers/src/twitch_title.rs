use super::generation::{
    GenerationError, GenerationMessage, TextGenerationProvider, TextGenerationRequest,
};
use super::model_selection::selected_model;
use brain_contracts::bot_tasks::{
    GeneratedTitle, TaskFailure, TaskUsage, TwitchTitleInput, TwitchTitlePort,
};
use brain_contracts::RequestDeadline;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TwitchTitleGenerator {
    provider: TextGenerationProvider,
    model_selection_path: PathBuf,
    max_output_tokens: u32,
}

impl TwitchTitleGenerator {
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

impl TwitchTitlePort for TwitchTitleGenerator {
    fn max_output_tokens(&self) -> u32 {
        self.max_output_tokens
    }

    fn available(&self) -> bool {
        selected_model(&self.model_selection_path).is_ok()
    }

    fn generate(
        &self,
        input: &TwitchTitleInput<'_>,
        deadline: &RequestDeadline,
    ) -> Result<GeneratedTitle, TaskFailure> {
        if input.max_tokens == 0 || input.max_tokens > self.max_output_tokens {
            return Err(TaskFailure::ProviderUnavailable);
        }
        let model = selected_model(&self.model_selection_path)
            .map_err(|_| TaskFailure::ProviderUnavailable)?;
        let messages = [GenerationMessage {
            role: "user",
            content: input.prompt,
        }];
        let generated = self
            .provider
            .generate(
                &TextGenerationRequest {
                    model: &model,
                    messages: &messages,
                    max_tokens: input.max_tokens,
                    temperature: input.temperature,
                },
                deadline,
            )
            .map_err(|error| match error {
                GenerationError::Unavailable => TaskFailure::ProviderUnavailable,
                GenerationError::RateLimited => TaskFailure::ProviderRateLimited,
                GenerationError::Timeout => TaskFailure::Timeout,
                GenerationError::InvalidOutput => TaskFailure::InvalidOutput,
            })?;
        Ok(GeneratedTitle {
            text: generated.content,
            model: generated.model,
            usage: TaskUsage {
                input_tokens: generated.prompt_tokens,
                output_tokens: generated.completion_tokens,
            },
        })
    }
}
