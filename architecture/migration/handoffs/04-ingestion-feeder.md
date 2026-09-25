# Übergabe · Chat 04 / Rust-Ingestion und Feeder

Status: vorgeschlagen; Implementierung blockiert; keine S04-Gesamtabnahme.
Arbeitsmodus: dokumentarische Vorprüfung, keine eigenständige Gatefreigabe.
Basis-Commit: `c00fc8935048bf490c1e4790f7c6195864ad49e2` (integriertes `origin/main`).
Ergebnis-Branch: `migration/s04-ingestion-preparation-20260924`.
Ergebnis / PR: https://github.com/EarlySalty/Deadlock-Brain/pull/16 (offen, Draft).
Geprüfter Dokument-Commit: `f911bbb851e5182fb0dd320b749127233502d3ed`. Der abschließende PR-Head samt erneutem Prüfergebnis steht im PR-Prüfprotokoll; diese nachträgliche Ergänzung verändert nur die Übergabe.
Tatsächlich geprüfter Code-Commit: `c00fc8935048bf490c1e4790f7c6195864ad49e2`, ausschließlich lesende Code-/Koordinationsprüfung.
Contract-/DB-Schema-Version: laut integriertem STATUS offen; keine Version erfunden.
Source-Paket: Plan 1.0; ZIP-SHA256 `945be6983ff0235eb0ec36b451f9613ea567e0962ca5658364e8665980177716`.
S04-Auftrags-SHA256: `b1278465416fa0acc9bd5f0a3142ac113459d96cf07c1fb89a6dbfeb1c7588a1`.
Corpus-/Knowledge-/Modellversion: in diesem Schritt weder verändert noch als aktiv verifiziert.

Betroffene Requirements: R03–R06, R08–R09, R13–R14, R17–R18, R20, R22–R24, R27, R38, R40, R42, R49, R58–R60 als Vorprüfung/Testanforderung, nicht als erfüllte Implementierungsnachweise.

## Ergebnis und konkrete Änderungen

Genau fünf neue Dokumentdateien; keine Abhängigkeiten und keine öffentlichen Schnittstellen geändert:

| Datei | Inhalt |
|---|---|
| `architecture/migration/s04/README.md` | Integrierte Basis, Startblocker und Codebelege mit Zeilenangaben. |
| `architecture/migration/s04/SOURCE_JOB_INVENTORY.csv` | 19 Quellen-/Job-/Infrastrukturpositionen; Codekenntnis und ungeprüfte Runtime/Rechte getrennt. |
| `architecture/migration/s04/CONTRACT_REQUESTS.md` | Vier vorgeschlagene Anträge an die zuständigen Besitzer; kein eigener Ersatzvertrag. |
| `architecture/migration/s04/PILOT_ACCEPTANCE.md` | 20 noch auszuführende Pilot-/Crash-/ACL-/Rebuild-/Last-/Feederfälle. |
| `architecture/migration/handoffs/04-ingestion-feeder.md` | Diese Übergabe und reproduzierbare Dokumentprüfung. |

Wichtige belegte Lücken: identischer Dokumenthash aktualisiert im vorhandenen SourceStore keine Metadaten; Raw-Schreiben und Runstatus belegen keine gemeinsame Crash-/Ack-Garantie; HTTP-Cache ist auf dem geprüften Pfad URL-basiert. Vorhandene Rust-Importer und der Wiki-Lock-/Staging-/Publikationspfad müssen erhalten bleiben. Details und fachliche Grenzen stehen im README; es wird kein tatsächliches Datenleck oder produktiver Ausfall behauptet.

## Tatsächliche Prüfungen und Grenzen

| Prüfung | Befehl / Umgebung | Tatsächliches Resultat |
|---|---|---|
| Integrierte Basis / Freigabe | `git fetch origin main`; `git show origin/main:architecture/migration/STATUS.md`; GATES und PFAD_OWNER gelesen | Basis `c00fc893...` bestätigt; G0/G1 und Contract-/Schema-Version offen. |
| Uploadabgleich | SHA256 des bereitgestellten und des versionierten ZIPs; zusätzlich SHA256 des S04-ZIP-Mitglieds | Beide Prüfsummen stimmen exakt überein. |
| Erste Inventar-/Referenzprüfung | Python-Standardbibliothek, nur S04-Dokumente und fixierter Git-Baum | Zunächst fehlgeschlagen: vier Referenzen auf ein auf main nicht entpacktes Planverzeichnis. Referenzen auf tatsächliches versioniertes ZIP-Mitglied korrigiert; fehlende koordinierte Paketablage als Blocker ergänzt. |
| Dokumentprüfung nach Korrektur | Reproduktionsblock unten, Commit `f911bbb851e5182fb0dd320b749127233502d3ed` | Exit 0: Diffprüfung; exakt fünf Dokumente; 19 eindeutige vollständige Inventarzeilen und gültige Belegreferenzen; beide SHA256-Abgleiche; 20 geplante Fälle; vier CRs. |
| Rust fmt / clippy / test / release build | Nicht ausgeführt | Kein Rust-Code geändert. Keine Rust-Testabnahme behauptet. |
| Storeintegration / Connector-E2E / Crash / Rebuild | Nicht ausgeführt | S02/S03-Verträge, G1 und isolierte Testumgebung fehlen. |
| Feeder-A/B / Liveverkehr / Providerverträge | Nicht ausgeführt | Kein Provider- oder produktiver Quellenzugriff; keine Messwerte vorhanden. |

### Reproduktion der Dokumentprüfung

Im Repository-Root am Ergebnis-Commit ausführen. Der einmalige Prüfaufruf benutzt ausschließlich Git und Python-Standardbibliothek; er ist kein produktiver Worker, kein Rebuildwerkzeug und keine neue Betriebsabhängigkeit.

```sh
python3 - <<'PY'
import csv
import hashlib
import re
import subprocess
from pathlib import Path
from zipfile import ZipFile

base = 'c00fc8935048bf490c1e4790f7c6195864ad49e2'
root = Path.cwd()
def git(*args):
    return subprocess.run(['git', *args], check=True, text=True,
                          capture_output=True).stdout

expected = {'architecture/migration/s04/' + name for name in (
    'README.md', 'SOURCE_JOB_INVENTORY.csv', 'CONTRACT_REQUESTS.md',
    'PILOT_ACCEPTANCE.md')}
expected.add('architecture/migration/handoffs/04-ingestion-feeder.md')
assert not git('status', '--porcelain'), 'Use the clean committed S04 tree'
git('diff', '--check', base, 'HEAD')
assert set(git('diff', '--name-only', base, 'HEAD').splitlines()) == expected
assert 'G1: offen' in git('show', base + ':architecture/migration/STATUS.md')

rows = list(csv.DictReader((root / 'architecture/migration/s04/SOURCE_JOB_INVENTORY.csv').open()))
assert len(rows) == 19 and len({row['id'] for row in rows}) == 19
for row in rows:
    assert None not in row and all(row.values())
    ref = row['evidence_ref']
    if '!' in ref:
        archive, member = ref.split('!', 1)
        with ZipFile(root / archive) as z:
            assert z.read(member)
    else:
        location, sha = ref.rsplit('@', 1)
        path, span = location.rsplit(':', 1)
        start, end = map(int, span.split('-'))
        assert sha == base
        assert 1 <= start <= end <= len(git('show', sha + ':' + path).splitlines())
    assert row['runtime_status'] in {'not_checked', 'not_applicable'}

package = root / 'Deadlock-Brain_Rust-Daten_Planpaket_v1.0.zip'
assert hashlib.sha256(package.read_bytes()).hexdigest() == '945be6983ff0235eb0ec36b451f9613ea567e0962ca5658364e8665980177716'
with ZipFile(package) as z:
    content = z.read('deadlock-brain-rust-planpaket-v1.0/chats/04_INGESTION_FEEDER.md')
    assert hashlib.sha256(content).hexdigest() == 'b1278465416fa0acc9bd5f0a3142ac113459d96cf07c1fb89a6dbfeb1c7588a1'

pilot = (root / 'architecture/migration/s04/PILOT_ACCEPTANCE.md').read_text()
assert re.findall(r'^\| (P\d+) \|', pilot, re.M) == [f'P{i:02}' for i in range(1, 21)]
requests = (root / 'architecture/migration/s04/CONTRACT_REQUESTS.md').read_text()
assert re.findall(r'^## (CR-S04-\d+):', requests, re.M) == [f'CR-S04-{i:03}' for i in range(1, 5)]
print('PASS: exact 5-document scope; 19 inventory rows and evidence refs; package hashes; 20 planned cases; 4 CRs')
print('Verified documentation commit:', git('rev-parse', 'HEAD').strip())
PY
```

Der exakte Fünf-Dateien-Vergleich schließt Änderungen an Produktcode, Manifests, Lockfile, SQL, CI, Diensten und Koordinationsregistern aus. Er bestätigt nicht die sachliche Richtigkeit beliebiger Codeaussagen; diese wurden separat am Basis-Commit gelesen. Nach einem Rebase müssen Basis und Belege ausdrücklich neu geprüft werden.

## Folgen

Datenmigration/Kompatibilität/Wiederanlauf: unverändert; kein Datenbankzugriff, kein Import, kein angefasster Writer. Wiederanlauffähigkeit nur als Anforderung beschrieben.
Berechtigungen/Secrets/Egress: keine Secrets geladen, keine privaten Dokument-/Sessioninhalte übernommen, keine Provider- oder externen Datenfeedaufrufe. Git-Lese-/Branch-/PR-Arbeit ist davon getrennt.
Latenz/Ressourcen/Kosten: keine Laufzeitänderung; kein Benchmark, keine Verbesserung behauptet.
Vorhandene Funktionen: vollständig unverändert; fremde uncommittete Arbeit im anderen Checkout nicht angefasst.
Python-/Legacyfreiheit: kein neuer Betriebsprozess. Der versionierte Build-Data-Wrapper ruft im Standard-Secrets-Pfad Python auf; aktive Installation nicht geprüft. Kein Timer entfernt oder umgestellt.

## Grenzen und Blocker

1. G0 und G1 offen; keine fixierten gemeinsamen Contract-/Schema-Versionen.
2. Pfadfreigaben überschneiden sich; S04/09/12/13/14 brauchen Entscheidung von 00.
3. Gesamtpaket im integrierten Stand nur als ZIP; entpackte gemeinsame Ablage vor Implementierung durch 00 integrieren.
4. Legacy-Docs-Feeder, GitHub-Event- und Sessionadapter nicht bis zur realen Implementierung verifiziert. Ein lesender Versuch zur weiteren Sibling-Codeprüfung wurde vom Werkzeug nicht ausgeführt; kein fehlender Nachweis wird daraus als Nichtvorhandensein interpretiert.
5. Quellenrechte, isolierte S03-Tests, Chunk-/Embeddingkonfiguration, Providerports und Lastprofil fehlen als integrierte S04-Eingaben.

## Übergabe an nächste Besitzer

00: Diese ausschließlich dokumentarische S04-Vorprüfung reviewen. G0, Paketablage, exakte Pfade und danach G1 koordinieren; STATUS/GATES nur selbst und anhand echter Nachweise ändern.

02/03: CR-S04-001 und CR-S04-002 entscheiden und gemeinsame Versionen sowie isolierten Testpfad integrieren. Keine private S04-Schnittstelle als Ersatz annehmen.

06/07/10: Eingaben und Abnahmeprofil aus CR-S04-003 liefern. 01/09/11 und Quellenbesitzer: belegte Legacy-/Rechte-/Pfadübergabe aus CR-S04-004.

04: Erst auf dem neu integrierten und ausdrücklich freigegebenen G1-Basis-Commit den einzelnen deterministischen Datei-Pilot aus `PILOT_ACCEPTANCE.md` implementieren. Danach echte Update-/Delete-/ACL-/Restart-Abnahme mit 03, erst anschließend weitere Quellen. Keine Übernahme der Implementierung anderer Pakete.

## Integration durch Chat 00

Merge-Commit: keiner.
Gate-/STATUS-Änderung: keine.
Freigegeben von: keine neue Implementierungsfreigabe.
Neue nächste Arbeitswelle: nicht durch S04 freigegeben.

## Zusätzliche Angaben v1.0

Planrevision: 1.0; Source-/Knowledge-/Rule-/Parserrevisionen nicht verändert.
Echte Integration oder Mock: weder noch; Code-Leseprüfung und Dokumentvorbereitung.
Betroffene Ownerpfade: nur fünf neue S04-Dokumente; Integration der Koordinationsartefakte durch 00.
Benötigte Folgearbeit: freigegebene S02/S03-Verträge und S04-Pilot, keine automatische Folgeausführung.
Sensible Quellen-/Replay-/Publikationsrechte: offen; keine Scopeerweiterung und keine Publikation von Quelleninhalten.
