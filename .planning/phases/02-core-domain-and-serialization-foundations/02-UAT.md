---
status: complete
phase: 02-core-domain-and-serialization-foundations
source:
  - .planning/phases/02-core-domain-and-serialization-foundations/02-01-SUMMARY.md
  - .planning/phases/02-core-domain-and-serialization-foundations/02-02-SUMMARY.md
  - .planning/phases/02-core-domain-and-serialization-foundations/02-03-SUMMARY.md
  - .planning/phases/02-core-domain-and-serialization-foundations/02-04-SUMMARY.md
started: 2026-04-25T12:27:04Z
updated: 2026-04-25T12:36:34Z
---

## Current Test

[testing complete]

## Tests

### 1. Pure-Core Crate Wiring
expected: From the repo root, `cargo metadata --manifest-path packages/Cargo.toml --format-version 1 --no-deps` shows `open-bitcoin-primitives` and `open-bitcoin-codec` as first-party workspace crates, and `packages/open-bitcoin-core/src/lib.rs` re-exports them for downstream consumers.
result: pass

### 2. Invariant-Bearing Primitives
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-primitives` passes, covering checked amounts, fixed-width hashes, script and witness containers, transaction and block structures, and foundational network domain types.
result: pass

### 3. Lossless Codec Round Trips
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec` passes, including CompactSize canonical encoding checks, transaction and block encode/decode round trips, witness-aware transaction serialization, message-header framing, inventory vectors, block locators, and EOF/trailing-data rejection paths.
result: pass

### 4. Pure-Core Verification Gate
expected: `bash scripts/verify.sh` succeeds and derives the pure-core coverage package list from `scripts/pure-core-crates.txt`, so the Phase 2 primitives and codec crates remain inside the 100% pure-core line-coverage gate.
result: pass

### 5. Parity Catalog Seed
expected: `docs/parity/catalog/core-domain-and-serialization.md` documents the Phase 2 domain and serialization surfaces with Knots source/test references, quirks, known-bug status, and suspected unknowns, and `docs/parity/index.json` links that catalog entry from the machine-readable root.
result: pass

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
