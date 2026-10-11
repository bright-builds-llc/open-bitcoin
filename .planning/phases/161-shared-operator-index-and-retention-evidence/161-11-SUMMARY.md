---
phase: 161-shared-operator-index-and-retention-evidence
plan: "11"
subsystem: node
tags: [basic-filters, immutable-reads, sticky-faults, operator-evidence, fjall]
requires:
  - phase: 161-04
    provides: Coherent bounded BASIC operator capture
  - phase: 161-05
    provides: Existing shared RPC and CLI forwarding contract
provides:
  - Source-local typed immutable read fault observations under the existing guard
  - Sticky fixed failure precedence and revoked read, append, progress and retention trust
  - Genuine immediate RPC, final barrier, managed peer, recovery and policy-negative evidence
affects: [161-06, 161-07, 161-08, 161-09]
tech-stack:
  added: []
  patterns: [Transient stack Cell observation, existing publication guard, exclusive verified reset]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/retention.rs
    - packages/open-bitcoin-node/src/network/operator_snapshot.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/recovery.rs
key-decisions:
  - Only source-local backend, codec and required-parent observations latch; callback error classes never classify faults.
  - Integrity dominates Backend; either observed category dominates ordinary owner pause evidence.
  - The first observed fault revokes read and append authority once; invalidated rereads do not renew or upgrade evidence.
  - Closing recovery and host-transition failure retain the previous sticky category; complete successful verification can reset despite optional measurement exhaustion.
requirements-completed: []
review-remediation: [WR-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T07:15:05Z"
duration: 19min evidence window
completed: 2026-10-11
---

# Phase 161 Plan 11: Sticky Immutable Read Failure Evidence Summary

**Genuine BASIC immutable read failures now revoke cached trust and remain visible as fixed failed operator evidence across RPC, final barrier and managed peer reads.**

## Accomplishments

All three tasks are complete within the fourteen existing owned Rust paths. No new module, manifest, public field, persisted schema, dependency, database, lock or index owner was added. The existing `basic_index()` group and RPC/CLI forwarding continue unchanged. Root owns independent finding closure, full native verification, formal requirement closure and finalization.

The private `BasicFilterReadFault::{Backend, Integrity}` lives with the existing publication epoch helpers. Each query wrapper creates one stack `Cell<Option<BasicFilterReadFault>>`, borrows it into its guarded reader, ends that borrow, then folds its fixed observation into the already-held mutable publication control. This avoids recursive locking. The test point adapter uses the same producer. Original caller errors survive even if checked revocation also poisons exhausted authority.

On the first genuine observation, the shared control revokes read integrity and complete retained accounting, advances its checked integrity epoch and invalidates append identity, pending coins and completed metadata proof. Old work tokens cannot publish a partial append or mint replacement work while a read fault remains. Repeated invalidated reads neither advance the epoch again nor convert a Backend observation to Integrity. A later genuine ownership decode fault can upgrade Backend to Integrity without a second revocation; the executed source-merge test proves that distinction.

Operator capture copies the optional private category under the same publication guard. Projection consumes it before ordinary owner coherence: both categories produce Failed, their existing fixed failure/next-action category, and unavailable current processed/durable/lag and complete retained presence/totals. RequiredBody, RequiredUndo and FactLimit do not mask a discovered read fault. Earned initial synchronization, accepted target and known configuration remain distinct. One scalar derived protection observation is captured from the previously verified owner before append proof revocation; raw/recovery invalidation and host transitions discard it conservatively, and it grants no pruning or work authority.

## Fault Source and Reset Inventory

| Source or boundary | Classification / reset rule |
| --- | --- |
| RPC and peer ownership/state backend gets | Backend observation only on actual get error. |
| RPC target/parent get; peer `peer_row` get | Backend observation after admission and before returning the original error. |
| Existing `BeforeQueryRead` seam | Backend at the same source; private marker never enters BASIC status. |
| Ownership/state decoding, admitted truncated envelopes, `parse_record`, `verify_parent`, `checkpoint_identity` | Integrity at the actual validation source. |
| Required predecessor absent for a present target | Integrity; ordinary target absence remains typed absence. |
| Envelope/hex/hash/copy bounds, peer work budgets, wire payload, captured-branch mismatch, disabled/stale/readiness/client refusal | No observation. Even Corruption-typed callback policy failures leave healthy evidence intact. |
| Explicit initialization | `reset_verified_forest` follows artifact-absence proof and successful guarded batch completion. Orphans cannot authorize empty totals. |
| Compatibility checkpoint publisher | Reset follows complete forest/projection and candidate validation plus successful batch completion; record-only partial publication never resets. |
| Recovered append installation / host enable | Existing complete forest/projection/fence/history preflight and successful publication precede installation. The outer guarded host wrapper retains/restores the previous fixed category on any later failure. |
| Direct disable | Full forest/projection checks and successful lifecycle/release effects precede final reset. Failed effects cannot clear the original sticky category. |
| Configured startup | Existing validation passes remain unchanged. The wrapper resets after its closing epoch advance succeeds; any error restores the prior sticky category and revokes retention/read trust. |

All intermediate helper work and restoration occurs under the same exclusive publication guard, so no caller can observe a temporarily cleared failure before a later closing error. Configured and direct BeforeEnable/AfterEnable/BeforeDisable/AfterDisable/AfterRelease controls explicitly assert the selected injection was consumed, rather than merely accepting an earlier unrelated failure. Closing epoch exhaustion is independently executed. Optional observation-budget exhaustion still permits completed mandatory validation to renew read trust while measured totals stay unavailable.

## Executed Verification

All Cargo commands ran serially through `bun run scripts/command-timings.ts run --key KEY -- ...`, using Bun 1.3.9 from `/tmp/open-bitcoin-bun-1.3.9-fresh`, matching `.bun-version`. Rust is pinned to 1.94.1. Sessions were polled without terminating jobs on yield or duration estimates.

Exact final Cargo commands, each run through its tabled timing key:

```bash
cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase161_read_fault -- --nocapture
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase160_budgeted_read -- --nocapture
```

Every regression table row uses the same exact test command with its listed selector substituted for `phase161_read_fault`.

Meaningful RED: `phase161-read-fault-storage-red` discovered two tests, both failed the actual old-epoch-current assertion before implementation (8,006 ms). Subsequent storage GREEN discovered two and passed (14,426 ms). Two initial Rust visibility errors were corrected without expanding source ownership. RPC caller expansion passed before peer/recovery expansion. Three peer-test setup expectations were corrected to match established transport behavior: another request needs an independent peer slot, a handed frame remains reserved until settlement, and a retired suffix returns Invalidated. No production transport behavior was changed for those fixture corrections.

The ten-selector planned regression run passed **114 tests** with nonzero discovery for every selector:

| Node `--all-features` selector | Passed | Wrapper duration |
| --- | ---: | ---: |
| `phase161_read_fault_storage` | 2 | 9,562 ms |
| `phase161_read_fault_projection` | 1 | 263 ms |
| `phase161_read_fault_callers` | 9 | 21,545 ms |
| `phase159_rpc_owner_faults` | 10 | 5,779 ms |
| `phase159_basic_readiness` | 21 | 16,517 ms |
| `phase160_managed_completeness` | 6 | 14,611 ms |
| `phase160_query_epoch` | 7 | 5,510 ms |
| `phase159_basic_query_measure` | 3 | 29,366 ms |
| `phase161_operator_authority` | 18 | 13,487 ms |
| `phase161_retention` | 37 | 34,609 ms |

After a semantics-preserving query-loader simplification and explicit Corruption-typed policy negative assertions, the final source passed:

| Final check / timing key | Actual result |
| --- | --- |
| `phase161-read-fault-clippy-release`: node `--all-targets --all-features -- -D warnings` | Passed, 7,567 ms |
| `phase161-read-fault-final`: node `phase161_read_fault` | 12 discovered, 12 passed, 29,688 ms |
| `phase161-read-fault-peer-regression`: node `phase160_budgeted_read` | 6 discovered, 6 passed, 6,842 ms |
| Scoped Rustfmt, edition 2024, `skip_children=true` | All fourteen paths passed check mode |
| `bun scripts/check-parity-breadcrumbs.ts --check` | 1,109 Rust paths verified |
| `bun scripts/bright-builds-check.ts all` | 1,439 inputs; zero findings; three repository lessons; zero findings |
| `git diff --check`, owned diff/source review and stub scan | Passed; no implementation stubs found |

There are **120 distinct selected test cases** across the twelve new controls and 108 historical cases. The final twelve-case rerun repeats the three new focused selectors and is not counted as twelve additional distinct cases.

The genuine caller matrix executes four faults (CorruptTarget, CorruptParent, MissingParent and BeforeQueryRead) through both immediate RPC and an earned barrier's actual final immutable read. Six genuine retained-authority combinations cover both read categories over RequiredBody/RequiredUndo/FactLimit. Each failure helper performs 32 repeated captures, confirms exactly two metadata gets and at most 4 KiB input/output, and compares every key/value across all nine namespaces, owner progress and cumulative immutable read/validation counters. Captures preserve safe scalar facts and never expose the injected private marker or block identifiers. Storage controls separately prove one epoch revocation across clones and blocked partial publication.

The managed-peer matrix executes the same four faults through real accepted/Fjall fixtures and actual preparation. A response earned beforehand, including a handed frame, becomes stale. Its remaining suffix is retired; before acknowledgement its progress contains zero invented bytes. The isolated full-write acknowledgement records only the achieved physical frame as PhysicalPrefixInvalidated, never CurrentRangeComplete. All request/frame reservations settle, and disconnect cleanup remains empty. This is explicit receipt/state-machine evidence, not a claim of a live socket write.

Negative controls execute unknown block, ordinary missing target, cancelled wait and readiness ID exhaustion, raw/stale final completion, disabled and initial catch-up absence, invalid/unsupported managed request, all seven peer admission axes, oversized envelope, malformed captured branch, Corruption-typed envelope/hex/hash callbacks, large predecessor admission and maximum-codec wire refusal. None latches a read fault. Recovery controls execute failed same-authority corruption recovery, metadata changes and blocked partial work, mandatory validation with optional measurement exhaustion, original-token nonrevival, and actual all-handle configured reopen after transient backend failure. Historical RPC controls also execute actual corrupt-parent reopen refusal.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-11-1: discovered fault followed by trusted status | Source-local enum, sticky merge, first revocation, genuine before/error/after/repeated operator captures and owner-pause precedence. |
| T-161-11-2: client or resource activity poisons shared index | No callback-error/string classification; executed absence, stale, invalid, Corruption-policy, work-axis, oversize and wire negatives. |
| T-161-11-3: private storage detail disclosure | Private fixed enum; unchanged fixed public categories; bounded serialized BASIC status excludes backend marker and identifiers. |
| T-161-11-4: stale authority or incomplete recovery renews trust | Revoked epoch/append/work proof, real stale prepared receipt/suffix controls, consumed closing host faults, closing epoch exhaustion, failed corruption recovery and verified reopen. |
| T-161-11-5: status scans or recursive locks | One existing guard and one transient scalar Cell; exactly two status metadata gets; unchanged rows/counters after polls; no status record/history/peer/body/undo/lock walks or filter copies. |

No new endpoint, authentication path, schema or filesystem boundary was introduced. No additional threat flag was identified. Security enforcement and independent final verification remain root-owned.

## Deviations and Simplification

No source ownership expansion was necessary. The private fixed protection scalar is derived evidence preserving an independently earned bound after append identity revocation; it is discarded on possible host/raw changes and cannot authorize any effect. Existing publication and recovery owners retain their responsibilities. The explicit simplification pass kept the transient Cell and fixed enum, simplified lifecycle branching and retained one projection mapping; it added no generic cache, retry worker, callback-error classifier, recovery scan or renderer resolver.

Material guidance: repository AGENTS/sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/local-guidance/Rust standards, Phase161 context/research, Plans04/05 summaries, WR-01 and the actual query/readiness/serving/observer APIs. Both canonical active lesson inputs were read completely (7,188 bytes; 2,397 conservative estimated tokens); no archive or lesson mutation was used. The existing audit baseline did not trigger another audit. Project skill directories are absent. Work followed the parent's GSD execution context and ownership contract.

## Known Stubs and Residual Limits

No stubs prevent this plan's goal. This change records discovered faults; it does not proactively discover unseen corruption, repair data, download history, alter protection or advance safe progress. Ordinary target absence intentionally remains absence. Status boundedness applies to the BASIC observation group, not unrelated existing snapshot collectors. Measurement limits are structural/local policy, not archive-scale or latency guarantees.

The focused WR-01 independent re-review, remaining consumer plans, full `bash scripts/verify.sh`, full-phase source/security review, formal lifecycle/requirement closure and final staging/commit/push remain parent-owned. CFOP-01 remains pending. D-16 exclusions and Phase162 ownership remain intact; there is no public-mainnet, production-readiness, funds-safety or stronger crash-window claim.

## Task Commits

None by the explicit outer-wrapper contract. This executor staged, committed and pushed nothing and did not modify STATE, ROADMAP, REQUIREMENTS, config, todo or lesson tracking.

## Self-Check: PASSED

All fourteen planned source paths and this summary exist. Nonzero focused and historical selectors passed, final strict lint and scoped formatting passed, registered breadcrumbs and managed checks passed, and the owned source/diff review found no placeholder or new trust boundary. Commit checks are inapplicable because the parent owns finalization.

## Correction WR-02 | 2026-10-11T08:31:59Z

The independent full review found a separate healthy-to-failed-disable gap. The earlier failed-host controls began with an already-observed read fault, so they did not prove revocation when disable itself first discovered invalid storage. `disable_basic_filter_index_guarded` advanced the epoch and invalidated append proof before validation, but those operations preserved read integrity and Complete retention. An undiscovered corrupt forest or checkpoint could therefore make direct/managed disable fail while old growth remained available and a new read epoch could still be issued. This correction supersedes any implication that the earlier matrix covered that healthy starting state.

The production correction changes one line in `filters/lifecycle.rs`: replace `control.advance_integrity_epoch()?` with `control.invalidate_read_integrity()?` before `invalidate_append` and every fallible owner/forest/checkpoint operation. The existing helper performs the same single checked epoch advance and additionally revokes read/retention trust. It preserves the optional sticky fault category. No new scan, authority, publication effect or public field is added; only existing successful complete paths restore trust.

Two new tests use the existing `filters/tests/retention/recovery.rs` and `filter_index/serving/tests.rs` paths. The storage matrix begins healthy without a fault-discovering query and exercises hidden immutable forest corruption, checkpoint projection corruption, owner decoding corruption and the genuine BeforeDisable backend seam through both direct and configured disable (eight scenarios). It asserts unavailable old complete growth, denied new read authority, rejected old token, no fabricated query-fault category, exactly one direct or two pre-failure configured epoch advances, and the existing zero/one validation-scan counts. The separate genuine accepted/Fjall managed fixture discovers corruption during actual managed disable and proves unavailable retention/presence and denied old/new epochs through the existing operator authority.

The meaningful RED discovered one test and failed `failed disable must revoke previously healthy read authority` before the production fix. GREEN discovered both new tests and passed. All Cargo commands used the pinned Bun timing runner and cooperative lock; no native verification, commit, push or tracking mutation was performed.

| Timing key / node selector | Actual result | Duration |
| --- | --- | ---: |
| `phase161-disable-validation-red` / `phase161_disable_validation` | 1 discovered, failed actual healthy-authority assertion | 8,374 ms |
| `phase161-disable-validation-green` / `phase161_disable_validation` | 2 discovered, 2 passed | 13,409 ms |
| `phase161-disable-regression-phase160_query_epoch` | 7 discovered, 7 passed | 5,018 ms |
| `phase161-disable-regression-phase161_retention` | 37 discovered, 37 passed | 34,294 ms |
| `phase161-disable-regression-phase161_read_fault` | 12 discovered, 12 passed | 24,702 ms |
| `phase161-disable-regression-phase161_operator_authority` | 18 discovered, 18 passed | 14,276 ms |
| `phase161-disable-clippy` / node all-targets/all-features strict Clippy | Passed | 6,255 ms |

The exact test command is `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features SELECTOR -- --nocapture`; the strict lint command remains the exact command above. This correction executed 76 passing cases (two new plus 74 targeted regressions), bringing the distinct selected cases documented across both evidence windows to 122 without double-counting reruns. Scoped Rustfmt passed all three touched paths, whitespace/source review passed, parity verified 1,122 current Rust paths, and the managed checker reported 1,439 source inputs with zero findings. The two test files remain below the managed limit at 596 and 621 lines.

Executor-run historical source checks passed for Phases157/159/160. Phase155 reported a stale Disabled guarded-body contract and five derivative conditional-validation/release diagnostics; Phase156 reported stale T-156-08 guarded-body anchors, both due to the intentional stronger revocation method. These guard files are parent-owned and were not edited by this executor. Root owns their narrow source-anchor/mutation-suite refresh and final evidence in the guard compatibility report; those initial lexical failures are not a native verification pass.

Subsequent root-owned evidence at 2026-10-11T08:32:28Z: the four-path Phase155/156 guard refresh passed 273 combined tests (155: 151; 156: 122), 544 assertions and zero failures in 45.92 seconds; both live checkers exited zero. Root recorded this separately in `161-GUARD-COMPATIBILITY.md` under healthy-start revocation. These are root-executed results, not additional executor-run Rust tests or a full native pass.

This addresses full-review WR-02 and threat mappings T-161-02-1/T-161-11-4. Independent re-review, refreshed historical guards and final full native/source/security/lifecycle closure remain parent-owned. No requirement is marked complete. Self-check of the three correction paths and this appended evidence passed; source remains frozen for the reviewer.
