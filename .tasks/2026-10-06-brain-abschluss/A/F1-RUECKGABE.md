# A-F1: Ursache belegt, Fix bereits auf Main

Rückgabe des nativen Workers A-F1 am 06.10.2026. Der konkrete Gitfehler benötigt keinen weiteren Produktfix. Frisch geprüftes Main `10ebbb208aaaac9ddf6e07bc89a6af4a620f86ef` enthält bereits `37cfc6c677bd69c9ec75a6fda9af988ea98c07bb`: `scanner::materialize_game_source` und Anschluss vor der Offline-Extraktion. Laufende Releasezeiger bleiben e56e075d, der Fix ist daher noch nicht ausgeliefert. Stufe 1 nicht live abgenommen.

## Ursache und empirischer Beleg

GameTracking ist ein Teilklon mit `blob:none`. Im aus vorhandenen Originalpaketen rekonstruierten kalten Cache fehlen sechs benötigte Blobs. Gehärtete Offline-Abfrage Exit 128, `lazy fetching disabled` und `could not fetch 142350c4ce703a75e9aff674878cf50fc89cb61d from promisor remote`. Die ungehärtete Abfrage lädt diese automatisch nach. Daher funktioniert die gleiche Abfrage außerhalb des Dienstes. 95 Dateien, 11.237.808 Bytes, größter Blob 7.540.784 Bytes. Keine Ursache in Source-Scopes oder Datei-/Gesamtbudgets. Zweite Gitquelle kein Teilklon.

## Erhaltener Stand

Eigener Worktree `/home/nathanael/.worktrees/brain-a-profile-20261006`, Branch `fix/brain-a-profile-20261006`, HEAD d6131cc52711a3e8b02d299704244f8d7dbdbce6, sauber, als Mainvorfahr mit Exit 0 geprüft. Eigener Doppelbau vollständig zurückgenommen, keine Commits/Pushes. Scratch-Postgres beendet. Keine Dienste, produktiven Configs, Migrationen oder Hooks verändert. Prüfartefakte unter `.core-test-logs/` im eigenen Worktree vor Cleanup sichern.

## Prüfung

`/home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-a-profile-20261006/rust/Cargo.toml -p brain-maintenance pinned_original_import -- --include-ignored --nocapture`: 1 bestanden, 0 ignoriert, Laufzeit 742,66 Sekunden auf Startbasis. Tatsächlicher Originalimport mit 95 GameTracking-Dokumenten/232.808 Fakten und 17 Deadlock-Data-Dokumenten/78.749 Fakten. Formatprüfung beider betroffener Pakete nach Rücknahme grün. Zurückgenommener eigener Git-Test: 7 bestanden, 1 rot. Kein grüner Clippy- oder vollständiger Pipeline-Nachweis behauptet.

Kein Gate angefordert, da kein eigener verbleibender Code-Diff. Paket A prüft/instaliert den vorhandenen aktuellen Mainstand über den bestehenden Releaseweg, belegt dann regulären Tick, Quittungen, Steckbriefdokumente, Standardaktivierung und drei normale Consumerantworten. Health oder erfolgreicher Originalimport allein erfüllen das nicht.

TESTNACHWEIS[TW-1]: 1 passed, 0 ignored | Baseline: 0 rot

MERGEPROTOKOLL[MS-1]: 8 Git-Schritte einzeln | Anläufe: 0 | Gate: entfällt, kein eigener Code-Diff
