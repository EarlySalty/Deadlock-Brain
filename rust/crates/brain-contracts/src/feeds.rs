use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub const PATCHNOTES_FEED_VERSION: &str = "brain.feed.patchnotes.v1";
pub const BUILD_PUBLISH_VERSION: &str = "brain.build_publish.v1";
pub const MAX_FEED_POSTS: usize = 5_000;
pub const MAX_POST_BYTES: usize = 1024 * 1024;
pub const MAX_BUILD_PAYLOAD_BYTES: usize = 256 * 1024;

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn valid_id(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.len() <= max && !value.chars().any(char::is_control)
}
fn valid_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchnotesPostV1 {
    pub post_id: String,
    pub title: String,
    pub url: String,
    pub published_at: Option<i64>,
    pub language: Option<String>,
    pub raw_text: String,
    pub raw_sha256: String,
    pub source_revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchnotesFeedV1 {
    pub contract_version: String,
    pub provider: String,
    pub export_revision: String,
    pub exported_at: i64,
    pub posts: Vec<PatchnotesPostV1>,
}

impl PatchnotesFeedV1 {
    pub fn validate(&self) -> Result<(), String> {
        if self.contract_version != PATCHNOTES_FEED_VERSION {
            return Err("unsupported patchnotes feed version".into());
        }
        if !valid_id(&self.provider, 128) || !valid_id(&self.export_revision, 256) {
            return Err("invalid provider or export revision".into());
        }
        if self.exported_at <= 0 || self.posts.is_empty() || self.posts.len() > MAX_FEED_POSTS {
            return Err("invalid export time or post count".into());
        }
        let mut ids = BTreeSet::new();
        for post in &self.posts {
            if !valid_id(&post.post_id, 256)
                || !valid_id(&post.title, 512)
                || !valid_id(&post.source_revision, 256)
                || !post.url.starts_with("https://")
                || !valid_id(&post.url, 2048)
                || post.language.as_deref().is_some_and(|l| !valid_id(l, 16))
                || post.published_at.is_some_and(|t| t <= 0)
            {
                return Err(format!("invalid post identity {}", post.post_id));
            }
            if post.raw_text.trim().is_empty() || post.raw_text.len() > MAX_POST_BYTES {
                return Err(format!("invalid post text {}", post.post_id));
            }
            if !valid_hash(&post.raw_sha256)
                || sha256_hex(post.raw_text.as_bytes()) != post.raw_sha256
            {
                return Err(format!("raw hash mismatch {}", post.post_id));
            }
            if !ids.insert(post.post_id.as_str()) {
                return Err(format!("duplicate post {}", post.post_id));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildPublishRequest {
    pub contract_version: String,
    pub request_id: String,
    pub hero_id: u32,
    pub hero_name: String,
    pub build_name: String,
    pub payload: serde_json::Value,
    pub caller: String,
}

impl BuildPublishRequest {
    pub fn request_sha256(&self) -> Result<String, String> {
        serde_json::to_vec(self)
            .map(|bytes| sha256_hex(&bytes))
            .map_err(|e| e.to_string())
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.contract_version != BUILD_PUBLISH_VERSION {
            return Err("unsupported build publish version".into());
        }
        if !valid_id(&self.request_id, 128)
            || !valid_id(&self.hero_name, 128)
            || !valid_id(&self.build_name, 256)
            || !valid_id(&self.caller, 128)
            || self.hero_id == 0
        {
            return Err("invalid build publish identity".into());
        }
        let size = serde_json::to_vec(&self.payload)
            .map_err(|e| e.to_string())?
            .len();
        if !self.payload.is_object() || size > MAX_BUILD_PAYLOAD_BYTES {
            return Err("build payload must be a bounded object".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildPublishErrorClass {
    Rejected,
    SteamUnavailable,
    RateLimited,
    Timeout,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum BuildPublishState {
    Queued,
    Running,
    Succeeded { hero_build_id: u64 },
    Failed { error_class: BuildPublishErrorClass },
}

impl BuildPublishState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Succeeded { .. } | Self::Failed { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildPublishStatus {
    pub contract_version: String,
    pub request_id: String,
    pub request_sha256: String,
    pub state: BuildPublishState,
    pub submitted_at: i64,
    pub updated_at: i64,
}

impl BuildPublishStatus {
    pub fn validate_for(
        &self,
        request_id: &str,
        request_sha256: Option<&str>,
    ) -> Result<(), String> {
        if self.contract_version != BUILD_PUBLISH_VERSION
            || self.request_id != request_id
            || !valid_hash(&self.request_sha256)
            || request_sha256.is_some_and(|h| h != self.request_sha256)
            || self.submitted_at <= 0
            || self.updated_at < self.submitted_at
            || matches!(
                self.state,
                BuildPublishState::Succeeded { hero_build_id: 0 }
            )
        {
            return Err("build publish status does not match the request".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn post(id: &str, text: &str) -> PatchnotesPostV1 {
        PatchnotesPostV1 {
            post_id: id.into(),
            title: "Update".into(),
            url: "https://example.invalid/p".into(),
            published_at: Some(1),
            language: Some("en".into()),
            raw_text: text.into(),
            raw_sha256: sha256_hex(text.as_bytes()),
            source_revision: "r1".into(),
        }
    }
    fn feed(posts: Vec<PatchnotesPostV1>) -> PatchnotesFeedV1 {
        PatchnotesFeedV1 {
            contract_version: PATCHNOTES_FEED_VERSION.into(),
            provider: "patchnotes-bot".into(),
            export_revision: "e1".into(),
            exported_at: 10,
            posts,
        }
    }

    #[test]
    fn feed_rejects_hash_mismatch_duplicates_unknown_fields_and_versions() {
        assert!(feed(vec![post("a", "x")]).validate().is_ok());
        let mut bad = post("a", "x");
        bad.raw_text = "y".into();
        assert!(feed(vec![bad]).validate().is_err());
        assert!(feed(vec![post("a", "x"), post("a", "z")])
            .validate()
            .is_err());
        let mut version = feed(vec![post("a", "x")]);
        version.contract_version = "brain.feed.patchnotes.v2".into();
        assert!(version.validate().is_err());
        let mut http = post("a", "x");
        http.url = "http://example.invalid".into();
        assert!(feed(vec![http]).validate().is_err());
        let mut value = serde_json::to_value(feed(vec![post("a", "x")])).unwrap();
        value["posts"][0]["extra"] = serde_json::json!(1);
        assert!(serde_json::from_value::<PatchnotesFeedV1>(value).is_err());
    }

    #[test]
    fn build_status_binds_request_identity_and_hash() {
        let request = BuildPublishRequest {
            contract_version: BUILD_PUBLISH_VERSION.into(),
            request_id: "req-1".into(),
            hero_id: 25,
            hero_name: "Warden".into(),
            build_name: "Test".into(),
            payload: serde_json::json!({"items": []}),
            caller: "brain".into(),
        };
        request.validate().unwrap();
        let hash = request.request_sha256().unwrap();
        let status = BuildPublishStatus {
            contract_version: BUILD_PUBLISH_VERSION.into(),
            request_id: "req-1".into(),
            request_sha256: hash.clone(),
            state: BuildPublishState::Succeeded { hero_build_id: 7 },
            submitted_at: 1,
            updated_at: 2,
        };
        assert!(status.validate_for("req-1", Some(&hash)).is_ok());
        assert!(status.validate_for("req-2", Some(&hash)).is_err());
        assert!(status.validate_for("req-1", Some(&"0".repeat(64))).is_err());
        let zero = BuildPublishStatus {
            state: BuildPublishState::Succeeded { hero_build_id: 0 },
            ..status
        };
        assert!(zero.validate_for("req-1", None).is_err());
        let mut array = request.clone();
        array.payload = serde_json::json!([]);
        assert!(array.validate().is_err());
    }
}
