---
phase: 18-service-lifecycle-integration
reviewed: 2026-04-27T03:02:45Z
depth: standard
files_reviewed: 10
files_reviewed_list:
  - packages/open-bitcoin-cli/src/operator/service.rs
  - packages/open-bitcoin-cli/src/operator/service/launchd.rs
  - packages/open-bitcoin-cli/src/operator/service/systemd.rs
  - packages/open-bitcoin-cli/src/operator/service/fake.rs
  - packages/open-bitcoin-cli/src/operator/service/tests.rs
  - packages/open-bitcoin-cli/src/operator/runtime.rs
  - packages/open-bitcoin-cli/src/operator/status.rs
  - packages/open-bitcoin-cli/src/operator/status/tests.rs
  - packages/open-bitcoin-cli/src/operator.rs
  - docs/parity/source-breadcrumbs.json
findings:
  critical: 1
  warning: 4
  info: 2
  total: 7
status: issues_found
---

# Phase 18: Code Review Report

**Reviewed:** 2026-04-27T03:02:45Z
**Depth:** standard
**Files Reviewed:** 10
**Status:** FLAG — issues found; one logic bug should be fixed before shipping

## Overall Verdict

**FLAG.** The implementation is structurally sound: the `ServiceManager` trait, `FakeServiceManager`, dry-run gate, and `Box<dyn ServiceManager>` injection are all correct. One correctness bug (`Stopped` state maps `enabled=false` contrary to the variant's own doc comment) needs a fix. One display bug shows misleading "Would write:" text after a real write. Two path-handling issues deserve attention before files with special characters in paths are encountered in the wild. No real `unwrap()` panics, no command injection surface, no hardcoded secrets.

---

## Critical Issues

### CR-01: `Stopped` State Incorrectly Reports `enabled: false`

**File:** `packages/open-bitcoin-cli/src/operator/status.rs:363-366`

**Issue:** `ServiceLifecycleState::Stopped` is defined as "Service is installed and **enabled** but not currently running." However, the `enabled` field mapping in `collect_service_status` only matches `Enabled | Running`, omitting `Stopped`. A service that stopped after being enabled (the most common post-boot scenario on a machine undergoing planned maintenance, or a node that crashed) would therefore report `enabled: false` — an incorrect and confusing status visible in both human and JSON output. The `Failed` state has the same omission.

```rust
// Current (incorrect):
let enabled = matches!(
    snapshot.state,
    ServiceLifecycleState::Enabled | ServiceLifecycleState::Running
);

// Fix — include states where the service is enabled but not actively running:
let enabled = matches!(
    snapshot.state,
    ServiceLifecycleState::Enabled
        | ServiceLifecycleState::Running
        | ServiceLifecycleState::Stopped
        | ServiceLifecycleState::Failed
);
```

`Stopped` and `Failed` both represent a service registered with the service manager that has a boot target, meaning it is "enabled" in the launchd/systemd sense. Whether to include `Failed` depends on product semantics, but `Stopped` is unambiguous given the variant doc comment.

---

## Warnings

### WR-01: Misleading "Would write:" Label in Non-Dry-Run Output

**File:** `packages/open-bitcoin-cli/src/operator/service.rs:221-223`

**Issue:** `render_service_outcome` unconditionally prints `"  Would write: {path}"` whenever `maybe_file_path` is set, regardless of whether `dry_run` is `true` or `false`. After a real `install --apply`, the user sees both "Wrote plist to /path" (from `description`) and "Would write: /path" on the next line — contradictory output that will confuse users.

```rust
// Current (always "Would write:"):
if let Some(path) = &outcome.maybe_file_path {
    lines.push(format!("  Would write: {}", path.display()));
}

// Fix — condition the label on dry_run:
if let Some(path) = &outcome.maybe_file_path {
    let label = if outcome.dry_run { "Would write" } else { "Wrote" };
    lines.push(format!("  {label}: {}", path.display()));
}
```

### WR-02: Unquoted Paths in systemd `ExecStart` Break on Paths with Spaces

**File:** `packages/open-bitcoin-cli/src/operator/service/systemd.rs:43`

**Issue:** `generate_unit_content` interpolates `binary_path`, `data_dir`, and `config_path` directly into the `ExecStart=` line without quoting:

```
ExecStart={binary_str} --datadir {data_dir_str}{config_arg}
```

systemd parses `ExecStart` using shell-like token splitting. A binary path such as `/home/user name/bin/open-bitcoin` or a datadir of `/mnt/my data/bitcoin` will be split into multiple tokens, causing the service unit to fail with `No such file or directory`. Paths should be quoted, or the generator should validate that paths contain no whitespace and document the restriction.

```
# Fix — wrap each path token in quotes:
ExecStart="{binary_str}" --datadir "{data_dir_str}"{config_arg}
```

Note: The plist generator (`launchd.rs`) is safe because each argument is wrapped in a separate `<string>` element.

### WR-03: Unescaped XML Special Characters in Plist Content

**File:** `packages/open-bitcoin-cli/src/operator/service/launchd.rs:34-76`

**Issue:** `generate_plist_content` interpolates path strings directly into XML using `{}` format args. Paths containing `&`, `<`, or `>` (unusual but valid on macOS) would produce malformed XML that `launchd` rejects at parse time. For example, a path like `/Users/alice/data&config` would yield `<string>/Users/alice/data&config</string>`, which is invalid XML.

```rust
// Fix — escape the four XML special characters before interpolation:
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}
```

Apply `xml_escape` to `binary_str`, `data_dir_str`, `config_path.display()`, and `log_str` before embedding them in the format string.

### WR-04: Silent `systemctl disable` Failure Swallowed During Uninstall

**File:** `packages/open-bitcoin-cli/src/operator/service/systemd.rs:149-151`

**Issue:** During `uninstall` (apply=true), `systemctl --user disable` output is discarded with `let _ = ...output()`. If the disable command fails (e.g., because D-Bus is unavailable in a headless SSH session), the failure is invisible: the unit file is deleted anyway and the error is never reported. The next boot could still start the service if the enable symlink was not removed.

This is more severe than a style issue because it can leave the system in an inconsistent state. The launchd adapter (`uninstall`) correctly propagates `launchctl bootout` failures (exit code 36 aside). systemd should follow the same pattern: inspect the result and either propagate or explicitly document why the error is intentionally ignored.

```rust
// Fix — check the result and surface non-fatal errors in the outcome description:
let disable_result = std::process::Command::new("systemctl")
    .args(["--user", "disable", OPEN_BITCOIN_SYSTEMD_FILE_NAME])
    .output();
let disable_note = match disable_result {
    Ok(out) if out.status.success() => None,
    Ok(out) => Some(format!(
        "disable warning (exit {}): {}",
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stderr).trim()
    )),
    Err(cause) => Some(format!("disable not run: {cause}")),
};
// ... then include disable_note in the outcome description if Some
```

---

## Info

### IN-01: `uid()` Fallback to 501 Could Register Service Under Wrong User

**File:** `packages/open-bitcoin-cli/src/operator/service/launchd.rs:101-109`

**Issue:** `uid()` falls back to UID 501 (the typical first macOS user) if `id -u` fails or returns non-numeric output. If this fallback fires in a CI or unusual environment, `launchctl bootstrap gui/501/{label}` will silently target the wrong user session rather than failing with a clear error. The intent is robustness, but a wrong UID means the bootstrap/bootout commands silently target the wrong session.

**Suggestion:** Return a `Result<u32, ServiceError>` from `uid()` (or return `Option<u32>`) and surface a diagnostic in the outcome rather than silently falling back to a specific UID. Alternatively, document in the struct that 501 is the macOS default and is intentional fallback behavior.

### IN-02: `--apply` Flag Is Global But Only Meaningful for `install`/`uninstall`

**File:** `packages/open-bitcoin-cli/src/operator.rs:71-76`

**Issue:** `ServiceArgs.apply` is declared `#[arg(global = true)]`, meaning `open-bitcoin service enable --apply` is silently accepted even though `enable` and `disable` always execute (ignoring the flag). A user who passes `--apply` to `enable` expecting gated behavior will be confused when the command executes immediately.

**Suggestion:** Either remove `global = true` and restrict `--apply` to the `install` and `uninstall` subcommands via clap, or print a note in the `enable`/`disable` output when `--apply` is set to clarify the flag has no effect there. The D-12 note in the `execute_service_command` doc comment explains the intent but it is not surfaced to the user.

---

## Per-File Analysis

| File | Verdict | Notes |
|------|---------|-------|
| `service.rs` | FLAG | WR-01: misleading "Would write:" label in non-dry-run path |
| `service/launchd.rs` | FLAG | WR-03: unescaped XML paths; IN-01: UID fallback to 501 |
| `service/systemd.rs` | FLAG | WR-02: unquoted paths in ExecStart; WR-04: swallowed disable failure |
| `service/fake.rs` | PASS | Clean; correct RefCell use; no I/O |
| `service/tests.rs` | PASS | RAII TestDirectory cleanup correct; good dry-run isolation coverage |
| `runtime.rs` | PASS | `current_exe` fallback is reasonable; no unwrap() |
| `status.rs` | FLAG | CR-01: Stopped state maps enabled=false incorrectly |
| `status/tests.rs` | PASS | FakeServiceManagerError covers error fallback; good coverage of state variants |
| `operator.rs` | FLAG | IN-02: --apply global flag accepted silently by enable/disable |
| `docs/parity/source-breadcrumbs.json` | PASS | No issues (data file) |

---

_Reviewed: 2026-04-27T03:02:45Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
