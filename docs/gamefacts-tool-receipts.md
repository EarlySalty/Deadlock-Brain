# Shared gamefacts tool receipts

## Public formatter

All tools consuming `serve.published_receipt` use the same pure implementation:

```rust
brain_contracts::canonical_receipt::format_receipt(
    receipt: &CanonicalReceipt,
) -> Result<Evidence, PortError>
```

`CanonicalReceipt` is public, owned, serializable typed metadata. Populate every field from the authoritative published receipt, not from a tool's partial fact or history row. The fields, in serialization order, are:

1. `raw_receipt_id`: positive signed 64-bit database receipt ID (`p.id`).
2. `raw_object_id`: positive signed 64-bit raw object ID.
3. `ingest_run_id`: positive signed 64-bit ingest run ID.
4. `source`: exact source string.
5. `source_revision`: exact observed source revision.
6. `path`: exact raw object path.
7. `locator_kind`: exact locator kind.
8. `locator`: exact locator string, including an empty raw-bytes locator.
9. `value_sha256`: exactly 64 lowercase hexadecimal characters, obtained from the raw receipt digest, for example with `encode(p.value_sha256, 'hex')`.
10. `extractor`: exact extractor identifier.
11. `patch`: `None`, serialized as JSON `null`.

No caller-dependent defaults, normalized source strings, hero/stat/history values, fact conditions, scores or official patch claims may enter this metadata. A published receipt describes an observed source revision. It does not establish an official patch. The formatter rejects a non-null patch.

## Exact content and hash contract, version 1

`Evidence.content` is `serde_json::to_string(&CanonicalReceipt)`: compact UTF-8 JSON with the declared struct field order above, no whitespace outside string values and no trailing newline. Integers are decimal JSON numbers. String escaping is serde_json's compact serializer. Optional patch is always present as `null`. Object-map ordering and a caller's original JSON whitespace are irrelevant because the typed struct is serialized again.

The SHA-256 input is the following exact concatenation:

```text
UTF-8("brain.gamefacts.receipt.v1") || [0x00] || UTF-8(Evidence.content)
```

The ID is the 28-byte ASCII prefix `gamefacts.receipt.v1:sha256:` followed by the 64 lowercase hexadecimal digest characters. The resulting ID is 92 bytes, below the 512-byte tool ID limit. No database identity is replaced or rekeyed. A change to any canonical metadata field changes the content address. A future serialization change requires a new version and hash domain.

Golden fixture:

```json
{"raw_receipt_id":42,"raw_object_id":7,"ingest_run_id":3,"source":"fixture","source_revision":"r1","path":"hero.json","locator_kind":"json_pointer","locator":"/name","value_sha256":"abababababababababababababababababababababababababababababababab","extractor":"fixture-v1","patch":null}
```

Expected ID:

```text
gamefacts.receipt.v1:sha256:60501a49e7739dbdc631ecccc54668862c87e1eaf0872a0ff9d1f1e1bbb0e239
```

## Complete Evidence identity

Every caller receives identical fields for the same receipt:

| Field | Value |
| --- | --- |
| `evidence_id` | Versioned content ID above |
| `citation` | Same content ID |
| `source_id` | `canon.source/` plus exact source |
| `logical_id` | `receipt/` plus decimal raw receipt ID |
| `revision` | Ingest run ID as unsigned integer |
| `kind` | `Fact` |
| `content` | Exact canonical JSON |
| `visibility` | `Public` |
| `allowed_scopes` | Empty set |
| `score` | `1.0` |
| `provenance` | `None` |
| `patch` | `None` |

Do not decorate any of these fields per tool. The kernel's full-Evidence conflict guard remains active. Sharing a numeric database receipt ID alone is insufficient; tools must share the full formatter output.

## Tool results, bounds and revalidation

Numeric raw receipt IDs remain internal SQL lookup keys. `find_entity` and `get_hero` replace their output row `citation` strings with the actual formatter IDs. `ToolResult.evidence_ids` and dependency Evidence use those same IDs. Tool-specific factual payload stays in the bounded `ToolResult`, never in receipt Evidence.

The executor counts distinct raw receipts first, trimming removable rows until at most 100 remain. It reads their full published metadata within the existing repeatable-read, read-only tool transaction, verifies source revisions, formats Evidence, replaces citations and then measures the serialized result with real hash IDs. It reserves the worst serialized 512-byte call ID. Only results strictly smaller than 16,384 bytes are bound. Further trimming repeats measurement using actual content IDs; identity and name receipts are retained. An untrimmable oversized result fails closed.

`ToolEvidenceDependency::with_canonical_result` binds the final bounded result, including its remapped citations and evidence IDs. Revalidation reruns the same request and deterministic bounding path. Call IDs are excluded by the existing result-binding contract. Changes to receipt metadata, factual result values or truncation metadata invalidate the dependency even if raw receipt IDs remain unchanged.

## Numeric answer traces and cache hits

Use the shared metadata resolver:

```rust
CanonicalReceipt::from_evidence(evidence: &Evidence) -> Result<CanonicalReceipt, PortError>
```

It parses typed content and reproduces the complete Evidence, rejecting noncanonical content, forged IDs and altered Evidence fields. Trace capture resolves successful tool-result membership through dependency Evidence and records `raw_receipt_id.to_string()` in the unchanged `ops.answer_trace.citation_ids` JSON array. It never parses content-addressed IDs as integers.

For a warm-cache answer with no executed tools, the trace wrapper resolves returned answer citations through the same method. IDs are deduplicated. No new schema, migration, token store or ID mapping table is needed.

## Verification boundary

Pure golden tests cover deterministic bytes, full Evidence equality, every authoritative field and rejected noncanonical metadata. The scratch PostgreSQL integration executes `find_entity`, `get_hero` and an independent adapter that calls the public formatter on the same published receipt through the actual shared kernel loop. Its four-turn adapter fixture explicitly proves rejection at the original 32,000 input-token budget, then runs the expanded conversation with a separate 128,000-token test context. Production provider budgets and configuration are unchanged. It also checks numeric cold/warm-cache traces, output bounds, dependency revalidation and the existing read-only role isolation. This is local verification with a scripted provider, not a production deployment or live AI claim.
