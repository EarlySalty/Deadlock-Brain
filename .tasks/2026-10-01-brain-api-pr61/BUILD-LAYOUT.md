status: aktiv
Datum: 2026-10-01

# Isoliertes Brain/Bots-Buildlayout

## Pin-Manifest

| Quelle | Commit | Basis | Verwendung |
|---|---|---|---|
| Brain PR #61 Integrator | `08c0c2e0368cd322deb359b37b5c1625a516cddf` | Integrationsbasis `25c6ed6951370b092f60c67a35bdbe37440ece5d`; `origin/main` bei `084cdfc80d48f6f1659fc764955d7f941485e6bf` | Aktueller Brain-Arbeitsstand nach neun einzelnen Steam-Ledger-Cherry-Picks. Der gemeinsame Freeze ist noch nicht erstellt. |
| Deadlock-Bots PR #450 | `a4ebd88ecaf5533a9166579db18498ccdb88dd66` | `main` | Separater Owner-Leaf, kein gemeinsamer Bots-Freeze. |
| Deadlock-Bots PR #451 | `68589a1ba676a9ce329539a6a4e884aee0677419` | `main` | Enthält den bestehenden `dl-ai`-Resolver. `dl-ai/Cargo.toml` Blob-SHA `8b17d4103c86dcc8339bf80dc01c33d71fb28ee3`. |
| Deadlock-Bots PR #459 | `46edc1023a819ba0ba3c37b7de96c333f485f797` | `main` | Separater C9-Consumer-Leaf, kein gemeinsamer Bots-Freeze. |

## Verzeichnisregeln

- Brain und Bots werden in getrennten, isolierten Worktree-Verzeichnissen ausgecheckt. Kein geteilter Checkout und keine dynamische Auflösung auf `main`.
- Jeder Buildlauf referenziert Commit-SHAs aus diesem Manifest. Die Bots-PR-Heads oben sind getrennte Prüfquellen, nicht ein fingierter gemeinsamer Bots-Stand.
- Vor dem gemeinsamen Freeze werden weder die Arbeitsverzeichnisse vereinigt noch eine relative Pfadabhängigkeit erfunden. Der finale Bots-Pin wird ergänzt, sobald der Integrationscommit existiert.
- Kein Credential-Crate wird ersetzt oder per Symlink ergänzt. `dl-token-secrets/Cargo.toml` war in den geprüften Bots-#450-, #451-, #459- und Main-Heads nicht vorhanden; der aktuelle Brain-Stand enthält keine direkte Manifestreferenz darauf. Falls eine spätere Owner-Änderung diesen Pfad benötigt, muss ihr exakter Quellcommit zuerst nachgewiesen werden.

## Geprüfte Integrationsgrenzen

- Brain `brain-serve` konstruiert derzeit selbst `OpenAiCompatibleProvider`; es bindet `dl-ai` nicht. Der genehmigte Fireworks/DeepSeek-Flash-Resolver in `dl-ai` ist daher ein vorhandener, aber noch nicht verdrahteter Providerpfad.
- Brain erwartet den Steam Reserve/Observe-Vertrag unter `127.0.0.1:8901`; ein passender Steam-Bot-Produktivhandler ist noch nicht belegt. Der Discord-Event-Vertrag unter `127.0.0.1:8783/events/discord` ist keine Ersatzschnittstelle.
- Cargo-Tests und Builds bleiben aus, bis der gemeinsame Quellstand feststeht und der Host-Cargo-Lock frei ist. Bis dahin sind nur Rustfmt- und Diff-Prüfungen vermerkt.
