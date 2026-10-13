# brain-eval

Offline evaluation runner for the Deadlock Brain. It sends every question of a gold file to a brain-serve `/v1/answer` endpoint and scores the answers. It is a command line tool, not a service.

## Gold file format `brain.e1-gold.v1`

File: `gold/e1-gold-v1.json`. Bump `version` (and the file name for incompatible changes) when cases change; reports record the SHA-256 of the gold file they ran against.

```json
{
  "schema": "brain.e1-gold.v1",
  "version": 1,
  "created": "2026-10-13",
  "sources": { "deadlock_api": ["..."], "patch_posts": ["..."], "docs_base": "...", "fetched": "2026-10-13" },
  "cases": [
    {
      "id": "item-001",
      "category": "item",
      "lang": "de",
      "question": "Was kostet Monster Rounds?",
      "profile": "fact",
      "expect": "answered",
      "facts": [
        {
          "any_of": ["800"],
          "source": {
            "kind": "deadlock_api",
            "url": "https://api.deadlock-api.com/v1/assets/items?language=english",
            "field": "upgrade_non_player_bonus.cost",
            "value": "800"
          }
        }
      ],
      "note": "optional"
    }
  ]
}
```

- `category`: `hero`, `item`, `timer`, `patch`, `docs`, `guide`, `unanswerable`.
- `lang`: `de` or `en`.
- `profile`: the brain `AnswerProfile` sent with the query (`fact`, `explain`, `build`, `coaching`).
- `expect`: `answered` (needs at least one fact) or `insufficient_evidence` (must have no facts). Every `unanswerable` case expects `insufficient_evidence`. Timer cases without a verifiable source (Soul Urn first spawn) also expect `insufficient_evidence` and say why in `note`.
- `facts`: all facts must match for a case to count as correct. A fact matches when any term in `any_of` occurs in the answer text.
- `source.kind`: `deadlock_api` (asset endpoint plus field path), `patch_post` (Steam announcement URL, `field` is the post title, `value` the quoted line) or `docs` (live docs page URL with section anchor, `value` the quoted terms).

Every fact was checked against the source snapshot fetched on 2026-10-13 (deadlock-api assets, Steam news API for app 1422450, the deployed docs pages under `https://deutsche-deadlock-community.de/docs/`). The questions are written from these public sources and contain no user data.

### Term matching

Case-insensitive substring match with token boundaries: an alphanumeric term edge must not touch another letter or digit, so `800` does not match `1800` and `12` does not match `12.5`. A decimal comma between digits counts as a dot. Integer terms with four or more digits also match with `.`, `,` or space thousands separators. Soft hyphens are ignored and typographic apostrophes fold to `'`.

## Running

```sh
cargo-slot build -p brain-eval
brain-eval --gold rust/crates/brain-eval/gold/e1-gold-v1.json --validate-only
BRAIN_EVAL_TOKEN=... brain-eval --gold ... --endpoint http://127.0.0.1:8788 --scope bot.public --out report.json
```

The bearer token is read from the environment variable named by `--token-env` (default `BRAIN_EVAL_TOKEN`) and is never written to the report or the output. `--limit N` runs only the first N cases.

## Report `brain.e1-eval-report.v1`

Per case: status or transport error, `outcome_ok` (status equals `expect`), `correct` (answerable: answered and all facts matched; non-answerable: insufficient_evidence), per-fact match, citation count, knowledge release, latency and answer text. Fact matches are computed on every answer text, including `insufficient_evidence` answers, but only count towards `correct` when the status is `answered`.

Summary rates:

- `answered_rate`: answerable cases answered / answerable cases.
- `correctness_rate`: answerable cases answered with all facts / answerable cases.
- `answered_precision`: correct / answered.
- `citation_rate`: answered with at least one citation / answered.
- `unanswerable_compliance`: non-answerable cases returning insufficient_evidence / non-answerable cases.

Breakdowns by category, language and status follow. A short text summary is printed to stdout.
