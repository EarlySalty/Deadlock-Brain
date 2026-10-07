# Betrieb vor gemeinsamer Auslieferung

## Aktualisierung 07.10.2026, etwa 00:40 CEST

Read-only erneut geprüft: Serve PID 2554801 läuft seit 00:14:07 aus Main `10ebbb208aaaac9ddf6e07bc89a6af4a620f86ef`. Beide Releasezeiger tragen diesen SHA. Serve-, Maintain- und Migrate-Binaryhash stimmen mit dem installierten Manifestformat 2 überein. Paket A hat diesen Deploy nicht ausgeführt. Installierter Migrator: Kernschema v2/brain.store.v2 und Spielprofilschema lesend kompatibel. Drei gespeicherte Leseheaderspalten, beide unveränderlichen Headerfunktionen und nötige Profil-/Quittungsgrants vorhanden. Keine Migration von A angewandt.

A startete den regulären Wartungsdienst auf diesem geprüften Release. Start 00:26:39, Beendigung 00:28:58 durch Signal 15; systemctl meldete den Startauftrag als abgebrochen, Exit 1. Kein abgeschlossener Tickbeweis. Timeout ist zwei Stunden, keine cgroup-Speichergrenze, OOMScoreAdjust 200. Die konkrete Ursache des TERM ist noch unbelegt. Der unprivilegierte Blick auf earlyoom enthält keine zugänglichen Einträge und erlaubt keine Aussage über Systemereignisse. Timer weiterhin inaktiv. Kein automatischer Neustart in einer Wiederholungsschleife.

Produktive Metadaten: 378 Profilentitäten, 64342 Profilfakten, davon 13295 GameTracking und 51047 Deadlock-Data, null Ableitungsquittungen. Neueste Wartungscheckpointfehler `JEV_RESPONSE_INVALID` und einmal `STAGE_FAILED` bei Docs-Source-Review. Der vorhandene Git-Materialisierungsfix ist nun live, ein vollständiger Steckbrief-/Antwortbeleg fehlt weiterhin. Keine manuellen DB-Korrekturen.

## Historischer Ausgangsstand

Read-only Snapshot vom 06.10.2026. Kein Deploy, Restart, Timerwechsel oder Datenbankschreibzugriff durch Paket A in diesem Schritt. Die Quelle des laufenden Binarys muss beim tatsächlichen Deploy erneut geprüft werden.

| Dienst | Tatsächlicher Stand | Vertrag für Abschluss |
|---|---|---|
| `brain-serve.service` | PID 1327506, active/running seit 20:37:33 CEST; bestehender Drop-in `90-maintenance.conf` | Neues geprüftes Main-Release, PID/Exe/Hash/Journal und normale Antworten, nicht nur Health |
| `brain-maintenance.service` | failed, PID 0; 22:07:29 beendet, Signal 15 | Stopursache belegen; Profilfix und gemeinsamen Release vor regulärer Wiederaufnahme |
| `brain-maintenance.timer` | inactive/dead, Unit vorhanden und enabled | Nach korrigiertem Produktivpfad regulärer Tick und tatsächlicher Datenfortschritt, kein ungeprüfter Legacy-Cutover |
| `deadlock-brain-sheet-sync.service` | Letzter Lauf 19:25:31 bis 19:26:16, Exit 0; Infisical-/Credential-/OnFailure-Drop-ins | F4 repariert Batch-Exit; Laufweg am geprüften Release und bestehender Secretlader, kein schmutziger Kanon |
| `deadlock-brain-patchnotes-sync.service` | Letzter regulärer Lauf 23:14:17 bis 23:14:18, Exit 0; Legacy-Weg noch nötig | Aktuelle zentrale Patch-ID tatsächlich über Rust fortschreiben; erst dann Altweg aus |
| `deadlock-brain-site.service` | PID 2002603, active/running seit 30.09.2026 19:14:30; bestehender Pythonserver | F3b portiert bestehenden Vertrag; PostgreSQL-Migration vor Code, gleiche öffentliche URL/Funktionen |

Alle FragmentPaths liegen unter `/home/nathanael/.config/systemd/user/`. `current` verweist weiterhin auf `/opt/deadlock-brain/releases/e56e075d486a75f83f4954b58d8113588082d3f1`, `maintenance-current` auf denselben SHA im Maintenance-Layout. Diese Zeigerbeobachtung ersetzt keinen Binaryhash-/Commitbeleg.

## Unveränderter Deployweg

`ops/brain-release/README.md` im aktuellen Integrationsworktree nachgelesen. Vorhandener root-eigener Helfer `/usr/local/libexec/brain-release`: `plan`, `build`, `verify`, `install` aus neuem vollständig sauberem eigenen Remote-main-Worktree. Sourcefingerprint, vollständiges Binaryinventar, Hashes, gemeinsame Sperre und beide Releasezeiger werden durch diesen Weg überprüft. Der Helfer ändert keine Units/DB/Configs und startet keine Dienste. Keine Umgehung bei Quell-, Gate- oder Herkunftsfehlern. Migrations-/Dienstaktivierung gesondert über vorhandenen Betriebsweg abnehmen, wertvolle lokale Artefakte und alte Releases erhalten.

Gate-Werkzeug ebenfalls erneut belegt: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --help`, Exit 0. Reguläre konfigurierte Kette ohne Modelloverride. Ein falscher PATH-Aufruf ist kein inhaltlicher BLOCK und kein ALLOW.
