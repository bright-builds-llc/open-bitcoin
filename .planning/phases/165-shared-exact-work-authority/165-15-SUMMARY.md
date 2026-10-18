---
phase: 165-shared-exact-work-authority
plan: "15"
subsystem: fixture-runtime-assembly
tags: [earned-work, fjall, basic, retained-prefix, mutation-controls]
requires:
  - phase: 165-13
    provides: Reviewed exact-work owners and bounded evidence guards
provides:
  - Public consuming genuine Fjall fixture constructor with explicit full parameters
  - Independent complete-prefix physical authentication and immutable final identity check
  - Normal same-core BASIC assembly and validated fixture ancestry with unknown historical order
  - External and private behavioral controls plus renewed current source guards
affects: [165-16, 165-17, 165-24]
tech-stack:
  added: []
  patterns: [Same owned core, Existing-engine independent replay, Captured final identity]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/live_fixture.rs
    - packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs
    - packages/open-bitcoin-node/tests/phase165_live_fixture.rs
  modified:
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/open_runtime.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - docs/parity/source-breadcrumbs.json
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
    - scripts/check-phase165-shared-exact-work.test.ts
key-decisions:
  - Require genuine source origin before independent replay; replay never upgrades raw ownership.
  - Compare both aliased views against independent expected coins and retain that expectation across startup.
  - Keep normal BASIC proof acquisition outside publication-held content checks.
  - Historical nonroot bodies retain unknown order; new events after genuine genesis earn actual preference.
requirements-completed: []
requirements-contributed: [CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T03:58:48Z"
client_date: "2026-10-16"
duration: approximately 31min
completed: 2026-10-16
tasks_completed: 3
files_modified: 12
---

# Phase 165 Plan 15: Genuine Fjall Fixture Runtime Summary

**A consuming public fixture constructor retains the genuine Fjall core through normal BASIC startup, independently authenticates the complete retained prefix, and checks immutable captured identity after graph assembly.**

## Delivered Scope

All three tasks are complete within the twelve approved implementation paths plus this summary. The approved plan SHA-256 remains de54b38f05b8dd0a4cca02d100ff36c884ea823710017a7c19d6b07778de63a3; the integration ledger remains 978fbfdf92d53c2619cc999872f3ba4662483f3d84b317f52032775a16a4881d. The current approved check supersedes its preserved earlier rejected drafts.

The seven-argument public constructor accepts only the already-owned managed Fjall bundle. Its pure gate checks actual live origin, full selected parameters, pending publication and all four canonical root identities before snapshots, backend-parent reads or normal initialization. Canonical refusal has a fixed specific reason independent of the selected config label.

Publication-held preflight proves the concrete parent and requires every actual active body, physical undo, exact positions, ScriptsValid identity and complete validation coverage. The existing Chainstate engine replays the entire active prefix into independent Memory coins with P2SH, the same parameters, actual block times and the ZERO compatibility carrier. Expected coins/B/heads, positions, undo and confirmation counts must match both genuine source and physical facts. Thus two fresh aliased Fjall snapshots cannot certify equally poisoned rows. Physical None counts remain unknown; conflicting Some counts refuse. No count write, enrichment, normalization or missing-input restoration was added.

The constructor captures owned independent facts and the private source binding, consumes the same source core, then runs normal initialization and recovered-owner assembly. The ordinary opener uses the same new prepare/finish helpers consecutively and retains its network-selected parameters. Fixture assembly supplies its explicit parameters throughout, including maturity one, instead of replacing them with config defaults.

Between prepare and finish, captured active headers are validated once, the unique root body gets checked linkage, and historical nonroots use the existing cached-body validator. Their order stays unknown through duplicates, copied facts and descendants. The graph grants no accepted-tip preference. Strict extensions work; unknown equal-work replacements refuse. A genesis-only source permits genuinely new A1/A2/B2 events to earn actual order: separate normal nonroot header batches, B2 pending before A bodies, A1 release, later A2 acceptance and a legitimate earlier-eligible equal reorg.

After graph assembly, the same final checker compares current source/physical/body/validation facts against the immutable capture under publication. Publication is released before graph mutation and normal BASIC proof/owner/readiness work. Normal BASIC startup creates its own legitimate generation; stable ScriptsValid source facts are checked separately.

## Executed Evidence

All Bun/Cargo commands used the command-local pinned Bun 1.3.9 PATH below. Rust was independently reported as 1.94.1. Cargo jobs ran serially through command-timings; substantial Bun matrices ran only after Cargo stopped. Quiet resumable sessions were polled, actual exits recovered, and no process was suspended or killed.

```bash
export PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

| Check                              | Actual result                                                                                                                                                                                                                                        |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Missing public method RED          | phase165-p15-external-red, 2026-10-17T03:27:24.426Z: E0599 for the absent constructor, exit101, 2.617s; session 52305. No behavioral test execution is claimed for this compiler RED.                                                                |
| First public GREEN                 | Genuine empty source, 1/1, exit0, 7.695s total; session 52396.                                                                                                                                                                                       |
| Expanded external corpus           | Initial 11/12 exposed an invalid empty-source flush premise. Corrected empty setup retained the genuinely empty core without flushing; 12/12 then passed.                                                                                            |
| Final external discovery/execution | 14 actual tests discovered and 14/14 executed, zero failed/ignored, 5.80s execution, exit0. Final execution key phase165-p15-external-final, 5.933s total; strict sequence session 35932. The target links the normally compiled Node library.       |
| Private discovery/execution        | Final 34 actual tests discovered, exit0; session 48797. Final unit execution 34/34, zero failed/ignored, 14.87s execution, 23.111s including compilation, exit0; session 8988. The prior 33/33 receipt remains separately recorded.                  |
| Scoped format                      | Workspace format and final check passed. Final check key phase165-p15-fmt-check, 1.383s, exit0.                                                                                                                                                      |
| Scoped strict lint                 | Node all-targets/all-features clippy with -D warnings passed after the final corpus addition, 6.197s, exit0.                                                                                                                                         |
| Scoped build                       | Node all-targets/all-features build passed after final source stabilization, 0.282s cached, exit0. Strict sequence session 48797 recovered exit0.                                                                                                    |
| Current source guard RED           | Before current identity renewal, the actual baseline refused the changed sync registration and test owner: one executed test, exit1. This is source-policy RED, separate from Rust execution.                                                        |
| Final focused current165           | 77 passed, zero failed, 397 expectations, 32.98s.                                                                                                                                                                                                    |
| Final focused current164           | 19 passed, zero failed, 109 expectations, 14.65s.                                                                                                                                                                                                    |
| Complete current164                | 781 passed, zero failed, 1,159 expectations, 295.58s.                                                                                                                                                                                                |
| Complete current165                | 1,069 passed, zero failed, 4,050 expectations, 257.50s.                                                                                                                                                                                              |
| Exact checker commands             | Current164 passed: 132 fixed Rust inputs/463 reviewed edges. Current165 passed: 84 reviewed owners/28 executable test-owner files.                                                                                                                   |
| Breadcrumbs/managed checks         | 1,201 Rust breadcrumbs passed. Managed checks: 1,521 scanned inputs, zero exceptions/findings; lesson check also passed.                                                                                                                             |
| Stable inputs and scope            | All 221 saved input/checker SHA-256s matched unchanged after both complete matrices and exact checks. Ordered current165 inventory has 216 entries and its renewed pre-read pin is d53e4c64edcb651547895005295f7a269841c52aaecbd84d47eb2d72141e00c7. |
| Final aggregate process            | Session 81511 ran final focused checks, both complete matrices, both exact checkers, breadcrumbs, managed checks and whitespace verification serially; recovered exit0.                                                                              |
| Caps/diff/index                    | Every owned Rust/TS path is at most628 lines; largest unit file611, helper registry609, current164 contracts604. Whitespace check passed and index remains empty.                                                                                    |

The final commands were:

```bash
bun run scripts/command-timings.ts run --key phase165-p15-external-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --test phase165_live_fixture --all-features
bun run scripts/command-timings.ts run --key phase165-p15-fmt -- cargo fmt --manifest-path packages/Cargo.toml --all
bun run scripts/command-timings.ts run --key phase165-p15-fmt-check -- cargo fmt --manifest-path packages/Cargo.toml --all -- --check
bun run scripts/command-timings.ts run --key phase165-p15-clippy-final -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
bun run scripts/command-timings.ts run --key phase165-p15-build-final -- cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features
bun run scripts/command-timings.ts run --key phase165-p15-unit-list -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib sync::tests::phase165_live_fixture --all-features -- --list
rg ': test$' .local/open-bitcoin-dev/phase165-plan15-final-unit-list.log
bun run scripts/command-timings.ts run --key phase165-p15-unit-final -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib sync::tests::phase165_live_fixture --all-features
bun run scripts/command-timings.ts run --key phase165-p15-external-list -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --test phase165_live_fixture --all-features -- --list
rg ': test$' .local/open-bitcoin-dev/phase165-plan15-final-external-list.log
bun run scripts/command-timings.ts run --key phase165-p15-external-final -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --test phase165_live_fixture --all-features
bun test ./scripts/check-phase165-shared-exact-work.test.ts --test-name-pattern 'phase165 live fixture guard|actual current source'
bun test ./scripts/check-phase164-canonical-identity.test.ts --test-name-pattern 'phase165 live fixture guard'
bun test ./scripts/check-phase164-canonical-identity.test.ts
bun test ./scripts/check-phase165-shared-exact-work.test.ts
bun run scripts/check-phase164-canonical-identity.ts
bun run scripts/check-phase165-shared-exact-work.ts
bun scripts/check-parity-breadcrumbs.ts --check
bun scripts/bright-builds-check.ts all
git diff --check
```

Detailed logs are retained under the ignored prefix .local/open-bitcoin-dev/phase165-plan15-. Final suffixes are final-unit.log, final-unit-list.log, final-external.log, final-external-list.log, final-fmt-check.log, final-clippy.log, final-build.log, final-focused165.log, final-focused164.log, final-guard164.log, final-guard165.log, final-check164.log, final-check165.log, final-breadcrumbs.log and final-managed.log. The saved input/checker map is `.local/open-bitcoin-dev/phase165-plan15-input-hashes.json`.

## Behavioral and Source Evidence Boundaries

External controls use only public source acceptance, extraction, actual flush with empty header entries and rewrapping. They cover genuine empty and populated Enabled paths, live counts/parameters, raw replayable refusal, changed parameters, three clean-cache public alias attacks, all four actual canonical routes, strict populated extension/unknown-equal refusal, public-sink pruning and changed physical undo.

Private controls additionally cover mechanical, preview, unresolved accepted publication, foreign parent, beyond-maturity parameter mismatch, unflushed state, B/heads/positions, Some-count conflict, missing/pruned/malformed body/undo, validation coverage, retained masks, normal BASIC proof and preserved physical None counts. Every canonical case actually accepts its canonical body using its own bits and test-only BIP34-off parameters, binds those exact parameters and reaches canonical-specific refusal with zero constructor physical-stage/init counters. Earlier backend creation and source acceptance already have effects; they are not described as a zero-effect opener.

The deterministic writer control executes the SAME production capture, actual normal initializer, same-core recovered assembly and actual graph stages, then an ordinary aliased public coins writer with unchanged B, then the SAME final captured checker. The source remains Live and the final check refuses. This is executed post-graph stage interleave evidence, paired with loaded-source public-order proof. It is not whole-public-constructor concurrent execution. No ABA/revision-history detection, rollback guarantee or consumed-source return is claimed; post-init errors can follow initializer effects.

Source mutations change actual loaded owners, assert their anchors and unchanged positive baseline, then require the owning path/rule. They protect origin/params/pending/canonical/parent, independent replay/body/undo/flags, complete physical facts, immutable capture/final ordering, same-core recovered BASIC, graph validation/unknown history, enabled external/private tests and old cfg-test helper boundaries. These source-policy mutations are not relabeled executed Rust semantic mutations.

Only affected current identities and new current edges were renewed after actual Rust GREEN and source review. Historical162/163 tooling, corpora, recipes and receipts, six exact native registrations and the old164 all-six-removal control remain preserved. The current preservation source is still20,688 bytes/SHA-256 e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b. Its original20,556-byte attribution remains Git blob0345bab1bfbd76be3078453a6b9201fdf73941cc/SHA-2563d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630. This source-blob identity is not a task commit.

## Diagnostics, Adjustments and Simplification

- Initial compile diagnostics exposed the correct CompleteCoinsSnapshot module, numeric PeerId alias, non-Copy OutPoint, Fjall keyspace factory signature, encoded-head Result and BlockUndo field. All were fixed in the owned source/tests before final verification.
- Empty accepted-prefix setup originally tried to flush an absent B. The genuinely empty source now skips that flush; the external test explicitly admits its root afterward and proves actual first connection/work, excluding Duplicate.
- One early private run passed25/26; its stringified HashMap comparison had nondeterministic ordering. Refusal preservation now compares the complete ordered physical namespace rows, including body/undo/meta/coins/index rows and prune observations. No physical namespace assertion was dropped.
- One expanded run passed27/30: two same-label temporary paths collided, and the genuine new-body tie reached BASIC but lacked retained body data. A new atomic suffix is confined to the new corpus factory; the old shared temp helper is unchanged. The next29/30 run exposed missing required undo. Persisting actual checked bodies and actual accepted core undo through existing APIs produced the33/33 BASIC-enabled positive and then final34/34. Neither required-data guard nor preference assertion was weakened.
- A tentative cloned Fjall setup did not compile because Fjall owners deliberately are non-Clone. Parent clarified the existing genuine consuming rebind for masks. After a real flush, explicit same-value/known-absent cache mutation taints origin; independent actual full-prefix validation consumes its genuine rebind into the SAME Fjall core while complete coins/positions/undo/counts match. Target masks survive; live origin/full binding and physical prerequisites are asserted before poisoning. This is validated-source rebind with retained masks, not a clone/flush claim.
- The public pruning control first encountered E0624 on the private store method. It now uses the existing public FlushPersistSink operation, with no visibility expansion.
- Test setup helpers live in the already-owned registry so all test functions remain in recognized test paths and all files satisfy628 lines. No additional implementation ownership or cap exception was needed.

The simplification pass retained one genuine returned core, one existing validator/arithmetic engine and the existing normal BASIC machinery. Replay supplies independent expected facts only; no second authority/count model, callback/interleave API, revision getter, test capability export or dependency was added. Existing package README scope remains accurate.

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust/TypeScript standards and relevant07–09/13 context informed execution. Both active lessons were loaded completely:7,188 bytes/2,397 conservative estimated tokens. No project skill directories were present. Scoped Markdown uses available GFM/frontmatter plugins; no TS formatter/project was available, so adjacent conventions, Bun execution and managed checks supply the applicable surface.

No blocking stub or undeclared endpoint/auth/schema/file-opening surface was introduced. All new trust surfaces are covered by the plan's declared threats. No authentication gate occurred.

## Final Source Identities

These are full-file SHA-256s, distinct from function/source-identity digests used by the current guards.

| Owned path                                                           | SHA-256                                                          |
| -------------------------------------------------------------------- | ---------------------------------------------------------------- |
| `packages/open-bitcoin-node/src/sync.rs`                             | 88cf2a12bca8a236a4a8d821afe96e17cdd13a63c5ff8f159da5d24e3dfbe035 |
| `packages/open-bitcoin-node/src/sync/open_runtime.rs`                | e0d80123609b1f85a9edb06c18f1c1682268215bace22f343a2c420357c338d5 |
| `packages/open-bitcoin-node/src/sync/live_fixture.rs`                | 4da004e8b12a1b6bd0da8d460dde94b70a7c1119893fbe03ba2d048d707b9b9c |
| `packages/open-bitcoin-node/src/sync/tests.rs`                       | a9b4b57990b015cee3909013834862c98e234ec121b7bee7467a460fdee21732 |
| `packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs` | 2768f24bf56b51acd8186a2e92eeb7feb21f7482e85c678f561ce03d75aba8f8 |
| `packages/open-bitcoin-node/tests/phase165_live_fixture.rs`          | 392ca0c03bf4dc095c6175ff05552beab3ad794cf78399c81d9d88d719720e4f |
| `docs/parity/source-breadcrumbs.json`                                | c3aacf47b0e514adb8f65ad1d08312b0a52f30e6746925986045a60dffa2ce68 |
| `scripts/check-phase164-canonical-identity/contracts.ts`             | 08df122587582cdb728ea2a699535377b67cac41019c08d7454dba43a5a7f690 |
| `scripts/check-phase164-canonical-identity.test.ts`                  | 0bbeb183dcbed3ea1c126e36c4fa4a4fd02e955947521b3e398cb09bac05cffb |
| `scripts/check-phase165-shared-exact-work.ts`                        | be3cf992cf6dabadfe1bde05becd8145612911ac03e8d715b699a82446830e76 |
| `scripts/check-phase165-shared-exact-work/contracts.ts`              | ff8ef8d9e6573c1edd7969f470864913ab7b5b1cdfeabd55afb2380454320437 |
| `scripts/check-phase165-shared-exact-work.test.ts`                   | 9f22efe3c6b7a1e335b0bce369ab2981a147978e66e05c9ab3f3b06145ec5858 |

## Task Commits and Remaining Parent Gates

All task/RED/GREEN/metadata commits are explicitly deferred to the parent after all24 plans, full default native verification and independent source/security/formal/lifecycle gates with normal hooks. This executor created no worktree, staged/committed/pushed nothing, touched no hooks and changed no shared STATE/ROADMAP/REQUIREMENTS/config/todo or other plan. Base remains d0104a997c0362bf65b81de1c8fec8ddc3012a1b with earlier uncommitted work preserved. No commit hashes are invented.

Plan15's own scope is GREEN. This does not erase Plan14's actual249-failure diagnostic or qualify the workspace; later repair plans and final default verifier remain parent-owned. No Plan16 implementation was started here. Phase166 retains canonical bootstrap, exact all-handle reopening and canonical genesis BASIC;167 retains contextual completeness;168 retains actual pinned-Knots qualification. The32 local Open Bitcoin refusal children are not pinned-Knots differential evidence.

## Self-Check: PASSED

All twelve declared implementation paths and this summary exist. All221 saved input/checker hashes remain unchanged across the complete matrices and final checks. The48 Rust behavioral cases, strict lint/build/format, both complete source guards, three new breadcrumbs, managed checks and owned-file caps are verified. The summary uses GFM/frontmatter formatting and exactly two frontmatter delimiters. No blocking stub remains; the index is empty, and no executor commit is expected. Shared phase acceptance and finalization remain parent-owned.
