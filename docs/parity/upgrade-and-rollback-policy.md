# Upgrade And Rollback Policy

Surface id: `v1-8-upgrade-rollback-policy`

v1.8 defines source-built upgrade boundaries for Open Bitcoin operators and
contributors. It does not claim production full-node readiness; it keeps
upgrade, rollback, backup, and compatibility decisions tied to repo-local
evidence and future-scoped mutation plans.

## Scope And Non-Claims

Use this policy with [`runtime-guide.md`](../operator/runtime-guide.md),
[`status-snapshot.md`](../architecture/status-snapshot.md), and
[`storage-decision.md`](../architecture/storage-decision.md). It follows the
Phase 82 support terms exactly: `supported`, `preview`, `opt-in UAT`,
`unsupported`, and `deferred`.

The current policy covers UPG-01, UPG-02, and UPG-03. UPG-04 is the
deterministic drift-check requirement for later Phase 84 verification wiring,
not an operator permission to mutate local state.

This document is source-built and local-first. It does not widen the Phase 82
production claim boundary, create a second support matrix, or turn historical
public-network evidence into a production-node support statement.

## Pre-Upgrade Checklist

Record this evidence before changing binaries or runtime state. The evidence is
for operator review and issue support; it is not permission to rewrite source
datadirs, wallets, service definitions, or config files.

| Evidence to record | How to collect it | Mutation status | Why it matters |
| --- | --- | --- | --- |
| current source revision or commit | Run `git rev-parse HEAD` from the repo root. | review-only evidence | Ties the attempted upgrade to an auditable checkout. |
| repo-local verification status | Run `bash scripts/verify.sh` before the upgrade. | review-only evidence | Shows whether the source-built checkout satisfied the repo-native gate before runtime changes. |
| binary provenance from Cargo or Bazel | Record whether the operator used `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --` or `bazel run //packages/open-bitcoin-cli:open_bitcoin --`. | review-only evidence | Keeps command evidence tied to the checkout instead of an installed alias. |
| Open Bitcoin JSONC config path | Record the explicit `--config` path or the selected datadir default `open-bitcoin.jsonc` path. | review-only evidence | Separates Open Bitcoin-only config from baseline-compatible `bitcoin.conf`. |
| bitcoin.conf path | Record the baseline-compatible config path for the selected datadir. | review-only evidence | Preserves RPC and node-setting provenance without rewriting baseline config. |
| selected datadir | Record the exact `--datadir` path used by the source-built command. | review-only evidence | Keeps status, support bundle, and rollback evidence scoped to one datadir. |
| datadir ownership and free-space review | Inspect ownership, permissions, mount, and available free space with platform-local read-only tools. | review-only evidence | Avoids treating storage pressure or wrong ownership as schema compatibility. |
| current sync/status evidence | Run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=&lt;path&gt; status --format json` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=&lt;path&gt; status --format json`. | review-only evidence | Captures field-level status, recovery, config, service, and build evidence before upgrade. |
| support-bundle evidence when available | Run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=&lt;path&gt; support bundle --output-dir=&lt;path&gt;/support` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=&lt;path&gt; support bundle --output-dir=&lt;path&gt;/support`. | review-only evidence | Preserves redacted local evidence when the support surface can collect it. |
| service state | Record service preview/status output or write `Unavailable: &lt;reason&gt;` when no service manager is involved. | review-only evidence | Keeps launchd/systemd state visible without silently changing supervisor files. |
| wallet scope | Record whether the run involves no wallet, a managed non-production wallet, or external Core/Knots wallet evidence. | review-only evidence | Prevents upgrade prose from implying production-funds or external-wallet support. |
| backup location | Record the operator-selected backup location and whether it covers the selected Open Bitcoin store and any relevant external high-value inputs. | review-only evidence | Makes rollback and backup_then_rebuild decisions auditable before any future mutation plan. |

Status, support bundle, config summary, service state, source revision, and
backup-location recording are all review-only evidence. Source datadir, wallet,
service, and config mutation requires a future scoped plan before any operator
workflow may recommend or automate it.
