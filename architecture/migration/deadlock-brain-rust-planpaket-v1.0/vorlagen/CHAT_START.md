# Start eines einzelnen Arbeitschats

```text
Arbeite am Deadlock-Brain-Umbau nach Plan v1.0 als Chat <NN>.
Lies architecture/migration/chats/<NN_DATEI>.md sowie die dort genannten Regeln.

Arbeitsmodus: <prepare_only | implement | integration_test | cutover_rehearsal>
Integrierter Basis-Commit: <SHA>
Contract-/Schema-Version: <Versionen>
Freigegebene Gates und ihre Nachweise: <G0/G1/...>
Deine realen freigegebenen Pfade: <aus PFAD_OWNER.csv>
Benötigte Übergaben: <Pfade und Ergebniscommits>

Prüfe die Startbedingungen im Parallelplan. Bei fehlender Freigabe nur
vorbereiten und konkrete Blocker melden; keine Ersatzarchitektur erfinden.
Arbeite im eigenen Branch/Worktree und mit isolierten Testdaten.
Gemeinsame Contracts/SQL/Lockfiles nur über ihre Besitzer ändern.
Produktive Datenänderungen, Publishing und Cutover sind hiermit NICHT freigegeben.
Liefere tatsächliche Änderungen/Tests und eine versionierte Übergabe.
```

00 bzw. der Nutzer füllt Platzhalter mit echten freigegebenen Werten. Leere Felder nicht mit angenommenen Commits, Tests oder Freigaben ersetzen. Bei fehlendem Repozugriff nur klar gekennzeichneten Entwurf liefern.
