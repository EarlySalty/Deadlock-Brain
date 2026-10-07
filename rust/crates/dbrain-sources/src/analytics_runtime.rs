use crate::{
    external::SourceIr,
    schema_watch::{validate_consumed, OpenApiSnapshot, SchemaDependency},
    Result, SourcesError,
};
use brain_contracts::tools::ToolAnalyticsSelection;
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions, SourceHttpResponse};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    time::{Duration, Instant},
};

pub const SOURCE: &str = "deadlock_analytics_api";
pub const PARSER_REVISION: &str = "dbrain-analytics-runtime/1";
pub const BASE_URL: &str = "https://api.deadlock-api.com";
const MAX_RESPONSE_BYTES: usize = 1024 * 1024;
const MAX_WINDOW_SECONDS: i64 = 31 * 86_400;
const MAX_ROWS: usize = 256;
const UPSTREAM_HOUR_SECONDS: i64 = 3_600;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalyticsKind {
    Meta,
    Population,
}

impl AnalyticsKind {
    fn dependency(self) -> SchemaDependency {
        let (id, path) = match self {
            Self::Meta => ("analytics_meta", "/v1/analytics/hero-stats"),
            Self::Population => ("analytics_population", "/v1/analytics/hero-stats"),
        };
        SchemaDependency {
            id: id.into(),
            path: path.into(),
            method: "get".into(),
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::Meta => "/v1/analytics/hero-stats",
            Self::Population => "/v1/analytics/hero-stats",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyticsLookupRequest {
    pub kind: AnalyticsKind,
    pub hero_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<u32>,
    pub patch: String,
    pub min_unix_timestamp: i64,
    pub max_unix_timestamp: i64,
    pub max_rows: usize,
}

impl AnalyticsLookupRequest {
    fn effective_window(&self) -> Option<(i64, i64)> {
        let min = self
            .min_unix_timestamp
            .checked_add(UPSTREAM_HOUR_SECONDS - 1)?
            / UPSTREAM_HOUR_SECONDS
            * UPSTREAM_HOUR_SECONDS;
        let max =
            self.max_unix_timestamp - self.max_unix_timestamp.rem_euclid(UPSTREAM_HOUR_SECONDS);
        (min >= 0 && max > min).then_some((min, max))
    }

    pub fn validate(&self) -> Result<()> {
        let patch = self.patch.trim();
        if self.hero_id == 0
            || !matches!(
                (self.kind, self.item_id),
                (AnalyticsKind::Meta, None) | (AnalyticsKind::Population, Some(1..))
            )
            || patch.is_empty()
            || patch != self.patch
            || patch.len() > 128
            || !patch
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
            || self.min_unix_timestamp < 0
            || self.max_unix_timestamp <= self.min_unix_timestamp
            || self.max_unix_timestamp - self.min_unix_timestamp > MAX_WINDOW_SECONDS
            || self.effective_window().is_none()
            || !(1..=MAX_ROWS).contains(&self.max_rows)
        {
            return Err(SourcesError::invalid_input(
                "analytics lookup must pin hero, patch, <=31d window and 1..=256 rows",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchMembership {
    Unverified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyticsProvenance {
    pub source: String,
    pub locator: String,
    pub observed_at: i64,
    pub raw_sha256: String,
    pub schema_sha256: String,
    pub api_version: String,
    pub parser_revision: String,
    pub patch_membership: PatchMembership,
    pub min_unix_timestamp: i64,
    pub max_unix_timestamp: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_average_badge: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_average_badge: Option<u32>,
    pub attempts: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyticsObservation {
    pub kind: AnalyticsKind,
    pub hero_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<u32>,
    pub rows: Vec<Value>,
    pub provenance: AnalyticsProvenance,
}

#[derive(Debug, Clone)]
pub struct DeadlockAnalyticsClient {
    http: HttpClient,
    base_url: String,
    request_timeout: Duration,
}

impl DeadlockAnalyticsClient {
    pub fn new(http: HttpClient, request_timeout: Duration) -> Result<Self> {
        if request_timeout.is_zero() || request_timeout > Duration::from_secs(10) {
            return Err(SourcesError::invalid_input(
                "analytics timeout outside policy",
            ));
        }
        Ok(Self {
            http,
            base_url: BASE_URL.into(),
            request_timeout,
        })
    }

    #[cfg(any(test, feature = "analytics-fixtures"))]
    pub fn with_base_url(
        http: HttpClient,
        base_url: impl Into<String>,
        request_timeout: Duration,
    ) -> Result<Self> {
        let base_url = base_url.into();
        if !loopback_origin(&base_url)
            || request_timeout.is_zero()
            || request_timeout > Duration::from_secs(10)
        {
            return Err(SourcesError::invalid_input(
                "analytics fixture origin or timeout outside policy",
            ));
        }
        Ok(Self {
            http,
            base_url,
            request_timeout,
        })
    }

    pub fn lookup(&self, request: &AnalyticsLookupRequest) -> Result<AnalyticsObservation> {
        self.lookup_with_deadline(
            request,
            Instant::now() + self.request_timeout.saturating_mul(2),
        )
    }

    pub fn lookup_with_deadline(
        &self,
        request: &AnalyticsLookupRequest,
        deadline: Instant,
    ) -> Result<AnalyticsObservation> {
        self.lookup_until(request, deadline, None, None)
    }

    pub fn lookup_with_request_deadline(
        &self,
        request: &AnalyticsLookupRequest,
        deadline: Instant,
        lifetime: &brain_contracts::RequestDeadline,
    ) -> Result<AnalyticsObservation> {
        self.lookup_until(
            request,
            deadline.min(lifetime.expires_at()),
            Some(lifetime),
            None,
        )
    }

    pub fn lookup_with_selection_and_request_deadline(
        &self,
        request: &AnalyticsLookupRequest,
        selection: &ToolAnalyticsSelection,
        deadline: Instant,
        lifetime: &brain_contracts::RequestDeadline,
    ) -> Result<AnalyticsObservation> {
        self.lookup_until(
            request,
            deadline.min(lifetime.expires_at()),
            Some(lifetime),
            Some(selection),
        )
    }

    pub fn lookup_comparison_with_request_deadline(
        &self,
        request: &AnalyticsLookupRequest,
        hero_ids: &[u32],
        selection: &ToolAnalyticsSelection,
        deadline: Instant,
        lifetime: &brain_contracts::RequestDeadline,
    ) -> Result<Vec<AnalyticsObservation>> {
        self.lookup_comparison_until(
            request,
            hero_ids,
            deadline.min(lifetime.expires_at()),
            Some(lifetime),
            Some(selection),
        )
    }

    fn lookup_until(
        &self,
        request: &AnalyticsLookupRequest,
        deadline: Instant,
        lifetime: Option<&brain_contracts::RequestDeadline>,
        selection: Option<&ToolAnalyticsSelection>,
    ) -> Result<AnalyticsObservation> {
        self.lookup_comparison_until(request, &[request.hero_id], deadline, lifetime, selection)?
            .pop()
            .ok_or_else(|| SourcesError::invariant("analytics observation missing"))
    }

    fn lookup_comparison_until(
        &self,
        request: &AnalyticsLookupRequest,
        hero_ids: &[u32],
        deadline: Instant,
        lifetime: Option<&brain_contracts::RequestDeadline>,
        selection: Option<&ToolAnalyticsSelection>,
    ) -> Result<Vec<AnalyticsObservation>> {
        validate_selection(request, hero_ids, selection)?;
        if lifetime.is_some_and(|lifetime| lifetime.check().is_err()) {
            return Err(SourcesError::invalid_input("analytics request cancelled"));
        }
        let url = self.url(request, selection);
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(SourcesError::invalid_input("analytics deadline exhausted"));
        }
        let options = SourceHttpOptions {
            max_bytes: MAX_RESPONSE_BYTES,
            attempts: 2,
            request_timeout: self.request_timeout.min(remaining),
            total_timeout: self.request_timeout.saturating_mul(2).min(remaining),
            backoff: Duration::from_millis(200),
            max_retry_wait: Duration::from_secs(2),
            headers: vec![("Accept".into(), "application/json".into())],
        };
        let response = match lifetime {
            Some(lifetime) => self
                .http
                .get_bounded_until_cancellable(&url, options, deadline, lifetime),
            None => self.http.get_bounded_until(&url, options, deadline),
        }?;
        let observations = prepare_analytics_responses(request, hero_ids, selection, response)?;
        if Instant::now() >= deadline || lifetime.is_some_and(|lifetime| lifetime.check().is_err())
        {
            return Err(SourcesError::invalid_input("analytics deadline exhausted"));
        }
        Ok(observations)
    }

    fn url(
        &self,
        request: &AnalyticsLookupRequest,
        selection: Option<&ToolAnalyticsSelection>,
    ) -> String {
        let mut url = Url::parse(&format!("{}{}", self.base_url, request.kind.path()))
            .expect("validated analytics origin");
        url.query_pairs_mut()
            .extend_pairs(expected_query(request, selection));
        url.into()
    }
}

fn validate_selection(
    request: &AnalyticsLookupRequest,
    hero_ids: &[u32],
    selection: Option<&ToolAnalyticsSelection>,
) -> Result<()> {
    request.validate()?;
    let unique: BTreeSet<_> = hero_ids.iter().copied().collect();
    if hero_ids.is_empty()
        || hero_ids.len() > MAX_ROWS
        || unique.len() != hero_ids.len()
        || unique.contains(&0)
        || !unique.contains(&request.hero_id)
    {
        return Err(SourcesError::invalid_input(
            "invalid analytics comparison population",
        ));
    }
    if let Some(selection) = selection {
        selection
            .validate()
            .map_err(|_| SourcesError::invalid_input("invalid analytics rank or time selection"))?;
        if selection.min_unix_timestamp != request.min_unix_timestamp
            || selection.max_unix_timestamp != request.max_unix_timestamp
        {
            return Err(SourcesError::invalid_input(
                "analytics selection window mismatch",
            ));
        }
    }
    Ok(())
}

fn expected_query(
    request: &AnalyticsLookupRequest,
    selection: Option<&ToolAnalyticsSelection>,
) -> BTreeMap<&'static str, String> {
    let (min, max) = request
        .effective_window()
        .expect("validated analytics window");
    let upstream_max = max - UPSTREAM_HOUR_SECONDS;
    let mut query = BTreeMap::from([
        ("bucket", "no_bucket".into()),
        ("game_mode", "normal".into()),
        ("match_mode", "ranked,unranked".into()),
        ("min_match_id", "0".into()),
        ("min_unix_timestamp", min.to_string()),
        ("max_unix_timestamp", upstream_max.to_string()),
    ]);
    if let Some(item) = request.item_id {
        query.insert("include_item_ids", item.to_string());
    }
    if let Some(selection) = selection {
        if let Some(badge) = selection.min_average_badge {
            query.insert("min_average_badge", badge.to_string());
        }
        if let Some(badge) = selection.max_average_badge {
            query.insert("max_average_badge", badge.to_string());
        }
    }
    query
}

fn validate_response_locator(
    request: &AnalyticsLookupRequest,
    selection: Option<&ToolAnalyticsSelection>,
    value: &str,
) -> Result<()> {
    let url = Url::parse(value)
        .map_err(|_| SourcesError::invalid_input("invalid analytics response locator"))?;
    let production = url.scheme() == "https"
        && url.host_str() == Some("api.deadlock-api.com")
        && matches!(url.port(), None | Some(443));
    #[cfg(any(test, feature = "analytics-fixtures"))]
    let loopback = url.scheme() == "http"
        && url.port().is_some_and(|port| port != 0)
        && url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(|character| matches!(character, '[' | ']'))
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|address| address.is_loopback())
        });
    #[cfg(not(any(test, feature = "analytics-fixtures")))]
    let loopback = false;
    if !(production || loopback)
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || url.path() != request.kind.path()
    {
        return Err(SourcesError::invalid_input(
            "analytics response origin or path outside policy",
        ));
    }
    let mut actual = BTreeMap::new();
    for (key, value) in url.query_pairs() {
        if actual
            .insert(key.into_owned(), value.into_owned())
            .is_some()
        {
            return Err(SourcesError::invalid_input(
                "duplicate analytics response query parameter",
            ));
        }
    }
    let expected: BTreeMap<String, String> = expected_query(request, selection)
        .into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect();
    if actual != expected {
        return Err(SourcesError::invalid_input(
            "analytics response locator does not match requested window",
        ));
    }
    Ok(())
}

#[cfg(any(test, feature = "analytics-fixtures"))]
fn loopback_origin(value: &str) -> bool {
    let Some(authority) = value.strip_prefix("http://") else {
        return false;
    };
    if authority.is_empty()
        || authority
            .bytes()
            .any(|byte| matches!(byte, b'/' | b'?' | b'#' | b'@'))
    {
        return false;
    }
    if let Ok(address) = authority.parse::<std::net::SocketAddr>() {
        return address.ip().is_loopback() && address.port() != 0;
    }
    authority
        .strip_prefix("localhost:")
        .and_then(|port| port.parse::<u16>().ok())
        .is_some_and(|port| port != 0)
}

pub fn prepare_analytics_response(
    request: &AnalyticsLookupRequest,
    response: SourceHttpResponse,
) -> Result<AnalyticsObservation> {
    prepare_analytics_responses(request, &[request.hero_id], None, response)?
        .pop()
        .ok_or_else(|| SourcesError::invariant("analytics observation missing"))
}

fn prepare_analytics_responses(
    request: &AnalyticsLookupRequest,
    hero_ids: &[u32],
    selection: Option<&ToolAnalyticsSelection>,
    response: SourceHttpResponse,
) -> Result<Vec<AnalyticsObservation>> {
    validate_selection(request, hero_ids, selection)?;
    let (effective_min, effective_max) = request
        .effective_window()
        .expect("validated analytics window");
    validate_response_locator(request, selection, &response.url)?;
    if response.content.len() > MAX_RESPONSE_BYTES {
        return Err(SourcesError::invalid_input(
            "analytics response exceeds byte limit",
        ));
    }
    if response.observed_at <= 0 || response.observed_at < request.max_unix_timestamp {
        return Err(SourcesError::invalid_input(
            "analytics observation predates requested window",
        ));
    }
    if !(1..=2).contains(&response.attempts) {
        return Err(SourcesError::invalid_input(
            "analytics response attempt count outside policy",
        ));
    }
    let attempts = response.attempts;
    let mut ir = SourceIr::from_http(SOURCE, PARSER_REVISION, response)?;
    ir.set_derivation_family("deadlock-api-runtime-analytics");
    let pinned = OpenApiSnapshot::pinned()?;
    ir.pin_schema(&pinned.schema_sha256);
    ir.pin_schema_version(&pinned.api_version)?;
    if let Ok(payload) = ir.payload() {
        let schema = pinned.response_schema(&request.kind.dependency())?;
        match validate_consumed(&schema, payload) {
            Ok(extra) if extra.is_empty() => {}
            Ok(_) => ir.quarantine("analytics response has unpinned fields"),
            Err(errors) => {
                for error in errors {
                    ir.quarantine(error);
                }
            }
        }
    }
    if ir.is_quarantined() {
        return Err(SourcesError::invalid_input(format!(
            "analytics response quarantined: {:?}",
            ir.validation()
        )));
    }
    let rows = ir
        .payload()?
        .as_array()
        .ok_or_else(|| SourcesError::invalid_input("analytics response is not an array"))?;
    if rows.len() > MAX_ROWS {
        return Err(SourcesError::invalid_input(
            "analytics raw response exceeds row limit",
        ));
    }
    let requested: BTreeSet<_> = hero_ids.iter().map(|id| u64::from(*id)).collect();
    let mut matching = BTreeMap::new();
    for row in rows {
        let Some(hero_id) = row.get("hero_id").and_then(Value::as_u64) else {
            return Err(SourcesError::invalid_input(
                "analytics hero identity missing",
            ));
        };
        if !requested.contains(&hero_id) {
            continue;
        }
        let matches = row.get("matches").and_then(Value::as_u64);
        let total = row
            .get("wins")
            .and_then(Value::as_u64)
            .zip(row.get("losses").and_then(Value::as_u64))
            .and_then(|(wins, losses)| wins.checked_add(losses));
        if row.get("bucket").and_then(Value::as_i64) != Some(0)
            || total != matches
            || row.get("matches_per_bucket").and_then(Value::as_u64) != matches
            || matching.insert(hero_id, row.clone()).is_some()
        {
            return Err(SourcesError::invalid_input(
                "analytics hero aggregation identity drift",
            ));
        }
    }
    if matching.len() > request.max_rows {
        return Err(SourcesError::invalid_input(
            "analytics response exceeds row limit",
        ));
    }
    let contract = ir.contract().data;
    let schema_sha256 = contract
        .provenance
        .schema_sha256
        .clone()
        .ok_or_else(|| SourcesError::invariant("analytics schema hash missing"))?;
    let api_version = match &contract.schema_version {
        brain_contracts::value::Observed::Known { value } => value.clone(),
        brain_contracts::value::Observed::Unknown { .. } => {
            return Err(SourcesError::invariant("analytics schema version missing"))
        }
    };
    let provenance = AnalyticsProvenance {
        source: SOURCE.into(),
        locator: contract.provenance.locator.clone(),
        observed_at: contract.provenance.observed_at,
        raw_sha256: contract.provenance.raw_sha256.clone(),
        schema_sha256,
        api_version,
        parser_revision: PARSER_REVISION.into(),
        patch_membership: PatchMembership::Unverified,
        min_unix_timestamp: effective_min,
        max_unix_timestamp: effective_max,
        min_average_badge: selection.and_then(|selection| selection.min_average_badge),
        max_average_badge: selection.and_then(|selection| selection.max_average_badge),
        attempts,
    };
    Ok(hero_ids
        .iter()
        .map(|hero_id| AnalyticsObservation {
            kind: request.kind,
            hero_id: *hero_id,
            item_id: request.item_id,
            rows: matching.remove(&u64::from(*hero_id)).into_iter().collect(),
            provenance: provenance.clone(),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    fn request(kind: AnalyticsKind) -> AnalyticsLookupRequest {
        AnalyticsLookupRequest {
            kind,
            hero_id: 18,
            item_id: (kind == AnalyticsKind::Population).then_some(42),
            patch: "2026-09-24".into(),
            min_unix_timestamp: 1_790_000_000,
            max_unix_timestamp: 1_790_086_400,
            max_rows: 8,
        }
    }
    fn response(request: &AnalyticsLookupRequest, body: Value) -> SourceHttpResponse {
        let http =
            HttpClient::new("analytics-url-test", tempfile::tempdir().unwrap().path()).unwrap();
        let url = DeadlockAnalyticsClient::new(http, Duration::from_secs(1))
            .unwrap()
            .url(request, None);
        SourceHttpResponse {
            url,
            status: 200,
            content: serde_json::to_vec(&body).unwrap(),
            headers: BTreeMap::from([("content-type".into(), "application/json".into())]),
            observed_at: 1_790_086_401,
            attempts: 1,
        }
    }
    fn hero_row(hero_id: u32) -> Value {
        json!({
            "hero_id":hero_id,"bucket":0,"wins":10,"losses":5,"matches":15,
            "matches_per_bucket":15,"total_kills":100,"total_deaths":50,
            "total_assists":200,"total_net_worth":300000,"total_last_hits":1000,
            "total_denies":10,"total_player_damage":500000,"total_player_damage_taken":400000,
            "total_boss_damage":10000,"total_creep_damage":200000,"total_neutral_damage":50000,
            "total_max_health":30000,"total_shots_hit":1000,"total_shots_missed":500,
            "permanent_buff_matches":0,"permanent_buff_timing_matches":0,
            "total_first_permanent_buff_time_s":0,"total_permanent_buffs":0
        })
    }

    #[test]
    fn comparison_applies_rank_and_time_once_to_a_shared_http_population() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let selection = ToolAnalyticsSelection {
            min_average_badge: Some(70),
            max_average_badge: Some(80),
            min_unix_timestamp: 1_790_000_000,
            max_unix_timestamp: 1_790_086_400,
        };
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut headers = Vec::new();
            while !headers.ends_with(b"\r\n\r\n") {
                assert!(headers.len() < 8192);
                let mut byte = [0];
                stream.read_exact(&mut byte).unwrap();
                headers.push(byte[0]);
            }
            let request = String::from_utf8(headers).unwrap();
            let path = request
                .lines()
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap();
            let url = Url::parse(&format!("http://{address}{path}")).unwrap();
            let query: BTreeMap<_, _> = url.query_pairs().into_owned().collect();
            assert_eq!(url.path(), AnalyticsKind::Meta.path());
            assert_eq!(query["min_average_badge"], "70");
            assert_eq!(query["max_average_badge"], "80");
            assert_eq!(query["min_unix_timestamp"], "1790002800");
            assert_eq!(query["max_unix_timestamp"], "1790082000");
            assert!(!query.contains_key("hero_id"));
            assert!(!query.contains_key("hero_ids"));
            let body = serde_json::to_vec(&json!([hero_row(7), hero_row(18)])).unwrap();
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len()).unwrap();
            stream.write_all(&body).unwrap();
            body
        });
        let scratch = tempfile::tempdir().unwrap();
        let http = HttpClient::new("analytics-comparison-test", scratch.path()).unwrap();
        let client = DeadlockAnalyticsClient::with_base_url(
            http,
            format!("http://{address}"),
            Duration::from_secs(10),
        )
        .unwrap();
        let lifetime = brain_contracts::RequestDeadline::after(Duration::from_secs(20));
        let observations = client
            .lookup_comparison_with_request_deadline(
                &request(AnalyticsKind::Meta),
                &[18, 7],
                &selection,
                lifetime.expires_at(),
                &lifetime,
            )
            .unwrap();
        let bytes = server.join().unwrap();
        assert_eq!(observations.len(), 2);
        assert_eq!(observations[0].hero_id, 18);
        assert_eq!(observations[1].hero_id, 7);
        assert_eq!(observations[0].rows, vec![hero_row(18)]);
        assert_eq!(observations[1].rows, vec![hero_row(7)]);
        assert_eq!(observations[0].provenance, observations[1].provenance);
        assert_eq!(observations[0].provenance.attempts, 1);
        assert_eq!(observations[0].provenance.min_average_badge, Some(70));
        assert_eq!(observations[0].provenance.max_average_badge, Some(80));
        use sha2::{Digest, Sha256};
        assert_eq!(
            observations[0].provenance.raw_sha256,
            format!("{:x}", Sha256::digest(bytes))
        );
        assert_eq!(
            observations[0].provenance.patch_membership,
            PatchMembership::Unverified
        );
    }

    #[test]
    fn comparison_rejects_rank_locator_drift_and_invalid_population() {
        let request = request(AnalyticsKind::Meta);
        let selection = ToolAnalyticsSelection {
            min_average_badge: Some(70),
            max_average_badge: Some(80),
            min_unix_timestamp: request.min_unix_timestamp,
            max_unix_timestamp: request.max_unix_timestamp,
        };
        let scratch = tempfile::tempdir().unwrap();
        let http = HttpClient::new("analytics-selection-test", scratch.path()).unwrap();
        let client = DeadlockAnalyticsClient::new(http, Duration::from_secs(1)).unwrap();
        let mut correct = response(&request, json!([hero_row(7), hero_row(18)]));
        correct.url = client.url(&request, Some(&selection));
        assert!(
            prepare_analytics_responses(&request, &[18, 7], Some(&selection), correct.clone())
                .is_ok()
        );
        for (old, new) in [
            ("min_average_badge=70", "min_average_badge=71"),
            ("max_average_badge=80", "max_average_badge=81"),
        ] {
            let mut drift = correct.clone();
            drift.url = drift.url.replace(old, new);
            assert!(
                prepare_analytics_responses(&request, &[18, 7], Some(&selection), drift).is_err()
            );
        }
        for heroes in [&[][..], &[18, 18][..], &[18, 0][..], &[7][..]] {
            assert!(prepare_analytics_responses(
                &request,
                heroes,
                Some(&selection),
                correct.clone()
            )
            .is_err());
        }
        let mut wrong_window = selection.clone();
        wrong_window.max_unix_timestamp += 1;
        assert!(prepare_analytics_responses(
            &request,
            &[18, 7],
            Some(&wrong_window),
            correct.clone()
        )
        .is_err());
        let mut wrong_rank = selection;
        wrong_rank.min_average_badge = Some(81);
        assert!(
            prepare_analytics_responses(&request, &[18, 7], Some(&wrong_rank), correct).is_err()
        );
    }

    #[test]
    fn meta_lookup_filters_hero_and_carries_window_and_schema_provenance() {
        let request = request(AnalyticsKind::Meta);
        let raw = json!([hero_row(7), hero_row(18)]);
        let observation = prepare_analytics_response(&request, response(&request, raw)).unwrap();
        assert_eq!(observation.rows.len(), 1);
        assert_eq!(observation.rows[0]["hero_id"], 18);
        assert_eq!(
            observation.provenance.patch_membership,
            PatchMembership::Unverified
        );
        assert!(serde_json::to_value(&observation.provenance)
            .unwrap()
            .get("patch")
            .is_none());
        assert!(!observation.provenance.schema_sha256.is_empty());
        assert!(!observation.provenance.raw_sha256.is_empty());
        assert_eq!(observation.provenance.api_version, "0.1.0");
        assert_eq!(observation.provenance.parser_revision, PARSER_REVISION);
        assert_eq!(observation.provenance.attempts, 1);
    }

    #[test]
    fn population_uses_item_filtered_hero_matches_not_purchase_counts() {
        let request = request(AnalyticsKind::Population);
        let good = prepare_analytics_response(
            &request,
            response(&request, json!([hero_row(7), hero_row(18)])),
        )
        .unwrap();
        assert_eq!(good.rows.len(), 1);
        assert_eq!(good.rows[0]["matches"], 15);
        assert_eq!(good.item_id, Some(42));
        assert!(good
            .provenance
            .locator
            .contains("/v1/analytics/hero-stats?"));
        assert!(good.provenance.locator.contains("include_item_ids=42"));
        assert!(good
            .provenance
            .locator
            .contains("min_unix_timestamp=1790002800"));
        assert_eq!(good.provenance.min_unix_timestamp, 1_790_002_800);
        assert_eq!(good.provenance.max_unix_timestamp, 1_790_085_600);
        assert!(prepare_analytics_response(
            &request,
            response(&request, json!([hero_row(18), hero_row(18)])),
        )
        .is_err());
        let mut drift = hero_row(18);
        drift["bucket"] = json!(12);
        assert!(prepare_analytics_response(&request, response(&request, json!([drift]))).is_err());
        let mut inconsistent = hero_row(18);
        inconsistent["matches"] = json!(16);
        assert!(
            prepare_analytics_response(&request, response(&request, json!([inconsistent])))
                .is_err()
        );
        let mut wrong_item = response(&request, json!([hero_row(18)]));
        wrong_item.url = wrong_item
            .url
            .replace("include_item_ids=42", "include_item_ids=43");
        assert!(prepare_analytics_response(&request, wrong_item).is_err());
        let mut missing_item = request.clone();
        missing_item.item_id = None;
        assert!(missing_item.validate().is_err());
    }

    #[test]
    fn response_origin_bytes_raw_rows_and_requested_window_are_bounded() {
        let request = request(AnalyticsKind::Meta);
        let mut wrong_origin = response(&request, json!([hero_row(18)]));
        wrong_origin.url = wrong_origin.url.replacen(
            "https://api.deadlock-api.com",
            "https://api.deadlock-api.com.evil.test",
            1,
        );
        assert!(prepare_analytics_response(&request, wrong_origin).is_err());

        let mut wrong_window = response(&request, json!([hero_row(18)]));
        wrong_window.url = wrong_window.url.replace(
            "max_unix_timestamp=1790082000",
            "max_unix_timestamp=1790082001",
        );
        assert!(prepare_analytics_response(&request, wrong_window).is_err());

        let mut oversized = response(&request, json!([hero_row(18)]));
        oversized.content = vec![b' '; MAX_RESPONSE_BYTES + 1];
        assert!(prepare_analytics_response(&request, oversized).is_err());

        let raw_rows: Vec<_> = (0..=MAX_ROWS).map(|_| hero_row(7)).collect();
        assert!(prepare_analytics_response(&request, response(&request, json!(raw_rows))).is_err());
    }

    #[test]
    fn schema_drift_is_quarantined() {
        let request = request(AnalyticsKind::Meta);
        let mut drifted = hero_row(18);
        drifted.as_object_mut().unwrap().remove("wins");
        assert!(
            prepare_analytics_response(&request, response(&request, json!([drifted]))).is_err()
        );
        let mut unknown = hero_row(18);
        unknown["unreviewed_field"] = json!(1);
        assert!(
            prepare_analytics_response(&request, response(&request, json!([unknown]))).is_err()
        );
        let mut error_status = response(&request, json!([hero_row(18)]));
        error_status.status = 500;
        assert!(prepare_analytics_response(&request, error_status).is_err());
        let mut wrong_type = response(&request, json!([hero_row(18)]));
        wrong_type
            .headers
            .insert("content-type".into(), "text/plain".into());
        assert!(prepare_analytics_response(&request, wrong_type).is_err());
    }

    #[test]
    fn request_and_origin_policy_fail_closed() {
        let http = HttpClient::new("analytics-test", tempfile::tempdir().unwrap().path()).unwrap();
        assert!(DeadlockAnalyticsClient::with_base_url(
            http.clone(),
            "http://example.invalid",
            Duration::from_secs(1)
        )
        .is_err());
        assert!(DeadlockAnalyticsClient::with_base_url(
            http,
            "http://127.0.0.1:1234",
            Duration::from_secs(1)
        )
        .is_ok());
        let mut invalid = request(AnalyticsKind::Meta);
        invalid.max_unix_timestamp = invalid.min_unix_timestamp + MAX_WINDOW_SECONDS + 1;
        assert!(invalid.validate().is_err());
        invalid.max_unix_timestamp = invalid.min_unix_timestamp + 1;
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn expired_deadline_never_starts_http() {
        let http = HttpClient::new("analytics-test", tempfile::tempdir().unwrap().path()).unwrap();
        let client = DeadlockAnalyticsClient::new(http, Duration::from_secs(1)).unwrap();
        let error = client
            .lookup_with_deadline(&request(AnalyticsKind::Meta), Instant::now())
            .unwrap_err();
        assert!(matches!(error, SourcesError::InvalidInput(_)));
    }

    #[test]
    fn bounded_lookup_uses_only_requested_origin_and_returns_validated_observation() {
        use std::{
            io::{Read, Write},
            net::TcpListener,
            thread,
        };

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let body = serde_json::to_vec(&json!([hero_row(7), hero_row(18)])).unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request_bytes = [0u8; 4096];
            let count = stream.read(&mut request_bytes).unwrap();
            let request = String::from_utf8_lossy(&request_bytes[..count]);
            assert!(request.starts_with("GET /v1/analytics/hero-stats?"));
            assert!(request.contains("min_match_id=0"));
            assert!(request.contains("min_unix_timestamp=1790002800"));
            assert!(request.contains("max_unix_timestamp=1790082000"));
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            )
            .unwrap();
            stream.write_all(&body).unwrap();
        });
        let http = HttpClient::new(
            "analytics-http-fixture",
            tempfile::tempdir().unwrap().path(),
        )
        .unwrap();
        let client = DeadlockAnalyticsClient::with_base_url(
            http,
            format!("http://{address}"),
            Duration::from_secs(2),
        )
        .unwrap();
        let observation = client.lookup(&request(AnalyticsKind::Meta)).unwrap();
        assert_eq!(observation.rows.len(), 1);
        assert_eq!(observation.rows[0]["hero_id"], 18);
        assert_eq!(observation.provenance.attempts, 1);
        assert_eq!(observation.provenance.source, SOURCE);
        server.join().unwrap();
    }

    #[test]
    fn runtime_module_has_no_database_or_central_dsn_dependency() {
        let source = include_str!("analytics_runtime.rs");
        for forbidden in [
            ["DEADLOCK", "_CENTRAL_DSN"].concat(),
            ["sqlx", "::"].concat(),
            ["Click", "House"].concat(),
            ["Source", "Store"].concat(),
        ] {
            assert!(
                !source.contains(&forbidden),
                "forbidden runtime dependency: {forbidden}"
            );
        }
    }
}
