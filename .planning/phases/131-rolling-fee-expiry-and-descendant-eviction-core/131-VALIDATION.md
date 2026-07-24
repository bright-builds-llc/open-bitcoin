---
phase: 131
slug: rolling-fee-expiry-and-descendant-eviction-core
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-07-24
---

# Phase 131 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `#[test]` / Cargo workspace tests (`open-bitcoin-mempool`, `open-bitcoin-node`, `open-bitcoin-rpc`) + `open-bitcoin-bench` |
| **Config file** | `packages/Cargo.toml` workspace; `rust-toolchain.toml` |
| **Quick run command** | `bun run scripts/command-timings.ts run --key mempool-press-quick -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib` |
| **Full suite command** | `bash scripts/verify.sh` |
| **Estimated runtime** | ~30–90s quick; ~25–40m full verify |

---

## Sampling Rate

- **After every task commit:** Run targeted `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib <filter>`
- **After every plan wave:** Run mempool + affected node/rpc package tests
- **Before `/gsd-verify-work`:** `bash scripts/verify.sh` must be green
- **Max feedback latency:** ~90 seconds for quick package tests

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 131-01-* | 01 | 1 | PRESS-01 | T-131-02 | Accounted-capacity trim only | unit | `cargo test -p open-bitcoin-mempool accounted_capacity` | ❌ W0 | ⬜ pending |
| 131-02-* | 02 | 1 | PRESS-02 | T-131-01 | Package bump + descendant remove | unit | `cargo test -p open-bitcoin-mempool pressure_bump` | ❌ W0 | ⬜ pending |
| 131-03-* | 03 | 2 | PRESS-03 | T-131-01 | Block-gated decay only | unit | `cargo test -p open-bitcoin-mempool rolling_fee_decay` | ❌ W0 | ⬜ pending |
| 131-04-* | 04 | 2 | PRESS-04 | T-131-03/04 | Expiry cleanup + injected time | unit | `cargo test -p open-bitcoin-mempool expiry_` | ❌ W0 | ⬜ pending |
| 131-05-* | 05 | 3 | PRESS-05 / evidence | T-131-05 | Oracle + labels + perf | unit/bench | `cargo test -p open-bitcoin-mempool sustained_pressure`; bench case | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs` — PRESS-01/02
- [ ] `packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs` — PRESS-03
- [ ] `packages/open-bitcoin-mempool/src/pool/tests/expiry_cases.rs` — PRESS-04
- [ ] Sustained scenario + oracle assertions — PRESS-05
- [ ] Hermetic perf threshold case — PRESS-05
- [ ] Update fixtures depending on `legacy_vsize_trim_limit` / `capacityenforcement: legacy_vsize` / `RollingFeeParityStatus::Deferred`

*Existing Cargo/Bun verify infrastructure covers the harness; Wave 0 adds the missing PRESS case files.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| — | — | — | All phase behaviors have automated verification. |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 90s for quick runs
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
