# Paket I: E Gate-Runden

## Runde 21: die eine fachliche URL-Runde tatsächlich ausgeführt

Fixer 15 c0e38302cd85c9c650ca07f658af037cedb81124, Basis 9d17ee52. Bestehende Suite 558 passed/25 ignored, explizite erweiterte Scratch-PG-Probe 1 passed/0 ignored mit 12 positiven und 7 negativen Identitätsfällen. Format, Compiler und striktes Clippy einschließlich brain-serve Exit 0. Bericht FIXER-15-BERICHT.md. Kein Main-Push oder Deploy, keine zweite Discovery-Fixrunde.

Begrenzter Selbst-Gate mit unverändert Claude Opus 5.5, Exit 1; Original /tmp/brain-fixer15-self-gate-opus55.log:

```text
BLOCK: A store URL does not match an existing Steam event URL for the same patch.
1. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:149 | BLOCKING: Reverse Steam alias is missing | When the feed supplies `/news/app/1422450/view/G` but the stored URL is `/app/1422450/event/G`, `identities` contains only the store URL. The lookup misses the existing ID and creates a new one. The reverse case, including query and trailing-slash variants, is covered by the newly added but ignored test.`
```

Nach BLOCK zuerst Status/log bestätigt, danach Graphify und tatsächliche Quelle geprüft. Der Community-Ereignisalias wird in Zeilen 150 bis 154 ausdrücklich ergänzt. Die genannte Richtung wurde in der echten PG-Probe ausdrücklich ausgeführt und bestand. Konkreter Befund somit nicht bestätigt, Gateurteil dennoch BLOCK, kein ALLOW daraus abgeleitet. Der vom Nutzer anschließend vorgeschriebene gemeinsame Gesamt-Gate gegen aktuellen main bleibt ein eigener noch ausstehender Integrationsschritt. Kein unveränderter Retry des begrenzten Selbst-Gates und kein sofortiger Discovery-Schnitt aus einer nicht bestätigten Behauptung.


## Runde 17: Fixer 12 ALLOW für HTML-Token und bestehende Patch-ID

Frischer nativer Fixer 12 liefert `b70dc6b6af35aeef26139993a7b515ed575f81e4`, auf eigenem Featurebranch gepusht. Erhält tatsächliche Originalbindung und Netzgrenzen; beseitigt den berechtigten HTML-Linktextrest und verwendet kanonische URL auch zur bestehenden Post-ID-Zuordnung. Nur die beiden vorhandenen Patchquelldateien geändert. Beide tatsächlichen Regressionen jeweils vor Korrektur 1 failed, Exit 101. Vollständige Cratesuite 152 passed, 0 failed, 3 ignored; explizite echte isolierte Scratch-PG-Post-ID-Probe 1 passed, 0 failed, 0 ignored, 116 filtered. Diese zwei Lauf-Auswahlen getrennt, nicht als ein einziger Gesamtlog ausgegeben. Compiler, Format und striktes Clippy Exit 0.

Begrenzter Gate gegen `aca42a50`, unverändert Claude Opus 5.5, Exit 0; Original `/tmp/brain-e-fixer12-gate-opus55.log`:

```text
ALLOW: The changed URL handling and database lookup show no merge-blocking defect in the reviewed diff.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 2/2 geprüft
```

Dies ist kein Gesamt-ALLOW. Frisch bestätigter origin/main bleibt `ca4d877f13042c9a7a7023e54f6bf2c688b69ac4`. Die Hauptsession übernimmt anschließend beide Originalquellenfixes samt Gegenprobe in den tatsächlichen gemeinsamen Kandidaten und prüft ihn vollständig sowie mit demselben Urteilmodell. Kein Main-Push, Deploy, Liveimport oder analytics_runtime-Übergabe daraus abgeleitet. Bildlink-NIT bleibt offen. Die bisherigen Hauptsession-Auftragsakten blieben während der Fixerarbeit unverändert/ungestaged.

## Runde 16: Fixer 11 BLOCK, frischer Kontext für zwei begrenzte Restkerne

Autorisierte Fortsetzung `ENTSCHEIDUNG-I-PATCH-ORIGINAL.md`, 16:09 UTC. Frischer nativer Fixer 11 liefert `aca42a505ff3b6c427352c4fde5651511f589678`, nur bestehende Patch-/HTML-/URL-Pfade. Patchsuite 42 passed, 0 failed, 0 ignored; vollständige deadlock-brain-Suite 151 passed, 0 failed, 2 ignored. Compiler, Format, striktes Clippy und Diffcheck Exit 0. Begrenzter Gate gegen `16eaecde`, unverändert Claude Opus 5.5, Exit 1; Original `/tmp/brain-e-fixer11-gate-opus55.log`.

```text
BLOCK: Zwei Eingabepfade können fremde Steam-Inhalte oder falsche Patch-IDs erzeugen.
1. rust/crates/deadlock-brain/src/pg_patchnotes.rs:723 | BLOCKING: GID-Erkennung scheitert an HTML-Anhang. Bei Links wie .../externalpost/steam_community_announcements/123\"&gt; und .../announcements/detail/123\"&gt; erkannte der bisherige Code die 123; jetzt liefert er None. Beide neuen GID-Wächter greifen dann nicht, sodass der Titel-/Zeit-Fallback einen fremden Steam-Eintrag wählen kann.
2. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:133 | BLOCKING: Fragmentbereinigung erreicht die Bestandszuordnung nicht. Die Suche verwendet weiter den unveränderten Feed-Link. Existiert die fragmentfreie URL in changelog_posts, aber noch nicht in brain.source_documents, erhält derselbe Patch eine neue negative ID statt der bestehenden ID.
```

**Verifikation des Fixers:** Befund 2 bestätigt. Befund 1 ist in der wörtlichen Variante mit bloß abschließendem \"&gt; durch die vorhandene Normalisierung bereits abgefangen; direkt anschließender Linktext ohne Leerzeichen bleibt jedoch im Token, verhindert GID-Erkennung und öffnet Legacy-Fallback. Diesen berechtigten Restkern mit Soll-Regression beheben, keinen bereits vorhandenen Schutz zweimal bauen. Neuer frischer Fixer 12, keine weitere Bearbeitung im Fixer-11-Kontext. Erste erfolglose Runde dieser autorisierten Fortsetzung; spätestens nach fünf erneut qualifiziert zurückgeben. Integrationskandidat 860793d7 noch unverändert, kein Main-Push oder Deploy.

## Runde 15: einmalig freigegebener gemeinsamer Gate, zwei neue Patch-BLOCKs

Entscheidung `ENTSCHEIDUNG-I-READER-GATE.md`, 07.10.2026, 15:09 UTC, umgesetzt: Gegenprobe `90c17800` in den tatsächlichen kombinierten Kandidaten übernommen. HEAD `860793d7f89d2af9f7510d663e56d592fedf18b2`, Basis `ca4d877f13042c9a7a7023e54f6bf2c688b69ac4`, Tree `9b917e98bbe2ef24c97319991d2d6bdbbf379254`. Gemeinsame Suite 554 passed, 0 failed, 24 ignored, 32 Targets; Format und striktes Clippy einschließlich brain-serve Exit 0. Genau ein gemeinsamer Gate mit unverändert Claude Opus 5.5, Exit 1; Original `/tmp/brain-i-e-evidence-common-gate-opus55.log`.

```text
BLOCK: Der Patchimport kann fremden Inhalt falsch zuordnen und bei einem zulässigen Link den Tageslauf abbrechen.

1. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:372 | BLOCKING: Bei Steam-Seiten ohne passende Ereigniskennung wählt der aufgerufene HTML-Leser ersatzweise irgendein Ereignis nach Titel und Zeit. Enthält die Seite nur ein anderes Ereignis mit Spieländerungen, wird dessen Inhalt unter der URL des angefragten Patches importiert. Kein gleichartiger neuer Auswahlpfad im Diff gefunden.
2. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:124 | BLOCKING: trusted_original_url akzeptiert Fragmente, get_bounded verwirft sie aber vor dem Abruf. Ein Feed-Eintrag mit #… beendet deshalb den gesamten Patchimport und über das Skript auch den Builddatenlauf. Das betrifft beide akzeptierten Quellen, Steam und Forum.
3. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:173 | NIT: Auch ein harmloser Bildlink lässt einen vollständigen Originalbeitrag als Vorschau durchfallen. Eine Original-Volltextprobe mit Bildlink würde den produktiven Umfang klären.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 6/6 geprüft
```

**Verifikation:** Beide neuen BLOCKs tatsächlich reproduziert, nicht bloß Gate-Meinung. Eine kontrollierte HTML-Probe führt fremden Ereignisbody unter der angefragten anderen GID-URL bis zum tatsächlichen PreparedPatch. Für Steam und Forum akzeptiert validate_post Fragmente, während echter HTTP-Core und Resolver vor dem Abruf Err liefern. Zwei diagnostische Zeugen bestanden, 0 failed, 0 ignored; das belegt Fehlverhalten, keine Produktfreigabe. Einzelheiten, genaue Befehle, Grenzen und Originalausgabe in `NACHWEIS-PATCH-GATE-BLOCK.md`. Temporäre Testanbindung vollständig entfernt, Produktcode unverändert; kein falsches Verhalten in der regulären Suite festgeschrieben.

Der frühere Reader-BLOCK ist im neuen Urteil nicht mehr genannt. Kein weiterer gemeinsamer Gate, kein Fixer 11, kein Produktfix, Main-Push oder Deploy. Neuer Kandidat auf eigenem Remote-Branch gesichert. Gemäß Entscheidung Schritt 4 erfolgt qualifizierte Rückgabe mit neuen reproduzierbaren Szenarien. Bildlink-NIT bleibt getrennt offen. Kein zweiter Fallback, kein Core6-Bruch, kein Modellwechsel oder Override.

## Runde 14: Produktintegration BLOCK, genannter Reader-Kern widerlegt

Gemeinsamer Produktkandidat `b63569afbf2bc6f686564063d769dbbcf0a5ef90`, Basis `ca4d877f`, unverändert Claude Opus 5.5, Exit 1. Original `/tmp/brain-i-e-product-integration-gate.log`. Vorgeschaltete Belegintegration `17974c66` und Schemapinintegration `ca4d877f` erhielten separat ALLOW und wurden tatsächlich nach main gepusht. Der vollständige Produktkandidat wurde nicht nach main gepusht und nicht deployt.

```text
BLOCK: Ein späterer Teilimport verdeckt weiterhin vorhandene Assets derselben Clientversion.

1. rust/crates/brain-storage/src/asset_mirror.rs:159 | BLOCKING: Der Leser wählt nur den neuesten passenden Lauf und fällt bei fehlendem Endpoint nicht auf einen älteren vollständigen Lauf zurück. Nach einem vollständigen Import kann ein mehr als 24 Stunden späterer Import nur von items, heroes und heroes_all die globalen Assets derselben Version unlesbar machen. Zwillinge: rust/crates/dbrain-sources/src/assets_api.rs:182 markiert diesen Teilimport als vollständig; rust/crates/brain-storage/src/asset_mirror.rs:90 verlangt ebenfalls nur diese drei Arten.
2. rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:173 | NIT: Ein beliebiger Link lässt auch einen vollständigen Originalbeitrag als Vorschau gelten; Zeile 212 verwirft zudem verlinkte Änderungszeilen. Vorhandene Tests belegen das selbst für einen Bildlink. Eine offizielle Volltextprobe mit harmlosem Link würde produktiven Umfang klären.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 3/3 geprüft
HOOK-NOTE: Twin-Suche für source_document_id übersprungen, mehr als 20 Fundstellen (31, genannte eingeschlossen).
```

**Verifikation:** Der genannte SQL-Kern trifft nicht zu. Der INNER JOIN auf das angefragte Endpoint-Dokument erfolgt bereits vor ORDER BY/LIMIT. Die zusätzliche echte Scratch-PG-Gegenprobe `later_core_mirror_preserves_globals_from_the_actual_older_run` besteht: 1 passed, 0 failed, 0 ignored. Alle 13 Value-/Receipt-Zugriffe funktionieren nach einem späteren vollständigen Core6-Run derselben Version. Globale Receipts nennen weiterhin den älteren tatsächlichen Original-Run. Explizites Pinnen auf den neuen Run lehnt dort nicht vorhandene globale Daten weiterhin ab. Details und Befehle in `NACHWEIS-CORE6-GLOBAL.md`. Keine Produktimplementation geändert, keine zweite Pipeline und kein Vermischen von Run-Belegen.

Dies ist der fünfte weitere erfolglose Fortsetzungs-BLOCK seit Receipt-ALLOW Runde 8 (Runden 9, 10, 11, 12, 14). Deshalb qualifizierte Fachrückgabe statt Fixer 11 oder Modellwechsel. Die Gegenprobe hebt das Gate nicht auf; kein weiteres Urteil angefordert. Core6-Kompatibilität bleibt entsprechend Spec erhalten. Der NIT ist damit weder geprüft noch geschlossen. Der vollständige Kandidat ist auf `origin/feat/brain-i-integration-blocked-20261007` gesichert. Keine analytics_runtime-Freigabe, kein F-Vertragsabschluss und kein Liveabschluss.

## Runde 13: Tatsächliche Schreibziele ALLOW

Frischer Fixer 10 liefert `438b7bfac0eb70a3912e766c26427148e328c9d1`. Bestehender Schutz prüft nun Datenordner, Raw, Cache und `raw/deadlock_assets_api` vor Nebenwirkungen. CLI-Proben 10 passed, 0 failed, 0 ignored. Finale serielle Suite 545 passed, 0 failed, 19 ignored; Format und striktes Clippy Exit 0. Begrenztes Gate gegen `eeb4116c`, unverändert Claude Opus 5.5, Exit 0; Original `/tmp/brain-e-fixer10-gate-opus55.log`.

```text
ALLOW: Die zusätzlichen Pfadprüfungen erfassen die Schreibziele vor Datenbankzugriff und Dateischreibvorgängen; kein blockierender Befund.
WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 1/1 geprüft
```

Dies ist kein Gesamt-ALLOW. Vor Main-Push folgen vollständige aktuelle Integration und gemeinsame Prüfung. Die durch R12 verweigerte DotEnv-CLI-Schreibaktion bleibt ausgelassen, nicht umgangen. Erster serieller Lauf 269 passed, 1 failed, 12 ignored; unveränderter Socket-Einzeltest danach 1 passed, erneuter vollständiger Schlusslauf grün. Keine ungemessene Flake-/Altfehlerbehauptung.

## Runde 12: Effektiver Pfadschutz BLOCK wegen Raw-/Cache-Zielen

Frischer Fixer 9 liefert `eeb4116c0b6fcb99a27da6a532c72cec0d58c99b`; finale serielle Suite 541 passed, 0 failed, 19 ignored; native CLI-Proben 6 passed. Format und striktes Clippy Exit 0. Begrenztes Gate gegen `879e4cc3`, unverändert Claude Opus 5.5, Exit 1; Original `/tmp/brain-e-fixer9-gate-opus55.log`.

```text
BLOCK: Der neue Pfadschutz prüft nicht die tatsächlichen Schreibziele.
1. main.rs:1637 | BLOCKING: Nur data_dir wird auf den Build-Arbeitsbaum geprüft. raw_dir und cache_dir werden nicht geprüft. Ein externes dauerhaftes data_dir mit einem Symlink raw oder cache in den Buildbaum besteht die Prüfung; prepare_dirs folgt diesen Verzeichnissen.
```

Restkern bestätigt. Die Prüfung muss die tatsächlichen Schreibziele einschließlich bestehender Verzeichnisverknüpfungen erfassen, nicht nur den übergeordneten Datenordner. Frischer Fixer 10 schließt diesen einen Lebenszykluskern im bestehenden Weg. Kein Parallelbau oder Modellwechsel. Seit dem Receipt-ALLOW in Runde 8 sind vier weitere BLOCKs angefallen; ein weiterer erfolgloser Fortsetzungs-BLOCK wird qualifiziert an die Hauptakte zurückgegeben.

**ABWEICHUNG:** Reale DotEnv-CLI-Probe vom Fixer 9 durch Schreib-Hook R12 blockiert, nicht umgangen. Keine tatsächliche DotEnv-CLI-Abnahme behauptet. Paralleler Erstlauf 268 passed, 2 failed, 12 ignored (Wiki-Inventarsperren); serieller Schlusslauf tatsächlich grün. Kein ungemessener Altfehlerclaim.

## Runde 11: Lebenszyklusfix BLOCK wegen effektivem relativem Pfad

Frischer Fixer 8 liefert `879e4cc301a0e307c38fa81a1e0bffe943359c0a`; finaler Paketlauf 538 passed, 0 failed, 19 ignored, Format, striktes Clippy und Shellsyntax Exit 0. Begrenztes Gate gegen `4b8db395`, unverändert Claude Opus 5.5, Exit 1; Original `/tmp/brain-e-fixer8-gate-opus55.log`.

```text
BLOCK: Ein relativer Konfigurationspfad umgeht die Pflicht zu dauerhaften Originaldateien.
1. main.rs:1605 | BLOCKING: Nur ein explizites --data-dir wird auf einen absoluten Pfad geprüft. config.rs:89 übernimmt relative ENV-/DotEnv-Pfade; der Timer nutzt diese gemeinsame Auswahl. Bei DEADLOCK_BRAIN_DATA_DIR=relative schreibt der Import Originaldateien in den Release-Arbeitsbaum.
WIRKUNGSPRUEFUNG[WP-1]: 1 Befund | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft
```

Berechtigter Restkern bestätigt: Die Prüfung muss die effektiv gewählte Konfiguration nach einem zulässigen expliziten CLI-Override und vor Schreibzugriffen erfassen. Absolute konfigurierte Pfade und die neue Originalsicherung erhalten. Frischer nativer Fixer 9 behebt ausschließlich diese Pfadprüfung und ihre bestehenden CLI-Proben; keine neue Pipeline oder ENV-Konfiguration. Der Testerstlauf des Fixers 8 war rot (266 passed, 4 failed, 12 ignored); korrigierte Testannahmen und Forumfixture sind nicht als vorbestehende Produktfehler ausgegeben.

## Runde 10: Pfad-Fix BLOCK wegen Wiederverwendung und Konfiguration

Frischer Fixer 7 liefert `4b8db3950f8246aa2612ea07f55c7bb5deb4a2a3`; 140 passed, 0 failed, 2 ignored; Format, Clippy, echte CLI-Pfadabweisung und Timerargumentprobe bestanden. Begrenztes Gate gegen `d4e7ce5f`, Claude Opus 5.5, Exit 1; Original `/tmp/brain-e-fixer7-gate-opus55.log`.

```text
BLOCK: Der neue Speicherpfad sichert vorhandene Originaldateien nicht zuverlässig.
1. main.rs:1615 | BLOCKING: Pfadwechsel ohne Migration. Frische Läufe werden ohne Originaldateiprüfung wiederverwendet; idempotente SourceStore-Dokumente behalten bei gleichem Inhalt ihren bisherigen raw_path.
2. scripts/run_build_data_with_infisical.sh:62 | BLOCKING: Der fest gewählte CLI-/Timerpfad übergeht eine bereits vorhandene DEADLOCK_BRAIN_DATA_DIR-Konfiguration und trennt gemeinsame Datenpfade.
WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft
```

Befund 1 ist am bestehenden SourceStore und Frischezweig bestätigt. Befund 2 hat einen berechtigten Kompatibilitätskern: Dauerhafter Standard darf keine bestehende ausdrücklich konfigurierte Auswahl übergehen. Frischer Fixer 8 repariert beide im vorhandenen gemeinsamen Weg. Originalbytes und Hash prüfen, bei Wiederverwendung nötigenfalls in das dauerhaft gewählte Verzeichnis sichern; Dokumentkennung, fachliche Metadaten und Herkunft nicht umschreiben. Keine Rekonstruktion vermeintlicher Originalbytes aus JSON, keine manuellen DB-Eingriffe oder zweite Pipeline. Bestehende Datenpfadkonfiguration erhalten, keine neue ENV-Konfiguration.

## Runde 9: Gesamtprüfung BLOCK wegen Originalpfad im Timer

Kandidat `d4e7ce5f7b3770062efae18f250f7912dc519347`, Gesamtprüfung gegen `f6f5cef6`, unverändert Claude Opus 5.5, Exit 1. Original `/tmp/brain-i-e-overall-gate.log`.

```text
BLOCK: Der geplante Release-Cleanup kann die gespiegelten Originaldateien löschen.
1. scripts/run_build_data_with_infisical.sh:62 | BLOCKING: Der neue Timer-Aufruf nutzt pull assets ohne --data-dir; auch main.rs:1605 erzwingt keinen dauerhaften Pfad. Ohne DEADLOCK_BRAIN_DATA_DIR schreibt die Standardkonfiguration unter den beim Build festgelegten Repo-Pfad. Nach Release-Worktree-Cleanup verbleiben Dokumente ohne Originaldateien.
WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 3/3 geprüft
```

Berechtigter Kern bestätigt: Der neue explizite Liveimportpfad allein sichert nicht spätere Timerimporte. Frischer nativer Fixer korrigiert den bestehenden Assets-CLI-Standard und Timeraufruf ohne neuen Importweg oder ENV-Konfiguration. Beiläufige Links und bisherige Ledger-Initialisierung sind NITs, kein Anlass zur Ausweitung auf einen neuen Patchpfad. Danach derselbe urteilsgebende Reviewer.

## Runde 8: Receipt-/Global-Assets-Vertrag ALLOW

Begrenzter gemeinsamer Reader-/Writerdiff `802abfba..d4e7ce5f`, Claude Opus 5.5, Exit 0. Original `/tmp/brain-i-e-receipt-gate.log`.

```text
ALLOW: No merge-blocking defect found in the supplied diff.
WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 1/1 geprüft
```

536 bestehende Fälle bestanden, 24 ignored; öffentliche Assets- und echte Scratch-PG-Probe 10 passed, 0 ignored. Format und striktes Clippy bestanden. Noch kein Gesamt-ALLOW oder Main-Push.

## Runde 7: Semikolon-Fix ALLOW

Frischer Fixer 6 liefert `802abfba66c17a5c22116bd33f3e32fee469018d`. Begrenzte Prüfung gegen `a731778547cc3a99753a5429cf823913b979f762`, unverändert Claude Opus 5.5, Exit 0. 39 Patchtests bestanden, 0 fehlgeschlagen, 0 ignoriert; Paketlauf 98 ausgefiltert, Workspacelauf 1641 ausgefiltert. Neue Semikolonprobe vor Fix 1 fehlgeschlagen, nach Fix 1 bestanden. Format und striktes Clippy Exit 0. Original `/tmp/brain-e-fixer6-gate-opus55.log`.

```text
ALLOW: Die Semikolon-Vererbung ist auf die aktuelle Zeile begrenzt, und ein ausdrücklich genannter Gegenstand ersetzt den bisherigen. Kein Merge-Blocker im Diff.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 0/0 geprüft
```

Dies ist das ALLOW des begrenzten Fixdiffs, kein ALLOW der noch zu ergänzenden Receipt-/Global-Assets-Integration oder des gemeinsamen F/G-Stands.

## Runde 6: Gegenstandsbezug nach Semikolon

Fix-SHA `a731778547cc3a99753a5429cf823913b979f762`. 38 gezielte Tests bestanden, 0 fehlgeschlagen, 0 ignoriert, 514 ausgefiltert; fmt und Clippy Exit 0. Sechster Gesamt-BLOCK mit unverändertem Urteilmodell Claude Opus 5.5; `/tmp/brain-e-fixer5-gate-opus55.log`.

```text
BLOCK: Semicolon-separated changes lose their subject.
1. BLOCKING `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:257` | Subject inheritance resets after `;` | In `Holliday: damage increased from 50 to 60; cooldown reduced from 10 to 8`, the cooldown event is imported as a general change. With `; added knockback`, the second change is dropped entirely. Both are the same defect; no other site in this diff resets the subject for a semicolon.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 1/1 geprüft
```

Die autorisierte Fortsetzung steht in `.tasks/2026-10-07-brain-fertigstellung-astra/PAKETE.md`. Frischer nativer Fixer 6 korrigiert ausschließlich die Vererbung über Semikolon innerhalb derselben Vorbereitung und Parserstrecke. Die Hauptsession schließt danach den bestehenden Receipt-/Global-Assets-Vertrag und F-Vertrag, ohne Gs Rechenkern zu duplizieren.

## Runde 5: Entitätsänderungen und falsche Ereignisprojektion

Fix-SHA `e66184cb31f505ca5fe8bcac9511a228194b9210`. 36 gezielte Tests bestanden, 514 ausgefiltert; fmt und Clippy Exit 0. Fünfter BLOCK insgesamt mit Claude Opus 5.5; `/tmp/brain-e-fixer4-gate-opus55.log`.

```text
BLOCK: Gültige Änderungen werden übersprungen; gemischte Zeilen werden falsch übernommen.
1. BLOCKING `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:312` | Bekannte Entitäten zählen nur bei einem reinen Zahlenwechsel, sofern kein Wort aus der Gameplay-Liste vorkommt | `Holliday: added a double jump`, `Scrap Grenade: added knockback` und `Holliday: increased from 1 to 2 charges` werden trotz erkannter Entität abgewiesen. Besteht ein Beitrag nur aus solchen Änderungen, wird er nicht importiert.
2. BLOCKING `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:197` | Die Prüfung trennt Klauseln, die anschließende Übernahme jedoch nicht | Die neu akzeptierten Varianten `Added new artwork and increased weapon damage from 50 to 60` und `Added new artwork. Weapon damage increased from 50 to 60` gelangen jeweils als **ein ungetrenntes Ereignis** in den Parser. Der Eintrag enthält damit auch die kosmetische Änderung, statt nur die erkannte Gameplay-Änderung.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 1/1 geprüft
```

Hauptsession hat den zweiten Kern unabhängig am Code bestätigt: `dbrain-normalize/src/patch.rs:817` prüft `added` vor numerischen Änderungen; der gemischte Text kann also einen falschen Änderungstyp erhalten. `api_sync.rs:68-74` prüft Klauseln, übergibt aber danach unverändert den gesamten Mischtext an `prepare_patch`. Kein bloßer Streit über die Darstellung eines Originalzitats. Originaltext für Herkunft muss erhalten bleiben, Ereignisprojektion muss dieselben bestätigten Gameplayklauseln verwenden wie die Freigabe. Bestehender Parser und vorhandene Vorbereitung/Importstruktur bleiben die gemeinsame Strecke. Nach fünf erfolglosen Gate-Runden erfolgt die qualifizierte Blockermeldung an die Hauptakte; ein nächster frischer Fixer soll nur diesen jetzt lokalisierten Kern und echte gebundene Entitätsänderungen korrigieren.


## Runde 4: Fehlfreigaben durch ungebundene Ausnahmen

Fix-SHA `5d6a76d1aab15e2592b041889d2b07d09873d409`. 34 gezielte Tests bestanden, fmt und Clippy Exit 0. Erneuter BLOCK mit Claude Opus 5.5, Exit 1; `/tmp/brain-e-fixer3-gate-opus55.log`.

```text
BLOCK: Der Filter lässt reine Ankündigungen als Gameplay-Patches durch.
1. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:191` | BLOCKING: Die Prüfung ignoriert Wörter nach „from“; beide neuen Ausnahmen in Zeile 286 umgehen zudem das Gameplay-Merkmal | „Increased from 1 to 2 hero skins“ passiert trotz „skins“. Auch eine Änderung der Besucherzahl passiert wegen des Zahlenwechsels, und „Added Holliday emotes“ passiert bei bekanntem Heldennamen. Solche Zeilen können als vertrauenswürdige Patch-Ereignisse importiert werden.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befund | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 1/1 geprüft
```

Frischer Fixer 4 soll die drei Fehlfreigaben an vollständigem Änderungsgegenstand und echtem Entitäts-/Gameplaykontext begrenzen, ohne die vorherigen positiven Proben erneut zu verwerfen. Bloße Zahlenwechsel und Heldennennungen sind keine ausreichenden Signale. Gesamte kleine Nachprüfung gegen `5d6a76d1` mit unverändertem Urteilmodell.


## Runde 3: Regressionen im Patchfilter

Fix-SHA `8917a5c7a98e9410d32bacc246b45c7fe8bd09d2`. Pflicht-Assets werden nun als nicht leer geprüft, auch im gemeinsamen Leser. 42 gezielte Tests bestanden, 1 bestehende Live-Probe ignored, fmt und Clippy Exit 0. Erneuter BLOCK mit demselben Urteilmodell Claude Opus 5.5, Exit 1, Rohantwort `/tmp/brain-e-fixer2-gate-opus55.log`.

```text
BLOCK: Der neue Patchfilter überspringt echte Gameplay-Änderungen.
1. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:211` | BLOCKING: Die Positivliste erkennt Änderungen ohne eines ihrer Stichwörter nicht | `- Holliday: increased from 1 to 2` ist für den bestehenden Parser ein Änderungsereignis, wird aber sowohl bei der Volltextprüfung (`:179`) als auch bei der Kandidatenprüfung (`:165`) verworfen. Der Patch wird nicht importiert.
2. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:190` | BLOCKING: Ein Kosmetikwort sperrt die gesamte Zeile | `- Increased weapon damage from 50 to 60 and added new artwork` enthält eine Gameplay-Änderung, wird aber an denselben beiden Prüfstellen verworfen.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 2/2 geprüft
```

Frischer Fixer 3 für diese beiden Regressionen. Er soll die vorhandenen Parser-/Ereignisverträge prüfen, statt die neue Positiv-/Sperrwortliste nur um einzelne Beispiele zu erweitern. Bloße Kosmetikmeldungen und Teaser dürfen dadurch nicht erneut durchrutschen. Kleine Nachprüfung gegen `8917a5c7`, unverändertes Urteilmodell.


## Runde 2: weiterer BLOCK nach Originaltext-Fix

Fix-SHA `9b521fdef435de50385d23150409c4776df1e627`. Der erste frische Fixer behebt den Teaser-/Fließtext-Kern; fmt, Clippy und 34 gezielte Tests bestanden. Erneute Prüfung gezielt mit dem urteilsgebenden Claude Opus 5.5, kein Modellwechsel zum Neu-Würfeln.

```text
BLOCK: Zwei Pfade können unvollständige Daten als gültigen Patch oder vollständigen Spiegel übernehmen.

1. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:164` | **BLOCKING:** Kosmetische Meldungen können als Gameplaypatch importiert werden. Die Zwillinge sind die pauschale Freigabe für Forumbeiträge (`:165`), Titel mit „update“ und als Änderung klassifizierter Text (`:166-167`); die Vollständigkeitsprüfung akzeptiert zusätzlich jeden längeren Erzählsatz (`:181`). Ein Forumbeitrag wie „We voted for our favourite bird. See you in the city!“ passiert diese Prüfungen und wird vom bestehenden Parser zum Ereignis. Das verschiebt auch den Beginn des Patch-Zeitfensters.
2. `rust/crates/dbrain-sources/src/assets_api.rs:178` | **BLOCKING:** Ein erfolgreiches, aber leeres `[]` zählt als vollständiger Spiegel. Das gilt für alle sechs Kombinationen aus `items`, `heroes`, `heroes_all` und Englisch, Deutsch: Ihre Schlüssel allein setzen `mirror_complete=true`. Der Leser kann diese Version danach als jüngste vollständige Version ausgeben, obwohl eine Art keine Einträge enthält.
3. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:173` | **NIT:** Auch beiläufige Bild- und Quellenlinks sperren vollständige Originaltexte. Der Lauf überspringt solche Patches sichtbar. Eine Probe mit einem vollständigen Original samt Bildlink würde den Umfang klären.
4. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:43` | **NIT:** Jeder Lauf ruft erneut alle Originalseiten ab; ein nicht erreichbarer historischer Eintrag bricht anschließend auch den Builddatenlauf ab. Eine Prüfung der Feed-Einträge und ihrer Originalseiten würde klären, ob das derzeit ein Betriebsfehler ist.

WIRKUNGSPRUEFUNG[WP-1]: 4 Befunde | Zwillingssuche: keine Fundstelle | Fremddienst-Pfade: 3/3 geprüft
```

Original `/tmp/brain-e-fixer-gate-opus55.log`. Frischer Fixer für die beiden neuen blockierenden Befunde, keine Delegation durch den alten Fixer. Zusatzanweisung vom Orchestrator: kleine Prüfdiffs gegen passende Basis-SHAs, nicht noch einmal über 1 MB. Schema-Pin `bfda408c..5e70da3a` separat geprüft: `[gpt-6.1-sol] ALLOW: this SHA already passed review_gate [reviewer_model=gpt-6.1-sol]`, Exit 0. Dieser Cachebeleg hebt den späteren inhaltlichen BLOCK ausdrücklich nicht auf.

## Runde 1

Kandidat `8107417036228c707582c76e9ffccdeb1b45cb90`, Basis `f6f5cef65f1f946113f0b8216c6475f6d38ec928`, Exit 1. GPT-Weg ohne Urteil wegen Eingabelimit; Rückfallmodell Claude Opus 5.5 urteilt BLOCK. Folgerunden müssen dessen Urteilskette beibehalten, kein Neu-Würfeln eines BLOCK.

```text
[claude-opus-5-5 (nach Ausfall von gpt-6.1-sol: review_gate: codex failed: Error: turn/start: turn/start failed: Input exceeds the maximum length of 1048576 characters. (code -32602), data: {"input_error_code":"input_too_large","max_chars":1048576,"actual_chars":1098088})] BLOCK: Der Patchabgleich kann vollständige Ereignisse durch Vorschauen ersetzen und Fließtext-Patches auslassen.

1. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:189` | **BLOCKING:** Eine einzelne Änderungszeile ohne eingebetteten Link gilt als Volltext. Dieselbe Annahme steht in `has_complete_patch_content` (157–167) und im Schlussguard (54); die Originalprüfung bei 221 verwendet ebenfalls nur diesen Maßstab. | Ein Feed-Teaser mit einer Änderung passiert alle Guards, obwohl `post.link` auf längere Originalnotizen zeigt. Der Import entfernt dann bestehende Ereignisse derselben Patchkennung vor dem Einfügen.
2. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:150` | **BLOCKING:** Patches ohne erkannte Bulletzeile fallen aus der Kandidatenauswahl; bei Forumseinträgen verwirft derselbe Bulletzwang sie spätestens an 157–167 und 54. | Der vorhandene Parser verarbeitet nachweislich Fließtext wie „Six New Heroes“, der neue Feedweg importiert ihn nicht.
3. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:160` | **NIT:** Auch ein vollständiger Originaltext mit beiläufigem Link wird verworfen. | Eine Regressionprobe mit vollständigen Notizen und einem Bildlink würde diese bewusst konservative Grenze festhalten.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 3/3 geprüft
```

Frischer nativer Fixer übernimmt die Verifikation der Befunde, den begrenzten Rust-Fix und die erneute Selbstprüfung. Keine Änderungen an main oder Deploy durch den Fixer. Originalbeleg: `/tmp/brain-i-e-gate.log`.
