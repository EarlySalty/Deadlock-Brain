# S07 · Provider und Jev: Vertragsvorbereitung

Stand: 24.09.2026. Status: **vorgeschlagen; Implementierung blockiert**.
Basis: `c00fc8935048bf490c1e4790f7c6195864ad49e2`.
Branch: `migration/07-provider-jev-20260924`.

## Entscheidung und Grenze

Der integrierte `../STATUS.md` und `../GATES.csv` lassen G0 und G1 offen.
Contract- und Schema-Version sind nicht festgelegt. Deshalb entsteht hier die früh
mögliche Vertragsprüfung aus `chats/07_PROVIDER_JEV.md`, **keine Implementierung,
keine private Ersatzschnittstelle und keine eigenständige Gate-Freigabe**. S07 ist
mit dieser Vorbereitung nicht vollständig abgenommen.

Diese Änderung betrifft ausschließlich `architecture/migration/s07/**` und
`architecture/migration/handoffs/07-provider-jev.md`. Chat 00 behält die Integration
und die globale Koordinationsdokumentation. `STATUS.md`, `GATES.csv`, Owner-Datei,
Workspace, Lockfile, Rust-Produktivcode, DB, Dienste und andere S-Pakete bleiben
unverändert. Der PR bleibt offen; weder Merge noch Deployment oder Neustart.

## Ergebnisse

| Artefakt | Aussage |
|---|---|
| [Providerinventar](PROVIDER_INVENTAR.md) | Geprüfte Wiederverwendung und konkrete Lücken am Basis-Commit |
| [HTTP-Verträge](HTTP_VERTRAEGE.md) | Aktuelle Primärquellen, Wire-Semantik und offene Annahmen |
| [Änderungsantrag an 00/02](CR-07-02-PORTS.md) | Benötigte gemeinsame Entscheidungen, keine neuen Ports im Code |
| [Fehlermatrix](FAULTMATRIX.csv) | Geplante Rust-Fake-Server- und Sicherheitstests, noch nicht ausgeführt |
| [Shadow und Aktivierung](SHADOW_UND_AKTIVIERUNG.md) | Fünf getrennte Entscheidungen; alle aus, keine erfundenen Schwellen |
| [Fixtures](fixtures/MANIFEST.json) | Ausschließlich selbst erstellte synthetische Testdaten, keine API-Mitschnitte |
| [Prüfbericht](PRUEFBERICHT.md) | Tatsächlich ausgeführte Prüfungen, vorhandene Fehler und Abnahmegrenzen |
| [Übergabe](../handoffs/07-provider-jev.md) | Voraussetzungen und nächste Besitzer |

Der Bestand wird erweitert, nicht durch neue `brain-*`-Crates ersetzt.
Die vorläufige S07-Reservierung betrifft `deadlock-brain-core/src/ai*`, `http*`
und `model_resolver*`; Implementierung erst nach G1 und bestätigter Abgrenzung
zu Chat 02. Alle vorgeschlagenen Schwellen, DTOs und Fehlerregeln in diesem
Verzeichnis sind **nicht** der freigegebene gemeinsame Contract.

## Herkunft und Nachweisgrenzen

Arbeitsauftrag: `Deadlock-Brain_Rust-Daten_Planpaket_v1.0(7).zip`, Plan 1.0.
Gelesene Kapitel: 00, 01, 02, 06, 07, 08_ERGAENZUNGEN_INTEGRIERT, 10,
Chat 07 und Übergabevorlage; zusätzlich die integrierten Koordinationsdateien.
Öffentliche Vertragsdokumentation wurde am 24.09.2026 geprüft. Es gab keine
realen Provideranfragen, keine Credential-Beschaffung, keine Nutzerdatenübermittlung
und keine Shadowmessung. JSON-/Hashprüfungen der Fixtures sind kein Test des
noch nicht implementierten Rust-Validators. Vor der Implementierung sind Gate-
und Quellendokumentationsstand erneut zu prüfen.
