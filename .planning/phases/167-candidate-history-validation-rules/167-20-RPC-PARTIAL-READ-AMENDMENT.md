---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-11-01T17:57:42Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 inbound partial-frame test read correction

After the checked EOF readiness correction, its exact test passed three independent focused runs and the full RPC library run reported that same EOF test `ok`. The full RPC library at ordinary ad-hoc concurrency then failed one different test, `phase160_inbound_transport_partial_second_frame_keeps_only_complete_prefix`, while 425 tests passed and one existing test was ignored. The actual failure was `io::ErrorKind::WouldBlock` at `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/failures.rs:99`: a single `client.readable().await` followed by one nonblocking `try_read` expected all 13 partial-frame bytes. Tokio readiness may be a false positive and TCP may split those bytes. The first complete frame, transport progress and network state assertions had already passed. This full RPC run is failed evidence and no default/Bazel/coverage/phase credit follows from it.

ROOT adds exactly one test-only Task1 owner, `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/failures.rs` (535/628 lines, existing parity breadcrumb), after capturing its exact whole original and the current 41-owner source frame. ROOT must issue a supplementary release before the worker edits it. Plan20 then has 42 direct owners (Task1 39, Task2 three) and 1,791 protected original product paths. In the shared `partial_failure` helper, change only the failed one-shot read: put a `readable().await`/`try_read(&mut partial[received..])` accumulation loop under a **fresh two-second timeout**. Start at zero, retry only `WouldBlock`, add each positive read to `received`, stop at exactly the existing nonempty 13-byte buffer length, and retain the explicit count assertion. EOF before 13, another I/O error or timeout must fail visibly; the remaining slice must never be empty when read. Tokio's `AsyncReadExt::read_exact` is unavailable because this crate does not enable `io-util`, so do not add that feature, import or dependency. Preserve both callers (partial first frame and partial second frame), the complete-first-frame decode, byte/message progress and transport-state assertions, forced `ConnectionReset`, fixture cleanup and all production writer/executor code. The separate socket-read site in this file that intentionally accepts an error/EOF remains unchanged. Do not increase the existing two-second response or EOF budget elsewhere.

No frozen Phase160/164/165 source-digest row for this test file was found; retain those guards' current source bytes and run their affected direct/tests after the edit. Independently review the exact one-file diff and 42-owner frame before tests. Run both named partial-frame cases repeatedly with nonzero discovery, full RPC library at ordinary ad-hoc concurrency and under the repo-native scoped four-thread setting, Phase160/164/165 guards, formatting, strict Clippy and all-target build. Then regenerate provisional worktree LOC, fresh coverage clean, and rerun the complete default verifier on one frozen source. Any real `UnexpectedEof`, other I/O error, timeout, wrong frame/progress or extra response remains a blocking failure requiring its own diagnosis; do not treat it as readiness noise.

The nine tracked 16-thread stress families, every native test, strict `<30s` physical bound, worker30s deadline, zero-uncovered pure-core coverage, Bazel, independent source/security/goal reports, lifecycle, ROOT staged-index LOC and normal-hook commit/push remain mandatory. Stop before Phase168.
