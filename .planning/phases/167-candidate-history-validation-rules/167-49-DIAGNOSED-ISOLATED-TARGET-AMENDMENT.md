---
phase: 167-candidate-history-validation-rules
plan: "49"
type: execution-environment-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T00:25:09Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 49 diagnosed Cargo target isolation

The first C2 test-first attempt did not reach compilation or a behavioral test. Two spaced samples of the owned Rust compiler found its main thread blocked in `SearchPath::new` → `ReadDir` → `readdir_r` → `__getdirentries64` while FD 6 enumerated the existing ignored `packages/target/debug/deps` directory (18,186,240 bytes). The [host setup record](../../../.local/open-bitcoin-dev/phase167/plan49/c2-execution-v1/red-host-setup.json) and retained `red-rustc.sample.txt`, `red-rustc-sample2.txt`, `red-rustc-fd2.txt` and `red-processes2.txt` bind the evidence. After the repeated unchanged state, the owned attempt was stopped with exit 143 and all owned child processes reaped. It earns no RED, GREEN, test discovery or normal-target pass credit. The C2 manager source and protected/native frame remained unchanged.

The published Plan 49 contract otherwise forbids target/cache/state/flags/toolchain overrides. For **this diagnosed host-cache condition only**, Root may release an ignored checkout-owned isolated `CARGO_TARGET_DIR` for the remaining Plan 49 C2–C4 and final commands. Change only the `CARGO_TARGET_DIR` entry of the explicit immutable nine-key environment snapshot; retain the original other eight key values, pinned Bun/Rust, canonical physical `TMPDIR`, literal Cargo argv, real asynchronous `Bun.spawn` probe before each application, same snapshot for probe and command, natural host-global process guard and checkout cooperative command-timings lock. Do not delete or quarantine the original cache, change test flags/timeouts/state directory, bypass hooks, overlap Cargo jobs or infer that the default target passed. The isolated target is an environment location, not a new source or acceptance criterion.

Before each retry, record the exact nine-key projection without exposing unrelated environment values, prove the isolated target is ignored and physically owned by this checkout, recheck current source/protected/native hashes, and retain complete raw stdout/stderr, timing, exit, discovered test names and failures. A compiled intended C2 mismatch assertion failure may count as behavioral RED; another compiler/setup failure may not. The original four task and thirteen final Plan 49 commands, all source/fit/inverse/independent-review gates, later Plan 50/51 dependencies, Plan 20 full default verifier, normal-hook commit/push on main, and STOP before Phase 168 remain unchanged. This amendment grants no automatic Plan 50/51 target exception or C2 source acceptance.
