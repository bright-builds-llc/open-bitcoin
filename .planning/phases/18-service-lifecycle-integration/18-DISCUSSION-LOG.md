# Phase 18: Service Lifecycle Integration - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-27
**Phase:** 18-service-lifecycle-integration
**Mode:** Yolo
**Areas discussed:** Service Manager Architecture, File Generation, Dry-Run Semantics, Service Status, Module Location

---

## Service Manager Architecture

| Option | Description | Selected |
|--------|-------------|----------|
| First-party trait + subprocess adapters | `ServiceManager` trait with `LaunchdManager`/`SystemdManager` using `launchctl`/`systemctl` subprocess calls | ✓ |
| Third-party service manager crate | Use a crate like `service-manager` from crates.io | |
| Platform-specific conditional compilation without trait | `#[cfg]` blocks with no abstraction layer | |

**User's choice:** First-party `ServiceManager` trait with `LaunchdManager` and `SystemdManager` implementations
**Notes:** Matches project dependency policy (no third-party production dependencies not already in use). macOS higher priority than Linux, same public interface for both.

---

## File Generation

| Option | Description | Selected |
|--------|-------------|----------|
| Pure helpers in open-bitcoin-node | No filesystem access; returns `String`; unit-testable | ✓ |
| Inline generation in CLI adapter | Plist/unit text generated in the service executor | |
| Template files embedded as `include_str!` | External template files baked into binary | |

**User's choice:** Pure plist/unit text generators in `open-bitcoin-node/src/service.rs`
**Notes:** Preserves functional core / imperative shell boundaries. Unit tests can assert on generated file content without OS or filesystem involvement.

---

## Dry-Run Semantics

| Option | Description | Selected |
|--------|-------------|----------|
| `--apply` flag required for writes | Default is preview; `--apply` executes real changes | ✓ |
| `--dry-run` flag to preview | Default is execute; `--dry-run` previews | |
| Always execute, no preview | Mutating commands act immediately | |

**User's choice:** `--apply` flag required for real writes; preview is the default behavior
**Notes:** Matches Phase 17 operator-trust philosophy. Operators see exactly what will happen before any change is made.

---

## Service Status

| Option | Description | Selected |
|--------|-------------|----------|
| Subprocess query + full state mapping | Query `launchctl list`/`systemctl is-active`, map to installed/enabled/running/failed/stopped/unmanaged | ✓ |
| Read plist/unit file only | Check file existence without querying manager subprocess | |
| Reuse existing `ServiceStatus` as-is without enrichment | Keep only `installed`/`enabled`/`running` fields | |

**User's choice:** Subprocess query with extended state mapping including `failed` and `unmanaged`
**Notes:** SVC-04 explicitly requires identifying failed and unmanaged states. `unmanaged` covers unsupported OS or missing manager binary gracefully.

---

## Module Location

| Option | Description | Selected |
|--------|-------------|----------|
| Trait in open-bitcoin-node, adapter in open-bitcoin-cli/operator/service.rs | Clean separation; pure logic in node crate, effectful shell in CLI | ✓ |
| New open-bitcoin-service crate | Separate crate for service lifecycle | |
| Everything in open-bitcoin-cli | No node crate involvement | |

**User's choice:** `open-bitcoin-node/src/service.rs` for trait + pure generators; `open-bitcoin-cli/src/operator/service.rs` for CLI adapter
**Notes:** Consistent with how status and onboarding are organized. Avoids premature new-crate overhead for a bounded surface.

---

## Claude's Discretion

- Exact field names, helper method signatures, and Rust module structure within the described boundaries
- Initial macOS implementation may default to user-level install path before adding system-level
- Linux systemd may be a stub on macOS builds that returns `unmanaged`

## Deferred Ideas

- Ratatui dashboard service status panel — Phase 19
- System-level macOS launchd install (requiring sudo) — possible Phase 18 follow-up if user-level satisfies SVC-01
- Windows SCM support — out of scope for v1.1
