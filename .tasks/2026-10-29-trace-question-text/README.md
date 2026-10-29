# Answer trace with question text

Goal: operators can see what users asked the Brain and why a question was not answered.

## Change

- Migration v51 (`scripts/migrations/2026-10-29-brain-answer-trace-text-v51.sql`) adds to `ops.answer_trace`:
  `source`, `route`, `question`, `history` (jsonb array, max 4), `answer_text`,
  `not_answered_stage`, `not_answered_reason`, `provider_rounds`, plus an index on `recorded_at`.
  `brain_service` keeps INSERT-only access.
- `brain_contracts::answer_trace` carries the request text and the not-answered note on the
  request thread. brain-api sets question and prior user turns, the kernel notes stage and reason in
  `log_not_answered`, brain-serve `TracedKernel` writes them.
- Status is the exact `AnswerStatus` in snake_case instead of the old three buckets.
- Source is `<actor>-<channel>` of the principal, route is the Discord answer purpose or channel.

## Privacy

- Question and history pass through `without_platform_ids` (Discord mentions, channel refs and digit
  runs of 15+ removed). Before storage brain-serve additionally replaces every `@handle` with `@user`,
  every digit run of 6+ with `[id]` and every control character (including NUL) with a space, for
  question, history and answer text alike. Platform IDs and `@` handles are therefore not stored.
- Not covered: a name typed as plain free text without `@` stays in the question, because it is part
  of what the user asked. Digit runs of 6+ that are not IDs (for example dates written as 20261029)
  are also replaced.
- The operator command escapes control characters in every printed field, so stored text cannot
  drive the terminal.
- Retention 90 days: `ops.purge_answer_trace()` (SECURITY DEFINER, fixed bound, EXECUTE only for
  `brain_ingest`) runs inside the daily `brain-job audit-receipts` job
  (`brain-job@audit-receipts.timer`) and prints `answer_trace_purged=N`.

## Operator command

    brain-job --config /etc/deadlock-brain/brain-job.json questions --since 7d --status not_answered [--limit 50]

`--since` accepts `Nd`, `Nh`, `Nm` or `YYYY-MM-DD`; `--status not_answered` means every status other
than `answered`, any other value matches exactly.

## Limits

- dl-bot sends only prior user turns, no assistant turns; history therefore holds user turns only.
- Cached or follower results and the `unverified` status may have no stage/reason.
- `answer_text` is the kernel text before public formatting.
