---
phase: 165-shared-exact-work-authority
plan: "01"
subsystem: consensus
tags: [proof-work, compact-target, arithmetic, parity]
requires:
  - phase: 160
    provides: Existing first-party exact ProofWork arithmetic and proof-equivalent time
provides:
  - Shared normalized compact validity for proof and hash admission
  - Selected-consensus powLimit and hash admission APIs
  - Lossless 32-byte LE and fixed-width unsigned hex ProofWork boundaries
  - Independently expected fixed wide proof, carry, wrap and refusal corpus
affects: [165-02, 165-03, 165-04, 165-05, 165-12]
tech-stack:
  added: []
  patterns: [Pure arithmetic, explicit numeric import without provenance]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/crypto/tests/phase165_tests.rs
    - packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs
    - docs/parity/fixtures/phase165-work-vectors.json
  modified:
    - packages/open-bitcoin-consensus/src/crypto.rs
    - packages/open-bitcoin-consensus/src/block.rs
    - packages/open-bitcoin-consensus/src/chain_proof.rs
    - packages/open-bitcoin-consensus/src/chain_proof/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Normalize small-exponent mantissas before zero and sign classification."
  - "Expose selected-limit admission explicitly while preserving existing generic API signatures."
  - "Imported ProofWork is a number only; byte and hex parsing mint no authority."
patterns-established:
  - "One normalized decoder feeds block proof and selected target/hash admission."
  - "Fixed corpus expectations use pinned source formulas independently of production helpers."
requirements-completed: [CWOR-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-14T16:59:22Z
duration: 8min
completed: 2026-10-14
---

# Phase 165 Plan 01: Shared Compact Validity and Exact Work Boundaries Summary

**Normalized compact admission and lossless unsigned 256-bit work boundaries with independently expected wide arithmetic vectors.**

## Performance

- Implementation start: 2026-10-14T16:49:32Z, measured from the first new test file's creation time.
- Final focused verification: 2026-10-14T16:57:49Z.
- Duration: approximately eight minutes of implementation and verification; preliminary context loading was not separately timed.
- Tasks: 2/2.
- Source, test, breadcrumb and fixture files created/modified: 8.
- Final verification commands used Rust 1.94.1 and the selected Bun 1.3.9 executable.

## Accomplishments

- Tiny positive targets truncated to zero and signed negative-zero now return `CompactTargetError::Zero`. Positive equivalent encodings remain valid. Normalized sign and exponent 32/33/34 overflow boundaries are tested across decoding, proof and admission.
- `crypto::check_proof_of_work_with_params(hash, bits, &ConsensusParams)` validates the selected compact powLimit and hash. `block::check_block_header_with_params(&BlockHeader, &ConsensusParams)` applies existing header error mapping. Equality succeeds; targets or hashes above their limits refuse.
- Existing `ProofWork` exposes `from_le_bytes([u8; 32])`, `to_le_bytes()`, `from_hex(&str)`, `to_hex()`, `Display` and `FromStr`. Hex is exactly 64 unsigned ASCII digits, most significant first; uppercase input is accepted and lowercase output is stable. Prefixes, signs, whitespace, Unicode and malformed width refuse.
- Fixed source-derived corpus contains 11 proof/target vectors, 6 cumulative-addition vectors and 14 invalid compact vectors. It includes proof(0x01010000)=2^255, proof(0x01020000)=64 hex digits of 5, mainnet proof 0x100010001, carry into and above bit128, mixed targets and modulo-2^256 wrap.
- Existing proof-equivalent-time regressions remain GREEN. The simplification pass reused the byte import in block proof and removed its now-redundant zero check; no parallel arithmetic engine or new dependency was added.

## Task Commits

Task 1, Task 2, RED/GREEN/REFACTOR commits and this summary's metadata commit are **deferred to the parent strict finalization wrapper**, as required by the execution contract. No files were staged, no commits or pushes were created, and no commit hashes or hook receipts are asserted here. Shared STATE.md, ROADMAP.md, REQUIREMENTS.md, config and todo were not edited by this executor.

## Executed Evidence

All Cargo commands ran sequentially through `scripts/command-timings.ts`; no concurrent target jobs were started.

Final commands used this command-local prefix so child scripts inherit the pin:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

Apply the same prefix to each command below when the shell's default Bun differs from the repo pin.

| Check                                                                                                                                                                                            | Actual result                                                                                           |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------- |
| Task 1 RED: `bun run scripts/command-timings.ts run --key phase165-compact -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-features phase165`                  | 2 discovered; 1 failed as intended: signed normalized zero returned Negative instead of Zero            |
| Task 2 RED: `bun run scripts/command-timings.ts run --key phase165-work-vectors -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-features chain_proof`          | Failed compilation as intended: missing byte/hex boundary APIs and FromStr                              |
| Final compact GREEN, same Task 1 command                                                                                                                                                         | 18 passed, 0 failed, 0 ignored                                                                          |
| Final work-vector GREEN, same Task 2 command                                                                                                                                                     | 32 passed, 0 failed, 0 ignored, including existing proof-equivalent-time and peer-evidence tests        |
| `bun run scripts/command-timings.ts run --key phase165-consensus-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-features`                          | 246 unit, 8 BASIC integration, 3 parity integration and 2 taproot integration tests passed; 0 doc tests |
| `bun run scripts/command-timings.ts run --key phase165-consensus-format -- cargo fmt --manifest-path packages/Cargo.toml --all`                                                                  | Exit 0                                                                                                  |
| `bun run scripts/command-timings.ts run --key phase165-consensus-lint -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-targets --all-features -- -D warnings` | Exit 0 after removing two clone-on-Copy test setup calls                                                |
| `bun run scripts/check-parity-breadcrumbs.ts --check`                                                                                                                                            | 1,162 Rust files verified                                                                               |
| Independent bounded Bun BigInt reproduction below                                                                                                                                                | 11 proofs, 6 sums and 14 refusals verified; no production helper invoked                                |
| `git diff --check` and owned diff review                                                                                                                                                         | Passed; no unintended owned changes                                                                     |
| Stub/threat scan                                                                                                                                                                                 | No goal-blocking stubs; numeric input boundary is covered by the existing threat register               |

The final focused runs have nonzero relevant discovery. Filtered integration binaries legitimately discover zero cases during filtered runs; the unfiltered regression run executed those integration tests. Full native verification, instrumented pure-core line coverage, phase goal/review/security verification and normal-hook Git finalization belong to Plan 14/root and are not claimed by this focused summary.

## Independent Corpus Provenance and Reproduction

Source pin: Knots `29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878`. The fixture records `SetCompact`, `GetBlockProof` and unsigned wrapping addition references, formulas and the `source-derived-arithmetic` evidence class. Expectations were calculated using independent Bun BigInt arithmetic and fixed in the corpus; production `block_proof` and encoding functions did not generate expectations.

The following bounded read-only command reproduces all fixed proof, addition and refusal expectations with native BigInt and the pinned formula. Its loops are limited to the fixture's 31 fixed cases. Run it with the command-local Bun pin above. Plan 12 may promote this formula reproduction into its checked tooling owner.

```bash
bun -e 'import {readFileSync} from "node:fs"; const f=JSON.parse(readFileSync("docs/parity/fixtures/phase165-work-vectors.json","utf8")); const modulus=1n<<256n; const hex=n=>n.toString(16).padStart(64,"0"); const target=b=>{const e=b>>>24;const m=BigInt(b&0x7fffff);return e<=3?m>>BigInt(8*(3-e)):m<<BigInt(8*(e-3));}; for(const v of f.proof_vectors){const t=target(parseInt(v.bits_hex,16));if(hex(t)!==v.target_hex||hex(modulus/(t+1n))!==v.proof_hex)throw Error(v.name);} for(const v of f.addition_vectors){const sum=v.bits_hex?v.bits_hex.reduce((sum,b)=>sum+modulus/(target(parseInt(b,16))+1n),0n):BigInt("0x"+v.left_hex)+BigInt("0x"+v.right_hex);if(hex(sum%modulus)!==v.sum_hex)throw Error(v.name);} for(const v of f.invalid_vectors){const b=parseInt(v.bits_hex,16),e=b>>>24;let m=b&0x7fffff;if(e<=3)m>>>=8*(3-e);const error=m===0?"Zero":b&0x800000?"Negative":e>34||(m>0xffff&&e>32)||(m>0xff&&e>33)?"Overflow":"valid";if(error!==v.error)throw Error(v.name);} console.log("Independent fixed corpus verified:",f.proof_vectors.length,"proofs,",f.addition_vectors.length,"sums,",f.invalid_vectors.length,"refusals");'
```

Rust tests compare production output to these fixed expectations, rather than constructing expected proof values with the function under test. The endian test uses an explicit fixed 64-digit expected string against distinct bytes, providing a separate encoding check.

## Files Created/Modified

- `crypto.rs`: normalized shared compact decoder, selected-limit admission and new test registration.
- `block.rs`: selected-consensus header admission with shared error mapping.
- `packages/open-bitcoin-consensus/src/crypto/tests/phase165_tests.rs`: normalized zero/sign/overflow, equivalent encoding and selected-limit/hash/header behavior tests.
- `chain_proof.rs`: lossless byte/hex APIs and numeric-only parsing error type; existing arithmetic retained.
- `packages/open-bitcoin-consensus/src/chain_proof/tests.rs`: child vector test registration.
- `packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs`: fixed corpus checks, high-limb order and lossless/malformed encoding tests.
- `docs/parity/fixtures/phase165-work-vectors.json`: independent fixed values and explicit evidence classification.
- `docs/parity/source-breadcrumbs.json`: all new first-party Rust files mapped; compact owner anchors refreshed.

## Decisions Made

Followed D-01 through D-04. Existing generic header/hash APIs keep their signatures; downstream selected-network owners must call the new explicit `*_with_params` APIs. A raw exact number never earns parent, header, body or coins validation provenance. Knots invalid block proof returns zero internally; the first-party API continues to return a decode error and documents that mapping.

AGENTS.md repo-local timing/breadcrumb rules, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust/TypeScript standards and both complete active lesson files materially informed the execution.

## Deviations from Plan

None in source scope or architecture. Deferred commits and shared tracker updates follow the approved execution contract rather than the generic executor defaults.

## Issues Encountered

- The shell initially selected Bun 1.4.2. The parent supplied the already-installed 1.3.9 executable; final focused tests, formatting, lint, breadcrumb and independent-corpus checks ran with a command-local PATH selection. No repo or global runtime configuration changed.
- Focused clippy found two unnecessary `clone()` calls on Copy consensus parameters in the new test setup. Removed them, reran formatting/lint and both focused GREEN commands. No suppression was added.

## User Setup Required

None.

## Next Phase Readiness

Plan 02 can use the byte API for exact untrusted records; Plan 03 can carry ProofWork; selected-network admission owners can adopt the explicit limit API. Plan 12 has independently reproducible source-derived vectors to consume.

Canonical durable bootstrap, exact-work checkpoint/all-handle reopen acceptance and genesis BASIC remain Phase 166; contextual prerequisites remain Phase 167; actual pinned-daemon differential qualification remains Phase 168. This plan adds no ordinary support opening, default networking activation or production/funds claim.

## Parent Test-Placement Revalidation

Before Plan 07, the parent moved the cfg-test module into `packages/open-bitcoin-consensus/src/crypto/tests/phase165_tests.rs` for the existing panic-policy scanner and preserved its logical module name. The initial source hash was identical; the first compile exited 101 for a relative fixture include, then the single extra parent component restored the same frozen codec fixture. Normalizing only that include recovers the original SHA-256 `4a44d1d93004daf493071baae040744ee8e8059c2bb578f37f6eb989d2fd38b8`; test logic and assertions are unchanged. The pinned timed consensus phase165 run passed all 18 tests. Formatting, panic policy, breadcrumbs (1,171), managed checks and current source guards passed. This remains focused revalidation, with whole-phase native acceptance pending.

## Self-Check: PASSED

All eight owned source/test/fixture/breadcrumb files and this summary exist. Focused checks are GREEN and the owned diff passes whitespace review. The index is empty; commit-existence verification is intentionally deferred because the approved parent contract prohibits executor staging/commits. Shared tracker changes present in the checkout predated this execution and were preserved.
