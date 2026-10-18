---
generated_by: gsd-plan-checker
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T02:38:30Z"
---

## ISSUES FOUND

**Scope:** Amended Phase 165 Plan 14 Task 2 and its cross-crate fixture integration only.
**Issues:** 4 blockers, 1 warning.
**Decision:** Revision required before implementation. The consuming handoff design is feasible, but its planned CLI lifecycle does not deliver the required unchanged assertions.
**Reviewed plan SHA-256:** `3c0570e17c7bc7115297d1b581f9f9e4d778576ec2992bed0e05cdfe0e1a090d`.

This is static source/plan analysis. No builds, tests, application execution, Git operations or implementation edits were performed. The parent owns the already-running workspace diagnostic and its actual exit receipt. Only this report was written. The latest network-unit-test placement amendment is included: `network/tests.rs` and `network/tests/phase165_runtime_fixture.rs`, plus the external `tests/phase165_live_fixture.rs`.

### Blocker 1: CLI work authority and equal-work preference are lost before the planned mutations

Task 2 modifies `packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs`, but the relevant callers in `basic_index.rs` explicitly drop and reopen that runtime:

- `phase161_operator_consumers_genuine_catchup_current_lag_ahead_reopen` constructs the 24-block fixture at line 287, closes it at line 288, and calls `open_configured` before the later real block connection around line 322.
- `phase161_operator_consumers_retained_pruned_stale_and_wallet_body_negatives` constructs the 401-block fixture at line 446, closes it at line 447, and calls `open_with_configured_runtime_activation` before pruning height 20 and the same-height replacement at line 521.

`sync/open_runtime.rs:105`–109 imports a raw core with `Chainstate::from_coins_cache`; opening does not preserve live provenance. The new builder invoked during initial fixture construction cannot authorize either reopened owner. Ordinary physical metadata writes persist unknown counts; strict populated rebind does not normalize those counts. Exposing the count-writing or BASIC attachment helpers would violate the explicitly required boundary.

There is an additional independent tie constraint: `network/work_authority.rs:237`–260 requires the active header linkage to match `ManagedChainstate::maybe_accepted_body_preference()`. Otherwise active order becomes unknown and equal-work preference refuses. Core-only transfer does not carry that network seal. Neither header readmission nor body readmission after raw reopen restores an actual accepted active preference event. The planned earlier disconnected replacement intake works on the original genuine network owner; closing that owner discards its necessary evidence.

The original CLI fixture uses `ConsensusParams { coinbase_maturity: 1, ..Default::default() }` while `SyncNetwork::Regtest.consensus_params()` uses maturity 100. The revision must explicitly state the source parameter preflight and selected fixture parameter policy. A populated maturity-1 replay cannot pass `bind_live_work_source` against maturity-100 parameters. Do not silently change the historical or same-block spends, change their parameters, or omit full source-parameter checks.

**Required change:** Add explicit ownership of the relevant `basic_index.rs` fixture/caller sections and describe a feasible lifecycle before implementation. Keep read-only reopened observations raw. Any post-reopen mutation needs a newly genuine source from actual complete available-prefix validation, exact physical coins/B/heads/positions/undo/count equality and a coherent selected parameter binding. Fixture-only missing-count preparation, if needed, must happen at the genuine original fixture flush with all other facts compared, through existing typed public storage APIs; it cannot become repair during reopen or corruption tests. For the equal-height replacement, retain the real accepted network history until after the replacement, and separately retain meaningful all-handle reopen observations. A same-live BASIC owner needs a concrete permitted assembly path; the cfg-test attachment helper is not available to CLI integration tests.

If pruning or deleted undo makes complete identity impossible at a proposed replay boundary, move the legitimate live assembly earlier; never restore the missing input or weaken the refusal. Preserve the immutable displaced-row assertion, actual pruning/body/wallet negatives, original consensus parameters, historical/same-block spends and actual all-handle observation assertions. Do not substitute a longer branch or invented score/order. If those constraints cannot coexist in the proposed bounded lifecycle, return that concrete incompatibility to the parent before editing.

### Blocker 2: The amended implementation exceeds the checker scope limit

Plan 14 declares 21 modified paths; Task 2 alone declares 17. This exceeds the plan-checker blocker threshold of 15 files and combines three separate proof obligations: production handoff/physical-state guards, downstream benchmark/CLI lifecycle repair, and current-source inventory/mutation-guard renewal. The additional required CLI caller ownership increases the inventory further. Editing in batches of 3–5 does not reduce execution context or supply independent acceptance boundaries.

**Required change:** Split the amendment into sequential bounded execution plans within Phase 165, each below the 15-file blocker threshold. A practical separation is strict handoff plus authority/public-path controls; downstream fixture lifecycle changes; and final source-guard renewal/native evidence closure. Explicitly carry reviewed current-source digest changes and their mutation controls with the corresponding owner changes, preserve shared-file dependency ordering, and prevent final acceptance until every segment is GREEN. This is a Phase 165 execution split, not permission to open Phase 166 or skip the full default verifier.

### Blocker 3: Task 2 automated verification is prose, not an executable contract

The entire `<automated>` body begins “Run focused new public/unit fixture tests…” and supplies no executable command, test target or filter. An executor cannot deterministically demonstrate nonzero discovery, the Node library compilation without cfg(test), or the exact benchmark/operator suite closure from that field. Nyquist being disabled does not remove ordinary task completeness requirements.

**Required change:** Specify separate timing-wrapped Cargo commands for the named unit corpus, `--test phase165_live_fixture` external target, benchmark cases and CLI `--test operator_flows` cases; provide exact current164/current165 Bun test/checker commands, breadcrumbs, applicable lint/build and whitespace checks. Pin the filters or list the intended test names and require actual nonzero discovery. Preserve sequential execution and actual RED/GREEN receipts. A missing-public-method compile RED is useful initial evidence but cannot replace executed behavioral refusal/positive tests after the method exists.

### Blocker 4: Reusing the current delegate leaks store/readiness effects on refusal

The existing `network/runtime_authority/live_fixture.rs::adopt_live_fixture_owner` uses `try_mutate`, whose `mutate` implementation in `runtime_authority/filter_index/readiness/owner.rs:116`–126 unconditionally calls `collect_basic_filter_wakes` after the command returns. That collection invokes transport invalidation and store-backed readiness queries even when the command returned a Fixture-scope or strict-adoption error. Therefore a canonical-target rejection inside the closure does not establish the plan's required pre-read/no-change boundary. This issue was independently raised by the executor and confirmed directly in the source during this check.

**Required change:** Specify direct `lock_authority()` use in the adoption delegate, check immutable Fixture scope first, perform strict adoption, then rotate incarnation. Return every error before readiness collection. Only after successful adoption collect the existing wakes under the authority lock, release that lock, and call `wake_all`. Preserve the existing readiness owner implementation and the already-correct collection-under-lock/waking-after-release contract. Add actual canonical/no-read and refusal/no-readiness-change controls plus an attributed loaded-source mutation that restores the unsafe wrapper. No new generic mutation or readiness API is needed.

### Warning: Task 1 requires closure before Task 2 can fix its diagnosed blockers

Task 1's dependency-graph output, acceptance criteria and done field require “no unresolved goal blocker,” while Task 2 explicitly depends on that task's diagnostic and exists to repair such blockers. This creates an impossible intermediate acceptance condition.

**Required change:** Make Task 1 complete after the actual diagnostic/audit and an explicit inventory of open findings. Put final no-unresolved-blocker closure after Tasks 2/3 and their actual successful verification. Do not mark the phase complete from the diagnostic.

### Handoff boundary findings that pass

The proposed single consuming builder can reuse the existing strict adoption path without a new validator or ordinary opener. `validate_live_origin` correctly accepts a genuine empty owner even when `has_earned_work` is false. `bind_live_work_source` is available for pure source/parameter preflight. `rebind_into` consumes the source, compares complete effective state, rejects preview targets and installs fresh owner/generation only after equality. Existing managed adoption checks the actual physical Fjall parent, pending validation and guarded metadata/undo/count identity; the amendment explicitly closes effective-versus-physical coins/B/heads masking.

The corrected explicit-lock ordering is feasible: take the network authority mutex, require immutable Fixture scope, then take the publication guard. The current try_mutate wrapper must first be replaced as described in Blocker 4. `load_chain_meta_for_open`, `load_all_undo_records`, Fjall `complete_snapshot` and cache `complete_snapshot` do not reacquire that publication mutex. Keep the physical comparison and consuming rebind within that same guard. Do not introduce a convenience reader that takes the mutex again. Incarnation replacement remains after successful rebind, with no newly fallible step after authority replacement.

Remove module-level cfg-test gating only for strict handoff compilation; preserve method-level cfg-test gating on missing-count enrichment, earned-work observation and BASIC attachment. The external integration target must call the public builder with the library compiled normally, while network unit tests can access private scope/preview/parent fixtures without new exports. Public-path positive/refusal tests plus actual loaded-source mutations are appropriate coverage.

The consuming signature drops both `self` and `source` on error; tests can retain an existing cloned handle/store observer to prove unchanged target authority and physical facts. Do not claim the runtime is returned on refusal, rollback of pre-existing startup effects, or a zero-effect backend opener. Document the consuming error semantics with the public method.

The disconnected-body strategy itself is valid before closing the genuine owner: `connect_stored_block` binds the live source and calls `retain_checked_candidate` before returning `Disconnected` for a non-genesis body on an empty active chain. With admitted accepted parent headers, the replacement can be genuinely checked pending ancestry, then receive its real earlier sequence at parent-body release. No second preference API or caller work inflation is required.

### Coverage, decisions and project rules

The amendment supports CWOR-02/CWOR-03 and D-06/D-07 without changing the existing CWOR-01/CWOR-05 arithmetic/refusal contract. D-09/D-11/D-12 require all four canonical ordinary refusals, default-off networking and Phase 166 canonical bootstrap/exact reopen/genesis BASIC ownership to remain closed. Nothing in this review authorizes canonical checkpoint acceptance, ordinary recovery, a new production Bitcoin dependency or daemon differential claims. The blockers concern the concrete fixture lifecycle and execution proof, not permission to weaken those decisions.

The checked dependency on completed Plan 13 is consistent; no new inter-plan cycle was found. The latest fixture unit placement is appropriate for private-field controls. No contradictory data transform or alternate work representation is proposed. The amendment needs its builder/test artifacts and wiring carried into the split plans' must-haves rather than only the final acceptance-document artifacts.

AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md and the architecture, code-shape, verification, testing, Rust and TypeScript standards materially informed this check. Both active lesson files were loaded completely: global 5,230 bytes and repository 1,958 bytes; total 7,188 bytes / 2,397 conservative estimated tokens. No archive input was loaded. No project skills were found in `.claude/skills` or `.agents/skills`.

Dimension 8: SKIPPED (nyquist_validation disabled). Research's listed open questions carry explicit RESOLVED markers. This bounded recheck does not redo or close the independent whole-phase source/security/native gates.

### Structured issues

```yaml
issues:
  - plan: "165-14"
    task: 2
    dimension: cross_plan_data_contracts
    severity: blocker
    description: "CLI callers close the genuine runtime before later mutation; reopened raw work and unknown active body preference cannot satisfy preserved connect/equal-work assertions."
    fix_hint: "Own the basic_index.rs callers and specify a coherent complete-source/physical-count/parameter lifecycle; keep equal-work acceptance on genuinely retained accepted network history and preserve meaningful raw all-handle observation checks."
  - plan: "165-14"
    task: 2
    dimension: scope_sanity
    severity: blocker
    description: "21 plan paths and 17 Task 2 paths exceed the 15-file plan blocker threshold, before adding the necessary CLI caller changes."
    fix_hint: "Split into bounded sequential Phase 165 execution plans with explicit owner/fixture/guard dependencies and final native closure."
  - plan: "165-14"
    task: 2
    dimension: task_completeness
    severity: blocker
    description: "Task 2 automated verification contains instructions but no runnable command or deterministic nonzero-discovery targets."
    fix_hint: "Supply separate timing-wrapped exact Cargo unit/external/benchmark/CLI targets plus exact Bun guards, lint/build, breadcrumb and whitespace commands."
  - plan: "165-14"
    task: 2
    dimension: key_links_planned
    severity: blocker
    description: "Existing try_mutate delegate collects store-backed BASIC readiness and invalidates transports even when Fixture/adoption checks return Err."
    fix_hint: "Use the existing direct authority lock; return errors before wake collection, then collect only after successful handoff under lock and wake after releasing it, with real no-read/no-change controls."
  - plan: "165-14"
    task: 1
    dimension: task_completeness
    severity: warning
    description: "Task 1 requires no unresolved blocker before the dependent repair task can run."
    fix_hint: "Accept the diagnostic plus explicit open findings in Task 1; require final closure after repair and full verification."
```

Revise and independently recheck before implementation. Keep the current diagnostic source frozen until its actual exit is collected.
