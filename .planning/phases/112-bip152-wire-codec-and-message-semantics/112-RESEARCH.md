# Phase 112: BIP152 Wire Codec and Message Semantics - Research

**Researched:** 2026-07-04 [VERIFIED: system date]
**Domain:** Bitcoin P2P BIP152 wire payload codec and malformed-input semantics [VERIFIED: .planning/phases/112-bip152-wire-codec-and-message-semantics/112-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: local code, pinned Knots baseline, BIP152]

<user_constraints>
## User Constraints (from CONTEXT.md)

Source: `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-CONTEXT.md`; the following phase boundary, locked decisions, discretion area, and deferred ideas are copied verbatim for planner enforcement. [VERIFIED: 112-CONTEXT.md]

## Phase Boundary

Phase 112 adds first-party BIP152 wire payload support for `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn`, plus malformed-input semantics that reject invalid compact-block payloads before any partial reconstruction state exists.

This phase owns pure payload types, encode/decode helpers, network-message command mapping, focused round-trip fixtures, malformed payload tests, and parity breadcrumbs for BIP152 message serialization. It may validate header, nonce, six-byte short ID, prefilled transaction differential index, block transaction request, and witness transaction serialization shapes.

This phase must not implement compact relay negotiation policy, compact-block announcement eligibility, mempool-based short ID reconstruction, missing transaction scheduling, `blocktxn` in-flight matching, fallback to full blocks, validation/connect handoff, operator evidence rollout, package relay, bloom/filter serving, compact filter serving, public-serving defaults, public-network CI, archive-node behavior, production full-node readiness, production-service operation, or production-funds wallet use.

## Implementation Decisions

### Message Surface

- **D-01:** Add explicit BIP152 message variants to `WireNetworkMessage` for `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn`; unknown BIP152 command strings should stop being `UnknownCommand` only for these four payloads.
- **D-02:** `sendcmpct` should be represented as a pure payload type with an announce/preference boolean and a `u64` version. Version 2 is the in-scope supported semantic, while unsupported versions should still decode as data so Phase 113 can ignore or suppress them through peer policy.
- **D-03:** Message command names must stay anchored to Knots `NetMsgType::{SENDCMPCT,CMPCTBLOCK,GETBLOCKTXN,BLOCKTXN}` and BIP152. Do not overload existing `GetData`, `Block`, or transaction relay message branches for these commands.

### Compact Block Payloads

- **D-04:** Model compact block payloads with a block header, nonce, six-byte short IDs, and prefilled transactions. Six-byte short IDs should be stored in a type that cannot accidentally accept or emit eight-byte values.
- **D-05:** Prefilled transaction indexes are differential indexes on the wire. The decoded model may expose the wire deltas directly for Phase 112, but helpers/tests must prove malformed overflow, non-canonical compact sizes, out-of-order expansion, and excessive transaction counts fail before reconstruction.
- **D-06:** `cmpctblock` parsing should validate the structural boundary only: header presence, nonce presence, short ID width, prefilled transaction differential-index shape, witness transaction serialization, no trailing data, and count bounds. Header-chain validity, short ID matching, mempool lookup, and block mutation checks belong to later phases.

### Block Transaction Round Trips

- **D-07:** `getblocktxn` should carry a block hash plus differential transaction indexes matching Knots `BlockTransactionsRequest`. Indexes should be bounded to `u16`-compatible values because Knots stores the request indexes as `std::vector<uint16_t>`.
- **D-08:** `blocktxn` should carry a block hash plus transactions serialized with witness data using the existing `TransactionEncoding::WithWitness` codec. Do not add a no-witness `blocktxn` runtime path in Phase 112.
- **D-09:** Round-trip tests should cover multi-index differential encoding, empty vectors where permitted by the wire shape, witness-carrying transactions, and byte-preserving decode/encode for representative payloads.

### Malformed Payload Boundary

- **D-10:** Malformed compact-block payloads should fail at decode with stable `CodecError` or `NetworkError` values before any partial compact-block state can be initialized. Prefer extending `CodecError` with precise BIP152 structural labels over using generic string errors.
- **D-11:** Reject trailing bytes, unexpected EOF, non-canonical compact sizes, six-byte short ID truncation/overflow, differential value overflow, indexes above the representable range, prefilled positions beyond the implied transaction count, null/structurally invalid transactions, and superfluous witness records.
- **D-12:** Keep misbehavior, disconnect, ignore, fallback, and peer-state consequences out of the codec. The codec returns typed decode results or errors; Phase 113+ maps those into peer behavior.

### Runtime Scope

- **D-13:** Phase 112 must leave `InventoryType::CompactBlock` serving behavior as Phase 111 locked it: bounded and classified, but not served as a compact block response from `getdata`.
- **D-14:** The implementation should live primarily in `open-bitcoin-codec` and `open-bitcoin-network::message`, preserving functional core / imperative shell boundaries and avoiding node-shell storage, mempool, socket, metric, log, status, or support-bundle effects.
- **D-15:** If a new Rust source file is introduced under `packages/open-bitcoin-*/src`, add parity breadcrumbs in the file comment and `docs/parity/source-breadcrumbs.json` unless an explicit `none` breadcrumb is defensible.

### Verification And Parity

- **D-16:** Unit tests should be focused and Arrange/Act/Assert structured for pure BIP152 codec behavior. Add mutation-style malformed payload cases around differential index overflow and short ID width because these are high-risk parsing bugs.
- **D-17:** Use repo-native verification and focused package checks first, then `bash scripts/verify.sh` before final phase completion. Public-network compact-block review remains out of default verification.
- **D-18:** Planning may split the phase into the roadmap's three plans: `sendcmpct` and message enum support, `cmpctblock` codec and fixtures, and `getblocktxn`/`blocktxn` codec plus malformed payload tests.

### Claude's Discretion

The planner may choose exact Rust type names, whether BIP152 payloads live in a new `compact_block.rs` codec module or inside existing codec/network modules, and how many fixtures to generate. Prefer a small pure API that later phases can consume without compatibility shims, and prefer typed malformed outcomes over broad catch-all errors.

## Deferred Ideas (OUT OF SCOPE)

Compact relay negotiation, compact-block announcement policy, mempool/recent-block short ID reconstruction, missing transaction scheduling, `blocktxn` in-flight matching, fallback, validation/connect handoff, operator status/RPC/CLI/dashboard/metrics/log/support evidence, parity/UAT release closeout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production-service operation, and production-funds wallet use remain outside Phase 112.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CMP-01 | Node encodes, decodes, and validates `sendcmpct` messages with version 2 semantics and documented handling for unsupported versions. [VERIFIED: .planning/REQUIREMENTS.md] | BIP152 defines `sendcmpct` as 1 byte boolean plus little-endian 8 byte version, Knots sends version 2, and Knots policy ignores unsupported versions after decode. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] [VERIFIED: packages/bitcoin-knots/src/protocol.h, packages/bitcoin-knots/src/net_processing.cpp, packages/bitcoin-knots/test/functional/p2p_compactblocks.py] |
| CMP-02 | Node encodes, decodes, and validates `cmpctblock` payloads with header, nonce, six-byte short IDs, and prefilled transaction differential indexes. [VERIFIED: .planning/REQUIREMENTS.md] | Knots `CBlockHeaderAndShortTxIDs` serializes header, nonce, 6-byte short IDs, and prefilled transactions; BIP152 documents the same wire fields. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] |
| CMP-03 | Node encodes, decodes, and validates `getblocktxn` and `blocktxn` payloads with differential indexes and witness transaction serialization. [VERIFIED: .planning/REQUIREMENTS.md] | Knots `BlockTransactionsRequest` uses `DifferenceFormatter` over `std::vector<uint16_t>`, and `BlockTransactions` serializes transactions with `TX_WITH_WITNESS`; BIP152 v2 requires witness data in `blocktxn`. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] |
| RCN-01 | Node validates compact block headers, transaction counts, prefilled ordering, null transactions, short ID bounds, and malformed payloads before accepting partial state. [VERIFIED: .planning/REQUIREMENTS.md] | Knots checks null header/empty compact data, transaction count bounds, null prefilled transactions, differential prefilled expansion, and impossible prefilled positions before `PartiallyDownloadedBlock` is usable; Phase 112 should implement the structural subset before any reconstruction state exists. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp] |
</phase_requirements>

## Summary

Phase 112 should add first-party, pure BIP152 payload codecs in `open-bitcoin-codec` and expose them through explicit `WireNetworkMessage` variants in `open-bitcoin-network::message`. [VERIFIED: 112-CONTEXT.md, packages/open-bitcoin-codec/src/lib.rs, packages/open-bitcoin-network/src/message.rs] The plan should reuse existing block-header, compact-size, transaction-with-witness, message-header, inventory, and typed error infrastructure instead of adding a new dependency or putting byte parsing in node-shell code. [VERIFIED: packages/open-bitcoin-codec/src/block.rs, packages/open-bitcoin-codec/src/compact_size.rs, packages/open-bitcoin-codec/src/transaction.rs, packages/open-bitcoin-network/src/message.rs, AGENTS.md]

The most important planning distinction is codec acceptance versus peer policy. [VERIFIED: 112-CONTEXT.md] `sendcmpct` unsupported versions should decode as payload data and be ignored later by Phase 113 policy, while malformed byte shapes for `cmpctblock`, `getblocktxn`, and `blocktxn` should fail with stable typed errors before partial reconstruction state can be constructed. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/test/functional/p2p_compactblocks.py, packages/bitcoin-knots/src/blockencodings.cpp]

**Primary recommendation:** Create a small `open_bitcoin_codec::compact_block` module for BIP152 payload structs and encode/decode helpers, then wire four new `WireNetworkMessage` variants to those helpers with focused Arrange/Act/Assert tests and parity breadcrumbs. [VERIFIED: standards/core/architecture.md, standards/languages/rust.md, packages/open-bitcoin-codec/src/lib.rs, packages/open-bitcoin-network/src/message.rs, AGENTS.md]

## Project Constraints (from .cursor/rules/)

- No `.cursor/rules/` files were found in this workspace. [VERIFIED: Glob `.cursor/rules/**`]
- No `.cursor/skills/**/SKILL.md` or `.agents/skills/**/SKILL.md` project skill indexes were found in this workspace. [VERIFIED: Glob `.cursor/skills/**/SKILL.md`, Glob `.agents/skills/**/SKILL.md`]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust workspace crates | `0.1.0`, Rust 2024 edition [VERIFIED: packages/Cargo.toml] | Implement first-party Bitcoin wire domain models and codecs. [VERIFIED: packages/Cargo.toml, AGENTS.md] | Project policy requires first-party Bitcoin domain implementation and forbids production use of existing Rust Bitcoin libraries. [VERIFIED: AGENTS.md] |
| Rust toolchain | `rustc 1.94.1`, `cargo 1.94.1` [VERIFIED: environment probe, rust-toolchain.toml] | Compile, lint, and test Phase 112 Rust code. [VERIFIED: scripts/verify.sh] | Repo pins Rust through `rust-toolchain.toml` for local Cargo, CI, and Bazel. [VERIFIED: AGENTS.md] |
| `open-bitcoin-codec` | workspace `0.1.0` [VERIFIED: packages/open-bitcoin-codec/Cargo.toml, packages/Cargo.toml] | Own BIP152 byte-level payload structs and encode/decode helpers. [VERIFIED: packages/open-bitcoin-codec/src/lib.rs] | Existing block, transaction, network, compact-size, and reader helpers already live here. [VERIFIED: packages/open-bitcoin-codec/src/block.rs, packages/open-bitcoin-codec/src/transaction.rs, packages/open-bitcoin-codec/src/network.rs, packages/open-bitcoin-codec/src/compact_size.rs, packages/open-bitcoin-codec/src/primitives.rs] |
| `open-bitcoin-network` | workspace `0.1.0` [VERIFIED: packages/open-bitcoin-network/Cargo.toml, packages/Cargo.toml] | Expose BIP152 payloads as P2P message variants and command mappings. [VERIFIED: packages/open-bitcoin-network/src/message.rs] | Existing `WireNetworkMessage` owns command names, payload encode/decode, wire checksums, and unknown-command behavior. [VERIFIED: packages/open-bitcoin-network/src/message.rs] |
| Bitcoin Knots baseline | `v29.3.knots20260210` submodule commit `a9aee730466ac67d35a3c03ee24676be5e045878` [VERIFIED: environment probe] | Anchor parity for command names, serializer shape, differential indexes, and malformed boundaries. [VERIFIED: packages/bitcoin-knots/src/protocol.h, packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/src/blockencodings.cpp] | Repo core value requires observable Knots parity for in-scope behavior. [VERIFIED: AGENTS.md] |
| BIP152 | Status `Deployed` [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | Define compact-block wire structures and version 2 witness semantics. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | It is the normative protocol specification for the four in-scope messages. [VERIFIED: packages/bitcoin-knots/src/protocol.h] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `open-bitcoin-consensus` | workspace `0.1.0` [VERIFIED: packages/Cargo.toml, packages/open-bitcoin-network/Cargo.toml] | Provide existing `MAX_BLOCK_WEIGHT` and `check_transaction` structural/consensus checks if the plan chooses to validate decoded prefilled transactions beyond byte shape. [VERIFIED: packages/open-bitcoin-consensus/src/lib.rs, packages/open-bitcoin-consensus/src/transaction.rs] | Use carefully from `open-bitcoin-network` tests or validation helpers only if needed; avoid making low-level `open-bitcoin-codec` depend on consensus because `open-bitcoin-codec` currently depends only on primitives. [VERIFIED: packages/open-bitcoin-codec/Cargo.toml, packages/open-bitcoin-network/Cargo.toml] |
| Bun | `1.3.9` [VERIFIED: environment probe, .bun-version] | Run repo-owned TypeScript verifier/checker scripts if Phase 112 adds deterministic docs or parity guardrails. [VERIFIED: AGENTS.md, scripts/verify.sh] | Use for substantial repo-owned automation scripts, not for Rust codec implementation. [VERIFIED: AGENTS.md] |
| Bazel | `8.6.0` [VERIFIED: environment probe] | Run the full verifier smoke build. [VERIFIED: scripts/verify.sh] | Use through `bash scripts/verify.sh`; `--fast` skips Bazel for local iteration only. [VERIFIED: AGENTS.md, scripts/verify.sh] |
| `cargo-llvm-cov` | `0.8.5` [VERIFIED: environment probe] | Run full verifier coverage path. [VERIFIED: scripts/verify.sh] | Required by full verification, not by focused local iteration. [VERIFIED: scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| First-party `open-bitcoin-codec::compact_block` module [VERIFIED: packages/open-bitcoin-codec/src/lib.rs] | Inline all BIP152 helpers inside `open-bitcoin-network/src/message.rs` [VERIFIED: packages/open-bitcoin-network/src/message.rs] | Inline parsing would enlarge an already central message file and make reusable byte-level tests harder to isolate. [VERIFIED: standards/core/code-shape.md, standards/core/testing.md] |
| `ShortId([u8; 6])` newtype [VERIFIED: 112-CONTEXT.md] | Store short IDs as raw `u64` everywhere [VERIFIED: packages/bitcoin-knots/src/blockencodings.h] | Raw `u64` matches Knots internals but can accidentally accept or emit 8-byte values; a 6-byte newtype makes the wire invariant explicit. [VERIFIED: standards/core/architecture.md, standards/languages/rust.md, packages/bitcoin-knots/test/functional/test_framework/messages.py] |
| Existing first-party transaction witness codec [VERIFIED: packages/open-bitcoin-codec/src/transaction.rs] | Add `rust-bitcoin` or another external Bitcoin codec [ASSUMED] | External Bitcoin production dependencies conflict with the repo dependency policy. [VERIFIED: AGENTS.md] |

**Installation:** No new package installation is recommended for Phase 112. [VERIFIED: packages/open-bitcoin-codec/Cargo.toml, packages/open-bitcoin-network/Cargo.toml, AGENTS.md]

```bash
# No npm, cargo add, or bun install step is recommended for this phase.
```

**Version verification:** Recommended package versions are workspace-owned or toolchain-pinned, not npm packages. [VERIFIED: packages/Cargo.toml, rust-toolchain.toml, environment probe] The environment probe confirmed `rustc 1.94.1`, `cargo 1.94.1`, `bun 1.3.9`, `bazel 8.6.0`, `cargo-llvm-cov 0.8.5`, and Knots submodule `v29.3.knots20260210`. [VERIFIED: environment probe]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-codec/src/
|-- compact_block.rs      # BIP152 payload structs, ShortId, differential index helpers, encode/decode
|-- lib.rs                # Re-export compact block payload APIs
|-- block.rs              # Existing block header codec reused by cmpctblock
|-- transaction.rs        # Existing witness transaction codec reused by cmpctblock/blocktxn
`-- error.rs              # Existing CodecError extended with precise BIP152 structural labels

packages/open-bitcoin-network/src/
|-- message.rs            # WireNetworkMessage variants and command mapping
`-- message/tests.rs      # Wire command/payload integration tests
```

This structure follows the Rust standard preference for `foo.rs` plus optional `foo/` children rather than `foo/mod.rs`. [VERIFIED: standards/languages/rust.md]

### Pattern 1: Parse BIP152 at the Codec Boundary

**What:** Parse raw bytes into domain payload types such as `SendCompactMessage`, `CompactBlockPayload`, `ShortId`, `PrefilledTransaction`, `BlockTransactionsRequest`, and `BlockTransactions`. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/src/blockencodings.h]

**When to use:** Use this for all four BIP152 command payloads before `WireNetworkMessage` exposes them to peer-state code. [VERIFIED: packages/open-bitcoin-network/src/message.rs]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/blockencodings.h and BIP152.
// Pattern only: exact names are planner discretion.
pub struct CompactBlockPayload {
    pub header: BlockHeader,
    pub nonce: u64,
    pub short_ids: Vec<ShortId>,
    pub prefilled_transactions: Vec<PrefilledTransaction>,
}
```

### Pattern 2: Keep Wire Deltas Explicit, Provide Expansion Helpers

**What:** Preserve differential indexes as wire data while adding a helper that expands them with overflow and ordering checks. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/test/functional/test_framework/messages.py]

**When to use:** Use this for `PrefilledTransaction.index` and `BlockTransactionsRequest.indexes`, both of which are differentially encoded in BIP152. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/blockencodings.h DifferenceFormatter.
fn expand_differential_indexes(deltas: &[u64]) -> Result<Vec<u16>, CodecError> {
    let mut shift = 0_u64;
    let mut indexes = Vec::with_capacity(deltas.len());
    for delta in deltas {
        shift = shift
            .checked_add(*delta)
            .ok_or(CodecError::DifferentialIndexOverflow)?;
        let index = u16::try_from(shift)
            .map_err(|_| CodecError::DifferentialIndexOverflow)?;
        indexes.push(index);
        shift = shift
            .checked_add(1)
            .ok_or(CodecError::DifferentialIndexOverflow)?;
    }
    Ok(indexes)
}
```

### Pattern 3: Wire Commands Stay in `WireNetworkMessage`

**What:** Add variants for `SendCompact`, `CompactBlock`, `GetBlockTxn`, and `BlockTxn` and map command strings exactly to `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn`. [VERIFIED: 112-CONTEXT.md, packages/open-bitcoin-network/src/message.rs, packages/bitcoin-knots/src/protocol.h]

**When to use:** Use this in `command_name`, `encode_payload`, and `decode_payload` only; do not route these payloads through `GetData`, `Block`, or `Tx` variants. [VERIFIED: 112-CONTEXT.md]

### Anti-Patterns to Avoid

- **Treating unsupported `sendcmpct` versions as decode errors:** Knots policy ignores unsupported versions after the message is decoded, and Phase 112 explicitly reserves peer-policy consequences for later phases. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/test/functional/p2p_compactblocks.py]
- **Representing short IDs as public raw `u64` wire values:** BIP152 short IDs are 6 little-endian bytes, while Knots pads them into 64-bit integers only as an internal convenience. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] [VERIFIED: packages/bitcoin-knots/test/functional/test_framework/messages.py]
- **Accepting byte decode as reconstruction readiness:** Knots performs structural checks before `PartiallyDownloadedBlock` is usable, including null header/empty data, count bounds, null transactions, and prefilled position checks. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp]
- **Adding node-shell effects to codec work:** Phase 112 excludes storage, mempool lookup, sockets, metrics, logs, status, support bundles, fallback, and peer-state consequences. [VERIFIED: 112-CONTEXT.md]

## Existing Codec And Message Surfaces To Modify

- `packages/open-bitcoin-codec/src/lib.rs` should re-export the new BIP152 codec module if a new module is created. [VERIFIED: packages/open-bitcoin-codec/src/lib.rs]
- `packages/open-bitcoin-codec/src/error.rs` should gain precise `CodecError` variants for BIP152 structural failures instead of generic strings. [VERIFIED: packages/open-bitcoin-codec/src/error.rs, 112-CONTEXT.md]
- `packages/open-bitcoin-codec/src/block.rs` already exposes `parse_block_header` and `encode_block_header` for the 80-byte compact-block header. [VERIFIED: packages/open-bitcoin-codec/src/block.rs]
- `packages/open-bitcoin-codec/src/transaction.rs` already supports `TransactionEncoding::WithWitness`, rejects superfluous witness records, and encodes witness transactions when witness data exists. [VERIFIED: packages/open-bitcoin-codec/src/transaction.rs]
- `packages/open-bitcoin-codec/src/compact_size.rs` already rejects non-canonical CompactSize encodings and enforces `MAX_SIZE`. [VERIFIED: packages/open-bitcoin-codec/src/compact_size.rs]
- `packages/open-bitcoin-codec/src/primitives.rs` already provides `Reader`, fixed-width little-endian reads, and trailing-data detection. [VERIFIED: packages/open-bitcoin-codec/src/primitives.rs]
- `packages/open-bitcoin-network/src/message.rs` already owns `WireNetworkMessage`, `command_name`, `encode_payload`, `decode_payload`, wire checksum integration, and `UnknownCommand` fallback. [VERIFIED: packages/open-bitcoin-network/src/message.rs]
- `packages/open-bitcoin-network/src/message/tests.rs` already uses direct payload and wire round trips with Arrange/Act/Assert comments. [VERIFIED: packages/open-bitcoin-network/src/message/tests.rs, standards/core/testing.md]
- `packages/open-bitcoin-primitives/src/network.rs` already defines `InventoryType::CompactBlock` as raw type `4`; Phase 112 should not change serving behavior for that inventory type. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs, 112-CONTEXT.md]

## Exact Serialization Shapes

### `sendcmpct`

- Command name is `sendcmpct`. [VERIFIED: packages/bitcoin-knots/src/protocol.h]
- Payload is 1 byte boolean followed by an 8 byte little-endian version number. [VERIFIED: packages/bitcoin-knots/src/protocol.h, packages/bitcoin-knots/test/functional/test_framework/messages.py] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]
- Version 2 is the Knots-supported compact-block version in the pinned baseline. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]
- Version 2 witness compact blocks include witness data in `cmpctblock` and `blocktxn`. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]
- Knots functional tests assert version 1 and version 3 `sendcmpct` inputs do not enable compact announcements. [VERIFIED: packages/bitcoin-knots/test/functional/p2p_compactblocks.py]

### `cmpctblock`

- Command name is `cmpctblock`. [VERIFIED: packages/bitcoin-knots/src/protocol.h]
- Payload is serialized `CBlockHeaderAndShortTxIDs`: 80 byte block header, `u64` nonce, CompactSize short ID count, `count * 6` little-endian short ID bytes, CompactSize prefilled transaction count, then prefilled transactions. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/test/functional/test_framework/messages.py] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]
- Short IDs are 6-byte values on the wire, and Knots' Python fixtures append two zero bytes only to read them as 64-bit integers. [VERIFIED: packages/bitcoin-knots/test/functional/test_framework/messages.py]
- Prefilled transaction entries contain a differentially encoded CompactSize index and a transaction serialized with witness data for version 2 compact blocks. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/test/functional/test_framework/messages.py] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]
- Knots rejects compact blocks whose total implied transaction count exceeds `uint16_t::max()` during `CBlockHeaderAndShortTxIDs` deserialization. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h]

### `getblocktxn`

- Command name is `getblocktxn`. [VERIFIED: packages/bitcoin-knots/src/protocol.h]
- Payload is `BlockTransactionsRequest`: 32 byte block hash, CompactSize index count, and differentially encoded CompactSize indexes. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/test/functional/test_framework/messages.py] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]
- Knots stores decoded request indexes as `std::vector<uint16_t>`, so Phase 112 should reject decoded absolute indexes above `u16::MAX`. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h]

### `blocktxn`

- Command name is `blocktxn`. [VERIFIED: packages/bitcoin-knots/src/protocol.h]
- Payload is `BlockTransactions`: 32 byte block hash, CompactSize transaction count, and transactions serialized with witness data for compact-block version 2. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/test/functional/test_framework/messages.py] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]
- Knots fixtures include a no-witness `msg_no_witness_blocktxn` test helper, but Phase 112 context explicitly excludes a no-witness runtime path. [VERIFIED: packages/bitcoin-knots/test/functional/test_framework/messages.py, 112-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| CompactSize parsing | A separate BIP152 varint parser [ASSUMED] | Existing `read_compact_size` and `write_compact_size` [VERIFIED: packages/open-bitcoin-codec/src/compact_size.rs] | Existing code already rejects non-canonical encodings and enforces `MAX_SIZE`. [VERIFIED: packages/open-bitcoin-codec/src/compact_size.rs] |
| Block header serialization | Manual 80-byte field concatenation in network code [ASSUMED] | Existing `parse_block_header` and `encode_block_header` [VERIFIED: packages/open-bitcoin-codec/src/block.rs] | Existing helper is already anchored to Knots block serialization breadcrumbs. [VERIFIED: packages/open-bitcoin-codec/src/block.rs] |
| Witness transaction serialization | A custom transaction loop for `blocktxn` or prefilled txs [ASSUMED] | Existing `encode_transaction(..., TransactionEncoding::WithWitness)` and `parse_transaction` [VERIFIED: packages/open-bitcoin-codec/src/transaction.rs] | Existing codec handles witness marker/flag and superfluous witness rejection. [VERIFIED: packages/open-bitcoin-codec/src/transaction.rs] |
| Differential index overflow | Ad hoc `last + delta + 1` with unchecked arithmetic [ASSUMED] | A shared checked helper modeled on Knots `DifferenceFormatter` [VERIFIED: packages/bitcoin-knots/src/blockencodings.h] | Knots throws `"differential value overflow"` on overflow or out-of-range decoded index. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h] |
| Short ID width | A public `u64` that serializes with slicing at call sites [ASSUMED] | `ShortId([u8; 6])` or equivalent invariant type [VERIFIED: 112-CONTEXT.md] | The wire width is exactly 6 bytes, and the type should make 8-byte emission unrepresentable. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h, standards/languages/rust.md] |
| Peer consequences | Misbehavior/disconnect/ignore inside decode helpers [ASSUMED] | Typed `CodecError`/`NetworkError` returned to later policy phases [VERIFIED: 112-CONTEXT.md] | Phase 112 excludes peer-state consequences, and Phase 113+ maps decoded data/errors into policy. [VERIFIED: 112-CONTEXT.md] |

**Key insight:** The hard part is not inventing a codec framework; the hard part is preserving Knots' boundary between byte-level structural failure, decoded unsupported-version data, and later peer policy. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/test/functional/p2p_compactblocks.py]

## Common Pitfalls

### Pitfall 1: Rejecting Unsupported `sendcmpct` Too Early

**What goes wrong:** A decoder rejects version 1 or version 3 and prevents later peer policy from matching Knots' ignore behavior. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/test/functional/p2p_compactblocks.py]

**Why it happens:** BIP152 v2 support is easy to conflate with byte-level version validation. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]

**How to avoid:** Decode any `u64` version into a payload type, and put supported-version interpretation behind a Phase 113 policy helper. [VERIFIED: 112-CONTEXT.md]

**Warning signs:** Tests assert `decode_payload("sendcmpct", ...)` fails for unsupported versions. [VERIFIED: 112-CONTEXT.md]

### Pitfall 2: Treating `u64` Short IDs as Wire Values

**What goes wrong:** Encode accidentally emits 8 bytes or decode accepts a value that was never represented as 6 bytes on the wire. [VERIFIED: packages/bitcoin-knots/test/functional/test_framework/messages.py]

**Why it happens:** Knots stores short IDs in `std::vector<uint64_t>` while serializing with `CustomUintFormatter<6>`. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h]

**How to avoid:** Use a six-byte newtype and only expose conversion helpers if later short-ID computation needs a numeric value. [VERIFIED: standards/languages/rust.md, 112-CONTEXT.md]

**Warning signs:** Public constructors accept `u64` without checking `value <= 0x0000_ffff_ffff_ffff`. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp]

### Pitfall 3: Missing Differential Overflow

**What goes wrong:** A malicious payload with large deltas wraps the running index or creates indexes above `u16::MAX`. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h]

**Why it happens:** The BIP152 delta formula is `current - previous - 1`, and reconstruction code needs absolute positions. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] [VERIFIED: packages/bitcoin-knots/test/functional/test_framework/messages.py]

**How to avoid:** Use checked addition for `shift += delta`, reject `shift >= u64::MAX`, reject values outside `u16`, and test multi-index and overflow cases. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h, 112-CONTEXT.md]

**Warning signs:** Code casts `u64` deltas to `usize` or `u16` before range checks. [VERIFIED: standards/languages/rust.md]

### Pitfall 4: Failing After Partial State Exists

**What goes wrong:** The system allocates or accepts partial compact-block state before discovering null transactions, impossible prefilled positions, or excessive transaction counts. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp]

**Why it happens:** Byte decoding and reconstruction initialization can be mixed together. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp]

**How to avoid:** Add a pure structural validation step that runs immediately after `cmpctblock` decode and before any later reconstruction state type exists. [VERIFIED: 112-CONTEXT.md, standards/core/architecture.md]

**Warning signs:** Tests only check decode EOF/trailing data and do not check prefilled positions beyond `short_ids.len() + inserted_prefilled_count`. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp, packages/bitcoin-knots/test/functional/p2p_compactblocks.py]

### Pitfall 5: Confusing Witness and Non-Witness Transactions

**What goes wrong:** Version 2 compact blocks or `blocktxn` responses silently drop witness data. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]

**Why it happens:** Version 1 and version 2 compact blocks differ primarily in transaction encoding and short-ID hash input. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]

**How to avoid:** Always use `TransactionEncoding::WithWitness` for Phase 112 prefilled transactions and `blocktxn` payloads. [VERIFIED: 112-CONTEXT.md, packages/open-bitcoin-codec/src/transaction.rs]

**Warning signs:** A Phase 112 plan adds a no-witness `blocktxn` branch. [VERIFIED: 112-CONTEXT.md]

## Malformed Payload Cases Plans Must Cover

- `sendcmpct` payload shorter than 9 bytes or longer than 9 bytes should fail with EOF or trailing-data style errors. [VERIFIED: packages/open-bitcoin-codec/src/primitives.rs, packages/open-bitcoin-network/src/message.rs]
- `sendcmpct` unsupported versions should decode as data and not fail solely because the version is unsupported. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/test/functional/p2p_compactblocks.py]
- `cmpctblock` missing header, missing nonce, truncated short ID, truncated CompactSize, or trailing bytes should fail at decode. [VERIFIED: packages/open-bitcoin-codec/src/primitives.rs, packages/bitcoin-knots/src/blockencodings.h]
- Non-canonical CompactSize encodings for counts or differential indexes should fail. [VERIFIED: packages/open-bitcoin-codec/src/compact_size.rs] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]
- `cmpctblock` with `short_ids.len() + prefilled.len() > u16::MAX` should fail. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h]
- `cmpctblock` with no short IDs and no prefilled transactions should fail structural validation before partial state. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp]
- `cmpctblock` prefilled differential expansion that overflows, exceeds `u16::MAX`, or points beyond `short_ids.len() + inserted_prefilled_count` should fail. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp, packages/bitcoin-knots/src/blockencodings.h]
- Prefilled transactions that decode to empty-input or empty-output transactions should be treated as structurally invalid for compact-block acceptance if the planner maps Knots `tx->IsNull()` to Open Bitcoin's existing transaction checks. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp, packages/open-bitcoin-consensus/src/transaction.rs]
- Superfluous witness records should fail through the existing transaction parser. [VERIFIED: packages/open-bitcoin-codec/src/transaction.rs]
- `getblocktxn` indexes whose differential expansion overflows or exceeds `u16::MAX` should fail. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h]
- `blocktxn` payloads with truncated transactions, non-canonical transaction vector count, superfluous witness records, or trailing bytes should fail. [VERIFIED: packages/open-bitcoin-codec/src/transaction.rs, packages/open-bitcoin-codec/src/compact_size.rs, packages/open-bitcoin-codec/src/primitives.rs]

## Code Examples

Verified patterns from local and official sources. [VERIFIED: packages/open-bitcoin-codec/src, packages/bitcoin-knots/src/blockencodings.h]

### Six-Byte Short ID Newtype

```rust
// Source: BIP152 HeaderAndShortIDs and Knots CustomUintFormatter<6>.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortId([u8; 6]);

impl ShortId {
    pub const fn from_wire_bytes(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    pub const fn as_wire_bytes(&self) -> &[u8; 6] {
        &self.0
    }
}
```

### Message Enum Wiring

```rust
// Source: packages/open-bitcoin-network/src/message.rs command mapping pattern.
match command.as_str() {
    "sendcmpct" => Ok(Self::SendCompact(decode_send_compact_payload(payload)?)),
    "cmpctblock" => Ok(Self::CompactBlock(decode_compact_block_payload(payload)?)),
    "getblocktxn" => Ok(Self::GetBlockTxn(decode_get_block_transactions_payload(payload)?)),
    "blocktxn" => Ok(Self::BlockTxn(decode_block_transactions_payload(payload)?)),
    other => Err(NetworkError::UnknownCommand(other.to_string())),
}
```

### Structural Compact Block Validation

```rust
// Source: packages/bitcoin-knots/src/blockencodings.cpp PartiallyDownloadedBlock::InitData.
fn validate_compact_block_structure(payload: &CompactBlockPayload) -> Result<(), CodecError> {
    if payload.short_ids.is_empty() && payload.prefilled_transactions.is_empty() {
        return Err(CodecError::CompactBlockEmpty);
    }

    let implied_count = payload
        .short_ids
        .len()
        .checked_add(payload.prefilled_transactions.len())
        .ok_or(CodecError::CompactBlockTransactionCountOverflow)?;
    if implied_count > u16::MAX as usize {
        return Err(CodecError::CompactBlockTransactionCountOverflow);
    }

    expand_prefilled_positions(payload)?;
    Ok(())
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| BIP152 version 1 transaction encoding without witness data. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | BIP152 version 2 uses witness serialization and wtxid short-ID inputs. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | BIP152 version 2 is documented in the BIP after version 1 and Knots pins `CMPCTBLOCKS_VERSION` to 2. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] | Phase 112 should implement version 2 payload round trips with witness transactions and leave version negotiation policy to Phase 113. [VERIFIED: 112-CONTEXT.md] |
| Compact-block command strings unknown to Open Bitcoin. [VERIFIED: packages/open-bitcoin-network/src/message.rs] | Four explicit `WireNetworkMessage` variants for `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn`. [VERIFIED: 112-CONTEXT.md] | Phase 112 scope. [VERIFIED: .planning/ROADMAP.md] | Unknown-command fallback remains for other unsupported commands such as `addrv2` and `sendaddrv2`. [VERIFIED: packages/open-bitcoin-network/src/message/tests.rs] |
| Compact block inventory type exists but compact serving remains suppressed. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs, 112-CONTEXT.md] | Payload codec exists without changing `getdata` compact-block serving behavior. [VERIFIED: 112-CONTEXT.md] | Phase 112 scope after Phase 111. [VERIFIED: .planning/ROADMAP.md, .planning/STATE.md] | Runtime compact-serving behavior remains deferred. [VERIFIED: 112-CONTEXT.md] |

**Deprecated/outdated:**
- Treating BIP152 version 1 as the only supported semantic is outdated for this repo's pinned Knots baseline because Knots uses `CMPCTBLOCKS_VERSION{2}` and Phase 112 requires version 2 round trips. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp, 112-CONTEXT.md]
- Planning public-network compact-relay CI as a default gate is out of scope for v2.1 deterministic verification. [VERIFIED: .planning/REQUIREMENTS.md, .planning/STATE.md, 112-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Inline all BIP152 helpers inside `open-bitcoin-network/src/message.rs` is less maintainable than a codec module. | Standard Stack / Alternatives Considered | Planner could choose inline helpers and later need to split the file if it grows or becomes hard to test. |
| A2 | Adding an external Rust Bitcoin codec would be the likely alternative to first-party codec work. | Standard Stack / Alternatives Considered | Low risk because repo policy forbids production Rust Bitcoin libraries regardless of the specific alternative. |
| A3 | Public raw `u64` short IDs increase the chance of accidental 8-byte emission. | Don't Hand-Roll | Medium risk if planner chooses a checked `u64` newtype instead; the invariant can still be safe if constructors and encoders enforce 48 bits. |
| A4 | Ad hoc unchecked arithmetic is the likely failure mode for differential indexes. | Don't Hand-Roll | Medium risk if implementation already centralizes arithmetic; planner should still require checked helper tests. |
| A5 | Manual block-header serialization in network code is a likely tempting shortcut. | Don't Hand-Roll | Low risk because existing helpers are straightforward to reuse. |

## Open Questions (RESOLVED)

1. **RESOLVED: Compact-block structural validation must not add an `open-bitcoin-codec` dependency on `open_bitcoin_consensus::check_transaction` in Phase 112.**
   - What we know: Knots rejects `tx->IsNull()` before partial compact-block state and Open Bitcoin has `check_transaction` errors for empty inputs and empty outputs. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp, packages/open-bitcoin-consensus/src/transaction.rs]
   - Resolution: Keep byte decode and wire-shape errors in `open-bitcoin-codec`; place compact-block structural acceptance in `open-bitcoin-network` or another pure helper that can reuse existing transaction checks without changing low-level codec dependencies. [VERIFIED: standards/core/architecture.md, packages/open-bitcoin-codec/Cargo.toml, packages/open-bitcoin-network/Cargo.toml]
   - Planning consequence: Plans should require byte-level decode tests in `open-bitcoin-codec` and structural compact-block validation tests at the network/message boundary before any future partial reconstruction state exists.

2. **RESOLVED: Decoded BIP152 models should preserve wire deltas and provide checked absolute-index expansion helpers.**
   - What we know: Context allows exposing wire deltas for Phase 112, while Knots converts deltas into absolute indexes for lookup helpers. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/test/functional/test_framework/messages.py]
   - Resolution: Store wire deltas in payload structs so round trips remain byte-faithful, and add checked expansion helpers that reject overflow and values above `u16::MAX`. [VERIFIED: 112-CONTEXT.md, packages/bitcoin-knots/src/blockencodings.h]
   - Planning consequence: Plans should require tests for both byte-preserving differential encoding and absolute expansion failure cases.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust compiler | Rust codec implementation and tests [VERIFIED: packages/Cargo.toml, scripts/verify.sh] | yes [VERIFIED: environment probe] | `rustc 1.94.1` [VERIFIED: environment probe] | none needed [VERIFIED: rust-toolchain.toml] |
| Cargo | Workspace build, clippy, and tests [VERIFIED: scripts/verify.sh] | yes [VERIFIED: environment probe] | `cargo 1.94.1` [VERIFIED: environment probe] | none needed [VERIFIED: scripts/verify.sh] |
| Bun | TypeScript checker scripts and LOC/parity checks [VERIFIED: AGENTS.md, scripts/verify.sh] | yes [VERIFIED: environment probe] | `1.3.9` [VERIFIED: environment probe, .bun-version] | avoid checker changes if Bun becomes unavailable [VERIFIED: scripts/verify.sh] |
| Bazel | Full verifier smoke build [VERIFIED: scripts/verify.sh] | yes [VERIFIED: environment probe] | `8.6.0` [VERIFIED: environment probe] | `bash scripts/verify.sh --fast` for iteration only [VERIFIED: AGENTS.md, scripts/verify.sh] |
| `cargo-llvm-cov` | Full verifier coverage [VERIFIED: scripts/verify.sh] | yes [VERIFIED: environment probe] | `0.8.5` [VERIFIED: environment probe] | `bash scripts/verify.sh --fast` for iteration only [VERIFIED: scripts/verify.sh] |
| Bitcoin Knots submodule | Parity anchors and fixtures [VERIFIED: AGENTS.md, 112-CONTEXT.md] | yes [VERIFIED: environment probe] | `v29.3.knots20260210` at `a9aee730466ac67d35a3c03ee24676be5e045878` [VERIFIED: environment probe] | run `git submodule update --init --recursive` if missing [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:**
- None found. [VERIFIED: environment probe]

**Missing dependencies with fallback:**
- None found. [VERIFIED: environment probe]

## Focused Verification Commands

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec compact_block` should be the focused codec test shape if the new module is named `compact_block`. [VERIFIED: packages/Cargo.toml] [ASSUMED]
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message` should cover message enum and command integration tests. [VERIFIED: packages/Cargo.toml, packages/open-bitcoin-network/src/message/tests.rs] [ASSUMED]
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-codec -p open-bitcoin-network --all-targets --all-features -- -D warnings` is a focused lint command for the two likely affected crates. [VERIFIED: scripts/verify.sh, packages/Cargo.toml]
- `bash scripts/verify.sh --fast` is appropriate for local iteration because repo guidance reserves the default verifier for final completion and `--fast` skips heavier full-only steps. [VERIFIED: AGENTS.md, scripts/verify.sh]
- `bash scripts/verify.sh` is the final repo-native verification contract for first-party code, including Bazel smoke build and coverage path. [VERIFIED: AGENTS.md, scripts/verify.sh]

## Parity Breadcrumb And Documentation Updates

- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumbs in file comments and entries in `docs/parity/source-breadcrumbs.json`. [VERIFIED: AGENTS.md, docs/parity/source-breadcrumbs.json]
- Likely breadcrumbs for new BIP152 codec files are `packages/bitcoin-knots/src/protocol.h`, `packages/bitcoin-knots/src/blockencodings.h`, `packages/bitcoin-knots/src/blockencodings.cpp`, `packages/bitcoin-knots/src/primitives/transaction.h`, `packages/bitcoin-knots/test/functional/test_framework/messages.py`, and `packages/bitcoin-knots/test/functional/p2p_compactblocks.py`. [VERIFIED: 112-CONTEXT.md, local files]
- If Phase 112 updates parity documentation, add or update a `docs/parity/index.json` catalog entry for BIP152 wire payload support and document that runtime negotiation/reconstruction/fallback remain deferred. [VERIFIED: AGENTS.md, docs/parity/index.json, 112-CONTEXT.md]
- If Phase 112 adds a deterministic no-claim checker, wire it into `scripts/verify.sh` near prior phase checkers and add mutation tests with Bun. [VERIFIED: scripts/verify.sh, .planning/phases/111-full-block-serving-request-path/111-03-PLAN.md]
- `docs/metrics/lines-of-code.md` is tracked generated output and may change when verification regenerates it. [VERIFIED: AGENTS.md, scripts/verify.sh]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no [VERIFIED: 112-CONTEXT.md] | No authentication surface is in scope. [VERIFIED: 112-CONTEXT.md] |
| V3 Session Management | no [VERIFIED: 112-CONTEXT.md] | No session surface is in scope. [VERIFIED: 112-CONTEXT.md] |
| V4 Access Control | no [VERIFIED: 112-CONTEXT.md] | Peer eligibility and compact announcement policy are deferred. [VERIFIED: 112-CONTEXT.md] |
| V5 Input Validation | yes [VERIFIED: 112-CONTEXT.md] | Parse raw payloads into typed domain values, reject malformed CompactSize, EOF, trailing data, width, overflow, and structural compact-block errors before partial state. [VERIFIED: standards/core/architecture.md, packages/open-bitcoin-codec/src/error.rs, packages/bitcoin-knots/src/blockencodings.cpp] |
| V6 Cryptography | limited [VERIFIED: 112-CONTEXT.md, .planning/ROADMAP.md] | Do not add short-ID SipHash computation in Phase 112 unless needed for fixtures; Phase 114 owns short-ID helper and reconstruction. [VERIFIED: .planning/ROADMAP.md, 112-CONTEXT.md] |

### Known Threat Patterns for BIP152 Codec

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Malformed payload resource pressure through huge CompactSize counts | Denial of Service [ASSUMED] | Reuse `read_compact_size` `MAX_SIZE` enforcement and add BIP152-specific count bounds before allocation. [VERIFIED: packages/open-bitcoin-codec/src/compact_size.rs, packages/bitcoin-knots/src/blockencodings.h] |
| Differential index overflow or wraparound | Tampering [ASSUMED] | Checked arithmetic modeled on Knots `DifferenceFormatter`, plus tests for overflow and `u16::MAX` boundaries. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h] |
| Short ID width confusion | Tampering [ASSUMED] | Six-byte newtype and exact-width reader/writer tests. [VERIFIED: packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/test/functional/test_framework/messages.py] |
| Partial-state acceptance from invalid compact block | Tampering / Denial of Service [ASSUMED] | Pure structural validation before any reconstruction state is initialized. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp, 112-CONTEXT.md] |
| Witness stripping in version 2 payloads | Tampering [ASSUMED] | Use `TransactionEncoding::WithWitness` for `cmpctblock` prefilled transactions and `blocktxn`. [VERIFIED: 112-CONTEXT.md, packages/open-bitcoin-codec/src/transaction.rs] [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] |

## Sources

### Primary (HIGH confidence)

- `AGENTS.md` - repo-local verification, Knots submodule, parity breadcrumb, dependency policy, functional-core, and docs freshness guidance. [VERIFIED]
- `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/rust.md` - Bright Builds architecture, testing, verification, and Rust rules. [VERIFIED]
- `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-CONTEXT.md` - locked decisions, phase boundary, canonical refs, and deferred scope. [VERIFIED]
- `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md` - Phase 112 requirement mapping, success criteria, and milestone state. [VERIFIED]
- `packages/bitcoin-knots/src/protocol.h` - BIP152 command names and `MSG_CMPCT_BLOCK` inventory constant. [VERIFIED]
- `packages/bitcoin-knots/src/blockencodings.h` - `DifferenceFormatter`, `BlockTransactionsRequest`, `BlockTransactions`, `PrefilledTransaction`, `CBlockHeaderAndShortTxIDs`, and short ID width. [VERIFIED]
- `packages/bitcoin-knots/src/blockencodings.cpp` - compact-block construction and structural checks before partial reconstruction state. [VERIFIED]
- `packages/bitcoin-knots/src/net_processing.cpp` - Knots compact-block version 2 constant and runtime peer-policy examples. [VERIFIED]
- `packages/bitcoin-knots/test/functional/test_framework/messages.py` - Python wire fixture shapes for BIP152 messages. [VERIFIED]
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` - version handling, malformed compact block, and compact-block behavior examples. [VERIFIED]
- `packages/open-bitcoin-codec/src/block.rs`, `compact_size.rs`, `transaction.rs`, `primitives.rs`, `error.rs`, and `lib.rs` - existing codec helpers and error surface. [VERIFIED]
- `packages/open-bitcoin-network/src/message.rs`, `message/tests.rs`, and `error.rs` - existing message enum, payload mapping, wire framing, tests, and network errors. [VERIFIED]
- `packages/open-bitcoin-primitives/src/network.rs`, `transaction.rs` - inventory type, message command, transaction primitive, coinbase/null-output helpers. [VERIFIED]
- `packages/open-bitcoin-consensus/src/lib.rs`, `transaction.rs` - existing block weight constants and transaction structural checks. [VERIFIED]
- `scripts/verify.sh` - repo-native verification contract. [VERIFIED]
- `https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki` - BIP152 deployed spec, wire shapes, and version 2 witness semantics. [CITED]

### Secondary (MEDIUM confidence)

- None needed; primary local and official sources covered the phase domain. [VERIFIED: source review]

### Tertiary (LOW confidence)

- None used as authoritative evidence. [VERIFIED: source review]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - The phase uses existing first-party crates and pinned local tools, all verified from manifests and environment probes. [VERIFIED: packages/Cargo.toml, environment probe]
- Architecture: HIGH - Existing module boundaries and locked context agree on `open-bitcoin-codec` plus `open-bitcoin-network::message`. [VERIFIED: 112-CONTEXT.md, packages/open-bitcoin-codec/src/lib.rs, packages/open-bitcoin-network/src/message.rs]
- Serialization shapes: HIGH - BIP152, Knots headers, Knots Python fixtures, and local codec helpers agree on the wire payloads. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] [VERIFIED: packages/bitcoin-knots/src/protocol.h, packages/bitcoin-knots/src/blockencodings.h, packages/bitcoin-knots/test/functional/test_framework/messages.py]
- Malformed semantics: MEDIUM-HIGH - Knots structural checks are verified, but exact Open Bitcoin `CodecError` names and the placement of transaction structural validation remain planner decisions. [VERIFIED: packages/bitcoin-knots/src/blockencodings.cpp, packages/open-bitcoin-codec/src/error.rs, Open Questions]
- Verification: HIGH - Repo-native verifier and focused crate commands are derived from existing scripts and workspace manifests. [VERIFIED: scripts/verify.sh, packages/Cargo.toml]

**Research date:** 2026-07-04 [VERIFIED: system date]
**Valid until:** 2026-08-03 for local architecture and pinned Knots baseline; re-check tools if the workspace or `rust-toolchain.toml` changes. [ASSUMED]
