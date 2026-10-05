---
phase: 157-safe-activation-and-scheduled-index-catch-up
status: passed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T23:08:57.691000Z
command: bash scripts/verify.sh
started_at: 2026-10-05T22:49:50.041171+00:00
ended_at: 2026-10-05T23:07:13.806741+00:00
exit_code: 0
wrapper_elapsed_ms: 1043773
verifier_elapsed_ms: 1042546
source_fingerprint: 5b225190f7257027f15c92832181384363d8d4c58ba749ad6a7181141179c4ad
---

# Phase 157 default native verification

The complete default contract passed from the beginning in **17m22.546s** (verifier), **17m23.773s** including its capture wrapper. Rust1.94.1 and Bun1.3.9 were pinned throughout. No fast mode, stage omission, public-network opt-in or pre-commit bypass was used.

| Gate                                                        | Actual result                                                                                                               |
| ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| Full historical/source/Bun mutations, parity and provenance | Passed, including123/128/129/134 routing corrections,135/155/156 and568 Phase157 controls                                   |
| Managed file lengths/architecture/panic/dependency policy   | Passed;1325files scanned,zero managed findings                                                                              |
| Rust formatting and workspace Clippy                        | Passed, all targets/features, warnings denied; timed workspace format write also passed before the contract                 |
| Workspace all-target/all-feature build                      | Passed                                                                                                                      |
| Primary workspace tests/doc-tests                           | 3604passed,0failed,2intentional ignores across39result groups; coverage runs excluded from this count                       |
| Real node suite                                             | 1251passed,0failed,2intentional ignores;463.90s                                                                             |
| Benchmarks                                                  | List/smoke executed and real JSON report validated                                                                          |
| Bazel                                                       | Core/node/RPC/CLI/test-harness/bench smoke build and provenance passed; successful86/84-action outputs                      |
| Pure-core coverage                                          | LLVM-cov ran all listed pure-core crates with show-missing-lines; verifier rejected any Uncovered Lines section, and passed |

The detailed run is captured at /tmp/phase157-native-full-attempt2.log and its structured result at /tmp/phase157-native-full-attempt2-result.json. This tracked record preserves command, timing and substantive results; temporary log availability is not assumed for future contributors. The mandatory normal commit hook will independently rerun the full contract after final content staging. Its actual outcome belongs to Git/command evidence, not a self-referential pre-commit assertion here.

The first attempt stopped before Rust at a stale Phase123 real-source anchor after81.331s wrapper elapsed. Historical guards were corrected without production changes, independently mutation-tested/reviewed, and this complete second attempt passed. Formal goal/lifecycle closure and final Git transport remain separate gates. Software fault/reopen evidence and zero uncovered lines do not establish hardware durability, arbitrary payload support, public-mainnet readiness, deferred serving or funds safety.
