status: aktiv
Datum: 2026-10-03
Vertrag: wiki-spielwissen-v1
Schreiber: ausschließlich Teilbereich C

# Vertrag für A, B und C

## Dateieigentum

Der vorhandene Crate `rust/crates/dbrain-sources/` nimmt beide neuen Quellenmodule auf. Bestehende Bausteine zuerst per Graphify suchen und wiederverwenden.

| Bereich | Ausschließliche neue Schreibpfade |
| --- | --- |
| A | `rust/crates/dbrain-sources/src/wiki_inventory.rs`, `rust/crates/dbrain-sources/src/wiki_inventory/`, eigene Dokumentation unter `bereiche/a/` |
| B | `rust/crates/dbrain-sources/src/game_files.rs`, `rust/crates/dbrain-sources/src/game_files/`, eigene Dokumentation unter `bereiche/b/` |
| C | Sämtliche vorhandenen Dateien, Cargo-Manifeste und Lockfile, Modulregistrierungen, gemeinsame Datentypen, Migrationen, Datenbankzugriff, Wissenslesepfad, CLI und Deployment |
| D | Eigene Bereichsartefakte `bereiche/d/` und eigener Downloadbestand außerhalb Git; neue Rust-Werkzeuge erst nach konkreter schriftlicher Pfadzuweisung, keine A/B/C-Dateien |

D übernimmt nach AN_BEREICHE.md Punkte 17 bis 20 und PAKETE.md den regulären kostenlosen Deadlock-Lizenztask, die Berechtigungsprüfung und den vollständigen Spieldownload. Die Lizenzgewährung ist vom Hauptorchestrator mit Task 4931460, EResult 1, App 1422450 bestätigt; sie ist kein vollständiger Downloadbeweis. B führt keine konkurrierenden Zugangsversuche aus und behält Parser/Fixes/Datenläufe. D übergibt tatsächliche Rohdateipfade, belegte App-/Build-/Depot-/Manifestkennungen und Originalhashes an B/C beziehungsweise belegte negative Antworten. B extrahiert zusätzliche D-Dateien nach demselben JSONL-Vertrag; C berücksichtigt D beim Gesamtabschluss. Kein Kauf, Klartext-Secrets, Verdrängen einer laufenden Steam-Sitzung oder selbstständiger D-Merge/Deploy.

Zusätzliche genaue Zuweisung des Hauptorchestrators: D arbeitet im Repo Deadlock-Steam-Bot unter `/home/nathanael/.worktrees/steam-brain-spieldepot-d`, Branch `feat/brain-spieldepot-download`, ausschließlich an `rust/crates/steam-core/src/task/handlers/game_download.rs`, `game_download/` und nötigen Registrierungszeilen in `handlers/mod.rs` und `task/mod.rs`. Weitere bestehende Dateien oder Abhängigkeiten benötigen konkrete Freigabe durch den Hauptorchestrator. Bestehende Steam-Verbindung und Konfigurationswege nutzen; kein Zweitlogin, Sessionexport oder frei wählbarer Zielpfad beziehungsweise beliebige Task-URL. C integriert auch dieses eigene Steam-Paket nach gemeinsamer Prüfung mit Bs Lesepfad und unabhängigen Sol-high-Gates für die zusammengehörigen SHAs. Der nötige Steam-Deploy und echte Downloadtask gehen dem abschließenden Brain-Import voraus. Höhere Harness-Grenzen bleiben erhalten.

Vorhandene Dateien dürfen A/B erst nach einer konkreten schriftlichen Pfadzuweisung durch C ändern. Bei bereits belegtem neuen Modulpfad melden sie den Konflikt, statt ihn zu überschreiben. Die unabhängige Datensammlung läuft weiter. A/B liefern eigene lokale Commits; nur C integriert, mergt und deployt.

C arbeitet in `/home/nathanael/.worktrees/brain-wiki-spielwissen-c`, Branch `feat/brain-wiki-spielwissen-c`, Ausgangsstand `2734c2da4e814ff79953e8e825275b0216a6af16`. Fremde Änderungen im Hauptcheckout bleiben unberührt. Vor Integration wird der aktuelle Stand von origin/main geprüft.

## Datenaustausch

A/B liefern UTF-8-JSONL außerhalb Git. Jede Zeile enthält ein Dokument nach `wiki-spielwissen-v1`. C validiert und importiert in die vorhandene Postgres-Wissenshaltung. Eine zusätzliche Ersatzdatenbank erfüllt den Auftrag nicht.

Pflichtfelder und Datentypen:

```json
{
  "contract_version": "wiki-spielwissen-v1",
  "source_kind": "wiki",
  "source_id": "deadlock-wiki",
  "document_id": "wiki:deadlock-wiki:page:123",
  "source_locator": "https://deadlock.wiki/Example",
  "title": "Example",
  "language": "en",
  "revision": "456",
  "observed_at": "2026-10-03T12:00:00Z",
  "content_sha256": "<64 lowercase hex characters>",
  "content": "Originaler oder nachvollziehbar extrahierter Quellentext",
  "evidence_status": "source_statement",
  "license": {
    "name": "unverified",
    "url": null,
    "attribution": "Quellenautor oder Quellenanbieter",
    "redistribution_allowed": false
  },
  "metadata": {},
  "facts": []
}
```

`source_kind` ist `wiki` oder `game_file`. Alle dargestellten äußeren Felder sind verpflichtend. `metadata` ist ein JSON-Objekt; `facts` eine Liste. `license.url` darf null sein. Nicht belegte Sprache wird als `und` angegeben.

`revision` ist die belegte Wiki-Revision beziehungsweise Build-/Manifestkennung. Fehlt sie, lautet sie `unknown:<content_sha256>`; diese Fälle werden in der Abdeckung gesondert gezählt. Ein Abrufdatum ersetzt keine Quellenversion. `observed_at` ist UTC nach RFC 3339. Der Hash wird über exakt die UTF-8-Bytes des übergebenen `content` berechnet.

Wiki-IDs verwenden die stabile MediaWiki-Seiten-ID: `wiki:<source_id>:page:<page_id>`. Ohne belegte Seiten-ID: `wiki:<source_id>:url:<sha256 der kanonischen URL>`. Spieldatei-IDs: `game:<belegte app_id>:<relativer normalisierter Depotpfad>`. IDs bleiben über Revisionen stabil. Keine App-ID raten. Keine absoluten lokalen Benutzerpfade als öffentliche Quellenangabe verwenden.

Für Spieldateien enthält `metadata` belegte App-ID, Build-/Manifeststand oder ausdrücklichen unbekannten Stand, Depot-ID soweit verfügbar, relativen Dateipfad, Originaldatei-SHA-256, Extraktionsverfahren und dessen Version. `source_locator` ist die belegte Steam-Quellenkennung oder ein relativer Depotpfad. Originaldateien und geschützte Assets bleiben außerhalb Git.

Ein strukturierter Fakt:

```json
{
  "fact_id": "stable identifier within the document",
  "subject": "hero:internal_id_or_source_name",
  "predicate": "ability.cooldown",
  "value": 12,
  "unit": "seconds",
  "evidence_status": "extracted_value",
  "source_span": "relative/path.txt:section/key",
  "qualifiers": {}
}
```

`value` ist ein JSON-Wert. `unit` und `source_span` dürfen null sein, wenn tatsächlich unbekannt; die übrigen Faktfelder sind verpflichtend. Einheit und Bedingungen bleiben erhalten. Ohne belegte Einheit nicht umrechnen. Zulässige Belegzustände für Dokumente und Fakten sind `extracted_value`, `source_statement` und `hypothesis`. Ableitungen ausdrücklich markieren. Binärdateien belegen allein keine ausgeführte Spiellogik.

Dokumente ohne passende strukturierte Fakten werden als quellengestützte Wissensdokumente erhalten. Widersprüche zwischen Quellen und Versionen bleiben getrennt nachvollziehbar.

## Rust-Module

A/B liefern öffentliche Konfigurationstypen und Funktionen zur jeweiligen Sammlung beziehungsweise Extraktion. Sie verwenden vorhandene Crate-Abhängigkeiten; benötigte neue Abhängigkeiten melden sie C. Keine Datenbankverbindungen, keine neuen LLM-Aufrufe, keine Änderung von `lib.rs` oder Cargo durch A/B. Die genaue aufrufbare Signatur dokumentieren sie in ihrer Übergabe. C registriert Module und CLI-Aufrufe.

Die JSONL-Schnittstelle ist sofort verbindlich und unabhängig von der Modulregistrierung. A/B können sie mit `serde_json::Value` erzeugen; C verantwortet gemeinsame typisierte Validierung. Rust ist die einzige produktive Sprache. Bestehendes Python bleibt lesbare Referenz.

## Import, Herkunft und Lizenz

C prüft die vollständige Eingabe vor produktiven Schreibzugriffen. Wiederholung derselben ID, Revision und desselben Hashs ist idempotent. Neue Revisionen bleiben erhalten. Gleiche ID und Revision bei abweichendem Inhalt wird als Konflikt erfasst und überschreibt keinen vorhandenen Wert. Bestehendes Wissen und dessen Quellen bleiben erhalten.

A/B belegen vor Import Nutzungsbedingungen, Robots/API-Regeln und Lizenz. Ungeprüfte Lizenz oder `redistribution_allowed=false` erlaubt keine Veröffentlichung. C prüft die zulässige interne Verarbeitung; rechtlich unklare Inhalte bleiben eine dokumentierte Importgrenze. Zugriffssperren respektieren. Keine Authentifizierungsumgehung, kein Kauf, keine Secrets, keine ENV-Dateien und keine Community-Nachrichten.

A/B übergeben in `bereiche/<paket>/UEBERGABE.md`: Worktree, Branch, eigener SHA, geänderte Pfade, Modulaufruf, absolute Daten- und Inventarpfade, Quellenanzahl, Revisionen/Builds, Abdeckung samt Nenner, Lizenznachweise, tatsächliche Lücken und Prüfungen. Rohdaten nicht committen.

C führt echte Importläufe mit idempotenter Wiederholung aus. Nachweise umfassen erhaltenen Altbestand, Dokumente und Fakten, Quellen-/Versionsverteilung, Konflikte und Restlücken. Das neue Wissen muss durch den bestehenden Brain-Lesepfad aus derselben Postgres-Datenhaltung erreichbar sein. Vor Abschluss gelten unabhängige Intent-Abnahme und Bug-/Security-Gate für denselben integrierten SHA. Merge, Push, Deploy, Neustart und Live-Prüfung werden getrennt belegt.

## Modell und Host

Ausschließlich GPT 6.1 Sol über `http://127.0.0.1:18768`. Aktive Startparameter von C: `gpt-6.1-sol[1m]`, Effort high. Die lokalen Aliaszuordnungen sind auf Sol geprüft. Native Worker höchstens high oder medium. Keine alternativen Starter oder ungeprüften Rückfälle.

Vor Cargo beide blockierenden Sperren aus HOSTPROBE.md in vorgegebener Reihenfolge halten, unmittelbar danach frische NonZombie-Compilerprobe und höchstens zwei Jobs. Fremde Compiler nicht beenden. Statusereignisse schreibt je Paket ausschließlich dessen Teil-Orchestrator; TODO.md und zentrales REGISTER.md bleiben unangetastet.

## Bestätigte Veröffentlichung und begrenzte Prüfzuweisung

Der Hauptorchestrator hat die erste Fassung am 2026-10-03 zentral geprüft und veröffentlicht. Nach AN_BEREICHE.md Punkt 6 schreibt C Vertrag und Koordinationsartefakte im eigenen Worktree; der Hauptorchestrator übernimmt sie zentral. Der Harness-Schutz bleibt bestehen.

Konkrete Ausnahme zur lokalen Modulprüfung: A darf ausschließlich in `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/lib.rs` temporär die Zeile `pub mod wiki_inventory;` ergänzen. B darf ausschließlich in `/home/nathanael/.worktrees/brain-wiki-spielwissen-b/rust/crates/dbrain-sources/src/lib.rs` temporär die Zeile `pub mod game_files;` ergänzen. Diese Prüfregistrierungen nicht als A/B-Änderung committen oder zur Integration übergeben. Keine produktiven Cargo-Dateien und keine weiteren bestehenden Dateien ändern. Die produktive Registrierung und die Abnahme am aktuellen Main-Stand bleiben bei C. Alle Cargo-Aufrufe auch für diese lokalen Prüfungen brauchen die beiden Hostlocks und die frische Compilerprobe.

Nach AN_BEREICHE.md Punkt 15 dürfen A und B alternativ im jeweiligen eigenen Worktree einen lokalen Rust-Prüfharness unter `.tasks/2026-10-03-wiki-spielwissen/pruefharness-a/` beziehungsweise `pruefharness-b/` erstellen, einschließlich eines eigenen neuen Harness-Cargo.toml. Eigene neue Module einbinden, bestehende Abhängigkeiten verwenden und weitere konkret an C melden. Gemeinsame produktive Manifeste, lib.rs und Lockfile bleiben bei C; keine entsprechenden Prüfänderungen zur Integration committen. Diese Harness laufen ebenfalls unter beiden Hostlocks mit höchstens zwei Jobs. Nach Punkt 16 gilt ausschließlich Sol mit Effort höchstens high auch für lokale Kritiker und den finalen Gate-Aufruf; keine Gate-Defaults mit anderem Modell oder ungeprüftem Rückfall.

Für produktiven C-Bau steht zusätzlich der saubere Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration`, Branch `feat/brain-wiki-spielwissen-c-integration`, auf Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`. Eigener stabiler Quellstand ist als ungeprüfter WIP-Commit `b1b9241805f470427570566faca37fc340d1c04c` gepusht. Keine erfolgreiche Compiler-, Import- oder Freigabeprüfung behaupten. Nur eigene A/B/C-Commits werden integriert; die fremden historischen Ausgangsbranch-Commits bleiben unangetastet. Vor finaler Integration origin/main erneut frisch prüfen.

Nach AN_BEREICHE.md Punkt 21 muss As Netzpfad während des Bodylesens begrenzt werden. Im vorhandenen aktuellen Core existiert bereits `HttpClient::get_bounded` mit `SourceHttpOptions`, `Read::take(max_bytes + 1)` und explizitem Fehler bei Überschreitung. Konkrete API und Belege stehen in `bereiche/c/HTTP_WIKI.md`. Die alte A-Basis hat diese neuere Schnittstelle nicht. C bindet bei Übernahme den tatsächlichen A-Aufruf an diesen vorhandenen begrenzten Corepfad an und prüft ihn; der bisherige `get_no_redirect`-Aufruf ist dadurch noch nicht korrigiert. A normalisiert die gesicherten Archive unabhängig davon weiter.
