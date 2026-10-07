status: aktiv
Datum: 2026-10-03

# On-demand-CLI-Betriebsvertrag für Z

Quelle: abgeschlossener lesender Workflow wf_c8707c3b-870, Werkzeug-ID w4ljy00qs. Keine Installation, Kompilierung, Secrets oder Anfrage in diesem Auftrag. Alle folgenden Aufrufe sind für die Zeit nach gemeinsamer Integration, unabhängiger Abnahme und Gate bestimmt.

## Feste Consumerverträge

| Consumer | Binary | Normale TOML | Transport und Secretname |
| --- | --- | --- | --- |
| Docs | deadlock-docs-brain-adapter | /home/nathanael/.config/deadlock-docs/bot.toml | http://127.0.0.1:8788, nach Installation durch Z bestätigen; BRAIN_SERVE_DOCS_PUBLIC_TOKEN |
| Second-Brain | deadlock-internal-brain-adapter | /home/nathanael/.config/second-brain/bot.toml | /home/nathanael/.local/state/deadlock-brain/operator/brain.sock; BRAIN_SERVE_SECOND_BRAIN_TOKEN |

Keine neuen Antwortdaemons. Docs verwendet ausschließlich /v1/answer und /v1/retrieve, second_brain.internal ausschließlich den privaten /v1/operator/query ohne Modellanbieter.

Dedizierte TOMLs ohne weitere Botfelder, da beide Loader unbekannte Felder zurückweisen:

```toml
[brain.docs]
endpoint = "http://127.0.0.1:8788"
timeout_ms = 5000

[brain.docs.infisical]
project_id = "<von Z bestätigte Projekt-ID>"
environment = "prod"
secret_path = "/"
socket_path = "/run/uplink-infisical/api.sock"
credential_fd = 5
token_secret = "BRAIN_SERVE_DOCS_PUBLIC_TOKEN"
```

```toml
[brain.second_brain]
internal_socket = "/home/nathanael/.local/state/deadlock-brain/operator/brain.sock"
timeout_ms = 5000

[brain.second_brain.infisical]
project_id = "<von Z bestätigte Projekt-ID>"
environment = "prod"
secret_path = "/"
socket_path = "/run/uplink-infisical/api.sock"
credential_fd = 5
token_secret = "BRAIN_SERVE_SECOND_BRAIN_TOKEN"
```

Keine Credentialwerte oder credential_file in diesen FD5-Konfigurationen. Die Docs-Beispiel-ID aus Nullen ist kein produktiver Projektwert. Bestehende nicht geheime Kernmetadaten nennen 2f5df3ca-12e5-4ae1-abf2-ca7fbd841705, prod und /; Z bestätigt die tatsächliche Bindung je Consumer. TOMLs: reguläre Dateien, vorgesehen UID 1000 und 0600, private Elternverzeichnisse 0700.

Servergrants: docs-client/docs mit docs.public und provider_egress=[public]; second-brain/internal mit second_brain.internal und provider_egress=[]. Keine Authorityfelder im Clientrequest. Der interne Grant ist aus der öffentlichen Registry ausgeschlossen; dessen Release muss exakt internal_operator.release entsprechen. Konkrete Release-IDs und Knowledge-Versionen liefert Q/Z erst nach hashgeprüftem Import.

## SHA-gebundener Bau und Installation

Z legt je Consumer einen absoluten SHA-gebundenen Benutzer-Installationsroot fest. Ein installierter Docs-Zielpfad ist bislang nicht belegt; /home/nathanael/.local/share/second-brain/releases/<COMMIT> ist nur ein dokumentierter Vorschlag. Vorhandene Cargo-Installation genügt, kein generischer privilegierter Dateischreiber.

Vor Ausführung die eigenen Worktrees einzeln frisch fetchen. HEAD und origin/main müssen dem ausdrücklich abgenommenen integrierten Repo-SHA entsprechen, der Baum muss sauber sein. Git-Schritte einzeln mit literalen absoluten Pfaden. Manifestpins bleiben Docs 3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef und Second-Brain 32f603219ad896182693dd85639ac19793272042, soweit nicht gemeinsam eine andere Bindung geprüft wurde.

Beide Sperren während des vollständigen Bauschritts halten:

```bash
exec 8>/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock
flock -x 8
exec 9>/tmp/deadlock-cargo-release.lock
flock -x 9
```

Unmittelbar vor Cargo die vorgeschriebene konservative HOSTPROBE.md-Prüfung ausführen. Bei Exit 75 beide Sperren behalten, 30 Sekunden warten und wiederholen. Nicht durch ein zeitlich begrenztes Wartescript oder vereinfachte Argumentausnahmen ersetzen. Höchstens zwei Jobs. Rustup-Cargo und dessen konkrete Version im Herkunftsbeleg festhalten; tatsächlich erfolgreiche bisherigen Prüfungen liefen mit Cargo/Rust 1.99.0. Die ungeprüfte feste 1.97.1-Auswahl aus dem Rohvorschlag wird nicht übernommen.

Mit von Z bestätigtem neuem INSTALL_ROOT und absolutem WT, jeweils einem der oben genannten BIN-Namen:

```bash
test ! -e "$INSTALL_ROOT"
install -d -m 0700 "$INSTALL_ROOT"
umask 077
/home/nathanael/.cargo/bin/cargo \
  --config 'build.build-dir="/home/nathanael/.cache/rust-build/{workspace-path-hash}"' \
  install --path "$WT/tools/brain-adapter" \
  --bin "$BIN" --root "$INSTALL_ROOT" \
  --locked --offline --jobs 2 \
  >"$INSTALL_ROOT/install.log" 2>&1
```

INSTALL_ROOT, WT und BIN vor diesem Block ausdrücklich validieren und setzen; die Platzhalter sind keine Freigabe zur beliebigen Pfadwahl. Fehlende Offline-Abhängigkeiten stoppen den Lauf. Danach denselben Quellstand und sauberen Baum erneut prüfen, tatsächlichen Repo-SHA, Cargo-/Rust-Version und Cargo.lock-Hash sichern, das installierte Binary in BINARY.sha256 hashen. Fehlgeschlagene Installation als solche erhalten und nicht durch einen alten Cacheinhalt ersetzen. FD9 und FD8 erst nach vollständig beendetem Bauschritt schließen; keine Kinder dürfen sie behalten.

Z übernimmt die echten Hash- und Herkunftsbelege in seine gemeinsame Installationsbindung. Produktionsbinary nie anhand des Datei-Alters wählen. Noch kein Installationsroot, installierter Binaryhash oder produktiver CLI-Release wird hier behauptet.

## Vertrauenswürdiger On-demand-Start

Vor jedem Start den von Z übernommenen installierten Binaryhash prüfen. BIN_PATH muss auf genau dieses Binary, CONFIG_TOML auf die passende normale dedizierte TOML zeigen. REQUEST_JSON enthält nur die harmlose Abnahmeanfrage mit synthetischen Kennungen und keine Credentials.

```bash
sha256sum --check "$INSTALL_ROOT/BINARY.sha256"
systemd-run --user --quiet --pipe --wait --collect \
  --property=Type=exec \
  --property=UMask=0077 \
  --property=LoadCredential=infisical-token:/home/nathanael/.config/infisical-tokens/infisical-token-bots \
  /usr/bin/bash -c \
  'set -e; exec 5<"${CREDENTIALS_DIRECTORY:?}/infisical-token"; exec "$@"' \
  brain-cli "$BIN_PATH" answer "$CONFIG_TOML" <"$REQUEST_JSON"
```

Der Starter nutzt denselben vorhandenen LoadCredential-/FD5-Weg wie brain-serve.service. Keine neue persistente Credentialdatei, kein ENV-Secret oder neuer Systemdienst. FD5 liefert eine private reguläre Datei. Der Adapter lädt ausschließlich sein benanntes Secret über den bestehenden geschützten lokalen Infisical-Transport. Erst der tatsächliche Starterlauf belegt, dass die vorhandenen Rechte und Credentials genügen; ein Deny wird nicht umgangen.

## Fachlicher Antwort- und Journalbeweis

Beide query-Befehle geben die typisierte Antwort als JSON aus; deren Feld .request_id erlaubt die Korrelation automatisch erzeugter IDs. Für die Abnahme vorzugsweise answer mit vorher gewählter einmaliger synthetischer request_id verwenden. Gleiche request_id in Clientrequest, Antwort und gehashtem Kernjournal nachweisen.

Docs: Frage „Wie erstelle und verwalte ich eine Casual-Lane auf dem Discord-Server?“, requested_scopes=[docs.public], profile=explain. Erwarteter auflösbarer Beleg: public/discord-server/workflows/voice-lane-erstellen-verwalten.html. Vorbereiten ohne Netz: deadlock-docs-brain-adapter prepare < Anfrage.json.

Second-Brain: Frage nach der Aufgabe von dl-brain-feeder laut gespeichertem Betriebsgedächtnis, requested_scopes=[second_brain.internal], profile=explain. Vorbereiten ohne Netz: deadlock-internal-brain-adapter prepare /home/nathanael/.config/second-brain/bot.toml < Anfrage.json. Private Antwort und Belege verbleiben lokal.

K korreliert den SHA256 der vollständigen ID ohne Zeilenumbruch mit Qs client-sha256:<Hash>. Journalziel brain_api::redacted_request, event authenticated_request. Passenden Consumer, Route, Status, Ergebnis und zeitlichen Ablauf prüfen. Nur diese ausgewählte redigierte Zeile in den Beleg übernehmen, keine vollständigen Journale, Anfrage-/Antworttexte, Header, Credentials oder Gesprächskennungen ausgeben.

Docs verlangt brain.public.v1, erwarteten knowledge_release, status=answered und fachlich richtige Antwort mit auflösbaren citations. Second-Brain verlangt brain.internal.operator.v1, audience=local_operator, erwarteten Release, status=answered und auflösbare excerpts mit source_id, logical_id und revision, ohne Modellaufruf. Exit 0, HTTP 200 oder insufficient_evidence ersetzen diese Beweise nicht. Das Journalereignis allein beweist keinen Anbieteraufruf.

## Verbleibende Z-Eingaben

Integrierte Repo-SHAs und festgelegte Installationsroots, tatsächliche Binaryhashes, bestätigte TOML-/Projektmetadaten, installierter Endpoint und private Socketbindung, eingerichtete Grants und benannte Infisical-Secrets, echte importierte C9-Release-IDs/Knowledge-Versionen sowie gemeinsame Installationsfreigabe. Erst danach führt K die Anfragen und den fachlichen Live-Nachweis aus.

Quellkontrolle nach Workflow: tatsächliche main.rs-Aufrufe beider eigener Adapter bestätigen prepare/answer/query und unverpackte typisierte JSON-Ausgabe. Keine produktiven Codeänderungen aus dieser Betriebsakte.
