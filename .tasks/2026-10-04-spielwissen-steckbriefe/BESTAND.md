Stand: 04.10.2026, 14:24 Uhr. D5 baut keine Produktquellen selbst.

Graphify-Abfrage mit vorhandenem Wortschatz: entity, summary, patch, history, search, knowledge, release, wiki, game, html, import. Gefunden: mcp/server.py:253/294/329 für patch_history, patch_search, entity_summary; dbrain-retrieval/src/game_wiki.rs:107 und deadlock-brain/src/pg_patchnotes.rs:2017. Der Graph ersetzt das Lesen des aktuellen Codes nicht.

Die MCP-Werkzeuge lesen brain.patch_changes direkt. entity_summary liefert Änderungsanzahl und betroffene Stats, noch keinen aktuellen Vollsteckbrief. In dieser Sitzung ist dl-brain nicht als Werkzeug verfügbar; vorhandenen Infisical-/FD-DB-Weg für echte Rust-Abfragen prüfen, keine zweite Patch-Historie erstellen.

Wiki-C liegt unverändert unter /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration, HEAD f7a03f9, mit umfangreicher uncommitteter Importarbeit. knowledge_contract.rs, knowledge_import.rs, CLI-Unterverzeichnis, pg_release.rs und knowledge_projection.rs wiederverwenden. Der alte Clippy-Befund zu BTreeSet ist im gelesenen Bestand bereits durch cfg(test) eingeschränkt; heutige Compilerprüfung steht aus. A-Commit 116f643 und B-Commits 7168574/f7a03f9 sind vorhandene Übergaben, kein Neubau der Parser.

Aktuelle eigene Basis: origin/main 8a88767a5dd95e1500cdf500c4d19e8c9e68fc5f. Der alte lokale main-Worktree brain-live-main steht dagegen auf 39710e3 und ist keine geeignete Baubasis.

deadlock-brain-site.service ist aktiv. ExecStart führt /home/naniadm/Documents/deadlock-build-corpus/site/server.py aus. Die Datei bedient über SimpleHTTPRequestHandler den vorhandenen Corpusroot /home/nathanael/Documents/deadlock-build-corpus. HTTP /site/ auf 127.0.0.1:8087 liefert 200. Rust kann dort erzeugte HTML-Dateien ausliefern lassen; keine Python-Änderung und kein zweiter Dienst erforderlich. Die öffentliche /brain-Pfadabbildung ist noch zu prüfen.

Nachtrag 14:27 Uhr: Aktive /etc/caddy/Caddyfile:433 bis 436 leitet /brain und /brain/ nach /brain/site/ um; handle_path /brain/* entfernt /brain und reicht an 127.0.0.1:8087 weiter. Steckbriefseiten im Corpus-Unterverzeichnis site/steckbriefe sind damit öffentlich über /brain/site/steckbriefe erreichbar. Keine Caddy-Änderung nötig.

Patchnotes-Kandidat bleibt inaktiv; Rechtefrage offen. Bestehende brain.patch_changes sind davon getrennt. Brain-main und Brain-Deploy ausschließlich W1; Worker tragen geprüfte Commits in welle1/w1/EINGANG.md ein. Alle importierten Wiki-Originaltexte bleiben intern und redistribution_allowed=false.
