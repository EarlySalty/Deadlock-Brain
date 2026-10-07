04.10.2026, 03:48 Uhr: Docsquery live erfolgreich. Ausschließlich die normale Konfiguration korrigiert: /home/nathanael/.config/deadlock-docs/bot.toml, timeout_ms von 5000 auf 30000. Scope bot.public, Endpoint und sämtliche Secretrefs erhalten. diff gegen die Sicherung zeigt genau diese eine Feldänderung; Datei weiterhin 0600/UID1000, Elternverzeichnis 0700.

Ursache belegt: Ein normaler CLI-query mit unveränderter TOML und vorübergehender Diagnose ausschließlich an lib.rs:117 ergab ClientError::Http mit is_timeout() = true, Ausgabe Clientfehlerklasse: HttpTimeout, Exit 64 nach 5,124 Sekunden. Keine Header, Bodies oder Secretwerte ausgegeben. PublicAnswerResponse und der betroffene Clientpfad stimmen zwischen dem gepinnten Brainclient 3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef und dem laufenden Brain 25ddcd896b02650e2f2d637319707f0c5ee20911 überein. Das laufende Backend hat request_ms=60000 und provider_ms=55000; die bestehende Docs-TOML erlaubt höchstens 30000. Der erfolgreiche Aufruf dauerte tatsächlich länger als die bisherigen 5000 Millisekunden.

Abschließender normaler Aufruf über die installierte CLI:

```bash
exec 5</run/user/1000/credentials/brain-serve.service/infisical-token
/home/nathanael/.local/share/deadlock-docs/releases/b04d1683a159109448ce93f4b88b841270e03fcd/bin/deadlock-docs-brain-adapter query /home/nathanael/.config/deadlock-docs/bot.toml 'Wie erstelle und verwalte ich eine Casual-Lane auf dem Discord-Server?'
```

Exit 0 nach 17,972 Sekunden, stderr leer. Antwort brain.public.v1, status=answered, request_id docs-brain-1909940-1791078472058251597. Tatsächlicher knowledge_release: maintenance-6edd9236639f9b06067a3b6b67fa410e5e668aa264db1b2baf28b7e1733940de, identisch mit dem gemeinsamen W1-/Second-Brain-Beleg.

Antwortauszüge: „Geh in den sichtbaren Voice-Router. Ohne gespeicherten Standard erstellt der Bot zunächst eine Casual-Lane, zieht dich hinein und macht dich zum Owner.“ Und: „Als Owner steuerst du deine Lane über das sichtbare Lane-Panel: Du kannst die Lane umbenennen, ein Teilnehmerlimit setzen sowie Mitglieder kicken, bannen und wieder entbannen.“ Die Antwort enthält außerdem die Voreinstellungen in der Willkommens-DM, die Moduswahl im Router-Panel, die Voraussetzungen für Owner-Aktionen und das automatische Aufräumen leerer Lanes.

Zwei tatsächlich ausgegebene öffentliche Quellen:

* Beleg 1: cite-d6824a5fa8c4eea94d94ffd299111cb90cbe6972a82a01c28f603dbe78f156fd.
* Beleg 2: cite-729614d8b1d30c2b370709248cfa9f31afd39a2730a19011411d871118be4bea.

Diagnosebau: /home/nathanael/.cargo/bin/cargo +1.97.1 build --manifest-path tools/brain-adapter/Cargo.toml -p deadlock-docs-brain-adapter --locked --offline --jobs 3, Exit 0, ein HOSTPROBE-Slot. Die vorübergehende Diagnose wurde vollständig entfernt; git diff --exit-code und git diff --check danach jeweils Exit 0. Keine dauerhafte Sourceänderung, kein Sourcecommit, kein zusätzliches Gate, keine Neuinstallation. Neu ausgeführte Tests: 0. Die 25 grünen Adaptertests gehören zum übergebenen Stand und wurden für diese reine Konfigurationskorrektur nicht erneut ausgeführt.

SHA-Sicherung nach erfolgreichem Livebeleg: Docs-Mainbasis und installierter Source-SHA b04d1683a159109448ce93f4b88b841270e03fcd; Brain live 25ddcd896b02650e2f2d637319707f0c5ee20911. Eigener Branch fix/docs-brain-live-transport-20261004 enthielt keinen zusätzlichen Commit und wurde nicht gepusht. Anschließend ausschließlich den eigenen Worktree mit git worktree remove und den eigenen Branch mit git branch -d entfernt, jeweils Exit 0. Sämtliche fremden Worktrees und die vorgefundenen Docs-Arbeitskopieänderungen bleiben erhalten. Keine Brain-/Second-Brain-/Twitch-Sourceänderung, Datenaufnahme oder Rechteänderung; kein Bedarf an W1.
