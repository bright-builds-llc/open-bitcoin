---
phase: 18
slug: service-lifecycle-integration
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-03
updated: 2026-05-03
generated_by: codex
lifecycle_mode: yolo
phase_lifecycle_id: 18-2026-04-27T02-01-54
generated_at: 2026-05-03T17:10:26Z
---

# Phase 18 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 18 adds operator-facing service lifecycle support for Open Bitcoin. The
phase introduces launchd and systemd service adapters, dry-run and `--apply`
command dispatch, and live service-state wiring into `open-bitcoin status`.

Security verification was limited to the declared threat registers in
`18-01-PLAN.md`, `18-02-PLAN.md`, and `18-03-PLAN.md`, plus the matching
implementation and targeted tests in `packages/open-bitcoin-cli`. No unrelated
vulnerability scan was performed.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Subprocess output to Rust service adapters | `launchctl`, `systemctl`, and `id -u` outputs are parsed into typed service state and command failures. | Exit codes, stdout, stderr, and UID strings. |
| Operator CLI flags to runtime dispatch | `open-bitcoin service ...` controls whether work stays preview-only or performs writes. | `--apply`, datadir, config path, and selected subcommand. |
| Generated service file content to platform managers | Launchd plist or systemd unit content is derived from runtime paths and then handed to the local service manager. | Binary path, datadir, optional config path, optional log path. |
| Detection roots to service candidate scan | Runtime builds a list of service directories and detect logic inspects those paths without mutating them. | Service directory paths and derived candidate file paths. |
| ServiceManager status output to shared status surface | Adapter status snapshots feed the `Service:` section in `open-bitcoin status`. | Manager name, installed/enabled/running booleans, and operator-visible diagnostics. |
| Runtime-owned trait injection to status collector | `StatusCollectorInput` receives the service manager from runtime, not from operator CLI input. | Boxed `ServiceManager` trait object. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-18-01 | Spoofing | subprocess output | mitigate | Adapter subprocess calls check `status.success()` and convert failures into `ServiceError::ManagerCommandFailed` with surfaced stderr instead of trusting raw stdout alone. Evidence: `packages/open-bitcoin-cli/src/operator/service/launchd.rs:314-340`, `packages/open-bitcoin-cli/src/operator/service/systemd.rs:209-233`, `packages/open-bitcoin-cli/src/operator/service.rs:124-125`. | closed |
| T-18-02 | Tampering | plist or unit file write | mitigate | Install paths remain preview-only unless `apply=true`, and apply mode rejects overwriting an existing service file. Evidence: `packages/open-bitcoin-cli/src/operator/service/launchd.rs:283-296`, `packages/open-bitcoin-cli/src/operator/service/systemd.rs:183-195`, `packages/open-bitcoin-cli/src/operator/service/tests.rs:346-437`. | closed |
| T-18-03 | Denial of service | double-install guard | mitigate | Both adapters return `ServiceError::AlreadyInstalled` before any write when the destination file already exists, and command-level tests surface that failure. Evidence: `packages/open-bitcoin-cli/src/operator/service/launchd.rs:293-296`, `packages/open-bitcoin-cli/src/operator/service/systemd.rs:193-195`, `packages/open-bitcoin-cli/src/operator/service/tests.rs:574-606`. | closed |
| T-18-04 | Information Disclosure | log path in generated content | accept | Generated content includes only operator-chosen filesystem log paths and no credential material. The risk is accepted and logged below. Evidence: `packages/open-bitcoin-cli/src/operator/service.rs:251-257`, `18-01-SUMMARY.md:137-139`. | closed |
| T-18-05 | Elevation of Privilege | user-level service scope | mitigate | Dry-run output explicitly states user-level scope with no sudo, launchd targets `gui/<uid>`, and systemd uses `systemctl --user`. Evidence: `packages/open-bitcoin-cli/src/operator/service.rs:248-249`, `packages/open-bitcoin-cli/src/operator/service/launchd.rs:313-333`, `packages/open-bitcoin-cli/src/operator/service/systemd.rs:209-224`. | closed |
| T-18-06 | Tampering | `--apply` flag default | mitigate | `ServiceArgs.apply` defaults to `false`, help text documents dry-run as the default, and parsing tests cover both with and without `--apply`. Evidence: `packages/open-bitcoin-cli/src/operator.rs:84-92`, `packages/open-bitcoin-cli/src/operator/service/tests.rs:668-692`. | closed |
| T-18-07 | Elevation of Privilege | `detection_roots().service_dirs` | accept | Runtime only adds read-only service directories to detection inputs, and detect logic extends candidate paths from those directories without writing. The residual read-scope risk is accepted and logged below. Evidence: `packages/open-bitcoin-cli/src/operator/runtime.rs:449-477`, `packages/open-bitcoin-cli/src/operator/detect.rs:330-344`, `18-02-SUMMARY.md:124-126`. | closed |
| T-18-08 | Denial of service | `current_exe()` failure | mitigate | Runtime falls back to `PathBuf::from("open-bitcoin")` instead of panicking if `current_exe()` fails. Evidence: `packages/open-bitcoin-cli/src/operator/runtime.rs:199-216`. | closed |
| T-18-09 | Denial of service | `manager.status()` error during status collection | mitigate | Service status collection catches adapter errors and degrades all four service fields to unavailable rather than propagating the failure. Targeted tests verify the fallback. Evidence: `packages/open-bitcoin-cli/src/operator/status.rs:479-518`, `packages/open-bitcoin-cli/src/operator/status/tests.rs:763-835`. | closed |
| T-18-10 | Information Disclosure | service diagnostics in status output | accept | Diagnostics are intentionally operator-visible status text and the phase does not introduce credential-bearing fields in that output. The residual disclosure risk is accepted and logged below. Evidence: `packages/open-bitcoin-cli/src/operator/service.rs:288-294`, `18-03-SUMMARY.md:120-122`. | closed |
| T-18-11 | Tampering | `maybe_service_manager` field injection | accept | Runtime owns service-manager construction and injects it directly into `StatusCollectorInput`; operators cannot swap the implementation through CLI flags. The residual trust-in-runtime risk is accepted and logged below. Evidence: `packages/open-bitcoin-cli/src/operator/runtime.rs:300-320`, `packages/open-bitcoin-cli/src/operator/status.rs:479-518`, `18-03-SUMMARY.md:120-122`. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-18-01 | T-18-04 | Generated log paths are ordinary operator-visible filesystem paths; the phase adds no credential values to generated plist or unit content. | Phase 18 threat model disposition | 2026-05-03 |
| AR-18-02 | T-18-07 | Adding service directories to detection broadens only read-only candidate discovery and does not introduce a write path in detection. | Phase 18 threat model disposition | 2026-05-03 |
| AR-18-03 | T-18-10 | Status diagnostics remain operator-facing troubleshooting output and are limited to manager-visible state text. | Phase 18 threat model disposition | 2026-05-03 |
| AR-18-04 | T-18-11 | Runtime-owned trait injection is trusted internal wiring, not a user-controlled extension point on this CLI surface. | Phase 18 threat model disposition | 2026-05-03 |

## Summary Threat Flags

| Summary | Threat Flags Result | Mapping |
|---------|---------------------|---------|
| `18-01-SUMMARY.md` | No new endpoints or auth paths; subprocess calls are local utilities with separate `arg()` usage; `AlreadyInstalled` guard closes the silent-overwrite case. | Supports T-18-01, T-18-02, T-18-03, T-18-05 |
| `18-02-SUMMARY.md` | Dry-run default and read-only detection-root addition match the plan dispositions; `current_exe()` fallback is non-fatal. | Supports T-18-06, T-18-07, T-18-08 |
| `18-03-SUMMARY.md` | `manager.status()` errors degrade to unavailable fields and no new trust boundaries are introduced. | Supports T-18-09, T-18-10, T-18-11 |

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-03 | 11 | 11 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| `rg -n "<threat_model>|threat_id|Threat Flags|T-18-|accepted|transfer|mitigation" .planning/milestones/v1.1-phases/18-service-lifecycle-integration/18-0{1,2,3}-PLAN.md .planning/milestones/v1.1-phases/18-service-lifecycle-integration/18-0{1,2,3}-SUMMARY.md` | Passed; found all 11 declared threats and the matching summary threat-flag notes. |
| `cargo run --manifest-path packages/Cargo.toml --package open-bitcoin-cli --bin open-bitcoin -- service --help` | Passed; `status`, `install`, `uninstall`, `enable`, `disable`, and the dry-run-by-default `--apply` help text are present. |
| `cargo run --manifest-path packages/Cargo.toml --package open-bitcoin-cli --bin open-bitcoin -- service install` | Passed; command stayed in dry-run mode, printed user-scope guidance, listed launchctl commands, and rendered generated plist content. |
| `cargo run --manifest-path packages/Cargo.toml --package open-bitcoin-cli --bin open-bitcoin -- service status` | Passed; unmanaged state surfaced preview guidance instead of failing. |
| `cargo run --manifest-path packages/Cargo.toml --package open-bitcoin-cli --bin open-bitcoin -- status` | Passed; `Service: manager=launchd installed=false enabled=false running=false` rendered in the integrated status output. |
| `cargo test --manifest-path packages/Cargo.toml --package open-bitcoin-cli --all-features service::tests` | Passed; 32 service-related tests succeeded, including dry-run isolation, `--apply` parsing, unmanaged preview diagnostics, and command preview coverage. |
| `cargo test --manifest-path packages/Cargo.toml --package open-bitcoin-cli --all-features collect_status_snapshot_with_error_manager_falls_back_to_unavailable` | Passed; status collection degraded to unavailable fields on manager error instead of panicking. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed. |
| `bash scripts/verify.sh` | Passed; hooks, LOC freshness, parity breadcrumbs, panic-site guard, Cargo checks, benchmark smoke, Bazel smoke build, coverage, and architecture checks completed successfully. |
| `git diff --check -- .planning/milestones/v1.1-phases/18-service-lifecycle-integration/18-SECURITY.md .planning/milestones/v1.1-phases/18-service-lifecycle-integration/18-UAT.md` | Passed. |

## Standards Inputs

Materially applied repo-local `AGENTS.md`, `AGENTS.bright-builds.md`,
`standards-overrides.md`, and the `gsd-secure-phase` workflow. The canonical
Bright Builds `standards/` pages referenced by repo guidance were not present in
this checkout on 2026-05-03, so this audit relied on the local guidance and
sidecar rules available in-repo.

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-05-03
