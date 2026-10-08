# G: Produktgate und Fixrunden

status: beide alten S2-Restkerne numerisch geprüft; voller S2-Gate auf b6c11533 hat zwei neue bestätigte Funde, 07.10.2026

## S2-Fortsetzung b6c11533: echter voller Gate-BLOCK

Commit b6c1153363f817ff1056a5fd28de13eec00cc58d mit Stack-Shred, gemeinsamer Itemnormalisierung und kompatibler kontrollierbarer RequestDeadline. Der gesamte frühere S3/S4-WIP bleibt unstaged erhalten. Numerische Combatprüfung auf committed Stand: 53 passed/0 failed/0 ignored/259 filtered. Deadline-Teilmenge: 6 passed/0 failed/0 ignored/37 filtered. Committed Compiler, striktes Clippy und Formatcheck Exit 0. Vollsuite Reasoner 300 passed/12 failed gegenüber tatsächlich gemessener gleicher Baseline 294 passed/12 failed, identische zwölf Fehlernamen; fehlende DB-/Livevoraussetzungen getrennt. Verträge auf committed Stand 43 Unit-, 23 Integrations- und 11 Erweiterungsfälle bestanden. Kein grüner Gesamtbeweis aus der roten Reasonersuite.

Regulärer voller Gate gegen S1 bd83d7ab, Rohlog G/pruefungen/g-m-s2-zeitbasis-2135/runde-1/s2-gate.log, Exit 1: `[gpt-6.1-sol] BLOCK: The new simulator ignores ability suppression and misapplies resistance to untyped damage.`

1. combat.rs:1360 im committed Stand: aktive Fähigkeitswahl ignoriert scenario.use_abilities. Beide öffentlichen simulate_calculation-Eingänge reichen den unveränderten Helden weiter. Auch bei false können Casts, Fähigkeitsschaden, Channels und Castfolgen entstehen. G bestätigte nach Graphify im aktuellen Arbeitsquelltext, dass use_abilities außerhalb der Testfixture nicht gelesen wird.
2. combat.rs:945 im committed Stand, aktuelle Arbeitsquelle :947-984: Receiver wählt bullet nur für Weapon und spirit für sämtliche übrigen Schadensarten. Untyped erhält dadurch Spiritwiderstände und -verstärkung. G bestätigte den tatsächlichen Fallback bei :972. Untyped darf keine typspezifischen Spiritmodifikatoren erben; vorhandene allgemeine Schutz-/Schadenssemantik erhalten.

Der native Fortsetzungsbericht meldete gate_ran=false und einen separaten abgewiesenen Scriptlesezugriff, widerspricht damit den inzwischen tatsächlich vorhandenen Gatebelegen. G konsumierte die Logs selbst. Der zusätzlich gestartete mechanische Gateaufruf b626tqrhl mit explizit demselben gpt-6.1-sol lieferte Exit 1 und `BLOCK: Both previous blocking defects remain in the reviewed revision.`, STILL an denselben beiden Stellen. Kein anderes Modell und kein zweiter inhaltlicher Freibrief. Danach status und log -1 einzeln geprüft; HEAD weiterhin b6c11533, Index leer, kein Push. Workflow wck1q4g81 regulär gestoppt; keine weitere Instanz des alten Writers.

Die verweigerte Inspektion von cargo-slot wurde nicht wiederholt oder umgangen. Seine tatsächlich erfolgreichen Prüfaufrufe und der reguläre Gate bleiben davon getrennt. Nächste beauftragte Runde ist ein frischer nativer Fixer ausschließlich für diese zwei bestätigten combat.rs-Kerne mit derselben ganzen S2-Gruppe und gleichem Urteilmodell. Keine Funde an den alten Implementierer zurückgeben.

## Historische autorisierte Fortsetzung: kein neuer Gate, tatsächliche Prüfsperre

Genau ein frischer nativer Fixer, Workflow wncl0gm5w / wf_c0bc5cf3-ded, nach direkter Entscheidung tatsächlich gestartet und beendet. Der veröffentlichte FD/flock-Testslotloop wurde einmal vor Ausführung von Worktree-Isolation abgewiesen. Kein Testprozess oder Sourcefix, keine weitere Gateantwort. HEAD unverändert 8feb8b6e, Quellmanifest durch Bereichsführung erneut 12/12 bestätigt. Fehlende numerische Fälle und beide Restkerne bleiben offen. Unveränderter originaler Deny im vorhandenen nativen Journal, exakter Befehl in slot-denied-command.txt; Beleg G-M-S2-PRUEFSPERRE-NACHWEISE.md. Kein Wiederholungs-/Ersatzweg oder Hookeingriff, Eskalation gemäß Entscheidung über AN_HAUPT-G.md.

## S2: sechs Gruppenanläufe, zwei bestätigte Restkerne

Lokaler HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206, fünf frische native Fixer nach erstem S2-Gate. Letzter voller Gate gegen S1 bd83d7ab, Rohlog s2-fix-r6/gate.log gelesen, Exit 1: `[gpt-6.1-sol] BLOCK: stack damage skips shred, and duplicate items replay effects.` Compiler/Clippy/Format auf committed Endstand Exit 0; Logs und Prüfbaum-HEAD durch Bereichsführung bestätigt. Tests nicht ausgeführt. Endmanifest erneut 12/12, leere Indexprüfung Exit 0, origin weiterhin S1, kein S2-Push.

Nach Graphify am Arbeitsquelltext bestätigt: Stackbonus an combat.rs:1927 erhält im Defaultpfad keinen bullet_shred, obwohl der Bulletanteil ihn schon enthält; Receiver ergänzt den fehlenden Wert nicht. simulate_calculation_inner :841 reicht doppelte Item-IDs direkt an alle Interaktionszustände weiter, während evaluate_core_with_deadline :545 sie bereits normalisiert. Enger gemeinsamer Semantikkern, kein Bedarf für neue Pipeline. Empfehlung in G-M-S2-BLOCK-NACHWEISE.md; nach fünf erfolglosen Runden über AN_HAUPT-G.md eskaliert, keine weitere Runde gestartet. S3/S4 nicht begonnen.

NIT bleibt getrennt: Waffen-Druckszenario mit gerastertem Zielwechsel meldet Zeitauflösung 0. Kein BLOCK aus diesem Hinweis abgeleitet.

## Reasoner-S1: finales ALLOW und origin bestätigt

R2 abgeschlossen, ausschließlich data.rs. Finaler SHA bd83d7abdef812a30daa47aa5f6a78de4f42263a. Bereichsführung las Fixdiff und Rohbelege: committed Compiler/striktes Clippy Exit 0; gesamtes S1 gegen F2 regulär Exit 0, `[gpt-6.1-sol] ALLOW: No merge-blocking defect found in the supplied changes.` Eigenes ls-remote bestätigt origin-bd83d7ab, Quellenmanifest erneut 12/12. Fehlende Rohmessungen bleiben Unknown; ungültige Pelletableitung und nichtendliche Quotienten werden verworfen. Drei neue R2-Tests nicht ausgeführt nach einmaliger Slotablehnung. Keine Wiederholung oder direkter Test-Ersatzweg. Bericht G-M-S1-R2-NACHWEISE.md.

Neuer NIT im Gate: negative Burstzyklen in mechanics.rs:388/:435. Bereichsführung hat ihn nicht am Quelltext verifiziert; context-mode blockierte wegen kanonischer Projektwurzel. Keine Prüfung über anderen Werkzeugweg oder Berechtigungsänderung. NIT nicht in BLOCK oder bestätigten Quellfehler umdeuten. S2/S3/S4-Fortsetzung gestartet, noch ohne Ergebnis; autonome frische Fixerschleife bei tatsächlichem BLOCK nach aktueller Regel.

## Reasoner-S1: Boolfix ALLOW, numerischer NIT bestätigt

Frischer G-M-S1-R1-Fixer tatsächlich abgeschlossen; nur data.rs und drei unmittelbare Datentests, primär 3 passed/0 failed/0 ignored/328 filtered. Wegen einer pauschalen Workflow-Autoritätsauslegung kein Workercommit, keine tatsächlich abgewiesene Gitaktion. Bereichsführung sicherte das eng gestagte eigene Delta als `4df1eb5aeeeb36b8fa4f68a320167be11bce68e6` und prüfte dessen committed Stand im eigenen sauberen Hilfsbaum: Compiler und striktes Clippy all-targets einschließlich Abhängigkeiten, jeweils Exit 0. Primärer Worker-Test lief nach Flock-Befehlsablehnung ohne vorgeschalteten Slot; tatsächlicher Lauf belegt, geforderter Slotweg dadurch nicht erfüllt. Keine Hilfsbaumtests oder rückwirkende Slotbehauptung.

Gemeinsamer regulärer S1-Gate F2..4df1eb5a, Task bd0t5yhab, tatsächlich Exit 0: `[gpt-6.1-sol] ALLOW: No confirmed merge-blocking defect in the supplied changes.` Log /tmp/brain-g-s1-fix-gesamt-gate-20261007.log gelesen.

NIT :859: bullets_per_second wird durch ungeprüfte Pelletzahl geteilt; bullets 0 ohne vorrangige Schuss-/Zyklusdaten erzeugt Infinity. Bereichsführung befragte Graphify und bestätigte den direkten Quotienten :904 im Arbeitsbaum sowie öffentliche Ablage im SourcedWeaponModel ohne Konverterguard. Neuer nativer Fixer G-M-S1-R2 eng gestartet. Das Urteil bleibt ALLOW, kein fingierter BLOCK; tatsächlicher numerischer Kern wird vor weiterer Sicherung behoben. Keine neue Formel oder Pellet-Ersatzkonstante, bestehende öffentliche Unknown-Semantik erhalten. S1 noch ungepusht, S2/S3/S4 ungestartet.

## Reasoner-S1: erster Gate BLOCK

Base F2 `2b67796fb80ae3440a0c9e76671dfc8169032844`, Head `e75477280004d86d65d0291ab123616a5e576196`, tatsächlicher Gate Exit 1: `[gpt-6.1-sol] BLOCK: The new item converter can silently clear a disabled flag.` Rohlog G/pruefungen/g-m-checkpoints/S1-gate.log gelesen.

Blocker data.rs:569 im committed S1, Arbeitsquelle :613: disabled null verhindert gültigen IsDisabled-true-Rückfall, weil erst der JSON-Wert gewählt und danach as_bool angewandt wird. Bereichsführung bestätigte nach Graphify die Quelle und den passenden bestehenden Loaderablauf :1461. Frischer Fixer w1eb98gpy / wf_f0f4ee56-92e nur für Konverter und unmittelbare Datentests gestartet; kein Befund an den Checkpointimplementierer. Gemeinsamer S1-Wiederholungsgate gegen F2 erforderlich, nicht bloß Fixdelta.

ID-NIT :1387: merged_payload könnte keine passende ID enthalten. Im tatsächlich gelesenen Arbeitsbaum stammt item_id bereits aus payload.id und bildet den Kataloglookup; merged_payload ist dessen Clone, nur item_card und Shopkategorie werden ergänzt. Kein bestätigter Identitätsfehler aus dem NIT. Fixer kontrolliert denselben Herkunftspfad am committed S1; I-Loader/SQL bleiben unverändert.

F1/F2 regulär ALLOW, je committed Compiler/Clippy Exit 0 und origin-F2 durch Bereichsführung bestätigt. S1 nicht gepusht, S2/S3/S4 ungestartet. Kein Rechenkernvertrag als gesichert ausgegeben.


## Gemeinsamer Provider-/Kernelgate nach R4: ALLOW

Base `a656862996661275092440d4f3bcd11534f0889e`, Head `dbce14aedadd94881a3cb21151d9840994094cd9`, Task `bejusihil`, tatsächlich abgeschlossen mit Exit 0. Tatsächlich gelesene Antwort: `[gpt-6.1-sol] ALLOW: No blocking defect established by the supplied code.` Log `/tmp/brain-g-provider-kernel-gesamt-r4-gate-20261007.log`. Umfasst korrigierten R4-Commit 1cc491a5 und bestehenden Verbraucherfixturefix dbce14ae. Kein Gesamt-G-, Main- oder Live-ALLOW.

NIT des Gates an execution.rs:207: Erstturn ohne Abhängigkeiten überlässt Anfrage-Egress dem Provider. Beide konkreten Provider lehnen fehlende Freigabe ab; für eigene Adapter benötigt der Reviewer den nicht mitgelieferten Autorisierungsvertrag. Kein BLOCK und kein zweiter Reviewer. Im Übergabevertrag an K ausdrücklich festhalten; keine Abschwächung oder zusätzlicher Connector.

R4-Belege: sechs konkrete Provideranschlüsse bestanden, Kernel/Provider/Serve zusammen 181 passed/25 failed/0 ignored, Quellenbindung 423/423 Exit 0. Der enge vorhandene Panel-/Dokufall bestand nach Ergänzung finish_reason stop; Fixture-Delta regulär ALLOW. Details in G-K-R4-NACHWEISE.md und G-VERBRAUCHER-FIXTURE-NACHWEISE.md. Frischer G-M-Checkpointworker `wyiyh90dz` / `wf_180de5c8-fe5` tatsächlich gestartet, exklusives Git-Schreibrecht; Reasoner-Zwischengates noch offen.

## Gemeinsamer Provider-/Kernelgate vor R4: BLOCK

Base `a6568629`, Head `b352472fbe75429a221cefe5a59133f41bd35520`, Task `b32iv233a`, Exit 1. Tatsächliche Antwort: `[gpt-6.1-sol] BLOCK: The tool loop cannot complete with either concrete provider.` Log `/tmp/brain-g-provider-kernel-gesamt-gate-20261007.log`.

> rust/crates/brain-kernel/src/execution.rs:210 | BLOCKING: Accumulated tool evidence is replaced with `&[]`. Twins: both input-ceiling calls at lines 187 and 194. After a tool result contains evidence IDs, authorize_turn rejects the next turn because those IDs are absent from the supplied evidence; final citation validation also uses that empty evidence set. This breaks both concrete providers. All three sites must receive the accumulated evidence. The kernel mock explicitly accepts empty evidence, concealing the integration failure.

Bereichsführung führte status und log -1 einzeln aus und las anschließend den Quellpfad: evidence wird gesammelt, aber an allen drei genannten Stellen durch einen leeren Slice ersetzt. Befund bestätigt. Neuer frischer Fixer G-K-R4 erhält diese drei Stellen und echte konkrete Provideranschlussprüfungen im vorhandenen Serve-Testbereich. Kein neuer Vertrag, Connector oder Providerwechsel. Der bisherige Mock ist kein Beweis der reparierten Integrationsgrenze.

### Zweck- und Buildschutzdelta abgeschlossen

G-K-R2: `3242fb36`, Zweckdelta ALLOW, Kernel 61 passed/18 gleiche failed; vier Zweckfälle bestanden. G-K-R3: `b352472f`, Buildschutzdelta ALLOW, Contracts/Kernel 144 passed/18 gleiche failed, letzter Buildschutzlauf `buildschutz-verified.log` 10 passed/0 failed/0 ignored/79 filtered. Bereichsführung las Gate und Exit 0 sowie bestätigte 423/423 Quellen. Der frühere Lauf `buildschutz-final.log` enthält acht Fälle. Berichte `G/G-K-R2-NACHWEISE.md` und `G/G-K-R3-NACHWEISE.md`.

Buildschutz-NIT bleibt: tatsächlicher F/G-V-Produktionsadapter noch nicht angeschlossen, positive Prüffälle benutzen Kernel-Fakes. Verbraucherlauf 414 passed/14 failed ohne passende Vorher-Baseline. Keine vollständige G-Abnahme und kein Main-/Liveabschluss.

## Gemeinsamer JSON-/Fehlervertrag

Base `5c2afa66`, Head `a6568629`, Task `bb90g3oe3`, Exit 0. Gate: `[gpt-6.1-sol] ALLOW: No merge-blocking defects found in the supplied diff and revision-specific snapshots.` Log `/tmp/brain-g-json-contract-gate-20261007.log`. Derselbe committed Stand wurde im eigenen sauberen Prüfworktree `brain-g-checkpoints-20261007` ohne übrigen WIP für Contracts, Provider, Kernel, Sources und Serve erfolgreich kompiliert, Task `bigvf8ywu`, Exit 0. Zusätzlicher isolierter Testaufruf wurde von der Worktree-Befehlsprüfung vor Ausführung abgewiesen; kein Testlauf daraus behauptet. Vorhandene tatsächliche G-K-R1-Tests bleiben separat dokumentiert. Keine Änderung oder Umgehung der Befehlsprüfung.

## Provider, Runde 1

Base `a6568629`, Head `4f42c209`, Task `bucgy2luf`, Exit 1. Gate `[gpt-6.1-sol] BLOCK: Transient HTTP failures can lose their retries.`

> rust/crates/brain-providers/src/transport.rs:272 | BLOCKING: Error-body reads now abort the retry loop on oversized, truncated, or timed-out bodies. This affects every 429 and 5xx response for both chat providers. A 503 with Content-Length above max_response_bytes immediately returns ResponseTooLarge, even with retries and budget remaining. Keep the bounded read for accounting, but retain the transient-status retry when diagnostic body reading fails.

Bereichsführung prüfte Quellpfad nach Graphify: `read_bounded(...)?` im 429/5xx-Zweig beendet tatsächlich den Retryloop. Berechtigter Befund. Neuer nativer Fixer erhält ausschließlich Providertransport und unmittelbare Tests, kein ursprünglicher Implementierer. Konservative Reservierung und beobachtete Usage dürfen dabei nicht verloren gehen. Keine Änderung an Fristen, Budget, Retrykonfiguration oder Modell.

### Provider-Fixrunde 2 abgeschlossen

Frischer Fixer `wrq0z0aau` / `wf_670d3b4c-132` tatsächlich zurückgegeben. Eigener Commit `1b5ea4527576c1b8b622e86e81067b396889f2b0`, ausschließlich transport.rs und faults.rs. Bereichsführung bestätigte Quellgleichheit gegen Commit, Exit 0, sowie tatsächliche Marker 11 Unit-/33 Faultfälle: 44 passed/0 failed/0 ignored/0 filtered. Format, Compiler, striktes Clippy und Tests Exit 0. Befehle/Exits in `G/pruefungen/g-p-r2/commands.log`, tatsächlicher Gate in gate.log: `[gpt-6.1-sol] ALLOW: No blocking defects found in the supplied diff and revision snapshots.`, Exit 0 auf `45f51d6f..1b5ea452`.

Fehlerkörperlesefehler behalten transienten Status und bestehende Reservierung. Loopback prüft Größenüberschreitung, abgeschnittene Körper und Timeout samt ursprünglicher Frist. Native Providerkonfiguration erlaubt weiterhin einen Versuch; dort ist Statuserhalt, kein Retry-Erfolg belegt. Delta-ALLOW ist keine vollständige Provider-/G-Abnahme. Kein Push oder Main.

## Kernel, Runde 1

Base `4f42c209`, Head `45f51d6f`, Task `bh62kq5ky`. Werkzeug stoppte nach dem fälschlich auf zwei Minuten begrenzten Hintergrundfenster. Log leer, kein Modellurteil. Unveränderter Retry `bgky63ys3` mit ausreichendem Prozessfenster: Exit 1, `[gpt-6.1-sol] BLOCK: The tool path bypasses existing Build and authorization semantics.` Log `/tmp/brain-g-kernel-loop-gate-retry-20261007.log`.

1. lib.rs:289: Nichtleere Toolsession umgeht den bestehenden deterministischen Buildschutz von execution::answer. Ein nichtdomainiger Build kann modellgenerierte Bauempfehlungen mit gewöhnlichen Toolbelegen erhalten, ohne geprüften deterministischen Build.
2. lib.rs:290: Tooldispatch verliert AnswerPurpose. Unbedingte Veröffentlichungsgates an lib.rs:191/327 und execution.rs:233 lehnen InternalRead trotz gültigem Lesen und Providerweitergabe ab. Frische Ausführung, Cachetreffer und Flightfolger betroffen.

Bereichsführung bestätigte den Dispatch ohne purpose und den alten Buildschutz in execution.rs:583. Frischer Kernelfixer übernimmt nach tatsächlicher Provider-Rückgabe, damit gezielte Featurecommits im gemeinsamen Worktree zeitlich getrennt bleiben. Keine Funde an den ursprünglichen Kernelimplementierer. Keine Abschwächung aktueller Quellenrechte; Lesen, Modellweitergabe und Veröffentlichung bleiben getrennt.
