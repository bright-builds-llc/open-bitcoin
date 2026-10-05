---
phase: 157-safe-activation-and-scheduled-index-catch-up
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T22:29:49Z
status: fixed_pending_rereview
---

# Phase 128 durable-save guard correction

The live Phase 128 check exited 1 with only `P128 durable trigger: accepted best-tip blocks must queue only after durable save`. Its unchanged real-corpus positive also failed: 19 passing mutations, 1 failing positive and 20 assertions on Bun 1.3.9. The old predicate required `self.store.save_block(block, self.config.persist_mode)?;` anywhere in the file. Phase 157 now uses an explicit save-error branch in `DurableSyncRuntime::record_block_disposition`: BASIC required-body failure notification and `return Err(error.into());` occur before block-hash evidence, accepted progress and durable-tip queueing.

## Authorized scope and plan

- [x] Capture the actual live-source positive failure and inspect the save-success/failure ordering.
- [x] Retarget only the Phase 128 durable-trigger predicate to the actual runtime method and error branch; preserve the other guard predicates.
- [x] Add independent save-removal, ignored-failure, ordering and non-production decoy mutations against the actual source corpus.
- [x] Run both affected live checks, complete Phase 128/129 mutation suites, scoped parse/style checks and ordinary snapshot diff review.
- [ ] Root independent re-review and complete native verification before finalization.

Changed files are only `scripts/check-phase128-production-compact-announcement-transport.ts`, its `.test.ts`, and this evidence report. The frozen Phase 157 `rust-evidence.ts` helper is a read-only import. Its dependency graph goes to the Phase 156 helper and neither helper imports Phase 128, so there is no import cycle. No Rust/runtime, schema, dependencies, target-source inventory, verifier wiring, Git/staging/commits, or shared lifecycle state was changed by this worker. Phase 129 required no edit.

The repo instructions, Bright Builds sidecar, standards overrides and local TypeScript, testing, verification and code-shape pages informed the scoped work. Both active lesson files were loaded completely: 7,188 combined bytes and 2,397 estimated tokens. No archived lesson input was loaded. Repository synchronization, lesson audits, whole-phase verification and finalization remain root-owned under the delegated scope.

## Corrected invariant

The guard selects one ordinary `record_block_disposition` declaration and requires its enclosing implementation to be `DurableSyncRuntime`. Comment/literal masking and ordinary/test-only declaration selection reuse the existing helper. The requested connected best-chain arm must perform the save unconditionally after the absent-block early return. The balanced save-error branch must terminate with the error return and must contain none of the dependent hash/progress/tip/queue calls. Immediately after that branch, the successful route must record the local block hash, record accepted progress, check the current best tip and queue its block, in that order.

The existing complete-corpus positive still copies the actual repository source, including the explicit error branch; it does not synthesize a former `?` spelling. Thirteen added independent negatives cover removed save, ignored save failure, accepted progress/hash/queue before error return, queue before save, conditional save, comment and raw-literal decoys, test-only function/module/implementation and an unrelated implementation. Each requires exactly the durable-trigger diagnostic. All nineteen pre-existing peer negotiation, transport bounds, write receipt, achieved evidence, observability, no-claim and verifier mutations remain unchanged.

## Verification evidence

All commands used the pinned Bun path `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64` at the front of `PATH`.

| Check                                                                                                                                                                      | Result                                                                                     |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `bun scripts/check-phase128-production-compact-announcement-transport.ts`                                                                                                  | Exit 0; live Phase 128 validated                                                           |
| `bun scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts`                                                                                        | Exit 0; unchanged Phase 129 delegate validated                                             |
| `bun test ./scripts/check-phase128-production-compact-announcement-transport.test.ts ./scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts` | 45 passed, 0 failed, 45 assertions, 2 files; final run 2.10s                               |
| Scoped Bun `Transpiler` parsing and whitespace/final-newline/628-line checks                                                                                               | Passed both owned source files; 627 guard lines and 300 test lines                         |
| Ordinary `diff -u` against pre-edit snapshots                                                                                                                              | Reviewed; only the targeted predicate/import/helper and independent mutation block changed |

The Phase 128 suite contains 33 tests (one actual-corpus positive, thirteen new independent negatives and nineteen existing mutations). Phase 129 contains 12 tests (one positive and eleven existing mutations, including its composed Phase 128 seam). There is no TS project configuration or dedicated TS formatter/linter to invoke for this repository; scoped parse/style checks follow the approved delegation. Cargo/Bazel and the complete native verifier were not run by this worker. Root must still refresh independent review and run the native contract from its beginning.

## Simplification and limits

The simplification pass retains one small source-specific predicate and reuses the frozen ordinary-source helper instead of adding a parser, dependency, source path or generic framework. This is a structural source contract, not a runtime persistence proof. The exact requested-arm sequence intentionally fails closed when the actual route changes again. The work introduces no known stubs or new production threat surface. All content staging, commits, push and whole-phase completion claims remain deferred to root.

The report passed `mdformat --check --extensions gfm --extensions tables --extensions frontmatter` after its initial check identified table alignment and a report-only formatting pass corrected it. Markdown whitespace, final newline and exactly two standalone frontmatter delimiters passed. The source snapshots are frozen for independent root review:

- Guard SHA-256: `e901cbbed2bc6c025152ca7905c037168fc4c5ba0bb8b813ccb9c974cc543911`.
- Test SHA-256: `66b5d281c73bfa62f336baccff1ca19b063b5b9d88aa18c1f4b58e3bad3bd84e`.
