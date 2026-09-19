#!/usr/bin/env bash
# Versionierter Patch-Sync-Betriebsweg v2 (2026-09-18).
#
# Ersetzt den fragilen MAX(id)-Vorcheck des alten Wrappers durch einen echten
# Inhalts-/Eventbasis-Abgleich: `deadlock-brain pg sync-patchnotes` erkennt neue
# UND geaenderte Quellen (gleiche id mit geaendertem raw_content, sowie fehlende
# oder gemischte Eventbasis) und importiert sie revisionssicher. Unveraenderte
# Quellen loesen keinen teuren Neulauf aus. Neue Quellen liefert weiterhin der
# vorhandene Sammler; dieser Wrapper baut keinen zweiten Sammler.
#
# Erwartet DEADLOCK_CENTRAL_DSN (setzt der Dienst per Infisical) und
# DEADLOCK_BRAIN_ROOT (installiertes Repo). Keine Secrets in dieser Datei.
# Voraussetzung: die Evidenzmigrationen 2026-09-18-patch-evidence*.sql sind
# angewendet (der Capture-Trigger sichert alte Fassungen als Revision).
set -euo pipefail

ROOT="${DEADLOCK_BRAIN_ROOT:?DEADLOCK_BRAIN_ROOT muss gesetzt sein}"
BRAIN_BIN="${DEADLOCK_BRAIN_BIN:-$ROOT/rust/target/release/deadlock-brain}"
: "${DEADLOCK_CENTRAL_DSN:?DEADLOCK_CENTRAL_DSN muss gesetzt sein (Infisical)}"

if [[ ! -x "$BRAIN_BIN" ]]; then
  echo "Brain-Binary nicht ausfuehrbar: $BRAIN_BIN" >&2
  exit 1
fi

# 1. Neue Quellen wie bisher ueber den vorhandenen Sammler in changelog_posts holen.
#    (Der Sammlerschritt bleibt der bestehende; hier nur delegiert, kein Neubau.)
"$BRAIN_BIN" pull patchnotes

# 2. Inhalts-/Eventbasis-Drift rein lesend pruefen. Kein Drift -> kein teurer Lauf.
#    Fehler muessen sichtbar sein: eine fehlgeschlagene oder formwidrige Pruefung
#    darf niemals als "kein Drift" (frischer Erfolg) durchgehen.
if ! check_json="$("$BRAIN_BIN" pg sync-patchnotes --dsn-env DEADLOCK_CENTRAL_DSN)"; then
  echo "patchnotes-sync-v2: Drift-Pruefung fehlgeschlagen; Abbruch." >&2
  exit 1
fi
echo "patchnotes-sync-v2: $check_json"
# Antwortform validieren, sonst hart abbrechen (kein stiller Weiterlauf).
if ! printf '%s' "$check_json" | jq -e '.mode=="check" and (.drift|type=="boolean")' >/dev/null; then
  echo "patchnotes-sync-v2: unerwartete Antwortform der Drift-Pruefung; Abbruch." >&2
  exit 1
fi
drift="$(printf '%s' "$check_json" | jq -r '.drift')"

if [[ "$drift" != "true" ]]; then
  echo "patchnotes-sync-v2: keine Drift, kein Reimport."
  exit 0
fi

# 3. Neue/geaenderte Quellen revisionssicher importieren (prune + reinsert; alte
#    Fassung bleibt als Revision, betroffene Reviews werden needs_revalidation).
"$BRAIN_BIN" pg sync-patchnotes --dsn-env DEADLOCK_CENTRAL_DSN --apply

echo "patchnotes-sync-v2: fertig."
