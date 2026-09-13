# Unabhängige Gesamt-Abnahme Build-Reasoner

Stand: 13.09.2026. **Technische Freigabe für den Reparaturstand `9bcc588`; fachliche Gesamtabnahme ausdrücklich NEIN.**

Geprüft: `/home/nathanael/.worktrees/deadlock-brain-abschluss`, gemeinsamer Diff gegen `eae2330`, abschließende Quellenkorrektur `6fa027f` und Ask-Kompaktierung `9bcc588`. Autor: `brain_abschluss` mit Ask-Worker; unabhängiger Kritiker: `abschluss_kritik`. Graphify zuerst im Hauptrepo abgefragt, danach Quellen und tatsächliche Verbraucher gelesen. Keine Secrets gelesen und keine produktiven Schreibaktionen durch den Kritiker.

## Urteil

- **Technische Implementation fertig: JA, bezogen auf diesen Reparatur- und Integrationsstand.** K1–K5 sind behoben, im letzten gemeinsamen Restdiff kein weiterer blockierender technischer Befund. Der Stand darf nach dem separaten regulären Bug-/Security-Gate ausgerollt werden.
- **Ursprünglicher Nutzer-Intent erfüllt: NEIN.** Der Warden-Build trifft nur zwei der neun Referenzwaffen; die geforderte Mehrheit wird nicht erreicht. Eigene belastbare Meta-Builds sind damit nicht als fertig nachgewiesen. Der ursprüngliche Auftrag und Intentthread bleiben fachlich offen.
- **Konkrete weitere Fixes nötig: JA für die fachliche Zielerreichung; NEIN als zusätzliche technische Voraussetzung dieser Reparaturfreigabe.** Keine weitere Kalibrierung oder kosmetische Runde in dieser Abnahme verlangt. Die spätere fachliche Arbeit muss die fehlenden Referenzmechaniken und ihre gemeinsame Komposition belegen, ohne Seed-Kopie oder nachträgliches Absenken des Qualitätsmaßstabs.

Diese technische Freigabe ersetzt weder das reguläre Merge-Gate noch Merge, Deploy, Neustart und Prüfung des tatsächlich ausgerollten Binaries. Sie bescheinigt keinen erfolgten Steam-Upload.

## Verbindlicher Maßstab

`AUFTRAG.md`, `PAKETE.md` einschließlich Entscheidung 6 und `ABSCHLUSSPLAN.md`: eigene mechanisch begründete Kaufkurven, aktuelle Helden-/Itemdaten, Wardens Spirit-Feuerrate, Kaufboni, tatsächlich verwendete Kombinationen, ehrlicher Referenzvergleich sowie Ask und bestehender Publish-Weg. Ein erfundener Nightshift-Patch oder eine unbelegte historische Meta-Prognose sind ausdrücklich kein Ersatznachweis. Die Mehrheit des Warden-Waffen-Kerns bleibt fachlicher Maßstab: mindestens fünf von neun Referenzwaffen.

## Gemeinsame Befunde und bestätigte Korrekturen

| ID | Gemeinsamer Befund | Bestätigter Endstand |
|---|---|---|
| K1 | Composer wählte erste Schadensachse statt des berechneten Imbue-Wirkungszuwachses. | `composer.rs:457` verwendet `mechanics::imbue_target`; dort werden IDs ≤ 0 zentral ausgeschlossen. Test prüft stärkere zweite Fähigkeit sowie ungültige ID 0 im tatsächlichen Verbraucher. |
| K2 | Why-Text behauptete Kaufphase nach Skalierungsstufe, obwohl der neue Rechner ausschließlich Kostenbänder nutzt. | Text und Rechner stimmen überein. Itembegründungen nennen echte Itemwerte, Bedingungen und Kaufbonus; technische Rechenfeldnamen werden nicht als Nutzererklärung ausgegeben. |
| K3 | Historische Autoren-Versionen konnten über JSON-Sortierung die aktuelle Skill-Reihenfolge verdrängen. | Der gemeinsame Loader wählt zuerst die aktuellste Version je `hero_build_id`. Historische Versionen gelangen nicht mehr als gleichwertige Skillquelle in den Composer. |
| K4 | Neue Verkaufslogik berechnete Paarbonus noch mit bereits verkauften Items. | Auswahl verwendet aktuellen Bestand und berücksichtigt den erforderlichen Verkauf vor dem nächsten Kauf. Der Quellenaufbau lässt dann verkaufte Partner weg. Ein gemeinsamer Test prüft Auswahl, Erklärung und Publish-Annotation. |
| K5 | S3 speichert bewusst den breiten GC-Katalog; der Leser behandelte diesen ungekürzt als Top-Autoren-Lehrer und Referenz. | `data.rs::load_author_source_rows` wählt die aktuelle Buildversion und verbindet anschließend mit aktiven `watched_build_authors`. Referenzen, Layout, Skillquellen, Kernrolle und Verkaufsbelege nutzen denselben Loader. Fremde und inaktive Autoren bleiben als Lehrer ausgeschlossen; der breite Katalog wird nicht gelöscht. Der Seed beeinflusst die Auswahl nicht mehr und bleibt separater Vergleich. |

Der Quellen-Regressionstest deckt aktive, inaktive und unbekannte Autoren, neue Versionen sowie einen Build ab, dessen neueste Version einem fremden Autor gehört. Es wird keine alte beobachtete Version ersatzweise wiederbelebt.

## Endgültige Echtdaten

Quelle: `nachweise/WATCHED-FINAL-LIVE.json`, Reasoner-Code `6fa027f`; SHA256 selbst geprüft: `ae77fddcb9c40179f85a603ba94680fda6ee3e8377d05db54a6cf037fabba1f0`. `9bcc588` verändert anschließend nur die Ask-Projektion und deren Prüfhilfe.

- **38/38 Helden liefern ein Build.** 32 Helden haben aktive beobachtete Autorenreferenzen.
- Für McGinnis, Holliday, Mo & Krill, Viscous, Venator und Silver fehlen solche Referenzen. Ihre Ausgabe benennt den Behelf aus 64 beobachteten Builds anderer Helden ausdrücklich und setzt die Build-Sicherheit auf Low. Aus erfolgreichem JSON wird keine validierte Heldenqualität abgeleitet.
- **Warden: 16 Käufe, zwei beobachtete Buildquellen, 3/19 Gesamtreferenzitems.** Recall 0,157895; Jaccard 0,093750. Gegen den getrennten Seed ebenfalls 3/19, bei anderer Referenzzusammensetzung.
- **Warden-Waffen: 2/9**, Titanic Magazine und Frenzy. Das erfüllt den Mehrheitsmaßstab nicht.
- Warden enthält sieben ausgewählte Paarbelege. Für 92 Items wird ein positiver Spirit-Feuerratenbeitrag ausgewiesen. Diese Zahlen belegen aktive Berechnungspfade, nicht die fachliche Güte des gesamten Builds.

Vor K5 waren bei Warden 67 Katalogbuilds von 64 unterschiedlichen Autoren enthalten, davon nur zwei beobachtete Builds. Die Zahlen 42 beziehungsweise 67 aus älteren Berichten bezeichneten Build-/Vergleichszeilen und dürfen nicht als eindeutige Autorenzahl ausgegeben werden.

### Eingefrorener Mechanikvergleich bleibt unverändert sichtbar

`nachweise/FINAL-MECHANIK-VERGLEICH.json` gehört zum früheren Code `be107d7` und dessen damaligem Layoutkorpus. Identischer Helden-/Item-/Referenzstand, gleiche skalare Meta-Signale, Paarstatistik auf beiden Seiten deaktiviert. Dieser Vergleich wurde nach K5 nicht als kausaler Vorher-/Nachhervergleich neu etikettiert.

| Stand | Referenzwaffen getroffen | Gesamtkern getroffen | Recall | Jaccard |
|---|---:|---:|---:|---:|
| Vor Mechanikfix | 2/9 | 3/19 | 0,157895 | 0,107143 |
| Nach Mechanikfix | 1/9 | 2/19 | 0,105263 | 0,068966 |

Korrekte Einzelmechanik bedeutet hier noch keine verbesserte Gesamtkomposition. Der spätere vollständige Lauf mit korrigiertem Quellenkorpus steht separat daneben und verdeckt den negativen Mechanikvergleich nicht.

## Ask und Publish

`nachweise/ask-publish-smoke-kompakt.json`, finaler Ask-Code `9bcc588`, wurde gelesen:

- `read_only=on`, Route `build_reasoner`, **vollständiges BuildObject identisch zum direkten Reasoner**.
- 16 Kernkäufe, 16 Fähigkeitsschritte, drei gültige Imbue-Ziele, fünf Verkaufsprioritäten.
- Prompt von 125.466 auf **14.030 Zeichen** verkleinert. Die Projektion bewahrt Item-IDs, Reihenfolge, Warum, Bedingungen, Imbue, Verkauf und Skillfolge; gemeinsame Unsicherheitsgrenzen stehen einmal zentral. Das strukturierte `build_context` bleibt vollständig. Eine erfragte besondere Spielweise wird weiterhin nicht als berechnet ausgegeben.
- Publish überträgt Annotationen, Reihenfolge, Imbue, Verkaufsprioritäten und Kategorieabmessungen. Abgleich mit `steam-core/src/task/handlers/builds/publish_original.rs` und lesender Payload-Smoke sind erfolgreich.
- **`upload_performed=false`: Es fand kein echter GC-Upload statt.** Auch die sichtbare Darstellung im Spiel wurde durch diesen Prüflauf nicht nachgewiesen.

## Prüfung und verbleibende Grenzen

Gelesene Logs nach K5: `/tmp/brain-watch-workspace-final.log` mit **303 bestanden, 0 fehlgeschlagen, 58 bestehend ignoriert**; `/tmp/brain-watch-clippy-final.log` mit erfolgreichem `cargo clippy --workspace --all-targets -- -D warnings`. Scoped Formatter laut Abschlussmeldung erfolgreich, `git diff --check` selbst ohne Befund. Für den abschließenden Ask-Diff wurde der gezielte Regressionstest erfolgreich gemeldet; Code und echter Smoke wurden unabhängig geprüft. Keine Wiederholung ganzer Suites ohne neue Ursache verlangt.

Der Mechanikfix trennt konkrete Bedingungsflags von Basiswerten, schließt Schwellen-/Metadaten und unbelegte Parade-/Killheilung als garantierte Dauerwirkung aus, rechnet Magazingröße und Feuerrate über denselben Nachladezyklus und erhält Shopboni. Das ist eine Reparatur des Bewertungsmodells, keine vollständige Simulation sämtlicher Fähigkeiten und Iteminteraktionen.

Autorenevidenz kann globale Situationsnamen überstimmen; belegte Verkaufsprioritäten geben Plätze frei. Eine vollständige Upgrade-/Inventarsimulation wird nicht bescheinigt. Paarstatistik ist ein beobachtetes Nebensignal, kein kausaler Mechanikbeweis. Aktuelle Autoren dienen zugleich als Eingangssignal und Vergleich; die Übereinstimmung ist keine unabhängige Meta-Prognose. Historische Patchwechsel bleiben ohne passende Vorher-/Nachherdaten nicht messbar; ein Nightshift-Patch wird nicht erfunden.

Der zusätzliche Read-only-Pool verwendet Infisical-FD und geschützten lokalen Socket ohne neue Secret-ENV. Er ist ausdrücklich keine vollständige Migration des produktiven Secretzugangs. In den geprüften neuen Pfaden kein weiterer blockierender Security-Befund; reguläres gemeinsames Bug-/Security-Merge-Gate separat.
