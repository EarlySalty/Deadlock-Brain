# G: Bestandsbefund Antwortdienst

status: erledigt, 07.10.2026. Recherche abgeschlossen, kein Werkzeugloop implementiert.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-serve/src/service.rs:418 | Anknüpfung: bestehenden Kernel, Provider, Rechte, Budgets und Serverwissen erweitern

## Vorhandene Strecke

`brain-api/src/http.rs:19` bietet `/v1/answer` mit Authentifizierung vor Body-Lesen, 64-KiB-Grenze, gemeinsamer Laufzeit und Workergrenze. `brain-api/src/lib.rs:162`, `:199` und `brain-policy/src/lib.rs:155` binden Release, Actor, einschränkbare Scopes und registrierte `bot.public`-Consumer.

`brain-serve/src/service.rs:418` verdrahtet ReleaseRetriever, AnalyticsRetriever, DiscordRetriever, Kernel, Cache und ApiService. `brain-kernel/src/execution.rs:183`, `:337`, `:397` prüft Belege vor Weitergabe und nach Modellrunde erneut, auch nicht zitierte Modelleingaben.

## Werkzeuglücke und gemeinsame Änderungen

Luna ist über `codex_subscription`, Loopback 18769 und den vorhandenen Provider konfigurierbar. `brain-serve/src/service.rs:208`, `config.rs:421` sichern Modell-, Transport- und Konfigurationsvertrag; laufende Konfiguration nicht gemessen.

Modellwerkzeuge fehlen: `brain-contracts/src/provider_input.rs:14` kennt Textnachrichten; `brain-providers/src/lib.rs:132` erwartet Textantworten. `transport.rs:143` setzt `tools: []` und `tool_choice: none`; `:330` verwirft `tool_use`.

Nötige Erweiterung im bestehenden Weg:

- `brain-contracts/src/lib.rs`, `provider_input.rs`: Tool-Schema, Aufruf-ID, Argumente, Tool-Ergebnisse, Provider-Turn und vollständige Eingabe-/Usagezählung.
- `brain-providers/src/lib.rs`, `transport.rs`, `hardening.rs`: native `tool_use`/`tool_result` sowie OpenAI-kompatible `tool_calls`, nullable Content, Folgerunden, Abschlussgrund und Belegprüfung vor jeder Weitergabe. Keine neue Providerart oder Modellwahl.
- `brain-kernel/src/execution.rs`, `flight.rs`: begrenzter Turn-Loop, Rechte, kumulierte Usage, unveränderter Request-Deadline und Cache-Neuprüfung sämtlicher Tool-Abhängigkeiten. Jede Belegabhängigkeit behält ihre typisierte Unteranfrage.

Die lokale Abobrücke muss Toolblöcke tatsächlich durchreichen. Kein Transportbeweis im Brain-Repo vorhanden. Der MiniMax-Loop in Deadlock-Bots ist kein zulässiger Ersatz.

## Budgets und lesende Tools

Bestehend: `max_network_rounds`, `max_input_tokens`, `max_output_tokens`, `max_cost_micros` und ursprüngliche Laufzeit aus `brain-contracts/src/lib.rs:135` und `brain-serve/src/config.rs:109`. Wiederholter Gesprächskontext, Tooldefinitionen, Argumente, Ergebnisse und sämtliche Usage kumulieren; keine frische Deadline pro Turn, keine ENV-Schalter oder neuen festen Timeouts.

Sieben Werkzeuge mit serverseitigem Actor, Rechten und Versionsbindung:

| Werkzeug | Ein-/Ausgabe und Bestand |
| --- | --- |
| `entity_find` | Suchtext, Art, Sprache; stabile Schlüssel oder Mehrdeutigkeit. Identitätsauflösung `brain-storage/src/local_pg_reader.rs:897` wiederverwenden, typisierter Kandidatenport fehlt. |
| `entity_profile` | Schlüssel, Feldauswahl, Version, Szenario; Roh-/Rechenwerte, Einheiten, Ränge, Herkunft und Unbekanntes. Vorhandener Profilvertrag `brain-contracts/src/entity_profile.rs:79`; Dokumentansicht durch Rechenansicht ersetzen. |
| `hero_compare` | Helden und gleiche Version/Szenario/Felder; passende Werte und Differenzen. Bisheriger Textpfad verarbeitet nicht zwei Profile zugleich. |
| `damage_calculate` | Held, Inventar, Imbues und explizites Ziel/Szenario; Zahlen, Regeln, Annahmen, unquantifizierte Effekte. Gemeinsamer `dbrain-reasoner`-Kern, keine freie SQL-/Netz- oder Schreibfähigkeit. |
| `patch_history` | Entität, Fähigkeit/Feld, Zeitraum; alte/neue Werte und Originalherkunft. Bestehender Leser `brain-storage/src/entity_profile.rs:587` und `local_pg_reader.rs:1071`, Bereichsfilter ergänzen. |
| `build_plan` | Held, Spielstil und belegte Version; BuildObject, Kauf-/Fähigkeitsreihenfolge, Varianten, Belege. Bestehender Planer `dbrain-reasoner/src/lib.rs:99`; keine Veröffentlichung. |
| `server_knowledge` | Frage und freigegebener Kanal; Dokumentbelege und aktuelle öffentliche Fakten. Vorhandenen DiscordRetriever `brain-serve/src/discord_live.rs:468` benutzen und bereits geladene Fakten wiederverwenden. |

Serverwissen kommt heute zusätzlich über `public_server_facts` auf Loopback 8890, mit Beobachtungszeit und Gültigkeit. Keine Verwaltungs-, Invite- oder Schreiboperationen als Tools. Lexikalische Suche künftig für freien Text, nicht aktuelle Spielwerte.

## Build- und Paketgrenze

`dbrain-retrieval/src/lib.rs:1302` besitzt einen lesenden Buildweg mit `ai: None`, `use_ai: false`, `persist: false`. Er nutzt bislang Pooldaten und aktuelle Patchannahme; erkannter Spielstil wird dort nicht angewandt (`:1356`). G verwendet Es versionsgebundene Rohwerte und den vorhandenen Planer. `reason_build` mit Defaults würde persistieren und ist dafür falsch.

F besitzt `deadlock-brain/src/main.rs:2714`, `:2813`, `:2921` und Publish-Regel/Confidence/Planer. Diese Aufrufer veröffentlichen und dürfen nicht als Tool dienen. A besitzt Übergangsfreischaltung, Bot-Consumer und Invites. G ändert deren Rechte nicht.

## Herkunft

Rechercheagent `a2e0399691fd75107`, Workflow `wf_4e39a761-389`, Abschluss bestätigt. Vollbericht im zugehörigen Taskoutput `wyqxc1iva.output`, `result[1]`. Bericht wegen Worker-Ablagegrenze zurückgegeben, durch Bereichsführung in diese Akte übernommen. Kein Produktcode, DB-Schreiben, Git-Schritt oder Livebeleg des Rechercheagenten.
