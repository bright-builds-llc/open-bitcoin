---
phase: 167-candidate-history-validation-rules
status: passed
threats_total: 187
threats_closed: 187
threats_open: 0
threats_mitigate: 181
threats_accept: 6
threats_transfer: 0
unregistered_flags: 0
asvs_level: 1
block_on: open
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-11-05T06:01:01Z"
---

# Phase 167 — Security

**Security verdict: passed for the bounded Phase 167 implementation and its declared threat controls.** No unresolved actionable high-severity finding was found. This verdict is tied to the final source and actual default verifier below. Independent source and goal review, lifecycle reconciliation, staged-index LOC, normal commit hooks and Git publication are separate subsequent gates.

## Review basis and threat inventory

The review covers all 51 Phase 167 PLAN threat models, the Plan 17–19 and 25 source/evidence handoffs, Plan 20's current amendments and A1/A2/B/C coverage receipts, and the final source under the successful default verifier. The 51 PLAN models contain 201 threat-table occurrences. Deduplicating the relocated T-167-13-02, the shared R6-01/R6-02 rows in Plans 45/46, and the six shared T-CDE rows repeated in Plans 49–51 leaves **187 distinct threat concepts**: 181 mitigations and six explicit accepted risks. The Plan 44 fixture-data risk is an explicit accepted table row without a numeric threat ID; it is counted once. No transfer disposition or unregistered Summary threat flag was found. Earlier failed runs retain their original diagnostic status.

ASVS 4.0.3 V4, V5 and V6 provide scoped vocabulary for authority, input validation and stored hash integrity. This report does not claim web certification or introduce an authentication/session review outside the phase scope. The reviewer made no production source edits and ran no Cargo, Bazel or verifier command.

The accepted Wave C source frame is `.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-c-applied82-v2/full-frame.json`, SHA-256 `b98af995b4ce42116a7d732f948bf89eabc31e22b1351b3cf2e15057cc284901`: 1,833 paths, 82 direct Plan 20 owners, 1,751 protected paths and zero protected drift. The independently checked Wave C receipt, SHA-256 `3394365ec5c5be4d43390b3552a5b61b139b59f6ea6541514b56da2ffba1a601`, binds 21 source-stable runs and zero uncovered lines in the original eight-package selection. Its old `Result` sigop APIs, witness helper, malformed syntax, flags and policy/context refusals remain intact.

## Trust boundaries and observed controls

| Boundary | Security finding |
| --- | --- |
| Caller, imported history or fixture → selected canonical rules | `CandidateHistory` is structural evidence, not accepted lineage. `engine/candidate_rules.rs` rederives history from the selected branch and compares the held candidate, disconnect branch and rules before selected context or staging. `stage_seeded_historical_block` requires a mechanical fixture and `IndependentlySeeded` origin; B's seeded-held-rules test refuses selected context and reorg staging without changing the snapshot. |
| Genesis and header import → earned network owner | `network/canonical_admission.rs` binds live origin, proved genesis hash/header/work, coins best block and network magic. Canonical header import requires ordinary scope, recomputed identities, linked parent ordering and readmission under selected rules. Fixture import remains fixture-only. Mainnet, testnet3 and default Signet ordinary routes refuse. |
| Raw transaction/block → consensus or policy decision | Selected proof of work, witness commitment, 80,000/80,001 sigop limits, script execution errors and codec bounds remain independent of source fixtures. Wave B tests cover actual invalid-to-valid PoW, BIP34 seed taint, witness/mutation/oversized encoding refusal, complete coin contexts and checked consensus sigop arithmetic. Wave C's count-only policy accessors share the same private syntax counters with the retained public `Result` functions; the protected valid-`None` witness helper still returns `Result`. Mempool saturation, witness flags and context-length refusal remain. |
| BIP30 overwrite → complete prior Coin and private overlay | `engine/overwrite_undo.rs` binds the exact block hash, height, transaction occurrence and both complete prior/new Coins into its integrity digest. `replace_outputs` requires the actual parent Coin to equal the captured prior Coin before spending; restoration operates on a private overlay and only publishes after all checks succeed. Tests exercise wrong prior Coin and controlled second-read storage faults with unchanged accepted state. A self-consistent imported undo never grants ancestry or acceptance. |
| Retained bytes → durable accepted history | `storage/canonical_history/records.rs` checks each parsed body hash against its durable key and canonical re-encoding. `storage/canonical_history/proof.rs` replays selected bodies and compares exact resulting hash, header, height, work, MTP, status and complete undo. The prospective projection requires complete validation coverage and refuses a conflicting or downgraded `ScriptsValid` identity. Header-known/body-invalid stays distinct from accepted coins or script-valid progress. |
| Existing source tree → detached inspection | The copy has finite entry, depth, path and 512 MiB byte budgets, strict single-link regular-file metadata and path/opened-file identity checks. On Unix, a known regular descendant whose link count drops to zero is classified `Changed` for a bounded live retry only after original identity and path-safety checks; unknown/root zero-link files and multi-link hardlinks still refuse. No original source mutation or unsafe copied-source acceptance was found. The accepted Plan 40–42 scheduling limits below remain explicit. |
| Peer/RPC bytes → state or response | Compact completion keeps selected held rules, witness and short-ID checks, typed incomplete/invalid/codec distinctions, request caps and error-before-success ordering. Actual success still requires the stored body and fresh header/linkage/epoch; operational failure does not silently clean up or refund requests. Phase 160's 13-byte partial-frame controls accumulate bounded first/second reads and refuse incomplete frames. `submitblock` is documented only for the bounded local Regtest route; no default public-network activation is inferred. |
| Current proof → historical claim | Plans 17–19 and 25 keep current source digests separate from frozen historical identities, use bounded regular-file checker inputs and preserve exact verifier projection/mutation controls. The parity page distinguishes 301 proved-root default-Regtest descendants and complete retained reopen from independently seeded Mainnet/Testnet3 script and Mainnet BIP30 cases. Those fixtures never become canonical product or pinned-Knots daemon differential evidence. |

## Plan 20 threat disposition

| Threat | Disposition | Evidence and limit |
| --- | --- | --- |
| T-167-20-01 — incomplete or stale validation | Mitigated | The complete default `bash scripts/verify.sh` v14 exited zero on its reviewed source; the revised-configuration v15 also exited zero. The original eight-package coverage report has no `Uncovered Lines:` heading, Bazel build provenance passed, and focused Phase 167 checkers and real Rust tests have nonzero discovery. Staged-index LOC and the complete normal saving hook remain later gates. |
| T-167-20-02 — unreviewed source or goal claim | Mitigated for security; final process handoff retained | Exact A1/A2/B/C owner frames and applied diffs received independent security/scope review. The accepted C frame has zero protected drift. This report closes security findings; separate independent source and goal reports and lifecycle validation must precede saving. |
| T-167-20-03 — evidence-class promotion | Mitigated | Receipts retain failed attempts, literal commands, source hashes and actual outcomes. Package-only consensus residuals in Wave C were not promoted to aggregate success; the separately run original eight-package selection is zero. Seeded historical fixtures, source guards, offline reopen and actual local daemon observations are labeled by their true evidence classes. Phase 168 remains the pinned-daemon differential boundary. |

## Explicit accepted risks

| Risk | Retained limit |
| --- | --- |
| T-167-40-05 | The asynchronous inspection worker has finite work but no hard whole-tree scheduler deadline. Exhaustion/refusal stays valid. |
| T-167-41-06 | Finite live observation does not bound an individual filesystem syscall or guarantee two stable attempts. |
| T-167-42-05 | Per-iteration anchoring and finite work do not guarantee eventual native quiet; a safe refusal remains possible. |
| Plan 44 fixture-data row | Owned offline synthetic fixtures and logs stay within the existing 401-position and verifier budgets. No production endpoint or secret is introduced. |
| T-167-45-R4-06 | Owned offline model fixtures use real helpers and default maturity but do not establish canonical 301 or differential authority. |
| T-167-46-06 | Synthetic owned scratch and public genesis fixtures do not activate an external or public network. |

These six are the dispositions declared in their owning PLAN models, not new waivers. They do not relax source identity, work budgets, refusal behavior or the default verifier.

## Actual final verification and source caveat

The retained runner `.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-c-default-verify-v14.json` has SHA-256 `0728e7a86cc5589fa6edcb13cf08fafa8c6e3e0b4d4f763a7d7728e71c80a62f`. It executed the literal default `bash scripts/verify.sh` through pinned Bun 1.3.9 and the cooperative timing wrapper, exiting zero after 12,591.673 seconds. Raw stdout SHA-256 is `795d10834099305d95ac189ebc24c8894e7f4975c08d434d1f156fc377e7bd83`; raw stderr SHA-256 is `ab635c67250c52eb5fa22299878c2f3e8ca86ed67e40e50d033201ddd9792565`. Both were independently rehashed. The output contains no `Uncovered Lines:` heading, reports Bazel provenance passed, and ends with `verify.sh completed in 3h 29m 50.953s`.

The v14 before and after 1,833-row snapshots are byte identical, SHA-256 `bb0fe7b60a1a95247e897f197a259c0812db7a41d15b0f3c91a3d83b224d8843`. Compared with the accepted C frame, the sole different row is generated `docs/metrics/lines-of-code.md`: its provisional worktree value changed from SHA-256 `cd8da292e1c45965ae787ef53ae85dd46ba82115ad39bf56b0b8a4549766b09a` to `3c8d7bebfcc010754b7c4f34647319a52ce9bfa0d3b8b587714feb5cd7435bc4`. Rust, guards and product behavior remained on the reviewed C bytes. ROOT still owns staged-index LOC reconciliation and the normal hook run; this security report does not grant that future result.

The supported product claim is a bounded private local default-Regtest route from proved genesis through 301 descendants, selected validation and complete retained-history reopen. Mainnet/testnet3/Signet ordinary product routes, public-default networking, canonical-history runtime wallet, arbitrary whole-chain operation and actual pinned-Knots daemon differential remain outside the accepted claim. Phase 168 owns the latter differential qualification. The six operational/fixture limits above remain visible; no secret, funds-readiness or production-readiness conclusion is made.

## Late Bazel host-tool build correction

After v14, the normal hook v2 passed its earlier guards and native tests but failed the six-target Bazel build when an `opt` host execution proc-macro dylib was rejected by macOS as a misaligned Mach-O LINKEDIT string pool. The retained hook log `.local/open-bitcoin-dev/phase167/plan20/hook-v2.log` has independently checked SHA-256 `c0d726d81a76c6bc116f16e25df230ab05fe440b7b0a65f14b92b4277`. The checked `167-20-BAZEL-HOST-TOOLCHAIN-AMENDMENT.md` records a fresh expunged default reproduction and a successful explicit six-target `--host_compilation_mode=fastbuild` build, log SHA-256 `aa5ca83841dab061fe982743bdcd43632fb83783422dad73c579821e8a1818bc`. The exact one-line `.bazelrc` change makes that host-tool mode the default. The independently rehashed fresh default build log `.local/open-bitcoin-dev/phase167/plan20/bazel-fresh-default-host-fix-v1.log`, SHA-256 `0054c9c6144b5e8fbe0a0f8c15073ac506d87e130934913c67bc9c7b36e38ddb`, reports all six targets and 753 actions completed successfully; the follow-up default build log SHA-256 `221887627ad11f8d0d5b6c7143b0b264a666a90314b077ec9f7bc27aee5df2fa` shows `.bazelrc` supplies the host flag. The correction changes no target compilation mode, Rust product/security source, test threshold or coverage selector, and adds no source-to-authority path. No actionable security finding follows from this build-only correction.

The v14 default verifier is evidence for its earlier `.bazelrc` bytes and reviewed product source. The revised-configuration default v15 verifier log `.local/open-bitcoin-dev/phase167/plan20/revised-default-v15.log` has independently checked SHA-256 `74a5226365e916707b0e723f74b586d754cb25025775ed0b1dcee7201afdb339` (2,502,984 bytes). It records a complete 2h 34m 54.863s run with zero failing test footers, two successful Bazel build conclusions, `Bazel build provenance check passed.`, and zero `Uncovered Lines:` headings in the original eight-package coverage selection. This closes the revised build-contract verifier gate. The earlier failed hook remains failed evidence; refreshed reports/lifecycle, staged-index LOC and the complete normal saving hook remain subsequent ROOT gates until their own retained outcomes are checked.

## Security audit trail

| Stage | Result |
| --- | --- |
| Earlier Plan 20 diagnostic/default attempts and A1/A2/B/C waves | Retained under their original exact source and outcome; none substitutes for v14. |
| Final C source and coverage | 82 direct owners, 1,751 protected, zero drift; original eight-package missing-line set empty. |
| 2026-11-04 default v14 and this independent audit | Default exit zero/source unchanged; 187 distinct declared threat concepts closed by 181 mitigations and six explicit acceptances; zero actionable high findings. |
| 2026-11-05 revised configuration v15 | Complete default verifier passes with fresh native tests, Bazel/provenance and original eight-package zero coverage; normal staged saving hook remains pending. |

**Security approval:** scoped threat controls and revised build configuration verified 2026-11-05. Independent source/goal review, lifecycle, staged-index LOC and normal-hook commit/push remain ROOT-owned subsequent gates. Stop before Phase 168.
