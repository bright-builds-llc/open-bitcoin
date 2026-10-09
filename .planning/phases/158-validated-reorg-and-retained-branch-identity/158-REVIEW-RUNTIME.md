---
phase: 158-validated-reorg-and-retained-branch-identity
reviewed: 2026-10-09T03:42:33Z
depth: standard
review_scope: runtime-integration
diff_base: 2f21ac2052208c7e7a084ed006d908c5ccb714aa
files_reviewed: 27
files_reviewed_list:
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests/faults.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
---

# Phase 158: Runtime and Integration Code Review

**Reviewed:** 2026-10-09T03:42:33Z
**Depth:** standard with cross-file authority tracing
**Files reviewed:** 27
**Status:** issues_found

## Summary

Read all 27 assigned runtime, manager, source-preflight and integration-test files in the current worktree, including new untracked modules. Traced direct manager, serialized network and sync reconciliation calls through preview suspension, genuine absorption, accepted-target visibility, guarded publication, current accepted-position append and normal own-flush fencing. One confirmed correctness issue remains in a cross-read storage prerequisite: a second reorg after a shorter accepted-unflushed replacement is refused solely because the authenticated displaced durable fence is taller than the current accepted tip. The independent storage reviewer confirmed the same issue; consolidate it as one finding, WR-158-01.

Material guidance: repo AGENTS.md and Repo-Local Guidance, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md and architecture, code-shape, testing, verification and Rust pages. Both active lessons were loaded completely: 7,188 bytes / 2,397 estimated tokens. Read Phase 158 context, Plans 01–06, their summaries and measured-work evidence. This report belongs to the parent's active GSD phase workflow. No implementation changes, build/test runs, staging or commits were performed by this reviewer.

## Warnings

### WR-158-01: Consecutive reorg refuses an authenticated taller displaced coins fence

**Severity:** Warning / P2
**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs:181-184`
**Affected runtime caller:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/filter_reorg.rs:109`
**Evidence gap:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs:263-265`

**Issue:** The preparation guard rejects every transition whose previous durable coins height exceeds the staged old accepted height. A successful shorter replacement intentionally leaves the original durable coins endpoint in place until ordinary own flush. Consequently, a second genuine reorg before that flush fails with `BASIC displaced endpoint behind progress`, even when every required body/undo is retained, the current lineage is confirmed and the achieved prior reorg binds that exact displaced durable tuple. This is an unnecessary branch-transition stall; it is independent of the documented finite source-resource refusal.

Concrete sequence: fully indexed A ends at height 23; accept B at height 14 sharing ancestor 10; drive normal B catch-up without coins flush, leaving durable A23 and shared safe height 10; attempt genuine B14→A23. Preparation sees durable height 23 > old accepted height 14 and refuses before source preflight. Existing A→B→A coverage flushes B first and therefore cannot detect this state.

**Fix:** Permit the taller durable endpoint only when the existing achieved same-store replacement identity binds `maybe_displaced_fence == Some((previous.durable_height, previous.durable_hash))`. Retain the separate processed-frontier rejection and all generation/revision/ancestor/provenance checks. Add a production A23→B14→A23 regression with consecutive accepted-unflushed transitions, unchanged coins endpoint until an ordinary flush, exact active/immutable commitments and conservative safe protection; add a negative control showing an absent or mismatched displaced tuple still refuses. Do not force a coins flush to bypass the guard.

**Validation:** Static control-flow proof plus independent storage-review confirmation. The parent is arranging actual RED/GREEN verification and owns the fix; this pre-fix report does not claim the repair or execution has passed.

## Reviewed authority and evidence

| Concern | Source and concrete evidence assessed |
| --- | --- |
| Preview versus acceptance | Manager freezes concrete publication before core preview; frozen old batches/tokens/new issuance and pending flush receipts are tested. Pre-absorb failure preserves explicit old accepted identity. Genuine absorption exposes the new optional target before later effects. |
| Failure versus achievement | Rewind and append faults retain conservative processed/safe state and pause; AfterCommit earns no capability. Full disconnect exposes None rather than a fabricated nonempty target. Later genuine connect advances explicit target. |
| Current branch append | Every nonempty ordinary live/recovered turn obtains sealed current accepted positions; no stage survives into later turns. Zero-record checkpoint promotion retains own coins/metadata fencing. |
| Required sources | Native body/undo admission precedes allocating decode and body/merkle validation. Exact full historical undo equality includes scripts, values and metadata. Missing/corrupt/other-branch mates refuse before preview; immutable reuse distinguishes Missing/Deferred/Ready and checks exact predecessor. |
| Mempool and sync | Existing preview→mempool→absorb order remains explicit. Sync tests install actual header entries / live headers then reconcile through the production reorg caller. Body-adapter fault projection is supplemented by the real production block-response notification source. |
| Recovery and retained identity | Three configured all-handle-drop reopen cases prove old-coins/ahead-index, rewound checkpoint/conflicting suffix and new-coins/partial-projection continuation. Physical rows are inspected after drop; exact immutable records and canonical active rows are asserted. |
| Prune and cadence | Reserved CRUD, stale manual work, automatic generation remeasurement and startup interrupted intent are exercised. IfNeeded reports no coins write before ordinary due flush; shared safe/checkpoint protection stays conservative. |
| Fixture validity | Fork oracles use separately genuine staging and drop capabilities before production calls. Compact fixtures explicitly use maturity one. Separate actual runtime maturity-100/P2SH fixture mines PoW, binds merkle/height scripts and proves historical plus same-block spends, flush and reopen below height 150. |
| Work and limits | Test-only observation returns Copy counters/durations and drops genuine stage/capabilities. Checked storage/source/capture composition and all turn dimensions are assessed. 54 configuration / 149 turn measurements are recorded evidence, not newly executed here. No arbitrary retained-fork or whole-runtime constant-work claim follows. |
| Generic and default-off | Generic/from_chainstate/clone paths carry no validated lineage; test progress installation grants no store capability. Existing startup policy remains explicit, and ordinary no-lineage behavior is preserved by the new preparation early return. |

No other actionable runtime/integration bug was confirmed in this assigned scope. These statements are source-review conclusions, not a replacement for the parent's native verification, source/security consolidation or formal lifecycle gate. Storage/core modules read for authority tracing are supplementary and are not counted as full assigned-file coverage.

## Verification limits

The parent explicitly prohibited Cargo/Bazel execution during concurrent native work. This reviewer therefore did not rerun tests or claim whole-phase success. Read the actual test assertions and recorded positive-count results in Plan 06: 98 required Phase 158 node tests, 8 focused failure tests, 4 protection tests and the explicitly executed 54-configuration/149-turn experiment. The sole ignored case is the explicit timing experiment. Root must validate the final repaired source and audit any changed file against the snapshot below.

## Reviewed snapshot

SHA-256 values captured at 2026-10-09T03:42:33Z; baseline is the supplied fixed commit plus current tracked modifications and the explicitly assigned untracked modules. The snapshot precedes the requested WR-158-01 repair and supports a targeted final changed-file audit.

| File | SHA-256 |
| --- | --- |
| `packages/open-bitcoin-node/src/chainstate.rs` | `3e04ceb45cfc3affdaae01ebd4ab791cd7b82110738e26e910b5242b14eb19f1` |
| `packages/open-bitcoin-node/src/chainstate/filter_index.rs` | `9902f1021730914bfffa4b1b99764dcf8350ebf20d33b3ed777808c98a86ec0d` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs` | `cbde61693ef860a2210b4a7261b74f095a9c60aae3771cf0775c0f9acd1134c5` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs` | `b7adbc9f1ec32d45e36429e2702fde89409050268b22f084bf2f5b5ede33d377` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs` | `43fd99edb144250f6c10b59781993a4a7189bfc424d92d4b25d540012961b32a` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs` | `b29db3b5927f81879c2ef90552775ad6ed65f9926d486b339dcc6dee6e5aa357` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store.rs` | `322b973e2f687d6ff4c7b9d06d46682a4723f499115da9d53d4f412d71d2353b` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs` | `41755114f92096de344dd0e2f987e324d713d1d9a4f3195700c192d1a644268a` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests.rs` | `951ec9688e42561d353dd724540a924dc28d63a0db9ac7d6da26b50d905e1fdd` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs` | `9f1115b019ae6846d251b71f2543c15e82721db1db782e555d6eca5275e9eecb` |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` | `d82c02194ec2bd0de54d16e4bb006f43a498c172a3d2a1c15ff79377f8fec16e` |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | `051cc7f80d4b937f17086197571abbb61f65f936f184e7c82dbea72296da934c` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs` | `f13b4e896dd803cf640dbdbafba13b86bb5b8c12921c7662563d90b193010f0b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs` | `935a82ecc7838fe0a109180a4a2961dc492f787a06b36537c2d7274bbdafd2c4` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs` | `cc6f28cdea3a16517fac0bffa0b7c267ea2765db3163cf80fbd9501c068e4838` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests/faults.rs` | `4b065e69f0aeff73a974f6e4c93d1d5a932aeb3a65d74c34b8ec2838f0637497` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs` | `89104de96b6cca93e3b5321027786ea9570dd610ecf5a1d1e40de51c10f01eb9` |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs` | `1c2088e1d39572e91d28247993f3bf73f234beb8200895969daf597ae1d0319d` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index.rs` | `0a0fd9b0a775c55b8c598f2abc410fcef6f255c6afd431c335405f0b95a18b7d` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs` | `5c2a5ba1c93ded830ed969e04ee7682dd3840a38c31652c6afb2a5fc4dd66bc8` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg.rs` | `597b2276dc5b5719c67fa555fcb5578c2bc49f0ab954e83852c7378127330c49` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs` | `46ecfe47b3f886e901c51e47d95fb1a696cc9c5d28e3f82ded3cc86ad4307c42` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs` | `f450a4741c0da057e7a2e0f941eb8cae2e26515ee4ce49720ed8366e86b8b100` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs` | `5dcec3e9bbf7380e82258bd9e1e05e0fc9d60d469104fd33ee11b3803e02b0b0` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs` | `7a8e1d798bc974fc0123ab2a8355453149e081ad404a5d4f772f1eacfb2c6a74` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs` | `2cdc131e3c7586d523e36924fa8c095e4f95b3da5f6c497cfb5bb5e4fb87810f` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs` | `8a3f4e94741796c8071277c5a2c732204631ff828a0a1d5f2e811baa35e91eae` |

***

_Reviewer: gsd-code-reviewer, runtime/integration assignment_
_Depth: standard plus necessary cross-file flow_
