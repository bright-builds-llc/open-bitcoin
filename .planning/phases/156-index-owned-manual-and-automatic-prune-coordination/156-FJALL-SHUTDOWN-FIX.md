---
phase: 156-index-owned-manual-and-automatic-prune-coordination
generated_by: gsd-execute-phase
phase_lifecycle_id: 156-2026-10-04T20-28-36
status: focused-repair-verified
git_finalization: pending consolidated root commit after full native verification
---

# Fjall shutdown dependency repair

## Failure and decision

The inherited filter-index sweep completed 101 of 102 tests before its remaining `filter_index_before_protection_insertion_keeps_checkpoint_and_protection` test blocked. That interrupted sweep is not a pass. The root-owned stack sample `/tmp/open-bitcoin-phase15607-node-sample.txt` records `checkpoint_fault_reopen` at `faults.rs:315`, dropping the actual store, inside Fjall 3.1.4 `DatabaseInner::drop` and a blocking Flume `Sender::send`. No worker threads remained in the sampled process. This failure occurred during backend shutdown, before the independent production reopen and persisted-state assertions.

Pin the existing dependency to official [Fjall PR 321](https://github.com/fjall-rs/fjall/pull/321) head [`aa30dca811399a201e0b9595da93a4582dcb2b57`](https://github.com/fjall-rs/fjall/commit/aa30dca811399a201e0b9595da93a4582dcb2b57). The fetched checkout reports that exact full HEAD. Against released 3.1.10 base `3adaa50261c9be58484971dc561cdd563156765e`, its entire diff is `src/db.rs` (+5/-11) and `src/worker_pool.rs` (+43/-12): shutdown delegates to a nonblocking Close/worker-handle join routine, together with comments and an active-thread-count test. `Cargo.toml` is unchanged by the PR.

The root's primary-source inspection found the blocking shutdown loop still present in published 3.1.10–3.1.12 tarballs; a release-number-only bump would not select this fix. This snapshot identifies itself as 3.1.10 and does not contain the later 3.1.11 fsync or 3.1.12 deletion fixes. The previous 3.1.4 baseline also lacks those later fixes. This repair must not be described as adopting all latest upstream fixes.

## Checkable repair plan

- [x] Load repo guidance, Rust/testing/verification standards, active lessons and Phase 156 context; enter the existing GSD execution context.
- [x] Change only the Fjall source pin and resolve the existing lockfile through the timing wrapper.
- [x] Verify fetched SHA, two-file upstream diff, manifest and database format markers.
- [x] Repeatedly complete the exact previously blocked test and neighboring checkpoint/commit fault reopen tests in independent Cargo invocations.
- [x] Complete the full `filter_index` node suite with default parallelism against the pin.
- [x] Review dependency provenance/owned diff, record limits and release Cargo ownership to root for the native/Bazel gate.

## Scope and compatibility evidence

Owned changes are `packages/open-bitcoin-node/Cargo.toml`, `packages/Cargo.lock`, this decision/evidence artifact, and `MODULE.bazel.lock` only if repo-native generation requires it. No adapter source changes, backend leaks, disabled workers, durability reductions or dependency-cache edits are part of this repair. Root owns phase tracking, source checks, final native verification, commits and pushes.

The original Fjall dependency name and `default-features = false` remain. The pinned package retains Rust edition 2021 and MSRV 1.90.0, within the repo's probed Rust 1.94.1. Its declared `lsm-tree ~3.1.10` resolves to 3.1.10. Bun was explicitly selected and probed as 1.3.9. `src/version.rs` is byte-identical to published Fjall 3.1.4; both versions' database open/create paths accept/write `FormatVersion::V3`. The `lsm-tree` 3.1.10 and 3.1.4 `src/format_version.rs` files are also byte-identical. These are format-marker compatibility observations, not an exhaustive migration or hardware-durability proof.

## Verification record

All ad-hoc Cargo/Bazel commands use `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`, with pinned Bun on PATH, one Cargo invocation at a time, and polling within 60 seconds. The initial exact regression invocation waited on the artifact lock held by the editor's Rust Analyzer Cargo check; this host/tool coordination delay is separate from the inherited Fjall shutdown deadlock.

Successful lock refresh: `phase156-fjall-shutdown-lock -- cargo update --manifest-path packages/Cargo.toml -p fjall`. It selected Fjall 3.1.10 from the exact Git revision and registry `lsm-tree` 3.1.10. Cargo also initially reselected broad-range `tempfile`/`errno`/`rustix` references to existing older `getrandom`/`windows-sys` packages. Those unrelated choices were restored to their previous compatible locked references; the subsequent `--locked` invocation accepted the minimized lock. A TOML package comparison confirms the only removed entries are Fjall/lsm-tree 3.1.4, the only added entries are Fjall/lsm-tree 3.1.10, and every shared package entry is identical. No unrelated package version, checksum or dependency edge changed in the final lock.

Initial exact regression: `phase156-fjall-before-protection-reopen -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::filters::tests::faults::filter_index_before_protection_insertion_keeps_checkpoint_and_protection -- --exact` passed 1/1 (1,122 filtered out), with real persisted-state assertions completing in 0.73 seconds. Its newly linked binary first waited before main; `/tmp/open-bitcoin-phase156-fjall-first-launch-sample.txt` shows only `_dyld_start`. That first-launch delay resolved without intervention.

The repeat loop on the minimized lock required a rebuild. `/tmp/open-bitcoin-phase156-fjall-rebuild-sample.txt` records compiler `SearchPath::new` directory enumeration (`ReadDir`/`__getdirentries64`), with no test/Fjall frames. The later `/tmp/open-bitcoin-phase156-fjall-rebuild-second-sample.txt` records progression into proc-macro dynamic loading. These host/cache boundaries are separate from the inherited database deadlock. They resolved without interruption, duplicate builds, weakened verification or cache changes.

### Completed Cargo regression gates

Each command below uses `cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::filters::tests::faults::<test_name> -- --exact`, through `command-timings.ts` with stable key `phase156-fjall-reopen-<test_name>`. A fail-fast Bash loop ran ten iterations of all three commands, sequentially. Every invocation ran its own test process and completed 1/1 with actual close/reopen assertions (1,122 filtered out). JSON timing records under `.local/open-bitcoin-dev/command-timings/` independently confirm ten successful zero exits per key.

| Test name                                                                  | Completed invocations | Result       |
| -------------------------------------------------------------------------- | --------------------- | ------------ |
| `filter_index_before_protection_insertion_keeps_checkpoint_and_protection` | 10                    | 10/10 passed |
| `filter_index_before_checkpoint_commit_keeps_checkpoint_and_protection`    | 10                    | 10/10 passed |
| `filter_index_after_successful_commit_error_reopens_complete_new_state`    | 10                    | 10/10 passed |

The tests themselves completed in 0.61–0.91 seconds. The initial minimized-lock rebuild and host waits are included in the first invocation's overall timing rather than hidden as test time. This adds 30 completed regression invocations to the initial exact 1/1 pass above.

Full filter-index suite: `phase156-fjall-filter-index-suite -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index` completed **102 passed, 0 failed, 0 ignored, 0 measured, 1,021 filtered out**, in 52.92 seconds, with default parallelism. The former shutdown test completed inside that sweep. This is a filter-index selection of the node library tests; the whole node/workspace and default native gate remain root-owned.

### Bazel lock and dependency provenance

`phase156-fjall-bazel-lock -- bazel mod deps --lockfile_mode=update` exited zero after its local Bazel 8.6.0 server started. It regenerated `MODULE.bazel.lock` through the existing Bzlmod/crate-universe path. The resulting `crate_index__fjall-3.1.10` uses Bazel's `git_repository`, official remote `https://github.com/fjall-rs/fjall` and exact commit `aa30dca811399a201e0b9595da93a4582dcb2b57`. Its `crate_index__lsm-tree-3.1.10` registry archive SHA256 is `5498808f4d41cf785079d3ef3f28716f1b4364af1b512f3097bfb651bded1b3e`, matching Cargo's checksum. Parsed metadata contains no stale Fjall/lsm-tree 3.1.4 references. This is dependency-resolution evidence; root's native gate must still build all required Bazel targets and check build provenance.

`phase156-fjall-dependency-features -- cargo tree --locked --manifest-path packages/Cargo.toml -p fjall --edges features` exited zero and resolved the exact Git pin and `lsm-tree` 3.1.10. Fjall's disabled default features and existing dependency names remain. The minimized Cargo package-entry comparison was rerun after Bazel resolution and still found only the two intended replacements and no changes to shared package entries.

## Review and handoff

The repair is the simplest scoped change: select the official upstream fix and its required LSM version, preserve all unrelated locked choices, and regenerate the existing Bazel metadata. No first-party source workaround or new production dependency/crate was introduced. The owned manifest/lock/generated-lock diff was reviewed, and `git diff --check` passed. No new network endpoint, authorization path, schema/trust-boundary change or adapter file-access surface was introduced; dependency provenance remains explicitly pinned.

Focused repair verification is complete and Cargo/Bazel cooperative ownership is released to root. No commit, push, hook bypass, state/requirement activation or phase-completion claim was made by this repair agent. Root owns `bash scripts/verify.sh`, all-target/all-feature formatting/lint/build/test and coverage, Bazel smoke builds/provenance, source/parity guards, phase lifecycle review and final Git finalization.

## Evidence limits

Existing fault tests drop all store handles, reopen the actual database and independently assert checkpoint, visible and immutable records, protection and unrelated operator locks. Repeated software-boundary fault tests can show the observed shutdown/reopen regression is resolved and persisted-state safety assertions still hold; they do not establish power-loss, disk-controller or hardware resilience. Repository tests exercise the adapter's use of the backend; the entire upstream test suite is not part of this focused gate.
