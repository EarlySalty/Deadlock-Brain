status: aktiv
Datum: 2026-10-03

# Z: gebundene Teilstände für die gemeinsame Integration

Aufnahme nach 18:41 UTC aus den tatsächlichen Fachübergaben. Quellen noch nicht gemeinsam integriert oder abgenommen. Keine zusätzliche G5-Voraussetzung aus alten Drafts.

## Bereits vorbereiteter Kern

Q-C9: `e48c189` und `5c220a8`, Übergabekopf `5c220a8f047eb980d953d9f9f285b34739b5ed88`. In Z konfliktfrei mit `cherry-pick --no-commit` vorbereitet, noch im Index. Dieser Stand enthält keine späteren Provider-/Audit-/Retentionsänderungen. Q meldet nach eigenem Consumer-Nachlauf elf fehlgeschlagene `brain-serve`-Librarytests rund um `docs_client_grant` und ein Importformatproblem in `brain-api/src/internal.rs`; Q korrigiert im eigenen Stand. Noch kein vollständig geprüfter neuer Consumercommit. Nicht als bereits gelöster oder gemergter Gesamtstand ausgeben.

Zs vier zusätzlicher Adapterdateien sind erhalten und formatiert. Workspace-Lockdatei muss minimal zur vorbereiteten Manifestgruppe passen. Fünf-Dateien-Fortbau ist getrennt beauftragt, echte Compiler-/Testresultate fehlen. Releaseinstaller `b86353a` ist gepusht, aber zentraler Gate BLOCK; frischer Fixer arbeitet im disjunkten Ops-Bereich. Keine Quellen oder Manifeste während lebender gemeinsamer Prüfungen zusätzlich verändern.

## P: neuer geprüfter Brain-Baustein

Commit `1a5b2b33ec9f8c79a073fe67c083c14232289a2b`, Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`. Einzige Produktdatei `rust/crates/brain-feeds/src/bin/brain-patchnotes-ingest.rs`, Datei-SHA256 `d443fe3c76eabac50174cbd4084ab215c2c01ce586ddec149f33b8dd5d87202f`. Laut tatsächlicher P-Übergabe sauber committed, noch nicht in Z aufgenommen. Check und striktes Clippy Exit 0; 16 Binarytests und drei bestehende Patchnotes-Tests bestanden, null failed/ignored, 16 andere Libraryfälle nicht gelaufen.

Vertrag `bereiche/p/BRAIN-KANDIDAT.md`: `stage-candidate --config <absoluter-pfad>`, vorhandener Credential-FD-5-/Infisicalweg, aktuelle Standardbasis und serialisierter Releasehash. Kandidat liefert Hashes, `allowed_changed_sources=["patchnotes-feed"]`, `activation_target="standard"`, `activation_performed=false`. Fremde Pins bleiben erhalten; No-change erzeugt keinen Release. Kein PostgreSQL-Integrationstest oder aktiver Readernachweis. Der Scope im Beispiel ist ausdrücklich ein Platzhalter: vor echtem Writerlauf tatsächlichen freigegebenen Standard-/Patchnotesscope und Rechte binden, keine stillschweigende Egress-/Publikationsfreigabe.

P insgesamt noch aktiv. Runtime meldet einen fehlgeschlagenen Bot-Test und offene Clippyfunde; DevFeed wartet an tatsächlich lebenden Hostlockprozessen. Quellenbausteine mit 84 bestandenen Tests sind kein voller P-Abschluss. Frischer Betriebsbefund: aktiver Pythonpublisher und Live-TOML unverändert, `brain_feed.enabled=false`. `60-global-toml.conf` und `70-release-main.conf` würden eine bloße `20-native.conf` überstimmen. Gemeinsamer Cutover muss die wirksame Reihenfolge und Feed-Aktivierung prüfen und `20-creds.conf` erhalten. P hat keine Units geändert.

## S: noch keine endgültige Freigabe

Brainzwischenstand `148e1a58a485def587f763c76c9ec7a9ff06040f` gepusht, 29 Publishtests und lokale Gateprüfung laut S bestanden. Drei Intentbefunde bleiben offen: gespeicherte CLI-Wiederaufnahme, sichtbares anhaltendes HTTP-429, Fehlerexit bei BLOCKED. Folgefix wartet an lebender Hostlockkette. Diesen Zwischenstand nicht als final freigegeben integrieren.

Steamkopf `9aec0cc897b01b74d417ab9b510314cbbbd02535`, minimaler Lockdiff noch uncommitted. S bindet die tatsächliche fremde Bots-Pfadabhängigkeit und erhält deren Verknüpfung. Z muss den endgültigen Dependencystand gemeinsam erneut prüfen. Keine parallele Releaseumschaltung; Z bleibt Installationsverantwortlicher.

## K: zwei geprüfte Consumer, Rest offen

Docs `3e570a8aa0bf867bf1baf35b064165804b77fcb4`: 22 Tests, Fmt/Clippy, frische Intent-Abnahme und lokaler Gate ALLOW. Second-Brain `54979646adde835335fa24ddd2545e10f996df52`: 16 Tests, Fmt/Clippy, frische Intent-Abnahme. Beide als on-demand Rust-CLIs für gemeinsame Prüfung aufnehmen, keine neuen Antwortdaemons. Eigene K-Worktrees unverändert, keine Z-Integration oder Installation ausgeführt.

Twitch `ffa037c885ca45c0b7a43759ed6714305ed92f99` hat 82 Tests und Formatbeleg, aber vollständiges Clippy noch nicht grün. Bots eigener geprüfter Ziel-SHA fehlt. Betriebsvertrag `bereiche/k/BETRIEBSVERTRAG-CLI.md` bindet normale TOMLs, vorhandenes LoadCredential-/FD-5-Bootstrap und benannte Infisicalsecrets; dauerhafte Consumerinstallationsroots und tatsächlich geprüfte Releasebindung vor Ausführung durch Z bestätigen. Keine Rootpfade erfinden oder aus vorhandenen fremden Artefakten ableiten.

## Gemeinsame Reihenfolge und Stop-Bedingungen

1. Erhaltene lokale Writer und Prüfungen zu einem wirklichen Abschluss führen. Keine Duplicate-Worker oder Nachrichtenresumes in problematische Workflowkontexte.
2. Danach geprüfte Teil-SHAs und enge Lockänderungen im eigenen Integrationsbaum aufnehmen. Tatsächliche externe Dependency-SHAs, Rechte, Release-/Serialisierungshashes und unveränderte Fremdpins erneut binden.
3. Genau den gemeinsamen SHA unabhängig gegen Nutzerintent abnehmen; Archiv vor produktiver Mutation zusätzlich konkret PostgreSQL-seitig abnehmen. Zentraler Merge-Gate ist alleiniger Bug-/Securityreviewer.
4. Nur nach ALLOW in main integrieren und gemeinsam SHA-geprüft installieren. Frischer Legacy-Abgleich erhält Zwischenzeitdaten und alte Generationen; keine neueren Kernrevisionen überschreiben. Fachliche Live-Beweise erst anschließend.
5. G5/G6, Wiki-Frist und voller natürlicher fehlerfreier Timer-Tageszyklus bleiben getrennte echte Nachweise. Cleanup ausschließlich bei gesicherter ancestry und Artefaktsicherheit; fremde Arbeit erhalten.
