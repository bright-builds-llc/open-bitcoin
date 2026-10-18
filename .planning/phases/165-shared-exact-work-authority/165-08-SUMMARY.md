---
phase: 165-shared-exact-work-authority
plan: "08"
subsystem: chainstate-network-authority
tags: [proof-work, branch-preference, sealed-capabilities, eligibility, basic-index]
requires:
  - phase: 165-07
    provides: Genuine live work lineage, private owner/generation stages and complete fixture transfer
provides:
  - Sealed work/body-preference grants checked by the lower managed effect owner
  - Immutable accepted body-order history and independent mutable eligibility freshness
  - Exact replacement failure attribution with preserved sync error classification
  - Pre-store refusal of stale invalidated BASIC targets
affects: [165-09, 165-12, 165-13, 165-14]
tech-stack:
  added: []
  patterns:
    - Non-Clone source and preference grants with actual owner/generation/tip binding
    - Immutable Arc linkage seals plus sole-owner Weak eligibility epochs
key-files:
  created:
    - packages/open-bitcoin-node/src/network/work_authority.rs
    - packages/open-bitcoin-node/src/chainstate/work_authority.rs
    - packages/open-bitcoin-node/src/network/tests/phase165_work_authority.rs
    - packages/open-bitcoin-node/src/network/tests/phase165_work_authority/freshness.rs
    - packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_authority.rs
    - scripts/check-phase164-canonical-identity/test-modules.ts
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
    - packages/open-bitcoin-node/src/sync/types/error.rs
    - packages/open-bitcoin-chainstate/src/engine/work_authority.rs
    - packages/open-bitcoin-chainstate/src/engine/stage.rs
    - packages/open-bitcoin-chainstate/src/error.rs
    - packages/open-bitcoin-network/src/header_store/eligibility.rs
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Source authenticity alone never grants managed branch preference."
  - "Historical body order uses immutable event identity; current eligibility uses a separate expiring epoch."
  - "Only the exact checked intake may carry a grant into its own new registry; unrelated stale facts cannot refresh it."
  - "Raw matching BASIC recovery remains separate from earned coins/work authority."
requirements-completed: ["CWOR-03", "CWOR-02"]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-15T05:15:51Z
duration: 68min
completed: 2026-10-15
---

# Phase 165 Plan 08: Managed Work Preference and Effect Authorization Summary

**Actual managed owners require a sealed, current work/body-preference grant before reorg preparation, BASIC, preview and commit; direct connects derive work from the actual accepted parent before intake.**

## Performance and Scope

- Tasks: 2/2.
- Approved inventory: 46 source/test/checker/breadcrumb paths, including ten new Rust files and one new static TypeScript inventory module; all exist and all owned Rust/TypeScript paths are at most 628 lines.
- First recorded command: 2026-10-15T04:08:05.090Z. The first behavior RED began at 04:08:48.393Z. Context loading was not separately timed.
- Summary prepared at 05:15:51Z, approximately 68 minutes from the first recorded command.
- Local AGENTS.md, Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/Rust/TypeScript standards, phase context/research and predecessor summaries informed execution. Both active lesson files were read completely: 7,188 bytes and 2,397 conservative estimated tokens. Parent orchestration owned sync/bootstrap and shared planning state.

## Accomplishments and Actual APIs

- `bind_live_work_source(params)` and `check_live_work_source(binding)` inspect the actual private core origin, complete selected parameters, held owner/generation allocations and full source tip without staging. Raw, mechanical, foreign and stale sources refuse. `check_reorg_work_source(binding, stage)` additionally binds the actual genuine stage and permits only its own sealed preview continuation. The new source binding is private-field and non-Clone; compile-fail documentation checks both properties.
- Public lower connect preparation calls `authorize_direct_connect` before staging or BASIC: genuine source/parameters, exact parent, selected target/hash admission and parent-plus-proof work. Supplied work does not grant strength. Local network connects run this before candidate retention; stored receipts check the genuine selected source before intake and retain legitimate non-extending candidates separately. The obsolete time-default preparation convenience was removed; all five former callers use the normal explicit-time path.
- `ManagedPeerNetwork::authorize_reorg` validates the actual retained disconnect bytes/undo, retained accepted ancestor and complete replacement body suffix. It derives every cumulative value from the accepted ancestor and actual target proof, then compares work and genuinely earned body order. MAX scores, height/hash ordering and a valid stage alone cannot grant preference. Unknown accepted arrival history refuses equal preference; stronger candidates can qualify through validated history.
- `ReorgAuthorization` is private-field, non-Clone and consumed by `prepare_authorized_reorg`. The lower owner checks source, candidate bytes, parameters, derived staged positions and eligibility before preparation/history/BASIC. It rechecks before preview and before commit. Public lower `reorg` and unqualified `prepare_reorg` refuse before all preparation/effects, including genuine weaker, later-equal and unqualified stronger core stages. Connect grants also recheck source and freshness before absorption.
- Immutable body-linkage seals are allocated only at actual sequence assignment. Accepted managed order captures the actual seal. A padded new registry or a pre-active mutable clone cannot reproduce that event by copying numbers. A post-active whole-network memory clone preserves genuine past order while receiving a fresh core owner/generation; old grants/stages still refuse across owners.
- Current eligibility uses a separate Weak epoch. The mutable registry owns its sole strong reference; independent registry clones get fresh epochs while sharing immutable past linkage seals. Actual body admission, availability change and failure change rotate it; reads, missing-hash operations and unchanged duplicates do not. Grants expire on mutation, registry replacement or owner drop, even when core generation stays unchanged. Checks cover prepare, preview, commit and commit after preview.
- The ordinary network path moves the exact checked HeaderStore into the owner once, then binds only that intake's current epoch. It checks the old grant before taking/publishing anything. Repeated intake refuses; a revoked grant cannot refresh itself. The resulting single grant can continue through its own preview and one acceptance.
- Pure reorg errors carry `ReplacementBlock { block_hash, source }` for the actual body being checked. Networking marks only a consensus-invalid body's identity; ancestry excludes its descendants while preserving a valid prefix and unrelated valid sibling. Missing data, source/owner/generation, work preference and storage refusals do not become invalid-body flags. Sync conversion unwraps attribution through its original typed conversion, preserving the underlying peer-data classification and message.
- A scheduled BASIC turn now compares an invalidated saved target with the actual core active identity under the existing runtime authority. A mismatch returns the existing paused outcome before proof acquisition or any work. Matching identities retain ordinary read-only recovery, including raw reopened work that remains untrusted. Genuine accepted reorg, publication-failure and resume positives remain exercised.

## Fixture Evidence Boundary

The real ReorgFixture now owns a small test wrapper around an actual ManagedPeerNetwork. It checks/retains the actual early fork bodies before later active connections and uses normal network authorization plus the normal lower authorized path. It never constructs a test preference grant or assigns a sequence. A genuinely weaker branch is not disguised with scores. Full-disconnect tests use explicit managed disconnect operations and assert actual BASIC invalidation, empty active state, retained identities and physical rows/checkpoints.

The lagging-common restart fixture preserves the actual missing common body/undo and immutable-row reuse checks. After a real flush it explicitly enriches missing fixture counts, closes/reopens, validates actual retained active bodies in a new pure owner and uses the existing strict full-state fixture transfer. Arrival history remains unknown after that reopen, so the replacement is genuinely stronger. The sync test adapter likewise uses actual complete-prefix validation and a stronger suffix. No missing physical input is silently restored. Production reopen acceptance, schema behavior and canonical authority are unchanged.

Refusal evidence includes a genuine Fjall lower owner with BASIC enabled: exact active/undo/count snapshots, complete effective coins/B/heads, cache entries/flags/counts, actual core source generation, BASIC progress/target/checkpoint/filter rows, durable undo and each original validation classification remain unchanged. A separate store spy proves zero queries/writes; preparation counters prove zero stage/history/BASIC/preview/commit calls. Network cases preserve mempool, peer/reject/projection state, queues and effect ledgers. Historical fixture genesis classifications are captured as actually observed, including UnknownLegacy, rather than relabeled ScriptsValid.

## Task Commits

All task, RED/GREEN and metadata commits remain deferred to the authorized parent strict wrapper after whole-phase native verification, independent review/security/goal checks and normal hooks. This executor staged, committed and pushed nothing, and did not edit shared STATE, ROADMAP, REQUIREMENTS, config, hooks or todo files. No commit hashes or qualification receipts are invented.

## Executed Evidence

All Cargo commands ran sequentially through `bun run scripts/command-timings.ts run --key <key> -- ...`, using Rust 1.94.1. Every Bun/Cargo command used command-local Bun 1.3.9:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

| Check                                  | Actual final result                                                                                                                                                                                                                                    |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `phase165-final-pure-coverage`         | Clean coverage run passed: chainstate370 unit+3 parity; network638 unit+2 parity+2 properties. Zero failed/ignored. Network unit execution17.90s.                                                                                                      |
| Pure changed-owner coverage            | Core work_authority267/267, stage263/263, error162/162; network eligibility318/318 and lib18/18:100% lines.                                                                                                                                            |
| `phase165-final-node-coverage`         | 122/122 affected Node tests passed, zero failed/ignored,33.19s execution. Both new Node authority modules are100%: lower109/109 and network264/264 lines. No exclusions added.                                                                         |
| Dedicated authority/freshness controls | 39/39 Node and3/3 pure epoch controls passed in `phase165-epoch-controls-final`; includes true whole-network memory clone, current-time rejection and physical Fjall preservation.                                                                     |
| `phase165-strict-clippy-final`         | Chainstate, network and Node all-targets/all-features with `-D warnings` passed,15.33s.                                                                                                                                                                |
| `phase165-node-all-targets-final`      | Explicit Node all-targets/all-features cargo check passed,15.50s.                                                                                                                                                                                      |
| `phase165-authority-doctests-final`    | 35/35 passed:23 core (2 regular+21 compile-fail),12 Node compile-fail, zero network examples.                                                                                                                                                          |
| `phase165-fmt-check-final`             | Workspace Cargo formatting check passed after final source changes.                                                                                                                                                                                    |
| Full current identity guard            | 576 passed, zero failed,732 expectations,183.63s. Run once in full after final source stabilization. Separate focused current mutations44/44 passed.                                                                                                   |
| Current boundary command               | Passed with121 fixed Rust inputs and377 reviewed source edges. Six added inputs,35 new reviewed edges and32 added mutations; historical snapshots, adapters, manifests and receipts preserved.                                                         |
| Static inventory extraction            | 26 TEST_MODULES rows moved unchanged in content/order/types; serialized data SHA256 `d47862d042e6376cce6f42e7d737ba5929228ff0a466800fe6ccff29e8d75614`.                                                                                                |
| Breadcrumbs / caps / diff              | Breadcrumbs passed for1,187 Rust files. Managed checks passed with1,521 tracked inputs and no findings/exceptions; all46 declared paths were independently checked for existence and owned Rust/TS caps. `git diff --check` passes and index is empty. |

The saved pure report's literal aggregate is **13,458/13,488 lines, 99.77758%**. It is not whole-crate 100%. Its 30-line deficit is retained below; it includes existing generic/library and test-specific callback/error-closure instantiations, not a blanket claim of generated code. Eight paths are byte-unchanged from HEAD. The other three are earlier-plan source whose reviewed function digests were preserved through Plan 08 and pass the current guard; none of these eleven paths was edited by this plan.

| Existing report path (under packages/)                       | Deficit | Attribution/evidence                                                                                                      |
| ------------------------------------------------------------ | ------- | ------------------------------------------------------------------------------------------------------------------------- |
| open-bitcoin-chainstate/src/engine/apply.rs                  | 1       | Existing `txid_serialization_error<CodecError>` instantiation; unchanged from HEAD.                                       |
| open-bitcoin-chainstate/src/engine/overlay_apply.rs          | 1       | Existing serialization-error closure; earlier Plan07 `apply_connect_on_overlay` reviewed digest unchanged.                |
| open-bitcoin-network/src/compact_reconstruction.rs           | 1       | Existing normal-library instantiation coverage; unchanged from HEAD.                                                      |
| open-bitcoin-network/src/header_store.rs                     | 3       | Existing arithmetic/target error closures in `admit_header`; earlier header-authority reviewed digest unchanged.          |
| open-bitcoin-network/src/inbound/permissions.rs              | 3       | Existing typed address-parser error/callback instantiations; unchanged from HEAD.                                         |
| open-bitcoin-network/src/message/filter.rs                   | 2       | Existing count-conversion/checked-length error closures; unchanged from HEAD.                                             |
| open-bitcoin-network/src/peer/message_dispatch.rs            | 7       | Existing per-test header-policy callback specializations; earlier `handle_headers_with_policy` reviewed digest unchanged. |
| open-bitcoin-network/src/peer/policy_state.rs                | 1       | Existing absent-record diversity fallback callback; unchanged from HEAD.                                                  |
| open-bitcoin-network/src/peer/transaction_relay/orphanage.rs | 4       | Existing announcer/reconsideration/cleanup/oracle callback instantiations; unchanged from HEAD.                           |
| open-bitcoin-network/src/peer/transaction_relay/retry.rs     | 6       | Existing maintenance-selection generic instantiation; unchanged from HEAD.                                                |
| open-bitcoin-network/src/peer/transaction_relay/scheduler.rs | 1       | Existing pending-relay predicate instantiation; unchanged from HEAD.                                                      |

Plan 14 retains phase-wide native/coverage closure ownership. These aggregate deficits were inspected without adding exclusions or changing unrelated code.

## Recorded RED and Diagnostic History

The following actual nonzero command outcomes are retained. Unless stated otherwise their exit status was 101; final results above supersede the diagnostic state without erasing it.

| Command key                                       | Actual observation and resolution                                                                                                                                            |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `phase165-managed-work-owner-red`                 | Compiler-only diagnostic: nonexistent ProofWork Default/MAX fixture APIs. Corrected fixture construction.                                                                    |
| `phase165-managed-work-owner-red-discovered`      | Compiler-only diagnostic: nonexistent from_u128 helper. Corrected to actual full-width `from_le_bytes([255;32])`.                                                            |
| `phase165-managed-work-owner-red-behavior`        | Three discovered tests failed: direct lower weaker/equal acceptance, weaker MAX network acceptance and later equal-body activation. First implementation made all3 GREEN.    |
| `phase165-managed-order-red`                      | One discovered registry-reseed case failed; accepted order needed retained evidence.                                                                                         |
| `phase165-source-core-checks`                     | 39/41 passed: a test used flush-commit after preview instead of exclusive absorption, and expected the wrong rejection spelling. Corrected the test operations/expectations. |
| `phase165-padded-order-red`                       | One discovered padded same-sequence reseed failed. Immutable event identity, including pre-active clone controls, resolved it.                                               |
| `phase165-managed-owner-compile-initial`          | Explicit fixture child-module path and obsolete convenience warning exposed; corrected the registration and later removed the unused convenience.                            |
| `phase165-managed-owner-compile`                  | Wrapper ownership transfer needed explicit consuming adapters and preservation of the real network handle. Corrected without duplicate Fjall writers.                        |
| `phase165-managed-owner-affected`                 | 66/71 passed; full-disconnect assumptions, raw reopened work and two sync fixture assumptions required the explicit migrations above.                                        |
| `phase165-managed-owner-affected-adapted`         | Compiler diagnostic: private handle read and a missed fifth convenience caller. Used public observations and migrated the actual caller.                                     |
| `phase165-authority-core-and-network`             | Core40/41 passed; one rejection spelling remained incorrect.                                                                                                                 |
| `phase165-replacement-error-diagnostic`           | One case printed actual `bad-txnmrklroot (hashMerkleRoot mismatch)`; the structured formatting assertion now uses that actual cause.                                         |
| `phase165-authority-affected-green`               | Core41 and network30 passed; Node138/139 exposed an incorrect expected error for the explicit disconnect turn.                                                               |
| `phase165-authority-node-affected-final-precheck` | Three compiler errors from a test edit applied to the wrong turn expression. Corrected the exact affected test blocks.                                                       |
| `phase165-owner-node-regression-expanded`         | 100/104 passed: three legacy empty/later-equal reorg fixtures and total-work assumption failed. Replaced with genuine stronger candidates and inspected the BASIC behavior.  |
| `phase165-owner-node-regression-final`            | 106/107 passed; scheduled BASIC work actually unpaused an invalidated stale identity. The bounded pre-store actual-identity guard resolved the root cause.                   |
| `phase165-local-parent-red`                       | One discovered case showed local wrong-parent rejection occurring after candidate retention. Actual parent/proof preflight now precedes intake.                              |
| `phase165-authority-core-network-coverage`        | 369/370 core tests passed; a legacy top-level MissingCoin assertion needed exact failed-hash+nested-cause matching. Preserved before/after state checks.                     |
| `phase165-replacement-mapping-red`                | One discovered actual invalid-body case lost InvalidData classification/message through the wrapper. Recursive existing conversion resolved it.                              |
| `phase165-authority-node-additional-green`        | 30/31 passed; historical fixture genesis was actually UnknownLegacy. The test now preserves each captured classification, without rewriting ledger evidence.                 |
| `phase165-owner-source-clippy`                    | One cloned-reference slice warning; replaced with `std::slice::from_ref`.                                                                                                    |
| `phase165-eligibility-epoch-red`                  | One discovered case accepted changed eligibility while core generation stayed unchanged. Separate registry freshness now refuses at all required boundaries.                 |
| `phase165-authority-final-clippy`                 | Two collapsible-if warnings in epoch mutators; used equivalent let chains without suppressions.                                                                              |

The initial current-edge check also refused nine changed reviewed functions until their actual changes were reviewed and renewed. Managed cap diagnostics found network.rs629 lines and contracts.ts652 lines; the module registration was kept within628 and the static ownership table was extracted, without exceptions or historical edits. Coverage-report diagnostics after switching package/profile sets are not substituted for the saved final clean coverage JSON results above. The final artifact probe initially assumed a fixed closing-frontmatter line number and exited 1; replacing that probe with the actual two-delimiter structural check passed without changing the frontmatter.

## Decisions, Simplification and Deviations

1. **[Rule2 - Correctness; parent formalized] Lower capability/source binding.** The actual lower commit owner and small authority modules were added to scope before edits. Reused the existing core held identities and ProofWork rather than creating parallel coins-generation or work arithmetic authorities.
1. **[Rule2 - Correctness; parent formalized] History and freshness separation.** Actual reseed and padded-reseed RED cases required immutable linkage identities. Registry-only revocation RED required a distinct sole-owner Weak epoch. Independent mutable clones do not extend an old epoch's lifetime; no persistence or global owner counter was added.
1. **[Rule2 - Correctness; parent formalized] BASIC actual-identity guard and error attribution compatibility.** Kept the scheduled regression and fixed its smallest pre-store owner check. Preserved matching raw read-only recovery and underlying sync error semantics.
1. **[Rule3 - Blocking; parent approved] Genuine fixture and explicit-time migrations.** Existing synthetic/empty/equal branch fixtures moved through real checked body order or genuinely greater work. Restart transfer revalidates actual bodies and retains unknown arrival order. No managed Fixture/mechanical guard exception was introduced.
1. **[Rule3 - Blocking; parent approved] Focused source/test/data modules.** Registered small child modules and split the unchanged26-row ownership table, keeping all owned source paths within628 lines. No tests were ignored, assertions skipped, dependencies added or coverage exclusions introduced.

The simplification pass retained one arithmetic type, one actual core source identity, one immutable body-history event and a separate expiring current-facts observation. Capabilities move through the existing lifecycle and consume once. No new endpoint, schema, service, canonical-bootstrap path or unrelated lifecycle redesign was added. Stub/threat review found no unimplemented goal or undeclared production trust surface; intentional unavailable imported/restart arrival history remains a bounded refusal.

## Handoff and Retained Limits

- Plan09 must use the actual network authorized path, revalidate cached pre-header bytes and consider all eligible retained side candidates. Lower public reorg is deliberately unqualified and refuses. Header/work numbers alone remain advisory.
- New current eligibility mutation invalidates outstanding grants. Obtain a fresh decision; never refresh after unrelated changes. Only `retain_authorized_reorg` can carry its exact checked intake once, followed by its own source-bound preview/commit.
- `ReplacementBlock` identifies only the actual failed replacement. Keep its source classification separate from preference, missing-data and storage refusals; valid prefixes/siblings remain eligible.
- Phase166 owns canonical bootstrap, durable exact-work/all-handle recovery and genesis BASIC; Phase167 contextual completeness; Phase168 actual pinned-daemon qualification. Four ordinary support/refusal classes and default-off networking remain unchanged. Parent Plan13 owns consolidated contributor-facing documentation updates.
- Whole-workspace native/Bazel acceptance, review/security/goal verification and normal-hook commits/push remain with Plan14 and the parent. This bounded plan supplies no canonical or ordinary qualification receipt.

## Self-Check: PASSED

Both tasks meet their bounded plan criteria. All 46 declared paths and the summary exist; the ten Rust files and one TypeScript file created by this plan are present. All owned Rust/TypeScript paths meet the 628-line cap. Final breadcrumb verification covers 1,187 Rust files; managed checks scan 1,521 tracked inputs with zero findings or exceptions. Stub review found no unfinished authority implementation. The saved final pure report independently confirms 13,458/13,488 lines (99.77758%), with the exact eleven-path residual inventory above. Both final coverage artifacts remain present. The final diff check passes, the index is empty, and all verification sessions have completed. Summary formatting passes with both required Markdown extensions. Commits and shared state updates remain intentionally deferred to the parent wrapper.
