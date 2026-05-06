---
phase: 25
slug: migration-source-selection-hardening
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-06
---

# Phase 25 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Operator CLI to local filesystem detection | `open-bitcoin migrate plan` reads local source-install evidence, including explicit custom source datadirs outside default home roots. | Source datadir paths, `bitcoin.conf` path, `.cookie` path, wallet candidate paths |
| Migration planner to operator output | Detection evidence is rendered into a dry-run migration plan for human review. | Review paths, migration actions, backup requirements, intentional-difference notices |
| Source install to Open Bitcoin target | A future migration may use source evidence to prepare a separate target datadir and config path. | Source install metadata, target datadir/config/log/metrics paths |
| Service detection to cutover review | Detected launchd or systemd definitions may or may not belong to the selected source install. | Service file paths, supervision hints, manual review guidance |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-25-01 | Tampering / Integrity | migration-scoped source detection | mitigate | `detect_migration_installations()` augments detection roots with explicit `--source-datadir` only for the migration command, while status, onboarding, service, dashboard, sync, config, and wallet commands keep the shared default detector path. | closed |
| T-25-02 | Spoofing / Integrity | explicit source selection | mitigate | `supports_explicit_source_selection()` requires source config, cookie, or wallet evidence before selecting an explicit datadir; unsupported explicit paths return manual-review guidance instead of being treated as valid source installs. | closed |
| T-25-03 | Information Disclosure / Tampering | migration plan rendering and source files | mitigate | The operator-binary regression verifies the dry-run plan includes review paths without printing cookie or wallet contents, and asserts source `bitcoin.conf`, `.cookie`, and wallet bytes remain unchanged after planning. | closed |
| T-25-04 | Integrity / Safety | source service review | mitigate | When detected service definitions cannot be confidently tied to the selected source install, the planner keeps service cutover as an explicit manual review step instead of surfacing an unrelated service path as concrete cutover evidence. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Evidence Reviewed

| Evidence | Security relevance |
|----------|--------------------|
| `packages/open-bitcoin-cli/src/operator/runtime.rs` | Migration-only explicit source datadir root augmentation is isolated from other operator commands. |
| `packages/open-bitcoin-cli/src/operator/migration/planning.rs` | Source selection remains evidence-driven, dry-run-only, source-datadir mutation stays out of scope, and ambiguous service associations remain manual review. |
| `packages/open-bitcoin-cli/tests/operator_binary.rs` | Custom source datadir regression covers dry-run output, secret non-disclosure, source non-mutation, and ambiguous service review. |
| `packages/open-bitcoin-cli/src/operator/migration/tests.rs` | Unit coverage proves bare explicit source paths stay manual review when config, cookie, and wallet evidence are absent. |
| `.planning/milestones/v1.1-phases/25-migration-source-selection-hardening/25-VERIFICATION.md` | Phase verification records explicit custom-path selection, conservative fallback, dry-run safety, and repo-native verification evidence. |

---

## Accepted Risks Log

No accepted risks.

*Accepted risks do not resurface in future audit runs.*

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-06 | 4 | 4 | 0 | Codex / `gsd-secure-phase` |

Notes:

- Phase 25 plans did not include a formal `<threat_model>` block, and the summaries did not include `## Threat Flags`.
- This audit derived the register from the phase's security-relevant plan truths, shipped code, tests, and verification report.

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-06
