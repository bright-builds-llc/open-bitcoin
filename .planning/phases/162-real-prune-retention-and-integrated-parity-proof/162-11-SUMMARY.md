---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "11"
subsystem: testing
tags: [bun, source-contracts, mutation-testing, retained-filters, native-verification]
requires:
  - phase: 162-10
    provides: Frozen shared lazy claim-state validator and tested historical transitions
  - phase: 162-09
    provides: Actual seven outbound and eleven inbound CF writer controls
  - phase: 162-15
    provides: Preserved lazy checkpoint, lineage and no-grant disconnect contracts
provides:
  - Fixed current-source caller, assertion, protocol and argument guards with isolated mutations
  - Genuine Pending bootstrap without future proof or formal artifacts
  - Exact Phase162 oracle/checker and shared Rust formatting routes in the native verifier
affects: [162-12, independent-review, native-verification, formal-activation]
tech-stack:
  added: []
  patterns: [One-version source cache, exact executable span contracts, shared claim-state authority]
key-files:
  created:
    - scripts/check-phase162-integrated-filter-proof.ts
    - scripts/check-phase162-integrated-filter-proof/contracts.ts
    - scripts/check-phase162-integrated-filter-proof.test.ts
    - .planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-11-SUMMARY.md
  modified:
    - scripts/verify.sh
key-decisions:
  - Keep checkPhase162ClaimState and its optional/exclusion/view helpers as the sole claim authority.
  - Preserve exact original literal-bearing assertions and occurrence counts for repeated caller edges.
  - Permit only the independently checked equal-offset view of the original shared Unix test registration.
  - Parse only bounded actual TypeScript declarations and arguments, with no Rust-as-TypeScript conversion.
  - Use rust-toolchain.toml through plain root rustfmt rather than duplicate its version.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-12T00:11:07Z
duration: approximately 48 min
completed: 2026-10-12
---

# Phase 162 Plan 11: Integrated Source Guard and Native Wiring Summary

**The native verifier now routes independent raw-oracle and current-source mutation controls while permitting genuine Pending bootstrap without future proof or formal outputs.**

## Performance and Ownership

- Tasks: 2/2. Exactly the four assigned source paths and this summary. No production, dependency, visibility, shared fixture, registry, requirement, STATE, ROADMAP, todo or generated LOC edits.
- Parent execution-position timestamp was 23:23:00Z. No separate executor-start clock was captured. The first test file was created 23:25:13.148390Z; first full matrix started 23:31:29.906682Z. Final full matrix ran 00:02:06.532279Z–00:06:05.347734Z; final scoped checks completed 00:07:48Z. Duration is approximate from the parent handoff.
- Material guidance: actual AGENTS and Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/language standards, execute-plan workflow and summary template, current STATE/CONTEXT/RESEARCH/UI contract, final Plan11 and its independently checked amendment, and final09/10/14/15 summaries.
- Both active lessons were fully read: 7,188 bytes, 2,397 conservative estimated tokens. Existing audit baseline has no new trigger; no project skills were found.
- Parent supplied synchronized main a77edab276a9daa086055aa61e6143ce86bc1218 and materialized Knots a9aee730466ac67d35a3c03ee24676be5e045878. No Git mutation was performed.
- Every Bun invocation used PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH and Bun 1.3.9. Active Rust is 1.94.1 from rust-toolchain.toml. No Cargo/Bazel job or dependency installation was necessary; Cargo is idle.

## Task 1: Fixed Actual Callers and Claim Consistency

The checker exports checkPhase162IntegratedFilterProof(maybeSources?: ReadonlyMap\<string,string>): string[]. Its 72 mandatory paths are current source, registrations, parity/requirement/context and native-routing inputs. It never traverses archives or discovers future reports. Supplied maps are hermetic: an omitted key cannot borrow the checkout file.

The matrix contains 208 fixed Rust function contracts,712 original executable assertions,536 caller anchors with exact occurrence counts,67 literal-bearing protocol/matrix facts,81 default Rust tests and51 owning module edges. Five preserved CHECKPOINT_CONTRACTS independently retain the historical delegate, IfNeeded, selected prefix, lineage, no-save_snapshot, ordering and error obligations. Exact final Plan10 script/test hashes freeze the already executed Plan15 guard implementation and mutations; this integrity check supplements actual function contracts.

Actual source obligations include normal-maturity recipe provenance, genuine accepted Fjall history/undo/coins, configured daemon open, production elapsed worker wiring and joined settlement, ordinary connect/reorg, authenticated manual pruning, automatic 550 pressure, paired physical absence, consuming all-handle reopen, raw-input oracle construction, active/stale positive h1000 checkpoints, authenticated RPC, actual inbound and outbound TCP, and body/wallet restrictions. Fresh/saved missing-history, pending intent, ahead-cursor, publication/checkpoint/protection/coins/metadata and corruption controls retain their original registered assertions.

All final seven outbound and eleven inbound CF controls remain mandatory. Their concrete writer and production caller paths are guarded through send_response/send_filter_plans, sync_connected_peer_with_cancel, handle_inbound_stream, execute_inbound_filter, InboundFilterWriter and SocketFilterWriter. Queue record/retire methods preserve externally supplied pressure. Actual 31-byte prefixes,13-byte headers, full-frame-only receipts,0/62 versus189 bytes, factual PhysicalPrefixInvalidated versus CurrentRangeComplete, independent generic charges, bounded/excluded padding, real Notify Interrupted and separate pending-future drop are distinguished. Test-only manual acknowledgement/completion is rejected. Announcement execution does not satisfy CF evidence.

All-family absent-own-capability evidence requires bounded zero-byte EOF/reset/aborted disconnect, every original denied exchange on a fresh physical session, no post-denial Pong, zero accounting, removed peers and joined cleanup. The distinct advertised All-only/no-backend source still requires supported silent/Pong behavior and invalid-type255 disconnect. Original granted/oracle/stale/checkpoint matrices remain exact.

Lazy supplier/capture/admission/write controls remain after selected readiness/body-undo prefix and before unlink/coins/metadata. Four-stage no-op zeros, actual before/after snapshot failures, typed Headers errors, atomic observations, retry/reopen and old-missing-stale/header-only refusal remain assertive. Snapshot publication cannot manufacture ScriptsValid authority.

### Checked Unix and TypeScript Amendment

Two initial verification boundaries exposed the existing helper's restricted cfg(test) ownership vocabulary. Parent obtained a fresh planner amendment and independent PASS with zero blockers/warnings before this compatibility work resumed.

Only the original process.rs top-level cfg(all(test, unix)) registration earns a view. Its exact path/declaration/attribute/uniqueness/nesting are validated first. The replacement is padded to identical length with every newline and selector/assertion offset preserved. That view is used only for ownership and meaningful-assertion helper recursion; original bodies, literals, spans and hashes remain authoritative. All nine default process tests and refused-helper assertions remain. The separately ignored subprocess child cannot become default evidence. Local classification handles both Rust ignore spellings without editing the global helper.

Four actual TypeScript functions have 71 code anchors and35 exact raw caller/argument spans: validateRecipe, rawExpectations, wireExpectations and deriveRecipe. The local lexer preserves offsets, masks comments/quoted strings/regular expressions/templates, correctly skips nested template interpolation/string/template/brace state, validates syntax with Bun and requires one actual top-level declaration. Unsupported/ambiguous syntax fails closed; no general AST or runtime proof is claimed.

Controls bind exact raw schemas and bounds, both Python argv/modes/root/input, validated returned nodes and prior ancestry, oracle-derived wire inputs and positive checkpoints, validateResult, magic-only normalization, default bounded runner, Git pin/dirty checks and raw ordered digest. TS source negatives remain valid syntax. Implementation/formal mutation maps recompute actual six-source proof rows and formal source_digest before requiring the intended named function/argument failure. Missing source, comments/strings/templates, false ownership, duplicate declarations and unterminated syntax have independent controls.

### Sole Shared Claim Authority

checkPhase162ClaimState, loadPhase162OptionalSources, checkPhase162ExcludedClaims and phase162ClaimView remain the only claim-state authority. Exact ownership stays v2-5-integrated-retained-filter-proof with ordered CFPR-02/CFGR-01/CFGR-02. No second requirement/proof/report validator was introduced.

Pending, implementation and formal-complete positives are synthetic in-memory consistency fixtures. Published branches require their source hashes/lifecycle and lazily required formal reports through the frozen helper. Source negatives use Pending normalization or legitimately rebound published maps. Dedicated ownership, checkbox/traceability, proof hash/registration, report metadata/freshness, native default-command and excluded-claim negatives remain separate.

## Task 2: Exact Native Routing

Only eight lines were added to scripts/verify.sh. Both documented and executed sections run these exact commands immediately after Phase161:

```bash
bun test ./scripts/phase162-filter-oracle.test.ts
bun test ./scripts/check-phase162-integrated-filter-proof.test.ts
bun run scripts/check-phase162-integrated-filter-proof.ts
```

Both sections additionally contain the actual shared-fixture formatting obligation:

```bash
rustfmt --edition 2024 --check packages/testdata/phase162/recipe.rs packages/testdata/phase162/process.rs
```

The executed formatting line follows Cargo fmt and precedes strict Clippy. Plain rustfmt uses the repo toolchain; an initially duplicated rustup version was removed on parent review. The 152 documented and 170 executed original command lines remain in their exact prior order, including historical oracle/checkers, Cargo formatting/strict Clippy/build/tests/CLI binary, benchmarks, Bazel, coverage and final Phase138 ordering. Removal, duplication, replacement and reordering mutations fail. No full-native or --fast phase-completion claim is made here.

## Verification and Initial Failures

All commands used explicit ./ Bun test paths. Sessions completed normally and were polled within 60 seconds. No timeout increase, skipped failing assertion or zero-discovery success was used.

| Command/control                                      | Actual result                                                            |        Time |
| ---------------------------------------------------- | ------------------------------------------------------------------------ | ----------: |
| Initial API RED                                      | 0 pass,1 fail,1missing-module error                                      |        74ms |
| Early current-source construction                    | 0/1; initial caller/module/indented-native parsing corrections           |       683ms |
| Corrected initial Pending bootstrap                  | 1 pass,0 fail                                                            |       782ms |
| First complete mutation matrix                       | 1,453 pass,12 fail; repeated-anchor/selector/replacement-builder escapes |     106.32s |
| Initial targeted corrections                         | 25 pass,4positive failures at shared Unix registration                   |       3.32s |
| First Unix-view positives                            | 0/4; separate ignored-child spelling mismatch                            |      1.195s |
| Corrected Unix/TS amendment preflight                | 49 pass,0 fail;255 expectations                                          |       3.24s |
| All amendment targeted controls                      | 488 pass,6mutation-builder syntax failures;4,168 expectations            |      29.14s |
| Corrected original-offset mutation builders          | 48 pass,0 fail;340 expectations                                          |       3.15s |
| Final dead/default-evidence preflight                | 25 pass,0 fail;47 expectations                                           |       3.34s |
| Actual caller/queue/writer controls                  | 147 pass,0 fail;434 expectations                                         |      15.36s |
| Last constructor preflight                           | 62 pass,5 fail; one missing original trailing comma                      |       9.59s |
| Corrected exact constructor/manual-receipt preflight | 22 pass,0 fail;56 expectations                                           |       4.08s |
| Final required checker suite                         | **2,596 pass,0 fail;9,657 expectations**                                 | **238.81s** |
| Independent raw oracle suite                         | **28 pass,0 fail;44 expectations**                                       |   **3.16s** |
| Live Phase162 Pending checker                        | Passed,208 contracts                                                     |      Scoped |
| Live historical158/159/160/161 guards                | All passed                                                               |      Scoped |

Repeated before/after calls now require exact function occurrence counts as well as order. Trait-method mutation builders use the actual concrete selector offset. Replacement mutations must change the live command. Generic valid-TS mutations now select identifiers in original masked source, preserving keyword boundaries, and explicitly assert syntactic validity. The manually authored RPC fixture constructor anchor was corrected to its actual trailing-comma syntax. None of these corrections changed production/tests/fixtures or weakened acceptance.

Final managed all checks: 0 findings, 1,463 files scanned. Bash syntax, two-space shfmt check, shared rustfmt, owned diff whitespace and stub scan pass. No standalone TS formatter, tsconfig, package manifest or LSP diagnostic tool is configured; Bun execution/mutations and managed checks cover these scripts. A default-tab shfmt probe showed pre-existing indentation differences; the correctly matched two-space check passes without rewriting the script.

Local temporary receipts are /tmp/phase162-plan11-first-full.log, targeted.log, unix-positive.log, unix-positive-final.log, ts-positive.log, amendment-targeted.log, all-targeted.log, valid-mutations.log, final-preflight.log, caller-final.log, final-caller-preflight.log, last-preflight.log, full-final.log and oracle.log, each with the phase162-plan11- prefix. They are local command receipts, not published parity certification.

## Exact Bootstrap and Parent Handoff

After parent registered the three actual new TS files, the live checker passed against the real checkout. Independent final bootstrap inspection observed:

- Shared state:pending; no failures.
- All three requirements retain Pending checkboxes/traceability and the exact owner remains in_progress.
- docs/parity/v2-5-integrated-filter-proof.md is absent and is not mandatory.
- 162-VERIFICATION.md, 162-NATIVE-EVIDENCE.md, 162-REVIEW.md and 162-SECURITY.md are all absent and not mandatory.
- The actual six-root digest remains a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575.
- Formal native evidence basename remains 162-NATIVE-EVIDENCE.md. Only the formal 162-VERIFICATION.md ends with that GSD verification suffix. Native completion requires the shared helper's exact standalone bash scripts/verify.sh; --fast-only and inline/substr-only alternatives fail.

Parent registered only these existing paths under the unique scoped owner: the checker, contracts child and test listed below. No new Rust path/breadcrumb was created. Current source breadcrumb groups and original literal source comments are checked without changing registries.

| Final owned source                                          | SHA256                                                           |
| ----------------------------------------------------------- | ---------------------------------------------------------------- |
| scripts/check-phase162-integrated-filter-proof.ts           | 7bcd8f30fd87d303e08c96701db78ef7e9436b149f61b0841ba5104e8aab1a57 |
| scripts/check-phase162-integrated-filter-proof/contracts.ts | 4643094814d644ba34ae4b7488d97f514fda1ac207a96e81331d35cd73c6e287 |
| scripts/check-phase162-integrated-filter-proof.test.ts      | 2112377599914f2e1a21ac0614f16cc7f94057538bbc389b9d643fca98b5f27f |
| scripts/verify.sh                                           | 0558f24398674b584f9653585cbf856c8c8752fc2a66a70fdf4977b8dcda6041 |

Source lengths are 398/585/503/425 lines in that table order. Parent owns final Cargo/shared formatting, Plan12's actual proof/UAT/docs, independent source/security reviews, the full default native verifier, formal lifecycle reports, requirement activation, generated LOC, state/roadmap/todo and normal-hook Git finalization.

## Simplification, Mitigations and Limits

The explicit simplification pass retained one shared claim validator, fixed paths, existing Rust selector/ownership/assertion helpers, one bounded original-source-version cache per file and declarative source matrices. No production abstraction, dependency, extra file or general parser framework was added. Functions remain below the managed approximate 161-line trigger; all files remain below 628 lines. The larger contracts file is the assigned declarative fixed evidence matrix.

| Threat     | Scoped control                                                                                                                                  |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| T-162-11-1 | Named real-caller/assertion/argument removals and valid semantic mutations fail; runtime/native/review evidence remains independent.            |
| T-162-11-2 | Exact once/order routing plus every original native command is preserved and mutation checked.                                                  |
| T-162-11-3 | Sole shared ownership/requirement/publication validator; genuine bootstrap and rebound synthetic state controls; exclusions remain independent. |
| T-162-11-4 | Fresh no-grant zero-byte terminal sessions/cleanup remain distinct from All-only/no-backend silent/Pong.                                        |
| T-162-11-5 | Lazy selected checkpoint/no-op/typed atomicity controls and frozen actual Plan15 lineage/IfNeeded/no-snapshot guards remain protected.          |

These are supplementary structural controls, not executed behavioral certification or ASVS certification. No blocking stub, unwired UI/data source or new production endpoint/auth/schema/trust authority was introduced. Empty denial expectations and in-memory synthetic report fixtures are intentional controls; no future evidence placeholder was written.

## Deviations and Remaining Gates

The parent-approved, independently checked Unix/TS amendment stays within the original four-file ownership and two tasks. Construction/mutation-builder fixes above are Rule1/3 corrections to the checker/tests. Root's toolchain-source correction removes duplicate configuration. No outside-owned source, global helper or architectural change was made.

Task and metadata commits are explicitly deferred by D-14. No stage, commit, push, branch change or hook bypass occurred. No authentication gate occurred.

All source/security/native/formal activation and normal-hook Git gates remain parent responsibilities. Preserve BIP37/V0, assumeutxo/dual-chainstate, archive scale, public defaults/network CI, implicit repair/import/download, production services/readiness/funds, unrelated v2.4 advisories and Phase161 presentation/redaction limits. The milestone remains unaudited and unshipped. Ready for Plan12 after parent acceptance; Cargo is idle.

## Self-Check: PASSED

The four owned source paths exist and match the final SHA256 table; the three created TS paths are registered under the exact owner. The final 2,596-test matrix, 28-test oracle, live bootstrap, four historical guards and managed/shell/shared-format/diff checks pass. Actual proof and all four formal reports are absent; requirements remain Pending and the six-source digest is unchanged. No blocking stubs were found. All owned sessions exited. Frontmatter uses the exact lifecycle identity, empty requirements-completed and only its two top delimiters. There are no nested commits to verify because D-14 defers Git to root.
