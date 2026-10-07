# A-G2: vorhandene Twitch-Spielwissensfreigabe prüfen

## Ziel und Vertrag

Grundding hat Vorrang: Discord und Twitch antworten zeitnah aus einem Brain mit aktuellem Spiel- und Serverwissen. A-G1 belegt die bestehende Twitch-Credential mit entity_profile_model_context=false. Discord/Docs sind true. Prüfe ausschließlich den vorhandenen Registrierungs-/ConfigWriter-Weg und die tatsächlich nötige Minimaländerung. Keine Umsetzung eines zweiten Readers, kein Modell-/Timeoutwechsel, keine Freigabe von Community-Rohdaten. Spielwerte sind freigegeben, Personen- und Rohchatdaten nicht.

## Eigentum

Read-only Auftrag. Produktcode, Runtimekonfiguration, Secrets, DB, Dienste und Root-Akten nicht verändern. Quellen in /home/nathanael/.worktrees/brain-a-abschluss-20261006 lesen. Eigene kleine nicht-geheime Belege nur unter /tmp/brain-a-twitch-grant-proof-20261007/. Keine zusätzlichen Agenten, Reviewer oder T3-Threads, keine fremde Sessionkoordination. Ergebnisse als Rückgabe an A.

## Arbeitsstand und Anknüpfung

Brain-Main 8d61a949 regulär gepusht, A-R1 baut im separaten Releaseworktree. Andere normale Wartungs-/Aktivierungsoperationen laufen möglicherweise gleichzeitig. Deshalb jetzt keine Konfigmutation oder Restarts.

Graphify zuerst, dann gezielte Fundstellen. A hat global entity_profile_model_context/ConsumerGrant/register_config abgefragt, konkrete zweite Abfragen liefen nach 120 Sekunden ins Timeout. Aktuelle Sourcebelege: brain-serve/src/config.rs:174 Credential, :532 bestehende explizite Spielprofilgrenze mit genau bot.public oder docs.public und genau public; brain-maintenance/src/integration/runner.rs:364 register_config; brain-maintenance/src/integration/activation.rs:89 gemeinsame Publikations-/Pin-Auswahl; brain-storage/src/local_pg_reader.rs:498/739 Lesefreigabe; ops/brain-maintenance/discord-credential.example.json als vorhandener Vertrag.

## Beweisziel

1. Exakte aktuelle Twitch-Identität/Kanal, Scopes, Egress, vorhandener Releasepin und Maintenancebindung belegen, ausschließlich nicht-geheime Felder ausgeben. Niemals DSNs, Geheimniswerte, Environment-Dumps, Rohkonfig oder Nutzerfragen ausgeben. G1 warnte bereits vor einem Zugangsteil im Tooloutput.
2. Bestehendes ConfigWriter-/Registrierungs-CLI, Rechte, Hash/CAS und Neustartwirkung mit tatsächlichen Pfaden und Aufrufen ermitteln. C9-/Docs-/Operatorpins, andere Credentials und Quellenpolicies müssen erhalten bleiben.
3. Entscheiden, ob die vorhandene enge boolesche Freigabe allein für Twitch genügt, und ob der gewünschte Profil-/Patch-Storybestand derzeit wirklich veröffentlicht/aktiviert ist. Fehlende Quellenfreigaben oder Rechte konkret melden, keine pauschale Erweiterung empfehlen.
4. Kurz ausführbaren normalen Aktivierungsweg nennen, nach A-R1 und mit frisch gemessenen Hashes. Nicht selbst ausführen. Kein eigener Code, keine Testpflicht oder Compilerkaskade für die Lesekarte.

## Routing und Rückgabe

Paket A, Versuch G2, alleiniger Ereignisproduzent ist dieser native Worker. Integrationsverantwortlich 2c7de4c9-bac4-43ad-b91a-f8ac889f09b4, Hauptorchestrator 3fcd8f71-443e-48ae-825c-527eb52fbe56. Rückgabe mit konkretem Urteil, Pfad:Zeile, erforderlicher Wirkung und Grenze, keine Secretwerte. Root-Akte schreibt nur A. Wache nach 20 Minuten, spätestens 30, keine selbst erfundenen kurzen Abbruchbudgets.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-abschluss-20261006
