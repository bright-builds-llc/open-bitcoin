---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-11-05T03:14:10Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 Bazel host-tool build correction

The first normal commit hook reached Phase 164 with 1,253/1,255 tests passing and two copied Phase 165 positive controls failing on the newly completed parity-catalog rationale. ROOT narrowed one `docs/parity/index.json` sentence without changing the bounded product claim; both exact controls, direct Phase 164/165/167 checkers and the entire 1,255-test Phase 164 file then passed. The second normal hook passed those controls, nine original 16-thread stress selectors and the complete native workspace, but its six-target Bazel build failed on freshly linked host proc-macro dylibs. The retained hook log SHA-256 is `c0d726d81a76c6bc116f16e25df230ab05fe440b7b0dbb79b0a65f14b92b4277` under `.local/open-bitcoin-dev/phase167/plan20/hook-v2.log`.

`bazel clean`, `bazel clean --expunge` and a fresh six-target build at 14 GiB free reproduced the identical `serde_derive` malformed Mach-O LINKEDIT string-pool offset, with consequent missing `thiserror_impl` and `zerofrom_derive` crate errors. Reclaiming two old Phase 167 scratch Cargo targets through scoped `cargo clean --target-dir` raised free space to 27 GiB; another expunged default build reproduced the same failure. Cargo's proc-macro dylib loads and has an aligned string table. Bazel action query shows its default execution configuration uses host `opt`, `strip=debuginfo` and `-Wl,-dead_strip`, while a directly built host `fastbuild` dylib loads. A six-target build with `--host_compilation_mode=fastbuild` passed all six targets in 20.505 seconds on unchanged source; its retained log SHA-256 is `aa5ca83841dab061fe982743bdcd43632fb83783422dad73c579821e8a1818bc`.

ROOT adds exactly one Plan 20 Task 1 owner, `.bazelrc`, original SHA-256 `e376db69afff7b03cdb61e5403994e4f9096451bafc84f40016426c38a0d514`, with the single line `build --host_compilation_mode=fastbuild`. This changes only Bazel host-tool compilation mode; target build mode, six smoke targets, provenance and verification gates remain unchanged. No Cargo profile, selected package, source rule, security refusal, test deadline or coverage gate changes. The initial v14 full default run remains valid evidence for the earlier source frame, not for this revised build configuration.

Before saving, require the literal default six-target `bazel build //:core //:node //:rpc //:cli //:test_harness //:bench` and `bun run scripts/check-bazel-build-provenance.ts` to pass with the new `.bazelrc`, plus affected current guards, independent review of the one-line build change, refreshed Plan 20 summary and Phase 167 reports/lifecycle, and a complete normal hook on staged bytes. Retain failed and diagnostic logs without promoting them to final credit. If the default build still fails, return to diagnosis; no hook bypass, partial acceptance, Git save or Phase 168 credit is granted here.
