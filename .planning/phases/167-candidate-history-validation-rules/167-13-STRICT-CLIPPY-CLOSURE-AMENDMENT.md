---
phase: 167-candidate-history-validation-rules
plan: "13"
status: proposed
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
source_execution_released: false
---

# Plan 13 strict Clippy closure

The first final-source history command passed seven named tests, including the 301-block reopen case. The next SubmitBlock diagnostic passed eight named tests. The final-source serial runner held after the first command because its raw stderr contained two dead-code warnings, and the exact checked strict Clippy command then exited 101 with five unique lint findings. Neither the seven-test pass nor the serial HOLD accepts Plan 13.

The actual compiler findings are two test-only ordinary-history helpers (`context/mempool_recovery.rs` and `context/wallet_state.rs`), one manual even-length check (`dispatch/submit_block.rs`), and two test-only complex types (`context.rs`). The Root diagnostic is `.local/open-bitcoin-dev/phase167/plan13/consumer13-execution-after-final47-v4-v1/runtime-v5d/strict-clippy-root-hold.json` (SHA-256 `a7f0a1a8be447a616e165c07b4431c9f30d25ebc1ac86be4e836e218d5961cd8`).

The previous 335-source frame omitted the compiled `context/mempool_recovery.rs` file. It remains historical evidence, not the authority for this repair. Fresh before-change capture adds all 103 RPC source files missing from that frame. The expanded frame has 438 source paths (436 present, two true absences), 201 native inputs, and the separate Byteview input. It retains the original eleven Plan 13 owners and promotes `context/wallet_state.rs` from the old protected set and `context/mempool_recovery.rs` from the newly captured set. The repair scope is therefore thirteen owners and 425 protected source paths: 323 from the old frame and 102 newly captured. The original eleven-owner inverse still proves eight whole original bodies and three true absences; the two promoted owners need their own exact whole-body inverse.

The source-only four-file proposal is `.local/open-bitcoin-dev/phase167/plan13/warning-fix-preparation-v4/index.json` (SHA-256 `633554af52bb24bb55917445f2484d7bee849dc3257390ca8601019c5c529256`). Its independent preparation review is `.local/open-bitcoin-dev/phase167/plan13/independent-warning-fix-v4-review-v1/review.json` (SHA-256 `380a68fc5e1e8201b925b097adf67bfa2cce094febecf1e6ba5b7ce85d1f654c`). The earlier V3 proposal is held because it incorrectly classified both added owners as protected.

1. Authenticate the fresh 438/201/1 before-image, the reviewed exact four before/after whole bodies, and unchanged live source. Apply only those four bodies. The two helpers and their imports become test-only; the SubmitBlock hex check uses `is_multiple_of(2)`; named test-only aliases replace the two complex inline types. Preserve the ordinary-history refusal tests and production behavior.
1. Verify exact live after-body hashes, `cargo fmt --all -- --check`, and strict `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings`, running both Cargo commands through `bun run scripts/command-timings.ts run --key <stable-key> -- ...` as repo-local guidance requires. This early Clippy check is diagnostic; only the later checked final-source command receives original-command credit. A compiler failure, warning, or zero-test result earns no pass credit. Repair any new failure before advancing.
1. Capture a fresh complete post-repair 438/201/1 frame. Run all four original Plan 13 commands in checked order against that frame, one Cargo job at a time. Preserve actual exits, named test discovery, complete raw stdout/stderr, warning classification, and before/after source/native projections. The old serial job and its partial final capture stay historical; do not relabel them as new-source passes.
1. Prove the original-eleven scratch inverse and the two promoted whole-body inverses without changing the live checkout. Review the final thirteen-owner source, completed CORE behavior, canonical Summary, and independent document QA before Root FINAL13. Plan 48 remains held until that acceptance.

The source repair, source capture, four-command verification, inverses, and reviews are required work. No Phase 168 work or Git finalization occurs in this amendment. Whole Phase 167, default verification and coverage/Bazel, independent security/goal/lifecycle reviews, and normal-hook main commit/push remain later gates.
