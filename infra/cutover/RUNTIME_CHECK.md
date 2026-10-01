# S11: Prozess- und Bibliotheksprüfung

Status: implementiertes, nur lesendes Betriebswerkzeug. **Keine G4-/G5-/G6-Abnahme.**
Fortsetzung von PR #26 auf `5b11aa5e77fa0a513f5ea3b68b41227da6a7129e`;
integrierte Basis weiterhin `30326512568b7370524956839100462ba71bdb92`.
Contract-/Schema-Versionen bleiben offen. Geändert werden nur S11-Artefakte,
nicht Root-Workspace, gemeinsame Verträge, Datenbanken oder laufende Units.

## Aufruf

```sh
cargo run --manifest-path infra/cutover/runtime-audit/Cargo.toml --release --locked --offline -- runtime
```

`snapshot` bleibt der bestehende Metadatenmodus. `runtime` ergänzt die rekursive
Aufnahme der Prozessgruppen der tatsächlich ermittelten Services. Die CLI nimmt
keine Pfade, PIDs, Befehle oder alternativen Dateisystemwurzeln entgegen.

Der Leser erkennt über `/proc/self/mountinfo` entweder cgroup v2 am vollständigen
Mount `/sys/fs/cgroup` oder die benannte cgroup-v1-Systemd-Hierarchie am
vollständigen Mount `/sys/fs/cgroup/systemd`. Andere Mountpunkte, eingeschränkte
Namespace-Wurzeln und mehrdeutige passende Mounts werden nicht geraten.
Auf dem am 24.09.2026 geprüften Host ist tatsächlich **Systemd cgroup v1** aktiv.

Pro Service liest der Check `cgroup.procs` einschließlich Untergruppen. Er
vereinigt doppelte PIDs innerhalb derselben Gruppe und gleicht jeden Prozess
mit seiner tatsächlichen benannten bzw. vereinheitlichten Hierarchie ab.
PID und Startzeit, ausführbare Datei, Gruppenbestand und Systemd-Metadaten
werden erneut geprüft. Erkannte Änderungen machen die ganze Aufnahme ungültig.
Ein fehlender MainPID im erfassten Bestand ist ebenfalls ein Fehler.

Aus `/proc/<pid>/exe` und `/proc/<pid>/maps` werden ausschließlich feste
Befundklassen abgeleitet: Python-/PyPy-Kandidat, Shell, sonstiger ungeprüfter
Entrypoint, namentlich erkennbare eingebettete Pythonbibliothek sowie gelöschte
ausführbare Dateien oder Datei-Mappings. Ein anderer Binaryname beweist weder
Rustherkunft noch einen bestimmten Git-Commit. Es werden keine Binaries oder
Bibliotheken ausgeführt; insbesondere kein `ldd` auf einem Prüfartefakt.

## Grenzen und Datenminimierung

Keine Umgebungsvariablen, Kommandozeilen, Secretdateien, Prozessspeicherinhalte,
Anwendungsdatenbanken oder Modell-/Netzwerkendpunkte werden gelesen.
Ausführbare Dateipfade, Mappingpfade und Prozessnamen aus `stat` werden nicht
in den Bericht oder in Fehlermeldungen übernommen. Ausgegeben werden geprüfte
Unitnamen, numerische PID-/Startidentitäten und feste Befundklassen.

Die Aufnahme begrenzt Einzeltexte auf 1 MiB, den Textumfang insgesamt auf
64 MiB, Prozesse insgesamt auf 4096 sowie Gruppen pro Durchlauf auf 256 mit
maximal 32 Unterebenen. Zusätzlich besteht eine zwischen Leseoperationen geprüfte
30-Sekunden-Beobachtungsgrenze. Diese ist kein harter Timeout für einen im Kernel
blockierten einzelnen Dateizugriff. Die vorhandenen Systemd-Abfragen haben ihren
separaten Zehn-Sekunden-Timeout; dessen kompletter Ausfallraum ist nicht neu
vermessen worden.

Exit 0 bedeutet **vollständig erfasste Momentaufnahme**, auch mit Befunden.
Exit 1 bedeutet Erfassungs-/Validierungsfehler; vor abgeschlossener Erfassung
wird kein Teilbericht ausgegeben. Exit 2 bedeutet ungültige Bedienung.
Jeder Bericht behält `cutover_authorized=false`,
`full_runtime_verification=not_performed` und im Prozessmodus
`point_in_time_only=true`.

Diese Aufnahme ist nicht atomar. Sie kann kurze, zwischen zwei Messungen
beendete Unterprozesse, umbenannte/statisch eingebettete Interpreter, spätere
`dlopen`-Vorgänge oder außerhalb der Servicegruppen laufende Prozesse nicht
vollständig ausschließen. Inaktive Jobs werden ausdrücklich als ohne
Prozessbeobachtung gezählt. Sie werden nicht gestartet, um ein positives
Prüfergebnis zu erzeugen. Regelmäßige Ingest-/Learning-/Reparsezyklen, andere
Hosts, System-Units, Cron, Netzwerkabhängigkeiten und direkte Consumer-Modellpfade
bleiben eigenständige, echte S11-Abnahmen. cgroup v1 wird hier zur Prozesszuordnung
verwendet, nicht als Nachweis wirksamer CPU-/Speicherlimits aller Controller.

## Tatsächlicher Live-Befund vom 24.09.2026

Die wiederholte Aufnahme während dieser Fortsetzung erfasste zwölf Units,
sieben Services und fünf aktive Timer. Zwei laufende Serviceprozesse wurden
über die cgroup-v1-Systemd-Hierarchie geprüft; fünf Services hatten keinen
beobachtbaren laufenden Prozess.

| Befund | Beobachtung | Folge |
|---|---|---|
| Periodische Jobs | `deadlock-brain-build-data.service` und `deadlock-brain-youtube-learning.service` weiterhin `failed`, letzter Status jeweils 1 | S04/S05 müssen Ursache und nächsten echten Zyklus nachweisen; hier kein Neustart |
| Laufende ausführbare Datei | Bei `deadlock-brain-site.service` ist die ausführbare Datei als gelöscht markiert | Ein aktueller Dateipfad oder Branch ist kein Nachweis für das tatsächlich laufende Artefakt; Releaseherkunft vor späterem freigegebenem Deploy klären |
| Gelöschte Mappings | Brain-Site und `dl-knowledge.service` haben mindestens ein als gelöscht markiertes Datei-Mapping | Erneute Prüfung im freigegebenen Release-/Neustartfenster; keine Behauptung eines bereits ausgefallenen Prozesses |
| Pythonhinweise | In den zwei aufgenommenen Prozessen keine erkannten Python-/PyPy-Entrypoints oder namentlichen Pythonbibliotheken | **Keine Pythonfreiheitsabnahme** für die fünf nicht laufenden Jobs oder den gesamten Stack |
| Ressourcen | Eigene Systemd-Memory-/CPU-/Tasksgrenzen der sieben Services weiterhin unbeschränkt | Übergeordnete Controller-/Slicegrenzen und Zielbudgets nicht damit abgenommen |

Die erste Prozessaufnahme scheiterte korrekt mit Exit 1, weil die zunächst nur
für v2 implementierte Prüfung nicht zum realen v1-Host passte. Nach Ergänzung
der expliziten v1-Erkennung und Mitgliedschaftsprüfung bestand die Aufnahme.
Weder ein Hostumbau noch zusätzliche Rechte oder ein Umgehen des Fehlers waren
nötig. Rohinhalte aus procfs und private Laufzeitpfade werden nicht committed.

## Prüfgrundlage

Die verwendete Prozesszuordnung, PID-Deduplikation und procfs-Felder richten sich
nach den primären Linux-Kernelbeschreibungen:

- [cgroup v1](https://docs.kernel.org/admin-guide/cgroup-v1/cgroups.html)
- [cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html)
- [procfs](https://docs.kernel.org/filesystems/proc.html)

Diese Quellen ersetzen keine Prüfung der tatsächlich sichtbaren Hostumgebung.
Konkrete Testbefehle und Gegenproben stehen in `TEST_REPORT.md`; die noch
nicht durchgeführten Betriebsproben bleiben in `REHEARSAL.md` unverändert offen.
