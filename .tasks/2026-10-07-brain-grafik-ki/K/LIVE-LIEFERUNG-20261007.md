# K: reguläre Brain-Installation abgeschlossen, laufender Prozess noch alt

status: installiert, Neustart blockiert, 7. Oktober 2026

## Quelle und tatsächlicher Releaseweg

Sauberer eigener Quellworktree /home/nathanael/.worktrees/brain-k-live-20261007, feat/brain-k-live-20261007, Source 0ee3e521def14f79d724a71bea7a90a18438c884, Gitbaum 609c25310da7978ace426c44af843acfb558b5e5. Remote-main vor regulärem Plan/Bau geprüft. Privates Bundle /home/nathanael/.local/state/brain-k-live-release-0ee3e521-20261007, Manifestformat 2. Fingerprint abff6d745e58850c2b34080f893369c260a899cec2f05b9079438d600bce689f.

Unveränderter /usr/local/libexec/brain-release plan, build und privilegierter install verwendet. Build b30bylc59 tatsächlicher Exit 0. install bp9jobnsc einschließlich neuem Verifikationsbau tatsächlicher Exit 0. Kein zweiter Build während laufender Installation und keine Quelländerung bis Abschluss. cargo/rustc 1.99.0, build --locked --release --jobs 4 --workspace --bins. Der Nutzer bestätigt diesen Helfer ausdrücklich als regelmäßigen erlaubten Releaseweg; cargo-slot steuert die Agentenprüfungen. Kein Helferumbau, keine manuelle Zeigeränderung.

## Unabhängiger Installationsnachweis

current zeigt /opt/deadlock-brain/releases/0ee3e521def14f79d724a71bea7a90a18438c884, dort bin-Unterverzeichnis. maintenance-current zeigt /opt/deadlock-brain/maintenance-releases/0ee3e521def14f79d724a71bea7a90a18438c884, Binaries direkt in der Wurzel. Beide Manifeste bytegleich zum gebauten Bundle. Beide Layouts unabhängig jeweils 17/17 SHA256 korrekt, 0 Abweichungen.

Relevante installierte Hashes:

- brain-serve: 1037443b287fa8556afe574178ef929d9c5c68d7ed193a168e481a591f323c05
- deadlock-brain-site: f9cb2474b0814e5dbefa648c9cb03aae9bf45c4368c3b7fea9b51e0948f18365
- brain-maintain: 6faac63b2aebe9c09639746d4485ae0dfb42ef0c294d07e86a6b88c7d6ce4b21

## Laufender Prozess und echte Grenze

brain-serve.service unverändert PID 2388861, active/running, NRestarts 0. Tatsächliche exe /opt/deadlock-brain/maintenance-releases/bfda408cb988722ddceadb56bca5b72e12d12731/brain-serve ohne deleted, tatsächlicher SHA256 c96ee6262a7c4b7c1d3832cea085ff162d0a5695ca14296746b76bc159b271ac. Das ist ausdrücklich der alte laufende Prozess, kein Livebeweis der neuen Installation.

Bestehender bot-restart-Hilfeaufruf bestätigt: brain-serve ist nicht in der Whitelist. brain-site benennt deadlock-brain-site, nicht diesen Dienst. brain-release startet laut Vertrag keine Dienste. Kein generischer sudo/systemctl-Ersatz, keine Whitelist-/Rechteänderung. Neustart erfordert den regulär freigegebenen Dienstweg.

Bestehender alter Prozess: http://127.0.0.1:8788/healthz HTTP 200, application/json, status ok. /readyz HTTP 200, application/json, status ready, release_id maintenance-5819bf58f1b7aebf5dfce8533c182efb1e15770f59a08c213745a71b7098b92f. Das belegt vorhandene Erreichbarkeit, keine neue Antwortfunktion. Keine Anfrage mit privaten Originalen gesendet.

LIVEBEWEIS[DV-1]: PID 2388861->2388861 | exe ohne (deleted), alter bfda408c-Prozess | neues Prozessjournal nicht behauptet | Anker nicht als neuer Laufzeitbeweis geprüft | Funktion: 17/17 Artefakte in beiden Layouts installiert, neuer Prozess/Antwortbeweis offen | Ort: http://127.0.0.1:8788/readyz

## Prüf- und Folgestand

Frischer contracts/providers-Lauf auf der installierten Quelle über cargo-slot +1.97.1: 79 passed, 0 failed, 0 ignored, Exit 0. Test-Gate erkennt diesen vorhandenen Befehlsweg nach belegter Regex-/Transcriptprüfung nicht. Hookpflege außerhalb K, keine Umgehung. Bots-Quotenprozess technisch geliefert, echte Nutzerprobe separat offen. Ortsvertrag jetzt exklusiv freigegeben; derselbe native Implementierer nach tatsächlichem Installationsabschluss zum Schreiben fortgesetzt. Das neue Ortsdelta ist noch nicht Teil der hier belegten Artefakte.

TESTNACHWEIS[TW-1]: 79 passed, 0 ignored | Baseline: nicht behauptet
MERGEPROTOKOLL[MS-1]: 0 Git-Schritte für neuen Source-Merge | Anläufe: 0 | Gate: installierter Source bereits auf main, kein neuer Source-Diff
