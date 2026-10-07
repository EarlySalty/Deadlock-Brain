status: beauftragt
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/Deadlock-Twitch-Bot-brain-consumer-fertig

# Twitch: bestehende vollständige Clippyprüfung nachziehen

## Ziel

Den bereits implementierten engen Konfigurationsbau ffa037c885ca45c0b7a43759ed6714305ed92f99 erhalten und die vollständige tb-config-Prüfung abschließen. Dein erhaltener Kontext und bisherige Bauwirkung bleiben gültig. Kein neuer Thread oder Modellwechsel. Kein Merge-Gatefund, sondern dein offener Prüffehler aus dem eigenen Abschlussbericht.

## Schreibgrenze

Ein zusätzlich vorab in AN_HAUPT.md angekündigter Pfad: rust/crates/tb-config/src/dashboard_options.rs, ausschließlich den bereits bestehenden Testaufbau bei Zeile 363 zu einem Clippy-konformen Initialisierer umbauen. Graphify zuerst, Stelle und Ausgangsblob bestätigen. Keine produktiven Configregeln, zusätzliche Felder, Kommentare oder Lint-Unterdrückungen. Bestehende Tests, Erwartungen und Ausführung erhalten. Der abgeschlossene Brain-CLI-/Editorbau bleibt unverändert, sofern kein tatsächlicher eigener Prüffehler eine enge Korrektur verlangt; Abweichung zuerst mit genauer Stelle melden.

## Prüfung

Rustup und höchstens zwei Jobs. Beide Hostlocks blockierend in vorgeschriebener Reihenfolge halten, unmittelbar vor jedem Compilerstart konservativ HOSTPROBE.md prüfen. Warten auf Locks nicht mit einer Deadline beenden, fremde Prozesse nicht stoppen. Bestehenden zentralen Buildpfad /home/nathanael/.cache/twitch-all-live-target benutzen.

Danach vollständiges cargo clippy für alle tb-config-Ziele mit -D warnings und gesamte tb-config-Suite einschließlich ignorierter Tests mit --include-ignored --test-threads=2, dazu Formatprüfung und git diff --check. Keine Filterung des fehlgeschlagenen Clippyziels. Prüflogs und Rust-/Cargo-Version an den tatsächlich geprüften SHA binden. Eventuelle neue Fehler wahrheitsgetreu mit genauer Stelle melden. Keine zusätzliche Testpflicht oder Security-/Bugreview.

## Git und Ausgabe

Eigene Änderung committieren, Git-Schritte einzeln mit literalem absolutem Worktreepfad. Commit mit Co-Authored-By: Claude Code <noreply@anthropic.com> abschließen. Kein Push, Merge, Deploy, Installer- oder Configaufruf, keine Nachricht. Abschließend vollen neuen SHA, sauberen Baum, genaue Exitcodes/Testzahlen und Belegpfade zurückgeben. Vorhandene Prüflogs aus /tmp/tb-config-final.8QvC37 nicht löschen. Keine aktiven eigenen Kinder oder offenen Lockdeskriptoren zurücklassen.

## Routing

teil-k, Versuch 1. Keine weiteren Agenten oder T3-Threads. Haupt e6c19079-657e-4db9-80bd-8e1313e7f785, gemeinsame Integration und unabhängige Abnahme bei Z. REGISTER.md und TODO.md nicht schreiben. Deutsch mit echten Umlauten, humanizer und no-em-dashes.
