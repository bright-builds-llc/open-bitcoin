---
phase: 167-candidate-history-validation-rules
plan: "50"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T04:43:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 50 diagnosed default Cargo target stall

The D1 named behavioral RED and GREEN completed on the default target. During the subsequent D4 test-only RED application, the owned Cargo process waited for rustc while two spaced samples of rustc, at about 2:21 and 3:09 elapsed, showed `rustc_session::search_paths::SearchPath::new → std::fs::ReadDir::next → __getdirentries64` on the checkout's `packages/target/debug/deps` search path. The rustc process stayed in macOS `U` state with near-zero CPU. ROOT's separate sample showed the same stack. At that observation point, D4 had no test result; no behavioral RED credit was assigned from the samples.

After preserving two raw samples, process identity/argv, elapsed/CPU/state, source precheck and the actual async nine-key probe, the executor may terminate **only its owned** stalled rustc/Cargo application and retain its raw streams and nonzero setup exit. No unrelated Bazel, Cargo, rustc or host work may be killed or overridden. ROOT must then verify all owned and protected source/native bytes remain unchanged and issue a separate exact isolated-target release before any retry. A quiet command or estimate alone is never this condition.

The release may change **only `CARGO_TARGET_DIR`** from the immutable default nine-key projection to a canonical physical, checkout-owned ignored directory under `.local/open-bitcoin-dev/phase167/plan50/`. The other eight keys (`PATH`, `TMPDIR`, `RUST_TEST_NOCAPTURE`, `CARGO_BUILD_TARGET`, `OPEN_BITCOIN_DEV_STATE_DIR`, `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTUP_TOOLCHAIN`) remain byte-for-byte identical, including null values. Every retry and later Plan 50 command must use the same explicit immutable projection in a real asynchronous `Bun.spawn` child proof immediately before the literal command, with the natural host-global process guard and command-timings serial lock. Do not change command arguments, features, cache, state directory, flags or toolchain. Poll owned queued/running commands within 60 seconds; no pass-only retry or zero-discovery credit.

## Natural completion and dormant exception

Before any termination or target release, the same owned default-target D4 command naturally resumed and completed. Its asynchronous nine-key probe passed, the source pre/post hashes matched, and one compiled named test failed at the expected unknown-valid-header outcome: `missing-canonical-compact-context` (exit 101, 0 passed/1 failed, 2,376 filtered). This is a **genuine D4 preimplementation behavioral RED**. No process was terminated, no target/cache/state key changed, and no isolated target was released. The conditional recovery process above is dormant and supplies no runtime credit or permission to switch targets for this completed command.

D1's genuine RED/GREEN remains separate. Continue Plan 50 on the original default target and exact nine-key process; use a new checked decision only if a future application develops a persistent confirmed stall. Final Plan 50 commands must run fresh on one frozen final causal source/native frame. This observation does not extend to Plan 51, grant an unowned source edit, permit Git action or enter Phase 168.
