---
artifact_role: historical_diagnostic_snapshot
snapshot_of: .planning/debug/phase162-descriptor-metadata-roundtrip.md
snapshot_at: 2026-10-11T21:03:56Z
status: diagnosed
trigger: "Phase162 bounded wallet metadata expected #ob:0:0:0 but storage_text emitted #ob::0:0:0"
created: 2026-10-11T21:00:09Z
updated: 2026-10-11T21:02:16Z
---

## Current Focus

hypothesis: Confirmed formatter delimiter duplication; existing durable decoder masks the public roundtrip defect by normalizing the double colon.
test: Source investigation completed; public roundtrip and real caller execution belong to the checked prerequisite executor.
expecting: Formatter-only correction fixes new public storage text while preserving all existing parser and decoder behavior; existing decoder acceptance prevents claiming end-to-end legacy rejection.
next_action: Parent plans minimal formatter correction and public regression proof before rerunning frozen Plan05 bounded preflight; preserve existing decoder normalizer.

## Symptoms

expected: Public bounded import metadata #ob:0:0:0 preserves descriptor body, original receive script, inclusive range and cursor, genuine funding and cold-reload lifecycle.
actual: Parent reports bounded preflight body 1 pass/wallet 1 fail, exit 101, 3.88 seconds, because storage_text emits #ob::0:0:0.
errors: Exact storage string comparison expected #ob:0:0:0 but observed #ob::0:0:0.
reproduction: Existing Phase162 frozen funded-wallet short-prefix preflight imports format!("{}#ob:0:0:0", recipe::WALLET_DESCRIPTOR) through public parser.
started: Newly exposed by legitimate explicit bounded range; previous default-wildcard genuine 102-block public scan/cold reload passed in 93.64 seconds.

## Eliminated

- hypothesis: The legitimate #ob:0:0:0 import uses the wrong grammar.
  evidence: format.rs:274-302 strips ob: and explicitly parses three u32 values; existing internal_format_and_range_helpers_cover_success_paths accepts #ob:1:4:2.
  timestamp: 2026-10-11T21:00:14Z
- hypothesis: The extra colon comes from recipe-body rewriting or consensus/funding behavior.
  evidence: storage_text builds the body independently and appends a format literal whose first separator duplicates the constant terminator; malformed metadata is determined before any scan/funding effects.
  timestamp: 2026-10-11T21:00:14Z
## Evidence

- checked: Active lesson inputs and debug knowledge base.
  found: Canonical global and repository active lessons total 7,188 bytes, estimated 2,397 tokens; both read completely. Existing knowledge-base entry concerns unrelated open-stdin networking timing and has no two-keyword match.
  implication: No known-pattern diagnosis applies. Follow exact-boundary evidence and preserve serial timed Cargo ownership.

- checked: descriptor.rs:18-20 and descriptor/format.rs:152-165, 263-302.
  found: RANGE_METADATA_PREFIX is ob:; storage format {body}#{}:{}:{}:{} adds a second colon. For zero range parser strips ob: from emitted suffix and sees four components, including the leading empty component, triggering DescriptorSyntax invalid descriptor range metadata.
  implication: Defect is formatter-owned. Preserve the constant and strict parser; remove only the formatter's redundant colon after prefix.
- checked: descriptor/format.rs:209-219, 222-247, 309-333 and descriptor/key_sources.rs:276-284, 309-317.
  found: Wildcard defaults are inclusive 0..=1000 with next_index=0; matcher walks inclusive range. Current address uses next_index, and advance rejects only next_index>end before saturating increment. Extended-key storage renders key origin, encoded key, path and wildcard independently of metadata.
  implication: Zero bound preserves derivation zero and script; metadata formatting must preserve start/end/cursor exactly, including exhausted cursor. Existing body canonicalization is unchanged, not universal byte preservation for arbitrary input spellings.
- checked: wallet.rs:288-295, 309-321, 423-425 and Phase162 restrictions.rs:63-94.
  found: Import and address allocation store descriptor.storage_text into original_text; Phase162 asserts body equality, exact storage text, range/cursor zero and original script before registry creation or public scan.
  implication: Immediate bounded failure is independent of wallet scan/maturity/prune behavior. Fixing formatter unblocks the existing acceptance assertion without weakening it.
- checked: node storage/snapshot_codec/wallet.rs:191-221.
  found: DTO stores original_text; decoder already replaces #ob:: with #ob: only for parsing, then retains the unnormalized original_text. Public parser rejects malformed duplicate-colon text directly.
  implication: Prior default-wildcard cold reload is consistent with existing compatibility normalization. A formatter-only correction does not add implicit repair, but cannot make legacy persisted double-colon text fail closed at this decoder. Parent informed of precise boundary conflict.
- checked: descriptor/tests.rs and both existing descriptor child test modules.
  found: Tests check internal range parsing and key-source storage fragments, optional suffix ignoring, defaults, malformed numbers/inverted ranges and exhausted cursor, but never public parse(storage_text(parsed)) for ranged descriptors.
  implication: Add focused public roundtrip regressions to existing modules; no new module/API/dependency needed.
- checked: node storage/snapshot_codec/tests.rs:166-197 and tests/chainstate_wallet_and_headers.rs:131-147.
  found: Existing wallet_snapshot_round_trips_through_original_descriptors imports a default wildcard and allocates twice, then compares full DTO roundtrip plus range 0/1000/cursor2. The fixture dynamically uses current formatter output. Repo-wide Rust search finds #ob:: only in the production normalizer, not an explicit legacy fixture.
  implication: Existing test currently covers normalization incidentally, but after formatter correction it covers canonical new writes; no existing explicit legacy-byte regression selector can be honestly named.
- checked: node storage/fjall_store/tests/wallet_persistence.rs and tests.rs:224-249.
  found: named_wallet_registry_and_selection_survive_reopen checks durable snapshots but its wallet uses a nonranged WIF descriptor.
  implication: Useful adjacent persistence regression, not evidence of legacy ranged normalization.
- checked: docs/parity/index.json:2; source-breadcrumbs.json wallet-descriptor group; pinned Knots descriptors.md:77,220,310-322, descriptor.cpp:2325-2361 and wallet_descriptor.py:191-199,261-267.
  found: Baseline remains 29.3.knots20260210. Existing descriptor files share the three Knots breadcrumbs. Knots supports inclusive default 0-1000 and separate import range/next_index; it validates descriptor checksums. Open Bitcoin current parser ignores non-ob: suffixes and storage omits checksum; #ob metadata is local storage syntax, not Knots checksum syntax.
  implication: Preserve existing source pin/breadcrumbs and checksum behavior; correction does not establish new checksum parity or introduce a new storage schema.

## Resolution

root_cause: RANGE_METADATA_PREFIX includes its terminal colon, but storage_text appends another colon before range.start. Every ranged descriptor emits #ob::start:end:next, which the public parser rejects because it expects exactly three components after ob:.
fix: Suggested only: replace formatter literal {body}#{}:{}:{}:{} with {body}#{}{}:{}:{} at descriptor/format.rs:163; leave prefix, parser, decoder normalizer, APIs, schemas and dependencies unchanged.
verification: High-confidence direct source trace corroborates parent-reported exit101 assertion. No Cargo/Bazel, production edits, test edits, plan/shared-state edits or Git operations performed by this agent. Executable RED/GREEN and genuine preflight remain prerequisite executor work.
files_changed: [.planning/debug/phase162-descriptor-metadata-roundtrip.md]

## Minimal Correction and Regression Ownership

The only production ownership needed is `packages/open-bitcoin-wallet/src/descriptor/format.rs`. Removing the redundant formatter separator preserves body construction and all start/end/cursor values, including zero and exhausted cursors. Changing the prefix would also change the public parser recognition grammar and is unnecessary. Relaxing the parser, normalizing new writes elsewhere, adding an API, changing schema/dependencies, or weakening the Plan05 assertion would miss the source of the defect.

Use the existing `packages/open-bitcoin-wallet/src/descriptor/tests/key_source_and_storage_helpers_cover_extended_branches.rs` (413 lines) for focused public parse → storage_text → parse tests. Use the existing `packages/open-bitcoin-wallet/src/descriptor/tests/parser_accepts_single_key_descriptors_with_optional_checksums.rs` (390 lines) for strict malformed metadata and optional checksum-suffix behavior. `descriptor/tests.rs` already registers both modules; it needs no edit. Preserve their existing parity breadcrumb headers and use focused Arrange/Act/Assert tests. No new first-party file or breadcrumb registry extension is required when using these files.

Decisive cases, with complete descriptor equality and public range/cursor/script/address checks as applicable:

1. Explicit `#ob:0:0:0`: exact canonical storage suffix, range/cursor `Some(0)`, body unchanged for the canonical existing extended-key fixture, script at index0 unchanged.
2. Explicit `#ob:1:4:2`: exact nonzero tuple retained through storage and reparse; same current-index script/address and inclusive endpoint matching. Use ordinary unhardened tpub paths when deriving scripts.
3. Advance cursor through the public `advance_next_index` API: range0..=1 goes cursor0→1→2; storage/reparse preserves cursor2 and a further advance returns `DescriptorCursorExhausted`. Do not clamp or reject an exhausted stored cursor, and do not add new cursor validation to this fix.
4. Default wildcard without metadata: getters remain 0/1000/0 and storage becomes `#ob:0:1000:0`; the reparse equals the first parsed descriptor. Exercise both extended public and private keys and wrapper variants where cheap; formatter is shared across pkh/sh(wpkh)/wpkh/tr.
5. Optional checksum suffix plus metadata: parse a known checksum-bearing body (existing pinned Knots test fixture at wallet_descriptor.py:193) with `#ob` appended; preserve body and range/cursor across canonical storage/reparse. Existing storage omits checksum and parser ignores non-ob suffixes; do not add checksum validation or claim raw checksum text preservation.
6. Strict public negative controls: `#ob::0:0:0` remains `DescriptorSyntax` invalid descriptor range metadata, alongside wrong component count, invalid u32 and inverted range. This asserts the public parser boundary only.
7. Ordinary nonranged descriptor storage remains checksum-free body with no metadata; no incidental range introduction.

Existing node codec selector (run serially with timing wrapper):

`storage::snapshot_codec::tests::chainstate_wallet_and_headers::wallet_snapshot_round_trips_through_original_descriptors`

Adjacent durable selector:

`storage::fjall_store::tests::wallet_persistence::named_wallet_registry_and_selection_survive_reopen`

Both use `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features <selector> -- --exact --test-threads=1`. They were inspected, not run here. Before the correction the first selector incidentally exercises malformed writer output plus the decoder normalizer; afterward it only proves new canonical output survives DTO load. There is no existing explicit legacy fixture selector. Preserve the normalizer unchanged and report its pre-existing compatibility honestly; this diagnosis does not authorize removing it or claiming persisted malformed legacy text is rejected.

Parent follow-up: the checked prerequisite will add an explicit legacy fixture regression in existing `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs`, with no node production edits. Source-derived assertions: construct valid canonical descriptor semantics and an encoded DTO whose `payload.descriptors[0].original_text` contains exactly one legacy `#ob::start:end:next` suffix, decode through the existing codec, and require (a) decoded raw `original_text` still byte-equals the legacy input, (b) public start/end/next getters preserve the exact tuple, (c) descriptor equality with the canonical parsed descriptor, preserving key/origin/path/wildcard/kind and signing capability, and (d) same current-index script/address for a derivable fixture. The decoded descriptor's future `storage_text()` should use canonical `#ob:start:end:next` after the formatter correction; it is intentionally different from retained legacy `original_text`. If re-encoding this unchanged decoded record is checked, DTO serialization still copies retained legacy `original_text`; never assume all future snapshot bytes rewrite legacy text automatically. Include nonzero start/end/cursor to detect accidental default substitution. This preserves established compatibility rather than introducing repair or extending accepted input grammar.

## Frozen Plan05 Caller Gate

After focused RED/GREEN and wallet/node regression checks, the executor must rerun the unchanged funded-wallet preflight through the existing cooperative timing wrapper:

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun run scripts/command-timings.ts run --key phase162-wallet-bounded-preflight -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --all-features phase162_wallet_funded_short_prefix_preflight -- --test-threads=1 --nocapture
```

Require one discovered passing test, real accepted102-block history, normal maturity100, actual h20 tx0/vout1 funding25000sats, original recipe descriptor body and script, complete public scan, cold registry reload with exact snapshot/job, all handles closed and cleanup. Preserve the historical93.64s default-wildcard pass and3.88s bounded failure as separate receipts; record the new actual bounded runtime without claiming it in advance. Keep full pressure blocked until that gate passes. No recipe, maturity, funded outpoint, public scan, retention order or acceptance assertion may be changed to compensate for the formatter bug.

## Guidance Applied

Loaded repo AGENTS, Bright Builds sidecar, placeholder-only overrides, standards index and architecture/testing/code-shape/verification/Rust standards, active lesson inputs, current STATE, Phase162 CONTEXT and latest05 PLAN. The scoped recommendation follows pure-core correction, existing strict boundary parsing, focused behavior tests, serial timed verification, pinned parity breadcrumbs, D-09 authority/dependency limits and D-14 deferred Git finalization. The parent accepted preserving the existing decoder normalization boundary unchanged after receiving the finding.
