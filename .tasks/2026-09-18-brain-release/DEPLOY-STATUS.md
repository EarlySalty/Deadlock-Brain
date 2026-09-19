# Release-Abschluss: tatsächlicher Stand vom 18.09.2026

## Code und technische Prüfung

Code-Commit: e48d58c40cd76603cb79e526766b7da417172a27. Branch: codex/brain-deploy-completion-20260918. Pull Request: EarlySalty/Deadlock-Brain #6, enthält die vorherigen Stände aus #5 und #4.

Der Code ist implementiert, committet und gepusht. 115 unterschiedliche Tests wurden erfolgreich ausgeführt; genaue Zählung und ausdrücklich nicht ausgeführte Tests stehen in ABSCHLUSS-TESTS.md. GitHub Actions des Code-Commits: Run 35374401535; rust, postgres, caption-integration und der neue yt-contract-Job sind SUCCESS. GitGuardian ebenfalls SUCCESS. Die vier Actions-Jobs wurden live über gh überprüft, nicht nur aus einer alten Übergabe übernommen.

Releasebau aus dem eigenen Worktree erfolgreich, Exit 0, --locked, SQLX_OFFLINE=true, maximal zwei Jobs. Die bestehende Meta.hero_ref-Warnung bleibt dokumentiert. Die folgenden drei Artefakte wurden NICHT produktiv installiert:

| Binary in target-rel/release | SHA-256 |
|---|---|
| deadlock-brain | 75f25b94d661a1041809474f2e52ef9e165152b9c19a552abfc8c50af0324308 |
| deadlock-brain-yt | e9cfbf49662e0eb9500e334f1399c235b60c7172d01c0c61a768d59c5b4a1612 |
| deadlock-brain-patch-review | 16f24059f733a0fd593b2d33873c1d745afaf2db1ffd4492453412f645c6716c |

## Warum kein Main-Merge und kein Live-Deploy erfolgt sind

Das reguläre Merge-Gate wurde unverändert über den vorgesehenen codex_gate_hook.py aufgerufen. Ergebnis: Exit 2, kein Modell der konfigurierten Kette lieferte ein Urteil. Grok 4.6 lief in einen Timeout nach 374 Sekunden; die beiden Claude-Stufen meldeten ihre Sitzungsgrenze; die Astra-Stufe meldete Kontingentende und zusätzlich einen Code-Mode-Host-Konfigurationsfehler. Das ist KEIN inhaltliches negatives Codeurteil, aber auch KEINE Freigabe. Ein erneuter Aufruf wurde anschließend von der vorgeschalteten Werkzeugsicherheitsprüfung vor Ausführung gesperrt. Weder Gate noch Kontingente, Modelle oder Berechtigungen wurden verändert oder umgangen.

Main, produktive Datenbank und laufende Dienste sind unverändert. Keine produktive Evidenzmigration, kein Binary-Austausch, keine Aktivierung des neuen Sync-Wrappers und kein angeblicher Live-Smoke. Die vorhandene YouTube-Automatik wird durch ihren bestehenden Wrapper standardmäßig nicht ausgeführt; sie wurde nicht ungefragt aktiviert.

Der Orchestrator hat den neuen Consumer-Code unabhängig vom Implementierer geprüft; das ersetzt nicht das vorgeschriebene reguläre Merge-Gate.

## Realer Primärdaten-Test, isoliert und ohne Creator-Claims

Eigene Datenbank: brain_completion_reference_20260918. Schema und tatsächlich vorhandene Quellen, Patch-Events, Assets-Snapshots sowie Entity-/Aliasnamen wurden aus dem Betrieb kopiert. Creator-Claims und Transkripte wurden NICHT kopiert. Alle drei Evidenzmigrationen sind auf diesem isolierten Klon erfolgreich angewendet.

Ein zunächst gestarteter vollständiger Backfill wurde als eigener Prozess gezielt beendet, weil für den Referenztest nur Patch 285 erforderlich ist. Danach wurde ausschließlich die Quelltabelle dieses Wegwerfklons auf id=285 begrenzt; andere Produktionsdaten wurden nicht gelöscht oder verändert. Der Test ist damit ausdrücklich ein Referenztest und kein Nachweis eines vollständig abgeschlossenen historischen Backfills.

Der aktuelle offizielle Steam-Inhalt wurde über den vorhandenen identitätsprüfenden refresh-official-Pfad frisch per HTTP abgerufen und NUR im Klon übernommen:

- Intern: patch_285, Titel Minor Update - 09-16-2026.
- Event-ID: 698776157349216434; Announcement-ID: 698776157349216435; App-ID: 1422450. Diese IDs sind nicht austauschbar.
- Quellenveröffentlichung: 2026-09-16T20:16:43Z, nicht automatisch bestätigter Spiel-Rollout.
- Normalisierter Quellen-SHA-256: 3d71db47d0988e821fe06a041f49ac1f308444c9138bb16b1d9aed8eadb18936.
- Raw-Body-SHA-256: 6603a1efd95ca16d802b19adb65e54ccac9fccc4f22e75d4c34bc8f89116a1ae.
- Vorher 123, nach revisionssicherem Sync 122 aktive Ereignisse. Die nicht mehr zur offiziellen Fassung gehörende Abrams-Zeile wird nicht weiter als aktives Ereignis behandelt. Historische Fassungen bleiben erhalten.
- Zweiter Sync-Check: checked=1, unchanged=1, drift=false, writes=false.

Der echte Prepare-Aufruf des Release-Binarys besteht. Der gespeicherte Kontext enthält 122 Ereignisse, 147 detaillierte Entities (47 direkt, 100 belegte Verbindungen) und 1371 weitere Katalogeinträge. Keine Creator-Transkripte, keine gelernten Creator-Insights im Generationskontext.

Kontext-SHA-256: 3a282fdc44f144079f6d459788d03f3a488440c6a1d9565dc43b1ec2d4be717a. Promptversion: autonomous_patch_review_v1. Es ist ein vorbereiteter Kontext, KEIN generierter Brain-Bericht.

## Modelllauf: versucht, nicht erfolgreich

Der Aufruf review --patch patch_285 --generate --write wurde über einen begrenzten systemd-Einmallauf versucht; Datenbankziel blieb durch die letzte explizite Env-Zuweisung der isolierte Klon. Der erste Lauf über die bestehende Bots-Umgebung scheiterte vor einem gespeicherten Ergebnis. Ein weiterer Versuch über den bestehenden Rust-Loader dl-infisical-env meldete fehlendes INFISICAL_SERVICE_TOKEN. Die Einbindung über systemd LoadCredential lieferte in dieser Einmalumgebung kein nutzbares CREDENTIALS_DIRECTORY; anschließend meldete Infisical 403. Das belegt ein nicht funktionierendes Bootstrap in dieser Ausführungsumgebung, NICHT die Ungültigkeit eines tatsächlich korrekt geladenen Produktionstokens.

Keine Secrets wurden ausgegeben, kopiert oder in Dokumentation eingecheckt. Kein neuer Provider, kein alternatives Modell, keine Veröffentlichung. Es gibt keinen gespeicherten erfolgreichen Review-Run und keine fachliche Abnahme. U1 bleibt offen, obwohl der Quellen-/Kontextteil jetzt real geprüft ist.

## Historienprüfung und Grenzen

Die neue Rust-Historien-CLI liefert im isolierten Klon zu air drag echte Quellenzeilen mit Veröffentlichungs-, Beobachtungs- und Löschstatus sowie sichtbarer Trefferbegrenzung. Ein Test mit exaktem Entity-Filter Soul Hex und Cooldown ergab keine Treffer; daraus wird nicht behauptet, diese Änderung existiere nicht. Die fachliche Entity-/Aliasabdeckung ist nicht vollständig abgenommen. Der produktive MCP-Prozess wurde nicht auf den neuen Stand umgestellt. Die sechs MCP-Tests bestehen; eine neue produktive Assistentensitzung mit dieser Funktion ist noch kein erbrachter Nachweis.

## Noch auszuführender Abschlussweg

1. Regulären Reviewer-/Gate-Betrieb wieder urteilsfähig machen. Inhaltliche Befunde beheben; keine Übersteuerung und kein erzwungener Main-Push.
2. Nach regulärer Freigabe den aktuellen geprüften Stand in main integrieren. Vor Installation origin/main und Artefakt-Code-SHA abgleichen; keine fremden Worktrees überschreiben.
3. Genau die drei versionierten Evidenzmigrationen als bestehender Schema-Owner deadlock anwenden, dann Owner/Rechte/Trigger prüfen. Niemals die Scratch-Fixtures produktiv laden; keine PUBLIC-Rechte ergänzen.
4. Die drei geprüften Binaries atomar mit Rückfallkopien installieren und DEADLOCK_BRAIN_ROOT auf das installierte Repository setzen. Den bestehenden Patch-Sync-Dienst auf den versionierten Wrapper scripts/ops/patchnotes-sync-v2.sh umstellen, ohne seine berechtigten Laufzeitcredentials offenzulegen. Daemon-Reload, tatsächlichen Oneshot starten und Erfolg/ausgeführten Artefaktstand prüfen.
5. Modell-Einmallauf über die funktionierende bestehende Credential-Einbindung betreiben, Bericht unverändert mit Kontext-Hash, Modell und Run-ID speichern. Erst danach inhaltlich prüfen und mit Vergleichsmaterial messen.
6. Neue produktive MCP-Sitzung mit Historienfrage prüfen. Autonome fachliche Verarbeitung, indirekte Mechanikmodelle, Bildauswertung und gerendertes Video nicht als durch diesen technischen Release erledigt melden.

## Belege und Rückfall

Private Hostablage: /home/nathanael/.local/share/deadlock-brain/releases/20260918-evidence. Dort liegen Test-/Gate-/Build-Logs, vorbereiteter Referenzkontext, Schema- und Datensicherung sowie beide bisherigen installierten Brain-/YT-Binaries als .before. Datenbankkopien und Dumps bleiben auf dem Host, nicht im Repository.

Eigene Testdatenbanken: brain_completion_ops_20260918, brain_completion_ci_20260918 und brain_completion_reference_20260918. Fremde Scratch-Datenbanken und fremde Worktrees wurden nicht übernommen. Der eigene Worktree bleibt absichtlich erhalten: Er enthält die geprüften, noch nicht ausgerollten Release-Artefakte und den vollständigen Fortsetzungsstand.
