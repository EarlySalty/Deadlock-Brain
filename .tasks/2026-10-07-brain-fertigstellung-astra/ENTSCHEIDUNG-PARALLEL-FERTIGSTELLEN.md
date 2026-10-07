[Orchestrator]
# Verbindlicher Nutzerentscheid: parallel bis live fertigstellen

Nutzernachricht vom 07.10.2026 nach Start der I-/G-Fortsetzungen. Ersetzt frühere starre Gesamtfolge Spiegel/F/G/K und jedes Warten auf den Merge eines anderen Pakets bei getrennten Schreibbereichen.

1. I b17d5729-a475-4bcb-8fc9-0aa6103d4555, G ee3de2ba-30ab-4558-a57c-6c1de154891e und K 79c97ab5-f014-4e17-9d00-20c7adaf83ff arbeiten gleichzeitig in ihren bestehenden Schreibbereichen. Keine Doppelwriter, keine uncommittierten Fremdänderungen übernehmen.
2. K verdrahtet jetzt gegen Gs gesicherten ANTWORTPORT-VERTRAG.md und die zugehörigen geprüften Commits, mit eigener gemeinsamer Compiler-/Vertragsprüfung. Nicht auf G-main warten. Tageslimit und Ortskontext sofort unabhängig vom Gesamtprojekt regulär bis live bringen. Laufenden kleinen Quotenabschluss nicht abbrechen oder ungeprüfte Gesamtintegration mitziehen; Ortskontext direkt als eigenen passenden Integrationsschritt liefern, nicht auf I/G warten.
3. I setzt die bereits genehmigte eine URL-Varianten-Fixrunde samt Gesamtgate fort. Bei ALLOW Spiegel und Discovery gemeinsam, nur bei neuem inhaltlichem Fund Discovery erhalten und Spiegel separat. F-Teil direkt nach dem Spiegel-Merge beginnen, nicht nach G-Merge. Gs gesicherte geprüfte Rechenverträge verwenden; tatsächliche Compiler-/Datenabhängigkeiten gemeinsam belegen, keinen zweiten Rechner oder Scheinadapter bauen. Liveabschluss des Spiegels bleibt erforderlich.
4. G führt S2/S3/S4 und Produktionsanschluss jetzt im eigenen Bereich bis Abschluss; benötigte Verträge früh gesichert bereitstellen. analytics_runtime bleibt bis Is ausdrücklicher Eigentumsfreigabe bei I. Das ist Schreibschutz, keine allgemeine Merge-Wartepflicht.
5. Merges in Reihenfolge tatsächlicher Fertigstellung, jeweils gegen den dann aktuellen main neu integrieren und regulär gateprüfen. Danach aktuelles origin/main über regulären serialisierten Deployweg liefern, Runtime-/SHA-/Funktionsnachweise und Cleanup. Keine starre Paket-Mergereihenfolge mehr, ALLOW bleibt Scope-/SHA-gebunden.

Docs-Thread 59740e62 arbeitet separat in Deadlock-Docs und wird von dieser Session und ihren Paketen weder gelesen noch verwaltet. Keine Wartefenster darauf. Q bleibt wie beauftragt erst nach tatsächlichem I/G/K-Livegang, identisches festes Evalset, zunächst Luna; keine kostenpflichtigen Läufe ohne Freigabe.

Unverändert: Rollenbindung, minimaler ID-freier Antwortkontext, keine Rohdaten an Codiermodelle, keine Secrets, bestehende zentrale Provider und Timeouts, eine Discord-Grenze 50 Fragen je Nutzer/Berliner Kalendertag aus bot.toml, keine zusätzliche Sekunden-/Kanal-/Globalquote. Kein Hook- oder Rechtebypass, kein PR-/Actions-Ersatzgate, keine unerlaubte Löschung.
