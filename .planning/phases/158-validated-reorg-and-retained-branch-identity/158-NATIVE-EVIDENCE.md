---
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-09T04:22:09.129Z"
default_native_status: passed
requirements-addressed: [CFIX-03]
requirements-completed: []
---

# Phase 158 Native Verification Evidence

The complete default `bash scripts/verify.sh` passed with **exit 0** on the frozen
worktree. It started **2026-10-09T03:59:49Z** and finished
**2026-10-09T04:19:15Z**. The verifier reported **19m24.571s (1,164,571 ms)**;
the outer time command reported 1,165.26 seconds. No fast mode, source exception,
timeout increase, test suppression or competing Cargo job was used.

## Reproduction and actual versions

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bash scripts/verify.sh
```

The verifier invokes its existing `verify-full` cooperative timing/lock runner.
Actual version probes after completion also used that runner for Cargo/Bazel:

| Tool | Observed version |
| --- | --- |
| rustc | 1.94.1 (e408947bf 2026-03-25) |
| Cargo | 1.94.1 (29ea6fb6a 2026-03-24) |
| Bun | 1.3.9 |
| Bazel | 8.6.0 |
| cargo-llvm-cov | 0.8.5 |

Baseline HEAD remains `2f21ac2052208c7e7a084ed006d908c5ccb714aa`; the feature
worktree was deliberately uncommitted under root's Git barrier. This is evidence
for the frozen worktree, not a nonexistent feature commit. The final checker,
checker test and Phase 103 harness SHA-256 prefixes are respectively
`e9b8ae3ddc7a`, `05d98dcd2abf` and `05991df40afc`. Root owns the complete
source-review hash inventory.

## Passed native gates

| Gate | Actual evidence |
| --- | --- |
| Hook/LOC, provenance, dependency and structural guards | Passed; required tracked LOC refreshed to 389,085 counted lines |
| Phase 103 amended historical checker | 14 passed / 0 failed; same 16 assertions, fresh fixtures, unchanged 5,000 ms timeout |
| Phase 157 current-route checker | 600 mutation tests passed / 0 failed; live checker passed |
| Phase 158 source/claim checker | 449 mutation tests passed / 0 failed; live checker passed |
| Rust formatting | Workspace `cargo fmt --all --check` passed |
| Strict Rust lint | Workspace all-target/all-feature Clippy with `-D warnings` passed |
| Build | Workspace all-target/all-feature build passed |
| Normal Cargo tests | 40 successful unit/integration/doctest result summaries: 3,735 passed / 0 failed / 3 ignored; excludes coverage rerun |
| Node library within that total | 1,353 passed / 0 failed / 3 ignored, 553.51 seconds; includes WR-158-01 consecutive shorter-unflushed return |
| Benchmark list/smoke/report | Passed; actual `smoke:1`, 11 groups / 15 cases; threshold-free evidence |
| Bazel | Six targets `//:core //:node //:rpc //:cli //:test_harness //:bench` built; build provenance checker passed |
| Pure-core coverage | Native `cargo llvm-cov clean` and coverage gate passed; no `Uncovered Lines:` section |

Coverage uses the eight crates in `scripts/pure-core-crates.txt`:
chainstate, network, core, primitives, codec, consensus, mempool and wallet.
The native helper writes its textual report to a temporary file and removes it
on exit; no persistent numeric percentage or adapter-wide coverage claim is
invented. The passed criterion is the actual native no-uncovered-lines gate.

The three normal-test exclusions are the explicitly opt-in public-network smoke,
Phase 157 timing experiment and Phase 158 timing experiment. No required behavior
test is ignored. Phase 158's separate experiment was already explicitly executed
in Plan 06: one passed / zero failed or ignored, 54 configurations / 149 turns.
Those preserved measurements are separate from the default verifier.

## Receipts and earlier failed attempts

| Attempt | Actual outcome and receipt |
| --- | --- |
| First default attempt, 03:55:59Z | Exit 1 after 205 ms: stale tracked LOC report; no Rust step ran. `/tmp/phase158-native-loc-stale.log` |
| Second default attempt, 03:56:15Z | Exit 1 after 34.896 seconds: two historical four-case Phase 103 mutation groups exceeded 5,000 ms. `/tmp/phase158-native-phase103-timeouts.log` |
| Isolated Phase 103 diagnosis | Same two timeouts reproduced, 6 passed / 2 failed, 20.52 seconds. `/tmp/phase158-phase103-diagnostic.log` |
| Root-authorized harness amendment | Split two groups into eight independent tests, preserving all mutations/assertions and six other tests. 14 passed / 0 failed, 19.58 seconds; live checker passed. `/tmp/phase158-phase103-split-green.log` |
| Final complete default attempt | Exit 0 and completion footer. `/tmp/phase158-native.log`; `/tmp/phase158-native.exit` contains `0` |

The source-only Phase 103 delta received independent review before the retry.
Required LOC generation used the exact repo command; no handwritten report edits
or historical production checker changes were made. A quiet panic-site scan was
identified as the owned `find` subprocess with advancing CPU work and completed
normally. Long-running node/daemon tests likewise completed normally. There was
no task termination, host security/cache change or foreign process control.

Benchmark artifacts are
`packages/target/benchmark-reports/open-bitcoin-bench-smoke.json` and its
Markdown companion. The native log records Bazel six-target success and
`Bazel build provenance check passed.` Temporary logs are supplementary;
this document preserves the decisive outcomes and limits.

## Scope and remaining root gates

[Current measurements](158-REORG-MEASUREMENTS.md), [UAT](158-UAT.md) and the
[parity page](../../../docs/parity/v2-5-validated-reorg.md) bound the evidence:
internal genuine validated reorg, immutable displaced lookup, physical-suffix
configured recovery, required-input refusal and conservative own-flush/prune
authority. Existing finite caps can refuse fully retained forks before effects.
Compact maturity-one fixtures and the separate default-maturity-100 case do not
establish full regtest parameter parity. Stage-inclusive times and reservations
do not establish isolated publication latency, whole-runtime constant memory,
RSS, archive scale, hardware crash resilience or production-funds safety.

Root owns final security mitigation closure, formal phase/lifecycle verification
and CFIX-03 activation. Native success alone does not complete those gates.
RPC 159, peers 160, operator projections 161 and integrated retained-client proof
162 remain pending/nonshipped. Git staging, commits, push and root state updates
remain deferred.


## Root closure | 2026-10-09T04:29:44.988632Z

All seven plans are complete. [Formal verification](158-VERIFICATION.md) passed 21/21 truths and the same-attempt lifecycle gate; [security](158-SECURITY.md) closed all 25 mitigations. CFIX-03 is Complete in canonical requirements and parity records. The earlier pending-root statements describe the execution handoff. Actual mandatory commit-hook, commit and push outcomes are derived from the saving commit and upstream refs, avoiding a self-referential stored commit hash. Phases 159–162 remain pending.
