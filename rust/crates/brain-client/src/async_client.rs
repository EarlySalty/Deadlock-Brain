use crate::{transport, ClientError, Result};
use brain_contracts::{PublicAnswerResponse, Query};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::time::Duration;

#[derive(Clone)]
pub struct AsyncBrainClient {
    client: reqwest::Client,
    base_url: String,
    bearer: HeaderValue,
}
impl std::fmt::Debug for AsyncBrainClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncBrainClient")
            .field("endpoint", &"<configured>")
            .field("bearer", &"<redacted>")
            .finish()
    }
}
impl AsyncBrainClient {
    pub fn new_local(base_url: &str, bearer_token: &str, timeout: Duration) -> Result<Self> {
        let parsed =
            reqwest::Url::parse(base_url.trim()).map_err(|_| ClientError::InvalidBaseUrl)?;
        let local = parsed.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(['[', ']'])
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        });
        if !local {
            return Err(ClientError::InvalidBaseUrl);
        }
        Self::new(base_url, bearer_token, timeout)
    }
    pub fn new(base_url: &str, bearer_token: &str, timeout: Duration) -> Result<Self> {
        let (base_url, bearer) = transport::endpoint(base_url, bearer_token, timeout)?;
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .connect_timeout(timeout.min(Duration::from_secs(3)))
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .build()?;
        Ok(Self {
            client,
            base_url,
            bearer,
        })
    }
    pub async fn answer(&self, query: &Query) -> Result<PublicAnswerResponse> {
        self.answer_with_identity(query, None, true, None, None)
            .await
    }

    pub async fn answer_for_discord(
        &self,
        query: &Query,
        discord_user_id: u64,
    ) -> Result<PublicAnswerResponse> {
        self.answer_for_discord_with_read_access(query, discord_user_id, true)
            .await
    }

    pub async fn answer_for_discord_with_read_access(
        &self,
        query: &Query,
        discord_user_id: u64,
        allow_discord_reads: bool,
    ) -> Result<PublicAnswerResponse> {
        self.answer_with_identity(
            query,
            Some(discord_user_id),
            allow_discord_reads,
            None,
            None,
        )
        .await
    }

    pub async fn answer_for_discord_with_history(
        &self,
        query: &Query,
        discord_user_id: u64,
        user_questions: &[String],
    ) -> Result<PublicAnswerResponse> {
        if discord_user_id == 0 {
            return Err(ClientError::InvalidResponse);
        }
        self.answer_with_identity(
            query,
            Some(discord_user_id),
            false,
            None,
            Some(user_questions),
        )
        .await
    }

    pub async fn answer_discord_task(
        &self,
        query: &Query,
        discord_user_id: u64,
        task: &crate::DiscordAnswerTask,
    ) -> Result<PublicAnswerResponse> {
        if discord_user_id == 0 || !task.valid() {
            return Err(ClientError::InvalidResponse);
        }
        self.answer_with_identity(query, Some(discord_user_id), false, Some(task), None)
            .await
    }

    pub async fn answer_discord_task_with_history(
        &self,
        query: &Query,
        discord_user_id: u64,
        task: &crate::DiscordAnswerTask,
        user_questions: &[String],
    ) -> Result<PublicAnswerResponse> {
        if discord_user_id == 0 || !task.valid() {
            return Err(ClientError::InvalidResponse);
        }
        self.answer_with_identity(
            query,
            Some(discord_user_id),
            false,
            Some(task),
            Some(user_questions),
        )
        .await
    }

    async fn answer_with_identity(
        &self,
        query: &Query,
        discord_user_id: Option<u64>,
        allow_discord_reads: bool,
        task: Option<&crate::DiscordAnswerTask>,
        user_questions: Option<&[String]>,
    ) -> Result<PublicAnswerResponse> {
        let request = if let Some(questions) = user_questions {
            let url =
                reqwest::Url::parse(&self.base_url).map_err(|_| ClientError::InvalidBaseUrl)?;
            if !url.host_str().is_some_and(|host| {
                host == "localhost"
                    || host
                        .trim_matches(['[', ']'])
                        .parse::<std::net::IpAddr>()
                        .is_ok_and(|ip| ip.is_loopback())
            }) {
                return Err(ClientError::InvalidBaseUrl);
            }
            let request = brain_contracts::discord_task::DiscordTaskRequest {
                query: query.clone(),
                user_questions: questions.to_vec(),
            };
            request
                .validate()
                .map_err(|_| ClientError::InvalidResponse)?;
            let encoded = serde_json::to_vec(&request)?;
            if encoded.len() > transport::MAX_REQUEST_BYTES {
                return Err(ClientError::RequestTooLarge);
            }
            encoded
        } else {
            transport::encode_request(query)?
        };
        let mut builder = self
            .client
            .post(format!("{}/v1/answer", self.base_url))
            .header(AUTHORIZATION, self.bearer.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(request);
        if let Some(user) = discord_user_id.filter(|id| *id != 0) {
            builder = builder.header("x-discord-user-id", user);
        }
        if !allow_discord_reads {
            builder = builder.header("x-discord-read-access", "disabled");
        }
        if let Some(task) = task {
            builder = builder.header("x-discord-answer-task", serde_json::to_string(task)?);
        }
        let mut response = builder.send().await?;
        let status = response.status();
        if response
            .content_length()
            .is_some_and(|n| n > transport::MAX_RESPONSE_BYTES as u64)
        {
            return Err(ClientError::ResponseTooLarge);
        }
        let is_json = transport::json_content_type(response.headers());
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if chunk.len() > transport::MAX_RESPONSE_BYTES.saturating_sub(bytes.len()) {
                return Err(ClientError::ResponseTooLarge);
            }
            bytes.extend_from_slice(&chunk);
        }
        transport::decode_response(status, is_json, &bytes, &query.request_id)
    }
}
