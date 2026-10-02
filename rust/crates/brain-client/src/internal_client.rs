//! Eigener privater Antworttyp über einen geschützten Unixsocket.
use crate::{transport, ClientError, Result};
use brain_contracts::{internal_api::InternalAnswerResponse, Query};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::{
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    time::Duration,
};

pub struct AsyncInternalBrainClient {
    client: reqwest::Client,
    bearer: HeaderValue,
    socket: PathBuf,
    owner: u32,
}

fn validate_socket(socket: &Path, owner: u32) -> Result<()> {
    uplink_infisical_transport::validate_socket(socket, owner)
        .map_err(|_| ClientError::InvalidBaseUrl)?;
    let path = std::fs::symlink_metadata(socket).map_err(|_| ClientError::InvalidBaseUrl)?;
    let directory = std::fs::symlink_metadata(socket.parent().ok_or(ClientError::InvalidBaseUrl)?)
        .map_err(|_| ClientError::InvalidBaseUrl)?;
    if path.mode() & 0o777 != 0o600 || directory.mode() & 0o777 != 0o700 || directory.uid() != owner
    {
        return Err(ClientError::InvalidBaseUrl);
    }
    Ok(())
}

impl AsyncInternalBrainClient {
    pub fn new(socket: &Path, owner: u32, token: &str, timeout: Duration) -> Result<Self> {
        let (_, bearer) = transport::endpoint("http://127.0.0.1", token, timeout)?;
        validate_socket(socket, owner)?;
        let client = uplink_infisical_transport::client_builder(socket, owner)
            .map_err(|_| ClientError::InvalidBaseUrl)?
            .timeout(timeout)
            .connect_timeout(timeout.min(Duration::from_secs(3)))
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .build()?;
        Ok(Self {
            client,
            bearer,
            socket: socket.to_owned(),
            owner,
        })
    }

    pub async fn query(&self, query: &Query) -> Result<InternalAnswerResponse> {
        validate_socket(&self.socket, self.owner)?;
        let bytes = transport::encode_request(query)?;
        let mut response = self
            .client
            .post("http://brain.internal/v1/operator/query")
            .header(AUTHORIZATION, self.bearer.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(bytes)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(ClientError::HttpStatus {
                status,
                body: "Interner Operatoraufruf abgelehnt".into(),
            });
        }
        if !transport::json_content_type(response.headers()) {
            return Err(ClientError::InvalidResponse);
        }
        if response
            .content_length()
            .is_some_and(|n| n > transport::MAX_RESPONSE_BYTES as u64)
        {
            return Err(ClientError::ResponseTooLarge);
        }
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if chunk.len() > transport::MAX_RESPONSE_BYTES.saturating_sub(body.len()) {
                return Err(ClientError::ResponseTooLarge);
            }
            body.extend_from_slice(&chunk);
        }
        let answer: InternalAnswerResponse =
            serde_json::from_slice(&body).map_err(|_| ClientError::InvalidResponse)?;
        if !answer.validate(&query.request_id) {
            return Err(ClientError::InvalidResponse);
        }
        Ok(answer)
    }
}
