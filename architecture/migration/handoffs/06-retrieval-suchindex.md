# Übergabe — S06 / Hybrid Retrieval und Suchindexentscheidung

Status: **S06 blockiert; Lese-/Entwurfsvorbereitung zur Prüfung als Draft-PR bereitgestellt**.
Arbeitsmodus: Lese-/Entwurfsvorbereitung vor G0; keine Implementierungsfreigabe.
Basis-Commit: `c00fc8935048bf490c1e4790f7c6195864ad49e2`.
Ergebnis-PR: https://github.com/EarlySalty/Deadlock-Brain/pull/18 (Draft, nicht integriert).
Dokumentations-Commit vor dieser Übergabe: `52695a770e195b4191620f3f2c478cd85fdf4cb9`.
Tatsächlich mit Rust-Tests geprüfter Commit: `52695a770e195b4191620f3f2c478cd85fdf4cb9`; zusätzlich identisches Ergebnis auf Basis `c00fc89`.
Contract-/DB-Schema-Version: offen, nicht von S06 festgelegt.
Source-Paket: 1.0; ZIP SHA256 `945be6983ff0235eb0ec36b451f9613ea567e0962ca5658364e8665980177716`.
Source-/Corpus-/Knowledge-/Modell-/Rule-/Parserrevisionen: für einen Pilot nicht freigegeben oder gefroren.
Requirements: S06-Kern R11, R17, R30, R35, R38 nur vorbereitet, nicht abgenommen. R23/R24 durch nachvollziehbare Übergabe und belegte Codebefunde unterstützt. Schnittstellenbezüge: R02, R06, R08, R14, R19, R20, R22, R28, R32, R34, R37, R40, R42, R43, R47, R51, R53, R57, R58, R60.

## Ergebnis und konkrete Änderungen

Ausschließlich folgende sechs Dokumentationsdateien:

1. `rust/crates/dbrain-retrieval/docs/s06/README.md`: Startprüfung, acht belegte Codebefunde und vorhandene Tests.
2. `rust/crates/dbrain-retrieval/docs/s06/PILOT.md`: getrennte lexical/dense-Baselines, autorisierter Ablauf, deterministische Fusionsvariante, Messverfahren, Rebuild-/Deltaanforderungen und noch offener Such-/Embedding-ADR.
3. `rust/crates/dbrain-retrieval/docs/s06/CONTRACT_REQUEST.md`: CR-S06-001 an die bestehenden Besitzer, keine private Ersatzschnittstelle.
4. `rust/crates/dbrain-retrieval/docs/s06/CASES.csv`: 30 vorgeschlagene Fälle, alle `proposed_not_run`; weder implementierte Regressionstests noch unabhängige Labels.
5. `rust/crates/dbrain-retrieval/docs/s06/pilot-manifest.json`: Planungscheckliste, ausdrücklich kein Runtimevertrag. Fehlende Freigaben/Revisionen/Hashes/Messwerte bleiben offen.
6. `architecture/migration/handoffs/06-retrieval-suchindex.md`: diese Übergabe.

Keine neuen oder entfernten Abhängigkeiten, keine geänderten öffentlichen Schnittstellen. Keine Änderungen an Root-Workspace, Lockfile, CI, Schema, gemeinsamer STATUS-/Gate-/Ownerdatei oder Produktivcode. Bestehende `dbrain-*`-Crates bleiben erhalten.

Wichtigste Befunde: Der Rust-Vektorpfad endet bei vorhandenen Tabellen in `todo!`; die vorhandene Textsuche ist heuristisch, kein belegter BM25-/FTS-Pfad. Der Altcode enthält SQLite-vec, aber weder dessen aktuelle Nutzung noch sein erfolgreicher Dense-Betrieb wurde hier belegt. Die betrachteten Einstiegssignaturen belegen keinen neuen gemeinsamen Auth-/Knowledge-Releasevertrag. Einzelheiten und genaue Fundstellen stehen in der README; dies ist kein öffentlicher Exploit- oder Live-Leak-Nachweis.

## Nachweise

| Prüfung | Befehl / Umgebung | Tatsächliches Ergebnis | Einordnung |
|---|---|---|---|
| Quellen-/Gateprüfung | `git show origin/main:architecture/migration/STATUS.md origin/main:architecture/migration/GATES.csv origin/main:architecture/migration/PFAD_OWNER.csv` auf Basis `c00fc89` | G0/G1 offen; Verträge/Schema/Release unbestimmt; S06 nach G1 | kein Implementierungsstart |
| Erster Rust-Testversuch | `cargo test --manifest-path rust/Cargo.toml --locked --offline -p dbrain-retrieval --lib -j 2` | Exit 101: System-Cargo kann Lockfile v4 nicht lesen | Fehler vor Ausführung von Tests |
| Bestehende Rust-Unittests | `$HOME/.cargo/bin/cargo +1.88.0 test --manifest-path rust/Cargo.toml --locked --offline -p dbrain-retrieval --lib -j 2` | Exit 0: **37 bestanden, 0 fehlgeschlagen, 14 ignoriert, 0 gemessen** auf `52695a7` | bestehende lokale Tests, keine Hybridintegration |
| JSON-/CSV-Struktur | lokaler Python-Standardbibliotheks-Check der fünf S06-Dokumente, siehe reproduzierbarer Befehl unten | Exit 0: JSON gültig, 30 eindeutige Fälle, alle offen, fehlende Verträge/Corpus-/Labelhashes sichtbar | keine Relevanz-/Corpusfreigabe |
| Whitespace | `git diff --cached --check` vor dem Dokumentationscommit | Exit 0 | Formatprüfung der neuen Dokumentation |
| Datenbank-/echte Wiki-Tests | nicht mit `--ignored` ausgeführt | 12 Postgres- und 2 echte Wiki-Fälle ignoriert | keine Integrationsbehauptung |
| Gesamtworkspace fmt/clippy/test/release-build; Providervertrag; Recall-/Last-/Rebuildmessung | nicht ausgeführt | offen; für diese reine Dokumentationsänderung keine Vollstack-Abnahme beansprucht | G2/G4 bleiben offen |

Die bereits installierte Rust-Toolchain 1.88.0 wurde explizit genutzt; keine Toolchaininstallation, keine Lockfileanpassung und keine MSRV-Freigabe. Testlaufzeit ist keine Retrieval-Latenz.

Reproduzierbare Strukturprüfung, aus dem Repository-Root; optionales lokales Prüfkommando ohne Betriebsabhängigkeit oder neu eingechecktes Pythonprogramm:

```sh
python3 - <<'PY'
import csv
import json
from pathlib import Path

p = Path('rust/crates/dbrain-retrieval/docs/s06')
m = json.loads((p / 'pilot-manifest.json').read_text())
with (p / 'CASES.csv').open(newline='') as f:
    rows = list(csv.DictReader(f))
assert len(rows) == len({r['case_id'] for r in rows}) == 30
assert all(None not in r and all(v for v in r.values()) for r in rows)
assert all(r['status'] == 'proposed_not_run' for r in rows)
assert m['measurements']['status'] == 'not_run'
assert m['freeze_status'].startswith('blocked_')
assert all(v is None for v in m['versions'].values())
assert m['corpus']['manifest_sha256'] is None
assert m['labels']['label_sha256'] is None
assert m['existing_code_check']['passed'] == 37
assert m['existing_code_check']['ignored'] == 14
assert {x.name for x in p.iterdir()} == {
    'README.md', 'PILOT.md', 'CONTRACT_REQUEST.md',
    'CASES.csv', 'pilot-manifest.json',
}
print('PASS: S06 planning artifacts are structurally consistent.')
PY
```

## Folgen

Datenmigration/Kompatibilität/Wiederanlauf: kein Import, Backfill, Reembedding, Indexwechsel oder Writerzugriff. Kein zweiter Releasezeiger. Bestehendes Verhalten wurde nicht geändert.

Berechtigungen/Secrets/Egress: keine Live-DB-/Providerabfrage, kein Zugriff auf Secrets und keine Übernahme produktiver Rohtexte, privater Querylogs oder Replays in den PR. Rechte-/Corpusfreigaben bleiben Blocker; selbst ein Hash oder ein interner Scope ersetzt keine Publikationsfreigabe.

Latenz/Ressourcen/Kosten: nur lokaler Offline-Testbuild mit zwei Cargo-Jobs; keine Leistungsverbesserung gemessen oder zugesagt. Kein Dienst neu gestartet oder konfiguriert.

Python-/Legacyfreiheit: kein neuer Python-Kern, Worker, Sidecar oder Rebuildpfad. Der Altcode wurde nur gelesen, nicht ausgeführt. Die vollständige produktive Pythonfreiheit ist nicht Gegenstand dieser Vorbereitung und nicht bewiesen.

## Grenzen und Blocker

- **B-S06-01, S00/S01/S10:** integriertes G0 inklusive Quellenrechten, aktiver Referenz, Lastprofil und unabhängigen Labels fehlt.
- **B-S06-02, S00/S02/S03:** integriertes G1 mit gemeinsamen Auth-/Evidence-/Retrieval-/Embedding-/Knowledge-Verträgen, Schema und ausdrücklicher Arbeitsfreigabe fehlt.
- **B-S06-03, S03/S04/S05/S12:** versionierter zugelassener Corpus, Facts, Mechaniknachbarn und kompakte Karten sind für den Pilot nicht übergeben.
- **B-S06-04, S07/S10:** feste Embeddingkonfiguration, zugelassener Egress sowie Qualitäts-/Kosten-/Lastgrenzen fehlen.
- **B-S06-05, S00/S12:** bestehende Ownerüberschneidung für `rust/crates/dbrain-retrieval/src/game_wiki*` vor Implementierung auflösen.

Weder neue Regressionstests noch ein lexical/dense-Pilot wurden ausgeführt. Die 30 Fälle sind Entwürfe. Es liegt kein echter Store-/Provider-/Kernel-Durchstich vor. Der Draft ersetzt weder die G2-Suchentscheidung noch die G4-Abnahme.

## Übergabe an nächsten Besitzer

**Next owner: S00.** Die Startbedingungen am dann aktuellen integrierten Commit prüfen; diese Übergabe darf keine Freigabe aus einem anderen Chat voraussetzen. S01/S10 liefern G0-Nachweise, S02/S03 die G1-Verträge. CR-S06-001 verteilt die übrigen benötigten Eingänge ohne Änderungen fremder Pakete.

Nach Freigabe: S06 auf den integrierten G1-Commit aktualisieren, reale Corpus-/Labelstände einfrieren, lexical und dense getrennt messen und erst danach Fusion/Jev evaluieren. G2-Entscheid vor Voll-Reembedding an S03/S04 übergeben. Neue globale CI-, Schema-, Provider- und Kerneländerungen bleiben bei deren Besitzern.

## Integration durch S00

Merge-Commit: keiner.
Gate-/STATUS-Änderung: keine durch S06.
Freigegeben von: offen.
Nächste Implementierungswelle: nicht von S06 freigegeben.
Sensible Quellen-/Replay-/Publikationsrechte: keine neuen Quellen in diesem PR; Pflichtfreigaben für spätere echte Quellen weiterhin offen.
