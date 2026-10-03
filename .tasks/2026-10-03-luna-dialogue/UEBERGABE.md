# Anschluss des zentralen Textdialogs

Der eigene Worktree `/home/nathanael/.worktrees/brain-luna-dialogue-20261003` beginnt auf `619dcb06d621f812699460427fa6d57fae056bb5`. Der unveränderte FIFO-Prüfstand läuft weiterhin in seinem getrennten Worktree. Dieser Dialogentwurf hat noch keine Compiler-, Test- oder Gatefreigabe.

Der Scratch-Abhängigkeitscommit `e7061c8` enthält ausschließlich den unveränderten `TextDialogue`-, `DialogueVisibility`- und `AnswerProviderPort::dialogue`-Vertrag aus Guide `849926b2`. Dieser Vertrag gehört dem Guide-Integrator. C9 übernimmt später ausschließlich den separaten Eigencommit der Providerhunks und bindet ihn an den bereits gemeinsam integrierten Guide-Vertrag. Kein Guide-Runtime-, Policy-, Serve- oder Mainhunk wird von uns übernommen.

Der Eigenumfang betrifft `brain-providers/src/lib.rs` und `codex.rs`. `TextProvider` delegiert über denselben Port an seine vorhandene Variante. `CodexSubscriptionProvider` verarbeitet öffentlichen Systemtext, strukturierte Daten und vorhandene Evidenz in einem einzigen Aufruf des bestehenden Runners. Query, Domain und Evidence bleiben unverändert; der bestehende Autorisierungscheck erhält die ACL- und Egressprüfung.

`PrivateDm` wird vor jedem Prozessstart mit `PermissionDenied` abgelehnt. Die tatsächliche CLI-Isolation, Persistenz und leere Werkzeugregistrierung sind weiterhin nicht abgenommen. Es gibt keinen zweiten Client, keinen API- oder Modellfallback und keinen gesonderten Umformaufruf.

Vor dem Aufruf gelten Deadline, Netzwerkrounds und Nullbudgets. Das Eingabebudget zählt den vollständigen serialisierten Systemtext, Daten, Grounding und die vorhandenen CLI-Instruktionen einschließlich Ausgabevorgabe. Nach dem Aufruf gelten Deadline und gemeldete Input-/Outputusage. Die CLI hat keinen belegten harten Tokendeckel; diese Prüfungen garantieren keinen maximalen bereits verbrauchten Abo-Anteil eines gestarteten Calls. Die Antwort verwendet dieselbe Evidenzprüfung und Usageabbildung wie der bestehende Antwortpfad.

Die vorbereiteten synthetischen Regressionen prüfen die private Ablehnung durch beide Providertypen, große System- und Datenfelder sowie Nullbudgets und abgelaufene Deadline vor einem Runneraufruf. Sie sind bisher nicht ausgeführt. Das vorhandene CLI-Binary wird dabei nicht benötigt.

Die spätere eigenständig abgestimmte Prüfung umfasst `brain-providers`-Tests und Clippy mit `-D warnings`, den nötigen Serve-Interfacecheck im gemeinsam integrierten Vertragsstand und das bestehende Gate gegen den Abhängigkeitshead. Ein gemeinsames Integrationsgate muss zusätzlich die Guide- und C9-Hunks enthalten. Der freigegebene FIFO-Lauf bekommt keine weiteren Dialogcompiler.
