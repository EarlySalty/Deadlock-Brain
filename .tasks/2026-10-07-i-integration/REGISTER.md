# Paket I: Register

status: aktiv, 2026-10-07

Intent-/Auftragsthread: `8827da25-c1f8-44f2-bef8-f3a7b7dd3137`. Delegator: `481426fe-b477-42b3-91c6-901811fcba1d`. Haupt-Orchestrator: `d3a1741e-82bc-4a48-865b-2845c663dca7`. Übergabe ausschließlich als `AN_HAUPT-I.md`, keine Sessionkontakte.

| Bereich | Arbeitsbaum | Branch | bestätigter Stand | Status |
| --- | --- | --- | --- | --- |
| E | `/home/nathanael/.worktrees/brain-e-deadlock-api` | `feat/brain-deadlock-api-daten` | `802abfba66c17a5c22116bd33f3e32fee469018d` | Semikolon-Fix ALLOW; Receipt und globale Assets in Prüfung |
| F | `/home/nathanael/.worktrees/brain-f-publish` | `feat/brain-build-publish-ohne-matchgrenze` | `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8` | übernommen, noch unverändert |
| Releasequelle | `/home/nathanael/.worktrees/brain-i-release-20261007` | detached | `f6f5cef65f1f946113f0b8216c6475f6d38ec928` | regulärer Releaseplan geprüft; kein Build oder Install |

Sechs abgeschlossene native Fixer, keine neuen T3-Threads. Fix-SHAs und wörtliche Gateurteile stehen in `REVIEW.md`. Fixer 6 erhielt ausschließlich den Semikolon-Kern; 39 gezielte Patchtests, Format und Clippy bestanden. ALLOW gilt nur für seinen Fixdiff.

Neuester eigener Fetch bestätigt `origin/main=f6f5cef65f1f946113f0b8216c6475f6d38ec928`. Noch kein eigener Main-Push, Deploy, Import oder Cleanup. Baseline des echten gemeinsamen Lesers: kein vollständiger lokaler Assets-Spiegel. Tatsächlich laufendes Brain-Binary beim Vorcheck: `bfda408cb988722ddceadb56bca5b72e12d12731`.

G wird nicht bearbeitet. Neuester committed Lieferstand `e75477280004d86d65d0291ab123616a5e576196` wurde lesend geprüft: G meldet den öffentlichen Wachstumskern jetzt geliefert, aber noch ohne Produktreview, Main-Merge oder Livebeweis. Übernahme erst nach eigener Prüfung des committed Vertrags; kein fremder WIP.

E-Vertragsergänzung: derselbe SQL-Leser liefert gepaarte Payload-/Run-/Manifest-/Originalhash-Belege und explizite Run-Bindung. Der vorhandene Import umfasst 13 versionierte Endpoint-/Sprachkombinationen. `pull assets --data-dir` hält die Originalbytes außerhalb des löschbaren Release-Arbeitsbaums, ohne neue Pipeline oder ENV-Konfiguration.

Prüfungen: 536 passed, 0 failed, 24 ignored im bestehenden Paketlauf; gesonderte echte öffentliche Assets-/Scratch-PG-Probe 10 passed, 0 failed, 0 ignored; strikter Parser 3 passed. Formatcheck und Clippy bestanden. Eigene neue CLI-Testabhängigkeit und die vorhandene verkürzte Mirror-Scratch-Schemafixture wurden vor diesem grünen Schlusslauf korrigiert. Ausgelassene isolierte DB-/Browserfälle werden nicht als Produktivbeweis gezählt.

TESTNACHWEIS[TW-1]: 536 passed, 24 ignored | Baseline: nicht erneut gemessen, keine Altfehler behauptet
