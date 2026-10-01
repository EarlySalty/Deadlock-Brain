# V1: ein bestehender öffentlicher Botfakt

Der Nutzerauftrag zum Fertigbau umfasst den echten bestehenden Consumer. Dafür
wird ausschließlich dieser bereits veröffentlichte, eigene Botfakt übernommen:

> `!commands` schickt einen Link zur Befehlsübersicht.

## Überprüfbare Herkunft und bestehender Zweck

Original: Deadlock-Twitch-Bot, Commit
`95c28e982e68ab948b078009dba1e687eac43f6f`,
`rust/knowledge/bot/chat-befehle.md`. Die exakte Git-Datei hat SHA256
`77ead8b97b92299740ee21397047be112d5f3f1e4579e7c41f66aeff630acf78`.
Der Import liest dieses unveränderliche Gitobjekt, prüft den Dateihash,
die öffentliche Audience `streamer`, Namespace `bot` und den exakten Auszug.

Öffentlicher Spiegel: Deadlock-Docs (GitHub `isPrivate=false`), Commit
`67bb24706fa37fbdccb6b99845f06c45d1448d0e`,
`public/twitch-bot/chat-befehle.html:25`, SHA256
`ccc072d6c3c952a7c0619fb73e7e6f48fd41137be5c951023e271d8b6009ddd3`.
Die Seite bestätigt, dass `!commands` auf die Befehlsübersicht verweist.

Implementierung am Bot-HEAD `d828481624d53408e0c0a4c3ed1a8e4a6d421c40`:
`rust/crates/tb-chat/src/catalog.rs:125`, Dispatch in `commands.rs:634`,
`cmd_commands:790`. Dies ist vorhandenes Verhalten, kein neues Verkaufsversprechen.
`rust/knowledge/SSOT-HINWEIS.txt` benennt diese Botdokumente als Livequelle
und Deadlock-Docs als öffentlichen Spiegel.

Bestehender generativer Zweck: `tb-knowledge/src/doc.rs::ist_oeffentlich`
erlaubt ausdrücklich streamer/public/viewer. Der öffentliche Self-Explainer
lädt diese Wissensbasis; `answer_question:440` reicht `grounding.facts:463`
an `fireworks_generate`, das über `tb_llm::complete:386` antwortet.
Die Veröffentlichung und Providerweitergabe dieses eigenen Botfakts existieren
bereits. Es wird ausschließlich dieser Zweck im neuen kanonischen Antwortkern
fortgesetzt; Drittinhalte oder Archivpatchnotes erhalten dadurch keine Rechte.

## Umfang und Abnahme

Source `ddc-bot-firstparty-v1`, Logical-ID `commands-overview`, ausschließlich
Scope `bot.public`. Strukturierter Git-Origin, Lizenz weiterhin unbekannt,
Autorisierungsreferenz SHA256 der konkreten belegten Zweckentscheidung,
Publication/Egress/Raw-Retention für diesen kleinen eigenen Text.
Bestehende zentrale Auswahl DeepSeek Flash bei Fireworks bleibt unverändert.

Ein separat bezeichnetes Probeobjekt `ddc-bot-firstparty-probe-v1` darf denselben
echten Auszug im selben Scope übernehmen. Es dient ausschließlich den vom
Nutzerauftrag umfassten Widerrufs-/Löschgegenproben. Der reguläre Importpfad
verwendet `DocumentSetSource`, `prepare_document_batch`, die vorhandene Lease,
atomaren Head-/Checkpointvergleich und `CorpusRelease`; kein SQL-Sentinel.
Nur das Probeobjekt darf durch die neue CLI widerrufen oder tombstoniert werden.
Nach einem Tombstone darf ein späterer Revoke-Aufruf es nicht wiederbeleben.

Consumerprobe über den bestehenden `BrainKnowledgeAdapter` gegen den echten
revisionsgebundenen Dienst. Der Self-Explainer-HTTP-Handler wird nicht für den
Test aufgerufen, weil seine nachgelagerten Discord-Logs Communitynachrichten
auslösen könnten. Keine Testnachricht und keine automatische Testlog-Nachricht.
Wiederverwendung desselben Releases/Index/Cache muss aktuelle Sperren beachten.
