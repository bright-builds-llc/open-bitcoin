---
phase: 145-parity-roots-and-no-claim-guardrails
verified: 2026-09-18T09:05:00Z
status: passed
score: 13/13 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 145-2026-09-18T03-36-29
generated_at: 2026-09-18T09:05:00Z
lifecycle_validated: true
overrides_applied: 0
re_verification:
  previous_status: passed
  previous_score: 4/4 leftover IDs named
  gaps_closed: []
  gaps_remaining: []
  regressions: []
---

# Phase 145: Parity Roots and No-Claim Guardrails Verification Report

**Phase Goal:** Parity evidence is auditable and the v2.3 claim cannot be read as prune, assumeutxo, archive, or production readiness.
**Verified:** 2026-09-18T08:48:04Z
**Status:** passed
**Re-verification:** No — initial gsd-verifier pass. An executor-authored `145-VERIFICATION.md` already existed with `status: passed` and no `gaps:` section, so this run used initial-mode evidence checks rather than gap closure.

## Goal Achievement

Phase 145 owns only CSVFY-01 and CSVFY-02. Live parity roots, claim copy, UAT, and the last-gate checker make the v2.3 surface auditable without reading as prune, assumeutxo, archive, or production readiness. CACHE-01 remains Phase 139 ownership and MGR-03 remains Phase 140 ownership even while this closeout names their leftover-Pending evidence.

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Parity roots cite pinned Knots coins, flush, manager, and serve-path anchors, or document intentional differences. | ✓ VERIFIED | Closeout `upstream.sources` lists the five D-06 files; catalog names `CanFlushToDisk` and `CheckBlockDataAvailability`; `known_gaps` records D-07 differences as differences, not missing deliverables. |
| 2 | Deterministic no-claim guardrails keep prune/archive modes, assumeutxo, compact filters, public defaults, and production readiness out of the v2.3 claim. | ✓ VERIFIED | Live checker denies D-16 overclaims on the curated corpus; README/runtime-guide/release-readiness/support-matrix/production-claim-boundary use deferred/`does not` wording. Mutation suite fails positive prune/archive/assumeutxo claims and accepts deferred wording. |
| 3 | Default `bash scripts/verify.sh` stays deterministic, and historical `.planning/phases/` directories remain tracked. | ✓ VERIFIED | `run_step` lines do not include `public-network`, `wall-clock`, or `run-live-mainnet-smoke`. Checker fails missing verifier-referenced `.planning/phases/` paths. `.gitignore` does not ignore those directories. |
| 4 | Every v2.3 requirement ID has exactly one machine-readable surface owner, and Phase 145 owns only CSVFY-01 and CSVFY-02. | ✓ VERIFIED | `REQUIREMENTS_BY_SURFACE` maps all 15 IDs once. Closeout checklist object lists only CSVFY-01 and CSVFY-02. Live checker fails duplicate owners and CSVFY on a Phase 139 surface. |
| 5 | Human checklist and catalog pages present disk-backed coins, cache-flush, manager restart, and honest availability as the current v2.3 claim, with leftover snapshot blobs labeled historical rather than live UTXO truth. | ✓ VERIFIED | `docs/parity/catalog/chainstate.md` has `## Current v2.3 claim` plus the verbatim D-14 sentence, leftover snapshots as `non-authoritative`, and `## Historical Phase 4 snapshot-engine coverage`. Checklist rows mirror the seven `v2-3-*` owners. |
| 6 | A filesystem-only Phase 145 checker returns `string[]` failures, names all 15 v2.3 IDs exactly once, and requires the five D-06 Knots files plus the has_block breadcrumb group. | ✓ VERIFIED | `checkPhase145ParityUatReleaseBoundary(maybeRepoRoot?)` reads `OPEN_BITCOIN_PHASE145_REPO_ROOT`, rejects path escapes, and has no `fetch(`, `Bun.spawn`, or `node:child_process`. `CLAIM_FILES` has no `.planning/` history. Required anchors omit `HaveBlockData`. |
| 7 | Verifier checks require 144 then 145, keep 117 and 138 present, and do not assert that Phase 145 is the file-final `check-phase*` command. | ✓ VERIFIED | `verifier.ts` asserts the adjacent 144 then 145 pair plus 117/138 presence. It does not contain `final gate must end with` or `lastPhaseCommand`. Fixture whose last `check-phase*` is 138 still passes. |
| 8 | README and the runtime guide contain the verbatim D-14 sentence and still contain the verbatim v2.2 D-21 sentence. | ✓ VERIFIED | Both files include the D-14 coins/flush/manager/availability sentence and `bounded local-package APIs, same-peer 1P1C assembly over ordinary transaction messages, ordinary transaction fanout, and initial-broadcast-retry of locally submitted unbroadcast members`. README P2P row still has `package relay` and `remain deferred` in one sentence. |
| 9 | Committed `145-UAT.md` lists the repo-local Cargo and Bazel twins and records public-network review as not run. | ✓ VERIFIED | Package contains `status: not run`, both `open-bitcoin` Cargo/Bazel `status --format human` forms, `/tmp/open-bitcoin-chainstate-durability-support`, `open-bitcoind`, and `bash scripts/verify.sh`. It does not claim `status: passed`. |
| 10 | `scripts/verify.sh` runs the Phase 145 pair immediately after Phase 144 in both the visible order block and the live `run_step` chain, and Phase 138 remains the file-final `check-phase*` command. | ✓ VERIFIED | Visible and executable order is 144 → 145 → 121. Ordering comment keeps `Phase 138 is the final changed-path v2.2 release-boundary` and adds the Phase 145 closeout sentence. Last `bun run scripts/check-phase*` in `VERIFY_COMMAND_ORDER` is still Phase 138. |
| 11 | Every leftover Pending v2.3 requirement checkbox is `[x]` and the ROADMAP coverage table Status cells match REQUIREMENTS, with no leftover Pending rows. | ✓ VERIFIED | REQUIREMENTS has `- [x] **CACHE-01**`, `- [x] **MGR-03**`, `- [x] **CSVFY-01**`, and `- [x] **CSVFY-02**`. All 15 v2.3 traceability and ROADMAP coverage Status cells are `Complete`. No leftover `- [ ]` rows for those IDs. |
| 12 | All seven v2.3 surfaces are `done`, and the Phase 145 checker fails a leftover unchecked CSVFY box or an `in_progress` closeout surface. | ✓ VERIFIED | Top-level and checklist `v2-3-*` statuses are `done`. Phase 135 human title `v2 snapshot schema, checkpointing, and recovery` stays `done`. Tests `fails_when_a_leftover_csvfy_checkbox_is_unchecked` and `fails_when_the_closeout_surface_stays_in_progress` pass. |
| 13 | ROADMAP, PROJECT, and STATE agree that v2.3 implementation and closeout evidence exist, and they do not archive the milestone. | ✓ VERIFIED | PROJECT Current State uses the verbatim D-14 sentence and keeps `/gsd-complete-milestone v2.3` as a future command. STATE current focus is Phase 145 closeout evidence and says the milestone is not archived. ROADMAP Phase 145 heading remains `- [ ] **Phase 145`. |

**Score:** 13/13 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `docs/parity/index.json` | Seven v2.3 surfaces plus closeout owner | ✓ VERIFIED | All seven `v2-3-*` top-level and checklist surfaces are `done`. Closeout owns CSVFY-01 and CSVFY-02 only. D-06 sources and `blockmanager_tests.cpp` are present. No `docs/parity/v2.3-closeout.md`. |
| `docs/parity/checklist.md` | Mirrored human owners | ✓ VERIFIED | Same seven IDs, `done`, and exactly-once CACHE/FLUSH/COIN/MGR/HAVL/CSOBS/CSVFY ownership. |
| `docs/parity/catalog/chainstate.md` | Current v2.3 claim above historical snapshot prose | ✓ VERIFIED | Heading `## Current v2.3 claim`, verbatim D-14 sentence, `CanFlushToDisk`, `CheckBlockDataAvailability`, leftover snapshots `non-authoritative`. `HaveBlockData` appears only as the locked discussion name beside `CheckBlockDataAvailability`. |
| `docs/parity/source-breadcrumbs.json` | Dedicated `has_block` group | ✓ VERIFIED | `node-stored-block-presence` lists `blocks.rs` and `block_presence.rs` and cites `node/blockstorage.cpp`. Remaining `node-storage-contract` keeps `"breadcrumbs": []`. No `HaveBlockData` path. |
| `scripts/check-phase145-parity-uat-release-boundary.ts` | Thin CLI plus re-export | ✓ VERIFIED | Exports `checkPhase145ParityUatReleaseBoundary`. Implementation honors `OPEN_BITCOIN_PHASE145_REPO_ROOT`. |
| `scripts/check-phase145-parity-uat-release-boundary/constants.ts` | D-14 sentence, D-16 deny list, anchors | ✓ VERIFIED | Contains the verbatim D-14 sentence. `REQUIRED_KNOTS_ANCHORS` is the five D-06 files. No `HaveBlockData` required-anchor token. |
| `scripts/check-phase145-parity-uat-release-boundary/verifier.ts` | 144-then-145 order plus 117/138 | ✓ VERIFIED | Names 144, 145, 117, and 138. Does not pin Phase 145 as file-final. |
| `scripts/check-phase145-parity-uat-release-boundary/checks.ts` | Tightened `[x]` and `done` pins | ✓ VERIFIED | `checkRequirementCheckboxes` requires `- [x] **${id}**` once per v2.3 ID. Surfaces must be `done`. gsd-tools `contains: "- [x] **CSVFY-01**"` miss is a regex-template false negative; live REQUIREMENTS and the leftover-checkbox mutation prove the pin. |
| `README.md` | Scoped v2.3 claim plus preserved D-21 | ✓ VERIFIED | Active-milestone D-14 sentence, preserved D-21 sentence, no `start future work with /gsd-new-milestone` current-state copy. |
| `.planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md` | Deterministic UAT; public-network not run | ✓ VERIFIED | Cargo/Bazel twins published. `status: not run` is never a default, CI, or release gate. |
| `scripts/verify.sh` | 144 then 145 insertion; 138 file-final | ✓ VERIFIED | Visible and executable 144 → 145 → 121. Phase 138 remains last `check-phase*`. |
| `.planning/REQUIREMENTS.md` | Flipped leftover checkboxes | ✓ VERIFIED | CACHE-01, MGR-03, CSVFY-01, and CSVFY-02 are `- [x]` / Complete. FUT-18 through FUT-26 stay future. |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `docs/parity/index.json` | `docs/parity/checklist.md` | matching v2-3 surface IDs and exactly-once ownership | ✓ WIRED | All seven IDs present in both roots. |
| `docs/parity/index.json` | `packages/bitcoin-knots/src/node/blockstorage.cpp` | closeout `upstream.sources` | ✓ WIRED | Closeout sources include the five D-06 files. |
| `checks.ts` | `docs/parity/index.json` | exactly-once v2.3 ownership | ✓ WIRED | `REQUIREMENTS_BY_SURFACE` and `checkSurfaceOwnership` read the live index. |
| `verifier.ts` | `scripts/verify.sh` | visible and executable 144 then 145 | ✓ WIRED | Adjacent 144/145 pairs in `VERIFY_COMMAND_ORDER` and `run_step`. |
| `README.md` | `docs/operator/runtime-guide.md` | verbatim D-14 sentence | ✓ WIRED | Same D-14 sentence in both required files. |
| `scripts/verify.sh` | `check-phase145-parity-uat-release-boundary.ts` | pair immediately after Phase 144 | ✓ WIRED | Lines 396–397 and 564–565. |
| `checks.ts` | `.planning/REQUIREMENTS.md` | every v2.3 ID checked `[x]` exactly once | ✓ WIRED | Regex `- \[x\] \*\*${id}\*\*`. gsd-tools escaped-pattern miss is a false negative; live file has `- [x] **CSVFY-01**` and `- [x] **CSVFY-02**`. |
| `145-04-SUMMARY.md` | `.planning/REQUIREMENTS.md` | `requirements-completed` activation | ✓ WIRED | SUMMARY frontmatter lists CACHE-01, MGR-03, CSVFY-01, and CSVFY-02. |

### Data-Flow Trace (Level 4)

This phase is a filesystem-only closeout gate, not a UI that renders fetched data. The last-gate checker reads live files and returns real failure strings.

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `checkPhase145ParityUatReleaseBoundary` | `failures: string[]` | Allow-listed repo files via `OPEN_BITCOIN_PHASE145_REPO_ROOT` | Yes — live run printed `Phase 145 parity UAT release boundary validated.` | ✓ FLOWING |
| `docs/parity/index.json` | surface owners / Knots anchors | Checklist surfaces and closeout `upstream.sources` | Yes — 15 IDs owned once; D-06 files present | ✓ FLOWING |
| `145-UAT.md` | operator command strings | Published Cargo/Bazel forms | Yes — copy-pasteable commands, not alias-only | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Phase 145 mutation suite | `bun test scripts/check-phase145-parity-uat-release-boundary.test.ts` | 22 pass, 0 fail | ✓ PASS |
| Live Phase 145 checker | `bun run scripts/check-phase145-parity-uat-release-boundary.ts` | `Phase 145 parity UAT release boundary validated.` | ✓ PASS |
| Parity breadcrumbs | `bun run scripts/check-parity-breadcrumbs.ts` | `Parity breadcrumbs verified for 847 Rust file(s).` | ✓ PASS |
| Active-milestone traceability | `bun run scripts/check-active-milestone-verification-traceability.ts` | checker passed | ✓ PASS |

Default `bash scripts/verify.sh` already ran in the plan commits. Focused checkers above are the re-run contract for this verification.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| CSVFY-01 | 145-01, 145-02, 145-04 | Parity roots cite pinned Knots coins, flush, manager, and serve-path anchors, or document intentional differences. | ✓ SATISFIED | Closeout D-06 sources, `CanFlushToDisk` / `CheckBlockDataAvailability`, `node-stored-block-presence`, D-07 differences in `known_gaps` and catalog. |
| CSVFY-02 | 145-02, 145-03, 145-04 | Deterministic no-claim guardrails keep prune/archive modes, assumeutxo, compact filters, public defaults, and production readiness out of the v2.3 claim. | ✓ SATISFIED | Last-gate D-16 deny list, D-14 sentence on README/runtime-guide, `145-UAT.md` public-network `not run`, 144-then-145 verify.sh wiring. |

CACHE-01 and MGR-03 appear in `145-04-SUMMARY.md` `requirements-completed` so leftover Pending rows could flip after named evidence. Those IDs remain Phase 139 and Phase 140 owners. REQUIREMENTS.md maps only CSVFY-01 and CSVFY-02 to Phase 145. No orphaned Phase 145 IDs.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `145-UAT.md` | 25–57 | Required deterministic tests still say `result: pending` | ℹ️ Info | Planned documentation lag. The live checker and this verification already prove ownership, D-14/D-16, command publication, and the 145 pair. Not a missing closeout gate. |
| `.planning/ROADMAP.md` | Next Step | Still says run `/gsd-execute-phase 145` | ℹ️ Info | Stale next-step copy after plan execution. Phase heading correctly remains unchecked until verification lands. |

No blocker stubs. Checker source has no TODO/FIXME/placeholder implementations.

### Human Verification Required

None. This phase is a filesystem-only parity, claim, and verifier-order closeout. Optional public-network UAT is explicitly `status: not run` and is never a default, CI, or release gate.

### Gaps Summary

No gaps. The phase goal holds: parity evidence is auditable, and the v2.3 claim cannot be read as prune, assumeutxo, archive, or production readiness.

---

_Verified: 2026-09-18T08:48:04Z_
_Verifier: Claude (gsd-verifier)_
