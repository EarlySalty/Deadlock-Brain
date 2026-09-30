status: vorbereitet, erst nach eingefrorener B2-Abgabe beauftragen
Datum: 2026-09-30

# B2: unabhängige Deltaabnahme des vorhandenen Importers

Intent562a877b-0939-440a-964d-1145d9e9431a. Bestehender Reviewer52c34332-8cdf-4772-9e1f-42aba432c6cf nach separater B1-Abnahme. Berichtworktree /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929, Branch review/pre-g5-core-abnahme-20260929. Du bist der einzige Reviewerthread für dieses Paket. Keine Unterthreads oder Unteragenten, keine Code-Kommentare.

## Bindung

Autorauftrag: B2-IMPORT-BRIEFING.md in dieser Akte. Quelle /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930. B1-Code0a6e095 und separater Bericht35673e7 sind vorhanden. Der Dispatch muss den tatsächlichen abgeschlossenen B2-Head und dessen Basis nennen. Ohne diese Bindung keine Abnahme eines beweglichen Arbeitsbaums. Autorbericht ist zu prüfende Behauptung, kein unabhängiger Nachweis. Vor Schreiben eigenen Gitstand prüfen; nur eigenen Bericht committen und auf eigenen Branch pushen.

## Nutzerziel und unveränderte Grenzen

Vorhandenen Rust-Importer eng für brain.brain_legacy nach brain.brain auf der dedizierten Instanz erweitern. Kein neuer Importpfad, kein Rollen-/Modellneubau, keine blinde Pilotkopie. Explizite normale Config und fail-closed Guards vor jeder Schreibwirkung. Quellrechte, private Patchnotes, Widerrufe und Tombstones erhalten. Quellfreigabe ist keine Import-, Laufzeit- oder Cutoverfreigabe.

Der Twitch-Integrator hat laut Nutzer das vollständige Originalgate ALLOW auf9f6f291d und beginnt den Produktionscutover. Bis zur ausdrücklichen Slotrückgabe keine Compiler, Rollenfixtures, DB-Schreibvorgänge, Produktprozesse oder Deployaktionen. Dieser Review bleibt unabhängig davon rein statisch: kein Cargo, Harness, DB-Zugriff, Modellaufruf, Dienststart, Secretlesen oder Testprozess. Graphify vor Codefragen, anschließend konkrete eingefrorene Quellstellen prüfen.

## Nachweisfragen

1. Welche CLI-/Library-Eintrittspfade können schreiben? Guards müssen jeden erreichbaren Claim-, Lease-, Record-, Checkpoint- und Releasepfad schützen. Bestehenden Pilotweg prüfen, nicht nur neue Produktionsverzweigung. Identische logische Quelle/Ziel bleiben verboten; dieselbe DB darf ausschließlich bei explizit gebundener Archiv-/Coreschemagrenze erlaubt sein.
2. Trägt die Bindung tatsächliche Instanz, Socket, Port, Datenbank, Schema und vorhandene Rollen? Was ist lokal konfiguriert und was wird an der Verbindung verifiziert? Fehlende, falsche oder widersprüchliche Werte müssen vor erster Zielmutation scheitern. Keine neuen ENV-Freischalter, dynamischen SQL-Bezeichner, DSN-/Secretlogs oder stillen Defaults auf Produktion.
3. Woher stammen freigegebener Archivsnapshot, aktuelle Rechte, Widerrufe und Tombstones? Die begrenzte Pilotprobe belegt private Patchnotes in brain.legacy.review, nicht die vollständige Policybaseline. Konfiguration darf keinen unbekannten Ausgangszustand zu public/game.public machen. Fehlende Herkunft oder unvollständige Abdeckung muss blockieren. Falls konservativ eingeschränkt wird, keine volle Quellenparität behaupten.
4. Können Wiederholung, Snapshotwechsel oder Abbruch ältere Rechte/Sperren überschreiben oder Tombstones wiederbeleben? Egress- und Publikationsrechte bleiben gebunden. Keine zweite unverbundene Policyablage, keine per Hand passend erfundene Freigabe. Prüfung anhand derselben fachlichen Regeln für beide vorhandenen Quellen.
5. Was ist die echte Atomaritätsgrenze? Teilimport, Fehler nach Claim/Checkpoint und Fehler vor Release getrennt beurteilen. Kein vollständiger Release aus unvollständigem Zustand; echte Pins und Idempotenz belegen. Rückweg darf nachfolgende Writes, Ownership, ACL oder Tombstones nicht verlieren. Ein dokumentierter Stopppfad ist noch kein belegter Restore.
6. Decken die ergänzten Testquellen fehlende Produktionsbindung, falsche Endpunkte/Rollen/Schemas, identische Quelle/Ziel, fehlende Policybaseline, private Patchnotes, Scopewiderruf, Tombstone, Wiederholung und Abbruch vor Release ab? Positiver Scratchfall muss die echte Produktionsverzweigung beweisen, nicht bloß den alten Pilotzweig. Nicht ausgeführte Testquellen ausdrücklich so nennen.
7. Bleibt der Scope beim bestehenden Importer, normaler Importconfig und Abgabedokumentation? Kein neuer Provider, Consumer, Dienst, Lock-/Dependencywechsel oder Abschwächen bestehender Tests. Beispielconfig ohne vollständige Bindung muss nicht ausführbar bleiben.

## Abgabe

B2-IMPORT-REVIEW.md mit exakter Basis/Head, fertigJ/N, FixJ/N und nummerierten überprüften Befunden samt Datei:Zeile, konkretem Fehlerszenario und Zwillingssuche. Unauffällige Schutzpfade ebenfalls knapp belegen. Pflichtzeile WIRKUNGSPRUEFUNG[WP-1] nach bestehender Prüferakte. Bei GO exakt benennen, was statisch abgenommen ist und welche Laufzeitgegenbeweise fehlen.

Exakte nächste Compiler-/Test-/Scratchbefehle gegen den geprüften Head verlangen: vorhandener Targetcache /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target, Toolchain1.97.1, locked/offline, ein Job, keine neuen Caches. Laufklassen und Schreib-/Rollen-/Lastwirkungen offenlegen. Serve-E2E bleibt600 Anfragen je8/16/32 Worker und ist kein leichter Smoke. Keine Befehle ausführen und keine bloßen Autorannahmen als Ressourcenbeweis übernehmen.

Bei berechtigtem Befund Fix im vorhandenen Sol-Thread durch Orchestrator, kein selbstständiger Produktfix durch den Reviewer. Fehlende reale Freigabebaseline als konkret fehlenden Pflichtinput melden; weder Produktentscheidung erfinden noch pauschal nach erneuter Deployfreigabe fragen.
