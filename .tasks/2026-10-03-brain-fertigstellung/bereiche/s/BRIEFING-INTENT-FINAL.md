status: vorbereitet
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-s

# S2: frische finale Intent-Abnahme

Dieses Briefing wird erst nach dem Abschluss beider eigenen Prüfnachläufe gestartet. Der Werkzeugauftrag nennt die tatsächlichen finalen Steam- und Brain-SHAs. Keine Abnahme eines unbenannten Zwischenstands.

## Ziel und Vertrag

Prüfe unabhängig, ob der lokal übergabefähige Stand das Nutzerziel erfüllt: Brain-CLI veröffentlicht über `brain.build_publish.v1` und den vorhandenen Steam-HTTP-Endpunkt; positive spielinterne Build-ID wird als DONE bestätigt. HTTP- und GC-Drosselung bleiben sichtbar. Ein unklar abgeschlossenes Publish wird aus derselben gespeicherten Anfrage wiederaufgenommen, ohne neue Berechnung oder neue ID.

Lies AUFTRAG.md, PAKETE.md, GEMEINSAM.md, BRIEFING-S.md und UEBERNAHME-CODEX.md in `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/`, außerdem den aktuellen Bereichsvertrag VON_HAUPT.md, REVIEW.md und DEPLOYVERTRAG.md unter bereiche/s. Spätere Bereichsentscheidungen haben Vorrang vor älteren Einzelmerge-/Deploy-Anweisungen. Z besitzt gemeinsame Integration und Installation; S führt danach den tatsächlichen Live-Publish aus.

## Eigentum

Ausschließlich lesen. Steam `/home/nathanael/.worktrees/steam-publish-fertig`, Brain `/home/nathanael/.worktrees/brain-fertig-s`. Keine Produktivdateien oder zentrale Akten ändern, keine Git-Mutationen, Builds, Compiler, Tests, Dienst- oder HTTP-Aufrufe. Keine Unteragenten oder T3-Threads, keine Sessionkoordination. Secrets und Prozessumgebungen nicht lesen, ausgeben oder speichern. Rohbericht als Rückgabewert an S2; eigene Berichtdatei darf unter bereiche/s/pruefung-v2/intent-final/ neu abgelegt werden, keine vorhandene Datei überschreiben.

Graphify zuerst, dann benannte Fundstellen und tatsächlichen Diff prüfen. Kein vollständiger Graph-Neuaufbau. Alter Graph oder alter Bericht ersetzt keine Prüfung des finalen Codes.

## Arbeitsstand

Prüfgrundlage ausschließlich die im Werkzeugauftrag ausdrücklich genannten vollen finalen SHAs und die dazu gebundenen Nachweise. Steam-Basis `4c5621763d5f01c96d7912400517c08aa1c40df1`, Brain-Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`. HEAD und Dateistand vor sowie nach der Abnahme abgleichen. Bei Änderung keinen gebundenen Abschluss behaupten. GPT-6.1 Sol erben, xhigh, kein Modellwechsel.

## Beweisziel

Die fünf ursprünglichen Befunde aus REVIEW.md müssen im tatsächlichen Endstand geschlossen sein: GET-Erholung nach unklarer POST-Antwort, bestätigter Endzustand vor Fristprüfung, unveränderte CLI-Wiederaufnahme, sichtbares anhaltendes 429 und Fehlerexit bei BLOCKED. Reguläre Freigabe- und explizite Review-Regeln dürfen nicht verwischt werden. Prüfe, dass der gespeicherte Pfad im Resultat genannt wird, der Resume-Pfad die vollständige validierte Anfrage nutzt und reguläre CLI-Publish-Pfade keine direkten DB-Queue-Helper aufrufen.

Prüfe Nachweisgrenzen ausdrücklich: echte Testzahlen und innere Cargo-Exits, finale Source- und Dependency-Bindung, unabhängiger minimaler Steam-Lockdiff, Gate am Endstand, eigener Featurepush. Ein vorhandenes altes ALLOW oder ein äußerer Exit 0 genügt nicht. Statische Rust-/Security-Befunde und etwaige offene Punkte in REVIEW.md berücksichtigen, aber selbst die Nutzerzielabweichungen suchen.

Rückgabe getrennt:

1. Lokal implementiert und fachlich an Z übergabefähig: ja/nein mit Belegen.
2. Offene lokale Abweichungen: konkrete Eingabe oder Zustand, Auswirkung, Pfad und Zeile, falls vorhanden.
3. Vollständiger Liveabschluss: nur bei tatsächlichem DONE-Nachweis mit positiver hero_build_id. Erwartet ist vor Zs Installation noch kein solcher Nachweis. Diese fehlende Wirkung allein blockiert die lokale fachliche Übergabe nicht, muss aber als offener Vertrag stehen bleiben.

## Routing

Teil-Orchestrator S2 dieser nativen Session. Hauptauftraggeber Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Status allein teil-s2 unter status/s/2. TODO.md und REGISTER.md nicht schreiben. Ergebnis enthält beide vollen geprüften SHAs und Nachweisorte. Integration, finales gemeinsames Gate und Installation bleiben bei Z; keine Selbstermächtigung zu Merge oder Publish.
