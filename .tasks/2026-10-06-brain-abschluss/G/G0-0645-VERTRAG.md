# G0-06:45: Erweiterter Werkzeugvertrag

Stand: 07.10.2026. Vorhandenen geprüften Vertrag aus `b4f4b866` fortgeführt. Worker `w5dr3fr2i`, Run `wf_1cdc6a56-2a1`, abgeschlossen. Dieser Bericht wird aus der tatsächlichen Rückgabe durch die Bereichsführung abgelegt; der Worker durfte nach seiner Rollenvorgabe ausschließlich als Rückgabe berichten. Kein Git- oder Runtimeeingriff durch den Worker. Paketgate steht noch aus.

## Geänderte Produktdateien

- `rust/crates/brain-contracts/src/tools.rs`
- `rust/crates/brain-contracts/tests/tool_extensions.rs`

`lib.rs`, `provider_input.rs`, bestehende Portsignaturen, Analytics-Referenzen, Original-API-Pin, Manifeste und historischer `G/G0-VERTRAG.md` blieben unverändert. Exporte der zusätzlichen Typen liegen unter `brain_contracts::tools`.

## Konkrete Erweiterungen

Achter Name: `ToolName::GameRules`, Drahtname `game_rules`. Unteranfrage: `ToolSubrequest::GameRules(GameRulesRequest)`.

```rust
pub struct GameRulesRequest {
    pub topic: ToolGameRuleTopic,
    pub entity: Option<ToolEntityRef>,
    pub game_time_seconds: Option<f64>,
    pub scenario: Option<ToolScenario>,
}

pub struct ToolBoonRange {
    pub min_boons: u32,
    pub max_boons: u32,
}

pub struct ToolAnalyticsSelection {
    pub min_average_badge: Option<u32>,
    pub max_average_badge: Option<u32>,
    pub min_unix_timestamp: i64,
    pub max_unix_timestamp: i64,
}
```

Geschlossene Regelthemen: `KillBounty`, `Comeback`, `Urn`, `Midboss`, `ResistStacking`, `Resources`; Drahtnamen `kill_bounty`, `comeback`, `urn`, `midboss`, `resist_stacking`, `resources`. Ein Aufruf wählt ein Thema, mehrere Themen benötigen mehrere Aufrufe. Keine frei ausführbaren Parameterobjekte oder Formeln.

Zusätzliche optionale Felder:

- `EntityProfileRequest.analytics: Option<ToolAnalyticsSelection>`
- `HeroCompareRequest.analytics: Option<ToolAnalyticsSelection>`
- `HeroCompareRequest.boon_range: Option<ToolBoonRange>`

Bei `None` fehlen die neuen Felder im serialisierten Argumentobjekt. Alte sieben Unteranfragen, Textports und sichere Defaults bleiben erhalten. Alle neuen Argumente werden in konkreter Unteranfrage, Abhängigkeiten, beiden Wireformen und Eingabezählung erhalten.

Spielzeit ist in Sekunden, endlich und nichtnegativ. Gültige Nullwerte und negative Resistwerte bleiben erhalten. Der Boonbereich ist beidseitig einschließlich; Strukturprüfung `min_boons <= max_boons`, keine erfundene Spielobergrenze. Die gültige Domäne bestimmt der gepinnte Fachport.

## API-Anschluss und Grenzen

Nachgeprüfte Referenz: `rust/crates/dbrain-sources/tests/fixtures/external/openapi-20260925.json`, SHA-256 `341bc2b2681d0bc353df721974bdbf46290c1f0645eba594d18381c3a2cbbc70`. Für `hero-stats` und `item-stats` entsprechen die Analytics-Feldnamen den API-Parametern. Badge-Grenzen `0..=116` sind aus diesem Originalschema belegt. Sie filtern den durchschnittlichen Badge-Wert beider Teams, nicht den individuellen Rang einer Person. Zeitwerte sind Unix-Sekunden als `i64`, mit nichtnegativem Beginn und späterem Ende. Gegen Es aktualisierten Pin ist dies bei Integration nochmals abzugleichen.

Der unveränderte `dbrain-sources::analytics_runtime` bietet noch keinen Rangfilter und keinen Item-Meta-Endpoint. Seine vorhandene 31-Tage-Grenze, Stundenrasterung, tatsächliche Fensterbindung und Herkunft bleiben Fachportaufgabe. Patchmitgliedschaft bleibt `Unverified`. Der Vertrag bestätigt keine echte Meta-Rangantwort oder Spielregel.

Aktuelle Clientversion und Mechanikrevision bleiben serverseitig. Modellfelder dafür, freie Patchmitgliedschaft, Formeln und Parameterobjekte werden abgewiesen. Das Regeltool benötigt dieselbe gebundene Spielversion und Abhängigkeitsprüfung wie andere Spielwerkzeuge.

## Prüfbelege

Verzeichnis `G/pruefungen/g0-0645/`. Vollständige Befehle in `commands.log`, Quellen in `sources-after.sha256`, archivierte Belege in `evidence.sha256`. Sämtliche Cargoläufe verwenden vorhandenen Buildslot, Debugtarget `/tmp/brain-g0-0645-target`, `--locked --offline --jobs 2`; Tests zusätzlich `--include-ignored --test-threads=1`.

| Prüfung | Beobachteter Exit |
| --- | --- |
| Formatprüfung eigener Dateien | 0 |
| Compiler, `brain-contracts`, alle Targets | 0 |
| Strict Clippy, alle Targets, `-D warnings` | 0 |
| Cratesuite | 0 |
| Provider/Kernel/Retrieval/Serve, Verbraucherkompilierung, alle Targets | 0 |

Bereichsführung las Befehle, Testlog und die fünf Abschluss-Exitdateien. Tatsächlich 33 Unitfälle, 23 bestehende Integrationsfälle und 11 neue Integrationsfälle bestanden: 67 passed, 0 failed, 0 ignored, 0 filtered. Doc-Tests haben 0 Fälle und zählen nicht mit. Historische Referenz 55 passed ist kein erneut ausgeführter Baselinelauf. Die Verbraucherkompilierung belegt ihren damaligen WIP, nicht die nachfolgenden laufenden G-P-/G-K-Änderungen. Diese Prüfungen beweisen Vertrag und Darstellung, keine echte Luna-Runde oder Produktionsantwort.

Bereichsführung bestätigte alle zehn Einträge von `sources-after.sha256` per `sha256sum --check`, einschließlich unveränderter Referenzen. Geänderte Quelldatei: `527c6b2f52275d0a717b2cd7c2646ff4f3c26d481865025c1dd42e54176e883d`. Neue Tests: `45a53d2099d4590e325f07be5edac381fbcfa3bf2a064f7f565735c6016dd053`.

## Folgeanschluss

G-P transportiert neue Argumente unverändert über die vorhandenen Wireformen. G-K trägt den neuen Namen durch dieselben Ausführungs-, Budget- und Abhängigkeitsprüfungen; sein serverseitiger Anfrage-Pin kommt vor Cache-/Flight-Schlüsselbildung aus dem Fachadapter. G-V verbindet echte gepinnte Regeln, gültige Boondomäne und API-Aggregatfilter. Keine stillschweigende Filterverwerfung, kein statischer Startup-Pin und keine behauptete Patchzuordnung.

TESTNACHWEIS[TW-1]: 67 passed, 0 ignored | Baseline: 0 rot
