# Nachfolgepakete C, D und E

Status aller Pakete: PLAN, nicht gestartet. Je C und D neuer Opus-4.8-Coder/Worktree auf dem tatsächlich freigegebenen und integrierten Vorgänger. Keine Implementierung vor dem Gate des Vorgängers. E erhält einen frischen unabhängigen Abnahme-Thread. Nutzerbefund, ORCHESTRIERUNG.md, PRUEFHINWEISE.md und vorherige Mess-/Reviewberichte sind gemeinsamer Vertrag. REGISTER muss reale Thread-/Commit-IDs statt erfundener Platzhalter enthalten.

## C – Downside und wirkliche Deckung

Vorhandenen MaxHealthLossPercent-Pfad reparieren/vervollständigen, nicht einen zweiten Abzug hinzufügen. Wirklicher Pool am Kaufzeitpunkt: Basisleben, datengeleitetes Wachstum/Rewards, Inventar-Boni, Prozentoperatoren und Reihenfolge. Prozent-, Flat- und multiplikative Downside nicht vermischen. Max-HP-Verlust und aktuelle HP/Zustandsanpassung müssen bei Kauf/Verkauf konsistent sein. Tests u.a. 1599 * 0,13 = 207,87 und 4615 * 0,13 = 599,95; Rundung erst an ausdrücklich belegter Stelle.

Durch B eingeführten Gegner-/Ereigniskontext nutzen. Schilde kanalgetrennt und zeitlich gültig; verbrauchtes Schild nicht pro Simtick neu schenken. Resistenzen nur auf passenden Schaden, Lifesteal/Regeneration nur bei wirklich fehlendem Leben, Anti-Heal berücksichtigen soweit Daten belegt. CC-Immunität/Dispel ist keine unbedingte Schadenimmunität, zeitweilige Weapon-Immunität kein Spirit-Schutz. Eine Defensive deckt Downside nur, wenn der konkrete erwartete Schadens-/Kontrollverlauf dadurch tatsächlich tragbar wird.

Generische Tests: identisches Offensivitem mit/ohne HP-Downside; gleicher hoher DPS bei Überleben vs. frühem Tod; passende/falsche Schildart, abgelaufener Cooldown/aktive Deckung, Burst vs. Dauerbedrohung, relevanter Kauf-/Levelzustand. Kein Name-Assert 'Glass Cannon immer negativ' als universelles Gesetz: die aus Nutzerfeedback abgeleitete Gegenprobe muss ihren realen gefährlichen Kontext explizit enthalten. Bei bekannt gefährlichem ungeschütztem Verlauf darf Mehrschaden die verlorene Handlungszeit nicht verdecken.

PHASE-C.md mit echten Tests/Metriken/Deltas/Restlücken. Kein Publish, keine zentralen DB-Writes, kein main-Merge durch Worker. Neue Produktlogik nur Rust. Patch auf eigenem Feature-Branch, Review separat.

## D – Compounding, Kauf und Verkauf, kausale Erklärungen

Ein Evaluator für Basis + Inventar + Kandidat, Upgrade und Verkauf/Ersatz. Datengeleitete Spirit-Konversion aus A und Trigger aus B mit Survival aus C durch alle Planner-/Composer-Pfade führen. Früh-/Spätphase und Slot-/Aktivlimits erhalten. Der Wert von A+B darf nicht aus isolierten Einzelitems addiert werden. Bei Ersatz Vergleich Inventar_alt gegen Inventar_alt - verkauft + gekauft; Komponenten, Shopboni und feste Imbues korrekt mitnehmen. Kein Verkauf eines Multiplikators aus einer veralteten Sololiste, keine Namenssperre als Lösung.

Begründungen aus wirklicher Berechnung: Quelle -> Statänderung -> Konversion -> Trigger/Uptime -> Waffen-/Ability-/Sustain-/Überlebensdelta -> Kaufentscheidung. Vorbedingungen/Downsides und nicht modellierte Wirkungen sichtbar, keine erfundene Erklärgeschichte. Namens-/ID-Umbenennung und Arrayreihenfolge dürfen bei identischer Mechanik keine andere Präferenz erzeugen (echte Gleichstände deterministisch behandeln). Sättigung/Redundanz aus Grenznutzen, nicht pauschal mehr Fire Rate als schlecht deklarieren.

Tests synthetisch plus reale eingefrorene Daten: Spirit/Rate-Multiplikatorverbund, Reload-/Magazin-/Imbue-Wechselwirkung, Replacement mit wegfallendem Multiplikator, Survival-Backup mit falschem/abgelaufenem Schutz, Stabilität bei Daten-Reihenfolge. Nutzer-Fehlurteile als Prüfgrößen messen, nicht Einkaufslisten im Produktcode hinterlegen. Vorher/Nachher und Holdouts identisch zu 0, PHASE-D.md mit Exitcodes und Grenzen. Kein Publish/Deploy/main-Merge.

## E – unabhängige Abnahme und Freigabe zum Release

Frischer Reviewer, keine Eigenfreigabe des letzten Coders. Prüfe gesamten Nutzer-Intent, nicht nur ob Tests grün sind: (1) fertig J/N, (2) Abweichungen mit Datei:Zeile + realem Fehlerszenario, (3) Fix nötig J/N, (4) Merge/Deploy/Publish jeweils frei oder BLOCK.

Pflichtnachweise: identischer eingefrorener Datenstand einschließlich PopulationPrior (bestehendes FROZEN-V2 allein lädt Population live nach); Referenzwaffen >= Phase-0, Staples/Gate, Kendall tau und Jaccard@12 mit klarer Quellenzuordnung; Multi-Hero-/Holdout- und Mechanikabdeckung, alle beanstandeten Kern-/Verkaufsentscheidungen; Rust-Pfad von Assets bis Publish-Payload, kein Python-Fallback; Guard für neue Namens-/ID-Sonderentscheidungen plus Verhaltenstest; mindestens drei nachweislich identische Rust-Replays. Drei unabhängige Live-KI-Läufe nicht aus no-ai-Replays oder bereinigten Unterschieden behaupten. Nondeterminismus/fehlende Live-Läufe sind separate offene Abnahme, keine Verschleierung.

Deployment nur durch Delegator nach vollständiger Freigabe und technischem Merge-Gate. Bestehenden .tasks/2026-09-12-build-reasoner/DEPLOY-BRAIN.md als historische Referenz lesen, tatsächlichen aktuellen Betriebsweg/Units/Binaryverbrauch erneut verifizieren. Sauberer isolierter Releasebuild, max. ein Host-Release, SHA und Rollbackkopie, kontrollierter Austausch, regulärer Restart/Oneshot, Exit/Journal/Prozesshash. Falls Migration nötig nur lokal getestet und durch Delegator angewandt. Publish erst nach fachlicher Abnahme; tatsächliche Steam-GC-Antwort / neue hero_build_id dokumentieren. Keine alte ID als neue ausgeben, keine Payload-Prüfung als Upload-Beleg.

REPORT.md / REGISTER.md führen Abschlussstand, Commit-/Datenhash, Tests, Review, Release, Live-Beweis, Publish-ID und verbleibende Grenzen. Fertige eigene Threads settle; Worktrees/Branches erst nach gesichertem SHA und nachgewiesener Integration löschen. Bei BLOCK kein Release/Publish und keine 'fertig'-Behauptung.
