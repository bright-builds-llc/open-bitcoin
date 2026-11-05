---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T20:22:55Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 detached copy/recovery measurement boundary

The eighth complete default verifier attempt passed all preceding guards, strict Clippy and the all-target Cargo build, then its node unit suite finished with 2,406 pass, two fail and four existing ignored. Both failed Phase167 BASIC-enabled durable confirmed-counts reopen cases panic at the unchanged test helper's strict `<30s` assertion: `actual isolated copy/recovery elapsed 32.596253125s` and `39.20486825s`. They fail before the caller's second `<30s` assertion, reopen or confirmed-counts equality. The full suite remains failed evidence; no source or timeout was changed in that attempt.

Two separate named focused reruns on the same source passed with helper elapsed `10.181301292s` and `10.085349584s`. A three-case default-parallel focused run passed with helper elapsed `10.202527459s`, `10.166588458s` and `10.224797334s`; those three cases serialize among themselves through their existing scenario lock. Broader node tests can contend during the full suite. The current helper starts its timer before `with_inspection` but stores `elapsed` only after the callback returns. The callback runs after source inventory/copy, isolated worker recovery/reap, source and recovered-tree checks and scratch cleanup. It then performs test-only namespace enumeration, complete-family decode, cloned corruption probes and logical assertions. The current elapsed field therefore includes post-recovery test analysis despite its copy/recovery label. The production worker's independent 30-second deadline remains unchanged.

ROOT adds exactly one present test-only Rust file to Plan20 Task1 ownership: `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/durable_tests.rs`. Plan20 now has **32 unique direct owners**, Task1 **29** and Task2 **three**. Of the 1,828 accepted product-source paths, **1,801 remain protected**; LOC remains a separate generated owner and four reports truly absent. The helper is 623/628 normal lines and already has a parity breadcrumb. Before its edit, close jobs, capture its exact original/live whole body and provisional LOC in a 32-owner stage frame, issue a supplementary ROOT source release and obtain independent review. Existing callers, production inspection/worker code, historical evidence and test deadlines remain protected.

Set the `DurableRowsMeasurement.elapsed` value at the beginning of the `with_inspection` validation callback, after the complete actual copy/recovery/check/cleanup path, and remove the post-callback elapsed overwrite. Preserve both existing `<30s` assertions, the production worker deadline, 4,096-entry/512 MiB/65,536-byte caps, full physical manifest and complete logical/undo/BASIC/count checks, every corruption probe and both failed test names. The resulting field measures actual isolated copy/recovery/cleanup wall-clock, not total test-only enumeration/corruption latency; disclose that limit in Plan20 validation. Do not serialize or skip tests, raise a threshold, shorten the fixture, alter worker scheduling, copy targets or suppress a failing result.

Run both exact previously failing tests and the three-case default-parallel selector on the edited source, recording helper elapsed and all assertions. Then run the relevant full node suite and complete default `bash scripts/verify.sh` on a frozen source after provisional LOC regeneration and fresh locked coverage clean. If copy/recovery alone still reaches 30 seconds in the full suite, stop for a separate checked root-cause diagnosis rather than relaxing the bound. Preserve raw failures/retries, source before/after and liveness. Independent source/security/goal review and ROOT staged-index LOC/hook reconciliation remain mandatory. No other Rust, guard, historical, target/cache/flag/toolchain override, staging, Git action, global CVLD closure or Phase168 work is granted.
