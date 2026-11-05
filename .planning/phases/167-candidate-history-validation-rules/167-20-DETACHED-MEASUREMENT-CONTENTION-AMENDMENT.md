---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-11-01T03:08:43Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 detached measurement contention boundary

The tenth complete default verifier attempt kept the 32-owner source frame unchanged, passed all preceding guards, strict Clippy, the all-target build, and the CLI library's 434 tests. Its node library finished with 2,404 pass, four fail, and four ignored. All four failures occurred at the unchanged test-only helper's strict `<30s` assertion for actual detached copy/recovery: synthetic state 30.378079917s, actual 301 disabled 34.218462667s, actual 301 enabled 33.168408708s, and preferred replacement 30.439426583s. The four tests overlapped in the default node run. Later 301 BASIC disabled/enabled durable-row tests passed after that contention. The identical source passed the standalone full node suite and focused measurements near 10s. The failed default run remains failed evidence; no timeout, source, or verifier setting changed in that run.

This amendment narrowly supersedes the earlier instruction in `167-20-DETACHED-COPY-RECOVERY-MEASUREMENT-AMENDMENT.md` to **not serialize tests**. Only the already-owned test helper `measure_detached_rows_for_test` in `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/durable_tests.rs` may acquire one process-local test-only mutex before lease acquisition, physical manifest capture, and the copy/recovery timer, holding it through final source-manifest verification. Recover a poisoned mutex so an assertion failure does not create unrelated lock failures. This serializes competing detached *measurements within the node test process*, while the default Cargo test command, test thread count, every test case, actual copy/recovery timer, both strict `<30s` assertions, and the production worker's independent 30-second deadline remain unchanged. Lock wait earns no timing credit and is disclosed separately from the measured operation. Existing concurrent inspection behavior remains covered by its own tests.

No production inspection algorithm, worker scheduling, fixture size, copy target, source inventory, caps, tamper probes, fallback, deadline, target/cache/flag/toolchain setting, or historical evidence may change. The helper remains the sole Rust edit owner; all other accepted product bytes stay protected. Before the edit, capture the exact helper bytes and the current 32-owner source frame, obtain ROOT release of this checked scope, and have independent source review inspect the delta. Keep the file within its 628-line managed limit.

Run the four exact previously failed cases, the relevant concurrent selector, and a full standalone node suite with nonzero discovered tests. Then regenerate provisional worktree LOC, clean coverage through the normal cooperative lock, and run complete default `bash scripts/verify.sh` on the frozen source. If any serialized actual copy/recovery still exceeds 30 seconds, stop and investigate physical copy or host contention rather than raising the bound. Preserve the raw v10 failure, all retries, source before/after, measured elapsed values, and liveness evidence. Plan20 final native/coverage/Bazel acceptance, independent source/security/goal reports, lifecycle validation, ROOT index-LOC reconciliation, normal hooks, commit and push remain mandatory. No Phase168 work is granted.
