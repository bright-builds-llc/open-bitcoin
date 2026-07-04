---
phase: 112-bip152-wire-codec-and-message-semantics
verified: 2026-07-04T20:54:32Z
status: passed
score: "4/4 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 112-2026-07-04T19-37-55
generated_at: 2026-07-04T20:54:32Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 112: BIP152 Wire Codec and Message Semantics Verification Report

**Phase Goal:** Add first-party BIP152 payload support and malformed-input semantics before compact relay runtime behavior depends on it.
**Verified:** 2026-07-04T20:54:32Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

I verified the roadmap success criteria first, then checked plan must-haves, artifacts, key links, malformed-input behavior, parity breadcrumbs, and scope boundaries against the actual code. Executor summaries were used only as supporting evidence; high-risk behavior was directly checked with source review and focused commands.

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | `sendcmpct` version 2 payloads round-trip and unsupported versions follow the documented Knots-compatible boundary. | VERIFIED | `SendCompactMessage`, `BIP152_COMPACT_BLOCKS_VERSION = 2`, fixed 9-byte encode/decode helpers, and unsupported version tests are present in `packages/open-bitcoin-codec/src/compact_block.rs` and `packages/open-bitcoin-codec/src/compact_block/tests.rs`. Direct check: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec phase112` passed the sendcmpct cases. |
| 2 | `cmpctblock` payloads encode and decode headers, nonces, six-byte short IDs, and prefilled transaction differential indexes. | VERIFIED | `CompactBlockPayload`, `ShortId([u8; 6])`, `PrefilledTransaction`, `encode_compact_block_payload`, `decode_compact_block_payload`, and `expand_prefilled_positions` exist and use block-header and witness transaction codec helpers. Direct codec tests cover exact six-byte short IDs and byte-preserving witness prefilled transaction round trips. |
| 3 | `getblocktxn` and `blocktxn` payloads encode and decode differential indexes and witness transaction serialization. | VERIFIED | `BlockTransactionsRequest`, `BlockTransactions`, checked `expand_block_transaction_indexes`, and request/response encode/decode helpers exist. Direct tests cover multi-index deltas, empty vectors, `u16` overflow rejection, and witness-preserving `blocktxn` round trips. |
| 4 | Malformed compact-block payloads are rejected before partial reconstruction state is accepted. | VERIFIED | `validate_compact_block_structure` rejects empty compact blocks, count overflow, differential overflow, out-of-bounds prefilled positions, and null prefilled transactions. Malformed codec and network-message tests cover EOF, trailing data, non-canonical counts, overflow, null transactions, and superfluous witness records. No reconstruction state or runtime fallback code was introduced. |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-codec/src/compact_block.rs` | BIP152 payload structs and encode/decode helpers | VERIFIED | Contains sendcmpct, cmpctblock, getblocktxn, and blocktxn types/helpers, six-byte `ShortId`, structural validation, checked differential expansion, and Knots breadcrumbs. |
| `packages/open-bitcoin-codec/src/compact_block/tests.rs` | Focused Phase 112 codec and malformed-payload tests | VERIFIED | Contains 17 passing Phase 112 tests from the direct focused run. Tests use Arrange/Act/Assert structure and cover round trips plus malformed matrices. |
| `packages/open-bitcoin-codec/src/error.rs` | Stable BIP152 structural error names | VERIFIED | Adds and displays `DifferentialIndexOverflow`, `CompactBlockEmpty`, `CompactBlockTransactionCountOverflow`, `PrefilledTransactionOutOfBounds`, and `CompactBlockNullPrefilledTransaction`. |
| `packages/open-bitcoin-codec/src/lib.rs` | Public exports for new codec surface | VERIFIED | Re-exports all Phase 112 payload types, encode/decode helpers, and structural validation helpers. |
| `packages/open-bitcoin-network/src/message.rs` | Explicit BIP152 `WireNetworkMessage` variants and command mapping | VERIFIED | Adds `SendCompact`, `CompactBlock`, `GetBlockTxn`, and `BlockTxn` variants with exact `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn` command names and delegate encode/decode calls. |
| `packages/open-bitcoin-network/src/message/cursor.rs` | Extracted cursor helpers | VERIFIED | New module is wired through `mod cursor;` and registered in parity breadcrumbs. It is a file-length refactor, not new runtime compact relay policy. |
| `packages/open-bitcoin-network/src/message/tests.rs` | Wire-level BIP152 regression tests | VERIFIED | Direct focused run passed 10 message-level Phase 112 tests for payload/wire round trips, malformed errors, all explicit BIP152 commands, and compact-block inventory staying under `getdata`. |
| `packages/open-bitcoin-network/src/peer.rs` | Exhaustive peer handling without runtime policy | VERIFIED | Handles decoded BIP152 messages with `Ok(Vec::new())`, preserving Phase 112's payload-only boundary. |
| `packages/open-bitcoin-network/src/peer/tests.rs` | Peer no-op and deferred-command guards | VERIFIED | `phase112_bip152_wire_messages_are_peer_noops` passed, and deferred relay/filter commands remain unknown. |
| `docs/parity/source-breadcrumbs.json` | Parity breadcrumb registration | VERIFIED | Registers `compact_block.rs`, `compact_block/tests.rs`, `message.rs`, `message/cursor.rs`, and `message/tests.rs` under appropriate Knots anchors. Direct `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 350 Rust files. |
| `docs/metrics/lines-of-code.md` | Generated LOC freshness artifact | VERIFIED | Present and consistent with the executor summary's final `bash scripts/verify.sh` run. |

Note: `gsd-tools verify artifacts` produced false negatives for several `contains` arrays because it joined multiple expected strings into one comma-separated pattern. Manual `rg` checks verified the individual symbols, command names, tests, and error variants.

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/message.rs` | `packages/open-bitcoin-codec/src/compact_block.rs` | BIP152 payload encode/decode delegates | VERIFIED | Direct source check found delegate calls for all four commands. `gsd-tools verify key-links` passed for all plan links. |
| `packages/open-bitcoin-codec/src/compact_block.rs` | `packages/open-bitcoin-codec/src/transaction.rs` | Witness transaction serialization | VERIFIED | `encode_compact_block_payload` and `encode_block_transactions_payload` use `TransactionEncoding::WithWitness`; decode paths parse transactions with witness support. |
| `packages/open-bitcoin-codec/src/compact_block.rs` | Knots BIP152 anchors | Parity breadcrumbs and tests | VERIFIED | File breadcrumbs and `docs/parity/source-breadcrumbs.json` cite `protocol.h`, `blockencodings.h`, `net_processing.cpp`, `p2p_compactblocks.py`, and `messages.py`. |
| `packages/open-bitcoin-network/src/peer.rs` | Phase 112 runtime boundary | No-op peer handling | VERIFIED | `WireNetworkMessage::{SendCompact, CompactBlock, GetBlockTxn, BlockTxn}` return no peer actions; no negotiation, reconstruction, fallback, mempool lookup, misbehavior, disconnect, metrics, RPC, CLI, dashboard, or support-bundle behavior was added. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| BIP152 codec helpers | Payload bytes | Function inputs and typed payload structs | Yes | VERIFIED - deterministic pure encode/decode path; no dynamic UI/store data flow applies. |
| Network message mapping | Wire payload bytes | `WireNetworkMessage::encode_payload` and `decode_payload` | Yes | VERIFIED - payload bytes flow through explicit codec delegates and round-trip through `ParsedNetworkMessage::decode_wire`. |
| Peer handler no-op boundary | Decoded BIP152 messages | `PeerManager::handle_message` | Yes | VERIFIED - messages are accepted as decoded data and intentionally produce no actions until later policy phases. |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Phase 112 codec behavior | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec phase112 -- --nocapture` | 17 passed, 0 failed | PASS |
| Phase 112 network and peer behavior | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase112 -- --nocapture` | 11 passed, 0 failed | PASS |
| Affected crate linting | `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-codec -p open-bitcoin-network --all-targets --all-features -- -D warnings` | Passed | PASS |
| Parity breadcrumb registry | `bun run scripts/check-parity-breadcrumbs.ts --check` | Passed for 350 Rust files | PASS |
| Full repo verifier | Executor summary evidence: `bash scripts/verify.sh` | Reported passed in `112-03-SUMMARY.md` after LOC regeneration, file-length split, and coverage fixes | PASS (summary evidence reused) |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| CMP-01 | `112-01-PLAN.md` | Encode, decode, and validate `sendcmpct` messages with version 2 semantics and documented unsupported-version handling | SATISFIED | Version 2 round-trip and unsupported versions 1/3 decode as data in codec and network tests. |
| CMP-02 | `112-02-PLAN.md` | Encode, decode, and validate `cmpctblock` payloads with header, nonce, six-byte short IDs, and prefilled differential indexes | SATISFIED | Typed compact-block payload, exact-width `ShortId`, witness prefilled transactions, structural validation, and wire-message tests are present and passing. |
| CMP-03 | `112-03-PLAN.md` | Encode, decode, and validate `getblocktxn` and `blocktxn` payloads with differential indexes and witness transaction serialization | SATISFIED | Request/response payload helpers, checked `u16` index expansion, witness transaction serialization, and direct tests are present and passing. |
| RCN-01 | `112-02-PLAN.md`, `112-03-PLAN.md` | Validate compact-block structural malformed inputs before accepting partial state | SATISFIED | Decode/validation rejects malformed compact blocks before any reconstruction state exists; runtime reconstruction remains absent and deferred. |

No orphaned Phase 112 requirements were found. `.planning/REQUIREMENTS.md` maps only CMP-01, CMP-02, CMP-03, and RCN-01 to Phase 112.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| N/A | N/A | None | Info | Direct scans found no TODO/FIXME/placeholders in touched BIP152 codec/message files and no forbidden reconstruction/fallback/mempool/runtime-policy terms in the new BIP152 codec or message tests. Existing `peer.rs` mempool and misbehavior symbols predate Phase 112 and are unrelated to the new BIP152 no-op arms. |

### Human Verification Required

None. Phase 112 is pure codec/message semantics with deterministic unit and lint checks. Visual, external-service, public-network, operator-surface, and runtime compact-relay behavior are outside this phase.

### Gaps Summary

No blocking gaps found. The phase goal is achieved: Open Bitcoin now has first-party BIP152 payload support for `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn`, rejects malformed compact-block payloads at codec/message boundaries before reconstruction state, carries parity breadcrumbs, and keeps compact relay runtime policy out of scope.

_Verified: 2026-07-04T20:54:32Z_
_Verifier: Claude (gsd-verifier)_
