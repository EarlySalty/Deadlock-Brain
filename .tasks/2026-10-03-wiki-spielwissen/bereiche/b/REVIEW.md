status: aktiv
Datum: 2026-10-03

WIRKUNGSPRUEFUNG[WP-1]: 4 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

# Lokale Prüfrunde 1

Frischer unabhängiger Prüfer: a09a2667313e865f4, GPT 6.1 Sol high. Lesende Prüfung, keine Compiler-/Test-/Netzwerk-/Gitaufrufe. Ergebnis: Fix erforderlich, keine lokale Freigabe und kein Merge-Gate. Die Dateien änderten sich während der Prüfung; deshalb kein abschließendes Urteil über einen unveränderten SHA.

1. P1: Eingabegrenze begrenzt Fakten-/Pfadverstärkung nicht. Ein langer KV-Elternschlüssel wird für viele kleine Blätter vervielfacht und kann aus einer kleinen Eingabe sehr große Speicher-/Ausgabeallokationen erzeugen. Zwillinge: KV, KV3, JSON-Flattening und VPK-Pfadkonstruktion. Budget vor jeder verstärkenden Allokation prüfen; Rohtext mit explizitem Budgetstatus erhalten.
2. P2: Lose GameTracking-Pfade mit `pak01_dir/` und echte VPK-Ressourcen bekommen unterschiedliche Dokument-IDs für dieselbe virtuelle Ressource. Präsentationslocator vom kanonischen Ressourcenpfad trennen, nur belegte Containerlayout-Mappings anwenden.
3. P2: JSON mit doppeltem Schlüssel, etwa `{"damage":12,"damage":13}`, verliert im serde_json::Value das erste Vorkommen. Rohtext bleibt erhalten, aber Faktenstatus meldet gewöhnliches JSON. Doppelte Eigenschaften erkennen und occurrences erhalten oder ausdrücklich als unstrukturierten Rohtext bewahren.
4. P2: Nach Prüfung ausgetauschte Verzeichnisvorfahren können trotz leaf-inode-Prüfung aus dem Root führen. Der statische Symlinkschutz genügt nicht gegen Vorfahren-Races. Traversierung an gehaltene Root-/Directory-Handles binden. Race-Reproduktion wurde vom Prüfer nicht ausgeführt; als strukturell belegte Race-Schwäche nachzuziehen.

Zusätzlicher unbestätigter Prüfpunkt: numerische Genauigkeit für Integer über u64 und lange Dezimallexeme, unter anderem abilities.vdata Zeile 143695. Nicht ungeprüft als bestätigten Befund behandeln. Quelllexem erhalten und ungenaue Darstellung explizit vermeiden.

Unauffällig geprüft: vollständiger Rohtextfallback, KV-Bedingungen und Mehrfachschlüssel, striktes UTF-16, getrennte Hashes, VPK-CRC/Bereiche, statische Pfadabwehr, unbekannte Einheiten, keine Spiellogik aus Assetnamen, redistribution_allowed=false, getrennte Quellenrevisionen.

Letzte beobachtete Hashes 2026-10-03T03:45:13Z:

```text
game_files.rs      df5da817ced8d2ca72ad180a3b6ab006fe2063c2928fc8a341a7984bb48858b9
game_files/kv.rs   214b6aeb9d29a2f7bbd323ddb9038940be28e9a240cd73661c4a9e95f58302d7
game_files/kv3.rs  69ea42610795082833b2b18d6bd465b0a776ea2dc0b7e3e8c5954b0f0e188475
game_files/vpk.rs  8f683e12a1854dce59cafe7444ff3f3b11c4f4b60019becf60836fc16a38ed5d
```

Frischer Fixer erhält nur diese Liste, Vertrag, Datenroots und den aktuellen Worktreestand. Keine Funde an den ursprünglichen Bau-Worker. C behält Gesamtintegration und abschließenden Gate.

## Unabhängige Nachprüfung nach Fixrunde 1

`NACHPRUEFUNG-1.md`: Die vier ursprünglichen Fundfamilien und genaue Zahlenlexeme sind auf Codeebene nachgezogen. Ein neuer verifizierter P2/F6 bleibt: `vpk.rs:190-195` reserviert Payload-Kapazität aus der deklarierten Länge vor Prüfung des tatsächlichen Bereichs und ohne Budget. Ein strukturell vollständiges 51-Byte-VPK mit 4-GiB-Payloadlänge und entsprechend hohem Optionslimit führt dadurch zu einem nahezu 4-GiB-Allokationsversuch vor Bounds-Ablehnung. Eingebettete und Geschwisterarchive teilen denselben Fehlerpfad. Belegt ist der Allokationsversuch im Code, kein gemessener RSS oder Crash. Grenzen zuerst prüfen, passende Budgets und recoverable Allokation vorsehen.

Der Baum änderte sich während der Nachprüfung wegen des tatsächlichen E0583-Compilefixes für Kindmodulpfade. Deshalb keine Freigabe für den vollständigen Baum. Der VPK-Hash blieb unverändert und bindet den neuen Fund. Dieser Fund geht ausschließlich an einen neuen frischen Fixer; Fixer 1 beendet nur seinen vorhandenen sequenziellen Compilercheck und gibt danach das Eigentum frei. Keine Funde an den vorherigen Implementierer zurückspielen, keine zweite parallele Prüfung.
