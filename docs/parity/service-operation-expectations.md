# Service Operation Expectations

Surface id: `v1-8-service-operation-expectations`

Phase 86 defines source-built daemon and local user-service expectations for
Open Bitcoin. It is the canonical service expectation source for SVC-01 and
SVC-02.

## Scope And Non-Claims

Use this document with [`production-claim-boundary.md`](production-claim-boundary.md),
[`support-matrix.md`](support-matrix.md),
[`operator-runbooks.md`](operator-runbooks.md),
[`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md), and the
operator [`runtime-guide.md`](../operator/runtime-guide.md).
For release review, use the v1.8 release-readiness checklist in
[`release-readiness.md`](release-readiness.md#v18-release-readiness-checklist);
it points back to this service expectation root rather than duplicating the
service classification table.

This document distinguishes direct source-built `open-bitcoind` operation,
local launchd/systemd definition preview, opt-in real user-service lifecycle
review, and deferred service-distribution claims. It does not add a new service
manager, package-manager service, signed package, Windows service integration,
automatic update channel, production service ownership, uptime guarantee,
automatic support-bundle upload, destructive repair, migration apply mode,
public-network default check, or broad production-node readiness.

generated launchd/systemd definitions supervise `open-bitcoind`, not the `open-bitcoin` operator wrapper. User-level service paths stay local to the
operator machine: `~/Library/LaunchAgents/org.open-bitcoin.node.plist` on
macOS launchd and `~/.config/systemd/user/open-bitcoin-node.service` on Linux
systemd.

`service preview` is always side-effect-free. `service install` and `service uninstall` are previews unless `--apply` is supplied. Starting,
stopping, restarting, enabling, disabling, or uninstalling a real service is
opt-in local UAT outside default verification.

## Support Terms

The only support terms for service expectations are:

- `supported`
- `preview`
- `opt-in UAT`
- `unsupported`
- `deferred`

Do not replace these with production-service, package-supported,
community-supported, managed-service, production-ready-service, or
production-grade-service wording.

## Service Surface Classification

| Service surface | Support term | What evidence proves | Cargo command evidence | Bazel command evidence | Default verification | Opt-in UAT | Residual risk | Next gate |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Direct source-built open-bitcoind operation | `supported` | The selected checkout can start the daemon path and expose status or sync fields for one selected datadir. | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd -server=1` plus status commands below. | `bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd -server=1` plus status commands below. | Source, docs, and deterministic checker evidence only. | Operators may run the daemon locally and inspect status fields. | Daemon startup, elapsed time, or public peer reachability alone does not prove sync-to-tip, service readiness, or production-node readiness. | Future production-readiness gate with scoped P2P, wallet, packaging, service, support, and release-policy evidence. |
| Local status and support evidence | `supported` | Status JSON, sync status JSON, structured logs, bounded metrics, resource and recovery fields, and redacted support evidence describe the selected datadir. | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support`. | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support`. | Deterministic status/support behavior and docs checks through `bash scripts/verify.sh`. | Operators may collect redacted local evidence for issue review. | Missing fields must stay unavailable with reasons; support bundle path alone is not proof. | Field-level support acceptance criteria before any support workflow expansion. |
| launchd/systemd generated definition preview | `preview` | The generated user-level service definition and manager command intent can be inspected without side effects. | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install`. | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install`. | Deterministic docs, source, and checker evidence. | Operators may review rendered launchd/systemd definitions locally. | Preview output does not prove installed service lifecycle, restart/resume, uptime, packaging, or platform support. | Platform-specific service UAT and rollback evidence. |
| Real user-level launchd/systemd lifecycle | `opt-in UAT` | An operator-selected local service can be installed, started, inspected, restarted, stopped, disabled, or uninstalled, and evidence can be captured through fields. | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply`. | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply`. | Not part of default verification. | Explicit local user-service UAT only. | Host policy, service-manager state, privileges, paths, and platform behavior vary by machine. | Future service-operation milestone with uptime, install, rollback, and platform gates. |
| Service-manager unavailable status | `supported` | Status keeps `unavailable-manager` and service unavailable reasons visible instead of inferring service health. | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status`; `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`. | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status`; `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`. | Deterministic docs/checker evidence and existing status behavior. | Operators may capture unavailable service evidence from their local platform. | Missing service manager evidence cannot be promoted to installed service proof. | Platform acceptance gate if service support expands. |
| Packaged service distribution | `deferred` | No current evidence proves package-manager installation or service ownership. | None in v1.8. | None in v1.8. | None. | None. | Source-built commands are the only supported local command evidence. | Release-engineering gate for signing, provenance, reproducibility, rollback, and package-manager delivery. |
| Windows service integration | `deferred` | No current evidence proves Windows service install, supervision, rollback, or lifecycle behavior. | None in v1.8. | None in v1.8. | None. | None. | Current service expectations are macOS/Linux user-level only. | Windows service milestone with platform UAT and rollback evidence. |
| Automatic updates | `deferred` | No current evidence proves automatic update, package rollback, or service restart policy. | None in v1.8. | None in v1.8. | None. | None. | Source-built checkout updates stay manual and review-only. | Release-channel and automatic-update policy gate. |
| Production service ownership and uptime guarantees | `deferred` | No current evidence proves production service operation, uptime, support response, or on-call ownership. | None in v1.8. | None in v1.8. | None. | None. | Local UAT does not create production service ownership. | Future service-operation milestone with uptime, monitoring, incident, and support gates. |
| Broad production full-node readiness | `deferred` | No current service evidence proves the whole production-node claim. | None in v1.8. | None in v1.8. | None. | None. | Service evidence alone cannot prove inbound serving, relay, wallet safety, migration apply, packaging, support, or release-policy readiness. | Future production-readiness milestone plus deterministic claim guardrails. |

## Repo-Local Command Evidence

Every operator-facing command in this document uses repo-local Cargo and Bazel
forms. Do not replace these examples with an installed `open-bitcoin` alias.

### Direct Daemon And Status Review

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd -server=1
bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd -server=1
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
```

### Service Preview And Install Review

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply
```

### Service Lifecycle Review

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply
```

### Restart Resume Review

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
```

### Logs Metrics Resources And Support Evidence

Operators inspect logs, metrics, resources, and support evidence through fields
from `status --format json`, `sync status --format json`, and redacted support bundle output. Do not invent shell commands that scrape raw stores, raw logs, or process tables.

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

### Safe Shutdown Review

Stop the selected local user service only when the operator explicitly chose
service UAT, then inspect fields for the same datadir.

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
```

## Field-Based Evidence Rules

service file existence, daemon startup, elapsed time, raw log tail, public peer
reachability, or a support bundle path is context only and is not sufficient
proof unless the expected fields and unavailable reasons are present. When an
expected field cannot be collected, write `Unavailable: &lt;reason&gt;`.

Required service and runtime evidence areas include `service.lifecycle`,
`service.log_path`, `service.manager_command`,
`service.generated_service_file_path`, `service.unavailable_reason`,
status JSON, sync status JSON, structured logs, bounded metrics,
`resource_bounds`, `sync.resource_pressure`, `recovery_category`,
`recovery_action`, `next_action`, `support-evidence.json`, and
`support-evidence.md`.

Service lifecycle labels are exactly:

- `unmanaged`
- `installed-stopped`
- `running`
- `failed`
- `disabled`
- `unavailable-manager`

## Restart Resume Evidence

Restart/resume interpretation comes from `service.restart_resume` fields for
the same selected datadir:

- `same_datadir`
- `prior_shutdown`
- `durable_progress`
- `stale_inflight`
- `recovery_category`
- `next_action`

`same_datadir` must show that the service definition, restart action, status
JSON, and sync status JSON describe the same selected datadir.
`prior_shutdown` distinguishes clean and unclean prior daemon state.
`durable_progress` carries downloaded and connected block heights and hashes.
`stale_inflight` keeps stale block requests visible until cleanup or recovery
guidance is recorded. `recovery_category` and `next_action` preserve the typed
storage-first recovery guidance.

service restart command success, daemon startup, and elapsed time do not prove durable resume.

## Default Verification And Opt-In UAT Boundaries

Default bash scripts/verify.sh remains deterministic, public-network-free, real-service-manager-free, and multi-day-free.

The default verifier may check source, docs, deterministic fixtures, and
checker wiring. It must not run public-network live smoke, real service-manager
commands, long wall-clock sleeps, package-manager service commands, Windows
service workflows, automatic support-bundle upload, production service
ownership checks, or broad production-node readiness checks.

Real launchd/systemd install, start, stop, restart, enable, disable, status, or
uninstall review is opt-in UAT. Public-network mainnet review and multi-day
soak review remain opt-in UAT under the runtime guide and release-readiness
boundaries.

## Sensitive Evidence Boundaries

Support guidance must not request or attach:

- wallet private material
- raw wallet files
- RPC cookies
- rpcpassword
- rpcauth
- raw datadirs
- unredacted logs
- raw unbounded logs
- automatic support-bundle upload
- production service ownership

Support evidence should use the smallest useful redacted local subset. Prefer
status JSON, sync status JSON, structured-log summaries, bounded metrics,
resource and recovery fields, and redacted `support-evidence.json` plus
`support-evidence.md` over raw artifacts.
