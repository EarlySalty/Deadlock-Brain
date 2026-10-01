use std::{
    env, fmt,
    time::{Duration, Instant},
};

use deadlock_brain_core::{
    http::{HttpAttempt, HttpClient, HttpGetOptions, HttpResult, RetryPolicy},
    CoreError,
};
use postgres::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const LEDGER_BASE_URL: &str = "http://127.0.0.1:8901";
const TOKEN_HEADER: &str = "X-Internal-Token";
const TOKEN_ENV_NAMES: [&str; 4] = [
    "SERVERSYNC_INTERNAL_TOKEN",
    "MASTER_BROKER_TOKEN",
    "MAIN_BOT_INTERNAL_TOKEN",
    "TWITCH_INTERNAL_API_TOKEN",
];
const CALLER_CLASS: &str = "standard";
const LEDGER_TIMEOUT: Duration = Duration::from_secs(10);
const JOURNAL_TABLE: &str = "brain.steam_web_api_pending_observations";
const JOURNAL_GUARD_ID: i64 = i64::MIN;
const JOURNAL_LOCK_KEY: i64 = 0x5354_4541_4d4c_4447;
const JOURNAL_LOCK_WAIT: Duration = Duration::from_secs(120);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SteamWebApiError {
    Denied { reason: String, retry_at: String },
    Ledger(String),
    Journal(String),
    RateLimited { retry_after: Option<String> },
    Upstream { status: u16 },
    Transport,
    Local(String),
}

impl SteamWebApiError {
    pub fn blocks_fallback(&self) -> bool {
        matches!(
            self,
            Self::Denied { .. } | Self::Ledger(_) | Self::Journal(_) | Self::RateLimited { .. }
        )
    }
}

impl fmt::Display for SteamWebApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Denied { reason, retry_at } => write!(
                formatter,
                "Steam-Kontingent verweigert ({reason}); frühestens wieder ab {retry_at}. Kein Steam-Aufruf erfolgt."
            ),
            Self::Ledger(message) => write!(
                formatter,
                "Steam-Kontingent nicht nutzbar: {message}. Kein weiterer Steam-Aufruf erfolgt."
            ),
            Self::Journal(message) => write!(
                formatter,
                "Steam-Beobachtungsjournal nicht nutzbar: {message}. Kein weiterer Steam-Aufruf erfolgt."
            ),
            Self::RateLimited { retry_after } => write!(
                formatter,
                "Steam Web API meldet HTTP 429 (Retry-After: {}); dem Kontingent gemeldet.",
                retry_after.as_deref().unwrap_or("fehlt")
            ),
            Self::Upstream { status } => {
                write!(formatter, "Steam Web API antwortete mit HTTP {status}.")
            }
            Self::Transport => write!(formatter, "Steam Web API war nicht erreichbar."),
            Self::Local(message) => write!(formatter, "Steam-Antwort nicht verarbeitbar: {message}"),
        }
    }
}

impl std::error::Error for SteamWebApiError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Observation {
    pub reservation_id: i64,
    pub http_status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<String>,
}

impl Observation {
    fn from_attempt(reservation_id: i64, attempt: &Result<HttpAttempt, CoreError>) -> Self {
        match attempt {
            Ok(attempt) => Self {
                reservation_id,
                http_status: Some(attempt.status.as_u16()),
                retry_after: (attempt.status.as_u16() == 429)
                    .then(|| attempt.retry_after.clone())
                    .flatten(),
            },
            Err(_) => Self::transport_failure(reservation_id),
        }
    }

    fn transport_failure(reservation_id: i64) -> Self {
        Self {
            reservation_id,
            http_status: None,
            retry_after: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingObservation {
    pub reservation_id: i64,
    pub dispatch_started: bool,
    pub answered: bool,
    pub http_status: Option<u16>,
    pub retry_after: Option<String>,
}

impl PendingObservation {
    fn observation(&self) -> Observation {
        if !self.answered {
            return Observation::transport_failure(self.reservation_id);
        }
        Observation {
            reservation_id: self.reservation_id,
            http_status: self.http_status,
            retry_after: self.retry_after.clone(),
        }
    }
}

pub trait ObservationJournal {
    fn lock(&mut self) -> Result<(), String>;
    fn pending(&mut self) -> Result<Vec<PendingObservation>, String>;
    fn arm(&mut self) -> Result<(), String>;
    fn disarm(&mut self) -> Result<(), String>;
    fn reserved(&mut self, reservation_id: i64, caller: &str) -> Result<(), String>;
    fn dispatched(&mut self, reservation_id: i64) -> Result<(), String>;
    fn answered(&mut self, observation: &Observation) -> Result<(), String>;
    fn delivered(&mut self, reservation_id: i64) -> Result<(), String>;
}

pub struct PgObservationJournal<'a> {
    client: &'a mut Client,
    locked: bool,
}

impl<'a> PgObservationJournal<'a> {
    pub fn open(client: &'a mut Client) -> Result<Self, SteamWebApiError> {
        let row = client
            .query_one("SELECT to_regclass($1)::text", &[&JOURNAL_TABLE])
            .map_err(|_| SteamWebApiError::Journal("Postgres nicht lesbar".to_string()))?;
        if row.get::<_, Option<String>>(0).is_none() {
            return Err(SteamWebApiError::Journal(format!(
                "Tabelle {JOURNAL_TABLE} fehlt, Migration 2026-09-29-steam-web-api-journal.sql ausstehend"
            )));
        }
        Ok(Self {
            client,
            locked: false,
        })
    }
}

impl Drop for PgObservationJournal<'_> {
    fn drop(&mut self) {
        if self.locked {
            let released = self
                .client
                .query_one("SELECT pg_advisory_unlock($1)", &[&JOURNAL_LOCK_KEY])
                .map(|row| row.get::<_, bool>(0));
            if !matches!(released, Ok(true)) {
                eprintln!("Steam-Journal-Sperre konnte nicht freigegeben werden.");
            }
        }
    }
}

impl ObservationJournal for PgObservationJournal<'_> {
    fn lock(&mut self) -> Result<(), String> {
        if self.locked {
            return Ok(());
        }
        let deadline = Instant::now() + JOURNAL_LOCK_WAIT;
        loop {
            let acquired: bool = self
                .client
                .query_one("SELECT pg_try_advisory_lock($1)", &[&JOURNAL_LOCK_KEY])
                .map_err(|_| "Journal-Sperre nicht abfragbar".to_string())?
                .get(0);
            if acquired {
                self.locked = true;
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err("ein anderer Brain-Lauf hält die Steam-Sperre".to_string());
            }
            std::thread::sleep(Duration::from_millis(250));
        }
    }

    fn pending(&mut self) -> Result<Vec<PendingObservation>, String> {
        let rows = self
            .client
            .query(
                "SELECT reservation_id, dispatch_started, answered, http_status, retry_after \
                 FROM brain.steam_web_api_pending_observations ORDER BY reservation_id",
                &[],
            )
            .map_err(|_| "offene Beobachtungen nicht lesbar".to_string())?;
        Ok(rows
            .into_iter()
            .map(|row| PendingObservation {
                reservation_id: row.get(0),
                dispatch_started: row.get(1),
                answered: row.get(2),
                http_status: row
                    .get::<_, Option<i16>>(3)
                    .and_then(|status| u16::try_from(status).ok()),
                retry_after: row.get(4),
            })
            .collect())
    }

    fn arm(&mut self) -> Result<(), String> {
        self.client
            .execute(
                "INSERT INTO brain.steam_web_api_pending_observations \
                 (reservation_id, caller, dispatch_started) VALUES ($1, 'deadlock-brain-guard', true)",
                &[&JOURNAL_GUARD_ID],
            )
            .map(|_| ())
            .map_err(|_| "Reservierungs-Sperrvermerk nicht speicherbar".to_string())
    }

    fn disarm(&mut self) -> Result<(), String> {
        let removed = self
            .client
            .execute(
                "DELETE FROM brain.steam_web_api_pending_observations WHERE reservation_id = $1",
                &[&JOURNAL_GUARD_ID],
            )
            .map_err(|_| "Reservierungs-Sperrvermerk nicht löschbar".to_string())?;
        if removed == 1 {
            Ok(())
        } else {
            Err("Reservierungs-Sperrvermerk fehlt".to_string())
        }
    }

    fn reserved(&mut self, reservation_id: i64, caller: &str) -> Result<(), String> {
        self.client
            .execute(
                "INSERT INTO brain.steam_web_api_pending_observations (reservation_id, caller) \
                 VALUES ($1, $2)",
                &[&reservation_id, &caller],
            )
            .map(|_| ())
            .map_err(|_| "Reservierung nicht speicherbar".to_string())
    }

    fn dispatched(&mut self, reservation_id: i64) -> Result<(), String> {
        let updated = self
            .client
            .execute(
                "UPDATE brain.steam_web_api_pending_observations \
                 SET dispatch_started = true WHERE reservation_id = $1 AND NOT answered",
                &[&reservation_id],
            )
            .map_err(|_| "Steam-Aufrufbeginn nicht speicherbar".to_string())?;
        if updated == 1 {
            Ok(())
        } else {
            Err("Reservierung im Journal nicht gefunden".to_string())
        }
    }

    fn answered(&mut self, observation: &Observation) -> Result<(), String> {
        let status = observation.http_status.map(|status| status as i16);
        let updated = self
            .client
            .execute(
                "UPDATE brain.steam_web_api_pending_observations \
                 SET answered = true, http_status = $2, retry_after = $3, answered_at = now() \
                 WHERE reservation_id = $1",
                &[
                    &observation.reservation_id,
                    &status,
                    &observation.retry_after,
                ],
            )
            .map_err(|_| "Steam-Antwort nicht speicherbar".to_string())?;
        if updated == 1 {
            Ok(())
        } else {
            Err("Reservierung im Journal nicht gefunden".to_string())
        }
    }

    fn delivered(&mut self, reservation_id: i64) -> Result<(), String> {
        self.client
            .execute(
                "DELETE FROM brain.steam_web_api_pending_observations WHERE reservation_id = $1",
                &[&reservation_id],
            )
            .map(|_| ())
            .map_err(|_| "gemeldete Beobachtung nicht abschließbar".to_string())
    }
}

pub struct SteamLedger {
    base_url: String,
    token: Option<String>,
    caller: &'static str,
}

impl fmt::Debug for SteamLedger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SteamLedger")
            .field("base_url", &self.base_url)
            .field("token", &self.token.as_ref().map(|_| "<redacted>"))
            .field("caller", &self.caller)
            .finish()
    }
}

#[derive(Serialize)]
struct ReserveRequest<'a> {
    caller: &'a str,
    caller_class: &'a str,
}

#[derive(Deserialize)]
struct ReserveResponse {
    ok: bool,
    granted: bool,
    #[serde(default)]
    reservation_id: Option<i64>,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default)]
    retry_at: Option<String>,
}

#[derive(Deserialize)]
struct ObserveResponse {
    ok: bool,
}

impl SteamLedger {
    pub fn from_env(caller: &'static str) -> Self {
        let token = TOKEN_ENV_NAMES.iter().find_map(|name| {
            env::var(name)
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        });
        Self::new(LEDGER_BASE_URL, token, caller)
    }

    pub fn new(base_url: impl Into<String>, token: Option<String>, caller: &'static str) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token,
            caller,
        }
    }

    fn options(&self, attempts: usize) -> Result<HttpGetOptions, SteamWebApiError> {
        let token = self.token.as_ref().ok_or_else(|| {
            SteamWebApiError::Ledger(
                "interner Schlüssel fehlt in der Infisical-Umgebung".to_string(),
            )
        })?;
        Ok(HttpGetOptions {
            cache_ttl_seconds: None,
            timeout: LEDGER_TIMEOUT,
            headers: vec![(TOKEN_HEADER.to_string(), token.clone())],
            retry: RetryPolicy {
                attempts,
                backoff: Duration::from_millis(500),
            },
            allow_forbidden: false,
        })
    }

    fn reserve(&self, http: &HttpClient) -> Result<i64, SteamWebApiError> {
        let options = self.options(1)?;
        let response = http
            .post_json_no_redirect(
                &format!("{}/steam-web-api/reserve", self.base_url),
                &ReserveRequest {
                    caller: self.caller,
                    caller_class: CALLER_CLASS,
                },
                options,
            )
            .map_err(|error| ledger_failure("Reservierung", &error))?;
        let reply: ReserveResponse = response
            .json()
            .map_err(|_| SteamWebApiError::Ledger("Reservierung unlesbar".to_string()))?;
        match reply {
            ReserveResponse {
                ok: true,
                granted: true,
                reservation_id: Some(id),
                ..
            } if id > 0 => Ok(id),
            ReserveResponse {
                ok: true,
                granted: false,
                reason: Some(reason),
                retry_at: Some(retry_at),
                ..
            } => Err(SteamWebApiError::Denied { reason, retry_at }),
            _ => Err(SteamWebApiError::Ledger(
                "Reservierung unvollständig".to_string(),
            )),
        }
    }

    fn observe(
        &self,
        http: &HttpClient,
        observation: &Observation,
    ) -> Result<(), SteamWebApiError> {
        let options = self.options(3)?;
        let response = http
            .post_json_no_redirect(
                &format!("{}/steam-web-api/observe", self.base_url),
                observation,
                options,
            )
            .map_err(|error| ledger_failure("Beobachtung", &error))?;
        match response.json::<ObserveResponse>() {
            Ok(ObserveResponse { ok: true }) => Ok(()),
            _ => Err(SteamWebApiError::Ledger(format!(
                "Beobachtung für Reservierung {} unbestätigt",
                observation.reservation_id
            ))),
        }
    }
}

fn ledger_failure(step: &str, error: &CoreError) -> SteamWebApiError {
    match error {
        CoreError::HttpStatus { status, body, .. } => {
            let code = serde_json::from_str::<Value>(body)
                .ok()
                .and_then(|value| {
                    value
                        .get("error")
                        .and_then(Value::as_str)
                        .map(str::to_string)
                })
                .filter(|code| {
                    code.len() <= 64 && code.chars().all(|ch| ch.is_ascii_lowercase() || ch == '_')
                })
                .unwrap_or_else(|| "unbekannt".to_string());
            SteamWebApiError::Ledger(format!("{step} mit HTTP {} ({code})", status.as_u16()))
        }
        _ => SteamWebApiError::Ledger(format!("{step} nicht erreichbar")),
    }
}

pub fn flush_pending(
    http: &HttpClient,
    ledger: &SteamLedger,
    journal: &mut impl ObservationJournal,
) -> Result<usize, SteamWebApiError> {
    let pending = journal.pending().map_err(SteamWebApiError::Journal)?;
    let uncertain_reservation = pending
        .iter()
        .find(|entry| {
            entry.reservation_id != JOURNAL_GUARD_ID && entry.dispatch_started && !entry.answered
        })
        .map(|entry| entry.reservation_id);
    if pending
        .iter()
        .any(|entry| entry.reservation_id == JOURNAL_GUARD_ID)
    {
        journal.disarm().map_err(SteamWebApiError::Journal)?;
    }
    let mut delivered = 0;
    for entry in pending.iter().filter(|entry| {
        entry.reservation_id != JOURNAL_GUARD_ID && !(entry.dispatch_started && !entry.answered)
    }) {
        ledger.observe(http, &entry.observation())?;
        journal
            .delivered(entry.reservation_id)
            .map_err(SteamWebApiError::Journal)?;
        delivered += 1;
    }
    if let Some(reservation_id) = uncertain_reservation {
        return Err(SteamWebApiError::Journal(format!(
            "Ausgang der Steam-Anfrage für Reservierung {reservation_id} unbekannt; vor weiteren Aufrufen manuell klären"
        )));
    }
    Ok(delivered)
}

pub fn fetch(
    http: &HttpClient,
    ledger: &SteamLedger,
    journal: &mut impl ObservationJournal,
    url: &str,
    options: &HttpGetOptions,
) -> Result<HttpResult, SteamWebApiError> {
    journal.lock().map_err(SteamWebApiError::Journal)?;
    flush_pending(http, ledger, journal)?;
    if let Some(ttl) = options.cache_ttl_seconds {
        if let Some(cached) = http
            .cached_result(url, ttl)
            .map_err(|_| SteamWebApiError::Local("Cache nicht lesbar".to_string()))?
        {
            return Ok(cached);
        }
    }

    journal.arm().map_err(SteamWebApiError::Journal)?;
    let reservation_id = match ledger.reserve(http) {
        Ok(id) => id,
        Err(error) => {
            journal.disarm().map_err(SteamWebApiError::Journal)?;
            return Err(error);
        }
    };
    if let Err(message) = journal.reserved(reservation_id, ledger.caller) {
        let released = ledger.observe(http, &Observation::transport_failure(reservation_id));
        return Err(match released {
            Ok(()) => {
                journal.disarm().map_err(SteamWebApiError::Journal)?;
                SteamWebApiError::Journal(message)
            }
            Err(error) => error,
        });
    }
    journal.disarm().map_err(SteamWebApiError::Journal)?;

    journal
        .dispatched(reservation_id)
        .map_err(SteamWebApiError::Journal)?;
    let attempt = http.get_single_attempt(url, options);
    let observation = Observation::from_attempt(reservation_id, &attempt);
    let answered = journal.answered(&observation);
    if let Err(error) = ledger.observe(http, &observation) {
        return Err(match answered {
            Ok(()) => error,
            Err(message) => SteamWebApiError::Journal(format!(
                "{message}; Beobachtung für Reservierung {reservation_id} offen"
            )),
        });
    }
    if let Err(message) = answered {
        journal
            .delivered(reservation_id)
            .map_err(SteamWebApiError::Journal)?;
        return Err(SteamWebApiError::Journal(message));
    }
    journal
        .delivered(reservation_id)
        .map_err(SteamWebApiError::Journal)?;

    let attempt = attempt.map_err(|_| SteamWebApiError::Transport)?;
    if attempt.status.as_u16() == 429 {
        return Err(SteamWebApiError::RateLimited {
            retry_after: attempt.retry_after,
        });
    }
    if !attempt.status.is_success() {
        return Err(SteamWebApiError::Upstream {
            status: attempt.status.as_u16(),
        });
    }
    let result = attempt.result.ok_or(SteamWebApiError::Transport)?;
    if options.cache_ttl_seconds.is_some() {
        http.write_cache(&result)
            .map_err(|_| SteamWebApiError::Local("Cache nicht schreibbar".to_string()))?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{ErrorKind, Read, Write},
        net::{TcpListener, TcpStream},
        thread::{self, JoinHandle},
        time::Instant,
    };

    #[derive(Default)]
    struct MemoryJournal {
        rows: Vec<PendingObservation>,
        fail_reserved: bool,
        fail_answered: bool,
    }

    impl ObservationJournal for MemoryJournal {
        fn lock(&mut self) -> Result<(), String> {
            Ok(())
        }

        fn pending(&mut self) -> Result<Vec<PendingObservation>, String> {
            Ok(self.rows.clone())
        }

        fn arm(&mut self) -> Result<(), String> {
            if self
                .rows
                .iter()
                .any(|row| row.reservation_id == JOURNAL_GUARD_ID)
            {
                return Err("guard already armed".to_string());
            }
            self.rows.push(PendingObservation {
                reservation_id: JOURNAL_GUARD_ID,
                dispatch_started: true,
                answered: false,
                http_status: None,
                retry_after: None,
            });
            Ok(())
        }

        fn disarm(&mut self) -> Result<(), String> {
            let index = self
                .rows
                .iter()
                .position(|row| row.reservation_id == JOURNAL_GUARD_ID)
                .ok_or_else(|| "guard missing".to_string())?;
            self.rows.remove(index);
            Ok(())
        }

        fn reserved(&mut self, reservation_id: i64, _caller: &str) -> Result<(), String> {
            if self.fail_reserved {
                return Err("journal down".to_string());
            }
            self.rows.push(PendingObservation {
                reservation_id,
                dispatch_started: false,
                answered: false,
                http_status: None,
                retry_after: None,
            });
            Ok(())
        }

        fn dispatched(&mut self, reservation_id: i64) -> Result<(), String> {
            let row = self
                .rows
                .iter_mut()
                .find(|row| row.reservation_id == reservation_id)
                .ok_or_else(|| "missing".to_string())?;
            row.dispatch_started = true;
            Ok(())
        }

        fn answered(&mut self, observation: &Observation) -> Result<(), String> {
            if self.fail_answered {
                return Err("journal down".to_string());
            }
            let row = self
                .rows
                .iter_mut()
                .find(|row| row.reservation_id == observation.reservation_id)
                .ok_or_else(|| "missing".to_string())?;
            row.answered = true;
            row.http_status = observation.http_status;
            row.retry_after = observation.retry_after.clone();
            Ok(())
        }

        fn delivered(&mut self, reservation_id: i64) -> Result<(), String> {
            self.rows.retain(|row| row.reservation_id != reservation_id);
            Ok(())
        }
    }

    struct Recorded {
        head: String,
        body: String,
    }

    struct Mock {
        url: String,
        handle: JoinHandle<Vec<Recorded>>,
    }

    impl Mock {
        fn start(responses: Vec<String>) -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            listener.set_nonblocking(true).unwrap();
            let url = format!("http://{}", listener.local_addr().unwrap());
            let handle = thread::spawn(move || {
                let mut recorded = Vec::new();
                let deadline = Instant::now() + Duration::from_secs(5);
                let mut responses = responses.into_iter();
                let mut next = responses.next();
                while let Some(response) = next.take() {
                    match listener.accept() {
                        Ok((mut stream, _)) => {
                            stream.set_nonblocking(false).unwrap();
                            recorded.push(read_request(&mut stream));
                            stream.write_all(response.as_bytes()).unwrap();
                            next = responses.next();
                        }
                        Err(error) if error.kind() == ErrorKind::WouldBlock => {
                            if Instant::now() > deadline {
                                break;
                            }
                            next = Some(response);
                            thread::sleep(Duration::from_millis(5));
                        }
                        Err(error) => panic!("{error}"),
                    }
                }
                recorded
            });
            Self { url, handle }
        }

        fn finish(self) -> Vec<Recorded> {
            self.handle.join().unwrap()
        }
    }

    fn read_request(stream: &mut TcpStream) -> Recorded {
        let mut request = Vec::new();
        let mut buffer = [0_u8; 1024];
        let header_end = loop {
            let read = stream.read(&mut buffer).unwrap();
            assert!(read > 0);
            request.extend_from_slice(&buffer[..read]);
            if let Some(index) = request.windows(4).position(|window| window == b"\r\n\r\n") {
                break index + 4;
            }
        };
        let head = String::from_utf8(request[..header_end].to_vec()).unwrap();
        let length = head
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().unwrap())
            })
            .unwrap_or(0);
        while request.len() < header_end + length {
            let read = stream.read(&mut buffer).unwrap();
            assert!(read > 0);
            request.extend_from_slice(&buffer[..read]);
        }
        Recorded {
            head,
            body: String::from_utf8(request[header_end..header_end + length].to_vec()).unwrap(),
        }
    }

    fn reply(status: &str, extra: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\n{extra}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    fn granted(id: i64) -> String {
        reply(
            "200 OK",
            "",
            &format!(
                r#"{{"ok":true,"granted":true,"reservation_id":{id},"reserved_at":"2026-09-29T12:00:00Z"}}"#
            ),
        )
    }

    fn observed(duplicate: bool) -> String {
        reply(
            "200 OK",
            "",
            &format!(
                r#"{{"ok":true,"response_at":"2026-09-29T12:00:01Z","cooldown_until":null,"duplicate":{duplicate}}}"#
            ),
        )
    }

    fn json(body: &str) -> Value {
        serde_json::from_str(body).unwrap()
    }

    fn client() -> (HttpClient, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        (
            HttpClient::new("deadlock-brain-test", dir.path()).unwrap(),
            dir,
        )
    }

    fn ledger(url: &str) -> SteamLedger {
        SteamLedger::new(url, Some("test-key".to_string()), "deadlock-brain-test")
    }

    fn steam_options() -> HttpGetOptions {
        HttpGetOptions {
            cache_ttl_seconds: Some(900),
            timeout: Duration::from_secs(5),
            ..HttpGetOptions::default()
        }
    }

    fn closed_url() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        drop(listener);
        url
    }

    #[test]
    fn granted_reservation_dispatches_once_and_reports_status() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(41), observed(false)]);
        let steam = Mock::start(vec![reply("200 OK", "", r#"{"appnews":{"newsitems":[]}}"#)]);
        let mut journal = MemoryJournal::default();

        let result = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap();

        assert!(result.text().contains("appnews"));
        assert_eq!(steam.finish().len(), 1);
        let calls = ledger_mock.finish();
        assert_eq!(calls.len(), 2);
        assert!(calls[0].head.starts_with("POST /steam-web-api/reserve "));
        assert!(calls[0]
            .head
            .lines()
            .any(|line| line.eq_ignore_ascii_case("x-internal-token: test-key")));
        assert_eq!(
            json(&calls[0].body),
            json(r#"{"caller":"deadlock-brain-test","caller_class":"standard"}"#)
        );
        assert!(calls[1].head.starts_with("POST /steam-web-api/observe "));
        assert_eq!(
            json(&calls[1].body),
            json(r#"{"reservation_id":41,"http_status":200}"#)
        );
        assert!(journal.rows.is_empty());
    }

    #[test]
    fn cache_hit_needs_no_reservation() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(1), observed(false)]);
        let steam = Mock::start(vec![reply("200 OK", "", r#"{"cached":true}"#)]);
        let url = format!("{}/news", steam.url);
        let mut journal = MemoryJournal::default();
        fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &url,
            &steam_options(),
        )
        .unwrap();

        let unused = closed_url();
        let cached = fetch(
            &http,
            &ledger(&unused),
            &mut journal,
            &url,
            &steam_options(),
        )
        .unwrap();

        assert!(cached.from_cache);
        assert_eq!(steam.finish().len(), 1);
        assert_eq!(ledger_mock.finish().len(), 2);
    }

    #[test]
    fn denial_prevents_dispatch_and_keeps_retry_at() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![reply(
            "200 OK",
            "",
            r#"{"ok":true,"granted":false,"reason":"cooldown","retry_at":"2026-09-29T12:01:30Z"}"#,
        )]);
        let steam = Mock::start(Vec::new());
        let mut journal = MemoryJournal::default();

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert_eq!(
            error,
            SteamWebApiError::Denied {
                reason: "cooldown".to_string(),
                retry_at: "2026-09-29T12:01:30Z".to_string()
            }
        );
        assert!(error.blocks_fallback());
        assert!(steam.finish().is_empty());
        assert_eq!(ledger_mock.finish().len(), 1);
        assert!(journal.rows.is_empty());
    }

    #[test]
    fn rate_limit_reports_original_retry_after() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(7), observed(false)]);
        let steam = Mock::start(vec![reply(
            "429 Too Many Requests",
            "Retry-After: Tue, 29 Sep 2026 12:05:00 GMT\r\n",
            "",
        )]);
        let mut journal = MemoryJournal::default();

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert_eq!(
            error,
            SteamWebApiError::RateLimited {
                retry_after: Some("Tue, 29 Sep 2026 12:05:00 GMT".to_string())
            }
        );
        assert!(error.blocks_fallback());
        assert_eq!(steam.finish().len(), 1);
        let calls = ledger_mock.finish();
        assert_eq!(
            json(&calls[1].body),
            json(
                r#"{"reservation_id":7,"http_status":429,"retry_after":"Tue, 29 Sep 2026 12:05:00 GMT"}"#
            )
        );
        assert!(journal.rows.is_empty());
        assert!(std::fs::read_dir(_dir.path()).unwrap().next().is_none());
    }

    #[test]
    fn truncated_rate_limit_body_still_reports_429() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(72), observed(false)]);
        let steam = Mock::start(vec![
            "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 90\r\nContent-Length: 20\r\nConnection: close\r\n\r\nshort".to_string(),
        ]);
        let mut journal = MemoryJournal::default();

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert_eq!(
            error,
            SteamWebApiError::RateLimited {
                retry_after: Some("90".to_string())
            }
        );
        assert_eq!(steam.finish().len(), 1);
        assert_eq!(
            json(&ledger_mock.finish()[1].body),
            json(r#"{"reservation_id":72,"http_status":429,"retry_after":"90"}"#)
        );
    }

    #[test]
    fn redirect_does_not_dispatch_an_unreserved_second_request() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(73), observed(false)]);
        let destination = Mock::start(Vec::new());
        let steam = Mock::start(vec![reply(
            "302 Found",
            &format!("Location: {}/other\r\n", destination.url),
            "",
        )]);
        let mut journal = MemoryJournal::default();

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert_eq!(error, SteamWebApiError::Upstream { status: 302 });
        assert_eq!(steam.finish().len(), 1);
        assert!(destination.finish().is_empty());
        assert_eq!(
            json(&ledger_mock.finish()[1].body),
            json(r#"{"reservation_id":73,"http_status":302}"#)
        );
    }

    #[test]
    fn server_error_reports_status_without_retry_after() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(8), observed(false)]);
        let steam = Mock::start(vec![reply(
            "503 Service Unavailable",
            "Retry-After: 5\r\n",
            "",
        )]);
        let mut journal = MemoryJournal::default();

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert_eq!(error, SteamWebApiError::Upstream { status: 503 });
        assert!(!error.blocks_fallback());
        assert_eq!(steam.finish().len(), 1);
        assert_eq!(
            json(&ledger_mock.finish()[1].body),
            json(r#"{"reservation_id":8,"http_status":503}"#)
        );
    }

    #[test]
    fn transport_failure_reports_null_status() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(9), observed(false)]);
        let mut journal = MemoryJournal::default();

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", closed_url()),
            &steam_options(),
        )
        .unwrap_err();

        assert_eq!(error, SteamWebApiError::Transport);
        assert_eq!(
            json(&ledger_mock.finish()[1].body),
            json(r#"{"reservation_id":9,"http_status":null}"#)
        );
        assert!(journal.rows.is_empty());
    }

    #[test]
    fn unreachable_ledger_blocks_dispatch() {
        let (http, _dir) = client();
        let steam = Mock::start(Vec::new());
        let mut journal = MemoryJournal::default();

        let error = fetch(
            &http,
            &ledger(&closed_url()),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert!(matches!(error, SteamWebApiError::Ledger(_)));
        assert!(steam.finish().is_empty());
    }

    #[test]
    fn ledger_redirect_does_not_forward_the_internal_key() {
        let (http, _dir) = client();
        let destination = Mock::start(Vec::new());
        let ledger_mock = Mock::start(vec![reply(
            "302 Found",
            &format!("Location: {}/other\r\n", destination.url),
            "",
        )]);
        let steam = Mock::start(Vec::new());
        let mut journal = MemoryJournal::default();

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert_eq!(
            error,
            SteamWebApiError::Ledger("Reservierung mit HTTP 302 (unbekannt)".to_string())
        );
        assert_eq!(ledger_mock.finish().len(), 1);
        assert!(destination.finish().is_empty());
        assert!(steam.finish().is_empty());
    }

    #[test]
    fn ledger_outage_status_and_missing_key_block_dispatch() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![reply(
            "503 Service Unavailable",
            "",
            r#"{"ok":false,"error":"ledger_unavailable"}"#,
        )]);
        let steam = Mock::start(Vec::new());
        let url = format!("{}/news", steam.url);
        let mut journal = MemoryJournal::default();

        let outage = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &url,
            &steam_options(),
        )
        .unwrap_err();
        let keyless = SteamLedger::new(closed_url(), None, "deadlock-brain-test");
        let missing = fetch(&http, &keyless, &mut journal, &url, &steam_options()).unwrap_err();

        assert_eq!(
            outage,
            SteamWebApiError::Ledger("Reservierung mit HTTP 503 (ledger_unavailable)".to_string())
        );
        assert!(matches!(missing, SteamWebApiError::Ledger(_)));
        assert!(!format!("{missing} {outage}").contains("test-key"));
        assert_eq!(ledger_mock.finish().len(), 1);
        assert!(steam.finish().is_empty());
    }

    #[test]
    fn lost_observation_is_retried_before_next_reservation() {
        let (http, _dir) = client();
        let failing = Mock::start(vec![granted(11)]);
        let steam = Mock::start(vec![
            reply("200 OK", "", r#"{"first":true}"#),
            reply("200 OK", "", r#"{"second":true}"#),
        ]);
        let mut journal = MemoryJournal::default();
        let first = fetch(
            &http,
            &ledger(&failing.url),
            &mut journal,
            &format!("{}/first", steam.url),
            &steam_options(),
        )
        .unwrap_err();
        assert_eq!(
            first,
            SteamWebApiError::Ledger("Beobachtung nicht erreichbar".to_string())
        );
        assert_eq!(failing.finish().len(), 1);
        assert_eq!(
            journal.rows,
            vec![PendingObservation {
                reservation_id: 11,
                dispatch_started: true,
                answered: true,
                http_status: Some(200),
                retry_after: None
            }]
        );

        let recovering = Mock::start(vec![observed(true), granted(12), observed(false)]);
        fetch(
            &http,
            &ledger(&recovering.url),
            &mut journal,
            &format!("{}/second", steam.url),
            &steam_options(),
        )
        .unwrap();

        let calls = recovering.finish();
        assert!(calls[0].head.starts_with("POST /steam-web-api/observe "));
        assert_eq!(
            json(&calls[0].body),
            json(r#"{"reservation_id":11,"http_status":200}"#)
        );
        assert!(calls[1].head.starts_with("POST /steam-web-api/reserve "));
        assert_eq!(steam.finish().len(), 2);
        assert!(journal.rows.is_empty());
    }

    #[test]
    fn interrupted_request_blocks_restart_even_with_a_cache_hit() {
        let (http, _dir) = client();
        let initial_ledger = Mock::start(vec![granted(19), observed(false)]);
        let steam = Mock::start(vec![reply("200 OK", "", r#"{"cached":true}"#)]);
        let url = format!("{}/news", steam.url);
        let mut journal = MemoryJournal::default();
        fetch(
            &http,
            &ledger(&initial_ledger.url),
            &mut journal,
            &url,
            &steam_options(),
        )
        .unwrap();
        assert_eq!(initial_ledger.finish().len(), 2);
        assert_eq!(steam.finish().len(), 1);
        journal.rows.push(PendingObservation {
            reservation_id: 20,
            dispatch_started: true,
            answered: false,
            http_status: None,
            retry_after: None,
        });

        let error = fetch(
            &http,
            &ledger(&closed_url()),
            &mut journal,
            &url,
            &steam_options(),
        )
        .unwrap_err();

        assert!(matches!(error, SteamWebApiError::Journal(_)));
        assert_eq!(journal.rows.len(), 1);
    }

    #[test]
    fn unanswered_reservation_after_restart_is_reported_as_transport_failure() {
        let (http, _dir) = client();
        let mut journal = MemoryJournal {
            rows: vec![PendingObservation {
                reservation_id: 20,
                dispatch_started: false,
                answered: false,
                http_status: None,
                retry_after: None,
            }],
            ..MemoryJournal::default()
        };
        let conflict = Mock::start(vec![reply(
            "409 Conflict",
            "",
            r#"{"ok":false,"error":"conflicting_report"}"#,
        )]);
        let steam = Mock::start(Vec::new());

        let error = fetch(
            &http,
            &ledger(&conflict.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        let calls = conflict.finish();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            json(&calls[0].body),
            json(r#"{"reservation_id":20,"http_status":null}"#)
        );
        assert_eq!(
            error,
            SteamWebApiError::Ledger("Beobachtung mit HTTP 409 (conflicting_report)".to_string())
        );
        assert!(steam.finish().is_empty());
        assert_eq!(journal.rows.len(), 1);
    }

    #[test]
    fn releases_unused_reservations_before_blocking_uncertain_request() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![observed(false)]);
        let steam = Mock::start(Vec::new());
        let mut journal = MemoryJournal {
            rows: vec![
                PendingObservation {
                    reservation_id: 19,
                    dispatch_started: false,
                    answered: false,
                    http_status: None,
                    retry_after: None,
                },
                PendingObservation {
                    reservation_id: 20,
                    dispatch_started: true,
                    answered: false,
                    http_status: None,
                    retry_after: None,
                },
            ],
            ..MemoryJournal::default()
        };

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert!(matches!(error, SteamWebApiError::Journal(message) if message.contains("20")));
        let calls = ledger_mock.finish();
        assert_eq!(calls.len(), 1);
        assert!(calls[0].head.starts_with("POST /steam-web-api/observe "));
        assert_eq!(
            json(&calls[0].body),
            json(r#"{"reservation_id":19,"http_status":null}"#)
        );
        assert_eq!(
            journal.rows,
            vec![PendingObservation {
                reservation_id: 20,
                dispatch_started: true,
                answered: false,
                http_status: None,
                retry_after: None,
            }]
        );
        assert!(steam.finish().is_empty());
    }

    #[test]
    #[ignore = "needs scratch Postgres via DEADLOCK_BRAIN_SCRATCH_DSN"]
    fn pg_journal_survives_reconnect_and_serializes_runs() {
        let Ok(dsn) = env::var("DEADLOCK_BRAIN_SCRATCH_DSN") else {
            eprintln!("skip: DEADLOCK_BRAIN_SCRATCH_DSN fehlt");
            return;
        };
        let mut first = Client::connect(&dsn, postgres::NoTls).unwrap();
        first
            .batch_execute(include_str!(
                "../../../../scripts/migrations/2026-09-29-steam-web-api-journal.sql"
            ))
            .unwrap();
        first
            .batch_execute(
                "ALTER TABLE brain.steam_web_api_pending_observations DROP COLUMN dispatch_started",
            )
            .unwrap();
        first
            .execute(
                "INSERT INTO brain.steam_web_api_pending_observations (reservation_id, caller) VALUES (-9, 'deadlock-brain-test')",
                &[],
            )
            .unwrap();
        first
            .batch_execute(include_str!(
                "../../../../scripts/migrations/2026-09-29-steam-web-api-journal.sql"
            ))
            .unwrap();
        let old_pending: bool = first
            .query_one(
                "SELECT dispatch_started FROM brain.steam_web_api_pending_observations WHERE reservation_id = -9",
                &[],
            )
            .unwrap()
            .get(0);
        assert!(old_pending);
        first
            .execute(
                "DELETE FROM brain.steam_web_api_pending_observations WHERE reservation_id = -9",
                &[],
            )
            .unwrap();
        first
            .execute(
                "DELETE FROM brain.steam_web_api_pending_observations WHERE reservation_id < 0",
                &[],
            )
            .unwrap();
        let mut second = Client::connect(&dsn, postgres::NoTls).unwrap();
        {
            let mut journal = PgObservationJournal::open(&mut first).unwrap();
            journal.lock().unwrap();
            let busy: bool = second
                .query_one("SELECT pg_try_advisory_lock($1)", &[&JOURNAL_LOCK_KEY])
                .unwrap()
                .get(0);
            assert!(!busy);
            journal.reserved(-1, "deadlock-brain-test").unwrap();
            journal.reserved(-2, "deadlock-brain-test").unwrap();
            journal.dispatched(-2).unwrap();
            journal.reserved(-3, "deadlock-brain-test").unwrap();
            journal.dispatched(-3).unwrap();
            journal
                .answered(&Observation {
                    reservation_id: -2,
                    http_status: Some(429),
                    retry_after: Some("90".to_string()),
                })
                .unwrap();
            assert!(journal
                .answered(&Observation::transport_failure(-4))
                .is_err());
        }
        let unlocked: bool = second
            .query_one("SELECT pg_try_advisory_lock($1)", &[&JOURNAL_LOCK_KEY])
            .unwrap()
            .get(0);
        assert!(unlocked);
        second
            .query_one("SELECT pg_advisory_unlock($1)", &[&JOURNAL_LOCK_KEY])
            .unwrap();
        drop(first);

        let mut journal = PgObservationJournal::open(&mut second).unwrap();
        journal.lock().unwrap();
        let pending: Vec<_> = journal
            .pending()
            .unwrap()
            .into_iter()
            .filter(|row| row.reservation_id < 0)
            .collect();
        assert_eq!(
            pending
                .iter()
                .map(|row| (row.reservation_id, row.dispatch_started, row.answered))
                .collect::<Vec<_>>(),
            vec![(-3, true, false), (-2, true, true), (-1, false, false)]
        );
        assert_eq!(
            pending
                .iter()
                .filter(|row| row.reservation_id != -3)
                .map(PendingObservation::observation)
                .collect::<Vec<_>>(),
            vec![
                Observation {
                    reservation_id: -2,
                    http_status: Some(429),
                    retry_after: Some("90".to_string()),
                },
                Observation::transport_failure(-1),
            ]
        );
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![observed(false), observed(false)]);
        let error = flush_pending(&http, &ledger(&ledger_mock.url), &mut journal).unwrap_err();
        assert!(matches!(error, SteamWebApiError::Journal(_)));
        let calls = ledger_mock.finish();
        assert_eq!(calls.len(), 2);
        assert!(calls
            .iter()
            .all(|call| call.head.starts_with("POST /steam-web-api/observe ")));
        journal.delivered(-1).unwrap();
        journal.delivered(-2).unwrap();
        journal.delivered(-3).unwrap();
        assert!(journal
            .pending()
            .unwrap()
            .iter()
            .all(|row| row.reservation_id >= 0));
        journal.arm().unwrap();
        drop(journal);
        drop(second);

        let mut restarted = Client::connect(&dsn, postgres::NoTls).unwrap();
        let mut journal = PgObservationJournal::open(&mut restarted).unwrap();
        journal.lock().unwrap();
        assert_eq!(
            flush_pending(&http, &ledger(&closed_url()), &mut journal).unwrap(),
            0
        );
        assert!(journal
            .pending()
            .unwrap()
            .iter()
            .all(|row| row.reservation_id != JOURNAL_GUARD_ID));
    }

    #[test]
    fn failed_journal_answer_is_not_reported_as_success() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(31), observed(false)]);
        let steam = Mock::start(vec![reply("200 OK", "", r#"{"news":true}"#)]);
        let mut journal = MemoryJournal {
            fail_answered: true,
            ..MemoryJournal::default()
        };

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert_eq!(error, SteamWebApiError::Journal("journal down".to_string()));
        assert!(journal.rows.is_empty());
        assert_eq!(steam.finish().len(), 1);
        assert_eq!(
            json(&ledger_mock.finish()[1].body),
            json(r#"{"reservation_id":31,"http_status":200}"#)
        );
    }

    #[test]
    fn restart_guard_replays_known_reservation_before_next_request() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![observed(false), granted(41), observed(false)]);
        let steam = Mock::start(vec![reply("200 OK", "", r#"{"appnews":{}}"#)]);
        let mut journal = MemoryJournal::default();
        journal.arm().unwrap();
        journal.reserved(40, "deadlock-brain-test").unwrap();

        fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap();

        let calls = ledger_mock.finish();
        assert_eq!(
            json(&calls[0].body),
            json(r#"{"reservation_id":40,"http_status":null}"#)
        );
        assert!(calls[1].head.starts_with("POST /steam-web-api/reserve "));
        assert_eq!(steam.finish().len(), 1);
        assert!(journal.rows.is_empty());
    }

    #[test]
    fn failed_reservation_write_and_observation_recover_unused_guard() {
        let (http, _dir) = client();
        let failing = Mock::start(vec![granted(32)]);
        let steam = Mock::start(Vec::new());
        let url = format!("{}/news", steam.url);
        let mut journal = MemoryJournal {
            fail_reserved: true,
            ..MemoryJournal::default()
        };

        let error = fetch(
            &http,
            &ledger(&failing.url),
            &mut journal,
            &url,
            &steam_options(),
        )
        .unwrap_err();

        assert!(matches!(error, SteamWebApiError::Ledger(_)));
        assert_eq!(failing.finish().len(), 1);
        assert!(steam.finish().is_empty());
        assert_eq!(journal.rows.len(), 1);
        assert_eq!(journal.rows[0].reservation_id, JOURNAL_GUARD_ID);

        let after_restart = fetch(
            &http,
            &ledger(&closed_url()),
            &mut journal,
            &url,
            &steam_options(),
        )
        .unwrap_err();
        assert!(matches!(after_restart, SteamWebApiError::Ledger(_)));
        assert!(journal.rows.is_empty());
    }

    #[test]
    fn journal_failure_releases_reservation_without_dispatch() {
        let (http, _dir) = client();
        let ledger_mock = Mock::start(vec![granted(30), observed(false)]);
        let steam = Mock::start(Vec::new());
        let mut journal = MemoryJournal {
            fail_reserved: true,
            ..MemoryJournal::default()
        };

        let error = fetch(
            &http,
            &ledger(&ledger_mock.url),
            &mut journal,
            &format!("{}/news", steam.url),
            &steam_options(),
        )
        .unwrap_err();

        assert!(matches!(error, SteamWebApiError::Journal(_)));
        assert!(steam.finish().is_empty());
        assert_eq!(
            json(&ledger_mock.finish()[1].body),
            json(r#"{"reservation_id":30,"http_status":null}"#)
        );
    }
}
