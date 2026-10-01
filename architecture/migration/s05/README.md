# S05 – Domain-Vorprüfung und erste Differentialprobe

**Status: Implementierung blockiert; nur nichtproduktive Preflight-Artefakte.**
Dies ist weder ein vollständiger Domain-Port noch eine G1-/G3-Freigabe.

## Basis, Grenzen und Zuständigkeit

Plan: `Deadlock-Brain_Rust-Daten_Planpaket_v1.0(5).zip`, Version 1.0 vom 24.09.2026, insbesondere Chat 05 sowie Kapitel 02, 06, 10, 11 und 13.
Integrierte Codebasis: `c00fc8935048bf490c1e4790f7c6195864ad49e2`.
`STATUS.md` und `GATES.csv` dieser Basis führen G0 und G1 als offen; Contract-, DB-Schema- und Knowledge-Version sind nicht festgelegt. Auch eine reguläre S05-Vorbereitungsfreigabe nach G0 ist dort nicht erteilt. Diese Vorprüfung ändert das nicht.

Das S01-Inventar aus Branch `feat/brain-s01-inventory-20260924` wurde als vorläufiger Hinweis gelesen. Es bezieht sich auf eine andere Codebasis (`2734c2d`) und ist kein integrierter G0-Beleg für diese Arbeit. Die hier genannten Rust-Pfade wurden zusätzlich im Baum der eigenen integrierten Basis gefunden.

Änderungsumfang ausschließlich `architecture/migration/s05/**`: Testartefakte, Matrix, Übergabe und Änderungsanfrage an die Besitzer. Keine Änderung an gemeinsamem STATUS, Gates, Ownerregister, Cargo-Workspace, Lockfile, produktiven Crates, Schema, Daten oder Diensten. Die neuen Dokumentationspfade bleiben ein Integrationsvorschlag an 00, keine selbst erteilte Pfadfreigabe. Kein Merge oder Deploy.

S05 übernimmt nach Freigabe die bestehenden `dbrain-normalize`, `dbrain-builds`, `dbrain-reasoner`, `dbrain-learn`, `dbrain-population` und `dbrain-enrich`-Fachbereiche; keine zweite `brain-domain`-Engine wird angelegt. Die Überschneidung von Population mit 14 muss 00 vor Implementierung auflösen.

## Erster geprüfter Fachbaustein: Aliasnormalisierung

Verglichen wird die vorhandene Python-Funktion `src/deadlock_brain/entity_normalizer.py::normalize_alias` mit dem unverändert extrahierten Körper von `rust/crates/dbrain-normalize/src/util.rs::normalize_alias`. Letztere wird bereits durch `dbrain-normalize/src/lib.rs` öffentlich reexportiert. Die Extraktion verweigert fehlende oder doppelte Signaturen und fehlende Abschlussmarkierungen. Sie ist nur für diesen bekannten, einfachen Funktionskörper geeignet und kein allgemeiner Rust-Parser.

Die Python-Referenzen wurden einmalig ausschließlich aus der FunctionDef des vorhandenen Quellcodes ausgeführt, ohne das Modul mit seinen Store-/Config-Imports zu laden. Sie sind als 18 synthetische Goldens in `probes/alias_cases.rs` eingefroren. Rust führt die reguläre Probe ohne Python, Cargo, Datenbank oder Netzwerk aus. Die Quelle wird über `include_str!` aus dem aktuellen Checkout gelesen; die Probe testet keine handgeschriebene Ersatzimplementierung.

Toleranz für diese Funktion: exakte UTF-8-Zeichenfolge, keine numerische Toleranz. Kein Zufallsseed, kein Patch-/Datensatz-/Zeitinput, keine beabsichtigten Seiteneffekte. Die Funktion allein belegt keine Entity-ID-, Store-, Rename- oder Rebuildparität.

| Fallgruppe | Python-Referenz | Vorhandenes Rust | Bewertung |
|---|---|---|---|
| 11 Grundfälle einschließlich normaler Hero-Aliase, NBSP, Umlauten und punktiertem I | identisch | identisch | exakte Fallparität |
| `ß`, `Straße`, `Σςσ`, `ﬀ` | Unicode-Casefolding | Kleinschreibung | vier nicht genehmigte Abweichungen |
| `__MO__`, `___` | Rand-Leerzeichen bleiben nach Unterstrichersatz erhalten | Rand-Leerzeichen entfernt | zwei Abweichungen; Altverhalten nicht idempotent, nicht blind übernehmen |
| `A` + U+001C + `B` | `a b` | Steuerzeichen bleibt erhalten | eine Abweichung; gemeinsame Whitespace-Regel fehlt |

**Ergebnis: 11/18 identisch, sieben offene Unterschiede.** Das ist keine quantitative Aussage über reale Nutzereingaben; die bewusst gewählten Grenzfälle sind keine repräsentative Stichprobe.

Zusätzlich werden alle Zeichenfolgen der Länge 0–4 über acht festgelegten Zeichen geprüft: 4.681 Fälle sowie drei längere/ergänzende Fälle, insgesamt 4.684. Geprüft werden im kleinen Korpus Idempotenz, Determinismus, entfernte Unterstriche, keine Rand-Leerzeichen und keine doppelten ASCII-Leerzeichen; die drei Zusatzfälle prüfen Idempotenz. Das ist eine begrenzte erschöpfende Eigenschaftsprüfung, kein Beweis für alle Unicode-Eingaben.

## Reproduzieren

Ab Repositorywurzel; für jeden Probelauf einen noch nicht existierenden Ausgabeordner wählen. Vorhandene Ordner werden absichtlich nicht überschrieben.

```sh
rustc --edition=2021 -Dwarnings architecture/migration/s05/probes/run.rs -o /tmp/s05-probe
rustc --edition=2021 -Dwarnings --test architecture/migration/s05/probes/run.rs -o /tmp/s05-probe-tests
/tmp/s05-probe-tests --nocapture
/tmp/s05-probe /tmp/s05-characterization-fresh --characterize
/tmp/s05-probe /tmp/s05-strict-fresh --strict
```

`--characterize`: Exit 0 bedeutet ausschließlich, dass die eingefrorene Rust-Baseline einschließlich bekannter Unterschiede reproduziert wurde und die Eigenschaften bestanden. Es bedeutet **nicht** Fachparität.

`--strict`: Im geprüften Stand Exit 1 wegen sieben ungeklärten Unterschieden. Dieser Fehler ist ein echter offener Paritätsbefund, nicht eine bestandene Abnahme. Fehler in Aufruf, Extraktion oder Kompilierung führen zu Exit 2. Kein `|| true` zur Verdeckung verwenden.

Der Treiber beendet seine Compiler-/Prüfprozesse jeweils nach 60 Sekunden; das ist nur ein Auditprozess-Limit. Es gibt keine nachgewiesene produktive CPU-/Speicher-/Abbruchgarantie. Testkorpus und Generierung sind begrenzt; ein Betriebslast- oder Performancevergleich wurde nicht durchgeführt. Getestete lokale Toolchain siehe Übergabe; daraus folgt keine Workspace-Kompatibilität.

## Fachliche Abnahme nach G1

`DOMAIN_PARITAET.csv` ist ein offener Arbeitskatalog, nicht die verlangte vollständige Paritätsabnahme. Nicht ausgeführte Prüfungen bleiben ausdrücklich unbestätigt. S01 muss den Gesamtfunktionsbestand einschließlich aller regelmäßigen Update-/Lernpfade abschließen.

Vorschläge zur Freigabe durch 00/02/03/10: IDs, Ganzzahlen, Zählwerte, Budget/Souls und Build-Legalität exakt; diskrete Kaufreihenfolgen mit festem Tie-Breaker exakt; für reine f64-Referenzrechnungen zunächst `abs(a-b) <= 1e-9 + 1e-9 * max(abs(a),abs(b))`, sofern die fachliche Einheit keine strengere Grenze fordert. Diese Float-Grenze ist noch nicht genehmigt und darf weder Rangfolgefehler noch Unknown/NaN/Infinity kaschieren. Seeds, Einheiten, Rundung, Patch, Mode, Variante und Verfügbarkeit bis zum Stichtag gehören pro Dataset in den späteren gemeinsamen Vertrag.

Build-/Rule-Abnahme benötigt kleine exakt enumerierte Referenzprobleme, Inventar-/Upgrade-/Budgetgrenzen, unbekannte Regeln und kontrollierte Laufzeitabbrüche. Learning-Abnahme benötigt sowohl Artefaktladung als auch einen vollständigen regelmäßigen Rust-Updatezyklus auf fixierten Daten. Population/Coaching benötigt deduplizierte zeitliche Match-Holdouts und getrennte mechanische versus empirische Evidenz. Keine dieser Prüfungen ist durch diese Aliasprobe erledigt.

## Nicht angefasst / nicht nachgewiesen

Keine Produktivdatenbank, keine Imports, keine echten Replays, keine Provideranfragen und keine Lernjobs ausgeführt. Keine Live-Rechte-/Runtime- oder Netzwerkauditbehauptung für die gesamten Fachcrates. Retrieval, Wiki-Projektion, Provider und Consumer bleiben bei ihren Besitzern. Es wurde kein allgemeines LLM-Prompt als Ersatz eingebaut.
