---
phase: 165-shared-exact-work-authority
plans_checked: ["165-21", "165-22", "165-23", "165-24"]
generated_by: gsd-plan-checker
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T03:28:32Z"
initial_check_at: "2026-10-17T03:08:41Z"
status: passed_conditional
blockers: 0
warnings: 0
initial_blockers: 1
resolved_issues: [B1]
---

# Phase 165 Plans 21–24 Check

## PLAN CHECK PASSED — current revision

**Four plans, twelve tasks, zero current blockers and zero warnings.** B1 is resolved by the amended Plan 24 Task 2 plus the finalized Plan 15 source-graph contract. This is a conditional plan pass: execution must first deliver actual Plan 15 and predecessor GREEN evidence, and final Plan 14 native acceptance must follow all repairs. It is not source implementation or runtime qualification.

The initial B1 finding, initial checked hashes and original reasoning below are retained as historical attribution. The appended revision recheck records the current verdict and current identities.

## ISSUES FOUND

Four plans, twelve complete tasks; **one blocker, zero warnings**. Plans 21–23 are coherent within this bounded static check. Plan 24 requires the explicit black-box restart repair below before execution. This report checks intended repairs, not delivered implementation or successful native acceptance.

## Blocking revision

**B1 — Plan 24 Task 2 leaves actual post-reopen block mutations without a genuine source.**

The action at `165-24-PLAN.md:187` says “Reopened serving/query observations remain raw and do not need live mutation authority.” The actual `packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs` closes the original runtime/store at lines 111–113, opens a raw runtime at lines 115–124, and then at lines 133–138 connects the entire maturity suffix through `restarted_context.connect_local_block`. These are actual coins/work mutations, before the transaction and socket-serving assertions. The initial genuine-empty construction does not survive that all-handle close.

Source tracing confirms this is a required authority path: `open-bitcoin-rpc/src/context/network.rs:426` forwards to the shared handle, `open-bitcoin-node/src/network/runtime_authority.rs:474` forwards to the actual managed owner, and lower `chainstate/work_authority.rs:23` calls `bind_live_work_source` before retention/staging. A raw reopened owner cannot pass that check. The observed diagnostic currently stops earlier at the missing initial tip (`phase127_composition.rs:70`); resolving that first failure will expose this existing later operation. This is a static prediction, not an invented executed panic.

**Feasible public handoff, preserving the after-reopen trigger:** replay every actual retained active body into `Chainstate::new()` through public `connect_block_with_current_time` under the original full fixture parameters and flags. Obtain the resulting genuine `snapshot()`. Public `Chainstate::from_parent(store.coins_view(), validated.active_chain, validated.undo_by_block, validated.maybe_confirmed_txid_counts)` can create a still-untrusted Fjall target using only that independently validated metadata; `genuine_memory.rebind_into(&mut target)` then consumes the genuine Memory owner and checks complete physical coins/B/heads plus exact derived positions/undo/counts before installing fresh Live ownership and its selected parameters. These signatures exist at `open-bitcoin-chainstate/src/engine.rs:126` and `engine/work_authority.rs:241`. Public `ManagedChainstate::from_chainstate(FjallChainstateStore::from_store(store.clone()), target, lifecycle)` can wrap that same genuine Fjall core for the provisional Plan 15 constructor. That constructor must independently validate the actual retained prefix and compare actual physical metadata/undo/body/coins before configuration; physical persisted None counts remain None and are not written or normalized. Fail closed on missing/corrupt/pruned inputs. The raw source remains unearned until successful validation and consuming handoff. This explicit fixture path is feasible under D-07, is extension-only, and restores no historical tie order or ordinary reopen acceptance. No shared helper or source implementation is added by this check.

**Minimal required revision:** explicitly own the maturity-loop repair in the existing `phase127_composition.rs` inventory. Specify either a separately genuine complete available-prefix source and exact identity handoff before the first post-reopen connect, with a concrete externally callable API/count contract; or perform the real maturity connections while the first genuine source is alive, flush them, truly drop every database/authority/context handle, and keep the reopened chain path read-only. If choosing the latter, retain actual transaction admission and all serving behavior and explicitly assert the final tip/work/coins identity across the real reopen. Adjust derived child heights/parents after adding an explicit model root; do not simply reuse heights starting at 1 if the scripted accepted block is now height 1. Keep seven achieved block writes, mixed transaction/block/NotFound ordering, real listener/authenticated HTTP, shared-context identity, stale-status override, cleanup/join, and all response-key/evidence assertions. Do not drop the maturity loop, lower maturity to evade it, retain a Fjall handle across the claimed close, promote raw snapshots/counts, or widen ordinary recovery. If a new helper is necessary, replan its exact ownership below fifteen files before editing.

```yaml
issues:
  - plan: "165-24"
    task: 2
    dimension: key_links_planned
    severity: blocker
    description: "The black-box composition test connects maturity blocks after raw all-handle reopen, but the plan supplies authority only before the initial sync and incorrectly treats the reopened phase as serving/query-only."
    source_evidence:
      - "packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs:111"
      - "packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs:115"
      - "packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs:133"
      - "packages/open-bitcoin-node/src/chainstate/work_authority.rs:23"
    fix_hint: "Explicitly repair the maturity suffix using a genuine complete-prefix source before mutation, or move its real connections before a flush and true all-handle close and preserve exact reopened identity plus all subsequent transaction/socket/RPC assertions."
```

## Scope and evidence

Read all four plans in full, including the parent's formatting-only correction restoring standalone closing threat-model tags. Loaded `AGENTS.md`, the Bright Builds sidecar, placeholder-only overrides, standards index, architecture/code-shape/verification/testing/Rust/TypeScript guidance, both complete active lesson files, Phase 165 context, relevant 07–09 summaries, acceptance evidence, actual diagnostic failure records and the local failure-owner map. No project skill directories were present. The active lessons are 5,230 global bytes plus 1,958 repository bytes: 7,188 combined bytes, 2,397 conservative estimated tokens, no omitted active blocks. Parent orchestration retains lesson-audit ownership.

The actual diagnostic is RED: exit 101, 17m4.509s, 4,461 passed / 249 failed / six old ignored, forty-one result groups and forty-six shared panic roots. Attributing both test-definition paths and owning panic/helper paths gives **90 unique failed cases** for this scope. Nested timing cases and listener/HTTP/daemon clients are counted once under their fixture owners, not added again as independent roots.

| Plan | Shared group | Attributed failed cases | Tasks | Declared files | Wave | Dependency | Static result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 165-21 | E: protocol ingress/lifecycle | 15 | 3 | 13 | 20 | 165-20 | Coherent |
| 165-22 | F: event loops/timing/logging | 19 | 3 | 9 | 21 | 165-21 | Coherent, constructor prerequisite retained |
| 165-23 | G: progress/restart | 12 | 3 | 11 | 22 | 165-22 | Coherent |
| 165-24 | H: RPC/listener/prune/composition | 44 | 3 | 13 | 23 | 165-23 | B1 requires revision |

All twelve `auto` tasks have nonempty Files/Action/Verify/Done and automated commands. Static `gsd-tools verify plan-structure` returned valid with no errors/warnings for each current plan. The dependency chain is sequential and acyclic within this scope; the shared five checker files are serialized. Declared inventories remain below fifteen, with no automatic helper/split/breadcrumb expansion permitted. Current owned Rust/TypeScript files are at most 628 lines; the tight source owners include announcement transport (602) and coins-worker filters (588), so execution must honor the explicit cap/replan gate. This report does not approve expanding those inventories.

## Goal and boundary checks

| Check | Finding |
| --- | --- |
| Requirements | All four plans declare CWOR-02/03/05. Their tasks repair actual protocol/connected-work consumers while preserving CWOR-05 refusals. CWOR-01 arithmetic is predecessor scope and remains protected; its absence from these four repair frontmatters is not a phase coverage gap. No relevant new PROJECT requirement is silently reassigned here. |
| Decisions D-01–D-04 | Existing ProofWork, parent-derived work, selected target/hash admission and full-width expected work are retained; no second arithmetic engine or caller-score authorization is planned. |
| Decisions D-05–D-08 | Plan 21 uses the existing sealed reorg path and genuine earlier side-body order; header and body arrival remain distinct. The per-step replacement/parent/child lifecycle assertions remain manual after actual lower prepare/commit. Public lower unqualified reorg remains refused. Unknown reopened arrival order cannot mint a tie. |
| Decisions D-09–D-12 | Four canonical selectors remain refused, observations stay unavailable where unproved, networking stays default off, historical identities stay frozen, and 166–168 remain deferred. B1 is the sole missed raw-reopen mutation path found. |
| Source/data contracts | Same-source construction uses actual selected parameters, actual managed history and real flush; no count normalization, missing/pruned-input repair, raw snapshot promotion, public cfg-test count writer or BASIC attachment is authorized. |
| Goal-derived artifacts | The planned protocol, event-loop, progress, HTTP/listener/cache and socket artifacts support observable behavior and specify actual wiring. B1 leaves one necessary connection unplanned. |
| Verification | Literal timing-wrapped Cargo targets/filters are executable forms, each has one filter or an unfiltered target, and nonzero discovery/no-new-ignore evidence is required. No test/build was run by this checker. |
| Current source guards | Tasks require only reviewed changed digests, actual enabled test owners, ordered pre-read inventory, real loaded-source mutations with asserted replacement anchors, attributed findings and a positive loaded-source baseline. Historical 162/163, the protected Node preservation source, all six mandatory lines and the older independent controls remain immutable. |
| Research/Nyquist | Research's four open questions each carry RESOLVED decisions. Dimension 8: SKIPPED (`workflow.nyquist_validation` is false). |
| Repo conventions | Pinned tools, serialized timing-wrapped Cargo/Bazel, default native closure, breadcrumbs, cap enforcement, simplification/diff review and no executor commits are explicit. The report itself adds no implementation exception. |

Plan-specific mechanics checked against source intent and actual diagnostic gates:

- **21:** The redundant source header re-admission actually follows successful local connects and yields `header-owner-already-bound`. The future body case must perform a real handshake and selected child header ingress, then assert exact nested time-too-new at the deliberately early body timestamp. The maintenance case currently invokes the unqualified lower reorg and must earn earlier body order before the original suffix, then use `authorize_reorg` → `retain_authorized_reorg` → lower prepare/commit; using the public combined lifecycle helper would erase the intended intermediate assertions. Protocol roots must be explicitly admitted outside P2P even when a null-parent header is already known. Invalid/malformed child cases preserve real retry/inflight cleanup, exact InvalidBlock/Validation classifications and no-credit effects.
- **22:** Actual initial acceptance failures precede announcement writes, compact timeouts and snapshot/log assertions. The factories must earn a genuine source under the actual parameters before any connection. The maturity-1 compact spend case must select those parameters before genesis/header binding and preserve normal-maturity cases; read-only child modules still provide the event-loop assertions. Structured log scripts keep four actual Version/Verack/Headers/Block messages using an accepted model root and real child; actual derived work/hash/heights replace stale synthetic values without faking log events or counters.
- **23:** The twelve actual failures arise at the unchanged exact confirmation-count assertion. New-path genuine seeds persist the real complete prefix and actual counts; raw request/status reopen uses header-only admission with explicit unearned-work assertions. Actual reopen/drop, NoProgress/help, useful-work, branch-awaiting-bodies, retry/backoff, cancellation, resource and fault behaviors remain. A later mutable reopen needs actual available-prefix revalidation and strict equality after real live-flush enrichment, never count writes on raw state or restored absent bodies.
- **24:** Listener/HTTP/daemon fixtures currently drop their accepting runtime before BASIC assembly; the same genuine managed Fjall source plus normal initialization is the right repair. Maturity-1 historical/same-block spends retain explicit parameters. History raw seed/missing-input controls stay separate from positive worker assembly and retain backlog prefixes, deadlines, pending/error/join and clean-marker assertions. Codec-only automatic-prune seed remains unearned even after mining. DenseHistory's cache-positive rebuild is explicitly genuine same-Fjall validation/flush/assembly, with minimal full height encoding, selected parameters/subsidy, measured pair bytes, physical/undo/prune assertions and cache eviction 0/1/713 versus retained 714; a separate exact raw codec refusal remains. The black-box setup has B1 above.

## Constructor and finalization prerequisites

Plan 15/16 are being revised independently and were not re-reviewed here. This check provisionally assumes the promised public fixture interface exists before these consumers execute. Before any consumer edit, confirm the actual constructor consumes the same genuine Fjall managed source, independently revalidates the complete retained prefix with the existing Memory engine (including aliased raw-coins poisoning refusal), compares captured complete source/physical identity after normal initialization, preserves full parameters including explicit maturity 1, rejects raw/mechanical/preview/canonical/bad-physical/parameter inputs before configuration, and creates BASIC only through normal initializer/recovered-manager lineage after real genuine flush with `header_entries=[]`. An empty core alone cannot establish BASIC. Failure of any prerequisite requires a bounded dependency replan rather than a local escape in 21–24.

Focused results do not establish full-suite GREEN while the other 159 diagnostic failures remain outside this check. Final Plan 14 default native verification must occur **after 24 and all repair plans**, including the normal workspace tests, coverage, Bazel and benchmark contract. Independent complete review/security/formal checks and the normal pre-commit hook remain mandatory later. Executors must not stage/commit/push, edit shared trackers/state/other plans, or enter Phase 166.

## Checked plan identity

| Plan | Bytes | SHA-256 |
| --- | --- | --- |
| 165-21 | 30396 | `b51f911cc69e9def02f8d2acf6bc37de131d4c07854835a99b09a3843cb4a6e3` |
| 165-22 | 27467 | `d543c0631023acf4b17523ce1fbeb02cb34e9edbab9189935b64992dd5f6cbda` |
| 165-23 | 28903 | `9dbb73dc40ea0aa4986071a8198db7f69266561b8c4860194c8735fcdf155463` |
| 165-24 | 32686 | `eb4b92ea2d9173ae440c18a8eef157bd3e223b43b12e868dc11511c75be64311` |

Static/source-only evidence: no application execution, Cargo/Bazel build/test, Bun test/live checker, source edit, Git command, receipt renewal or qualification was performed. Only this dedicated report was written. Revision-gate outcome: return Plan 24 Task 2 for the bounded B1 repair and recheck its changed action/acceptance contract before implementation.


## Revision recheck | 2026-10-17T03:28:32Z

### PLAN CHECK PASSED

**Current issues: zero blockers, zero warnings. Initial B1: resolved in the plan contract.** Plans 21–23 retain their reviewed semantics; parent changes since the initial receipt only repaired STRIDE closing-tag/padding formatting. Plan 24 remains thirteen files and three tasks, with the original wave-23 dependency on 23. The current amendment adds no new implementation path outside the existing owner inventory.

The amended Task 2 no longer calls the whole restarted phase read-only. It explicitly identifies the actual maturity-loop mutation, retains true forceflush/all-handle closure, and requires the following before the first maturity extension:

1. Load every actually persisted active-prefix body from genesis to the captured endpoint; fully validate in a new genuine Memory core using the original full parameters and flags. Check the derived height/hash/parent/work against the intended prefix. Missing, corrupt or pruned required input cannot be skipped or restored.
2. Build the untrusted Fjall target only from the newly genuine snapshot's positions, undo and counts plus the actual physical coins view. Consume the actual Memory owner through public `rebind_into`; exact complete coins/B/heads and derived metadata equality precede Live origin. Public `from_parent`, `rebind_into`, `FjallChainstateStore::from_store` and `ManagedChainstate::from_chainstate` signatures were rechecked in source. No raw score or metadata row is a source of earned authority.
3. Supply that same genuine Fjall source to Plan 15. Its independent replay checks actual physical metadata/undo/body/coins and guards aliases, retaining physical None counts without writes. Plan 15 rejects all four canonical roots before constructor physical reads/initialization and rejects invalid source/parameters. Its later immutable captured-identity check follows initialization and graph setup. A configured replacement owner is not available when these checks fail.
4. Execute every original maturity connection AFTER the real reopen and handoff, with endpoint-derived heights after the explicit root/child setup and the original maturity value. Preserve actual original-coinbase transaction admission, shared context, seven achieved serving writes, mixed response ordering, authenticated HTTP keys/evidence, wallet/physical observations and cleanup/join. No retained database handle or moved/omitted maturity trigger substitutes for the actual restart.

**The linkage prerequisite is now concrete in Plan 15.** Read the finalized interface, Task 1 graph and final-check actions, Task 2 controls, and guard ordering. `initialize_live_fixture_source_graph` runs on the direct unwrapped Fixture network using the independently captured validated blocks. It admits selected active headers once under full explicit parameters, checks the unique root body for first-ancestor linkage, and validates each historical nonroot through `retain_cached_candidate`. Historical nonroot order remains sticky unknown; from-recovered assembly leaves accepted-tip body preference absent. The first post-reopen maturity child therefore has accepted parent proof and checked body ancestry, while no old tie preference is reconstructed. Historical sources support real extensions/strictly greater work; unknown equal-work alternatives refuse. A genesis-only source leaves genuinely new A/B events able to earn real order through ordinary ingress. No second fixture-header binding or P2P null-parent root is permitted.

The source-graph calls run without a held publication guard; the final captured physical/body/validation/source check follows graph setup under the guard, and finish/readiness/BASIC work follows guard release. Normal BASIC initialization and recovered-manager proof ownership remain required. These concrete steps close the dependency identified by the amendment without adding a public cache/order API or ordinary reopen upgrade. Plan 15's external/private controls require actual populated extension, unknown-equal refusal and new A/B behavior; loaded-source controls protect the once-only binding, historical cached admission and final ordering. Their future execution is a condition of consumer readiness, not a result produced by this recheck.

**B1 regression controls:** the same existing `phase127_composition.rs` owner must contain or explicitly attribute two actual controls, `phase127_fixture_prefix_requalification_refuses_missing_required_body` and `phase127_fixture_prefix_requalification_refuses_physical_identity_mismatch`. Each uses a genuinely persisted isolated prefix, applies the actual fault, calls the same helper, asserts its exact typed refusal, unchanged post-fault physical manifest and absence of a configured replacement owner/work grant. The new literal timing-wrapped command uses `--test black_box_parity phase127_fixture_prefix_requalification`; it must discover both controls. The following unfiltered `--test black_box_parity` command must execute the original successful maturity/transaction/socket/RPC path. The focused controls are a subset of that broader run, not additional unique cases in a combined count. Static plan-structure validation of amended 24 passes with three complete tasks and no errors/warnings. No tests were run here.

The amendment keeps every prior group-H assertion and strict raw/missing-input boundary. All four plans still require reviewed current guard renewal, actual enabled ownership/inventory and mutation controls, source caps/breadcrumbs, serial timed Cargo, exact Bun matrices and no task commits. Current file-count/wave summaries remain 13/9/11/13 files and waves 20/21/22/23. Any helper/cap seam outside declared paths still requires a bounded replan before editing; none is approved implicitly by this pass.

```yaml
issues: []
resolved_issues:
  - id: B1
    plan: "165-24"
    task: 2
    dimension: key_links_planned
    initial_severity: blocker
    status: resolved_in_plan
    resolution: "Explicit actual post-reopen complete-prefix Memory validation, genuine-snapshot-only Fjall target, consuming rebind, Plan15 same-source physical/graph validation, then original maturity suffix; two exact refusal controls added."
```

### Current checked identity

These current hashes supersede only the active plan identity receipt; the initial plan hashes above remain unchanged historical evidence.

| Plan | Bytes | SHA-256 |
| --- | --- | --- |
| 165-21 | 30368 | `cfd86b16a4717fbc4300aa55f19db41d65d0fd4636378a7855c71989957c40d8` |
| 165-22 | 27439 | `6421e30c3cfeb6c4f4c185949414129deca12c69f4366d0c141493ee9457e087` |
| 165-23 | 28875 | `6ab513e11cec8e793c52b19f347c11925cc95f25b7c916f11745653247b91269` |
| 165-24 | 39924 | `0be46b9a6488fac2176c206a40e945e47d35d8ac8d14187de8c7059beefc4125` |

Dependency contract read: 165-15 SHA-256 `de54b38f05b8dd0a4cca02d100ff36c884ea823710017a7c19d6b07778de63a3`. Parent-reported independently checked 165-16 identity: `23457ba216ffca1733c82f7e8b6b91ca4c153ffb235a018a9603a5025ab425d2`; corresponding ledger identity: `978fbfdf92d53c2619cc999872f3ba4662483f3d84b317f52032775a16a4881d`. This checker verified the current 15/16 file hashes and read 15's actual contract; it did not repeat the separately owned full 15/16/ledger review.

**Conditional readiness:** Plan 15's actual implementation and required nonzero semantic/current-guard GREEN receipts, followed by the preceding owned repair plans' GREEN, must precede 21–24 execution. Final Plan 14 default native workspace/coverage/Bazel/benchmark verification follows 24 and all repairs, with independent full review/security/formal verification and normal hooks later. Phase 166 remains closed.

Only this report was updated. No source/other-plan/tracker/state edit, build/test/live checker, Git operation, subagent or qualification was performed by this recheck. The original 249-failure diagnostic and 90-case attribution remain discovery evidence; current conditional plan approval does not claim any of those tests now pass.
