---
phase: 165-shared-exact-work-authority
plan: "12"
subsystem: native-source-and-evidence-verification
tags: [proof-work, bun, source-contracts, mutation-controls, historical-attribution]
requires:
  - phase: 165-11
    provides: Delivered work owners, adapter refusal evidence and preserved historical tooling
provides:
  - Bounded regular-input loader with a pinned ordered read inventory
  - Pure current exact-work source/evidence policy and executable test ownership checks
  - Real loaded-source adversarial controls and mandatory default verifier registrations
affects: [165-13, 165-14, 166, 167, 168]
tech-stack:
  added: []
  patterns:
    - Literal-bearing reviewed owner identities plus lexical executable-test ownership
    - Exact mechanical verification projection preserving original historical receipts
key-files:
  created:
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
    - scripts/check-phase165-shared-exact-work.test.ts
  modified:
    - scripts/verify.sh
    - scripts/check-phase164-canonical-identity/historical-tooling.ts
    - scripts/check-phase164-canonical-identity.ts
key-decisions:
  - Source identities and mutation controls remain distinct from Rust execution and runtime acceptance.
  - Pin the complete ordered inventory before reads so imported manifest data cannot add paths.
  - Preserve historical manifests and receipts; recognize only six exact mandatory verification additions.
  - The older mandatory guard independently rejects removal of all six additions.
requirements-completed: []
requirements-contributed: [CWOR-01, CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T02:11:44Z"
duration: approximately 18min from first checker scaffold; context loading not separately timed
completed: 2026-10-16
tasks_completed: 2
files_modified: 6
---

# Phase 165 Plan 12: Native Exact-Work Guard Summary

**A bounded read-only loader, pure source/evidence policy and 837 adversarial cases protect exact-work ownership, enabled behavioral evidence and mandatory native registration without claiming runtime acceptance.**

## Delivered Contracts

`loadInputs(maybeRoot)` reads only the reviewed ordered inventory. A fixed SHA-256 inventory pin is checked before any source read, preventing imported historical-manifest bindings from broadening paths. Each input must be regular, final symlinks are refused, the opened descriptor is checked again, and reads stop at 512,001 bytes with a 512,000-byte acceptance bound. Missing, oversized, directory and symlink inputs fail visibly without borrowing missing files from the checkout. There is no network access, archive traversal, new dependency or write in the loader.

`checkPhase165(inputs)` is a pure source/evidence check. It combines 80 reviewed literal-bearing owner identities, 26 actual executable test-owner files, inherited current claim rules, the independent arithmetic corpus and immutable historical identities. Existing lexical helpers classify functions, attributes and inline/file ownership. Enabled test attributes and enclosing cfg are checked separately from reviewed source bodies; comment-only declarations, ignored tests and file-level disabling cannot retain evidence. Ordinary numeric facts remain constructible where advisory; source/stage/preference ownership supplies authority.

Reviewed owners preserve the single existing ProofWork implementation and normalized selected-parameter proof admission; strict header work preference; body linkage/FIFO and unknown-order ancestry; private origin/owner/generation-bound stages; immutable linkage seals separate from expiring eligibility epochs; actual-owner checks before preparation; full selected-parameter proof filtering for both best and side requests; detailed Validation-versus-Network errors; typed consensus-only fallback, fatal local failure, fresh candidate queries and accepted-prefix turn termination; stale BASIC actual-active-identity refusal; and explicit unavailable work decisions before backend access. Narrow genuine fixture forwarding remains explicit. No parallel arithmetic, selector or validator was added.

Actual loaded-source mutations assert their anchors and require an attributed owner/rule finding. Controls cover scalar/count/saturation arithmetic, equal/hash/height ties, raw acceptance, public staged fields, foreign/stale bindings, late preference, unknown cached-order upgrades through ancestry/duplicates/clones/authorization, copied-sequence history, revoked epochs, error masking, raw or wrong-parameter requests, stale BASIC identity, backend opening, test removal/commenting/disablement, independent corpus expectations/pin and historical receipt digests, and later-phase overclaims. Corpus mutation controls preserve unrelated formatting; expected values are never regenerated with a production helper.

## Native Registration and Historical Attribution

`verify.sh` has exactly six added lines: the Phase165 test/check pair after Phase164 in each real `run_step` and `VERIFY_COMMAND_ORDER` surface, plus the existing Phase68 Bun test immediately before its checker in each surface. Explicit `./` test paths are retained. Every old verifier byte, command order and hook behavior remains pinned by the original verifier SHA after exact projection of those additions.

The parent approved the two additional historical-adapter paths. The adapter recognizes only the six exact, unique, contiguous additions at their required anchors. Missing, duplicate, commented, disabled, redirected, reordered or unrelated edits refuse. The already mandatory Phase164 guard independently requires all six lines, including when all six are removed together. Its control uses valid actual inputs, asserts no missing-input finding, and confirms that raw historical attribution remains available separately. The historical manifest was not rebaselined, and no receipt or raw historical snapshot changed.

The current mechanically adapted Node preservation source remains 20,688 bytes with SHA-256 `e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`. Read-only Git blob verification independently confirmed the original 20,556-byte source at blob `0345bab1bfbd76be3078453a6b9201fdf73941cc`, SHA-256 `3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630`. These identities remain separately attributed.

## Executed Evidence

Every Bun command used the command-local pinned Bun 1.3.9 PATH. Heavy source suites ran serially; active sessions were polled and were not suspended, interrupted or given relaxed timeouts. No Cargo/Bazel run was necessary because this plan changed no Rust or production behavior. Parent Plan14 retains full workspace/native verification and Git finalization.

| Command or check                         | Actual result                                                                                                                                                                                                                                                                                                                 |
| ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial checker scaffold                 | Expected missing-module RED: 0 passed, 1 failed, 1 module error; no product behavioral RED claimed.                                                                                                                                                                                                                           |
| Initial actual baseline                  | 1/1 passed after implementing the checker; initial checker CLI passed.                                                                                                                                                                                                                                                        |
| First complete Phase165 matrix           | 814 passed / 12 failed across 826 cases, 155.11s. All twelve failures were mutation anchors assuming column-zero functions in four existing inline test modules; the guard was not reached for those twelve.                                                                                                                  |
| Focused anchor repair and added controls | 61 passed, 771 filtered out, zero failed, 13.36s. Lexical offsets now select the actual attribute span including indentation.                                                                                                                                                                                                 |
| Final declared combined Bun test command | bun test ./scripts/check-phase68-active-chain-persistence.test.ts ./scripts/check-phase164-canonical-identity.test.ts ./scripts/check-phase165-shared-exact-work.test.ts: **1,594 passed, zero failed, 4,133 expectations, 429.32s**, session `92109`, exit 0. Component counts: **6 Phase68 / 751 Phase164 / 837 Phase165**. |
| Final Phase68 checker                    | bun run scripts/check-phase68-active-chain-persistence.ts: passed.                                                                                                                                                                                                                                                            |
| Final Phase164 checker                   | bun run scripts/check-phase164-canonical-identity.ts: passed, 131 fixed Rust inputs / 452 reviewed edges.                                                                                                                                                                                                                     |
| Final Phase165 checker                   | bun run scripts/check-phase165-shared-exact-work.ts: passed, 80 reviewed source owners / 26 executable test-owner files.                                                                                                                                                                                                      |
| Managed checks                           | bun scripts/bright-builds-check.ts all: 1,521 scanned inputs, zero exceptions/findings; lesson check passed.                                                                                                                                                                                                                  |
| Shell checks                             | bash -n scripts/verify.sh and shfmt -i 2 -d scripts/verify.sh: passed.                                                                                                                                                                                                                                                        |
| Source stability and diff                | All 216 hardened input/checker hashes matched at closure; all 162 older Phase164 input hashes match the earlier record. git diff --check passed; index empty.                                                                                                                                                                 |
| Source caps                              | New checker/contracts/test files: 164/136/281 lines. Approved older checker/helper: 243/67 lines. All below 628 with no exemption.                                                                                                                                                                                            |

### Exact Mixed-Run Attribution

The combined command began before the inventory hardening edit. Only `scripts/check-phase165-shared-exact-work.ts` and its test changed during the older Phase164 component; all 162 older guard inputs remained byte-identical and Phase68 dependencies were unchanged. Bun deferred Phase165 import/discovery until after those edits. The actual combined log visibly executes all five new inventory controls at lines 821–825, then completes all 837 hardened Phase165 cases. No code changed after that discovery. This is **not** a claim that all 216 files were stable from the combined command's start.

The before/hardened maps identify precisely those two changed files. All 216 hardened hashes were subsequently rechecked unchanged after the completed run. The actual after-edit Phase165 component and final checker result serve both task gates; no redundant full guard run was performed. Evidence is retained in ignored `.local/open-bitcoin-dev/phase165-plan12-final-guard.log`, `phase165-plan12-repair.log`, `phase165-plan12-stable-input-hashes.json` and `phase165-plan12-hardened-input-hashes.json`.

## Deviations and Execution Notes

1. **[Rule3 - Blocking; parent approved] Historical verifier projection.** Required registrations would otherwise violate the existing immutable current-tooling fingerprint. Added the strict six-line mechanical projection and renewed only its reviewed helper digest/current registration checks in the older guard. Original manifests/receipts and unrelated verifier bytes remain unchanged.
1. **[Rule1 - Test bug] Inline mutation anchors.** The first complete run exposed twelve test-construction failures. Lexical function offsets replaced whitespace assumptions; focused and final controls passed without weakening ownership checks.
1. **[Rule2 - Correctness] Pre-read inventory pin.** Review found imported manifest data could otherwise change the loader's path list before source validation. Added a complete ordered inventory hash checked before reads, with positive and traversal/extra/missing/reordered controls.
1. The installed shfmt rejects the compact spelling `-i2`; the initial command exited 2 without edits. The supported `-i 2` spelling passed and preserved every old verifier byte. An initial receipt-list generation attempted nonexistent `163-CLOSURE-VERIFICATION.md` and stopped before writing; the delivered inventory requires only existing files. Neither issue was hidden as a successful check.

The simplification pass retained existing lexical/source/historical helpers and fixed identities rather than introducing a Rust cfg interpreter, alternate work engine or new dependency. No configured TS formatter or TypeScript build project exists and no TS formatter was available on PATH; adjacent style, actual Bun parsing/execution and managed checks were used. AGENTS.md, its sidecar, placeholder-only overrides and architecture/code-shape/verification/testing/Rust/TypeScript guidance informed implementation. Both active lesson files were read completely: 7,188 bytes / 2,397 conservative estimated tokens. No blocking stub or new production endpoint/auth/schema surface was introduced.

## Scope and Integration Handoff

Passing source/formula checks are not actual Rust execution or runtime acceptance. Plan11's normal Open Bitcoin startup matrix of 32 owned daemon children remains local refusal evidence, separately from configured factories, and is not pinned-Knots differential qualification. Four canonical selectors retain ordinary refusal and networking remains default off. Phase166 retains bootstrap/recovery/all-handle exact reopen and genesis BASIC; Phase167 retains contextual completeness; Phase168 retains actual pinned-Knots daemon qualification.

The loader does not require Plan13's not-yet-created parity document or Plan14 acceptance. Before Plan13 executes, its scope must be amended to include **`scripts/check-phase165-shared-exact-work.ts`** in addition to its declared contracts/test paths: adding the newly created claim document requires changing the actual fixed input inventory, renewing the ordered inventory pin, and adding actual overclaim checks. Merely adding a test or contracts entry cannot extend the imported current claim inventory. The parent owns this amendment and subsequent work; this executor stops at Plan12.

All task, RED/GREEN and metadata commits are deferred to the parent strict wrapper. This executor staged, committed and pushed nothing, changed no hooks or worktree, and did not edit shared STATE/ROADMAP/REQUIREMENTS/config/todo. No commit hash or runtime receipt was invented. Parent Plan14 owns final full verification, independent review/security/verifier, shared state updates and normal-hook Git finalization.

## Self-Check: PASSED

All six implementation/test/checker paths and this summary exist. The final combined session exited zero with the exact 6/751/837 component counts and five visible inventory controls. Final live checkers, managed checks, shell syntax/formatting, caps, input hashes and whitespace checks passed. All 216 hardened inputs remain unchanged; only the two identified checker/test files differ from the earlier 216-file snapshot, and no older Phase164 input changed. No stub markers remain in the six owned implementation paths. Historical preservation bytes and receipts retain their separate identities. No commits are expected from this executor; Git index and shared state finalization remain parent-owned.
