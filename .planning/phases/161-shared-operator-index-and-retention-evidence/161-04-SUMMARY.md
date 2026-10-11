---
phase: 161-shared-operator-index-and-retention-evidence
plan: "04"
subsystem: node
tags: [basic-filters, operator-evidence, fjall, bounded-observation, lifecycle]
requires:
  - phase: 161-01
    provides: Typed independent BASIC availability contract
  - phase: 161-02
    provides: Complete recovery-owned retained observations
  - phase: 161-03
    provides: Achieved immutable publication deltas
  - phase: 161-10
    provides: Resolved configuration and constant-size peer provenance counters
provides:
  - Authoritative read-only BASIC projection with branch and fenced-state coherence
  - Bounded single-publication capture preserving independent retained presence and totals
  - Conservative configuration uncertainty after failed host transitions
affects: [161-05, 161-06, 161-08, 161-09]
tech-stack:
  added: []
  patterns: [Existing authority-to-publication lock order, fixed metadata admission, pure typed projection]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/operator_snapshot/tests.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
  modified:
    - packages/open-bitcoin-node/src/network/operator_snapshot.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/network/limited_serve.rs
    - packages/open-bitcoin-node/src/network/tests/filter_capability.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/retention.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Current requires the earned initial-sync latch; zero instantaneous lag alone remains catching up.
  - Failed enable or disable records unknown configuration because an error can follow durable effects.
  - Retained presence remains observed when complete validation exceeds optional measurement budgets.
  - Diagnostic capture-budget refusal leaves operational state unavailable without inventing an owner failure.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T06:36:37Z"
duration: 21min evidence window
completed: 2026-10-11
---

# Phase 161 Plan 04: Authoritative BASIC Projection Summary

**One existing network/publication authority now projects configured capability, earned synchronization, accepted target, verified progress, detailed failure, retained growth and reserved protection without status-triggered effects.**

## Accomplishments and API

Both planned tasks are complete across eleven source/manifest paths. `ManagedNetworkOperatorSnapshot::basic_index() -> &BasicIndexOperatorStatus` exposes the owned typed section for Plan 05 and later consumers. The existing generic store borrow, `ChainstateStore::maybe_basic_filter_query_store`, supplies the same Fjall handle; no trait bridge, downcast, second store open, database, dependency, lock or authority was added.

`FjallNodeStore::basic_filter_operator_observation()` captures retained measurement, independently observed retained presence, bounded authority facts and work counters under one existing publication guard. Its injected-limit variant supports exact/one-under tests. Production admits at most eight metadata reads and 4 KiB of metadata; the implementation makes exactly two point reads, for existing state and lifecycle values. It inspects borrowed Fjall value lengths and cumulative checked byte arithmetic before decoding or copying values. Oversized values are rejected before decode/copy. There is no peer, immutable-record, history, body, undo or lock-map traversal and no filter payload copy. Every integration helper capture checks BASIC JSON at most 4 KiB and metadata work bounds; the serialized schema contains only bounded enums, booleans, heights and aggregate counts.

The capture compares persisted lifecycle, saved endpoint, protection and fence against the existing validated publication identity and its current revision. Owner progress additionally requires matching generation, branch identity, processed endpoint, safe checkpoint, authoritative accepted target and actual active-chain tip, with no suspended publication or invalidated owner. It never authorizes progress from height equality alone. Mismatches expose invalidated/unavailable progress while preserving independently known latch, accepted target, configuration, failure and retained evidence. Backend poison downgrades the BASIC group while the existing outer authority retains its original failure behavior.

Detailed failures come directly from `AcceptedBasicIndexFailure`: body, undo, persistence, invalid facts, fact limit and invalidation remain distinct. Coarse pause labels are not used to infer them. Current requires both zero lag and an earned initial-sync latch. Processed-ahead guidance leaves the safe durable height unchanged. Protection uses exact valid inclusive bounds; starts beyond the representable final bound expose height-space exhaustion rather than lowering the start. Failed or invalidated progress does not clear faults, advance durability, release protection or initiate work.

Retained presence is independent of both owner failure and optional measurement availability. Mandatory complete validation with an exhausted optional measurement budget still proves records were observed, while byte/count totals remain explicitly unavailable. Raw integrity revocation and poisoned control cannot expose previously earned totals. The old `basic_filter_retention()` survives only as a justified `cfg(test)` adapter to the same production capture; production uses `measurement()` through the coherent collector. Both earlier dormant-method strict Clippy failures are eliminated without suppression or dummy calls.

Successful host enable/disable calls record their resolved configuration after completion. Every transition error records unknown configuration because the existing fallible API provides no proof that effects did not occur. Real BeforeCommit, AfterDisable, AfterRelease, BeforeEnable and AfterEnable controls exercise this uncertainty. Status never calls either setter.

## Verification Evidence

All Cargo commands ran serially through `bun run scripts/command-timings.ts run --key KEY -- ...`, with `/tmp/open-bitcoin-bun-1.3.9-fresh` prepended to PATH. It was re-probed as Bun 1.3.9, matching `.bun-version`. Timing records are ignored checkout-local diagnostics under `.local/open-bitcoin-dev/command-timings/`; durations below are actual wrapper durations.

| Key / exact check | Actual result | Duration |
| --- | --- | --- |
| `phase161-authority-red`: node `phase161_operator_authority` | Compilation failed with three missing `basic_index()` diagnostics; no tests discovered | 3,877 ms |
| `phase161-authority-green`: same selector | 1 discovered, 1 passed | 13,416 ms |
| `phase161-authority-matrix-final` | 14 discovered; 13 passed, one incorrect completed-reorg expectation failed and was corrected | 25,280 ms |
| `phase161-authority-expanded` | 14 discovered, 14 passed | 18,601 ms |
| `phase161-authority-complete` | 16 discovered, 16 passed | 21,822 ms |
| `phase161-authority-final`: node `--all-features phase161_operator_authority` | 17 discovered, 17 passed | 21,549 ms |
| `phase161-authority-latch-red`: node `--all-features phase161_operator_authority_zero_lag` | 1 discovered, failed actual Current-versus-CatchingUp assertion | 7,782 ms |
| `phase161-authority-release`: node `--all-features phase161_operator_authority` | Final 18 discovered, 18 passed | 17,553 ms |
| `phase161-authority-storage-regression`: node `--all-features storage::fjall_store::filters::tests` | 167 discovered, 167 passed | 104,911 ms |
| `phase161-authority-peer-regression`: node `--all-features storage::fjall_store::filters::query::peer::tests` | 6 discovered, 6 passed | 15,037 ms |
| `phase161-authority-reorg-regression`: node `--all-features storage::fjall_store::filters::reorg::tests` | 3 discovered, 3 passed | 2,320 ms |
| `phase161-authority-input-regression`: node `--all-features storage::fjall_store::filters::turn_inputs::tests` | 1 discovered, 1 passed | 525 ms |
| `phase161-authority-config-regression`: node `--all-features phase161_peer_config` | 6 discovered, 6 passed | 2,922 ms |
| `phase161-authority-capability-regression`: node `--all-features network::tests::filter_capability::` | 7 discovered, 7 passed | 261 ms |
| `phase161-authority-fact-boundary-regression`: node `--all-features phase157_accepted_fact` | 3 discovered, 3 passed | 138 ms |
| `phase161-authority-clippy-release`: node `--all-targets --all-features -- -D warnings` | Passed on final source | 5,466 ms |
| Scoped Rustfmt, edition 2024, `skip_children=true`, all ten touched Rust paths | Passed final check | Passed |
| `bun scripts/check-parity-breadcrumbs.ts --check` | 1,108 Rust paths verified | Passed |
| `bun scripts/bright-builds-check.ts all` | 1,439 source inputs, zero findings; three repository lessons, zero findings | Passed |
| `git diff --check`, owned diff and simplification review | Passed | Passed |

The four storage selectors partition the original broader `storage::fjall_store::filters::` scope: 167 + 6 + 3 + 1 = 177 passing cases, preserving all 176 earlier controls plus the new presence/measurement-budget test. The excluded inline cases were query-peer, reorg and turn-input controls, not a retention test module. No broad suite was repeated only to obtain another aggregate count. The cumulative startup-validation scan instrumentation was left untouched.

The final authority matrix uses genuine validated spend histories, durable Fjall coins, configured startup, ordinary scheduled turns, actual accepted connects, genuine flush acknowledgement and a completed same-height replacement branch. It proves unearned catch-up, current status, earned latch with later lag, processed ahead of durable, disabled no-genesis without invented zero progress, body versus undo loss, persistence poison, transition uncertainty, retained evidence during failure, displaced rows counted after reorg, exact metadata budgets and raw invalidation. The completed reorg correctly reports lag eight and the verified common prefix at height fifteen; it does not reuse the displaced tip as current.

At validated prefixes of sixteen and 128 blocks, 64 repeated polls preserve every key/value in all nine namespaces, the owner, validation-scan counter and immutable integrity point-read counter. Controlled private InvalidFacts, FactLimit and Invalidated injection checks otherwise impractical owner classifications against genuine retained evidence. Exact/one-over accepted-fact byte/item/overflow controls separately pass; this plan does not claim to allocate a 384 MiB invalid accepted fact set. Pure controls cover same-height different hashes, lifecycle/publication generation mismatch, publication branch mismatch, suspended/integrity-invalid captures, active-target mismatch, empty-prefix genesis lag and all three high protection boundaries. A real `replace_validated_branch` value provides the zero-lag/unearned-latch regression.

The first expanded compilation used an incorrect hash constructor spelling; it was corrected before discovered tests ran. Initial strict lint found a new production `expect` and a test default reassignment; both were simplified, and strict node lint subsequently passed twice. A temporary Cargo artifact-lock wait resolved normally. All running sessions were polled; no job was terminated because of a yield or duration estimate.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-04-1: incoherent or mutating authority/publication capture | Existing network-to-publication lock order; revision, persisted fence, full branch/generation and endpoint comparisons; real reorg/raw/poison tests; all-nine-namespace equality after repeated polls. |
| T-161-04-2: peer/protection disclosure | Five aggregate scope counts copied from Plan 10; only validated scalar protection bounds; internal hashes stay internal; no keys, scripts, filter bytes, backend messages, peer IDs or raw lock names enter BASIC status. |
| T-161-04-3: unbounded status work | Two bounded metadata gets, checked cumulative 4 KiB admission before decode/copy, injected exact/one-under bounds, oversized-value refusal, no record/history/peer/lock walks and bounded JSON. |
| T-161-04-4: capability becomes false operating authority | Configuration, provenance, latch, accepted target, verified progress, failure and retention have independent availability. Backend faults preserve known configuration; ambiguous lifecycle effects make it unknown. Zero lag without the earned latch remains catching up. |

No new endpoint, authentication route, persisted schema or file-access trust boundary was introduced. The existing operator snapshot boundary contains the new observations; no additional threat flag was identified.

## Deviations and Simplification

1. **[Rule 3 - Blocking; parent authorized]** Extended `filters/retention.rs` and its existing tests for one guarded scalar/metadata capture. The previous scalar getter could not prove publication generation, branch, persisted fence and retention in one lifetime; the complete ownership loader would walk lock maps and records.
2. **[Rule 2 - Correctness; parent authorized]** Updated the single existing `limited_serve.rs` failure test to expect unknown configuration. This supersedes Plan 10's historical claim that an arbitrary failed transition proves enabled intent unchanged. No new effect receipt or architectural service was added.
3. **[Rule 3 - Maintainability; parent authorized]** Moved pure tests into `network/operator_snapshot/tests.rs` when the new latch control would push the production entry file over the managed size threshold. Both new Rust paths have parity breadcrumbs. The final-native correction appendix records the canonical test-path rename.
4. **[Rule 1 - Bug]** A zero-lag branch replacement can preserve an unearned initial latch. An actual failing pure-domain regression established the erroneous Current label; the projection now remains CatchingUp until the latch is earned.

The simplification pass retained the existing borrowed store seam, one coherent capture, two metadata reads and small pure mappings. Fallible protection-range construction replaced both clamping and a production panic. No cache, refresh owner, parser, getter side effect or renderer-owned state model was added. Material guidance loaded included repository AGENTS/sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized context/research/UI and preceding summaries. Both complete active lesson inputs totaled 7,188 bytes / 2,397 conservative tokens; no archive was loaded or lesson file edited.

## Known Stubs and Residual Limitations

No stubs prevent this authority plan's goal. Memory/unsupported stores and an absent live owner retain unavailable live facts; the known no-genesis disabled fixture never invents a height or earned latch. Plans 05–06 own carrying this producer into the wider operator/RPC/rendering consumers, and Plan 08 owns cross-consumer evidence. This summary does not close CFOP-01 or activate requirements.

The final full native verifier, independent source/security review, formal lifecycle gate, tracker updates and all commits remain parent/Plan 09 responsibilities. D-16 exclusions, Phase 162 integrated-proof ownership, body/wallet eligibility and soft block/undo targets remain intact. No archive-scale, public-default, production-service, production-readiness, funds-safety or stronger v2.4 crash-window claim is made.

## Task Commits

None by design. This executor staged, committed and pushed nothing, and did not modify STATE, ROADMAP, REQUIREMENTS, configuration or todo tracking.

## Self-Check: PASSED

All eleven source/manifest paths and this summary exist. The final authority tests discover eighteen cases and pass; all regression partitions and strict lint pass. Both new Rust files are registered, scoped formatting and whitespace checks pass, and no implementation placeholder was found. Commit checks do not apply because the parent explicitly owns finalization.

## Final Native Correction: Canonical Test Path — 2026-10-11T08:51:59Z

The parent's first full native attempt failed after 10m 51.964s at `scripts/check-panic-sites.sh`, before its Cargo stages. The guard intentionally recognizes canonical `*/tests.rs` and `*/tests/*` paths; it does not follow the parent module's `#[cfg(test)]` into an arbitrarily named child. A standalone pre-edit run reproduced the same seven test-only `expect` findings in `network/operator_snapshot/basic_tests.rs`.

Renamed that child to `network/operator_snapshot/tests.rs`, changed the parent declaration to `#[cfg(test)] mod tests`, and updated only its existing parity-manifest entry and this summary's path references. No assertion, test body, production behavior, panic allowlist or checker changed. The child file's SHA-256 is identical before and after the rename: `bb80882d322ef3567841b339e23ae50d53a92e13137506956021ea843b8ba85c`. All other frozen phase changes were preserved.

| Correction check | Actual result | Wrapper duration |
| --- | --- | --- |
| `phase161-authority-canonical-tests`: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase161_operator_authority -- --nocapture` | 18 discovered, 18 passed; renamed pure cases execute under `network::operator_snapshot::tests` | 19,140 ms |
| `phase161-authority-canonical-panic`: `bash scripts/check-panic-sites.sh` | Passed: no unclassified production panic-like sites | 99,364 ms including cooperative-lock wait |
| `phase161-authority-canonical-clippy`: node `--all-targets --all-features -- -D warnings` | Passed; compiler work 5.61s | 92,214 ms including cooperative-lock wait |
| Scoped Rustfmt check for parent and renamed child | Passed | Passed |
| `bun scripts/check-parity-breadcrumbs.ts --check` | 1,122 current Rust paths verified | Passed |
| `git diff --check`, old-path absence and new-path existence | Passed | Passed |

All timed commands used Bun 1.3.9 through the prepared PATH and the repository timing runner. Cargo jobs did not overlap. The panic guard's live filesystem enumeration and cooperative-lock waits completed normally; no process was terminated. This correction adds no new threat surface or claim and does not rerun or replace the parent-owned full native gate. No staging, commit, push or tracker mutation occurred. Correction self-check passed; source is released for the parent's second full native attempt.
