# Deferred Items

## 91-06-local-cli-cargo-rustc-stall | 2026-06-25

- **Found during:** Plan 91-06 verification
- **Issue:** Local `open-bitcoin-cli` Cargo verification stalls in idle `rustc` metadata compilation before diagnostics. This is broader than the already-known executable test-binary launch hang.
- **Observed commands:** `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features`, guarded non-incremental check, guarded build, clippy, no-run test compile, and focused test filters.
- **Disposition:** Deferred as a local toolchain/verification blocker. No cargo or rustc processes were left running after bounded attempts.
