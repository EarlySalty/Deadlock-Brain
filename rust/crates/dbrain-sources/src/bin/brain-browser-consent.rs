//! Bestätigt den nativen Brave-Dialog über AT-SPI ohne Webseitenzugriff.
use serde::Deserialize;
use std::{
    collections::BTreeSet,
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Config {
    accessibility_bus: String,
    interval_seconds: u64,
    command_timeout_seconds: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            accessibility_bus: "unix:path=/home/nathanael/.cache/at-spi/bus_10.0".into(),
            interval_seconds: 10,
            command_timeout_seconds: 5,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Object {
    bus: String,
    path: String,
}
fn objects(value: &str) -> Result<Vec<Object>> {
    let pattern = regex::Regex::new(
        r"\('(:[0-9]+\.[0-9]+)', (?:objectpath )?'(/org/a11y/atspi/accessible/(?:root|[0-9]+))'\)",
    )?;
    Ok(pattern
        .captures_iter(value)
        .map(|c| Object {
            bus: c[1].into(),
            path: c[2].into(),
        })
        .collect())
}
fn call(config: &Config, object: &Object, method: &str, extra: &[&str]) -> Result<String> {
    let mut args = vec![
        "call",
        "--address",
        &config.accessibility_bus,
        "--dest",
        &object.bus,
        "--object-path",
        &object.path,
        "--method",
        method,
    ];
    args.extend_from_slice(extra);
    text(config, "/usr/bin/gdbus", &args)
}
fn number(value: &str) -> Result<u32> {
    Ok(value
        .trim()
        .strip_prefix("(uint32 ")
        .and_then(|s| s.strip_suffix(",)"))
        .ok_or("Ungültige native Zahl")?
        .parse()?)
}
fn role(config: &Config, object: &Object) -> Result<u32> {
    number(&call(
        config,
        object,
        "org.a11y.atspi.Accessible.GetRole",
        &[],
    )?)
}
fn children(config: &Config, object: &Object) -> Result<Vec<Object>> {
    objects(&call(
        config,
        object,
        "org.a11y.atspi.Accessible.GetChildren",
        &[],
    )?)
}
fn named(config: &Config, object: &Object, expected: &str) -> Result<bool> {
    Ok(call(
        config,
        object,
        "org.freedesktop.DBus.Properties.Get",
        &["org.a11y.atspi.Accessible", "Name"],
    )?
    .trim()
        == format!("(<'{expected}'>,)"))
}
fn native_container(role: u32) -> bool {
    matches!(role, 75 | 23 | 39)
}
fn native_button(config: &Config, alert: &Object) -> Result<Option<Object>> {
    let mut queue = children(config, alert)?;
    let mut seen = BTreeSet::new();
    let mut buttons = Vec::new();
    while let Some(object) = queue.pop() {
        if object.bus != alert.bus || seen.len() > 128 {
            return Err("Mehrdeutiger nativer Dialogbaum".into());
        }
        if !seen.insert(object.clone()) {
            continue;
        }
        match role(config, &object)? {
            39 => queue.extend(children(config, &object)?),
            43 if named(config, &object, "Allow")? => buttons.push(object),
            _ => {} // Dokumente, Eingaben und deren Kinder werden nicht gelesen.
        }
    }
    Ok(if buttons.len() == 1 {
        buttons.pop()
    } else {
        None
    })
}
fn find_alerts(config: &Config, root: &Object) -> Result<Vec<(Object, Object)>> {
    let mut queue = vec![root.clone()];
    let mut seen = BTreeSet::new();
    let mut found = Vec::new();
    while let Some(object) = queue.pop() {
        if object.bus != root.bus || seen.len() > 512 {
            return Err("Mehrdeutiger nativer Fensterbaum".into());
        }
        if !seen.insert(object.clone()) {
            continue;
        }
        let native_role = role(config, &object)?;
        if native_role == 2 && named(config, &object, "Allow remote debugging?")? {
            if let Some(button) = native_button(config, &object)? {
                found.push((object, button));
            }
        } else if native_container(native_role) {
            queue.extend(children(config, &object)?);
        }
    }
    Ok(found)
}
fn owner_pid(config: &Config, bus: &str) -> Result<u32> {
    number(&call(
        config,
        &Object {
            bus: "org.freedesktop.DBus".into(),
            path: "/org/freedesktop/DBus".into(),
        },
        "org.freedesktop.DBus.GetConnectionUnixProcessID",
        &[bus],
    )?)
}
fn confirm_once(config: &Config) -> Result<usize> {
    if !dbrain_sources::forum_browser::has_pending_consent() {
        return Ok(0);
    }
    let registry = Object {
        bus: "org.a11y.atspi.Registry".into(),
        path: "/org/a11y/atspi/accessible/root".into(),
    };
    let mut found = Vec::new();
    for app in children(config, &registry)? {
        let pid = owner_pid(config, &app.bus)?;
        if std::fs::read_to_string(format!("/proc/{pid}/comm"))?.trim() != "brave"
            || !dbrain_sources::forum_browser::has_pending_consent_for_brave(pid)
        {
            continue;
        }
        if role(config, &app)? != 75 {
            continue;
        }
        for (alert, button) in find_alerts(config, &app)? {
            found.push((app.clone(), pid, alert, button));
        }
    }
    if found.len() != 1 {
        return Ok(0);
    }
    let (app, pid, alert, button) = found.pop().ok_or("Nativer Dialog fehlt")?;
    // Unmittelbar vor der nativen Aktion werden Identität und Hierarchie erneut geprüft.
    if owner_pid(config, &app.bus)? != pid
        || find_alerts(config, &app)? != vec![(alert.clone(), button.clone())]
        || !dbrain_sources::forum_browser::has_pending_consent_for_brave(pid)
    {
        return Ok(0);
    }
    if call(config, &button, "org.a11y.atspi.Action.DoAction", &["0"])?.trim() != "(true,)" {
        return Err("Native Brave-Aktion wurde abgelehnt".into());
    }
    Ok(1)
}
fn run(config: &Config, program: &str, args: &[&str], limit: usize) -> Result<Vec<u8>> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    command.stdin(Stdio::null());
    let mut child = command.spawn()?;
    let stdout = child.stdout.take().ok_or("Werkzeugausgabe fehlt")?;
    let reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout
            .take(limit as u64 + 1)
            .read_to_end(&mut bytes)
            .map(|_| bytes)
    });
    let deadline = Instant::now() + Duration::from_secs(config.command_timeout_seconds);
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            child.kill()?;
            child.wait()?;
            let _ = reader.join();
            return Err("Desktopwerkzeug hat die Ladefrist überschritten".into());
        }
        thread::sleep(Duration::from_millis(20));
    };
    let bytes = reader
        .join()
        .map_err(|_| "Werkzeugausgabe wurde unterbrochen")??;
    if !status.success() || bytes.len() > limit {
        return Err("Desktopwerkzeug hat keine begrenzte gültige Antwort geliefert".into());
    }
    Ok(bytes)
}

fn text(config: &Config, program: &str, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8(run(config, program, args, 256 * 1024)?)?)
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() != Some("--config") {
        return Err("Aufruf: brain-browser-consent --config <JSON-Datei> [--once]".into());
    }
    let path = PathBuf::from(args.next().ok_or("Config-Datei fehlt")?);
    let once = match args.next().as_deref() {
        None => false,
        Some("--once") => true,
        _ => return Err("Unbekanntes Aufrufargument".into()),
    };
    if args.next().is_some() {
        return Err("Unbekannte Aufrufargumente".into());
    }
    let config: Config = serde_json::from_reader(std::fs::File::open(path)?)?;
    if !(1..=60).contains(&config.interval_seconds)
        || !(1..=10).contains(&config.command_timeout_seconds)
        || !config.accessibility_bus.starts_with("unix:path=/")
        || config.accessibility_bus.contains([',', '\n', '\r'])
    {
        return Err("Ungültige lokale Display- oder Zeitkonfiguration".into());
    }
    if once {
        println!("Brave-Freigaben bestätigt: {}", confirm_once(&config)?);
        return Ok(());
    }
    println!("Brave-Freigabewächter läuft");
    let mut failed = false;
    loop {
        match confirm_once(&config) {
            Ok(count) => {
                if failed {
                    println!("Brave-Freigabewächter erreicht das Desktopfenster wieder");
                }
                failed = false;
                if count > 0 {
                    println!("Sichtbaren Brave-Remote-Debugging-Dialog bestätigt");
                }
            }
            Err(_) => {
                if !failed {
                    eprintln!("Brave-Freigabewächter wartet auf ein erreichbares Desktopfenster");
                }
                failed = true;
            }
        }
        thread::sleep(Duration::from_secs(config.interval_seconds));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dokumentrollen_bleiben_unberuehrt() {
        for role in [95, 15, 92, 93, 94, 61] {
            assert!(!native_container(role));
        }
    }
    #[test]
    fn native_objekte_werden_eindeutig_gelesen() {
        let parsed = objects("([(':1.138', objectpath '/org/a11y/atspi/accessible/root'), (':1.138', '/org/a11y/atspi/accessible/19')],)").unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[1].path, "/org/a11y/atspi/accessible/19");
        assert!(objects("[('fremd', '/document')] ").unwrap().is_empty());
    }
}
