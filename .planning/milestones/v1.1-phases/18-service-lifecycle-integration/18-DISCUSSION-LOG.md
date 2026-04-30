# Phase 18: Service Lifecycle Integration - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-27
**Phase:** 18-service-lifecycle-integration
**Mode:** Yolo
**Areas discussed:** Service Module Structure, Plist/Unit Generation, Platform Detection, Service Scope, Dry-Run Safety, Status Snapshot Integration, Testing

---

## Service Module Structure

| Option | Description | Selected |
|--------|-------------|----------|
| Embed in runtime.rs | Inline service command handling in existing runtime module | |
| New top-level service crate | Separate `open-bitcoin-service` crate | |
| operator/service/ submodule with trait | `ServiceManager` trait + platform adapters in cli crate | ✓ |

**User's choice:** operator/service/ submodule with ServiceManager trait (auto-selected)
**Notes:** Keeps service logic co-located with other operator modules while maintaining trait-based testability and functional-core/imperative-shell separation.

---

## Plist and Unit File Generation

| Option | Description | Selected |
|--------|-------------|----------|
| Template engine (askama, tera) | Use a Rust template engine | |
| Pure Rust struct serializers | Typed structs with dry_run_content() string methods | ✓ |
| Embedded static templates | Hard-coded string templates | |

**User's choice:** Pure Rust struct serializers (auto-selected)
**Notes:** Avoids new template engine dependency, keeps content generation deterministic and testable as pure functions.

---

## Platform Detection

| Option | Description | Selected |
|--------|-------------|----------|
| Runtime detection via uname | Detect platform at runtime | |
| cfg(target_os) + injectable factory | Compile-time selection + test injection | ✓ |
| Explicit --platform flag | Require operator to specify platform | |

**User's choice:** cfg(target_os) + injectable factory (auto-selected)
**Notes:** Compile-time selection avoids runtime branching overhead; factory injection keeps tests hermetic.

---

## Service Scope (User vs System)

| Option | Description | Selected |
|--------|-------------|----------|
| System-level only | LaunchDaemons / system systemd, requires root | |
| User-level default | LaunchAgents / systemd --user, no root required | ✓ |
| Both with --system flag | Support both scopes in this phase | |

**User's choice:** User-level default (auto-selected)
**Notes:** User-level avoids sudo requirements for typical operator use. System-level deferred to a future phase.

---

## Dry-Run Safety

| Option | Description | Selected |
|--------|-------------|----------|
| Always apply, --dry-run to preview | Default applies, preview optional | |
| Always dry-run, --apply to execute | Default preview, explicit apply required | ✓ |
| Interactive confirm prompt | Prompt operator before each write | |

**User's choice:** Always dry-run by default, --apply to execute (auto-selected)
**Notes:** Consistent with project principle of "explicit before destructive." Operators can review generated content before any filesystem mutation.

---

## Status Snapshot Integration

| Option | Description | Selected |
|--------|-------------|----------|
| Separate service status command only | service status is standalone, not fed into status snapshot | |
| Injected adapter in collect_status_snapshot | Optional service adapter fills ServiceStatus fields | ✓ |
| Always attempt service inspection in status | Unconditionally inspect service in open-bitcoin status | |

**User's choice:** Injected adapter in collect_status_snapshot (auto-selected)
**Notes:** Follows established injection pattern from status.rs; keeps open-bitcoin status resilient when service manager is unavailable or uninspected.

---

## Testing

| Option | Description | Selected |
|--------|-------------|----------|
| Integration tests hitting real launchd/systemd | Test against actual service managers | |
| FakeServiceManager + isolated temp dirs | Trait-object fake + temp dir injection | ✓ |
| Unit tests only, no integration | Pure unit tests without filesystem writes | |

**User's choice:** FakeServiceManager + isolated temp dirs (auto-selected)
**Notes:** Required by SVC-05 (tests never modify real developer launchd/systemd state). Consistent with existing operator test patterns.

---

## Claude's Discretion

- Exact adapter struct names, internal helper names, and field ordering
- --apply vs --execute flag name
- systemd [Install] section target
- Double-install guard behavior (typed error vs --force flag)

## Deferred Ideas

- System-level service scope
- Ratatui dashboard service panels (Phase 19)
- Service restart metric wiring to dashboard (Phase 19)
- Windows service support (out of scope v1.1)
- Socket-activation / launch-on-demand variants
