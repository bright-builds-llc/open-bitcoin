# Operator Runbooks

Surface id: `v1-8-operator-runbooks`

Phase 85 provides procedural evidence guidance for source-built long-running
operator review. It does not claim production full-node readiness, production
service ownership, relay/inbound support, production-funds wallet safety,
migration apply mode, destructive repair, or automatic support-bundle upload.

## Scope And Non-Claims

Use this runbook with [`production-claim-boundary.md`](production-claim-boundary.md),
[`support-matrix.md`](support-matrix.md), and
[`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md). The support
terms remain exactly `supported`, `preview`, `opt-in UAT`, `unsupported`, and
`deferred`; this page does not add new support terms or promote any deferred
surface.
Service-specific lifecycle, restart/resume, log path, manager command,
generated file path, and unavailable-reason expectations live in
[`service-operation-expectations.md`](service-operation-expectations.md).
For release review, use the v1.8 release-readiness checklist in
[`release-readiness.md`](release-readiness.md#v18-release-readiness-checklist);
it points back to these runbooks rather than duplicating the procedural tables.
The v1.8 deterministic claim guardrails scan this document for overbroad production-readiness and deferred-surface promotion; they define gates only and do not claim production full-node readiness.

This runbook tells an operator what to record, what to inspect, when to stop,
and what to preserve for support triage. It does not authorize source datadir
mutation, wallet mutation, service-manager mutation, config rewrite, automatic
rebuild, destructive repair, hosted support upload, response timelines, or
production service ownership.

Default bash scripts/verify.sh remains deterministic, public-network-free, service-manager-free, and multi-day-free.

## Production-Boundary Preflight

Before any long-running source-built operation, review the current boundary,
support, and rollback roots:

1. [`production-claim-boundary.md`](production-claim-boundary.md) for allowed
   statements, support terms, and deferred surfaces.
2. [`support-matrix.md`](support-matrix.md) for issue-evidence expectations and
   the smallest useful redacted evidence set.
3. [`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md) for
   source-built rollback, backup, compatibility, and no-hidden-mutation rules.

Collect the table below as review-only evidence. Status, support bundles,
config summaries, service state, and local report paths are acceptable evidence;
source datadir, wallet, service, and config mutation remains outside Phase 85.

| Evidence to record | How to collect it | Mutation status | Escalation use |
| --- | --- | --- | --- |
| selected datadir | Record the exact `--datadir` path used for the command. | review-only evidence | Scopes every status snapshot, support bundle, and report to one local store. |
| source revision | Run `git rev-parse HEAD` from the repository root. | review-only evidence | Ties evidence to the source-built checkout. |
| repo-local verification status | Record the result of `bash scripts/verify.sh` for the checkout. | review-only evidence | Shows whether the default deterministic gate passed before operator review. |
| Cargo or Bazel command form | Record whether evidence was collected through the Cargo or Bazel command form. | review-only evidence | Prevents installed aliases from obscuring the executable path. |
| config paths | Record `bitcoin.conf`, `open-bitcoin.jsonc`, explicit `--config`, and unavailable reasons. | review-only evidence | Distinguishes baseline-compatible config from Open Bitcoin-only config. |
| current status evidence | Capture JSON status for the selected datadir. | review-only evidence | Provides field-level runtime, sync, resource, recovery, service, and build facts. |
| resource/disk review | Record disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle bounds when available. | review-only evidence | Separates resource pressure from peer or sync diagnosis. |
| service state or unavailable reason | Capture service preview/status output or write `Unavailable: &lt;reason&gt;`. | review-only evidence | Preserves service context without starting, stopping, installing, or rewriting a service. |
| wallet scope | Record no wallet, managed non-production wallet, or external wallet evidence. | review-only evidence | Prevents production-funds or external-wallet support inference. |
| support-bundle availability | Record support-bundle paths or `Unavailable: &lt;reason&gt;`. | review-only evidence | Shows whether redacted local issue evidence can be collected. |

Status command forms:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=&lt;path&gt; status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=&lt;path&gt; status --format json
```

## Long-Run Monitoring

Monitor long-running source-built review through field-level evidence from the
shared status snapshot, bounded metrics, structured logs, support-bundle
summaries, soak reports, and live-smoke reports. A soak report, live-smoke
report, public-network opt-in, stay-current opt-in, or multi-day soak opt-in is
useful only when the operator explicitly chose that non-default workflow and
kept the local report bounded and redacted.

Useful monitoring evidence includes:

| Evidence | Required fields or labels | How to use it |
| --- | --- | --- |
| progress credit | `progress_credit`, `last_useful_work`, `last_peer_contribution`, `expected_progress_window`, `no_progress_threshold` | Confirms whether durable active-chain progress or current-at-tip evidence advanced. |
| no-progress diagnosis | `stall_diagnosis`, `stalled subsystem`, `sync.no_progress_diagnosis`, `sync.no_progress_next_action` | Names the typed stalled subsystem and bounded next action instead of relying on time alone. |
| stop and recovery state | `latest_stop_reason`, `recovery_evidence`, `support_forensics` | Explains why a cycle stopped and what evidence supports the recovery class. |
| resources | `resource_bounds`, `sync.resource_pressure` | Shows disk, file, cache, queue, peer, in-flight, log, metric, support-bundle, and sync-envelope pressure. |
| structured logs | structured logs with progress, recovery, configured target, peer contribution, and stop-reason labels | Corroborates compact status fields without attaching raw log tails. |
| metrics | bounded metrics for heights, peer count, disk usage, RPC health, and service restarts | Correlates trends without retaining unbounded runtime arrays. |
| support summaries | support-bundle summaries and support_forensics | Provides redacted timeline, evidence basis, confidence, and unavailable-field reasons. |
| opt-in reports | soak reports and live-smoke reports | Documents explicit public-network, stay-current, or multi-day review outside default verification. |
| checkpoint timeline | checkpoint timeline entries from status, support, or soak evidence | Preserves ordered progress, no-progress, resource, recovery, and operator-action events. |

## No-Progress Diagnosis

elapsed time, daemon startup, peer reachability, raw log tail, report existence,
and support bundle existence are not sufficient proof. Treat them as context
only. A no-progress decision requires field-level evidence plus
`Unavailable: &lt;reason&gt;` for critical fields that cannot be collected.

Start with these checks:

1. Compare `progress_credit`, `last_useful_work`, and
   `last_peer_contribution` against `expected_progress_window` and
   `no_progress_threshold`.
2. Inspect `stall_diagnosis`, `stalled subsystem`,
   `sync.no_progress_diagnosis`, and `sync.no_progress_next_action`.
3. Check `resource_bounds` and `sync.resource_pressure` before retrying peers.
4. Use structured logs, metrics, support-bundle summaries, soak reports, and
   live-smoke reports only as corroborating evidence tied to the same selected
   datadir and command.
5. Preserve the checkpoint timeline so later support review can see when
   progress, no-progress, resource, recovery, and operator-action events
   happened.

Keep public-network opt-in, stay-current opt-in, and multi-day soak opt-in
outside the default verifier. Operators may collect those reports deliberately,
but default local verification must remain deterministic and short-running.

## Recovery And Stop Decisions

Recovery guidance uses decision classes from existing status and support
evidence. These classes are evidence guidance only; they do not authorize hidden
mutation.

| Recovery class | Required evidence | Allowed action | Forbidden action | Escalation bundle content |
| --- | --- | --- | --- | --- |
| `safe_retry` | `recovery_evidence`, latest command, selected datadir, config paths, latest_stop_reason, and unavailable-field reasons. | Retry after preserving current evidence and confirming no higher-priority recovery class is present. | Clearing markers, deleting files, changing configs, or retrying until evidence is overwritten. | Status JSON, support summary, command output, structured log summary, and retry timestamp. |
| `read_only_inspection` | Lock, backend, resource, peer, or config evidence with affected namespace/path when available. | Inspect status, service preview/status, config summaries, and local reports without mutation. | Source datadir mutation, external wallet mutation, service-manager mutation, config rewrite, or lock cleanup. | Status JSON, config summary, service state or unavailable reason, resource evidence, and support_forensics. |
| `backup_then_rebuild` | Corruption, partial write, unreadable namespace, or resource evidence plus backup location and evidence basis. | Preserve backup and redacted evidence before any future operator-decided rebuild workflow. | Automatic rebuild, destructive repair, source datadir mutation, or external wallet mutation. | Backup location, recovery_evidence, resource_bounds, command output, and redacted support-bundle timeline. |
| `stop_and_escalate` | Repeated typed no-progress, incompatible schema, inconsistent evidence, unavailable critical fields, or stop-required resource pressure. | Stop normal attempts, preserve evidence, redact sensitive data, and escalate with exact commands. | Response timelines, hosted support upload, production service ownership, automatic support-bundle upload, or continued mutation attempts. | Minimum useful bundle, final status, escalation decision, and `Unavailable: &lt;reason&gt;` entries. |

Phase 85 forbids destructive repair, source datadir mutation, external wallet
mutation, service-manager mutation, config rewrite, automatic rebuild, response
timelines, hosted support upload, and production service ownership.

## Escalation Evidence Thresholds

Escalate instead of continuing normal attempts when any threshold applies:

- repeated no-progress with typed cause
- unavailable critical fields
- recovery class requiring stop/escalate
- resource pressure crossing documented bounds
- inconsistent status/support evidence
- failure to collect the minimum redacted support-bundle timeline

When escalation is required, stop the current normal attempt, preserve the
selected datadir evidence without mutation, redact sensitive material, record
the exact Cargo or Bazel commands, and attach unavailable-field reasons instead
of inferring a healthy or failed state.

## Support-Bundle Timeline

A useful support-bundle timeline is redacted, ordered, and tied to one selected
datadir. Use these exact timeline event labels:

1. preflight evidence
2. command start
3. status snapshots
4. progress or no-progress events
5. resource/recovery events
6. support-bundle collection
7. operator action taken
8. final status
9. escalation decision

The minimum useful bundle contains:

- redacted `support-evidence.json`
- redacted `support-evidence.md`
- exact command output
- bounded log summary
- config summary
- service state or unavailable reason
- resource evidence
- recovery/progress evidence
- sync evidence
- version/toolchain context
- platform details
- exact repo-local reproduction command
- `Unavailable: &lt;reason&gt;`

Support-bundle command forms:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=&lt;path&gt; support bundle --output-dir=&lt;path&gt;/support --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=&lt;path&gt; support bundle --output-dir=&lt;path&gt;/support --format json
```

## Privacy And Safety Boundaries

Do not attach or request:

- wallet private material
- raw wallet files
- RPC cookies
- rpcpassword
- rpcauth
- raw datadirs
- unredacted logs
- raw unbounded logs
- full peer tables with sensitive local data
- automatic support-bundle upload

Support evidence should be the smallest useful redacted local subset. Keep raw
datadirs, raw wallets, credentials, unbounded logs, and sensitive peer data out
of issue reports and support bundles unless a future scoped support-upload or
forensics plan explicitly changes that boundary.
