//! Bestätigt ausschließlich den sichtbaren Remote-Debugging-Dialog von Brave.
//! Bilder und OCR-Ergebnisse bleiben im Speicher.

use std::{
    collections::BTreeMap,
    ffi::{c_char, c_int, c_ulong, c_void, CString},
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Config {
    display: String,
    interval_seconds: u64,
    command_timeout_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: ":10".into(),
            interval_seconds: 10,
            command_timeout_seconds: 5,
        }
    }
}

#[derive(Clone, Debug)]
struct Word {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    confidence: f32,
    text: String,
}

fn button_from_tsv(tsv: &str, image_width: i32, image_height: i32) -> Option<(i32, i32)> {
    let mut lines: BTreeMap<String, Vec<Word>> = BTreeMap::new();
    for line in tsv.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 12 || fields[0] != "5" || fields[11].is_empty() {
            continue;
        }
        let word = Word {
            x: fields[6].parse().ok()?,
            y: fields[7].parse().ok()?,
            width: fields[8].parse().ok()?,
            height: fields[9].parse().ok()?,
            confidence: fields[10].parse().ok()?,
            text: fields[11].to_owned(),
        };
        if !word.confidence.is_finite()
            || word.x < 0
            || word.y < 0
            || word.width <= 0
            || word.height <= 0
            || word.x.checked_add(word.width)? > image_width
            || word.y.checked_add(word.height)? > image_height
        {
            return None;
        }
        lines.entry(fields[1..5].join(":")).or_default().push(word);
    }
    let rows: Vec<_> = lines
        .values()
        .map(|words| {
            (
                words
                    .iter()
                    .map(|w| w.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
                words,
            )
        })
        .collect();
    let prompts: Vec<_> = rows
        .iter()
        .filter(|(text, words)| {
            text == "Allow remote debugging?" && words.iter().all(|w| w.confidence >= 85.0)
        })
        .collect();
    let buttons: Vec<_> = rows
        .iter()
        .filter(|(text, words)| text == "Allow" && words.len() == 1 && words[0].confidence >= 85.0)
        .collect();
    if prompts.len() != 1 || buttons.len() != 1 {
        return None;
    }
    let prompt = &prompts[0].1[0];
    let button = &buttons[0].1[0];
    if button.y < prompt.y + 70
        || button.y > prompt.y + 360
        || button.x < prompt.x
        || button.x > prompt.x + 650
    {
        return None;
    }
    Some((button.x + button.width / 2, button.y + button.height / 2))
}

fn run(
    config: &Config,
    program: &str,
    args: &[&str],
    input: Option<Vec<u8>>,
    limit: usize,
) -> Result<Vec<u8>> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    command.stdin(if input.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    });
    let mut child = command.spawn()?;
    let stdout = child.stdout.take().ok_or("Werkzeugausgabe fehlt")?;
    let reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout
            .take(limit as u64 + 1)
            .read_to_end(&mut bytes)
            .map(|_| bytes)
    });
    let writer = if let Some(bytes) = input {
        let mut stdin = child.stdin.take().ok_or("Werkzeugeingabe fehlt")?;
        Some(thread::spawn(move || stdin.write_all(&bytes)))
    } else {
        None
    };
    let deadline = Instant::now() + Duration::from_secs(config.command_timeout_seconds);
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            child.kill()?;
            child.wait()?;
            let _ = reader.join();
            if let Some(writer) = writer {
                let _ = writer.join();
            }
            return Err("Desktopwerkzeug hat die Ladefrist überschritten".into());
        }
        thread::sleep(Duration::from_millis(20));
    };
    let bytes = reader
        .join()
        .map_err(|_| "Werkzeugausgabe wurde unterbrochen")??;
    if let Some(writer) = writer {
        writer
            .join()
            .map_err(|_| "Bildeingabe wurde unterbrochen")??;
    }
    if !status.success() || bytes.len() > limit {
        return Err("Desktopwerkzeug hat keine begrenzte gültige Antwort geliefert".into());
    }
    Ok(bytes)
}

fn text(config: &Config, program: &str, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8(run(
        config,
        program,
        args,
        None,
        256 * 1024,
    )?)?)
}

fn brave_pid(config: &Config, id: &str) -> Result<Option<u32>> {
    let properties = text(
        config,
        "/usr/bin/xprop",
        &[
            "-display",
            &config.display,
            "-id",
            id,
            "_NET_WM_PID",
            "WM_CLASS",
        ],
    )?;
    if !properties
        .lines()
        .any(|line| line == "WM_CLASS(STRING) = \"brave-browser\", \"Brave-browser\"")
    {
        return Ok(None);
    }
    let pid = properties
        .lines()
        .find_map(|line| line.strip_prefix("_NET_WM_PID(CARDINAL) = "))
        .and_then(|value| value.parse::<u32>().ok());
    let Some(pid) = pid else { return Ok(None) };
    let process = std::fs::read_to_string(format!("/proc/{pid}/comm"))?;
    if process.trim() != "brave" {
        return Ok(None);
    }
    // Der Eigentümer des Fensters muss im selben Benutzerkonto laufen.
    let status = std::fs::read_to_string(format!("/proc/{pid}/status"))?;
    let owner = status
        .lines()
        .find_map(|line| line.strip_prefix("Uid:\t"))
        .and_then(|line| line.split_whitespace().next())
        .and_then(|id| id.parse::<u32>().ok());
    let current = std::fs::read_to_string("/proc/self/status")?;
    let own_uid = current
        .lines()
        .find_map(|line| line.strip_prefix("Uid:\t"))
        .and_then(|line| line.split_whitespace().next())
        .and_then(|id| id.parse::<u32>().ok());
    Ok((owner.is_some() && owner == own_uid).then_some(pid))
}

fn detect(config: &Config, id: &str) -> Result<Option<(i32, i32)>> {
    let Some(pid) = brave_pid(config, id)? else {
        return Ok(None);
    };
    if !dbrain_sources::forum_browser::has_pending_consent_for_brave(pid) {
        return Ok(None);
    }
    let geometry = text(
        config,
        "/usr/bin/xwininfo",
        &["-display", &config.display, "-id", id],
    )?;
    let dimension = |prefix: &str| {
        geometry
            .lines()
            .find_map(|line| line.trim().strip_prefix(prefix))
            .and_then(|value| value.parse::<i32>().ok())
    };
    let width = dimension("Width: ").ok_or("Fensterbreite fehlt")?;
    let height = dimension("Height: ").ok_or("Fensterhöhe fehlt")?;
    if !(1000..=8000).contains(&width) || !(600..=8000).contains(&height) {
        return Ok(None);
    }
    let crop_x = (width - 1000) / 2;
    let crop_y = 60;
    let crop = format!("1000x420+{crop_x}+{crop_y}");
    let image = run(
        config,
        "/usr/bin/import",
        &[
            "-limit",
            "memory",
            "128MiB",
            "-limit",
            "map",
            "0",
            "-limit",
            "disk",
            "0",
            "-display",
            &config.display,
            "-window",
            id,
            "-crop",
            &crop,
            "png:-",
        ],
        None,
        8 * 1024 * 1024,
    )?;
    let tsv = String::from_utf8(run(
        config,
        "/usr/bin/tesseract",
        &["stdin", "stdout", "--psm", "11", "tsv"],
        Some(image),
        1024 * 1024,
    )?)?;
    Ok(button_from_tsv(&tsv, 1000, 420).map(|(x, y)| (x + crop_x, y + crop_y)))
}

fn confirm_once(config: &Config) -> Result<usize> {
    if !dbrain_sources::forum_browser::has_pending_consent() {
        return Ok(0);
    }
    let tree = text(
        config,
        "/usr/bin/xwininfo",
        &["-display", &config.display, "-root", "-tree"],
    )?;
    let mut matches = Vec::new();
    for line in tree.lines() {
        let line = line.trim();
        let Some(id) = line
            .split_whitespace()
            .next()
            .filter(|id| id.starts_with("0x") && id[2..].bytes().all(|b| b.is_ascii_hexdigit()))
        else {
            continue;
        };
        if !line.contains("\"brave-browser\" \"Brave-browser\"") {
            continue;
        }
        if let Some(position) = detect(config, id)? {
            matches.push((id.to_owned(), position));
        }
    }
    if matches.len() != 1 {
        return Ok(0);
    }
    let (id, position) = &matches[0];
    // Nach der ersten Erkennung wird der Dialog vor dem Klick erneut geprüft.
    if detect(config, id)? != Some(*position) {
        return Ok(0);
    }
    if !dbrain_sources::forum_browser::has_pending_consent() {
        return Ok(0);
    }
    let Some(pid) = brave_pid(config, id)? else {
        return Ok(0);
    };
    if !dbrain_sources::forum_browser::has_pending_consent_for_brave(pid) {
        return Ok(0);
    }
    let window = c_ulong::from_str_radix(&id[2..], 16)?;
    click_native(&config.display, window, position.0, position.1)?;
    thread::sleep(Duration::from_millis(300));
    if detect(config, id)?.is_some() {
        return Err("Brave hat die erkannte Freigabe nicht geschlossen".into());
    }
    Ok(1)
}

#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

struct Library(*mut c_void);
impl Library {
    fn open(name: &str) -> Result<Self> {
        let name = CString::new(name)?;
        // SAFETY: Der Bibliotheksname ist eine gültige CString aus einer festen Liste im Code.
        let pointer = unsafe { dlopen(name.as_ptr(), 2) };
        if pointer.is_null() {
            return Err("Native X11-Bibliothek fehlt".into());
        }
        Ok(Self(pointer))
    }
}
impl Drop for Library {
    fn drop(&mut self) {
        // SAFETY: Der Handle stammt aus einem erfolgreichen dlopen und wird genau einmal geschlossen.
        unsafe {
            dlclose(self.0);
        }
    }
}

fn click_native(display: &str, window: c_ulong, x: i32, y: i32) -> Result<()> {
    let x11 = Library::open("libX11.so.6")?;
    let xtst = Library::open("libXtst.so.6")?;
    macro_rules! symbol {
        ($lib:ident, $name:literal, $ty:ty) => {{
            // SAFETY: Handle bleibt offen und der fest vorgegebene Symbolname ist nullterminiert.
            let pointer = unsafe { dlsym($lib.0, concat!($name, "\0").as_ptr().cast()) };
            if pointer.is_null() {
                return Err("Native X11-Funktion fehlt".into());
            }
            // SAFETY: Symbolnamen und Funktionssignaturen entsprechen den X11-/XTest-Headern.
            unsafe { std::mem::transmute::<*mut c_void, $ty>(pointer) }
        }};
    }
    type Display = *mut c_void;
    let open = symbol!(
        x11,
        "XOpenDisplay",
        unsafe extern "C" fn(*const c_char) -> Display
    );
    let close = symbol!(x11, "XCloseDisplay", unsafe extern "C" fn(Display) -> c_int);
    let root = symbol!(
        x11,
        "XDefaultRootWindow",
        unsafe extern "C" fn(Display) -> c_ulong
    );
    let translate = symbol!(
        x11,
        "XTranslateCoordinates",
        unsafe extern "C" fn(
            Display,
            c_ulong,
            c_ulong,
            c_int,
            c_int,
            *mut c_int,
            *mut c_int,
            *mut c_ulong,
        ) -> c_int
    );
    let focus = symbol!(
        x11,
        "XSetInputFocus",
        unsafe extern "C" fn(Display, c_ulong, c_int, c_ulong) -> c_int
    );
    let raise = symbol!(
        x11,
        "XRaiseWindow",
        unsafe extern "C" fn(Display, c_ulong) -> c_int
    );
    let motion = symbol!(
        xtst,
        "XTestFakeMotionEvent",
        unsafe extern "C" fn(Display, c_int, c_int, c_int, c_ulong) -> c_int
    );
    let button = symbol!(
        xtst,
        "XTestFakeButtonEvent",
        unsafe extern "C" fn(Display, u32, c_int, c_ulong) -> c_int
    );
    let sync = symbol!(x11, "XSync", unsafe extern "C" fn(Display, c_int) -> c_int);
    let display = CString::new(display)?;
    // SAFETY: Der Displayname ist eine gültige CString; XOpenDisplay liefert einen eigenen Handle.
    let display = unsafe { open(display.as_ptr()) };
    if display.is_null() {
        return Err("Native X11-Verbindung fehlgeschlagen".into());
    }
    let mut root_x = 0;
    let mut root_y = 0;
    let mut child = 0;
    // SAFETY: Der Displayhandle ist gültig, Fenster-ID ist geprüft, alle Ausgabepointer sind initialisiert.
    let mapped = unsafe {
        translate(
            display,
            window,
            root(display),
            x,
            y,
            &mut root_x,
            &mut root_y,
            &mut child,
        )
    };
    if mapped == 0 {
        // SAFETY: Der eigene Displayhandle ist gültig und wird anschließend nicht weiter benutzt.
        unsafe {
            close(display);
        }
        return Err("Dialogposition konnte nicht übersetzt werden".into());
    }
    // SAFETY: Der Displayhandle ist gültig; ausschließlich das geprüfte Fenster erhält den Fokus.
    let moved = unsafe {
        raise(display, window);
        focus(display, window, 2, 0);
        sync(display, 0);
        motion(display, -1, root_x, root_y, 0)
    };
    if moved == 0 {
        // SAFETY: Der eigene Displayhandle ist gültig; bei fehlgeschlagener Bewegung folgt kein Klick.
        unsafe {
            close(display);
        }
        return Err("Native Mausbewegung zum Dialog fehlgeschlagen".into());
    }
    // SAFETY: Zielkoordinaten wurden erfolgreich gesetzt; der Handle wird nach Freigabe der Taste geschlossen.
    let succeeded = unsafe {
        let pressed = button(display, 1, 1, 0);
        let released = button(display, 1, 0, 0);
        sync(display, 0);
        close(display);
        pressed != 0 && released != 0
    };
    if !succeeded {
        return Err("Native Dialogbestätigung fehlgeschlagen".into());
    }
    Ok(())
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
    if !config.display.starts_with(':')
        || !config.display[1..]
            .bytes()
            .all(|b| b.is_ascii_digit() || b == b'.')
        || !(1..=60).contains(&config.interval_seconds)
        || !(1..=10).contains(&config.command_timeout_seconds)
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
    use super::button_from_tsv;

    fn fixture(extra: &str) -> String {
        format!("level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\twidth\theight\tconf\ttext\n5\t1\t1\t1\t1\t1\t100\t40\t40\t20\t96\tAllow\n5\t1\t1\t1\t1\t2\t150\t40\t60\t20\t96\tremote\n5\t1\t1\t1\t1\t3\t220\t40\t100\t20\t96\tdebugging?\n5\t1\t2\t1\t1\t1\t450\t240\t40\t20\t96\tAllow\n{extra}")
    }
    #[test]
    fn exact_dialog_and_isolated_button_required() {
        assert_eq!(button_from_tsv(&fixture(""), 1000, 420), Some((470, 250)));
        assert_eq!(
            button_from_tsv(&fixture("").replace("debugging?", "downloads?"), 1000, 420),
            None
        );
        assert_eq!(
            button_from_tsv(&fixture("").replace("96\tAllow", "40\tAllow"), 1000, 420),
            None
        );
    }
    #[test]
    fn duplicate_or_outside_buttons_are_rejected() {
        let extra = "5\t1\t3\t1\t1\t1\t600\t240\t40\t20\t96\tAllow\n";
        assert_eq!(button_from_tsv(&fixture(extra), 1000, 420), None);
        assert_eq!(
            button_from_tsv(&fixture("").replace("450\t240", "990\t240"), 1000, 420),
            None
        );
        assert_eq!(
            button_from_tsv(&fixture("").replace("450\t240", "450\t20"), 1000, 420),
            None
        );
    }
}
