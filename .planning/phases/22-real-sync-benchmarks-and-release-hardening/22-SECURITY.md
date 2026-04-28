---
phase: 22
slug: real-sync-benchmarks-and-release-hardening
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-27
---

# Phase 22 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

This security register was synthesized from the executed Phase 22 plans,
summaries, and shipped implementation because the phase artifacts do not contain
a dedicated `<threat_model>` block.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Benchmark fixtures -> runtime benchmark harness | The Phase 22 benchmark runner accepts only repo-owned scripted transports, local temp storage, and deterministic fixture data for the runtime-backed cases. | Scripted P2P messages, fixture blocks or headers, tempdir paths, durable snapshot data |
| Benchmark smoke report -> verification contract | The generated JSON report crosses from the benchmark binary into the validator and repo-native release-hardening gate. | Benchmark group ids, case ids, profile metadata, measurement metadata |
| Operator docs and parity ledger -> operator or reviewer decisions | Release-facing docs and machine-readable parity records shape what operators and reviewers believe is safe, shipped, or intentionally deferred. | Install workflow claims, service semantics, migration boundaries, verification commands, deferred-surface notes |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-22-01 | Tampering / Spoofing | Runtime benchmark harness | mitigate | `packages/open-bitcoin-bench/src/runtime_fixtures.rs` pins sync fixtures to `Regtest`, loopback peers, and empty DNS seeds; `packages/open-bitcoin-bench/src/cases/sync_runtime.rs` runs against `ScriptedTransport` and scripted messages rather than live sockets; `docs/operator/runtime-guide.md` states that the default `bash scripts/verify.sh` path stays offline and does not require public-network sync. | closed |
| T-22-02 | Tampering | Benchmark evidence validation | mitigate | `scripts/check-benchmark-report.ts` rejects wrong schema, wrong mode, wrong smoke profile, missing required groups or Phase 22 case ids, and invalid durability metadata; `scripts/verify.sh` requires both smoke-report generation and validator success before the release-hardening path passes. | closed |
| T-22-03 | Spoofing / Information Disclosure | Operator docs and parity ledger | mitigate | `docs/operator/runtime-guide.md`, `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`, and `docs/parity/catalog/operator-runtime-release-hardening.md` consistently preserve the source-built-only install claim, dry-run migration boundary, `--apply` gates for destructive service actions, and the deferred status of packaged installs, Windows service support, hosted dashboards, and public-network verification. | closed |
| T-22-04 | Tampering / Denial of Service | Benchmark temp storage fixtures | mitigate | `packages/open-bitcoin-bench/src/runtime_fixtures.rs` builds temp paths from label + PID + nanosecond timestamp + atomic counter and cleans them up on drop; the same helper is reused by the durable runtime benchmark cases; `22-01-SUMMARY.md` records the explicit closeout fix for parallel tempdir collisions. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

No accepted risks.

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-27 | 4 | 4 | 0 | GPT-5.4 + read-only security auditor |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-04-27
