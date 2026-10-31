# brain-contracts

## Bot task contract (`brain.bot_tasks.v1`)

The conversation capabilities are an additive extension of v1, not a v2.

Reason: bots that are deployed today only send `discord_answer` and `twitch_title`, and those stay byte-compatible on the wire. A version bump would force a simultaneous cutover of the Brain, the Discord bot and the Twitch bot. With additive v1 the Brain ships first and each bot moves when its package is ready.

Rules for consumers:

- Send `{"kind":"conversation","capability":"<capability>",...}`. Bots send structured input only. Prompts, persona and wording live in the Brain.
- Post `reply.text` only when `reply.kind` is `answered`. `silent` and `failed` mean nothing is posted, and the bot adds no text of its own.
- Links passed as `invite_url` are placed into `reply.text` by the Brain.
- `capability_status` also returns `capabilities`, the state of every capability of the consumer's platform. Call it at start.
- `readable_channel_ids` is allowed only on a `discord_concierge` task from a direct message (`guild_id` absent). It must hold the channels the asking member can read. A non-empty set from a guild channel is rejected, and the Brain never reads a channel outside the set.
