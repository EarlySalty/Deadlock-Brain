status: erledigt, reine Ressourcenzuordnung; keine Start- oder Stopaktion
Datum: 2026-09-30

# Fremder Cleanroom-Releasebuild PID1366750

## Eigentums- und Laufzuordnung

PID1366750 ist Cargo1.97.1 unter UID/GID1000. Parent1366743 ist sh/dash unter Usermanager933. Bestehende User-Unit codex-job-j-1790781891-22802.service, Type=simple, Start30.09./15:24:51UTC, active/running. Cgroup beider Prozesse gehört zu genau dieser Job-Unit. Arbeitsverzeichnis /home/nathanael/.worktrees/brain-independent-cleanroom-20260930-continued/rust.

Sicher ausgewertete Cargo-Flags: build, --workspace, --release, --locked, --offline sowie der Manifestpfad im genannten Cleanroom. Es ist ein Workspace-Releasebuild, nicht unser begrenztes Paket-Clippy. Dieser Prozess wurde nicht von der eigenen G5-Kette mit Intent562a877b oder deren aktuellem Autor-/Reviewerauftrag gestartet. Eine bestimmte fremde Chatsitzung oder Person wurde nicht aus dem Unitnamen abgeleitet und nicht kontaktiert.

Kein --jobs/-j oder --target-dir in den ausgewerteten Cargoargumenten. Daraus keine effektive Konfiguration raten; ENV wurde nicht gelesen. In zwei Stichproben waren jeweils zwei direkte sccache-Kinder vorhanden. jobs1 ist für diesen fremden Lauf daher nicht belegt. Die Kinderargumente wurden ausschließlich auf sichere --out-dir-/--emit-Felder reduziert, keine Rohargumentlisten oder Umgebungswerte ausgegeben.

## Tatsächlicher Targetpfad

Offene Cargo-Dateideskriptoren zeigen .cargo-build-lock, .cargo-lock und .cargo-artifact-lock unter /home/nathanael/.worktrees/brain-independent-cleanroom-20260930-continued/rust/target/release. Die beobachteten sccache-Kinder schreiben laut --out-dir ebenfalls in release/deps beziehungsweise release/build darunter. Zusätzlich besteht ein offener Deskriptor auf /home/nathanael/.cargo/.package-cache-mutate; daraus keinen exklusiven Sperrmodus behaupten.

Realpath und Inodevergleich:

| Target | Gerät | Inode |
|---|---:|---:|
| /home/nathanael/.worktrees/brain-independent-cleanroom-20260930-continued/rust/target |2820716161|5202445|
| /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target |2820716161|5341511|

Beide Realpaths entsprechen ihrem Pfad, damit kein über Symlink identischer Targetcache. Das Cargo-Home bleibt gemeinsam; getrennte Targets sind keine Zusage freier CPU-/RAMressourcen.

## Ressourcenstichprobe und Entscheidung

Hostmeldung:16 Online-CPUs, Loadavg21,79/21,20/20,84. MemTotal48GiB, MemAvailable rund8,2GiB, kein Swap. Die Prozessliste nennt zusätzlich zum einen sichtbaren Cleanroom-Releasebuild einen Steam-Core-all-targets-Check mit jobs2 und zwei weitere Cargo-Testläufe. In der beobachteten Cargo-Liste ist genau ein --release-Lauf enthalten; dies ist keine dauerhafte hostweite Freigabe oder vollständige Erfassung anderer Buildwerkzeuge.

Cargo selbst rund114MiB RSS; der gemeinsam genutzte sccache-Server beziehungsweise dessen Compilerarbeit ist dadurch nicht vollständig dem Cargo-Prozessbaum zurechenbar. Unit-Properties MemoryCurrent/CPUUsageNSec/TasksCurrent sind nicht gesetzt. Keine ungemessene Spitze oder feste RAMobergrenze behaupten.

Eigener G5-Clippy auf b3523fa/8949198 nach Reviewdcfee1d bleibt konkret beantragt, aber nicht gestartet. Höchstens ein Releasebuild, eigener gebundener Targetcache, locked/offline/jobs1 und ausdrückliche zentrale Zuteilung gelten weiter. Kein fremder Prozess beendet, kein Konfigwechsel, keine neue pauschale Genehmigungsfrage.
