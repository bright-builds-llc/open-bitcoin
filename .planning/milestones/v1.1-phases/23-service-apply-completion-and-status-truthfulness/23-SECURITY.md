---
phase: 23
slug: service-apply-completion-and-status-truthfulness
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-05
---

# Phase 23 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| CLI to host service manager | Operator commands transition from Open Bitcoin CLI logic into `launchctl` or `systemctl --user` side effects. | Service lifecycle intents, generated service file paths, manager command arguments |
| Service file generation to local filesystem | Install or uninstall writes and removes launchd plist or systemd unit artifacts in user-owned directories. | Generated service definitions, optional config path, optional log path |
| Service manager status to operator surfaces | Manager-reported state is projected into CLI status and dashboard surfaces. | Installed, enabled, running, diagnostics, log-path availability |
| Test harness to service adapters | Repo verification exercises service logic through fake managers and tempdir fixtures instead of the real machine manager state. | Synthetic service snapshots, dry-run command previews, parser inputs |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-23-01 | Tampering / Integrity | launchd and systemd install apply path | mitigate | Shared preview-command helpers keep dry-run output aligned with the real apply sequence, and focused dry-run tests assert the exact launchd and systemd commands. | closed |
| T-23-02 | Integrity | service status projection | mitigate | `ServiceStateSnapshot::maybe_enabled` carries manager evidence so CLI and dashboard surfaces stop inferring enabled state from `ServiceLifecycleState` alone. | closed |
| T-23-03 | Elevation / Integrity | dashboard service actions | mitigate | Dashboard service actions stay confirmation-gated and reuse `execute_service_command()` instead of introducing a dashboard-only side-effect path. | closed |
| T-23-04 | Safety / Denial of Service | local verification path | mitigate | Service verification remains hermetic through tempdir dry-run tests, parser coverage, and fake-manager status tests without mutating the developer machine service manager. | closed |
| T-23-05 | Integrity / Availability | install apply atomicity | accept | Apply mode can still leave a written plist or unit on disk if manager registration fails after the file write; the risk is documented and accepted for this repair phase. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-23-01 | T-23-05 | Phase 23 repaired sequencing and truthfulness but did not make service install fully atomic. A failed post-write `launchctl` or `systemctl` step can leave a partial local artifact; diagnostics and operator visibility are considered sufficient for this phase, and a full rollback strategy is deferred. | Codex via `/gsd-secure-phase 23` | 2026-05-05 |

*Accepted risks do not resurface in future audit runs.*

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-05 | 5 | 5 | 0 | Codex / `gsd-secure-phase` |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-05
