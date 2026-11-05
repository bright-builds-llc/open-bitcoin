---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-11-01T17:18:48Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 inbound RPC EOF readiness test correction

The first complete default verifier on the controlled 39-owner source passed every pinned Bun guard, all nine tracked 16-thread native stress selectors, formatting, strict Clippy, all-target build, and the scoped four-thread workspace node library (2,410 pass, zero fail, four existing ignored among 2,414 discovered), plus bench/chainstate/CLI/network and other native groups. The RPC library then failed exactly one of 427 discovered tests: `out_of_window_notfound_closes_the_inbound_socket` asserted `matches!(read, Ok(0))` at `packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs:443` after receiving a complete `NotFound` response. Its test calls `TcpStream::readable().await` once and `try_read` once; the actual read result was not logged. Tokio's documented readiness can be a false positive, after which `try_read` returns `WouldBlock`. The full run exited nonzero and did not reach benchmark/Bazel/coverage; no phase acceptance follows. Source before/after was unchanged.

ROOT adds exactly two test/guard Task1 owners: `packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs` (445/628 lines) and `scripts/check-phase164-canonical-identity/source-files.ts` (627/628 lines). This expands Plan20 to 41 direct owners (Task1 38, Task2 three) and leaves 1,792 original product paths protected. Capture exact original/current whole bodies and the 39-owner frame; ROOT must issue a supplementary two-owner release before the worker edits either path. In the single RPC test, replace only the one-shot `readable`/`try_read` pair with a fresh two-second `tokio::time::timeout` around a loop that awaits readability, retries **only** `std::io::ErrorKind::WouldBlock`, and otherwise returns the actual read result. Keep the buffer nonempty. The assertion must still require `Ok(0)` and include the actual result on failure: any `Ok(n>0)`, other I/O error, or elapsed timeout remains a visible failure. Preserve the complete `NotFound` response assertion, handshake, request, worker shutdown and production socket code. This removes a false-positive readiness assumption without extending the existing two-second response budget or weakening the EOF behavior check.

The Phase164 source-files guard pins this exact test function body. Renew only that one current digest row after independently reviewing the test diff; do not change its frozen prior/source-edge meaning or add lines to its 627-line file. Keep all other Phase164/165/167 guards and historical snapshots unchanged. Run the exact RPC test repeatedly with nonzero discovery, the full RPC library test suite, Phase164/165 current/historical guards and mutation tests, and then the full default verifier on the newly frozen source. A focused pass does not transfer to the default run. Retain the original nonzero verifier log, the actual new read-result diagnostics if a failure recurs, source before/after and all test outcomes. If the failure recurs as `Ok(n>0)`, another I/O error or timeout, diagnose that concrete behavior rather than treating it as readiness noise.

The existing nine 16-thread stress commands, scoped four-thread native tests, strict `<30s` physical check, worker30s deadline, fresh zero-uncovered pure-core coverage, Bazel, independent source/security/goal reports, lifecycle, ROOT staged-index LOC and normal-hook commit/push remain mandatory. Do not stage, commit, push or begin Phase168 from this bounded correction alone.
