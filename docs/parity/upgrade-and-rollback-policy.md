# Upgrade And Rollback Policy

Surface id: `v1-8-upgrade-rollback-policy`

v1.8 defines source-built upgrade boundaries for Open Bitcoin operators and
contributors. It does not claim production full-node readiness; it keeps
upgrade, rollback, backup, and compatibility decisions tied to repo-local
evidence and future-scoped mutation plans.

## Scope And Non-Claims

Use this policy with [`runtime-guide.md`](../operator/runtime-guide.md),
[`status-snapshot.md`](../architecture/status-snapshot.md), and
[`storage-decision.md`](../architecture/storage-decision.md). For long-run
review, no-progress handling, support-bundle timelines, and escalation evidence,
use the canonical [`operator-runbooks.md`](operator-runbooks.md). It follows the
Phase 82 support terms exactly: `supported`, `preview`, `opt-in UAT`,
`unsupported`, and `deferred`.
For service operation expectations, use
[`service-operation-expectations.md`](service-operation-expectations.md); that
guidance preserves no-hidden service/config/datadir mutation boundaries while
keeping service preview, opt-in lifecycle UAT, and production-service non-claims
separate.
For release review, use the v1.8 release-readiness checklist in
[`release-readiness.md`](release-readiness.md#v18-release-readiness-checklist);
it points back to this upgrade policy rather than duplicating the policy tables.

The current policy covers UPG-01, UPG-02, and UPG-03. UPG-04 is the
deterministic drift-check requirement for later Phase 84 verification wiring,
not an operator permission to mutate local state.

This document is source-built and local-first. It does not widen the Phase 82
production claim boundary, create a second support matrix, or turn historical
public-network evidence into a production-node support statement.
The operator runbook link above is long-run and escalation guidance only; it
does not authorize hidden mutation, automatic rebuild, destructive repair, or
config/service changes.

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

## State And Schema Compatibility Decision Table

Compatibility decisions use existing recovery vocabulary from status, support
evidence, runtime guidance, and storage decisions. They require field-level
evidence from the selected Open Bitcoin datadir, plus `Unavailable: &lt;reason&gt;`
for any expected field that cannot be collected.

| Evidence observed | Compatibility category | Action class | Allowed next action | Forbidden hidden mutation | Required evidence |
| --- | --- | --- | --- | --- | --- |
| `clean_shutdown` | clean selected store state | `safe_retry` | Retry the source-built command after confirming the same explicit datadir and config paths. | Changing source datadirs, wallets, service files, or configs because shutdown was clean. | Status or support fields naming `clean_shutdown`, source revision, command, datadir, and config paths. |
| `unclean_shutdown` | interrupted selected store state | `safe_retry` | Retry only after preserving the current status fields and confirming no higher-priority recovery evidence is present. | Clearing markers, deleting files, or rewriting the store before the retry. | Status or support fields naming `unclean_shutdown`, latest stop reason, recovery action, and unavailable reasons. |
| `storage_lock_contention` | selected store lock contention | `read_only_inspection` | Inspect read-only evidence, confirm whether another process owns the selected datadir, and stop normal upgrade attempts until classified. | Deleting lock artifacts, scanning OS process tables as policy, or changing supervisor state. | Recovery evidence naming `storage_lock_contention`, affected path or namespace when available, and lock evidence basis. |
| `incompatible_schema` | selected store schema is incompatible | `stop_and_escalate` | Stop the upgraded process, preserve redacted evidence, and escalate with source revision and command context. | Rewriting schema records, downgrading files in place, or auto-converting stores. | Recovery category/action fields, schema version evidence, `schema_mismatch` when present, and unavailable reasons. |
| `schema_mismatch` | schema mismatch cause | `stop_and_escalate` | Stop and attach redacted field-level evidence before any retry or rollback decision. | Inferring compatibility from daemon startup or mutating schema metadata. | `recovery_evidence.cause` or equivalent support field naming `schema_mismatch`. |
| `store_corruption` | selected store corruption | `backup_then_rebuild` | Preserve a backup and field evidence before any future operator-decided rebuild workflow. | Automatic destructive rebuild, repair, compaction, reindex, or source datadir mutation. | Recovery category/action fields, affected namespace or path when available, and corruption evidence basis. |
| `corruption_marker` | corruption marker cause | `backup_then_rebuild` | Preserve the marker evidence and backup before any future rebuild plan. | Clearing markers or rewriting records as part of rollback guidance. | `recovery_evidence.cause` or equivalent support field naming `corruption_marker`. |
| `corrupt_record` | corrupt record cause | `backup_then_rebuild` | Preserve redacted evidence and backup before future rebuild or escalation. | Editing records in place or treating partial status as repair permission. | `recovery_evidence.cause` or equivalent support field naming `corrupt_record`. |
| `partial_write` | partial write cause | `backup_then_rebuild` | Preserve the selected store and backup before future rebuild planning. | Retrying repeated mutations until the partial-write class is understood. | `recovery_evidence.cause` or equivalent support field naming `partial_write`. |
| `unreadable_namespace` | unreadable namespace cause | `backup_then_rebuild` | Preserve a backup and namespace evidence before escalation or future rebuild work. | Dropping namespaces, compacting, or replacing files inside this policy. | `recovery_evidence.cause` or equivalent support field naming `unreadable_namespace`. |
| `backend_open_failure` | backend-open failure pending classification | `read_only_inspection` | Inspect read-only evidence first; escalate when field evidence cannot classify the cause. | Assuming corruption, deleting locks, or changing datadir ownership without evidence. | Backend error class, unavailable-field reasons, datadir path, and any recovery category/action fields that were available. |

## Evidence That Is Not Sufficient

The following signals are useful context but are not compatibility proof by
themselves:

- daemon startup
- elapsed time
- peer reachability
- raw logs
- report existence alone

Compatibility decisions require field-level evidence and `Unavailable:
&lt;reason&gt;` for missing fields. A status command, support bundle, log tail, or
report may support a decision only when it preserves the typed recovery category,
cause, action class, evidence basis, and unavailable-field reasons needed to
interpret the selected datadir.

## Open Bitcoin Store Versus External State

Open Bitcoin-owned durable store state is the selected Open Bitcoin datadir and
its typed status/support evidence. It is separate from external Core/Knots
source datadirs and wallets.

External Core/Knots source datadirs and wallets are high-value input. Rollback
guidance must not rewrite, repair, restore, import, or otherwise mutate them;
any such action requires a future scoped migration, wallet, or recovery plan
with explicit backup and operator-consent gates.

## Failed Upgrade Guidance

When an attempted upgrade fails, preserve evidence before repeating commands or
changing local state:

1. stop the attempted upgraded process
2. record exact command and commit
3. collect redacted local evidence
4. preserve backups
5. avoid repeated mutation until the compatibility class is understood

Use the pre-upgrade checklist, state/schema table, and support-matrix redaction
rules together. Attach the smallest useful redacted local evidence set and
write `Unavailable: &lt;reason&gt;` for missing fields rather than inferring a
healthy or incompatible state from logs, elapsed time, or startup behavior.

## Rollback Guidance

Rollback guidance is source-built and local-first:

1. return to the previous checked-out source revision or known binary
2. use the same explicit datadir and config paths
3. verify with repo-local commands
4. record rollback evidence

The repo-local verification path stays explicit: run `bash scripts/verify.sh`
for the checked-out source and collect status/support evidence through the
Cargo or Bazel command forms in the pre-upgrade checklist. Use the same selected
datadir and config paths when comparing before-upgrade, failed-upgrade, and
rollback evidence.

This policy does not imply package-manager rollback, signed release channels,
or automatic update behavior. Those surfaces remain deferred until a future
release-engineering plan defines signing, provenance, distribution, rollback,
and operator-support gates.

## Boundary And Deferred Work

Phase 84 does not recommend hidden mutation of source datadirs, external wallets, service files, launchd/systemd state, bitcoin.conf, or Open Bitcoin JSONC config.

Any future mutation guidance must be explicit, scoped, backup-aware, and tied to
the relevant migration, wallet, service, storage, or release-engineering plan.
This policy may tell an operator to stop, inspect, preserve, back up, retry
safely, return to a previous source revision, or escalate with redacted
evidence; it does not authorize the policy text itself to change local state.

Destructive repair remains deferred.

backup_then_rebuild is evidence and operator-decision guidance, not permission for automated destructive rebuild or repair.
