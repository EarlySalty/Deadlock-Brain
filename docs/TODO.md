# Deadlock-Brain — Offene Punkte (Stand 2026-06-28)

Lebende TODO-Liste. Wissensstand: **11.941 Creator-Claims** (2.458 accepted / 2.875
needs_review / 137 rejected / 6.471 unverified) über 1.045 Entities, plus Ground Truth
(12.523 patch_events, 10.886 entity_snapshots, 10.918 hero_stat_values). Pyramide L0
Patchnotes → L1 Stats → L2 Creator-Wissen steht; `ask-context` liefert vertrauenssortierte
Bündel. Details: [brain-qa-roadmap.md](brain-qa-roadmap.md),
[transcript-claims-pipeline.md](transcript-claims-pipeline.md).

## 1. Wissens-Zufuhr (Transcript→Claims-Kampagne)

- [ ] **19 Monster-VODs (>150k Zeichen) offen.** 8/27 erledigt 2026-06-28 via GLM-5.2.
  Resume: GLM-Pipeline unter `~/.cache/deadlock_brain_campaign/`
  (`driver.py --mode monster --prepare cal_monster_all.json --conc 8`); `prepare`/Ledger
  reselektiert die 19 automatisch. Danach `bundle.py` → `transcript-claims ingest --write`
  → `model`-Relabel auf `glm-5.2`. ~$16 GLM-Budget. Z.ai cappt ~10 concurrent.
- [ ] **~111 caption-lose Videos (`needs_asr`).** Brauchen ASR (kein Transcript vorhanden).
  Empfohlen: **Gemini 2.0 Flash Audio** (~$0.001/min, ~$5 für alle 111) → schreibt
  Transcripts → bestehende Pipeline drüber. NICHT DeepSeek (text-only), NICHT MiniMax
  (Datenfundament-Regel). An 1–2 Videos validieren (Game-Jargon).
- [ ] **4 permanent unholbare Videos** (gelöscht/SME-blockiert) — optional `mark-hard-failed`,
  damit `fetch-transcripts` sie nicht jeden Lauf neu probiert.
- [ ] **visual_tech-Videos** (Movement/Tech, transcript-arm) bleiben bewusst außen vor —
  „hold W and space" ergibt ohne Bild keinen Sinn. Nur relevant, falls später VLM/Bild-Analyse.

## 2. Retrieval & Nutzbarmachung (größter Hebel)

- [ ] **`ask-context` an einen echten Konsumenten verdrahten.** Aktuell reiner CLI-Zubringer —
  niemand ruft es automatisch auf. Das ist die *letzte Meile*, die das ganze Wissen erst
  nutzbar macht: z. B. Discord-`/coach`-Command, Twitch-Chat-Frage, oder Website-Q&A, der
  `ask-context` aufruft und das Bündel an ein antwortendes LLM gibt.
- [ ] **SP-C: Semantische Suche (Embeddings).** Für freie Mechanikfragen ohne Wort-Treffer.
  Retrieval ist sonst „im Kern fertig" (intent/recall hoch) — das ist die verbleibende Lücke.

## 3. Pipeline-Infrastruktur / Daten-Hygiene

- [ ] **`transcript-claims ingest` braucht `--model`-Flag.** Hardcodet aktuell `model='claude'`;
  GLM-Claims mussten per manuellem UPDATE auf `glm-5.2` relabelt werden. Flag = automatisch korrekt.
- [ ] **GLM-Kampagnen-Artefakte in `~/.cache/` sind nicht repo-durable.** `glm_pipeline.py`,
  `driver.py`, `bundle.py`, `split_prepare.py` ggf. nach `scripts/` migrieren + Runbook in `docs/`.
- [x] **Lokale SQLite-Backups aus dem Betriebspfad entfernt.** Runtime nutzt zentrale Postgres;
  Rollback bleibt Backup-/DB-Aufgabe.
- [ ] **Vollautonome In-Rust-Extraktion** (optional): GLM/OpenAI-Client in `core` + Timer, damit die
  Kampagne ohne externen Orchestrator läuft (pay-per-use Key statt Abo-Auth).

## 4. Trust-Engine (optional, User-Opt-in)

- [ ] **L2-Verifikation als wiederkehrendes Rust-Feature** (LLM-Judge mit abgeleiteter Mechanik +
  Patch-Zeitachse). Bisher nur als Workflow gefahren. Lektion: ohne Mechanik-Recompute +
  Patch-Zeitbezug erzeugt der Judge systematische Über-Streng-False-Positives.
