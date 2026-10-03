//! Liest öffentliche Forumseiten aus der vorhandenen Brave-Sitzung.
//! Die Verbindung liest nur DevToolsActivePort, keine Cookies oder Zugangsdaten.

use std::{
    io::Write,
    os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures_util::{SinkExt, StreamExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ForumBrowserConfig {
    pub cdp_port_file: PathBuf,
    pub page_timeout_seconds: u64,
}

impl Default for ForumBrowserConfig {
    fn default() -> Self {
        Self {
            cdp_port_file: PathBuf::from(
                "/home/nathanael/.config/BraveSoftware/Brave-Browser/DevToolsActivePort",
            ),
            page_timeout_seconds: 30,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ForumBrowserError(String);

type Result<T> = std::result::Result<T, ForumBrowserError>;
type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Serialize, Deserialize)]
struct ConsentRequest {
    pid: u32,
    process_start: u64,
    started_at_ms: u64,
    cdp_port: u16,
    cdp_port_file: PathBuf,
}

fn read_port_file(path: &std::path::Path) -> Result<String> {
    let metadata =
        std::fs::symlink_metadata(path).map_err(|_| error("Brave-Debugportdatei fehlt"))?;
    if path.file_name().and_then(|name| name.to_str()) != Some("DevToolsActivePort")
        || !metadata.is_file()
        || metadata.len() > 512
        || Some(metadata.uid()) != current_uid()
    {
        return Err(error("Ungültige Brave-Debugportdatei"));
    }
    std::fs::read_to_string(path)
        .map_err(|_| error("Brave-Debugportdatei konnte nicht gelesen werden"))
}

fn owns_debug_socket(pid: u32, port: u16, listener: bool) -> bool {
    let Ok(tcp) = std::fs::read_to_string("/proc/net/tcp") else {
        return false;
    };
    let address = format!("0100007F:{port:04X}");
    let inodes: Vec<_> = tcp
        .lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<_> = line.split_whitespace().collect();
            let endpoint = if listener { 1 } else { 2 };
            let state = if listener { "0A" } else { "01" };
            (fields.len() > 9 && fields[endpoint] == address && fields[3] == state)
                .then(|| format!("socket:[{}]", fields[9]))
        })
        .collect();
    let Ok(files) = std::fs::read_dir(format!("/proc/{pid}/fd")) else {
        return false;
    };
    files.filter_map(std::result::Result::ok).any(|file| {
        std::fs::read_link(file.path())
            .ok()
            .and_then(|path| path.to_str().map(str::to_owned))
            .is_some_and(|target| inodes.contains(&target))
    })
}

fn exclusive_socket_inodes(
    tcp: &str,
    tcp6: &str,
    port: u16,
) -> Option<(String, String, Vec<String>)> {
    let mut clients = Vec::new();
    let mut accepted = Vec::new();
    let mut listeners = Vec::new();
    for table in [tcp, tcp6] {
        for line in table.lines().skip(1) {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() < 10 {
                return None;
            }
            let local = u16::from_str_radix(fields[1].rsplit_once(':')?.1, 16).ok()?;
            let remote = u16::from_str_radix(fields[2].rsplit_once(':')?.1, 16).ok()?;
            if local != port && remote != port {
                continue;
            }
            let inode = format!("socket:[{}]", fields[9]);
            match fields[3] {
                "0A" if local == port => listeners.push(inode),
                "02" | "03" => return None,
                "01" => {
                    if local == port && remote == port {
                        return None;
                    }
                    if remote == port {
                        clients.push(inode);
                    } else {
                        accepted.push(inode);
                    }
                }
                _ => {}
            }
        }
    }
    if clients.len() != 1 || accepted.len() != 1 || listeners.is_empty() {
        return None;
    }
    Some((clients.pop()?, accepted.pop()?, listeners))
}

fn process_socket_inodes(pid: u32) -> Option<Vec<String>> {
    let files = std::fs::read_dir(format!("/proc/{pid}/fd")).ok()?;
    Some(
        files
            .filter_map(std::result::Result::ok)
            .filter_map(|file| {
                std::fs::read_link(file.path())
                    .ok()?
                    .to_str()
                    .map(str::to_owned)
            })
            .filter(|target| target.starts_with("socket:["))
            .collect(),
    )
}

fn exclusive_debug_client(pid: u32, port: u16, brave_pid: Option<u32>) -> bool {
    let Ok(tcp) = std::fs::read_to_string("/proc/net/tcp") else {
        return false;
    };
    let Ok(tcp6) = std::fs::read_to_string("/proc/net/tcp6") else {
        return false;
    };
    let Some((client, accepted, listeners)) = exclusive_socket_inodes(&tcp, &tcp6, port) else {
        return false;
    };
    let Some(owner) = process_socket_inodes(pid) else {
        return false;
    };
    if !owner.contains(&client) || !owns_debug_socket(pid, port, false) {
        return false;
    }
    brave_pid.is_none_or(|pid| {
        let Some(browser) = process_socket_inodes(pid) else {
            return false;
        };
        browser.contains(&accepted) && listeners.iter().all(|inode| browser.contains(inode))
    })
}

fn current_uid() -> Option<u32> {
    std::fs::metadata("/proc/self/status")
        .ok()
        .map(|metadata| metadata.uid())
}

fn request_directory() -> Option<PathBuf> {
    Some(PathBuf::from(format!(
        "/run/user/{}/deadlock-brain-browser-consent",
        current_uid()?
    )))
}

fn process_start(pid: u32) -> Option<u64> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    stat.rsplit_once(')')?
        .1
        .split_whitespace()
        .nth(19)?
        .parse()
        .ok()
}

fn now_ms() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_millis()
        .try_into()
        .ok()
}

struct PendingConsent(PathBuf);
impl PendingConsent {
    fn create(port: u16, cdp_port_file: PathBuf) -> Result<Self> {
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let directory = request_directory()
            .ok_or_else(|| error("Benutzer für Browserfreigabe nicht gefunden"))?;
        match std::fs::DirBuilder::new().mode(0o700).create(&directory) {
            Ok(()) => {}
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(_) => {
                return Err(error(
                    "Privates Browserfreigabeverzeichnis konnte nicht angelegt werden",
                ))
            }
        }
        let metadata = std::fs::symlink_metadata(&directory)
            .map_err(|_| error("Browserfreigabeverzeichnis fehlt"))?;
        if !metadata.is_dir()
            || Some(metadata.uid()) != current_uid()
            || metadata.permissions().mode() & 0o777 != 0o700
        {
            return Err(error(
                "Browserfreigabeverzeichnis hat unsichere Zugriffsrechte",
            ));
        }
        let request = ConsentRequest {
            pid: std::process::id(),
            process_start: process_start(std::process::id())
                .ok_or_else(|| error("Importprozess nicht gefunden"))?,
            started_at_ms: now_ms().ok_or_else(|| error("Systemzeit für Browserfreigabe fehlt"))?,
            cdp_port: port,
            cdp_port_file,
        };
        let path = directory.join(format!(
            "request-{}-{}.json",
            request.pid,
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        let guard = Self(path);
        let bytes = serde_json::to_vec(&request)
            .map_err(|_| error("Browserfreigabe konnte nicht vorbereitet werden"))?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&guard.0)
            .map_err(|_| error("Browserfreigabe konnte nicht angelegt werden"))?;
        file.write_all(&bytes)
            .map_err(|_| error("Browserfreigabe konnte nicht geschrieben werden"))?;
        Ok(guard)
    }
}
impl Drop for PendingConsent {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Prüft ausschließlich kurzlebige Freigabeanfragen eigener laufender Importe.
pub fn has_pending_consent() -> bool {
    pending_consent(None)
}

/// Bindet den sichtbaren Brave-Dialog an dessen eigenen lokalen Debuglistener.
pub fn has_pending_consent_for_brave(brave_pid: u32) -> bool {
    pending_consent(Some(brave_pid))
}

fn pending_consent(brave_pid: Option<u32>) -> bool {
    let Some(directory) = request_directory() else {
        return false;
    };
    let Ok(metadata) = std::fs::symlink_metadata(&directory) else {
        return false;
    };
    if !metadata.is_dir()
        || Some(metadata.uid()) != current_uid()
        || metadata.permissions().mode() & 0o777 != 0o700
    {
        return false;
    }
    let Ok(files) = std::fs::read_dir(directory) else {
        return false;
    };
    let Some(now) = now_ms() else { return false };
    files.filter_map(std::result::Result::ok).any(|file| {
        let path = file.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            return false;
        };
        if !name.starts_with("request-") || !name.ends_with(".json") {
            return false;
        }
        let Ok(metadata) = std::fs::symlink_metadata(&path) else {
            return false;
        };
        if !metadata.is_file()
            || Some(metadata.uid()) != current_uid()
            || metadata.permissions().mode() & 0o777 != 0o600
            || metadata.len() > 1024
        {
            return false;
        }
        let Ok(bytes) = std::fs::read(path) else {
            return false;
        };
        let Ok(request) = serde_json::from_slice::<ConsentRequest>(&bytes) else {
            return false;
        };
        let valid_port = read_port_file(&request.cdp_port_file)
            .ok()
            .is_some_and(|text| {
                local_endpoint(&text).is_ok()
                    && text
                        .lines()
                        .next()
                        .and_then(|line| line.parse::<u16>().ok())
                        == Some(request.cdp_port)
            });
        request.cdp_port != 0
            && valid_port
            && request.started_at_ms <= now
            && now - request.started_at_ms <= 60_000
            && process_start(request.pid) == Some(request.process_start)
            && exclusive_debug_client(request.pid, request.cdp_port, brave_pid)
    })
}

pub struct ForumBrowser {
    socket: Socket,
    next_id: u64,
    target_id: String,
    session_id: String,
    timeout: Duration,
}

fn error(message: &str) -> ForumBrowserError {
    ForumBrowserError(message.to_owned())
}

fn allowed_url(raw: &str) -> bool {
    let Ok(url) = Url::parse(raw) else {
        return false;
    };
    if url.scheme() != "https"
        || url.host_str() != Some("forums.playdeadlock.com")
        || url.port_or_known_default() != Some(443)
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return false;
    }
    let path = url.path();
    if path == "/" {
        return true;
    }
    if path.starts_with("/forums/") || path.starts_with("/threads/") {
        return !path.contains('%') && !path.contains("//");
    }
    path.starts_with("/sitemap")
        && path.ends_with(".xml")
        && !path[1..].contains('/')
        && !path.contains('%')
}

fn local_endpoint(raw: &str) -> Result<String> {
    let mut lines = raw.lines();
    let port = lines
        .next()
        .and_then(|line| line.parse::<u16>().ok())
        .filter(|port| *port != 0)
        .ok_or_else(|| error("Ungültiger Brave-Debugport"))?;
    let path = lines.next().ok_or_else(|| error("Brave-Debugpfad fehlt"))?;
    if !path.starts_with("/devtools/browser/")
        || !path
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"/-_".contains(&c))
    {
        return Err(error("Ungültiger Brave-Debugpfad"));
    }
    Ok(format!("ws://127.0.0.1:{port}{path}"))
}

fn same_page(requested: &str, actual: &str) -> bool {
    if requested.trim_end_matches('/') == actual.trim_end_matches('/') {
        return true;
    }
    fn identity(raw: &str) -> Option<(String, u64, String)> {
        let url = Url::parse(raw).ok()?;
        let mut parts = url.path().trim_matches('/').split('/');
        let section = parts.next()?;
        if !matches!(section, "threads" | "forums") {
            return None;
        }
        let id = parts.next()?.rsplit('.').next()?.parse().ok()?;
        Some((section.to_owned(), id, parts.collect::<Vec<_>>().join("/")))
    }
    matches!((identity(requested), identity(actual)), (Some(left), Some(right)) if left == right)
}

impl ForumBrowser {
    pub async fn connect(config: &ForumBrowserConfig) -> Result<Self> {
        if !(1..=120).contains(&config.page_timeout_seconds) {
            return Err(error(
                "Browser-Ladefrist muss zwischen 1 und 120 Sekunden liegen",
            ));
        }
        let port_file = config.cdp_port_file.clone();
        let port_text = tokio::task::spawn_blocking(move || read_port_file(&port_file))
            .await
            .map_err(|_| error("Brave-Debugport konnte nicht gelesen werden"))?
            .map_err(|_| error("Brave-Debugport nicht verfügbar; Remote-Debugging muss im Browser eingeschaltet sein"))?;
        let endpoint = local_endpoint(&port_text)?;
        let port = port_text
            .lines()
            .next()
            .and_then(|port| port.parse::<u16>().ok())
            .ok_or_else(|| error("Brave-Debugport fehlt"))?;
        let _pending = PendingConsent::create(port, config.cdp_port_file.clone())?;
        let (socket, _) = tokio::time::timeout(
            Duration::from_secs(60),
            tokio_tungstenite::connect_async(endpoint),
        )
        .await
            .map_err(|_| error("Brave-Debugverbindung hat nicht geantwortet. Unter brave://inspect/#remote-debugging Remote Debugging prüfen und beim Verbindungsversuch „Allow remote debugging?“ mit „Allow“ bestätigen"))?
            .map_err(|_| error("Brave-Debugverbindung fehlgeschlagen"))?;
        drop(_pending);
        let mut browser = Self {
            socket,
            next_id: 1,
            target_id: String::new(),
            session_id: String::new(),
            timeout: Duration::from_secs(config.page_timeout_seconds),
        };
        let created = browser
            .call("Target.createTarget", json!({"url":"about:blank"}), false)
            .await?;
        browser.target_id = created
            .get("targetId")
            .and_then(Value::as_str)
            .ok_or_else(|| error("Brave hat keinen Import-Tab erzeugt"))?
            .to_owned();
        let attached = browser
            .call(
                "Target.attachToTarget",
                json!({"targetId":browser.target_id,"flatten":true}),
                false,
            )
            .await;
        let attached = match attached {
            Ok(value) => value,
            Err(err) => {
                if browser.close().await.is_err() {
                    return Err(error(
                        "Brave konnte den Import-Tab weder verbinden noch schließen",
                    ));
                }
                return Err(err);
            }
        };
        let Some(session) = attached.get("sessionId").and_then(Value::as_str) else {
            if browser.close().await.is_err() {
                return Err(error(
                    "Brave konnte den Import-Tab ohne Sitzung nicht schließen",
                ));
            }
            return Err(error("Brave hat keine Import-Sitzung erzeugt"));
        };
        browser.session_id = session.to_owned();
        Ok(browser)
    }

    pub async fn fetch_html(&mut self, url: &str) -> Result<String> {
        if !allowed_url(url) {
            return Err(error(
                "Browserimport erlaubt nur öffentliche Seiten des offiziellen Forums",
            ));
        }
        let navigation = self.call("Page.navigate", json!({"url":url}), true).await?;
        if navigation.get("errorText").is_some() {
            return Err(error("Brave konnte die Forumseite nicht laden"));
        }
        let deadline = tokio::time::Instant::now() + self.timeout;
        loop {
            let snapshot = self
                .call(
                    "Runtime.evaluate",
                    json!({
                        "expression": SNAPSHOT,
                        "returnByValue":true
                    }),
                    true,
                )
                .await?;
            if snapshot.get("exceptionDetails").is_some() {
                return Err(error("Forumseite konnte nicht gelesen werden"));
            }
            let value = snapshot.pointer("/result/value").unwrap_or(&Value::Null);
            let final_url = value.get("url").and_then(Value::as_str).unwrap_or("");
            if !final_url.is_empty() && final_url != "about:blank" && !allowed_url(final_url) {
                return Err(error(
                    "Forum hat auf eine nicht freigegebene Adresse weitergeleitet",
                ));
            }
            if value.get("logged_in").and_then(Value::as_bool) == Some(true) {
                return Err(error(
                    "Forumimport braucht eine öffentlich sichtbare Sitzung ohne Anmeldung",
                ));
            }
            if same_page(url, final_url)
                && value.get("ready").and_then(Value::as_bool) == Some(true)
            {
                let html = value.get("html").and_then(Value::as_str).unwrap_or("");
                if !html.is_empty() {
                    return Ok(html.to_owned());
                }
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(error("Forumseite bleibt in der Browserprüfung oder liefert keine öffentlichen Inhalte"));
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        if self.target_id.is_empty() {
            return Ok(());
        }
        let result = self
            .call(
                "Target.closeTarget",
                json!({"targetId":self.target_id}),
                false,
            )
            .await?;
        if result.get("success").and_then(Value::as_bool) != Some(true) {
            return Err(error("Brave hat den Import-Tab nicht geschlossen"));
        }
        self.target_id.clear();
        Ok(())
    }

    async fn call(&mut self, method: &str, params: Value, page: bool) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;
        let mut request = json!({"id":id,"method":method,"params":params});
        if page {
            request["sessionId"] = json!(self.session_id);
        }
        self.socket
            .send(Message::Text(request.to_string().into()))
            .await
            .map_err(|_| error("Brave-Debugverbindung wurde beim Senden getrennt"))?;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
        loop {
            let response = tokio::time::timeout_at(deadline, self.socket.next())
                .await
                .map_err(|_| error("Brave hat den Debugaufruf nicht beantwortet"))?
                .ok_or_else(|| error("Brave-Debugverbindung wurde geschlossen"))?
                .map_err(|_| error("Brave-Debugantwort konnte nicht gelesen werden"))?;
            match response {
                Message::Text(text) => {
                    let response: Value = serde_json::from_str(&text)
                        .map_err(|_| error("Ungültige Brave-Debugantwort"))?;
                    if response.get("id").and_then(Value::as_u64) != Some(id) {
                        continue;
                    }
                    if response.get("error").is_some() {
                        return Err(error("Brave hat den Debugaufruf abgelehnt"));
                    }
                    return Ok(response.get("result").cloned().unwrap_or(Value::Null));
                }
                Message::Ping(data) => self
                    .socket
                    .send(Message::Pong(data))
                    .await
                    .map_err(|_| error("Brave-Debugverbindung wurde getrennt"))?,
                Message::Close(_) => return Err(error("Brave-Debugverbindung wurde geschlossen")),
                _ => {}
            }
        }
    }
}

// Formularwerte und Skripte gehören nicht ins Wissensarchiv.
const SNAPSHOT: &str = r#"(() => {
  const url = location.href;
  const address = new URL(url);
  const path = address.pathname;
  const publicPath = path === '/' || ((path.startsWith('/forums/') || path.startsWith('/threads/')) &&
    !path.includes('%') && !path.includes('//')) ||
    (path.startsWith('/sitemap') && path.endsWith('.xml') && !path.slice(1).includes('/') && !path.includes('%'));
  if (address.origin !== 'https://forums.playdeadlock.com' || address.username || address.password ||
      address.search || address.hash || !publicPath) return {url, ready:false};
  const root = document.documentElement;
  const logged_in = root?.dataset.loggedIn === 'true';
  const xml = document.contentType.includes('xml');
  const sitemap = xml ? document.querySelector('sitemapindex, urlset') : null;
  const publicSession = root?.dataset.loggedIn === 'false';
  const ready = Boolean(document.readyState === 'complete' && (sitemap ||
    (publicSession && (document.querySelector('.js-post') || document.querySelector('.node') ||
     document.querySelector('.structItem--thread')))));
  if (!ready || logged_in) return {url, logged_in, ready:false};
  if (sitemap) return {url, logged_in, ready: true, html:new XMLSerializer().serializeToString(sitemap)};
  const clone = root.cloneNode(true);
  clone.querySelectorAll('script, input, textarea, select, form, meta').forEach(el => el.remove());
  for (const el of [clone, ...clone.querySelectorAll('*')]) {
    for (const attr of [...el.attributes]) {
      if (attr.name.startsWith('on') || /token|csrf|nonce|session/i.test(attr.name)) el.removeAttribute(attr.name);
      if (['href', 'src', 'action', 'formaction'].includes(attr.name)) {
        try {
          const link = new URL(attr.value, url);
          if ([...link.searchParams.keys()].some(key => /token|csrf|nonce|session|auth|key|signature/i.test(key))) {
            el.removeAttribute(attr.name);
          }
        } catch (_) { el.removeAttribute(attr.name); }
      }
    }
  }
  return {url, logged_in, ready, html:clone.outerHTML};
})()"#;

#[cfg(test)]
mod tests {
    use super::{allowed_url, local_endpoint, same_page, ForumBrowser, ForumBrowserConfig};

    #[test]
    fn forum_scope_rejects_other_origins_and_account_routes() {
        for url in [
            "https://forums.playdeadlock.com.evil.test/threads/x.1/",
            "http://forums.playdeadlock.com/",
            "https://forums.playdeadlock.com/login/",
            "https://forums.playdeadlock.com/threads/x.1/?token=secret",
            "https://user@forums.playdeadlock.com/",
            "https://forums.playdeadlock.com/threads/%2e%2e/login/",
        ] {
            assert!(!allowed_url(url), "{url}");
        }
        for url in [
            "https://forums.playdeadlock.com/",
            "https://forums.playdeadlock.com/threads/x.1/page-2",
            "https://forums.playdeadlock.com/forums/bug-reports.6/",
            "https://forums.playdeadlock.com/sitemap-1.xml",
        ] {
            assert!(allowed_url(url), "{url}");
        }
    }

    #[test]
    fn debug_endpoint_is_loopback_only() {
        assert_eq!(
            local_endpoint("9222\n/devtools/browser/abc-123\n").unwrap(),
            "ws://127.0.0.1:9222/devtools/browser/abc-123"
        );
        assert!(local_endpoint("0\n/devtools/browser/abc").is_err());
        assert!(local_endpoint("9222\n/devtools/browser/abc?target=evil").is_err());
    }

    #[test]
    fn canonical_slug_preserves_thread_and_page_identity() {
        assert!(same_page(
            "https://forums.playdeadlock.com/threads/old.123/",
            "https://forums.playdeadlock.com/threads/new.123"
        ));
        assert!(!same_page(
            "https://forums.playdeadlock.com/threads/old.123/page-2",
            "https://forums.playdeadlock.com/threads/new.123/page-3"
        ));
        assert!(!same_page(
            "https://forums.playdeadlock.com/threads/old.123/",
            "https://forums.playdeadlock.com/threads/old.124/"
        ));
    }

    #[test]
    fn pending_consent_requires_the_real_process_socket() {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let client = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
        let (accepted, _) = listener.accept().unwrap();
        assert!(super::owns_debug_socket(std::process::id(), port, true));
        assert!(super::owns_debug_socket(std::process::id(), port, false));
        assert!(!super::owns_debug_socket(u32::MAX, port, false));
        assert!(super::exclusive_debug_client(
            std::process::id(),
            port,
            Some(std::process::id())
        ));
        assert!(!super::exclusive_debug_client(
            u32::MAX,
            port,
            Some(std::process::id())
        ));
        let second_client = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
        let (second_accepted, _) = listener.accept().unwrap();
        assert!(!super::exclusive_debug_client(
            std::process::id(),
            port,
            Some(std::process::id())
        ));
        drop((second_client, second_accepted));
        drop((client, accepted, listener));
    }

    #[test]
    fn another_ipv6_client_prevents_consent() {
        let header = "sl local_address rem_address st tx_queue rx_queue tr tm->when retrnsmt uid timeout inode\n";
        let tcp = format!("{header}0: 0100007F:2406 00000000:0000 0A 0:0 0:0 0 1000 0 11\n1: 0100007F:3456 0100007F:2406 01 0:0 0:0 0 1000 0 12\n2: 0100007F:2406 0100007F:3456 01 0:0 0:0 0 1000 0 13\n");
        assert!(super::exclusive_socket_inodes(&tcp, header, 9222).is_some());
        let tcp6 = format!("{header}0: 00000000000000000000000001000000:3457 00000000000000000000000001000000:2406 01 0:0 0:0 0 1000 0 22\n");
        assert!(super::exclusive_socket_inodes(&tcp, &tcp6, 9222).is_none());
        let additional_ipv4 =
            format!("{tcp}3: 0100007F:3458 0100007F:2406 01 0:0 0:0 0 1000 0 32\n");
        assert!(super::exclusive_socket_inodes(&additional_ipv4, header, 9222).is_none());
    }

    #[tokio::test]
    #[ignore = "Braucht den geöffneten regulären Brave-Browser"]
    async fn public_forum_browser_smoke() {
        let mut browser = ForumBrowser::connect(&ForumBrowserConfig::default())
            .await
            .expect("Öffentliche Brave-Verbindung");
        let result = async {
            let url = "https://forums.playdeadlock.com/threads/posting-bugs.5/";
            let html = match browser.fetch_html(url).await {
                Ok(html) => html,
                Err(err) => {
                    let diagnosis = browser.call("Runtime.evaluate", serde_json::json!({
                        "expression": "(() => { if(location.origin !== 'https://forums.playdeadlock.com') return { official:false }; return {official:true, complete:document.readyState==='complete',guest:document.documentElement.dataset.loggedIn==='false',loggedIn:document.documentElement.dataset.loggedIn==='true',posts:document.querySelectorAll('.js-post').length,nodes:document.querySelectorAll('.node').length,challenge:/checking your browser|just a moment/i.test(document.title)}; })()",
                        "returnByValue":true
                    }), true).await?;
                    println!("Öffentliche Forum-Diagnose: {}", diagnosis.pointer("/result/value").unwrap_or(&serde_json::Value::Null));
                    return Err(err);
                }
            };
            let page = scraper::Html::parse_document(&html);
            let posts = scraper::Selector::parse(".js-post").unwrap();
            let bodies = scraper::Selector::parse(".message-body .bbWrapper").unwrap();
            let count = page.select(&posts).count();
            let text_len: usize = page
                .select(&bodies)
                .map(|body| body.text().collect::<String>().len())
                .sum();
            assert!(count > 0 && text_len > 0, "Echte Forenbeiträge fehlen");
            println!("Öffentliche Inhaltsprobe: {url}, {count} Beiträge, {text_len} Textbytes");
            Ok::<(), super::ForumBrowserError>(())
        }
        .await;
        let closed = browser.close().await;
        result.expect("Öffentliche Forum-Inhaltsprobe");
        closed.expect("Import-Tab schließen");
    }
}
