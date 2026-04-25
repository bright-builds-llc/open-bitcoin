---
phase: 02
slug: core-domain-and-serialization-foundations
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-25
updated: 2026-04-25T12:43:12Z
---

# Phase 02 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Standards And Guidance Reviewed

This audit was materially informed by repo-local guidance in `AGENTS.md`, the Bright Builds sidecar in `AGENTS.bright-builds.md`, `standards-overrides.md`, and the pinned Bright Builds architecture, verification, testing, and Rust standards. The relevant checks were functional-core boundaries, parse-at-boundaries domain types, illegal-state modeling, pure-core test coverage, and repo-native pre-commit verification.

## Trust Boundaries

| Boundary | Description | Data Crossing |
| --- | --- | --- |
| TB-02-01 | Raw byte and primitive inputs enter first-party domain types consumed by later consensus and wallet code. | Serialized Bitcoin data, amounts, hashes, scripts, transactions, blocks, and message fields |
| TB-02-02 | Workspace/package topology becomes the enforceable pure-core dependency boundary for later crates. | Cargo members, Bazel targets, pure-core allowlist, and umbrella re-exports |
| TB-02-03 | Untrusted serialized bytes become first-party parsed structs. | CompactSize values, transactions, blocks, message headers, inventory vectors, locators, and addresses |
| TB-02-04 | First-party domain types serialize to byte outputs later consumed by peers, fixtures, or adapters. | Wire-format transaction, block, and network bytes |
| TB-02-05 | Upstream vectors become first-party copied or derived fixture files. | Knots source/test corpora and checked-in Open Bitcoin hex fixtures |
| TB-02-06 | Repo-native verification becomes contributor confidence in pure-core parity claims. | Test, coverage, Bazel smoke build, and architecture-policy results |
| TB-02-07 | Upstream baseline files and tests become contributor-facing catalog assertions. | Knots source/test references, quirks, known-bug status, and suspected unknowns |
| TB-02-08 | First-party implementation status becomes parity index and catalog status fields. | `docs/parity/index.json` and catalog document metadata |

## Threat Register

| Threat ID | Category | Component | Disposition | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| T-02-01 | Tampering | Package membership, pure-core allowlist, and crate re-exports | mitigate | closed | `packages/Cargo.toml` registers `open-bitcoin-primitives` and `open-bitcoin-codec`; `BUILD.bazel` exposes root aliases for both; `scripts/pure-core-crates.txt` includes both crates; `packages/open-bitcoin-core/src/lib.rs` re-exports both crates. |
| T-02-02 | Integrity | Invariant-bearing domain constructors | mitigate | closed | `Amount::from_sats` rejects values outside `0..=MAX_MONEY`; `Hash32::from_slice` and semantic hash wrappers enforce 32-byte widths; `ScriptBuf::from_bytes` enforces `MAX_SCRIPT_SIZE`; `MessageCommand::new` and `from_wire_bytes` reject NUL, non-ASCII, overlong, and malformed padding cases; unit tests cover rejection paths. |
| T-02-03 | Integrity | CompactSize, witness flags, and transaction/block byte order | mitigate | closed | `read_compact_size` rejects non-canonical values and sizes above `MAX_SIZE`; the reader/writer helpers encode fixed endianness; transaction parsing keeps witness mode explicit, rejects unknown flags, and rejects superfluous witness records; transaction and block tests include round trips plus malformed-input rejection. |
| T-02-04 | Tampering | Network message headers and foundational payloads | mitigate | closed | Message-header parsing reads fixed field widths, validates 12-byte command padding through `MessageCommand::from_wire_bytes`, rejects trailing bytes via `Reader::finish`, and has fixture-backed round-trip and rejection tests for malformed command bytes. |
| T-02-05 | Integrity | Fixture provenance and byte-level expectations | mitigate | closed | Checked-in fixtures exist under `packages/open-bitcoin-codec/testdata/`; transaction, block-header, and message-header tests include the fixture files and re-serialize byte-for-byte; the parity catalog cites concrete Knots source and test paths; this report records per-fixture provenance below. |
| T-02-06 | Repudiation | Pure-core coverage gate | mitigate | closed | `scripts/verify.sh` derives coverage packages from `scripts/pure-core-crates.txt`, runs the pure-core dependency/import gate, Rust format/lint/build/test checks, Bazel smoke build, and `cargo llvm-cov` with missing-line failure behavior. |
| T-02-07 | Repudiation | Subsystem catalog provenance | mitigate | closed | `docs/parity/catalog/core-domain-and-serialization.md` cites Knots source/test paths for amounts, identifiers, serialization primitives, scripts, transactions, blocks, and protocol framing, and records quirks plus suspected unknowns. |
| T-02-08 | Integrity | Root parity index status fields | mitigate | closed | `docs/parity/index.json` links the `core-domain-and-serialization` catalog entry, records the Phase 02 status as `seeded`, and mirrors source/test/highlight metadata for the catalog document. |

Status legend: `closed` means mitigation found, accepted risk documented, or transfer documented. No threats remain open.

## Fixture Provenance Notes

| Fixture | Provenance | Byte-Exact Check |
| --- | --- | --- |
| `packages/open-bitcoin-codec/testdata/transaction_valid.hex` | Genesis-style transaction fixture derived from the Knots transaction/vector corpus named in `docs/parity/catalog/core-domain-and-serialization.md` and Phase 02 context: `packages/bitcoin-knots/src/test/transaction_tests.cpp`, `packages/bitcoin-knots/src/test/data/tx_valid.json`, and related transaction/script vector files. | `packages/open-bitcoin-codec/src/transaction.rs` includes the fixture and asserts re-encoding equals the original bytes. |
| `packages/open-bitcoin-codec/testdata/block_header.hex` | Genesis block-header fixture derived from the block/header surfaces named in the Phase 02 catalog: `packages/bitcoin-knots/src/primitives/block.h` and upstream deserialize fuzz coverage. | `packages/open-bitcoin-codec/src/block.rs` includes the fixture and asserts re-encoding equals the original bytes. |
| `packages/open-bitcoin-codec/testdata/message_header.hex` | Message-header fixture derived from the protocol framing surface named in the Phase 02 catalog: `packages/bitcoin-knots/src/protocol.h` and `packages/bitcoin-knots/src/test/fuzz/protocol.cpp`. | `packages/open-bitcoin-codec/src/network.rs` includes the fixture and asserts re-encoding equals the original bytes. |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
| --- | --- | --- | --- | --- |
| 2026-04-25 | 8 | 8 | 0 | Codex |

## Verification Evidence

| Check | Evidence |
| --- | --- |
| Phase verification | `.planning/phases/02-core-domain-and-serialization-foundations/02-VERIFICATION.md` reports Phase 02 as passed with 4/4 must-haves verified. |
| UAT | `.planning/phases/02-core-domain-and-serialization-foundations/02-UAT.md` records 5 passed tests, 0 issues, 0 skipped, and 0 blocked. |
| Pre-commit verification contract | Run `cargo fmt --manifest-path packages/Cargo.toml --all`, `cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings`, `cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`, `cargo test --manifest-path packages/Cargo.toml --workspace --all-features`, and `bash scripts/verify.sh` before committing this report. |

## Sign-Off

- [x] All threats have a disposition: mitigate, accept, or transfer.
- [x] Accepted risks documented in Accepted Risks Log.
- [x] `threats_open: 0` confirmed.
- [x] `status: verified` set in frontmatter.

Approval: verified 2026-04-25.
