use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fmt, fs,
    io::{BufRead, Read, Write},
    os::unix::{
        io::{AsRawFd, FromRawFd},
        net::UnixStream,
        process::CommandExt,
    },
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeminiErrorKind {
    NotLoggedIn,
    RateLimited,
    ProfileLocked,
    Refused,
    Timeout,
    Unknown,
}
impl GeminiErrorKind {
    pub fn pauses_run(self) -> bool {
        matches!(
            self,
            Self::NotLoggedIn | Self::RateLimited | Self::ProfileLocked
        )
    }
}
impl fmt::Display for GeminiErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NotLoggedIn => "not_logged_in",
            Self::RateLimited => "rate_limited",
            Self::ProfileLocked => "profile_locked",
            Self::Refused => "refused",
            Self::Timeout => "timeout",
            Self::Unknown => "unknown",
        })
    }
}
#[derive(Debug, thiserror::Error)]
#[error("{kind}: {message}")]
pub struct GeminiError {
    pub kind: GeminiErrorKind,
    pub message: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    #[serde(default)]
    learning_enabled: bool,
    account_id: String,
    infisical_config: PathBuf,
    #[serde(default = "key_name")]
    key_secret: String,
    #[serde(default = "browser_path")]
    browser_path: PathBuf,
    #[serde(default = "response_timeout")]
    response_timeout_seconds: u64,
    #[serde(default)]
    alert_webhook_secret: Option<String>,
}
fn key_name() -> String {
    "DB_MASTER_KEY_V1".into()
}
fn browser_path() -> PathBuf {
    "/opt/brave.com/brave/brave".into()
}
fn response_timeout() -> u64 {
    300
}
pub fn learning_enabled() -> Result<bool> {
    Ok(config()?.learning_enabled)
}
pub(crate) fn infisical_config() -> Result<PathBuf> {
    Ok(config()?.infisical_config)
}
fn config() -> Result<Config> {
    let mut c: Config = serde_json::from_slice(&fs::read(
        crate::db::repo_root().join("config/gemini-browser.json"),
    )?)?;
    if !dbrain_session_store::valid_account(&c.account_id)
        || c.response_timeout_seconds == 0
        || c.response_timeout_seconds > 900
    {
        bail!("invalid configuration");
    }
    if c.infisical_config.is_relative() {
        c.infisical_config = crate::db::repo_root().join(&c.infisical_config);
    }
    Ok(c)
}
const GEMINI_URL: &str = "https://gemini.google.com/app";
// State JSON is quoted as a JS string and once more inside the CDP envelope.
// Each quoting step is at most 2x for already-valid JSON. Keep bounded room
// for the restore code, session ID and protocol metadata as well as 1 MiB state.
const CDP_REQUEST_LIMIT: usize = 4 * dbrain_session_store::LIMIT + 64 * 1024;
const CONVERT:&str="Wandle die oben genannten Erkenntnisse in genau dieses JSON um. Gib NUR das JSON aus, ohne Markdown, ohne Codeblock, ohne weiteren Text:\n{\"claims\": [{\"entity\": \"Hero, Item oder Fähigkeit\", \"claim_type\": \"build|item_timing|matchup|mechanic|combo|meta\", \"assertion\": \"klarer deutscher Satz\", \"patch_context\": null, \"confidence\": 0.0}]}\nEnthält die Analyse kein konkretes Deadlock-Gameplay-Wissen, gib {\"claims\": []} zurück.";

// Chromium's private NUL-framed CDP pipe. No listening debug port, no persisted
// credential profile: all navigation takes place in an incognito context.
struct Browser {
    child: Child,
    pipe: UnixStream,
    reader: std::io::BufReader<UnixStream>,
    id: u64,
    session: String,
    context: String,
    origins: Vec<String>,
    _scratch: tempfile::TempDir,
}
impl Drop for Browser {
    fn drop(&mut self) {
        // SAFETY: process_group(0) creates the spawned child's own PGID equal
        // to its PID. This Child has not been waited/reaped before Drop, so its
        // PID cannot have been reused for an unrelated process group. Only this
        // browser group is signalled; Child::wait below reaps its leader.
        unsafe {
            libc::kill(-(self.child.id() as i32), libc::SIGKILL);
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
impl Browser {
    fn open(path: &PathBuf) -> Result<Self> {
        let (parent, child) = UnixStream::pair()?;
        parent.set_read_timeout(Some(Duration::from_secs(45)))?;
        parent.set_write_timeout(Some(Duration::from_secs(45)))?;
        // Keep source descriptors above Chromium's fixed descriptors, avoiding dup collisions.
        // SAFETY: child owns a live UnixStream FD throughout fcntl. F_DUPFD_CLOEXEC
        // creates a distinct descriptor without changing the stream's ownership.
        let input = unsafe { libc::fcntl(child.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 10) };
        if input < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        // SAFETY: fcntl succeeded (nonnegative FD checked above) and returned a
        // fresh descriptor owned exclusively here. No other owner closes it;
        // ownership is transferred exactly once into OwnedFd.
        let input = unsafe { std::os::fd::OwnedFd::from_raw_fd(input) };
        let scratch = tempfile::tempdir()?;
        let mut command = Command::new(path);
        command.process_group(0);
        command
            .args([
                "--remote-debugging-pipe",
                "--no-first-run",
                "--no-default-browser-check",
                "--incognito",
            ])
            .arg(format!("--user-data-dir={}", scratch.path().display()))
            .arg("about:blank")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        // SAFETY: after fork the callback only borrows the captured live FD and
        // uses async-signal-safe dup2 calls, or obtains errno via last_os_error.
        // It allocates nothing and takes no locks. The input FD is >=10, so the
        // fixed child-only FD3/FD4 targets cannot clobber it before the second dup.
        unsafe {
            command.pre_exec(move || {
                if libc::dup2(input.as_raw_fd(), 3) < 0 || libc::dup2(input.as_raw_fd(), 4) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        // Complete fallible descriptor preparation before spawning. After spawn
        // Self construction is infallible and Drop owns every failure in CDP init.
        let reader = std::io::BufReader::new(parent.try_clone()?);
        let process = command.spawn()?;
        drop(child);
        let mut b = Self {
            child: process,
            reader,
            pipe: parent,
            id: 0,
            session: String::new(),
            context: String::new(),
            origins: Vec::new(),
            _scratch: scratch,
        };
        b.context = b.call(
            "Target.createBrowserContext",
            json!({"disposeOnDetach":true}),
            false,
        )?["browserContextId"]
            .as_str()
            .context("context missing")?
            .into();
        let target = b.call(
            "Target.createTarget",
            json!({"url":"about:blank","browserContextId":b.context}),
            false,
        )?["targetId"]
            .as_str()
            .context("target missing")?
            .to_owned();
        b.session = b.call(
            "Target.attachToTarget",
            json!({"targetId":target,"flatten":true}),
            false,
        )?["sessionId"]
            .as_str()
            .context("session missing")?
            .into();
        b.call("Page.enable", json!({}), true)?;
        b.call("Runtime.enable", json!({}), true)?;
        Ok(b)
    }
    fn call(&mut self, method: &str, params: Value, page: bool) -> Result<Value> {
        self.id += 1;
        let id = self.id;
        let mut request = json!({"id":id,"method":method,"params":params});
        if page {
            request["sessionId"] = json!(self.session);
        }
        let mut raw = serde_json::to_vec(&request)?;
        if raw.len() > CDP_REQUEST_LIMIT {
            bail!("request too large");
        }
        raw.push(0);
        self.pipe.write_all(&raw)?;
        let deadline = Instant::now() + Duration::from_secs(45);
        loop {
            let mut frame = Vec::new();
            loop {
                self.reader.get_mut().set_read_timeout(Some(
                    deadline
                        .saturating_duration_since(Instant::now())
                        .max(Duration::from_millis(1)),
                ))?;
                let buf = self.reader.fill_buf()?;
                if buf.is_empty() {
                    bail!("browser pipe closed");
                }
                let delimiter = buf.iter().position(|v| *v == 0);
                let count = delimiter.unwrap_or(buf.len());
                if frame.len().saturating_add(count) > 4 * 1024 * 1024 {
                    bail!("response too large");
                }
                frame.extend_from_slice(&buf[..count]);
                self.reader
                    .consume(count + usize::from(delimiter.is_some()));
                if delimiter.is_some() {
                    break;
                }
            }
            let response: Value = serde_json::from_slice(&frame)?;
            if response["id"].as_u64() == Some(id) {
                if response.get("error").is_some() {
                    bail!("browser command failed");
                }
                return Ok(response["result"].clone());
            }
            if Instant::now() >= deadline {
                bail!("browser timeout");
            }
        }
    }
    fn eval(&mut self, expression: &str) -> Result<Value> {
        let r = self.call(
            "Runtime.evaluate",
            json!({"expression":expression,"awaitPromise":true,"returnByValue":true}),
            true,
        )?;
        if r.get("exceptionDetails").is_some() {
            bail!("browser evaluation failed");
        }
        Ok(r["result"]["value"].clone())
    }
    fn navigate(&mut self, url: &str) -> Result<()> {
        let r = self.call("Page.navigate", json!({"url":url}), true)?;
        if r.get("errorText").is_some() {
            bail!("navigation failed");
        }
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(45) {
            std::thread::sleep(Duration::from_millis(250));
            if self
                .eval("document.readyState")?
                .as_str()
                .is_some_and(|v| v == "complete" || v == "interactive")
            {
                return Ok(());
            }
        }
        bail!("navigation timeout")
    }
    fn restore(&mut self, state: &Value) -> Result<()> {
        self.call(
            "Storage.setCookies",
            json!({"cookies":state["cookies"],"browserContextId":self.context}),
            false,
        )?;
        for origin in state["origins"].as_array().context("origins missing")? {
            let url = origin["origin"].as_str().context("origin missing")?;
            let parsed = reqwest::Url::parse(url)?;
            if parsed.scheme() != "https"
                || !parsed.username().is_empty()
                || parsed.password().is_some()
                || parsed.port().is_some()
                || parsed.query().is_some()
                || parsed.fragment().is_some()
                || parsed.origin().ascii_serialization() != url
                || !parsed
                    .host_str()
                    .is_some_and(|h| h == "google.com" || h.ends_with(".google.com"))
            {
                bail!("unsupported credential origin");
            }
            self.navigate(url)?;
            if self.eval("location.origin")? != json!(url) {
                bail!("credential origin redirected");
            }
            self.eval(&format!(
                "({})(JSON.parse({}))",
                RESTORE,
                serde_json::to_string(&serde_json::to_string(origin)?)?
            ))?;
            self.origins.push(url.to_owned());
        }
        Ok(())
    }
    fn state(&mut self) -> Result<Value> {
        let cookies = self.call(
            "Storage.getCookies",
            json!({"browserContextId":self.context}),
            false,
        )?["cookies"]
            .clone();
        // Store every origin restored at launch plus the current Gemini origin. All
        // contexts stay private; failures abort instead of silently losing IDB state.
        let current = self
            .eval("location.origin")?
            .as_str()
            .context("origin missing")?
            .to_owned();
        if !self.origins.contains(&current) {
            self.origins.push(current.clone());
        }
        let mut origins = Vec::new();
        for url in self.origins.clone() {
            self.navigate(&url)?;
            if self.eval("location.origin")? != json!(url) {
                bail!("credential origin redirected");
            }
            origins.push(self.eval(EXPORT)?);
        }
        let state = json!({"cookies":cookies,"origins":origins});
        if serde_json::to_vec(&state)?.len() > dbrain_session_store::LIMIT {
            bail!("state too large");
        }
        Ok(state)
    }
    fn authenticated(&mut self) -> Result<bool> {
        Ok(self.eval(r#"(()=>{const prompt=document.querySelector("rich-textarea div[contenteditable='true'],div[contenteditable='true'][role='textbox'],textarea");const login=[...document.querySelectorAll('a[href*="accounts.google.com"]')].some(a=>a.getClientRects().length&&/^(sign in|anmelden)$/i.test(a.innerText.trim()));return !!prompt&&!login;})()"#)?==json!(true))
    }
    fn submit(&mut self, prompt: &str) -> Result<usize> {
        let before = self
            .eval("document.querySelectorAll('message-content').length")?
            .as_u64()
            .context("response count missing")? as usize;
        let deadline = Instant::now() + Duration::from_secs(45);
        loop {
            if self.eval("(()=>{const e=document.querySelector(\"rich-textarea div[contenteditable='true'],div[contenteditable='true'][role='textbox'],textarea\");if(!e)return false;e.focus();return true})()")?==json!(true){break;}
            if Instant::now() > deadline {
                bail!("prompt input missing");
            }
            std::thread::sleep(Duration::from_millis(500));
        }
        self.call("Input.insertText", json!({"text":prompt}), true)?;
        std::thread::sleep(Duration::from_secs(9));
        self.call(
            "Input.dispatchKeyEvent",
            json!({"type":"keyDown","key":"Enter","code":"Enter","windowsVirtualKeyCode":13}),
            true,
        )?;
        self.call(
            "Input.dispatchKeyEvent",
            json!({"type":"keyUp","key":"Enter","code":"Enter","windowsVirtualKeyCode":13}),
            true,
        )?;
        Ok(before)
    }
    fn response(
        &mut self,
        before: usize,
        timeout: u64,
    ) -> std::result::Result<String, GeminiError> {
        let start = Instant::now();
        let mut stable = Instant::now();
        let mut last = String::new();
        while start.elapsed() < Duration::from_secs(timeout) {
            let v=self.eval("(()=>{const a=[...document.querySelectorAll('message-content')];return {count:a.length,text:a.length?a[a.length-1].innerText:'',body:document.body.innerText,streaming:!!document.querySelector(\"button[aria-label*='Stop generating'],button[aria-label*='Antwort stoppen'],mat-progress-spinner\")}})()").map_err(|_|failure(GeminiErrorKind::Unknown))?;
            let body = v["body"].as_str().unwrap_or("").to_lowercase();
            if [
                "rate limit",
                "too many requests",
                "quota",
                "try again later",
                "später erneut",
                "zu viele anfragen",
            ]
            .iter()
            .any(|m| body.contains(m))
            {
                return Err(failure(GeminiErrorKind::RateLimited));
            }
            let text = if v["count"].as_u64().unwrap_or(0) > before as u64 {
                v["text"].as_str().unwrap_or("").trim()
            } else {
                ""
            };
            if !text.is_empty() && text != last {
                last = text.into();
                stable = Instant::now();
            }
            if !last.is_empty()
                && stable.elapsed() >= Duration::from_secs(4)
                && v["streaming"] == json!(false)
            {
                if [
                    "i can't assist",
                    "i can’t assist",
                    "i cannot assist",
                    "dabei kann ich nicht helfen",
                ]
                .iter()
                .any(|m| last.to_lowercase().contains(m))
                {
                    return Err(failure(GeminiErrorKind::Refused));
                }
                return Ok(last);
            }
            std::thread::sleep(Duration::from_secs(1));
        }
        Err(failure(GeminiErrorKind::Timeout))
    }
}
fn browser_failure(error: anyhow::Error) -> GeminiError {
    let timeout = error.downcast_ref::<std::io::Error>().is_some_and(|e| {
        matches!(
            e.kind(),
            std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
        )
    }) || error.to_string() == "browser timeout"
        || error.to_string() == "navigation timeout";
    failure(if timeout {
        GeminiErrorKind::Timeout
    } else {
        GeminiErrorKind::Unknown
    })
}
fn failure(kind: GeminiErrorKind) -> GeminiError {
    GeminiError {
        kind,
        message: match kind {
            GeminiErrorKind::NotLoggedIn => "Gemini-Anmeldung erforderlich.",
            GeminiErrorKind::RateLimited => "Gemini-Anfragelimit erreicht.",
            GeminiErrorKind::Refused => "Gemini hat die Analyse abgelehnt.",
            GeminiErrorKind::Timeout => "Gemini-Antwort hat zu lange gedauert.",
            _ => "Browser- oder DB-Sitzungsfehler. Konto und Sitzungsdienst prüfen.",
        }
        .into(),
    }
}
pub fn run_login() -> Result<()> {
    std::thread::spawn(|| -> Result<()> {
        let c = config()?;
        let rt = tokio::runtime::Runtime::new()?;
        let (mut db, cipher) = rt.block_on(dbrain_session_store::connect_with_values(
            &c.infisical_config,
            &c.key_secret,
            rt.block_on(crate::db::runtime_secrets(&c.infisical_config))?,
        ))?;
        let envelope = dbrain_session_store::load(&mut db, &cipher, &c.account_id)?;
        let mut b = Browser::open(&c.browser_path)?;
        b.restore(&envelope["state"])?;
        b.navigate(GEMINI_URL)?;
        println!("Im Browser bei Gemini anmelden und danach hier Enter drücken …");
        std::io::stdin().read_line(&mut String::new())?;
        b.navigate(GEMINI_URL)?;
        std::thread::sleep(Duration::from_secs(6));
        if !b.authenticated()? {
            bail!("login required");
        }
        let state = b.state()?;
        dbrain_session_store::save(
            &mut db,
            &cipher,
            &c.account_id,
            envelope["revision"].as_i64().context("revision missing")?,
            &state,
        )?;
        Ok(())
    })
    .join()
    .map_err(|_| anyhow::anyhow!("Sitzungsdienst fehlgeschlagen"))
    .and_then(|result| result)
    .map_err(|_| {
        anyhow::anyhow!(
            "Gemini-Anmeldung oder DB-Speicherung fehlgeschlagen. Konto und Sitzungsdienst prüfen."
        )
    })
}
pub fn analyze_url(url: &str, prompt: &str) -> std::result::Result<String, GeminiError> {
    let parsed = reqwest::Url::parse(url).map_err(|_| failure(GeminiErrorKind::Unknown))?;
    if parsed.scheme() != "https"
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || !parsed.host_str().is_some_and(|h| {
            matches!(
                h,
                "youtube.com" | "www.youtube.com" | "youtu.be" | "m.youtube.com"
            )
        })
    {
        return Err(failure(GeminiErrorKind::Unknown));
    }

    let prompt = if prompt.contains(url) {
        prompt.to_owned()
    } else {
        format!("{prompt}\n\n{url}")
    };
    std::thread::spawn(move || {
        let c = config().map_err(|_| failure(GeminiErrorKind::Unknown))?;
        let rt = tokio::runtime::Runtime::new().map_err(|_| failure(GeminiErrorKind::Unknown))?;
        let (mut db, cipher) = rt
            .block_on(dbrain_session_store::connect_with_values(
                &c.infisical_config,
                &c.key_secret,
                rt.block_on(crate::db::runtime_secrets(&c.infisical_config))
                    .map_err(|_| failure(GeminiErrorKind::Unknown))?,
            ))
            .map_err(|_| failure(GeminiErrorKind::Unknown))?;
        let envelope = dbrain_session_store::load(&mut db, &cipher, &c.account_id)
            .map_err(|_| failure(GeminiErrorKind::Unknown))?;
        if envelope["revision"] == json!(-1) {
            return Err(failure(GeminiErrorKind::NotLoggedIn));
        }
        let mut b = Browser::open(&c.browser_path).map_err(browser_failure)?;
        b.restore(&envelope["state"]).map_err(browser_failure)?;
        b.navigate(GEMINI_URL).map_err(browser_failure)?;
        std::thread::sleep(Duration::from_secs(6));
        if !b.authenticated().map_err(browser_failure)? {
            return Err(failure(GeminiErrorKind::NotLoggedIn));
        }
        let before = b
            .submit(&prompt)
            .map_err(|_| failure(GeminiErrorKind::Unknown))?;
        b.response(before, c.response_timeout_seconds)?;
        let before = b
            .submit(CONVERT)
            .map_err(|_| failure(GeminiErrorKind::Unknown))?;
        let result = b.response(before, c.response_timeout_seconds)?;
        let state = b.state().map_err(|_| failure(GeminiErrorKind::Unknown))?;
        dbrain_session_store::save(
            &mut db,
            &cipher,
            &c.account_id,
            envelope["revision"]
                .as_i64()
                .ok_or_else(|| failure(GeminiErrorKind::Unknown))?,
            &state,
        )
        .map_err(|_| failure(GeminiErrorKind::Unknown))?;
        Ok(result)
    })
    .join()
    .unwrap_or_else(|_| Err(failure(GeminiErrorKind::Unknown)))
}
pub fn send_pause_alert(kind: GeminiErrorKind, _message: &str) {
    let result = std::thread::spawn(move || -> Result<()> {
        let c = config()?;
        let Some(name) = c.alert_webhook_secret else { return Ok(()); };
        // Notification metadata contains no credentials. Serialize cross-process
        // sends, retaining suppressed repetitions for the next permitted notice.
        use std::os::unix::fs::OpenOptionsExt;
        let path=crate::db::repo_root().join("data/gemini-alert-status.json");
        let mut file=fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).mode(0o600).open(path)?;
        // SAFETY: file owns this valid open FD until the closure returns; flock
        // takes no pointers and the file cannot be closed while this call runs.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) } != 0 { bail!("alert status unavailable"); }
        let mut raw=String::new();file.read_to_string(&mut raw)?;
        let mut status:Value=if raw.is_empty(){json!({"sent":[],"suppressed":0})}else{serde_json::from_str(&raw)?};
        let now=crate::db::now_epoch_seconds();
        let sent:Vec<i64>=status["sent"].as_array().context("alert status invalid")?.iter().filter_map(Value::as_i64).filter(|t|now-*t<7*86400).collect();
        let repetitions=status["suppressed"].as_u64().unwrap_or(0);
        if sent.len()>=2||sent.last().is_some_and(|t|now-*t<86400){status=json!({"sent":sent,"suppressed":repetitions+1});}
        else {
            let rt=tokio::runtime::Runtime::new()?;
            let values=rt.block_on(crate::db::runtime_secrets(&c.infisical_config))?;
            let webhook=values.iter().find(|(k,_)|k==&name).context("webhook missing")?;
            reqwest::blocking::Client::builder().timeout(Duration::from_secs(15)).build()?.post(webhook.1.as_str()).json(&json!({"content":format!("⚠️ Deadlock-Brain YouTube-Lernen pausiert [{kind}]. Wiederholungen seit der letzten Meldung: {repetitions}."),"kind":kind.to_string()})).send()?.error_for_status()?;
            let mut sent=sent;sent.push(now);status=json!({"sent":sent,"suppressed":0});
        }
        use std::io::{Seek,SeekFrom};
        file.seek(SeekFrom::Start(0))?;file.set_len(0)?;file.write_all(&serde_json::to_vec(&status)?)?;file.sync_data()?;Ok(())
    }).join();
    if !matches!(result, Ok(Ok(()))) {
        eprintln!("Pausenhinweis konnte nicht sicher versendet werden. Benachrichtigungsdienst und Statusspeicher prüfen.");
    }
}

// Playwright-compatible storage format. Preserve database schema, indexes and
// explicit keys. Unsupported structured-clone types fail rather than being lost.
const EXPORT: &str = r#"(async()=>{
function safe(x,seen=new Set()){
    if(x===null||['string','boolean'].includes(typeof x))return x;
    if(typeof x==='number'){
        if(!Number.isFinite(x)||Object.is(x,-0))throw Error('unsupported storage number');
        return x;
    }
    if(typeof x!=='object'||seen.has(x))throw Error('unsupported storage type');
    seen.add(x);
    let result;
    if(Array.isArray(x)){
        const keys=Reflect.ownKeys(x);
        if(keys.length!==x.length+1||keys.some(k=>k!=='length'&&(!/^(0|[1-9][0-9]*)$/.test(k)||Number(k)>=x.length)))throw Error('unsupported storage array');
        result=[];
        for(let i=0;i<x.length;i++){
            if(!Object.hasOwn(x,i))throw Error('unsupported sparse storage array');
            result.push(safe(x[i],seen));
        }
    }else if(Object.getPrototypeOf(x)===Object.prototype){
        result=Object.create(null);
        for(const k of Reflect.ownKeys(x)){
            if(typeof k!=='string'||!Object.prototype.propertyIsEnumerable.call(x,k))throw Error('unsupported storage property');
            result[k]=safe(x[k],seen);
        }
    }else throw Error('unsupported storage type');
    // JSON cannot preserve cycles or shared object identity; retain all seen
    // references for this value and reject a repeated reference explicitly.
    return result;
}
const request=r=>new Promise((ok,no)=>{r.onsuccess=()=>ok(r.result);r.onerror=()=>no(Error('storage read failed'));});
const databases=[];
for(const d of await indexedDB.databases()){
    const db=await request(indexedDB.open(d.name));
    try{
        const stores=[];
        for(const name of db.objectStoreNames){
            const tx=db.transaction(name,'readonly');const store=tx.objectStore(name);
            // IndexedDB exposes no non-mutating read of the current generator.
            // Reject before persistence instead of silently resetting it.
            if(store.autoIncrement)throw Error('unsupported storage key generator');
            const records=await new Promise((ok,no)=>{const values=[];const r=store.openCursor();r.onerror=()=>no(Error('cursor failed'));r.onsuccess=()=>{const c=r.result;if(c){try{values.push({key:safe(c.key),value:safe(c.value)});c.continue();}catch(e){no(e);}}else ok(values);};});
            stores.push({name,keyPath:store.keyPath,autoIncrement:false,indexes:[...store.indexNames].map(n=>{const i=store.index(n);return {name:n,keyPath:i.keyPath,unique:i.unique,multiEntry:i.multiEntry};}),records});
        }
        databases.push({name:d.name,version:db.version,stores});
    }finally{db.close();}
}
return {origin:location.origin,localStorage:Object.entries(localStorage).map(([name,value])=>({name,value})),indexedDB:databases};
})()"#;
const RESTORE: &str = r#"async(s)=>{for(const d of s.indexedDB||[])for(const st of d.stores)if(st.autoIncrement)throw Error('unsupported storage key generator');for(const d of await indexedDB.databases())await new Promise((ok,no)=>{const r=indexedDB.deleteDatabase(d.name);r.onsuccess=()=>ok();r.onerror=r.onblocked=()=>no(Error('storage cleanup failed'));});localStorage.clear();for(const v of s.localStorage||[])localStorage.setItem(v.name,v.value);for(const d of s.indexedDB||[]){const db=await new Promise((ok,no)=>{const r=indexedDB.open(d.name,d.version);r.onerror=()=>no(Error('storage open failed'));r.onupgradeneeded=()=>{for(const st of d.stores){const store=r.result.createObjectStore(st.name,{keyPath:st.keyPath,autoIncrement:false});for(const i of st.indexes)store.createIndex(i.name,i.keyPath,{unique:i.unique,multiEntry:i.multiEntry});}};r.onsuccess=()=>ok(r.result);});try{if(d.stores.length)await new Promise((ok,no)=>{const tx=db.transaction(d.stores.map(v=>v.name),'readwrite');tx.oncomplete=()=>ok();tx.onerror=tx.onabort=()=>no(Error('storage write failed'));for(const st of d.stores){const store=tx.objectStore(st.name);for(const record of st.records){if(st.keyPath===null)store.put(record.value,record.key);else store.put(record.value);}}});}finally{db.close();}}}"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "real Chromium and virtual display required; synthetic HTTP origin only"]
    fn chromium_private_pipe_storage_roundtrip() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut request = [0; 4096];
                let _ = stream.read(&mut request);
                let _=stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 35\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n<html><body>synthetic</body></html>");
            }
        });
        let url = format!("http://{address}");
        let mut first = Browser::open(&browser_path()).unwrap();
        first.navigate(&url).unwrap();
        first.eval(r#"(async()=>{localStorage.setItem('synthetic','fixture');await new Promise((ok,no)=>{const r=indexedDB.open('synthetic-db',2);r.onupgradeneeded=()=>{const s=r.result.createObjectStore('records',{keyPath:['first','second']});s.createIndex('idx','label',{unique:false,multiEntry:false});};r.onsuccess=()=>{const db=r.result;const tx=db.transaction('records','readwrite');tx.objectStore('records').put({first:'a',second:'b',label:'synthetic'});tx.oncomplete=()=>{db.close();ok();};tx.onerror=()=>no(Error('fixture failed'));};r.onerror=()=>no(Error('fixture failed'));});return true;})()"#).unwrap();
        first.eval(r#"(async()=>{await new Promise((ok,no)=>{const r=indexedDB.open('synthetic-db',2);r.onsuccess=()=>{const db=r.result;const tx=db.transaction('records','readwrite');const v=JSON.parse('{"first":"proto","second":"own","__proto__":{"value":"preserved"}}');tx.objectStore('records').put(v);tx.oncomplete=()=>{db.close();ok();};tx.onerror=()=>no(Error('fixture failed'));};});return true;})()"#).unwrap();
        first
            .eval(r#"localStorage.setItem('escaped-boundary','\\'.repeat(300*1024));true"#)
            .unwrap();
        let state = first.eval(EXPORT).unwrap();
        assert!(serde_json::to_vec(&state).unwrap().len() < dbrain_session_store::LIMIT);
        assert!(state["indexedDB"][0]["stores"][0]["records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|record| record["value"]["__proto__"]["value"] == "preserved"));
        assert_eq!(
            state["indexedDB"][0]["stores"][0]["keyPath"],
            json!(["first", "second"])
        );
        first.call("Storage.setCookies",json!({"browserContextId":first.context,"cookies":[{"name":"synthetic","value":"fixture","domain":"127.0.0.1","path":"/"}]}),false).unwrap();
        let cookies = first
            .call(
                "Storage.getCookies",
                json!({"browserContextId":first.context}),
                false,
            )
            .unwrap()["cookies"]
            .clone();
        drop(first);
        let mut second = Browser::open(&browser_path()).unwrap();
        second.navigate(&url).unwrap();
        second.eval(r#"(async()=>{localStorage.setItem('page-init-extra','must-disappear');await new Promise((ok,no)=>{const r=indexedDB.open('synthetic-db',1);r.onupgradeneeded=()=>r.result.createObjectStore('page-init');r.onsuccess=()=>{const db=r.result;const tx=db.transaction('page-init','readwrite');tx.objectStore('page-init').put('extra',1);tx.oncomplete=()=>{db.close();ok();};tx.onerror=()=>no(Error('fixture failed'));};});return true;})()"#).unwrap();
        second
            .eval(&format!(
                "({RESTORE})(JSON.parse({}))",
                serde_json::to_string(&serde_json::to_string(&state).unwrap()).unwrap()
            ))
            .unwrap();
        assert_eq!(second.eval(EXPORT).unwrap(), state);
        let oversized_expression = format!(
            "localStorage.setItem('oversized-mutation','must-not-run');/*{}*/",
            "x".repeat(CDP_REQUEST_LIMIT)
        );
        assert!(second.eval(&oversized_expression).is_err());
        assert_eq!(
            second
                .eval("localStorage.getItem('oversized-mutation')")
                .unwrap(),
            Value::Null
        );
        second
            .call(
                "Storage.setCookies",
                json!({"browserContextId":second.context,"cookies":cookies}),
                false,
            )
            .unwrap();
        assert_eq!(
            second.eval("document.cookie").unwrap(),
            json!("synthetic=fixture")
        );
        // Unsupported structured-clone values fail before persistence rather
        // than being normalized, stripped or silently turned into null.
        for expression in [
            "new Date()",
            "[1,,3]",
            "Object.assign([1],{extra:'preserve-me'})",
            "NaN",
            "Infinity",
            "-Infinity",
            "-0",
            "(()=>{const shared={a:1};return {first:shared,second:shared};})()",
            "(()=>{const cycle={};cycle.self=cycle;return cycle;})()",
        ] {
            second.eval(&format!(r#"(async()=>{{const r=indexedDB.open('bad-db',1);await new Promise((ok,no)=>{{r.onupgradeneeded=()=>r.result.createObjectStore('records');r.onsuccess=()=>{{const db=r.result;const tx=db.transaction('records','readwrite');tx.objectStore('records').put({expression},1);tx.oncomplete=()=>{{db.close();ok();}};tx.onerror=()=>no(Error('fixture failed'));}};}});return true;}})()"#)).unwrap();
            assert!(second.eval(EXPORT).is_err(), "must reject {expression}");
            second.eval(r#"(async()=>{await new Promise((ok,no)=>{const r=indexedDB.deleteDatabase('bad-db');r.onsuccess=()=>ok();r.onerror=()=>no(Error('fixture failed'));});return true;})()"#).unwrap();
        }
        // A cleared generator at 101 cannot be inferred from stored records.
        second.eval(r#"(async()=>{await new Promise((ok,no)=>{const r=indexedDB.open('generator-db',1);r.onupgradeneeded=()=>r.result.createObjectStore('records',{autoIncrement:true});r.onsuccess=()=>{const db=r.result;const tx=db.transaction('records','readwrite');const st=tx.objectStore('records');st.put('fixture',100);st.clear();tx.oncomplete=()=>{db.close();ok();};tx.onerror=()=>no(Error('fixture failed'));};});return true;})()"#).unwrap();
        assert!(second.eval(EXPORT).is_err());
        assert!(second.eval(&format!(r#"({RESTORE})({{"localStorage":[{{"name":"must-not-change","value":"fixture"}}],"indexedDB":[{{"stores":[{{"autoIncrement":true}}]}}]}})"#)).is_err());
        assert_eq!(
            second
                .eval("localStorage.getItem('must-not-change')")
                .unwrap(),
            Value::Null
        );
    }
}
