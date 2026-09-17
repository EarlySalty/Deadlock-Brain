# Unabhängige Eingangsprüfung der Familien-Fortsetzung

Stand: 17.09.2026. Chronologischer Vorher-Nachher-Bericht; die abschließende Bewertung steht im Abschnitt **Korrekturen und Endprüfung**. Die anfänglich beschriebenen Account-, Skillorder- und Legacy-Publish-Fehler sind inzwischen korrigiert. Fachliche Holdout- und Live-Daten-Gates bleiben offen.

Zu Beginn wurden `families/mod.rs`, `canonical.rs`, `evidence.rs`, `conditioned.rs`, `holdout.rs` und `tests/regression_tests.rs` parallel bearbeitet. Diese Session ließ den fremden WIP unverändert und ergänzte zunächst nur `examples/family_input_review.rs` und diese Prüfakte. Nachdem der Familienstand als `583ce64` committed und der Worktree bis auf diese beiden eigenen Dateien sauber war, übernahm diese Session gezielt die nachfolgend belegten Skillorder-/Publish-Korrekturen in `meta.rs`, `composer.rs`, `publish.rs` und den Regressionstests sowie den Dreifach-Replaymodus im vorhandenen `family_evaluation`. Keine DB-/Infisical-/Produktionsänderungen.

## Neuer belegter Blocker: Teilnehmer-Leakage zwischen Quellen

`families::is_holdout` hasht `player:<participant>` und `author:<participant>` unterschiedlich. Derselbe numerische Account kann dadurch als Match-Spieler im Training, aber als Build-Autor in der Validierung liegen oder umgekehrt. Die vorhandene Gegenprobe für mehrere Matches desselben Spielers deckt das nicht ab.

Der neue Audit `family_input_review` prüft 1.000 synthetische, identische Account-IDs über beide Quelltypen: **300 abweichende Partitionen**. Der echte Freeze bestätigt Teilnehmerüberschneidungen: Warden 2, Viscous 5, Infernus 4, Abrams 8, Lady Geist 1, Vindicta 10. Diese Zahlen sind hero-bezogene Überschneidungen, keine 30 verschiedenen Personen.

Korrektur: Quelltypen dürfen nicht Bestandteil der Split-Identität desselben Accounts sein. Die Familienbildung und das Scoring bleiben unverändert. Ein eigener Regressionstest muss dieselbe Teilnehmer-ID als `Player` und `Author` vergleichen; alle bisherigen Holdouts anschließend neu auswerten, weil sich die Stichprobenzuordnung bewusst ändert. Das ist keine Anpassung eines Güte-Gates.

## Kataloghypothese nicht bestätigt

Der Audit findet in allen sechs Helden **251 geladene Itemmodelle**, keine in Spielerbeobachtungen fehlende Katalog-ID und keine beobachtete, nicht kaufbare Katalog-ID. Eine frühere Vermutung aus älteren Reports, drei Warden-IDs fehlten im aktuellen Freeze, ist damit nicht belegt und darf nicht als Fehlerursache verwendet werden. Die alten Holdout-Metriken sind unverändert aufzubewahren und mit neuen, hashgebundenen Ausgaben zu vergleichen, nicht nachträglich zu bereinigen.

## Reproduzierbarer Nachweis

Eingabe: `rust/target/families-20260917-input.json`

SHA-256: `f90ed6be72c66ca37bafbd7f04b4e77ed4d26989c4869dcb1265671541217907`

Ausgabe: `rust/target/families-20260917-input-review-astra-01.json`

Befehle aus dem bestehenden Worktree `rust/` mit `/home/nathanael/.cargo/bin/cargo`:

- `test -p dbrain-reasoner --example family_input_review`: Exit 0, drei Tests einschließlich des vorhandenen write_new-Schutztests.
- `run -p dbrain-reasoner --example family_input_review -- target/families-20260917-input.json target/families-20260917-input-review-astra-01.json`: Exit 0, Audit-Artefakt geschrieben. Exit 0 bedeutet erfolgreicher Auditlauf, nicht bestandene fachliche Prüfung; `cross_source_partition_probe.passed` ist false.

Der Audit liest ausschließlich den vorhandenen Freeze, nutzt bestehende Reasoner-Typen/Populationsfunktionen und den vorhandenen Write-New-Helfer. Keine Filterung unbekannter IDs, keine Scoreänderung, keine neuen Provider.

## Weitere beobachtete Prüfstände

Zu Beginn bestanden 244 Reasoner-Library-Tests, 16 DB-Tests waren ignored. Danach kamen parallel vier rote Regressionstests hinzu: gleiche Familien-ID für unterschiedliche Vollsignaturen, widersprüchliche Doppelimporte in Patch- und Holdout-Statistik, erneut eingeführte Duplikate bei Familienkonditionierung (120 statt 12 Beobachtungen). Im Folgelauf: 26 Familientests bestanden, vier fehlgeschlagen. Inzwischen sind dazu parallel Produktänderungen eingetroffen; den Endstand neu testen, nicht den Anfangswert als Freigabe verwenden.

Der Freeze hat Patchbeginn 16.09.2026 22:16:43 Europe/Berlin; die letzten Spieler-Matches reichen nur bis 16.09.2026 04:16:28. Kein Nach-Patch-Match ist damit belegt. Ein neuer Live-Freeze scheiterte vor DB-Zugriff: `Infisical benötigt einen regulären Credential-Dateideskriptor.` Keine Credentials ausgelesen, kein anderer Berechtigungsweg eröffnet, keine Konfiguration geändert. Ein Live-Beleg bleibt offen und muss über den vorgesehenen autorisierten Startweg erfolgen.

## Nachprüfung: Account-Trennung korrigiert

Die parallel integrierte Korrektur wurde mit dem unveränderten Freeze erneut ausgeführt: `target/families-20260917-input-review-astra-02.json`. Der synthetische Test meldet jetzt **0/1.000** abweichende Zuordnungen und alle sechs echten Helden **0 Teilnehmerüberschneidungen**. Der anfängliche Befund oben bleibt als Vorher-Nachher-Nachweis erhalten. Die vier zuvor roten Familientests bestanden im folgenden Lauf; dort insgesamt **30/30 Familientests**. Der Audit-Code bestand `clippy --example family_input_review -- -D warnings`.

## Neuer belegter Blocker: Ungültige Skillquelle verhindert vorhandenen Fallback

Der Audit wurde um die Prüfung derselben Skillquellenauswahl erweitert, die der Composer nutzt: `context.ability_order(hero_id)` plus `progression::coherent_order`. Keine erfundene Reihenfolge, keine Änderung des numerischen Planers.

Artefakt: `target/families-20260917-input-review-astra-skills-01.json`, gleicher Input-SHA wie oben. Bereits vorhandene Ausgabe wurde nicht überschrieben.

Betroffen im echten Freeze: **Abrams, family-3f507e1036329766, Melee / Brawler**. `MetaIndexWithSources::ability_order` wählt eine syntaktisch lesbare Autorenfolge mit 16 Schritten. `coherent_order` verwirft sie bereits bei Buchung 1 und akzeptiert **0 Schritte**. In derselben Familie existieren **11 andere Autorenfolgen mit je 16 kohärenten Schritten**; zusätzlich ist die globale Folge mit **15 kohärenten Schritten** verfügbar. Der familieninterne beobachtete Präfix ist ebenfalls 15 Schritte lang, jedoch nur 31,8 % Acht-Schritt-Support und daher keine gesicherte Familien-Skillorder.

Ursache: Quellenauswahl geschieht vor der Prüfung der Fähigkeit-IDs/Freischaltungen/Upgrade-Kohärenz. Nach Verwerfung der zuerst gewählten Quelle wird nicht mit der nächsten Quelle fortgefahren. Dadurch erscheinen leere Skillorders trotz brauchbarer Daten.

Erforderliche generische Korrektur: Kandidaten vor der endgültigen Auswahl gegen den geladenen Hero validieren; eine vollständig ungültige Quelle mit erklärter Provenienz überspringen und die nächste geeignete Quelle gemäß unveränderter Priorität prüfen. Den vorhandenen Schutz für ungültige Tails behalten, nicht jeden unvollständigen Präfix verwerfen. Kein Abrams-Hardcode. Regressionen für fremde erste Fähigkeit, wiederholte Freischaltung am Anfang und gültigen Alternativ-/Global-Fallback. Die verwarfene Quelle mit Ursache sichtbar lassen. Prüfen, dass Simulation, erklärter Build und Publish-Payload dieselbe gültige Reihenfolge verwenden.

## Neuer belegter Blocker: Legacy-Eingabe kann Publish-Validierung umgehen

Der reine Aufruf `publish::validate_publish_input` akzeptiert aktuell einen synthetischen Build mit `family=None`, leerem Core, leerer Skillorder, `Confidence::Low` und Patch-Tag `unverified`. Die Prüfung von Core/Confidence ist innerhalb des `if let Some(family)`-Blocks eingeschlossen. Der Queue-Aufruf liegt erst nach diesem Validator; diese Probe ruft **ausschließlich den Validator auf**, weder DB noch Queue.

Nachweis im selben unveränderten Freeze: `target/families-20260917-input-review-astra-publication-01.json`; gelesen mit dem neuen reinen `family_input_review summary REPORT`-Modus. `legacy_publication_probe.validator_accepted_empty_unverified_build=true`, `passed=false`, `database_or_queue_called=false`. Account-Überlappung bleibt überall null; die oben beschriebene Abrams-Skillorder-Lücke bleibt in dieser Ausgabe bestätigt.

Erforderliche Korrektur: allgemeine Mindestanforderungen wie nichtleerer Core und ausreichende Daten-/Mechanikabsicherung außerhalb des optionalen Familienblocks prüfen. Fehlende Familien-/Patch-Abnahme darf kein stiller Legacy-Bypass sein. Payload-Serialisierung kann rückwärtskompatibel bleiben; tatsächliche Queue-Freigabe muss unzureichend belegte Eingaben ablehnen. Regression ohne Datenbankverbindung für leere Low-Eingabe ohne Familie und für fehlende Patch-/Familien-Abnahme. Kein Test einer echten Queue-Schreiboperation an Central.

## Neue Holdouts nach dem Account-Fix (nicht grün)

Gelesenes Artefakt: `target/families-20260917-holdout-account-fixed.json`. Warden: 522 zugeordnete Holdout-Spieler-Matches, Staple-Gate falsch; fehlend Veil Walker (865958998), Spirit Lifesteal (876563814), Sprint Boots (3399065363). Kendall 0,8441, Jaccard@12 0,9167. Die Itemidentitäten wurden aus dem eingefrorenen Katalog mit dem vorhandenen `family_evaluation inspect-items`-Modus geprüft: alle vorhanden und kaufbar, keine Kataloglücke.

Viscous Melee: 89 Holdout-Matches, Staple-Gate zwar wahr, aber Stichprobe nicht ausreichend. Spirit: 61 Holdout-Matches, Stichprobe nicht ausreichend, Staple-Gate falsch (Veil Walker, Boundless Spirit, Spirit Snatch). Infernus dominante Familie: 367 Holdout-Matches, Staple-Gate falsch, Ricochet fehlt. Abrams dominante Familie: 168 Holdout-Matches, Staple-Gate falsch, Kendall -0,0667. Lady Geist dominante Familie: 231 Holdout-Matches, Staple-Gate wahr, aber Kendall nur 0,1068. Vindicta: 194 Holdout-Matches, Staple-Gate falsch, Spiritual Overflow und Titanic Magazine fehlen. Diese Gegenbefunde dürfen nicht durch ein grünes Training-Populations-Gate ersetzt werden.

Erneuter kompletter Library-Lauf: **249 bestanden, 0 fehlgeschlagen, 16 DB-Tests ignored** (265 insgesamt). `clippy -p dbrain-reasoner --all-targets -- -D warnings`: Exit 0. Diese Werte beziehen sich auf den Zeitpunkt vor weiterer Korrektur der oben neu belegten Fehler. Drei Release-Replaydateien a/b/c liegen vor; allein deren Existenz ist kein durch diese Session ausgeführter Bytevergleich.

## Korrekturen und Endprüfung

### Skillorder-Fallback umgesetzt

`MetaIndexWithSources::coherent_ability_order(hero)` prüft die bestehenden Kandidaten vor der endgültigen Auswahl. Unbrauchbare erste Quellen werden mit Herkunft und Ablehnungsgrund übersprungen, gültige Präfixe weiterhin bewahrt. Der Rohdatenzugriff `ability_order(hero_id)` bleibt für Provenienzprüfungen erhalten. Gleichgewichtige Folgen desselben Autors werden numerisch nach ihren Schritten statt nach zufälliger Eingabereihenfolge aufgelöst. Sowohl Composer als auch Kaufplan nutzen dieselbe kohärente Auswahl; Originalbeobachtungen bleiben unverändert.

Fünf neue Library-Regressionen prüfen fremde erste Fähigkeiten, Upgrade vor Freischaltung mit globalem Fallback, wiederholte Freischaltung im Tail, vollständig fehlende gültige Quellen und gleichgewichtige Autorenfolgen. Der erste Test vergleicht zusätzlich Plan, Build, Publish-Payload sowie die unveränderten Quelldaten.

Nachweis am unveränderten Input: `target/families-20260917-input-review-fixed-01.json`. Abrams `family-3f507e1036329766` hat jetzt **16 akzeptierte Schritte statt 0**. Zwei ungültige Autorenquellen bleiben mit Ablehnungsgrund sichtbar, anschließend wird eine tatsächlich vorhandene gültige Autorenfolge genutzt. Bei allen sechs Helden ist `empty_despite_usable_fallback` falsch; Account-Überschneidungen und Kataloglücken bleiben null. Der Erstellungsaufruf traf bereits auf diese Datei und überschrieb sie nicht; die folgenden Summary- und direkten Dateiprüfungen bestätigten Inhalt, Eingabe-Hash und Ergebnis.

### Legacy-Publish-Bypass geschlossen

Die Queue-Validierung erfordert jetzt eine belegte Familie. Die bestehende produktive Mindestzahl von 100 Nach-Patch-Matches kann durch eine kleinere lokale Diagnose-Policy nicht abgesenkt werden. Leerer Core, Low-Confidence, fehlende Skillorder und leere Patchkennung sperren weiterhin beziehungsweise zusätzlich vor jedem SQL-Aufruf. Die reine Payload-Serialisierung bleibt rückwärtskompatibel.

Fünf neue Library-Regressionen prüfen die fehlende Familie vor einer Datenbankverbindung, den positiven rein lokalen Validatorfall, den nicht absenkbaren Stichprobenboden, fehlenden Core/Low-Confidence und fehlende Skillorder/Patchkennung. Der unabhängige Audit bestätigt `legacy_publication_probe.passed=true` und `database_or_queue_called=false`.

### Neue Dreifach-Replays des korrigierten Codes

Der vorhandene `family_evaluation` hat nun einen `replay INPUT NEW_DIRECTORY [HERO,HERO]`-Modus: drei neue Auswertungen des bestehenden Planers, kein AI-/DB-Zugriff, Prüfung unveränderter Eingabebytes vor und nach jedem Lauf, exklusives Ausgabeverzeichnis und Vergleich der vollständigen Ergebnisbytes. Keine Rundung, keine Normalisierung und kein Entfernen schlechter Metriken. Vier neue Gegenproben verhindern Freigabe mit weniger als drei Ausgaben, numerischen Abweichungen, leeren oder fehlerhaften Ausgaben. Niedrige Confidence und rote fachliche Werte bleiben unverändert enthalten.

Ausgeführt mit Exit 0:

`cargo run --release -p dbrain-reasoner --example family_evaluation -- replay target/families-20260917-input.json target/families-20260917-replays-after-audit-fixes`

Alle **sechs Pflichthelden**, drei vollständige byteidentische Ausgaben. Gemeinsamer SHA-256:

`d99b8f9f59022a9458251b039581b706f28b1e434cfc55e3e4013633cea5ec9b`

Beweisdatei: `target/families-20260917-replays-after-audit-fixes/verification.json`; Originalausgaben `replay-1.json`, `replay-2.json`, `replay-3.json` im selben Verzeichnis. Der Input-Hash bleibt `f90ed6be72c66ca37bafbd7f04b4e77ed4d26989c4869dcb1265671541217907`. Die alten a/b/c-Dateien sind nicht die Grundlage dieser Endprüfung.

Warden im vollständigen Corpus: genau eine geplante Familie, zehn Core-Käufe, alle zehn Familien- und globalen Staples, Kendall 0,8667 und Jaccard@12 0,8333. Viscous: zwei belegte geplante Familien, Melee/Brawler mit zehn Core-Käufen und Spirit/Ability mit neun. Keine erzwungenen zusätzlichen Gun-/Support-Varianten. Insgesamt geplante Familien im vollständigen Corpus: Warden 1, Viscous 2, Infernus 3, Abrams 4, Lady Geist 3, Vindicta 2. Das ist weiterhin eine historische datengetriebene Analyse, keine aktuelle Patch-Freigabe.

### Neuer Holdout nach den Produktkorrekturen

Ausgeführt mit Exit 0 für alle sechs Pflichthelden:

`cargo run --release -p dbrain-reasoner --example family_evaluation -- holdout target/families-20260917-input.json target/families-20260917-holdout-skill-fallback-fix.json Warden,Viscous,Infernus,Abrams,Lady\ Geist,Vindicta`

Der oben dokumentierte fachliche Gegenbefund bleibt bestehen. Bei der zuvor betroffenen Abrams-Melee-Holdout-Familie verbessert sich Kendall von 0,5556 auf 0,6111; die gültige Skillorder erreicht damit tatsächlich die Kaufplanung. Die anderen berichteten Holdout-Kennwerte bleiben unverändert. Keine Staple-Whitelist, keine Gewichtserhöhung und kein gelockertes Gate wurden verwendet.

### Abschließende technische Checks

- `cargo test -p dbrain-reasoner --all-targets --quiet`: **259 Library-Tests plus 32 Tests der Prüfprogramme bestanden, 0 fehlgeschlagen; 16 DB-Tests ignored**. Insgesamt 291 ausgeführte Tests grün.
- `cargo clippy -p dbrain-reasoner -p dbrain-retrieval --all-targets -- -D warnings`: Exit 0.
- Fokussiertes `cargo fmt --manifest-path target/astra-review-format/Cargo.toml -- --check --config skip_children=true`: Exit 0; nur die sechs hier berührten Rust-Dateien.
- Der bestehende ausdrücklich ausgeführte Read-only-Live-Test `fix_tests::fix_e_live_warden_evidence --ignored --exact` endete dagegen **vor der Verbindung mit Exit 101**, weil `DEADLOCK_CENTRAL_DSN` nicht gesetzt ist. Zusammen mit dem fehlenden Infisical-Credential-FD bleibt dies ein realer Umgebungsblocker; es ist kein bestandener Live-Test.

### Verbleibende Release-Blocker und nächste fachliche Arbeit

1. Aktuelle Spielerbeobachtungen über den vorgesehenen autorisierten Sync-/Read-only-Weg bereitstellen. Der bestehende Freeze enthält weiterhin keine Beobachtung nach dem Patchbeginn; auch zentrale Feldfrische muss danach erneut geprüft werden. Keine Credentials aus Dateien auslesen, keine zentrale DB direkt korrigieren.
2. Die oben konkret aufgeführten Holdout-Lücken und Kaufreihenfolgen fachlich bearbeiten, insbesondere Warden, Ricochet/Mehrziel-Wirkung bei Infernus, Abrams und Vindicta. Ein fehlendes Item nicht blind erzwingen: Familienzuordnung, Support, Mechanikabdeckung und Core-versus-Situational müssen die Abweichung erklären. Viscous braucht ausreichend große unabhängige Familien-Holdouts; das aktuelle Melee-Gate bei 89 Matches ist keine Freigabe.
3. Danach aktuelle Live-Abnahme und unabhängige fachliche Prüfung. Eigene Unit-Tests und byteidentische Replays ersetzen diese nicht.

Die getesteten Änderungen können auf dem bestehenden Featurebranch gesichert werden; die Gesamtfreigabe fehlt. **Kein Main-Merge, kein Deploy, kein Steam-Publish und keine Branch-/Worktree-Löschung**, solange diese Gates offen sind.
