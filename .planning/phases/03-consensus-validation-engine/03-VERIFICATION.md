---
phase: 03-consensus-validation-engine
verified: 2026-04-11T23:14:00.000Z
status: diagnosed
score: 3/4 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 3-2026-04-11T21-32-30
generated_at: 2026-04-11T23:14:00.000Z
lifecycle_validated: false
---

# Phase 3: Consensus Validation Engine Verification Report

**Phase Goal:** Implement script, transaction, and block validation behavior that matches the pinned baseline for consensus-critical decisions.  
**Verified:** 2026-04-11T23:14:00.000Z  
**Status:** diagnosed

## Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Open Bitcoin has a pure-core consensus crate with deterministic hashing, script evaluation, typed transaction and block outcomes, and explicit validation contexts. | ✓ VERIFIED | `packages/open-bitcoin-consensus/src/lib.rs`, `context.rs`, `script.rs`, `transaction.rs`, `block.rs`, and `validation.rs` |
| 2 | Phase 3 now covers context-free plus contextual transaction and block checks, including coinbase maturity, finality, sequence locks, coinbase-height prefix checks, witness commitments, and block-weight validation. | ✓ VERIFIED | `validate_transaction_with_context`, `check_tx_inputs`, `check_block_contextual`, `validate_block_with_context`, and their direct tests |
| 3 | Deterministic script, transaction, block, contextual, and witness fixtures can fail the repo-native pure-core gate when the implemented consensus slice regresses. | ✓ VERIFIED | `bash scripts/verify.sh` passes, including the 100% pure-core line-coverage gate |
| 4 | The full in-scope baseline consensus surface is implemented, including signature opcodes, legacy or segwit sighash, P2SH, segwit program execution, and taproot paths. | ✗ GAP | `script.rs` still treats `CHECKSIG` and `CHECKMULTISIG` as unsupported, and there is no full signature, P2SH, segwit-program, or taproot execution path yet |

## Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `packages/open-bitcoin-consensus/` | Pure-core consensus crate | ✓ EXISTS + SUBSTANTIVE | Contains hashing, script, transaction, and block validation modules |
| `docs/parity/catalog/consensus-validation.md` | Phase 3 parity ledger entry | ✓ EXISTS + SUBSTANTIVE | Documents implemented surfaces and deferred consensus gaps |
| `03-CONTEXT.md` + `03-01`..`03-04` plans | Phase 3 lifecycle context and plans | ✓ EXISTS | Phase directory contains context, research, and plan artifacts for the current yolo lifecycle |

## Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `packages/Cargo.toml` | `packages/open-bitcoin-consensus` | workspace member | ✓ WIRED | Cargo workspace includes the new pure-core crate |
| `packages/open-bitcoin-core/src/lib.rs` | `open-bitcoin-consensus` | `pub use` re-export | ✓ WIRED | Downstream pure-core/runtime code can import consensus through `open-bitcoin-core` |
| `validate_transaction` | `verify_script` | explicit API call | ✓ WIRED | Transaction validation reuses the script engine for provided prevouts |
| `check_block_header` | `check_proof_of_work` | compact-target decode + hash compare | ✓ WIRED | Block-header validation fails on invalid targets or high hashes |

## Requirements Coverage

| Requirement | Status | Blocking Issue |
|-------------|--------|----------------|
| `CONS-02`: The node validates scripts, transactions, and blocks with consensus behavior matching the pinned baseline. | DIAGNOSED GAP | Signature opcodes, sighash semantics, P2SH, segwit-program execution, and taproot paths are still missing |
| `CONS-03`: Automated fixtures surface any consensus mismatch with the baseline before merge. | PARTIAL | The fixture suite now covers the implemented context-free and contextual slice, but it does not yet cover the full in-scope signature and witness surface |

## Gaps Summary

Phase 3 has a solid pure-core foundation, but it is **not clean enough to mark
complete or push under the strict wrapper gate**. The remaining gaps are:

1. Signature opcode execution and signature-hash semantics
2. P2SH, segwit-program, and taproot execution paths
3. Completing the parity fixture corpus for the still-missing signature and witness surface

## Verification Metadata

**Automated checks run so far:** `bash scripts/verify.sh`  
**Lifecycle validation:** not yet ready to pass because the phase still has known consensus gaps  
**Next action:** continue Phase 3 rather than commit/push through the strict gate
