status: erledigt
Datum: 2026-10-03
Abnahme übernommen: 2026-10-03T04:50:37Z

# Unabhängige native Ergebnis- und Sicherheitsabnahme D

Abgeschlossener frischer nativer Worker a8647a2884592df3e, geerbtes gpt-6.1-sol, keine Delegation. Der Worker lieferte seine unabhängige Abnahme direkt zurück. Keine eigenen Dateien, POSTs, Anmeldung, Downloads, Neustarts oder Git-Aktionen.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft

## Feststellungen

1. Der Worker fragte ausschließlich Task 4931460 einmal lesend ab. Tatsächlich erneut gemessen: DONE, result.ok=true, app_id=1422450, response.eresult=1, granted_appids=[1422450], granted_packageids=[]. Das gespeicherte Anfrageprotokoll nennt genau App 1422450 und Botkonto 1. Der geprüfte Handler sendet appids: vec![app_id]. Die erfolgreiche kostenlose App-Lizenzanforderung ist damit unabhängig bestätigt. Die leere Paketliste belegt weder fehlende Paketrechte noch verweigerten Depotzugriff.
2. Die anschließende Health-Abfrage meldete version=0.1.0, steam_connected=true und gc_connected=true. Diese Felder belegen vorhandene Verbindungshandles, keine Depotberechtigung und keinen Produktionscommit. Es wurde keine neue Anmeldung gestartet.
3. Die vollständig geprüfte default_registry, der Core-Router und der Betriebsbefehlsverbraucher bieten im geprüften Bestand keinen nutzbaren Appinfo-, Depotberechtigungs- oder Spieldepotdownloadweg über die vorhandene Anmeldung. Unbekannte Tasktypen werden abgewiesen. Generische Nachrichtenmethoden der Steam-Bibliothek sind kein fertig verdrahteter Betriebsweg. SteamCMD und Appinfo-/Paketcachedateien existieren; ihre Existenz belegt keine sicher wiederverwendbare Anmeldung. Cacheinhalte wurden nicht gelesen. Die begrenzte Nachprüfung fand keine passenden Appmanifeste oder VPK-Dateien, zwei steam-core und einen steam-bot und keine Steam-/Depot-benannten Unix-Sockets.

Die drei Befunde sind die bestätigten Aussagen beziehungsweise offenen Wirkungsgrenzen der Abnahme, keine zusätzlich entdeckten Codefehler.

## Urteil und Stopgrund

Appgrant für 1422450 bestätigt. Depotberechtigungen und vollständiger Download ungeprüft. Ein konkret nutzbarer sicherer Downloadweg ohne neue Anmeldung ist im geprüften Bestand nicht belegt. Das ist keine Steam-Ablehnung. Für den vollständigen Download fehlen Depot-, Manifest-, Vollständigkeits- und Spieldatei-Hashnachweise. Der Worker kann keinen vorhandenen sicheren nächsten Downloadaufruf benennen.

Für einen neuen Rust-Servicepfad ist die konkrete Eigentumszuweisung aus AN_HAUPT.md durch /root nötig. Das vollständige Nutzerziel ist nicht abgeschlossen; die lokalen Zugangs- und Downloadaussagen sind unabhängig abgenommen.

## Gesicherte Quellen

| Quelle | SHA-256 |
| --- | --- |
| /home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/d/license-task-4931460.json | bd282da08eeb9ed6cf6fc78e6c3ed7e16ad658c7f76b60f73a595e1caa6be377 |
| /home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/task/handlers/free_license.rs | 0aca7c1b1d8b32440a59d5714ad3cc2c6d65bbca3ae531586a899b84145ff1b3 |
| /home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/task/mod.rs | 63f21300094c488cf56ab4c8ff2ac2be77dbfa101befca07d3a3b10568a8e3d3 |
| /home/nathanael/repos/Deadlock-Steam-Bot/rust/crates/steam-core/src/api/mod.rs | 2619c47435924ca32bef080e16ad5739309a22cb29ce55e625cd0aa9f6df1361 |

D-Artefakte und B-Zugangsberichte wurden berücksichtigt. Kein Klartextgeheimnis oder Konto-/Cacheinhalt ausgegeben. Keine Aussage über sämtliche Datenträger oder unverfügbare Werkzeuge abgeleitet.
