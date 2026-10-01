# Schnittstellen-/Scope-Änderungsantrag CR-S05-001

Antragsteller-Chat: 05
Besitzer des betroffenen Moduls: 02 (gemeinsame Verträge), 03 (Store/Schema), 00 (Freigaben und Owner)
Basis-Commit/Contract-Version: `c00fc8935048bf490c1e4790f7c6195864ad49e2` / offen
Betroffene Dateien/Requirements: `rust/crates/dbrain-normalize/src/util.rs`, künftiger gemeinsamer Entity-/Fact-/Rule-/Evidence-Vertrag und Aliasstore; R10, R28–R34, R36–R37, R43, R47, R52–R53, R58.

## Benötigte Änderung

**Keine Änderung in diesem PR umgesetzt.** G0/G1 sind im integrierten STATUS offen. Zuerst integrierte S01-/S10-Nachweise, gemeinsame Contract-/DB-Schema-Version und S05-Arbeitsfreigabe mit eindeutigem Pfadbesitz bereitstellen. Population-Pfade überschneiden sich aktuell zwischen 05 und 14; keine parallelen Änderungen ohne Auflösung.

Für die zuerst geprüfte Aliasnormalisierung zeigt `probes/alias_cases.rs` drei Semantikfragen:

1. Python `casefold()` gegen Rust `to_lowercase()`: `ß`, `Straße`, `Σςσ`, `ﬀ` liefern unterschiedliche Schlüssel. Vorschlag: gemeinsames Unicode-Casefolding samt festgelegter Unicode-/Normalisiererversion prüfen. Keine ad-hoc-Ersetzung lediglich dieser vier Fälle und keine neue Root-Dependency durch S05 vor Freigabe.
2. Nach Unterstrichersatz entstehen in Python Rand-Leerzeichen (`__MO__` → ` mo `; `___` → ein Leerzeichen). Erneute Normalisierung verändert diese Ergebnisse. Vorschlag: Idempotenz und leere Aliase explizit entscheiden, statt diesen Altbug unbesehen zu portieren. Die im vorhandenen Rust strengere Trimmung ist bisher nicht als genehmigte Abweichung belegt.
3. Python behandelt U+001C als Whitespace, das vorhandene Rust lässt es im Schlüssel stehen. Vorschlag: Menge zulässiger Zeichen bzw. Behandlung von Steuerzeichen festlegen und alle Alias-Erzeuger/-Leser auf denselben Vertrag verpflichten.

Die kleinste Umsetzung nach Entscheidung soll die vorhandene gemeinsame Normalisierungsfunktion nutzen; keine neue private Entity-ID-Schnittstelle. Aliase sind keine stabilen Entity-IDs.

Die übrigen S05-Arbeiten benötigen gemeinsam fixierte Semantik für Fact/Effect/Rule, begrenzte Ausdrücke, Unknown, Einheiten/Rundung, Patch/Mode/Variante, Algorithmus-/Datensatzrevision, DomainStore/Artefaktladung, Observation/Population und getrennte Quellverfügbarkeit vs. Game-Gültigkeit. S05 liefert hierzu den offenen Arbeitskatalog, aber keine parallelen Typdefinitionen.

## Folgen

Eine Änderung normalisierter Schlüssel kann Alias-Zuordnungen und Lookup-Indizes betreffen. 03 muss Bestandskollisionen prüfen, Backfill/Reindex und Wiederanlauf dokumentieren; bei Mehrdeutigkeit nicht willkürlich eine Entity auswählen. Umfang realer betroffener Datensätze wurde nicht gemessen. S04/12 verwenden dieselbe Normalisierung; S06/S08/S09 müssen dieselbe Lookup-Semantik lesen. Keine ACL- oder Providerfreigabe ergibt sich aus einer normalisierten Zeichenfolge.

Ressourcen- und Float-Toleranzvorschläge stehen in README und Matrix. Sie sind noch nicht abgenommen und kein gemessenes Produktionsbudget. Es wurde kein Legacy-Dienst als Rückfallpfad eingerichtet.

## Übergang / Rückwärtskompatibilität

Zunächst nur synthetische Fixtures und eingefrorene Alt-/Rust-Belege. Vor Aktivierung einer neuen Normalisiererversion: freigegebene Kollisionsfälle, versionierter Rebuild und geklärter Rollback mit 03/00. Eine bewusste Altbugkorrektur braucht einen dokumentierten Produktentscheid sowie neue freigegebene Goldens; nicht einfach sieben fehlgeschlagene Fälle aus dem Test entfernen.

## Entscheid von Besitzer + Chat 00

Status: vorgeschlagen / offen
Freigabe/ADR: nicht erteilt
Neue Version: offen
Verpflichtende Regressionstests: alle 18 vorhandenen Goldens; Unicode-Casefolding; Rand-Unterstriche/Leerwerte; Steuerzeichen; wiederholte Normalisierung; Alias-Kollisionen und Mehrdeutigkeit; versionierter Rebuild/Reindex; idempotenter Wiederanlauf. Zusätzlich G1-Vertragstests für Facts/Rules/Unknown/Units/Zeitfilter und Ownerabgrenzung 05/14.
