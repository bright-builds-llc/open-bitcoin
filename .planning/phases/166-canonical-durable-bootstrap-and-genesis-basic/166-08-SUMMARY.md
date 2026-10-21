---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "08"
subsystem: indexing
tags: [canonical-genesis, basic-filter, saved-ownership, publication-fencing]
requires:
  - phase: 166-07
    provides: Earned canonical managed genesis and conditional genuine BASIC lineage
  - phase: 166-01
    provides: Frozen independently source-derived genesis and BASIC vectors
provides:
  - Canonical saved Active BASIC recovery through the existing full Enabled lifecycle
  - Four-selector actual durable genesis filter, hash and zero-predecessor header evidence
  - Fresh Disabled absence, saved disable/re-enable and all-handle reopen evidence
  - Existing publication faults and pre-effect saved-index corruption/cleanup refusal controls
affects: [166-09, 166-10, 166-11, 166-12, 166-13, 166-14]
tech-stack:
  added: []
  patterns: [canonical-only saved-mode resolution, existing accepted-state owner, unchanged generic recovery]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/canonical_genesis/basic.rs
    - packages/open-bitcoin-node/src/sync/canonical_genesis/basic_tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests/basic.rs
  modified:
    - packages/open-bitcoin-node/src/sync/canonical_genesis.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Keep the already-working Enabled genesis path and its existing bounded accepted-state owner.
  - After consuming and checking canonical proof, resolve only PreserveSaved with saved Active intent through existing Enabled startup; retain its generation and preserve absent/Disabled semantics.
  - Restore generic startup and lifecycle byte-for-byte instead of changing metadata-only recovery or retained-record scan contracts.
  - Reuse existing test publication controls and two narrowly visible test-only cleanup accessors without changing production inspection.
patterns-established:
  - A saved mode observation selects an existing lifecycle only after canonical proof; it never creates authority by itself.
  - Distinguish baseline GREEN, actual behavior RED, test setup failures and superseded implementations in evidence.
requirements-completed: [CBOT-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T17:36:46Z"
commits: false
duration: 18m 42s
completed: 2026-10-20
---

# Phase 166 Plan 08: Canonical Genesis BASIC Owner Summary

**Canonical genesis BASIC now retains its accepted owner across saved Active reopen, using the existing bounded lifecycle and exact four-network durable filter commitments.**

## Completion Boundary

Tasks: **2/2 complete**. The recorded execution window is 2026-10-20T17:18:04Z through 2026-10-20T17:36:46Z, excluding earlier read-only preparation. Seven implementation/evidence paths changed, plus this summary. The new BASIC test files have 515 and 221 lines; the existing storage test root has 607 lines. All owned Rust files remain below 629 lines.

This accepts plan 166-08's implementation contribution, not whole-phase requirement closure. No staging, commit, push, hook bypass, shared STATE/ROADMAP/REQUIREMENTS/task edit, LOC refresh or historical source-contract renewal occurred. Root owns lifecycle closure and normal-hook Git saving; plans 166-09 onward retain their integration and acceptance work.

## Existing Owner and Canonical Saved Intent

The first actual Enabled behavior test already passed all four independent vectors. Plan 166-07's genuine managed lineage, followed by `packages/open-bitcoin-node/src/sync/open_runtime.rs::finish_initialized_runtime`, initializes the existing ordered BASIC owner and drives its bounded turn. That turn reads the complete stored canonical body with no height-zero undo; it generates the exact P2PK-containing filter and explicit zero predecessor. There is no replacement index engine, seeded history, fixture-owner attachment or hardcoded production filter.

The actual missing behavior was saved Active PreserveSaved startup: generic metadata recovery retained its filter but supplied no live accepted owner. The final production change is the thin `resolve_startup_mode` in `packages/open-bitcoin-node/src/sync/canonical_genesis/basic.rs`. The canonical runtime first consumes `OpenedCanonicalGenesis::into_parts`, which checks the sealed store capability. Only then does it resolve PreserveSaved plus saved Active intent to the existing Enabled lifecycle. `ManagedChainstate::from_canonical_genesis` rechecks the capability before initialization effects. Enabled's existing retained-input, prune, generation and append-proof checks establish the genuine lineage; the ordinary initializer and turn then run unchanged.

Explicit Enabled/Disabled requests are unchanged. Absent PreserveSaved remains absent; saved Disabled PreserveSaved retains its saved rows and disabled intent. All four saved Active reopen cases retain their generation. Disabling saved indexing retains the immutable genesis filter, and explicit re-enable uses the new current generation. A captured old append proof and stale ordered owner refuse after disable/re-enable.

`packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs` and `packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs` are restored byte-for-byte to their pre-08 contents. Generic metadata-only recovery, refusal ordering and single-scan accounting remain intact.

## Durable and Failure Evidence

The 13 new Rust test functions exercise actual canonical runtime storage. Four-selector cases read the frozen `docs/parity/fixtures/phase166-genesis-vectors.json` and compare exact encoded bytes, raw filter hash, header, zero predecessor, nonempty full genesis output script, sole immutable record/projection, height-zero processed/safe progress and empty coins with the exact genesis outpoint absent. Fresh Disabled and absent PreserveSaved create no owner/state/checkpoint/filter/projection or BASIC prune lock.

Enabled and saved Active PreserveSaved cases release runtime handles before same-path canonical reopen. A separate Weak-owner control proves a retained network handle keeps the original store alive and blocks reopening; after the last handle drops, the original owner disappears and independent canonical reopen preserves the filter and coins B.

Thread-local configuration/append seams inject only the existing `FilterPublicationFault` controls: BeforeEnable/AfterEnable, BeforeRecords/BeforeCheckpoint/BeforeProtection and BeforeCommit/AfterCommit. Every injected startup returns an error with no accepted runtime; subsequent independent canonical startup proves a complete state and returns the exact genesis commitment. SyncAll controls are software pre/post-commit evidence, not hardware power-loss evidence.

Seven saved-byte corruption cases cover a wrong predecessor, an internally self-consistent empty filter conflicting with the canonical body, wrong checkpoint hash, wrong fence hash, unknown saved owner mode and missing record/projection half-pairs. Each actual canonical reopen refuses before the original backend builder and preserves every original file byte, mode and mtime; atime is excluded. Stale generation fencing is tested separately through actual disable/re-enable rather than inventing an independently knowable saved generation.

The saved-BASIC cleanup failure control uses the existing isolated recovery child and existing cleanup fault. Original builder count and physical source snapshot remain unchanged. The exact observed private root retains a nonempty child result; the scratch engine lock is reacquired after child completion. Only that root is removed after canonical-parent, private directory, non-symlink and device/inode checks. Production uncertain-child handling remains unchanged and never authorizes deletion.

## Decisions and Deviations

**[Rule 1 - Bug] Restore canonical saved Active live ownership.** The first actual reopen test failed at the absent accepted owner while its filter remained saved. The canonical-only resolver reuses the existing full lifecycle and fixes this without broadening generic recovery or descendant authority.

**[Rule 3 - Integration] Preserve shared recovery contracts after two rejected approaches.** Initial shared Active recovery delegation passed the new tests but performed two retained-record scans; an unchanged existing cost guard failed, observing 2 instead of 1. A second approach restored ordering and reused the private preflight helper, passing 11 retention/recovery tests, but the broader suite found the existing metadata-only recovery-before-disable contract now rejected missing bodies (173 passed, one failed). Root explicitly re-planned after these two failures and obtained a fresh independent plan check before implementation. The final canonical-only resolver preserves all these old tests unchanged. Both shared files and the helper visibility are fully restored. Prior GREEN receipts describe their earlier implementations only.

**[Rule 3 - Test integration] Add narrow cleanup controls and split storage evidence.** Root amended ownership for `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs` solely to add two cfg(test) accessors for existing thread-local cleanup fault/observed-root state. Production code, visibility, limits and child lifecycle are unchanged. Root also approved `packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests/basic.rs` because the existing 604-line test root could not accommodate full physical-preservation evidence without crossing the managed length limit. Existing tests remain intact and the new file has parity breadcrumbs.

The simplification pass leaves one small mode resolver and the existing index lifecycle/owner. No production dependency, crate, schema, network endpoint, authentication path or new unmodeled trust surface is introduced. Stub scanning found no goal-blocking placeholder. Guidance applied: repo AGENTS native verifier/tool pins/parity/LOC contracts, Bright Builds sidecar and standards overrides (no concrete exception), architecture/code-shape/testing/verification/Rust standards, execute-plan context and summary template. Both active lesson inputs were read completely: 8,856 bytes and 2,953 conservative estimated tokens. No project skills were found or lesson/archive files edited.

## Issues and Limits

The first compile-only attempt used an incorrect txid helper name and a private byte reader in test scaffolding; the expanded compile-only attempt omitted the CoinsView trait import. Neither is behavioral RED. An early fault group mis-aimed BeforeBody at indexing reads; that existing control belongs to block writes and was removed from this read-only scenario. The retained failed test is evidence of an unhit test injection, not a production body-read defect. No new body-read hook was added.

All failures and superseded results remain in the receipts below. Final test counts do not include zero-discovery integration harnesses. The 22 final retention cases and the one metadata recovery case are subsets of the 174 final filter regressions and are not added as distinct coverage. The 13 new canonical BASIC functions are separate; their internal four-selector/scenario loops are described above without inflating the test count.

This is macOS/Unix source-derived vector, software fault and durable-byte evidence. It does not renew historical receipts, qualify above-u128 genesis work, establish actual pinned-daemon differential parity or claim hardware/power-loss recovery. Separate arithmetic vectors remain separate. Supported cooperative application leases and the existing engine lock define concurrency; arbitrary direct-engine/filesystem writers, parent/system crash, forced termination and OOM remain outside the inherited guarantee. No command was terminated for quiet output or elapsed estimates; all resumable commands were polled within 60 seconds.

Ordinary descendants, Signet challenge/contextual validation, networking activation, new networks, import/repair and broader canonical pruning remain gated on their assigned future phases. Plans 166-09/13 own operator/contributor docs, 166-11/12 own current source-contract/registration changes, and 166-14/root own the full default native verifier, independent final reviews, LOC and normal-hook saving. No authentication gate or external setup was required.

## Task Commits

None. `commits: false` follows the strict wrapper. Commit-existence checks are inapplicable to this intentionally uncommitted plan.

## Automated Evidence

Every Cargo command ran serially through `scripts/command-timings.ts` with pinned Bun 1.3.9 PATH, the coordinator-approved isolated target and canonical process-local TMPDIR. Logs are ignored and preserved. The commands use `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features FILTER`; final Clippy uses `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings`.

| Check / actual discovery | Actual outcome | Full repository-relative receipt |
| --- | --- | --- |
| Initial scaffold compile | Exit 101; no behavior tests executed | `.local/open-bitcoin-dev/phase166/plan08/baseline.log` |
| First actual `phase166_basic` baseline | Exit 101; 2 passed, 1 failed; Enabled/absence GREEN, saved Active accepted owner RED | `.local/open-bitcoin-dev/phase166/plan08/baseline-behavior.log` |
| First shared implementation | Exit 0; 3/3, 47.88s; superseded | `.local/open-bitcoin-dev/phase166/plan08/owner-green.log` |
| First expanded fault attempt | Exit 101; 8 passed, 1 failed at unhit BeforeBody injection | `.local/open-bitcoin-dev/phase166/plan08/recovery-initial.log` |
| Expanded storage scaffold compile | Exit 101; missing test trait import, no behavior executed | `.local/open-bitcoin-dev/phase166/plan08/recovery-expanded.log` |
| Expanded shared implementation | Exit 0; 13/13, 56.19s; superseded | `.local/open-bitcoin-dev/phase166/plan08/recovery-final.log` |
| Existing single-scan guard RED | Exit 101; 0 passed, 1 failed; observed 2 versus required 1 scan | `.local/open-bitcoin-dev/phase166/plan08/retention-red.log` |
| Second shared implementation retention/recovery | Exit 0; 11/11, 24.06s; superseded | `.local/open-bitcoin-dev/phase166/plan08/retention-green.log` |
| Second shared implementation filter regressions | Exit 101; 173 passed, 1 failed, 119.54s; metadata-only recovery regression | `.local/open-bitcoin-dev/phase166/plan08/filter-regressions.log` |
| Final canonical-only unchanged metadata recovery case | Exit 0; 1/1, 3.46s | `.local/open-bitcoin-dev/phase166/plan08/metadata-recovery-green.log` |
| Final `phase161_retention_recovery_` controls | Exit 0; 22/22, 22.61s | `.local/open-bitcoin-dev/phase166/plan08/retention-final.log` |
| Final `storage::fjall_store::filters::tests` regressions | Exit 0; 174/174, 131.68s | `.local/open-bitcoin-dev/phase166/plan08/filter-regressions-final.log` |
| Final `phase166_basic` canonical vector/fault/reopen cases | Exit 0; 13/13, 60.39s | `.local/open-bitcoin-dev/phase166/plan08/canonical-basic-final.log` |
| Final node all-target/all-feature Clippy | Exit 0; warnings denied, 9.06s | `.local/open-bitcoin-dev/phase166/plan08/clippy-final.log` |
| Shared startup/lifecycle exact baseline restoration | Exit 0; empty diff | `.local/open-bitcoin-dev/phase166/plan08/shared-baseline-restored.log` |
| Final scoped rustfmt check | Exit 0 | `.local/open-bitcoin-dev/phase166/plan08/format-final.log` |
| Final breadcrumbs | Exit 0; 1234 Rust files | `.local/open-bitcoin-dev/phase166/plan08/breadcrumbs-final.log` |
| Final managed checks | Exit 0; zero findings | `.local/open-bitcoin-dev/phase166/plan08/managed-final.log` |
| Final whitespace check | Exit 0 | `.local/open-bitcoin-dev/phase166/plan08/diff-check-final.log` |

The intermediate liveness receipt is `.local/open-bitcoin-dev/phase166/plan08/filter-liveness.log`; that snapshot was taken after the earlier process exited, so it is not claimed as active-process proof. Later progress polling observed the final filter process actively consuming CPU and reporting test completions. Test result/exit status is the acceptance evidence.

## Self-Check: PASSED

All seven changed implementation/evidence paths, this summary and all 20 referenced receipts exist. Final canonical BASIC 13/13, existing filter regressions 174/174, overlapping retention controls 22/22 and metadata-only recovery 1/1 pass. All-target/all-feature Clippy with warnings denied, scoped formatting, 1234-file breadcrumb coverage, managed checks and whitespace review pass. Generic startup/lifecycle have no diff; new Rust files are registered, owned files remain below the size limit, and no unresolved stub or plan-08 issue remains. Frontmatter delimiters are valid. Commit checks are inapplicable because saving is intentionally deferred to root. Root retains subsequent plan acceptance and full native/normal-hook lifecycle closure.
