---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T03:21:33Z"
---

# Phase 165 integration repair design

This is the design/ownership handoff for Plans 15–16. The parent independently checks these plans before any execution and owns Plan 14 dependency/wave repair and shared planning state. Only this ledger and the two new plans were written by the planner. No implementation, build, test or Git saving was performed.

## Evidence and hardest constraint

The completed Plan14 diagnostic, as reported by its executor, is exit101: 4,461 passed,249 failed,6 pre-existing ignored across41 test groups. Actual logs are `.local/open-bitcoin-dev/phase165-plan14-preflight-diagnostic.log`, with249 panic records/46 canonical-root fixture clusters in `phase165-plan14-failure-roots.json` and definition-file mappings in `phase165-plan14-failure-owner-map.json` in the same directory. These are diagnostic failures, not a success receipt. Plans15–16 address the external constructor/benchmark/CLI clusters only; the other fixture clusters require their own bounded ownership and evidence.

The hardest constraint is retaining genuine SAME Fjall core/counts and normal BASIC ownership in a normally compiled external library consumer. An already-open raw runtime plus pure-core handoff does not solve it: raw recovery lacks origin, persisted normal counts are None, enabled BASIC lacks same-live attachment, CLI maturity1 differs from default100, and dropping the network loses the accepted body-linkage seal needed for equal ties.

## Selected public contract

```rust
DurableSyncRuntime::from_live_fixture_source(
    source: ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
    config: SyncRuntimeConfig,
    consensus_params: ConsensusParams,
    basic_filter_mode: BasicFilterStartupMode,
    relay_activation: RelayActivationConfig,
    block_relay_activation: BlockRelayActivationPolicy,
    inbound_enabled: bool,
) -> Result<DurableSyncRuntime, SyncRuntimeError>;
```

The existing ManagedChainstate is the typed source bundle, keeping this at seven arguments without a new options hierarchy or Clippy suppression. Derive actual store through source.store().inner(); inspect pending publication before consuming. Pure origin/full-params/canonical-root checks use borrowed source core before snapshot/store effects. SAME genuine source remains mandatory even when independent replay would succeed; no raw promotion, destination rebind, count write or test-helper export.

Confirmed alias vulnerability: public FjallCoinsView.batch_write[\_with_persist_mode] can change coins with SAME B through actual begin/completeCoinsWrite. This advances BASIC publication revision but does not invalidate CoreWorkOrigin or validation-history coverage. A clean genuine source and fresh physical complete_snapshot may both read the SAME poisoned rows, while ScriptsValid/positions/undo/counts remain unchanged. Complete source==physical comparison alone is insufficient, including with validation coverage checks.

Plan15 therefore requires, AFTER the mandatory pure genuine/params/canonical gate, publication-guarded independent FULL retained active-prefix validation using the EXISTING Chainstate engine in fresh Memory coins with runtime P2SH flags and the same explicit parameters. Every retained active body including genesis and every required physical undo must exist and match; missing/pruned history refuses before initialization with no restoration. Independently derived full coins/B/heads/positions/undo/counts must match genuine source and physical facts. Physical None counts stay unknown solely because SAME genuine source retains counts; conflicting Some refuses. This is fixture physical identity validation through the existing validator/arithmetic, not another validator, origin upgrade or ordinary reopen authority.

Capture owned independent expected identity under initial guard: actual retained active body bytes/headers, positions, undo, complete coins/B/heads, counts and validation metadata required for BASIC assembly. Release guard for normal initialize_configured, reacquire and compare source plus current physical/body/validation facts against that CAPTURED immutable expectation. Never compare only two fresh aliased views. No new revision getter: PublicationControl.revision is storage::fjall_store-private, and normal init legitimately changes BASIC revision. Keep existing normal BASIC owner/proof guards. No claim of ABA/revision-history detection when every final captured fact is identical.

Deterministic interleave stays within12 owned files: private capture/final-check production helpers in new live_fixture.rs are called by the actual public constructor. Unit control runs those SAME stages around actual initialize_configured and an actual ordinary clone batch_write changing a coin with SAME B; final check must reject against capture while source remains Live. Separate loaded-source controls verify actual public constructor call order. Report this honestly as executed production STAGE interleave plus structural public source-order proof, not executed whole-public-constructor concurrency. No callback/test gate/new public source argument/type or publication-module expansion is required.

Existing public source construction was inspected with the Plan14 executor: fresh empty Fjall target receives a genuine empty pure core by consuming rebind; existing ManagedChainstate wraps it; actual managed genesis connection returns Ok and has published validation history; into_parts returns SAME core plus sink; existing public FlushLifecycle.execute_flush writes actual undo/bodies/positions with header_entries=[]; the same sink/core are rewrapped. Raw save_header_entries would invalidate validation coverage and is not a substitute. Only extract after successful managed acceptance. The constructor then runs normal initialize_configured, passes SAME core into from_recovered_chainstate, and reuses normal BASIC/network/announcement assembly with full explicit parameters.

Source ownership is consuming on error. Retained observers prove unchanged invalid-SOURCE preflight state; canonical/origin/params/full-prefix/physical mismatch refusal occurs before normal initialization. A post-init concurrent-write or startup/I/O error may follow actual initializer effects; no rollback/zero-effect guarantee or return of consumed source is claimed. An already-created backend is not a zero-effect opener. The existing cfg-test adoption delegate's readiness-on-Err bug is not reused or modified.

Canonical coverage is mandatory four ACTUAL constructor routes, not predicates or conditional reachability. For each actual Mainnet/Testnet3/Signet/Regtest genesis use explicit TEST fixture params pow_limit_bits=genesis.header.bits and enforce_bip34_height_in_coinbase=false with remaining defaults. Actual managed connect must return Ok, source Live/full-param bind must succeed, and constructor must return its canonical-specific refusal with zero constructor physical-read/init counters and preserved state. Earlier origin/params/height0-target/BIP34 failure does not count. These fixture parameters never alter product validation or ordinary profiles.

## Source graph for later strict fixture extensions

Work provenance alone does not establish body-ancestor linkage. Plan15 must prepare an UNWRAPPED Fixture ManagedPeerNetwork via existing pub(crate) from_initialized_chainstate, validate captured actual active headers once with existing public direct try_admit_fixture_headers/full explicit params, establish unique root linkage through existing pub(crate) retain_checked_candidate(root), and admit EVERY historical NONroot active body with existing pub(crate) retain_cached_candidate. That uses the same actual body validators but keeps historical order sticky unknown; graph setup never writes maybe_accepted_body_preference or reconnects an active tip to mint preference. Genesis-only source has known unique root and no historical nonroot unknown ancestor, so genuinely NEW A/B events can earn actual order and accepted active binding. Populated historical source supports strict extensions and refuses unknown equal ties, including through duplicates/copies.

Visibility/locking is feasible within12 paths: network.chainstate() is an existing public immutable accessor returning ManagedChainstate, so final captured source/physical/body/validation comparison can run AFTER initialize/configure AND graph construction before handle wrapping. Do not hold publication guard over graph admission/header publication or later handle/readiness mutations. Already-owned open_runtime.rs splits concrete private prepare_initialized_network/finish_initialized_runtime helpers; ordinary route calls both consecutively unchanged, fixture route inserts validated graph and final check between them. No new network field/accessor, generic callback, public cache/order API or unowned module is required. Existing normal BASIC/proof initialization stays in finish after publication guard release.

Plan15 adds actual public populated strict-extension/unknown-equal refusal and unit genesis-only NEW A1/A2/B2 preference controls, plus real source mutations omitting header/body validation, treating cached history as known, minting accepted tip or moving final check before graph. Executed deterministic stage interleave now includes actual graph before clone writer/final captured check; whole-public-constructor concurrency remains unclaimed. This supplies Plan24's bounded post-reopen fixture requalification graph without recovering destroyed arrival order.

Dependent Plan16 correction is authorized within its SAME8paths: populated constructor already bound header owner, so future CLI headers use actual public connect_outbound_peer + receive_sync_message Version/Verack/Headers with same params. For401 blocks send contiguous A1..A400 FIRST and [B400] separately SECOND; no root P2P Headers, no B appended after A400 in one batch, no repeated try_admit_fixture_headers. Assert real accepted header/proof facts; only after BOTH batches retain B body pending, before any A child bodies. Modeled peer cleanup uses existing public disconnect_peer; no sockets or new API. Empty benchmark remains genuinely empty with later explicit root admission. Plan16 guards mutate this real ingress/order contract alongside its preserved consumer assertions.

## Bounded dependency and ownership

| Plan   | Needs                                      | Creates                                                                      | Wave | Files | Tasks | Checkpoint |
| ------ | ------------------------------------------ | ---------------------------------------------------------------------------- | ---- | ----- | ----- | ---------- |
| 165-15 | Completed165-13 and parent-reviewed design | Public same-core constructor, private/external controls, current guards      | 14   | 12    | 3     | none       |
| 165-16 | Actual165-15 GREEN                         | True empty-source benchmark and original-owner CLI lifecycle, current guards | 15   | 8     | 3     | none       |

The two plans overlap only sequentially on current guard files. Parent must move Plan14's final native acceptance after all repair plans, including any additional fixture clusters; this planner does not edit Plan14 or claim its current wave is globally final. No task/metadata commit is authorized inside these plans. Parent later performs default native, independent review/security/formal and normal-hook commit/push on main, stopping before166.

## Decision coverage

This is additive integration coverage over completed Plans01–13, not a replacement/reduction of their decisions.

| Decision | Full existing owner                          | Added repair ownership                                                                          | Coverage       |
| -------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------- | -------------- |
| D-01     | Existing ProofWork/source contracts          | No new arithmetic/dependency                                                                    | Full, retained |
| D-02     | Plans01–03 exact arithmetic/codec            | No numeric behavior change                                                                      | Full, retained |
| D-03     | Plans01/04 selected proof/admission          | Explicit params and actual mined/admitted fixture headers in15/16                               | Full           |
| D-04     | Plans01/12 independent vectors               | Immutable corpus/receipts preserved in15.3/16.3                                                 | Full           |
| D-05     | Plans04/09 strict header best-order          | Nonroot header benchmark16.1 preserves real selector                                            | Full           |
| D-06     | Plans06/08/09 body eligibility               | Real pending B400 release before A400 in16.2                                                    | Full           |
| D-07     | Plans07/08 genuine source/effect owner       | SAMEcore constructor15.1/15.2; original owner/ZERO carrier16.2                                  | Full           |
| D-08     | Plans08/13 bounded ties                      | No accepted binding from Duplicate, no restart tie reconstruction in16.2                        | Full           |
| D-09     | Plans10/11 read-only ordinary gate           | All-four canonical pre-init refusal15.1/15.2; no ordinary route change                          | Full           |
| D-10     | Plans02/03 legacy classification             | No raw promotion or historical rebaseline in15.3/16.3                                           | Full, retained |
| D-11     | Plans10/11 preservation/default-off evidence | Default-off and physical refusal controls15; actual prune/body/wallet/all-handle observations16 | Full           |
| D-12     | Plans10–13 explicit scope                    | Both plans preserve166/167/168 deferrals and local daemon evidence limits                       | Full           |

## Review issue disposition

| Rejected amendment issue                                    | Concrete repair                                                                                                                                                            |
| ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI closes genuine owner before later mutation              | 16.2 owns basic_index.rs; all actual work mutations precede final all-handle drop, raw observations/refusal follow.                                                        |
| Maturity1 versus100; BASIC same-live attachment unavailable | 15 consumes genuine source with explicit full params before normal BASIC init; no attachment/count helper exposure.                                                        |
| Excess21/17-file scope                                      | 12-file/3-task Plan15 and8-file/3-task Plan16, serial shared guards.                                                                                                       |
| Verification prose/no nonzero targets                       | Each task supplies executable timed Cargo or exact pinned Bun commands and actual discovery acceptance.                                                                    |
| try_mutate performs readiness effects after Err             | New constructor accepts no existing runtime and never uses that delegate; pure/source/physical checks precede normal init. Existing private delegate remains out of scope. |
| Task1 closure precedes its own repair                       | Diagnostic completion and open findings remain valid; whole-phase no-blocker closure belongs after repairs and default native gates.                                       |

## Premortem and retained limits

1. A benchmark could pass through preconnected Duplicate. Plan16 requires a genuine empty source/BASIC Disabled, initial no tip, Version/Verack/actual genesis Block and final exact tip/work/counter/persistence checks.
1. CLI equal work could appear legitimate using only copied sequence numbers or root Duplicate. Plan16 requires actual pending B400 NonExtending intake, A399 release before A400, genuine A400 acceptance binding and unchanged replacement bytes; Duplicate earns linkage only.
1. Current guard renewal could erase evidence boundaries. Both plans preserve original identities, six default registrations, fixed bounded ordered reads and actual loaded-owner mutations with real positive baselines before review/digest renewal.
1. Source and physical snapshots could alias poisoned rows without losing Live origin. Plan15 requires existing-engine independent full-prefix expected facts; public clean-cache alias extra/missing/changed attacks, existing masked attacks, missing-body/undo cases, captured-final stage interleave and actual source mutations prove the independent comparison and final capture survive.

Canonical bootstrap, exact durable/all-handle authority and genesis BASIC remain166; contextual consensus remains167; actual pinned-Knots daemon qualification remains168. The32 local refusal subprocesses remain local Open Bitcoin evidence. No deterministic restart tie or production-funds claim follows.

Material guidance: repo AGENTS timing/native/breadcrumb contracts; Bright Builds sidecar and placeholder-only overrides; architecture/code-shape/verification/testing/Rust/Bun standards. Both active lesson inputs were read completely:5,230 global+1,958 local=7,188 bytes,2,397 conservative estimated tokens; no archives loaded, and existing below-threshold audit baseline needs no new audit. No project skill directories were found. Level0 discovery uses inspected existing source and the GSD planning/revision workflow; no library research is needed.
