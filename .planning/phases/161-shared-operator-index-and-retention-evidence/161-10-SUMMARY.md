---
phase: 161-shared-operator-index-and-retention-evidence
plan: "10"
subsystem: networking
tags: [basic-filters, peer-capability, configuration, bounded-observation]
requires:
  - phase: 161-01
    provides: Identifier-free shared BASIC status contract
  - phase: 161-03
    provides: Achieved retained-record publication accounting
provides:
  - Checked lifecycle-maintained five-counter peer capability scope
  - Resolved BASIC indexing and general-serving facts independent of health
  - Actual durable startup, saved reopen and failed-effect configuration evidence
affects: [161-04, 161-05, 161-08, 161-09]
tech-stack:
  added: []
  patterns: [Checked old/new session deltas, construction-time configuration capture]
key-files:
  created:
    - packages/open-bitcoin-network/src/peer/basic_operator.rs
  modified:
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-network/src/filter_serving/capability.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/inbound_state.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-network/src/peer/inventory_state.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/limited_serve.rs
    - packages/open-bitcoin-node/src/network/peer_network_clone.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/sync/open_runtime.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Keep permission scope, actual immutable session offers, configured indexing and general serving separate.
  - Retain requested all provenance alongside explicit blockfilters without changing grant policy.
  - Count only modeled implicit-origin sessions; currently zero because whitelist provenance is unsupported.
  - PreserveSaved without validated effective ownership remains unknown, even after an earlier fresh disabled startup.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T06:12:21Z"
duration: 14min including context and focused evidence
completed: 2026-10-11
---

# Phase 161 Plan 10: Peer Scope and Resolved Configuration Summary

**Session transitions now maintain five checked scalar counters, while actual startup and serving setters retain resolved BASIC configuration independently of offers, backend health and synchronization.**

## Accomplishments

Both tasks completed. Fifteen source/manifest paths changed, plus this summary. No staging, commit, push or shared-tracker mutation occurred; the strict parent workflow owns those actions and final lifecycle gates.

`PeerManager::maybe_basic_peer_scope() -> Option<BasicPeerScope>` copies the publicly re-exported five-counter type. Each counter is `u64`: `general_sessions`, `explicit_sessions`, `all_sessions`, `implicit_sessions`, and `offered_sessions`. Inbound admission records known explicit/all scope before a local offer; version emission captures general/offered scope using the existing capability policy. Outbound insertion, inbound insertion, version/disconnect outcomes and actual removal apply checked old/new deltas. Duplicate insertion remains refused. Removal followed by re-add is the supported replacement path. Self-connection and duplicate-version disconnects revoke their contribution once; later cleanup does not underflow. Any arithmetic underflow/overflow makes the entire observation sticky unavailable until a new manager is constructed. Collection traverses no sessions and allocates no peer table.

Mixed explicit `blockfilters` plus `all` retains both requested facts. The new `all_permission_requested` boolean is populated only by the existing `PeerPermissionSet::requested_token(All)` path; effective grants, prerequisite checks and service policy are unchanged. General toggles preserve existing version offers. An `all` offer does not imply an enabled or working backend.

`implicit_sessions` means modeled sessions with proven implicit origin. The current permission grammar has no implicit whitelist resolver or provenance, so that count is zero. Ordinary peers and explicit `mempool,noban,addr,relay` sets are **not** relabeled implicit. Tests prove those non-filter permissions do not confer a BASIC grant. Whitelist compatibility remains deferred under Phase 160's scope; this observation makes no broader whitelist claim.

`ManagedPeerNetwork::maybe_configured_basic_indexing() -> Option<bool>` and `configured_basic_peer_serving() -> bool` expose retained intent. Compatibility constructors capture their supplied local-service intent once and leave indexing unresolved. `set_basic_filter_serving(bool)` records its actual input; `set_prune_mode` preserves that retained serving fact. The existing staged clone copies both fields. `record_basic_indexing_configuration(Option<bool>)` is crate-visible for Plan 04 to update **after successful** runtime enable/disable effects.

Actual `DurableSyncRuntime::open_with_configured_runtime_activation` records Enabled/Disabled explicitly after successful `initialize_configured`. PreserveSaved calls `FjallNodeStore::resolved_basic_filter_indexing_after_startup()` once under the existing publication guard. This bounded effective-owner read returns Active/Disabled intent or unknown absence, propagates errors, performs no forest validation/recovery and never runs during status polling. Index state, service advertisement and initial synchronization do not supply configuration. Fresh Disabled startup is known false during that run; if it materialized no saved owner, a later PreserveSaved construction is correctly unknown.

## Verification

All Cargo work used pinned Bun 1.3.9, Rust 1.94.1 and serialized `scripts/command-timings.ts run`; no full native verifier or Bazel build was run in this plan.

| Check | Actual result |
| --- | --- |
| Task 1 RED, `phase161-peer-scope-red` | Initial absent aggregate APIs failed compilation; timing record exit 101. |
| Task 2 RED, `phase161-peer-config-red` | Eight absent configuration API diagnostics, exit 101. |
| `cargo test -p open-bitcoin-network phase161_peer_scope` | 7 relevant tests passed; final full suite below also includes them. |
| `cargo test -p open-bitcoin-node --all-features phase161_peer_config` | 6 relevant tests passed, 1,594 filtered out. |
| Full network `cargo test --all-features` | 605 unit + 2 parity + 2 property tests passed; 0 doc tests. |
| Node `--all-features phase157_activation_configured` | 11 real startup/prune/history/fault regression tests passed, 1,589 filtered out. |
| Network Clippy, all targets/features, `-D warnings` | Passed. |
| Combined network/node Clippy, all targets/features | Only two inherited dormant retention APIs failed; exact diagnostics below. |
| Scoped rustfmt check and `git diff --check` | Passed. |
| Parity breadcrumb checker | 1,106 Rust paths passed. |
| Managed starter checks `all` | 1,439 source files, 3 repository lessons, zero findings. |
| Phase 160 source/claim guard | 38 production edges and 46 executable selectors passed. |
| Phase 157 source/claim guard | Inherited lifecycle refactor assertion failed; parent owns compatibility repair. |

The peer tests exercise ten permission/general cases, both fields of mixed explicit/all provenance, refused duplicate insertion, remove/reconnect replacement, failed handshakes, duplicate-version disconnect, unknown cleanup, immutable general offers, manager clone/copy and 64 repeated reads. Ten arithmetic cases independently overflow/underflow each of the five fields and prove sticky unavailability. A test deliberately clears the internal table without updating counters to demonstrate that the getter cannot reconstruct scope from table contents.

Node tests use a genuinely staged/validated proof-of-work genesis, saved durable coins metadata and real Fjall bodies. Enabled/Disabled construction, actual all-handle PreserveSaved reopen, absent saved ownership, an injected BeforeCommit disable failure and known serving-only compatibility construction exercise production paths. Permissioned `all` still offers BASIC while general serving remains false; prune updates and clone behavior preserve the independent facts. The injected error proves failed effects do not erase enabled intent; it is not a physical-device or stronger crash-window claim.

Fixture corrections during focused execution replaced an incorrect `PruneMode::Manual` spelling with `ManualOnly`, added a collision-free per-process directory suffix, and seeded a real lifecycle before expecting a saved Disabled owner. The production semantics were preserved.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-10-1: offers become invented configured/working capability | Existing grant policy reused; requested all retained; separate stored indexing/serving facts; actual startup/reopen/backend-failure and permissioned-general-false tests. |
| T-161-10-2: session scope drifts or arithmetic wraps | Checked old/new lifecycle updates, duplicate refusal, failed-handshake/remove/reconnect tests and every-field underflow/overflow controls. |
| T-161-10-3: polling traverses or copies peer data | Five scalar counters; constant-cost getter; no table inspection, peer IDs or raw permissions in the returned type. |

No new endpoint, authentication route, database/schema, production dependency or file-access trust boundary was introduced. The startup getter reads existing guarded ownership only. New observations contain safe booleans/counts and do not activate networking, query arbitrary filters, recover history, clear faults, advance durability or release prune protection. Growth/soft-target accounting and body/wallet eligibility remain untouched. D-16 exclusions, Phase 162 proof ownership and existing readiness/public-default/funds-safety/crash-window claim limits remain intact.

## Deviations and Simplification

1. **[Rule 3 - Blocking; parent authorized]** Added narrow changes to `filter_serving/capability.rs` and `lib.rs` for requested-all provenance and a clearly public aggregate type. The original capability discarded the combined all/explicit distinction.
2. **[Rule 3 - Blocking; parent authorized]** Copied the two new facts in `network/peer_network_clone.rs`; otherwise the existing staged clone would lose configuration.
3. **[Rule 3 - Blocking; parent authorized]** Added the startup-only guarded getter in `storage/fjall_store/filters/startup.rs`, because existing node-visible progress/state accessors cannot distinguish disabled retained records or absent ownership. All earlier recovery/accounting edits in that file were preserved.
4. **[Rule 3 - Verification; parent authorized]** Added a cfg(test)-only safe configuration tuple accessor in `network/runtime_authority.rs` so actual durable constructors could be checked under the existing lock. No production API or second status capture was added.
5. Duplicate session additions are intentionally refused by the existing API; replacement evidence uses actual remove/re-add transitions. No new replacement API was invented.

The simplification pass kept version handling and `snapshot_own_services` in their existing guarded production functions. Only focused inbound construction and scalar transition logic live in the new helper. This avoids a wrapper dispatch method and preserves historical serving guards. All source files remain within managed limits (`peer.rs` 622 lines; `runtime_authority.rs` 627). There is no second peer cache, permission parser, index-health resolver or recovery scan.

Material guidance: repository AGENTS/sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized phase context/research/UI contract and prior summary contracts. Both active lesson inputs total 7,188 bytes / 2,397 conservative tokens; no audit trigger or archive loading was needed.

## Deferred Issues and Parent Handoff

- The combined strict Clippy command reports only `retention.rs:113` (`RetainedBasicFilterObservation::measurement` never used) and `retention.rs:145` (`FjallNodeStore::basic_filter_retention` never used). Plan 04's production projection consumes these inherited APIs; no suppression was added.
- The Phase 157 checker reports `T-157-30/D-04: ordinary boundary packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs:enable_basic_filter_index`. Plan 02's guarded refactor predates this plan; the parent assigned compatibility repair separately. No Plan 10 lifecycle implementation edits were made.
- Plan 04 must record indexing configuration only after successful actual enable/disable transitions, then project this same captured scope/configuration into shared status. Full `bash scripts/verify.sh`, independent source/security review, lifecycle verification, requirement activation, tracking updates and commits remain parent/Plan 09 responsibilities.

## Known Stubs

None. The documented zero implicit-origin count reflects the intentionally unsupported whitelist model, not fabricated implicit provenance or an unfinished Plan 10 implementation.

## Self-Check: PASSED

New helper and summary exist. All named source paths exist; the final owned diff is clean of whitespace errors and the final parity/managed checks pass. Recorded focused and regression runs discovered nonzero tests. No task commits were expected or created under the parent-owned strict finalization rule. The inherited node-lint/source-guard limitations are explicitly recorded above and do not claim a passed final phase gate.
