---
phase: 03
slug: consensus-validation-engine
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-25
updated: 2026-04-25
---

# Phase 03 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 03 delivers the pure-core consensus validation foundation: deterministic
hashing, typed transaction and block validation outcomes, contextual consensus
checks, script classification, signature-verification plumbing, and parity
evidence for the narrowed Phase 03 surface.

No explicit `<threat_model>` block or `## Threat Flags` entries were present in
the Phase 03 plan and summary artifacts. This report therefore records an empty
declared threat register and verifies the security-relevant controls implied by
the phase artifacts and repo guardrails.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Bitcoin payloads to typed domain values | Raw blocks, transactions, scripts, witnesses, and headers are parsed by earlier domain and codec layers before consensus code evaluates them. | Untrusted network, disk, fixture, or test bytes become typed `open-bitcoin-primitives` values. |
| Typed domain values to pure consensus validation | Phase 03 consensus APIs accept typed transactions, blocks, spent-output views, validation contexts, and consensus parameters without direct chainstate, network, filesystem, or runtime side effects. | Transaction and block structures, spent outputs, heights, median time past, verify flags, and consensus params. |
| Consensus validation to cryptographic verification | Signature checks cross from repo-owned validation logic into the minimal `secp256k1` dependency. | Public keys, signatures, signature-hash digests, and verification flags. |
| Consensus status to parity documentation | Phase 03 verification and parity artifacts communicate implemented scope, deferred follow-on work at the time of Phase 03, and later parity status to contributors and reviewers. | Human-readable evidence, machine-readable parity entries, and known-gap status. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| _none declared_ | - | Phase 03 artifacts | - | No explicit Phase 03 threat model or summary threat flags were found. Security-relevant controls verified below. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| Pure-core isolation | `scripts/pure-core-crates.txt` includes `open-bitcoin-consensus`; `scripts/check-pure-core-deps.sh` rejects runtime, network, filesystem, process, thread, random, and TLS dependencies or imports for pure-core crates. | verified |
| Unsafe-code and panic-like production paths | `packages/open-bitcoin-consensus/src/lib.rs` forbids unsafe code and denies unwrap, expect, panic, unreachable, todo, unimplemented, and panic-in-result functions outside tests. `scripts/check-panic-sites.sh` scans production Rust sources for unclassified panic-like sites. | verified |
| Typed validation outcomes | `transaction.rs`, `block.rs`, and `validation.rs` return typed validation errors and Knots-style reject reasons instead of ad-hoc panics or boolean-only failures. | verified |
| Explicit contextual boundaries | `TransactionValidationContext`, `TransactionInputContext`, `SpentOutput`, and `BlockValidationContext` make chainstate-derived inputs explicit, keeping validation pure and reviewable. | verified |
| Consensus resource limits | Transaction and block validation enforce size, weight, sigop-cost, money-range, coinbase, duplicate-input, merkle-mutation, finality, sequence-lock, proof-of-work, reward-limit, and witness-commitment checks through typed results. | verified |
| Cryptographic dependency minimization | `open-bitcoin-consensus` depends only on repo-local crates plus `secp256k1 = "0.31"` with default features disabled and `std` enabled; Cargo and Bazel both wire the same dependency. | verified |
| Honest parity and scope signaling | Phase 03 verification records the narrowed foundation scope and follow-on consensus work; the parity ledger remains the contributor-facing source of truth for current in-scope consensus status. | verified |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-25 | 0 | 0 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| `rg -n "threat_model|Threat Flags|threat|Threat|security|Security|mitigation|trust boundary|boundary" .planning/phases/03-consensus-validation-engine/*-PLAN.md .planning/phases/03-consensus-validation-engine/*-SUMMARY.md` | No explicit Phase 03 threat model or summary threat flags found. |
| `rg -n "unsafe|unwrap\\(|expect\\(|panic!|unreachable!|todo!|unimplemented!|std::fs|std::net|std::env|std::process|std::thread|tokio|reqwest|rustls|\\brand\\b" packages/open-bitcoin-consensus/src packages/open-bitcoin-core/src packages/open-bitcoin-consensus/Cargo.toml packages/open-bitcoin-consensus/BUILD.bazel MODULE.bazel` | Production controls and dependency surfaces reviewed; panic-like findings are test-only or governed by the production guard script. |
| `bash scripts/check-pure-core-deps.sh` | Passed: pure-core dependency and import checks passed. |
| `bash scripts/check-panic-sites.sh` | Passed: no unclassified production panic-like sites. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus` | Passed: 147 unit tests, 3 parity-closure tests, 2 taproot tests, and doc-tests passed. |
| `git diff --check` | Passed. |

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-04-25
