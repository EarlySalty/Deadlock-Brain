use crate::{
    external::SourceIr,
    schema_watch::{validate_consumed, OpenApiSnapshot, SchemaDependency},
    Result, SourcesError,
};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions, SourceHttpResponse};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

pub const SOURCE: &str = "deadlock_analytics_api";
pub const PARSER_REVISION: &str = "dbrain-analytics-runtime/1";
pub const BASE_URL: &str = "https://api.deadlock-api.com";
const MAX_RESPONSE_BYTES: usize = 1024 * 1024;
const MAX_WINDOW_SECONDS: i64 = 31 * 86_400;
const MAX_ROWS: usize = 256;

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
            Self::Population => ("analytics_population", "/v1/analytics/build-item-stats"),
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
            Self::Population => "/v1/analytics/build-item-stats",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyticsLookupRequest {
    pub kind: AnalyticsKind,
    pub hero_id: u32,
    pub patch: String,
    pub min_unix_timestamp: i64,
    pub max_unix_timestamp: i64,
    pub max_rows: usize,
}

impl AnalyticsLookupRequest {
    pub fn validate(&self) -> Result<()> {
        let patch = self.patch.trim();
        if self.hero_id == 0
            || patch.is_empty()
            || patch != self.patch
            || patch.len() > 128
            || !patch
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
            || self.min_unix_timestamp < 0
            || self.max_unix_timestamp <= self.min_unix_timestamp
            || self.max_unix_timestamp - self.min_unix_timestamp > MAX_WINDOW_SECONDS
            || !(1..=MAX_ROWS).contains(&self.max_rows)
        {
            return Err(SourcesError::invalid_input(
                "analytics lookup must pin hero, patch, <=31d window and 1..=256 rows",
            ));
        }
        Ok(())
    }
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
    pub patch: String,
    pub min_unix_timestamp: i64,
    pub max_unix_timestamp: i64,
    pub attempts: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyticsObservation {
    pub kind: AnalyticsKind,
    pub hero_id: u32,
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

    #[cfg(test)]
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
        request.validate()?;
        let url = self.url(request);
        let response = self.http.get_bounded(
            &url,
            SourceHttpOptions {
                max_bytes: MAX_RESPONSE_BYTES,
                attempts: 2,
                request_timeout: self.request_timeout,
                total_timeout: self.request_timeout.saturating_mul(2),
                backoff: Duration::from_millis(200),
                max_retry_wait: Duration::from_secs(2),
                headers: vec![("Accept".into(), "application/json".into())],
            },
        )?;
        prepare_analytics_response(request, response)
    }

    fn url(&self, request: &AnalyticsLookupRequest) -> String {
        match request.kind {
            AnalyticsKind::Meta => format!(
                "{}{path}?bucket=no_bucket&game_mode=normal&match_mode=ranked%2Cunranked&min_unix_timestamp={min}&max_unix_timestamp={max}",
                self.base_url,
                path = request.kind.path(),
                min = request.min_unix_timestamp,
                max = request.max_unix_timestamp,
            ),
            AnalyticsKind::Population => format!(
                "{}{path}?hero_id={hero}&min_last_updated_unix_timestamp={min}&max_last_updated_unix_timestamp={max}",
                self.base_url,
                path = request.kind.path(),
                hero = request.hero_id,
                min = request.min_unix_timestamp,
                max = request.max_unix_timestamp,
            ),
        }
    }
}

fn expected_query(request: &AnalyticsLookupRequest) -> BTreeMap<&'static str, String> {
    match request.kind {
        AnalyticsKind::Meta => BTreeMap::from([
            ("bucket", "no_bucket".into()),
            ("game_mode", "normal".into()),
            ("match_mode", "ranked,unranked".into()),
            ("min_unix_timestamp", request.min_unix_timestamp.to_string()),
            ("max_unix_timestamp", request.max_unix_timestamp.to_string()),
        ]),
        AnalyticsKind::Population => BTreeMap::from([
            ("hero_id", request.hero_id.to_string()),
            (
                "min_last_updated_unix_timestamp",
                request.min_unix_timestamp.to_string(),
            ),
            (
                "max_last_updated_unix_timestamp",
                request.max_unix_timestamp.to_string(),
            ),
        ]),
    }
}

fn validate_response_locator(request: &AnalyticsLookupRequest, value: &str) -> Result<()> {
    let url = Url::parse(value)
        .map_err(|_| SourcesError::invalid_input("invalid analytics response locator"))?;
    let production = url.scheme() == "https"
        && url.host_str() == Some("api.deadlock-api.com")
        && matches!(url.port(), None | Some(443));
    #[cfg(test)]
    let loopback = url.scheme() == "http"
        && url.port().is_some_and(|port| port != 0)
        && url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(|character| matches!(character, '[' | ']'))
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|address| address.is_loopback())
        });
    #[cfg(not(test))]
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
    let expected: BTreeMap<String, String> = expected_query(request)
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

#[cfg(test)]
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
    request.validate()?;
    validate_response_locator(request, &response.url)?;
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
    let rows: Vec<Value> = match request.kind {
        AnalyticsKind::Meta => {
            let matching: Vec<_> = rows
                .iter()
                .filter(|row| {
                    row.get("hero_id").and_then(Value::as_u64) == Some(request.hero_id.into())
                })
                .cloned()
                .collect();
            if matching.len() > 1
                || matching
                    .iter()
                    .any(|row| row.get("bucket").and_then(Value::as_i64) != Some(0))
            {
                return Err(SourcesError::invalid_input(
                    "analytics hero aggregation identity drift",
                ));
            }
            matching
        }
        AnalyticsKind::Population => {
            let mut seen = BTreeSet::new();
            if rows.iter().any(|row| {
                !row.get("item_id")
                    .and_then(Value::as_u64)
                    .is_some_and(|id| id > 0 && seen.insert(id))
            }) {
                return Err(SourcesError::invalid_input(
                    "analytics item identity missing or repeated",
                ));
            }
            rows.to_vec()
        }
    };
    if rows.len() > request.max_rows {
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
    Ok(AnalyticsObservation {
        kind: request.kind,
        hero_id: request.hero_id,
        rows,
        provenance: AnalyticsProvenance {
            source: SOURCE.into(),
            locator: contract.provenance.locator.clone(),
            observed_at: contract.provenance.observed_at,
            raw_sha256: contract.provenance.raw_sha256.clone(),
            schema_sha256,
            api_version,
            parser_revision: PARSER_REVISION.into(),
            patch: request.patch.clone(),
            min_unix_timestamp: request.min_unix_timestamp,
            max_unix_timestamp: request.max_unix_timestamp,
            attempts,
        },
    })
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
            .url(request);
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
            "total_max_health":30000,"total_shots_hit":1000,"total_shots_missed":500
        })
    }

    #[test]
    fn meta_lookup_filters_hero_and_carries_patch_window_and_schema_provenance() {
        let request = request(AnalyticsKind::Meta);
        let raw = json!([hero_row(7), hero_row(18)]);
        let observation = prepare_analytics_response(&request, response(&request, raw)).unwrap();
        assert_eq!(observation.rows.len(), 1);
        assert_eq!(observation.rows[0]["hero_id"], 18);
        assert_eq!(observation.provenance.patch, "2026-09-24");
        assert!(!observation.provenance.schema_sha256.is_empty());
        assert!(!observation.provenance.raw_sha256.is_empty());
        assert_eq!(observation.provenance.api_version, "0.1.0");
        assert_eq!(observation.provenance.parser_revision, PARSER_REVISION);
        assert_eq!(observation.provenance.attempts, 1);
    }

    #[test]
    fn population_lookup_is_bounded_and_schema_checked() {
        let mut request = request(AnalyticsKind::Population);
        request.max_rows = 1;
        let good = prepare_analytics_response(
            &request,
            response(&request, json!([{"item_id":1,"builds":42}])),
        )
        .unwrap();
        assert_eq!(good.rows[0]["builds"], 42);
        assert!(prepare_analytics_response(
            &request,
            response(
                &request,
                json!([{"item_id":1,"builds":42},{"item_id":2,"builds":9}]),
            ),
        )
        .is_err());
        assert!(prepare_analytics_response(
            &request,
            response(&request, json!([{"item_id":"wrong","builds":42}])),
        )
        .is_err());
        request.max_rows = 2;
        assert!(prepare_analytics_response(
            &request,
            response(
                &request,
                json!([{"item_id":1,"builds":42},{"item_id":1,"builds":9}])
            ),
        )
        .is_err());
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
            "max_unix_timestamp=1790086400",
            "max_unix_timestamp=1790086401",
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
            assert!(request.contains("min_unix_timestamp=1790000000"));
            assert!(request.contains("max_unix_timestamp=1790086400"));
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
