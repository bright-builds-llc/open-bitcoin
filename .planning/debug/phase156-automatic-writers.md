---
status: awaiting_human_verify
trigger: "Default-parallel automatic_prune::tests aborts with writers.rs Fjall ENOENT and TempStore double panic; 25 serial controls pass."
created: 2026-10-05T00:00:00Z
updated: 2026-10-13T19:19:24Z
---

## Current Focus

hypothesis: Identical wall-clock paths give multiple TempStore owners; a failed open's cleanup deletes the sibling database.
test: Original equal-clock red/green and repeated parallel controls are preserved below. Later Phase 162 full-default and normal-hook receipts were rechecked on 2026-10-12 with success/exit0.
expecting: Historical automated verification uncertainty is resolved by actual later receipts; no human confirmation has been performed.
next_action: Preserve the original dated failures and historical e0 acceptance below. The current 21-input source14fa passed the full default verifier, four complete layout suites, independent acceptance, functional activation, renewed audit, readiness execution and final5/5verification. Strict normal-hook saving is the wrapper gate; actual Git outcomes are derived from its receipts and repository state. The original human-confirmation condition remains unperformed; keep this session unarchived.

## Symptoms

expected: The automatic-prune controls pass with the default parallel Rust test runner.
actual: Two writer fixtures failed before Phase 156 snapshot construction; process aborted during cleanup. All 25 controls pass serially.
errors: FjallNodeStore::open BackendFailure Runtime ENOENT; TempStore::drop remove_dir_all NotFound; SIGABRT after double panic.
reproduction: cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib network::runtime_authority::automatic_prune::tests
started: Phase 156 Plan 05 verification; writers.rs unchanged by Plan 05.

## Plan

- [x] Reproduce original parallel failure and minimize to writer-fixture lifetime.
- [x] Rank and test falsifiable alternatives before changing behavior.
- [x] Add a failing regression at the concurrent fixture allocation/lifetime seam.
- [x] Apply the smallest test-helper fix, remove instrumentation, and verify default-parallel controls repeatedly.
- [x] Review scoped diff and hand Cargo ownership back for full native phase verification.

Material guidance: AGENTS.md, Bright Builds sidecar, placeholder overrides, code-shape/testing/verification/Rust standards, full active lessons (7,188 bytes; 2,397 conservative tokens), Phase 156 context and Plan 05, diagnosing-bugs skill. No lesson-audit trigger. Root initialized GSD debug state, synchronized the checkout and owns phase tracking; no commits or pushes in this task.

## Eliminated

- hypothesis: Independent backend initialization race or external removal is required.
  evidence: Tagged allocation logs showed two duplicate path pairs, followed by Locked, cleanup and sibling ENOENT. Local cleanup fully explains the disappearance.
  timestamp: 2026-10-05T00:07:00Z

## Evidence

- timestamp: 2026-10-05T00:00:00Z
  checked: Captured Plan 05 diagnostic and timing b17d1c1b-32ed-4856-a7e7-3a3c0a88f27c.
  found: Original 25-test parallel run aborted; serial run 8b50e85c passed all 25 in 4.84 seconds. Failure lacks the requested path, so the surviving directory cannot establish a collision.
  implication: Investigate fixture allocation/lifetime before Phase 156 behavior; serial success is not closure.

- timestamp: 2026-10-05T00:00:00Z
  checked: Debug knowledge base.
  found: Only resolved entry concerns a CLI RPC/timeout regression; no two-keyword match.
  implication: No known pattern overrides fresh investigation.

- timestamp: 2026-10-05T00:04:00Z
  checked: Original command and minimized writer-only command, unmodified source.
  found: Original 25-test default-parallel passed in 2.75s; immediately subsequent writer-only four-test run reproduced open ENOENT at line 41, cleanup NotFound at line 30 and SIGABRT on attempt one (timing key phase156-debug-writer-repro).
  implication: Four concurrent real-store fixtures suffice; unrelated automatic-prune and Phase 156 scenarios are not necessary triggers.

- timestamp: 2026-10-05T00:04:00Z
  checked: Hypothesis ranking sent to root before probes.
  found: Allocation collision predicts duplicate paths; lifetime predicts unique paths deleted with live clones; backend initialization predicts unique paths and open ENOENT before local Drop; external removal predicts disappearance without local Drop. Instrumentation changes only observability.
  implication: Path logs differentiate alternatives without guessing collision from the serial pass.

- timestamp: 2026-10-05T00:07:00Z
  checked: Instrumented writer-only default-parallel run (phase156-debug-writer-paths), PID 61149.
  found: Four fixtures allocated two paths, each twice: automatic-prune-writers-61149-1791158820660225000 and automatic-prune-writers-61149-1791158820660224000. Losers returned Locked and dropped TempStore; siblings subsequently returned ENOENT and cleanup NotFound, then SIGABRT.
  implication: Wall-clock nanos do not guarantee unique fixture ownership. Reserve directories before any Fjall open. All temporary DEBUG-156-writers instrumentation removed when extracting the equal-clock seam.

- timestamp: 2026-10-05T00:09:37Z
  checked: Equal-clock real fixture regression before allocation fix (phase156-debug-equal-clock-red).
  found: A second thread opening at the same injected timestamp got Locked; its destructor removed the first fixture directory. The first fixture's unwinding cleanup got NotFound and aborted with SIGABRT. This requires only two real fixture lifetimes.
  implication: Correct regression seam forces duplicate clock values and exercises actual Fjall open plus fixture cleanup with a live survivor.

- timestamp: 2026-10-05T00:11:32Z
  checked: Same equal-clock regression after create_dir reservation (phase156-debug-equal-clock-green).
  found: One passed, zero failed; test body 1.40 seconds. The closed sibling path disappears while survivor path and its actual retained three-byte payload remain.
  implication: Reservation directly addresses the deterministic shared-lifetime failure. Parallel original controls remain to validate stability.

- timestamp: 2026-10-05T00:13:13Z
  checked: Repeated default-parallel writer filter and full automatic-prune filter through pinned Bun 1.3.9, serialized Cargo timing wrapper.
  found: Writer filter passed 20/20 runs, each 5/5 (100 test executions); full controls passed 3/3 runs, each 26/26 (78 test executions), with bodies 3.90s, 3.90s, 4.12s. All 23 timing records have exitStatus 0.
  implication: Original failure is resolved in repeated default-parallel operation, including existing writer controls and new equal-clock lifetime regression. Serial-only success was not used for closure.

- timestamp: 2026-10-05T00:13:13Z
  checked: Final source diff, rustfmt --edition 2024 --check, scoped git diff --check and DEBUG-156-writers search.
  found: Only fixture allocation, injected timestamp/open seam and one lifetime regression changed; 65 additions/5 deletions, total 282 lines. Formatting and whitespace checks pass; temporary logs removed. No production code or dependencies changed. Strict remove_dir_all cleanup remains unchanged.
  implication: Simplification pass chooses filesystem reservation as the single ownership authority; no redundant atomic counter, mutex, sleep, serialization or cleanup error suppression. Root may resume Cargo and owns full native phase gate.

## Verification Commands and Records

All Cargo commands ran with `PATH=/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64:$PATH` through `bun run scripts/command-timings.ts run --key <key> --`.

| Key                                  | Command after wrapper                                                                                                                                       | Result / record                                                                                               |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `phase156-debug-automatic-original`  | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib network::runtime_authority::automatic_prune::tests`                              | Initial unmodified 25/25 pass; `10fcd4e1-a125-4a0f-8a25-3cfb93c768e7`                                         |
| `phase156-debug-writer-repro`        | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib network::runtime_authority::automatic_prune::tests::writers`                     | First minimized attempt SIGABRT, open ENOENT/cleanup NotFound; `fd5423a4-e84c-4596-ad54-f082b2597fd5`         |
| `phase156-debug-writer-paths`        | Same writer filter with `-- --nocapture`                                                                                                                    | Duplicate paths observed, Locked → cleanup → sibling ENOENT/SIGABRT; `940799a6-1293-4a0b-a2f0-089c82b016ac`   |
| `phase156-debug-equal-clock-red`     | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib automatic_prune_equal_clock_fixture_cleanup_preserves_live_store -- --nocapture` | Deterministic equal-clock shared-lifetime SIGABRT; `78784728-bdcc-4a23-9544-a65fb34c8703`                     |
| `phase156-debug-equal-clock-green`   | Same regression without `-- --nocapture`                                                                                                                    | 1 passed; `bbeb2e54-94c9-46b4-93b9-93dc79f9f0e3`                                                              |
| `phase156-debug-writer-stability`    | Writer filter with `-- --quiet`                                                                                                                             | 20 runs × 5 passed, first `82c70818-8c5b-4cb0-ba05-bee258d016c0`, last `3d7dcd4e-ab16-497c-a382-759c511431c8` |
| `phase156-debug-automatic-stability` | Full automatic filter with `-- --quiet`                                                                                                                     | 3 runs × 26 passed, first `7c5dc9f4-2ed8-4fc2-ac59-8a660fb6840d`, last `89ea57bd-1226-447f-a3e7-d5ac990ceb70` |

The new regression keeps one real fixture live while another thread opens and closes a second fixture at the same timestamp, then verifies independent paths, sibling cleanup and retained payload access. Changing only allocation reservation turned its deterministic red into green. Compiler/test startup latency remained a separate host symptom; no process/cache/host remediation was attempted. Diagnostic rustc sample `/tmp/rustc_2026-10-04_190601_eaIH.sample.txt` was captured read-only while compiling temporary instrumentation.

## Resolution

root_acceptance: Root accepted automated verification under yolo pipeline based on actual reproduction, duplicate-path evidence, deterministic red/green and 23 parallel pass records. This records orchestrator acceptance, not human testing or human confirmation. The required debugger instructions say archive_session runs only when checkpoint response confirms the fix works end-to-end and require user confirmation before marking resolved; therefore the artifact remains unarchived under that instruction. No human-specific test gap exists for the fixture; the outstanding full native phase gate is separate and root-owned.

root_cause: Multiple parallel fixtures can have equal SystemTime timestamps; unreserved pid+timestamp paths alias. A losing Fjall open panics, its TempStore cleanup removes the sibling path, and the sibling ENOENT cleanup causes double-panic SIGABRT.
fix: Reserve the directory with create_dir before creating TempStore. Retry only AlreadyExists with a checked numeric suffix; report other filesystem errors. Preserve strict Drop cleanup and existing automatic-prune controls.
verification: Equal-clock regression red/green; 20 default-parallel five-writer passes; three default-parallel 26-control passes; rustfmt/scoped whitespace checks and final simplification/diff review passed. Full native verification remains root-owned before phase finalization. Session is not archived or committed; workflow confirmation remains pending under the debug protocol.
files_changed: [packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/writers.rs]

## Automated evidence disposition | 2026-10-12

The original 2026-10-05 symptoms, hypotheses, ENOENT/SIGABRT failures, deterministic equal-clock red/green and 23 parallel success records above remain historical evidence. The earlier statement that the full native gate was next describes that date's state.

Read-only inspection of actual `verify-full` JSON receipts confirmed `outcome: success`, `exitStatus: 0` and `verifyMode: full` for `6c7c6001-7fbd-42ec-875d-840c0c380707` (2026-10-12T02:51:15.991Z–03:26:53.544Z; 2,137,553ms wrapper) and `b427fa6a-122a-4c28-9dcb-e58342ec3eac` (04:32:30.640Z–05:06:55.177Z; 2,064,537ms wrapper). Both record baseline `a77edab276a9daa086055aa61e6143ce86bc1218` with `dirty: true`, Darwin arm64 and Rust 1.94.1. These are Phase 162 worktree/default-verifier and normal-hook evidence, not exact-d47 tree runs or fresh Phase 163 results. Receipt paths are `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-12T02-51-15.991Z-6c7c6001-7fbd-42ec-875d-840c0c380707.json` and `2026-10-12T04-32-30.640Z-b427fa6a-122a-4c28-9dcb-e58342ec3eac.json` in that same directory. [Phase 162 native evidence](../phases/162-real-prune-retention-and-integrated-parity-proof/162-NATIVE-EVIDENCE.md) records the actual full-contract scope and preserved failed attempts.

The GitHub read-only recheck also confirmed [CI 38206596321](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/38206596321) and [Bright Builds 38206596267](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/38206596267) both `completed`/`success` at exact head `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`. That historical remote success is distinct from the uncommitted Phase 163 tree.

Automated native/CI uncertainty is superseded to the extent above. Human confirmation was not performed. The original debugger's human-confirmation archival condition remains unsatisfied; this record remains `awaiting_human_verify` and is neither moved nor deleted. Current archive-gap readiness still requires Phase 163's independent/full-native/formal/re-audit gates.

## Logical-control source revalidation | 2026-10-13T17:42:27Z

Earlier current-focus statement retained as dated e0 history: next_action: Preserve this unarchived session under the original debugger human-confirmation condition. Corrected21-input Phase163 now has actual full6 native acceptance (38c0607f,176stages/exit0) and fresh functional verification05:36:20Z. Actualrenewedaudit/readiness/finalSC1–SC5nowpassed; normal-hook saving remains separate. Historical receipts are preserved; no human confirmation or debug-session archival has occurred.

Actual source repair replaces one active physical requirements assertion with four explicit logical availability/refusal cases and refreshes its frozen hash. Production selectors, claims and runtime proof remain unchanged. Original e0 reports are preserved in `.planning/reports/phase163-e0c-before-logical-control-revalidation/`; no failed or interrupted run becomes a PASS.

## Current audit boundary | 2026-10-13T19:09:31Z

The actual corrected-source audit passed22/22 at19:07:36Z after functional activation and independent integration31/31seams/18/18flows. Readiness publication followed at19:08:16Z; its actual suite is in progress. This update changes current focus only and does not retime earlier receipts or perform human confirmation, Git saving or archival.

## Final corrected-source acceptance | 2026-10-13T19:19:24Z

Fresh final SC1–SC5 verification passed on the unchanged source after actual native, matrix, activation, audit and readiness execution. Canonical verification SHA256 2d2531cf866871d78bb42f7570970d4715e71c84a49f44e115f8c21464f0472b. Root reconciled all22 unique Complete owners,5/5phaseplans,90/90milestoneplans and10/10phases. Original historical observations and the separate human-confirmation condition remain unchanged. Normal-hook source saving and actual archival are separate transactions.
