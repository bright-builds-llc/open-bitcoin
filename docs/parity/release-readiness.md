# Release Readiness

This release-hardening handoff preserves the headless v1.3 Public Mainnet Sync
Proof and Node Hardening evidence, the v1.4 Operator Evidence, Threat Model,
and Release Boundaries closeout, the v1.5 Unattended Mainnet Node Operation
Readiness closeout, and the v1.6 Mainnet Full-Sync Completion closeout as
historical context. It adds the current v1.8 Production Claim Boundary handoff
and points reviewers at repo-owned evidence instead of reproducing full phase
logs, checking generated benchmark, support-bundle, soak, or live-smoke
artifacts into git, or making public-network checks part of default
verification.
For v1.9, this handoff adds the Network Participation Evidence and Release
Boundary closeout for opt-in inbound peer serving evidence while keeping relay,
public-default, service-operation, and production-readiness claims deferred.
For v2.0, this handoff adds the Transaction Relay and Mempool Participation
Boundary closeout for deterministic local relay/mempool evidence, repo-local
UAT commands, and no-claim guardrails while keeping public relay defaults,
compact block relay, package relay, bloom/filter serving, public-network relay
CI, production service operation, production full-node readiness, and
production-funds wallet use deferred.
For v2.1, this handoff records the milestone shipped and archived on 2026-07-22
with bounded, explicit, default-off block serving and compact-block relay,
deterministic local evidence, and optional public-network operator review. It
does not promote public serving or relay defaults, archive-node behavior,
production-scale historical serving, or any production readiness claim.

## Readiness Verdict

The v1.3 readiness claim remains historical: a source-built, opt-in,
local-evidence public-mainnet sync proof and node-hardening review surface. It
covers the documented live-smoke workflow, durable sync status truth, peer
resilience, resource bounds, durable recovery, redacted support evidence, and
reviewer traceability needed for the Phase 50 evidence closeout.

The v1.4 readiness claim remains historical: a source-built, opt-in outbound
IBD evidence surface. It covers outbound peer compatibility, validated header
progress, downloaded block progress, connected block progress, same-datadir
restart/resume evidence, redacted support evidence, field-level operator
interpretation, and explicit release boundaries for reviewer closeout.

The v1.5 readiness claim remains historical: source-built, explicit opt-in extended
unattended mainnet operator review readiness. It covers the bounded unattended
daemon sync loop, resource bounds, recovery states, long-run truth surfaces,
user-level launchd/systemd service supervision, same-datadir service
restart/resume evidence, redacted support evidence, compatibility wrapper
reports, deterministic local verification, and parity roots that make those
claims auditable.

The v1.6 readiness claim remains historical: source-built, explicit opt-in
full-sync completion evidence. It covers validated active-chain progress to the
best-known peer tip, durable restart/resume state, stay-current review,
reorg/no-progress/recovery handling, resource bounds, shared status evidence,
redacted support bundles, opt-in UAT commands, deterministic local verification,
and parity roots that make those claims auditable.

The current v1.8 boundary defines the support terms and evidence gates required
before a future production full-node readiness claim. It does not claim
production full-node readiness. The canonical boundary is
[`docs/parity/production-claim-boundary.md`](production-claim-boundary.md) with
surface id `v1-8-production-claim-boundary`. The Phase 83 support matrix and
issue-evidence handoff is
[`docs/parity/support-matrix.md`](support-matrix.md) with surface id
`v1-8-support-matrix-issue-evidence`. The Phase 84 source-built upgrade,
rollback, backup, and compatibility policy is
[`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md) with surface
id `v1-8-upgrade-rollback-policy`.
The Phase 85 operator runbook is
[`operator-runbooks.md`](operator-runbooks.md) with surface id
`v1-8-operator-runbooks`.
The Phase 86 service operation expectations document is
[`service-operation-expectations.md`](service-operation-expectations.md) with
surface id `v1-8-service-operation-expectations`.
The Phase 87 release-readiness checklist is this document with surface id
`v1-8-release-readiness-checklist`.
The Phase 88 deterministic claim guardrails are rooted in this handoff with
surface id `v1-8-deterministic-claim-guardrails`. The v1.8 deterministic claim
guardrails prevent overbroad production-readiness and deferred-surface claims
in the public release/operator docs; they do not claim production full-node
readiness.

The v1.7 readiness claim remains historical: source-built, explicit opt-in
full-sync soak and recovery hardening. It covers durable multi-day soak
evidence, disk and resource bounds, corruption and lock recovery diagnosis,
progress guarantees, stall diagnosis, support-bundle forensics, opt-in UAT
commands, deterministic release-boundary checks, and parity roots that make
those claims auditable.

The current v1.9 Network Participation Evidence and Release Boundary closes
the inbound-serving milestone as bounded, explicit opt-in evidence. It connects
the Phase 90 listener/admission, Phase 91 permissions, Phase 92 address
boundaries, Phase 93 eviction/ban/misbehavior policy, Phase 94 resource
governance, Phase 95 support-redaction evidence, Phase 96 peer-policy runtime
bridge evidence, Phase 97 inbound metrics evidence, and Phase 98 traceability
reconciliation through existing parity roots. The canonical surface id is
`v1-9-network-participation-release-boundary`.

The current v2.0 Transaction Relay and Mempool Participation Boundary closes
the bounded local relay/mempool evidence milestone through Phase 100 relay
activation, Phase 101 inventory/download scheduling, Phase 102
orphan/admission bridging, Phase 103 mempool lifecycle and recovery, Phase 104
relay serving/fanout, Phase 105 operator evidence, and Phase 106 parity/UAT
guardrails. The canonical surface id is
`v2-0-parity-uat-release-boundary`.

The shipped v2.1 Block Serving and Compact Block Relay Boundary closes the
bounded, explicit, default-off block-serving and compact-block-relay milestone
through Phase 110 activation and eligibility, Phase 111 full-block request
handling, Phase 112 BIP152 messages, Phase 113 negotiation and announcement,
Phase 114 reconstruction, Phase 115 fallback and validation handoff, Phase 116
aggregate operator evidence, and Phase 117 parity/UAT guardrails. Its canonical
surface id is `v2-1-parity-uat-release-boundary`.

Phase 126 independently verified and promoted the six residual-hardening
requirements after removing the factless receive route, aligning compact
announcement nonce generation with the randomized Knots shell boundary, and
refreshing exact parity and deterministic regression evidence. Phases 127
through 129 then closed the remaining production integration gaps. The
[`v2.1 milestone audit`](../../.planning/milestones/v2.1-MILESTONE-AUDIT.md)
passed at 39/39 requirements, 20/20 phases, 13/13 integration links, and 11/11
end-to-end flows. v2.1 shipped and was archived on 2026-07-22. The only next
workflow route is `/gsd-new-milestone`.

This is not a production-node or production-funds claim. It does not claim
public inbound serving by default, public relay defaults, full address relay
beyond the Phase 92 boundary, package relay, BIP37 bloom-filter serving,
compact-filter serving, public block-serving or compact-relay defaults,
archive-node or production-scale historical serving, production-funds wallet
safety or use, migration apply mode, signed packaging, Windows service support,
GUI parity, hosted dashboards, public-network default checks or release gates,
automatic support-bundle upload, destructive repair, production service or
deployment, or broad production-node readiness.

Treat [`docs/parity/index.json`](index.json) as the machine-readable root,
[`docs/parity/checklist.md`](checklist.md) as the human checklist view,
[`docs/parity/catalog/p2p.md`](catalog/p2p.md) as the P2P and v1.9 network
participation catalog root, this release-readiness page as the v1.9 closeout
handoff record,
[`docs/parity/production-claim-boundary.md`](production-claim-boundary.md) as
the current v1.8 production claim boundary,
[`docs/parity/support-matrix.md`](support-matrix.md) as the canonical support
matrix and issue-evidence root,
[`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md) as the
canonical source-built upgrade and rollback policy, this release-readiness page
[`operator-runbooks.md`](operator-runbooks.md) as the canonical operator
runbook, [`service-operation-expectations.md`](service-operation-expectations.md)
as the canonical service operation expectation root, this release-readiness page
as the handoff record, and the Phase 80 plan threat model as the historical
v1.7 security boundary for the soak workflow.
[`docs/parity/threat-model-v1.6.md`](threat-model-v1.6.md),
[`docs/parity/threat-model-v1.5.md`](threat-model-v1.5.md),
[`docs/parity/threat-model-v1.4.md`](threat-model-v1.4.md), and
[`docs/parity/threat-model-v1.3.md`](threat-model-v1.3.md) remain historical
scoped threat models.

Readiness is evidence-based, not timing-threshold based. The blocking local
verification command is:

```bash
bash scripts/verify.sh
```

That command remains deterministic and public-network-free. It includes local
formatting, linting, builds, tests, benchmark smoke evidence, parity breadcrumb
checks, Bazel smoke builds, coverage, panic-site checks, and deterministic
release-boundary assertions through v2.0.
Phase 82 and Phase 83 add the narrow v1.8 production-boundary and
support-matrix traceability checks. Phase 84 adds the upgrade-policy roots that
later UPG-04 drift checks consume. Phase 85 adds the operator-runbook root for
RUN-01 through RUN-03. Phase 86 adds the service-operation root for SVC-01 and
SVC-02. Phase 87 adds this release-readiness checklist and narrow deterministic
checker for the current v1.8 release-review map, including the Phase 88
guardrail evidence rows for REL-02, REL-03, and REL-04. Phase 95 adds the
v1.9 closeout matrix here, Phase 97 closes retained inbound metrics evidence,
and Phase 98 adds `scripts/check-phase98-traceability-reconciliation.ts`
without making public-network or service operations part of default
verification.
Phase 106 adds the v2.0 parity/UAT/release-boundary guardrail checker without
making public-network relay review, service-manager operation, production
deployment, wall-clock soak, or production-funds wallet use part of default
verification.
Phase 117 adds the v2.1 aggregate parity/UAT/release-boundary checker without
making public-network block-relay review, service-manager operation,
production deployment, wall-clock soak, or production-funds wallet use part of
default verification.

## Complete Surfaces

[`docs/parity/checklist.md`](checklist.md) records these current review
surfaces as `done`:

- `reference-baseline`
- `architecture-workspace`
- `core-serialization`
- `consensus-validation`
- `chainstate`
- `mempool-policy`
- `p2p-networking`
- `wallet`
- `rpc-cli-config`
- `verification-harnesses-fuzzing`
- `drop-in-audit-migration`
- `real-sync-benchmarks`
- `operator-runtime-release-hardening`
- `live-mainnet-smoke-closeout`
- `security-analysis-audit`
- `v1-3-threat-model-release-boundaries`
- `v1-4-operator-evidence-release-boundaries`
- `v1-5-unattended-operation-release-boundaries`
- `v1-6-full-sync-completion-release-boundaries`
- `v1-7-full-sync-soak-recovery-release-boundaries`
- `v1-8-production-claim-boundary`
- `v1-8-support-matrix-issue-evidence`
- `v1-8-upgrade-rollback-policy`
- `v1-8-operator-runbooks`
- `v1-8-service-operation-expectations`
- `v1-8-release-readiness-checklist`
- `v1-8-deterministic-claim-guardrails`
- `v1-9-inbound-listener-admission-policy`
- `v1-9-peer-permissions-connection-classes`
- `v1-9-address-advertisement-discovery-boundaries`
- `v1-9-eviction-ban-misbehavior-policy`
- `v1-9-dos-resource-governance`
- `v1-9-network-participation-release-boundary`
- `v2-0-relay-activation-boundary`
- `v2-0-transaction-inventory-download-scheduling`
- `v2-0-orphan-handling-admission-outcome-bridge`
- `v2-0-mempool-chainstate-lifecycle-durable-recovery`
- `v2-0-relay-serving-fanout-rebroadcast-policy`
- `v2-0-operator-rpc-metrics-logs-support-evidence`
- `v2-0-parity-uat-release-boundary`
- `v2-1-block-serving-activation-eligibility-boundary`
- `v2-1-full-block-serving-request-path`
- `v2-1-bip152-wire-codec-message-semantics`
- `v2-1-compact-relay-negotiation-announcement-policy`
- `v2-1-compact-block-reconstruction`
- `v2-1-missing-transaction-fallback-validation-handoff`
- `v2-1-operator-block-relay-evidence`
- `v2-1-parity-uat-release-boundary`

Primary current-cycle evidence:

- [`docs/parity/production-claim-boundary.md`](production-claim-boundary.md)
  records the current v1.8 support-term glossary, production claim boundary,
  claim-to-evidence matrix, and deferred-surface inventory for PROD-01 through
  PROD-04.
- [`docs/parity/support-matrix.md`](support-matrix.md) records the canonical
  v1.8 support matrix, issue-evidence checklist, contributor update rules, and
  carried-forward residual risks for SUP-01 through SUP-04.
- [`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md) records
  the canonical source-built upgrade, rollback, backup, and compatibility
  policy for UPG-01 through UPG-04. Path:
  `docs/parity/upgrade-and-rollback-policy.md`.
- [`operator-runbooks.md`](operator-runbooks.md) records the canonical v1.8
  operator runbook for RUN-01 through RUN-03, including
  production-boundary preflight, long-run monitoring, no-progress diagnosis,
  recovery/stop decisions, redacted support-bundle timeline, and escalation
  evidence.
- [`service-operation-expectations.md`](service-operation-expectations.md)
  records the canonical v1.8 service operation expectations for SVC-01 and
  SVC-02, including source-built daemon operation, launchd/systemd preview,
  opt-in real service lifecycle UAT, restart/resume fields, repo-local
  Cargo/Bazel commands, and production-service non-claims.
- [`docs/parity/release-readiness.md`](release-readiness.md) records the
  canonical v1.8 release-readiness checklist for REL-01 through REL-06,
  including Phase 88 guardrail evidence for REL-02, REL-03, and REL-04, mapping
  all current v1.8 requirements to canonical evidence, deterministic
  verification, UAT/manual evidence, residual risk, and no-claim or next-gate
  status.
- The v1.8 deterministic claim guardrails record REL-02, REL-03, and REL-04
  as deterministic checks over the public release/operator docs, preventing
  overbroad production-readiness and deferred-surface claims without claiming
  production full-node readiness.
- [`docs/parity/catalog/p2p.md`](catalog/p2p.md), this release-readiness page,
  [`docs/parity/index.json`](index.json), and
  [`docs/parity/checklist.md`](checklist.md) record the canonical v1.9
  Network Participation Evidence and Release Boundary for BOUND-01 through
  BOUND-06, including Knots anchors for `net.cpp`, `net_processing.cpp`,
  `addrman.cpp`, `banman.cpp`, and `net_permissions.cpp`.
- [`docs/parity/catalog/p2p.md`](catalog/p2p.md),
  [`docs/parity/catalog/mempool-policy.md`](catalog/mempool-policy.md),
  [`docs/parity/catalog/rpc-cli-config.md`](catalog/rpc-cli-config.md),
  [`docs/operator/runtime-guide.md`](../operator/runtime-guide.md), this
  release-readiness page, [`docs/parity/index.json`](index.json), and
  [`docs/parity/checklist.md`](checklist.md) record the canonical v2.0
  Transaction Relay and Mempool Participation Boundary for `BOUND-01` through
  `BOUND-05`, including Knots anchors for `net_processing.cpp`,
  `txdownloadman_impl.cpp`, `txorphanage.cpp`, `txmempool.cpp`,
  `validation.cpp`, `policy.cpp`, `rpc/net.cpp`, `rpc/mempool.cpp`, and
  `rpc/rawtransaction.cpp`.
- [`docs/parity/release-readiness.md`](release-readiness.md) records this
  current v1.8 handoff plus the historical v1.7 full-sync soak and recovery
  hardening boundary matrix and traceability for SOAK-01 through REL-04.
- [`docs/parity/threat-model-v1.6.md`](threat-model-v1.6.md) records the
  historical v1.6 STRIDE register, ASVS L1 mapping, evidence acceptance, release
  boundary matrix, and requirement traceability for REL-01, REL-02, and REL-03.
- [`docs/parity/threat-model-v1.5.md`](threat-model-v1.5.md) records the
  historical v1.5 STRIDE register, ASVS L1 mapping, evidence acceptance,
  release boundary matrix, and requirement traceability for REL-01, REL-02,
  REL-03, and REL-04.
- [`docs/parity/threat-model-v1.4.md`](threat-model-v1.4.md) records the
  historical v1.4 STRIDE register, ASVS L1 mapping, evidence acceptance, release
  boundary matrix, and requirement traceability for OBS-01, OBS-02, OBS-03,
  SEC-01, SEC-02, and SEC-03.
- [`docs/parity/threat-model-v1.3.md`](threat-model-v1.3.md) records the
  historical scoped STRIDE register, evidence acceptance criteria, boundary
  matrix, and requirement traceability for PROOF-06, SEC-01, and SEC-02.
- [`docs/operator/runtime-guide.md`](../operator/runtime-guide.md) provides the
  source-built operator workflow, opt-in live-mainnet smoke commands, support
  bundle commands, redaction boundaries, and known limitations.
- [`scripts/run-live-mainnet-smoke.ts`](../../scripts/run-live-mainnet-smoke.ts)
  provides the explicit opt-in live-mainnet evidence flow and writes local JSON
  plus Markdown reports.
- [`docs/architecture/status-snapshot.md`](../architecture/status-snapshot.md)
  defines `OpenBitcoinStatusSnapshot`, the shared status model embedded in
  support evidence.
- [`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md)
  preserves deferred production-adjacent surfaces for review.
- [`scripts/verify.sh`](../../scripts/verify.sh) provides the repo-owned local
  verification contract for the release surface.

## v2.1 Block Serving and Compact Block Relay Boundary

Open Bitcoin v2.1 shipped and was archived on 2026-07-22. It provides bounded,
explicit, default-off block serving and compact-block relay. Deterministic
local review follows the Phase 110 through 117 surfaces in
[`index.json`](index.json), [`checklist.md`](checklist.md), and
[`catalog/p2p.md`](catalog/p2p.md), with consensus-validation handoff evidence
in [`catalog/consensus-validation.md`](catalog/consensus-validation.md).

The passed
[`v2.1 milestone audit`](../../.planning/milestones/v2.1-MILESTONE-AUDIT.md)
covers 39/39 requirements, 20/20 phases, 13/13 integration links, and 11/11
end-to-end flows. Future work starts only with `/gsd-new-milestone`.

Phase 116 exposes aggregate-only `block_relay` evidence across status, the
`openbitcoinnetworkstatus` RPC extension, metrics, logs, dashboard rows, and
redacted support bundles. Raw block and transaction payloads, hashes, peer ids,
endpoints, permission strings, credentials, secrets, and dynamic labels are not
part of this evidence contract.

Release review uses the Phase 117
[`checker`](../../scripts/check-phase117-parity-uat-release-boundary.ts),
[`mutation suite`](../../scripts/check-phase117-parity-uat-release-boundary.test.ts),
the repo-native [`scripts/verify.sh`](../../scripts/verify.sh), and the
[`117-UAT.md`](../../.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-UAT.md)
package. Public-network block-serving or compact-relay review is optional UAT,
may be recorded as not run, and is outside pre-commit, default CI,
release-boundary verification, and `bash scripts/verify.sh`.

Package relay, BIP37 bloom-filter serving, compact-filter serving, public
serving or relay defaults, archive-node and production-scale historical
serving, public-network CI or release gates, production service/deployment,
production full-node readiness, production-funds wallet safety or use,
packaging, GUI and hosted dashboards, migration apply mode, destructive repair,
and automatic support upload remain deferred or unsupported.

## v1.8 Production Claim Boundary

The current production-boundary root is
[`docs/parity/production-claim-boundary.md`](production-claim-boundary.md). It
defines the production full-node readiness gate vocabulary and does not claim
production full-node readiness.

The support terms are exactly `supported`, `preview`, `opt-in UAT`,
`unsupported`, and `deferred`. The canonical surface id is
`v1-8-production-claim-boundary`.

Phase 82 satisfies PROD-01, PROD-02, PROD-03, and PROD-04 by defining the
allowed production-related statement, its evidence gate, the future gates
required before broader claims are allowed, and the deferred production-adjacent
surface inventory. This section is a docs/parity traceability handoff rather
than the Phase 88 scanner. Phase 88 owns broad deterministic claim guardrails.

## v1.8 Support Matrix And Issue Evidence

The current support-matrix root is
[`docs/parity/support-matrix.md`](support-matrix.md). It records the canonical
`v1-8-support-matrix-issue-evidence` surface for SUP-01, SUP-02, SUP-03, and
SUP-04 without duplicating the matrix in this release-readiness handoff.

Release reviewers should use it to inspect support level, evidence basis,
default verification status, opt-in UAT/manual validation, residual risk, and
next gate for the supported, preview, opt-in UAT, unsupported, and deferred
surfaces. Broad all-doc claim guardrails remain Phase 88 scope per D-13.

## v1.8 Upgrade And Rollback Policy

The current upgrade-policy root is
[`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md). It records
the canonical `v1-8-upgrade-rollback-policy` surface for UPG-01, UPG-02,
UPG-03, and UPG-04 without duplicating the pre-upgrade checklist, compatibility
tables, or support matrix in this release-readiness handoff.

Release reviewers should use it for source-built rollback, backup, state/schema
compatibility, failed-upgrade evidence, and no-hidden-mutation boundaries.
Broad all-doc claim guardrails remain Phase 88 scope.

## v1.8 Operator Runbooks

The current operator-runbook root is
[`operator-runbooks.md`](operator-runbooks.md). It records the canonical
`v1-8-operator-runbooks` surface for RUN-01, RUN-02, and RUN-03 without
duplicating the runbook tables in this release-readiness handoff.

Release reviewers should use it for production-boundary preflight, long-run
monitoring, no-progress diagnosis, recovery/stop decisions, redacted
support-bundle timeline, and escalation evidence. It is procedural evidence
guidance only; public-network default checks, real service-manager defaults,
multi-day default gates, automatic support-bundle upload, destructive repair,
and broad production-node readiness remain out of scope.

## v1.8 Service Operation Expectations

The current service-expectation root is
[`service-operation-expectations.md`](service-operation-expectations.md). It
records the canonical `v1-8-service-operation-expectations` surface for SVC-01
and SVC-02 without duplicating the service classification table in this
release-readiness handoff.

Release reviewers should use it for source-built daemon operation,
launchd/systemd preview, opt-in real service lifecycle UAT, restart/resume
fields, repo-local Cargo/Bazel commands, and production-service non-claims.
The document preserves deferred packaged service distribution, Windows service
support, automatic update behavior, production service ownership, uptime
guarantees, public-network default checks, real service-manager defaults,
multi-day default gates, automatic support-bundle upload, destructive repair,
and broad production-node readiness boundaries.

## v1.8 Release Readiness Checklist

Surface id: `v1-8-release-readiness-checklist`

This checklist is the current v1.8 release-review map. It links every current
v1.8 requirement to canonical evidence, default deterministic verification,
operator UAT or manual evidence posture, residual risk, and the no-claim or
next-gate status. It does not duplicate the full support, rollback, runbook, or
service tables from their canonical documents.

| Requirement | Phase | Canonical evidence | Default verification | UAT or manual evidence | Residual risk | No-claim or next gate |
| --- | --- | --- | --- | --- | --- | --- |
| PROD-01 | Phase 82 production boundary | `docs/parity/production-claim-boundary.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `README.md` | `bun run scripts/check-phase82-production-claim-boundary.ts`; `bash scripts/verify.sh` | Reviewer inspects the allowed production-related statement and support vocabulary. | Gates are defined but not satisfied. | v1.8 defines gates only and does not claim production full-node readiness. |
| PROD-02 | Phase 82 production boundary | `docs/parity/production-claim-boundary.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/catalog/operator-runtime-release-hardening.md` | `bun run scripts/check-phase82-production-claim-boundary.ts`; `bash scripts/verify.sh` | Reviewer confirms each deferred production-adjacent surface remains deferred. | Future scope can still be misread if copied without the support terms. | Deferred surfaces need future scoped gates and Phase 88 broad claim guardrails. |
| PROD-03 | Phase 82 production boundary | `docs/parity/production-claim-boundary.md`, `docs/operator/runtime-guide.md`, `docs/parity/release-readiness.md` | `bun run scripts/check-phase82-production-claim-boundary.ts`; `bash scripts/verify.sh` | Reviewer checks that artifact existence, daemon startup, peer reachability, elapsed time, raw logs, and support bundle existence are not accepted as proof by themselves. | Evidence can still be incomplete or unavailable. | Field-based evidence and unavailable reasons are required before stronger claims. |
| PROD-04 | Phase 82 production boundary | `docs/parity/production-claim-boundary.md`, `docs/parity/index.json`, `docs/parity/checklist.md` | `bun run scripts/check-phase82-production-claim-boundary.ts`; `bash scripts/verify.sh` | Reviewer confirms the support terms remain exactly `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`. | New terms can dilute release meaning. | Future support-term changes need a scoped support-policy phase. |
| SUP-01 | Phase 83 support matrix | `docs/parity/support-matrix.md`, `docs/parity/production-claim-boundary.md`, `docs/parity/index.json` | `bun run scripts/check-phase83-support-matrix-issue-evidence.ts`; `bash scripts/verify.sh` | Reviewer inspects support classifications for source-built install, runtime, network, storage, service, wallet, migration, packaging, dashboard, GUI, support upload, destructive repair, and verification/CI. | Support labels can drift if copied into catalogs. | Support classification remains canonical in `support-matrix.md`. |
| SUP-02 | Phase 83 support matrix | `docs/parity/support-matrix.md`, `docs/operator/runtime-guide.md`, `docs/parity/deviations-and-unknowns.md` | `bun run scripts/check-phase83-support-matrix-issue-evidence.ts`; `bash scripts/verify.sh` | Issue evidence includes the smallest useful redacted evidence set or `Unavailable: <reason>`. | Reports may omit fields or include excessive raw material. | Future support promotion needs concrete evidence, verifier or opt-in UAT, residual risk, and next gate. |
| SUP-03 | Phase 83 support matrix | `docs/parity/support-matrix.md`, `docs/parity/catalog/operator-runtime-release-hardening.md`, `docs/parity/catalog/p2p.md`, `docs/parity/catalog/chainstate.md`, `docs/parity/catalog/wallet.md` | `bun run scripts/check-phase83-support-matrix-issue-evidence.ts`; `bash scripts/verify.sh` | Reviewer checks catalog pages point to the canonical support matrix instead of duplicating support rows. | Catalog prose can become stale. | Catalogs remain evidence pointers; support changes go through the matrix. |
| SUP-04 | Phase 83 support matrix | `docs/parity/support-matrix.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/release-readiness.md` | `bun run scripts/check-phase83-support-matrix-issue-evidence.ts`; `bash scripts/verify.sh` | Reviewer inspects residual risks and next gates for current and deferred surfaces. | Residual risk can be under-specified. | Future scoped phases must update residual risk and next gate before support promotion. |
| UPG-01 | Phase 84 upgrade policy | `docs/parity/upgrade-and-rollback-policy.md`, `docs/operator/runtime-guide.md`, `docs/architecture/storage-decision.md` | `bun run scripts/check-phase84-upgrade-rollback-policy.ts`; `bash scripts/verify.sh` | Reviewer inspects pre-upgrade evidence and source-built command provenance. | Evidence collection can be incomplete on local hosts. | Upgrade guidance remains review-only and source-built. |
| UPG-02 | Phase 84 upgrade policy | `docs/parity/upgrade-and-rollback-policy.md`, `docs/parity/support-matrix.md`, `docs/parity/deviations-and-unknowns.md` | `bun run scripts/check-phase84-upgrade-rollback-policy.ts`; `bash scripts/verify.sh` | Reviewer checks compatibility decisions use recovery vocabulary and unavailable reasons. | State/schema evidence can be unavailable or inconclusive. | Hidden datadir, wallet, service, or config mutation remains forbidden without future scope. |
| UPG-03 | Phase 84 upgrade policy | `docs/parity/upgrade-and-rollback-policy.md`, `docs/parity/production-claim-boundary.md`, `docs/parity/release-readiness.md` | `bun run scripts/check-phase84-upgrade-rollback-policy.ts`; `bash scripts/verify.sh` | Reviewer verifies rollback and backup steps are review-only unless a future mutation plan exists. | Operators can still choose local actions outside the docs. | Automatic backup/restore, destructive repair, signed packaging, and automatic updates need separate gates. |
| UPG-04 | Phase 84 upgrade policy | `docs/parity/upgrade-and-rollback-policy.md`, `scripts/check-phase84-upgrade-rollback-policy.ts`, `scripts/verify.sh` | `bun run scripts/check-phase84-upgrade-rollback-policy.ts`; `bash scripts/verify.sh` | Reviewer confirms upgrade-policy drift checks remain deterministic and default-local. | Broad all-doc production-claim scanning is not in Phase 84. | Phase 88 owns broad deterministic claim guardrails. |
| RUN-01 | Phase 85 operator runbooks | `docs/parity/operator-runbooks.md`, `docs/operator/runtime-guide.md`, `docs/parity/support-matrix.md` | `bun run scripts/check-phase85-operator-runbooks.ts`; `bash scripts/verify.sh` | Reviewer inspects production-boundary preflight and long-run monitoring evidence fields. | Live public-network behavior remains environment-dependent. | Public-network default checks and production-node readiness remain future-scoped. |
| RUN-02 | Phase 85 operator runbooks | `docs/parity/operator-runbooks.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md` | `bun run scripts/check-phase85-operator-runbooks.ts`; `bash scripts/verify.sh` | Reviewer checks no-progress diagnosis, recovery or stop decisions, and escalation evidence. | Diagnosis can be inconclusive when fields are unavailable. | Recovery remains diagnosis and evidence only, not destructive repair. |
| RUN-03 | Phase 85 operator runbooks | `docs/parity/operator-runbooks.md`, `docs/parity/support-matrix.md`, `docs/operator/runtime-guide.md` | `bun run scripts/check-phase85-operator-runbooks.ts`; `bash scripts/verify.sh` | Reviewer checks redacted support-bundle timeline and support escalation guidance. | Sensitive evidence can be over-shared if docs are ignored. | Automatic support-bundle upload and hosted support workflows need future gates. |
| SVC-01 | Phase 86 service expectations | `docs/parity/service-operation-expectations.md`, `docs/parity/support-matrix.md`, `docs/parity/production-claim-boundary.md` | `bun run scripts/check-phase86-service-operation-expectations.ts`; `bash scripts/verify.sh` | Reviewer inspects service surface classification and support terms. | Preview or opt-in UAT service evidence can be overread as production support. | Production service ownership, uptime guarantees, packaged services, and Windows service support remain deferred. |
| SVC-02 | Phase 86 service expectations | `docs/parity/service-operation-expectations.md`, `docs/operator/runtime-guide.md`, `docs/parity/operator-runbooks.md` | `bun run scripts/check-phase86-service-operation-expectations.ts`; `bash scripts/verify.sh` | Reviewer checks repo-local Cargo/Bazel command evidence, restart/resume fields, and service unavailable reasons. | Host service-manager behavior varies. | Real service-manager lifecycle remains opt-in UAT outside default verification. |
| REL-01 | Phase 87 release readiness | `docs/parity/release-readiness.md`, `docs/parity/index.json`, `docs/parity/checklist.md` | `bun run scripts/check-phase87-release-readiness.ts`; `bash scripts/verify.sh` | Reviewer uses this checklist as the current v1.8 release handoff. | Checklist rows can become stale when future phases add gates. | Future release-readiness updates must refresh this checklist and parity roots. |
| REL-02 | Phase 88 deterministic claim guardrails | `docs/parity/release-readiness.md`, `docs/parity/production-claim-boundary.md`, `docs/parity/support-matrix.md`, `docs/parity/deviations-and-unknowns.md`, `scripts/check-phase88-deterministic-claim-guardrails.ts`, `scripts/check-phase88-deterministic-claim-guardrails.test.ts` | `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts`; `bun run scripts/check-phase88-deterministic-claim-guardrails.ts`; `bash scripts/verify.sh` | Reviewer inspects scoped no-claim wording and checker output only; public-network, real service-manager, and multi-day checks remain outside default verification. | The production full-node readiness claim remains gated by future evidence and cannot be inferred from artifact existence or daemon startup. | v1.8 defines gates only and does not claim production full-node readiness. |
| REL-03 | Phase 88 deterministic claim guardrails | `docs/parity/deviations-and-unknowns.md`, `docs/parity/upgrade-and-rollback-policy.md`, `docs/parity/operator-runbooks.md`, `docs/parity/service-operation-expectations.md`, `scripts/check-phase88-deterministic-claim-guardrails.ts`, `scripts/check-phase88-deterministic-claim-guardrails.test.ts` | `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts`; `bun run scripts/check-phase88-deterministic-claim-guardrails.ts`; `bash scripts/verify.sh` | Reviewer confirms deferred surfaces remain scoped as deferred, unsupported, opt-in UAT, outside default verification, historical, or future-gated. | Deferred-surface wording can still drift if copied into docs outside the curated corpus. | Inbound serving, relay, production-funds wallet use, migration apply mode, signed packaging, hosted dashboards, GUI parity, public-network CI, destructive repair, and automatic support-bundle upload need separate scoped gates before promotion. |
| REL-04 | Phase 88 deterministic claim guardrails | `scripts/verify.sh`, `scripts/check-phase87-release-readiness.ts`, `scripts/check-phase88-deterministic-claim-guardrails.ts`, `docs/parity/release-readiness.md` | `bun test scripts/check-phase87-release-readiness.test.ts`; `bun run scripts/check-phase87-release-readiness.ts`; `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts`; `bun run scripts/check-phase88-deterministic-claim-guardrails.ts`; `bash scripts/verify.sh` | Public-network, real service-manager, and multi-day checks remain opt-in evidence only. | The default verifier proves deterministic guardrail coverage, not live-network production readiness. | Default verification must stay deterministic unless a later release-policy decision changes it. |
| REL-05 | Phase 87 release readiness | `README.md`, `docs/parity/README.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, `docs/parity/catalog/operator-runtime-release-hardening.md` | `bun run scripts/check-phase87-release-readiness.ts`; `bash scripts/verify.sh` | Reviewer follows compact entrypoint links to the checklist without duplicated matrices. | Entrypoints can drift from canonical docs. | Future docs changes should keep compact pointers and avoid duplicate checklist tables. |
| REL-06 | Phase 87 release readiness | `docs/parity/release-readiness.md`, `docs/parity/production-claim-boundary.md`, `docs/parity/deviations-and-unknowns.md` | `bun run scripts/check-phase87-release-readiness.ts`; `bash scripts/verify.sh` | Reviewer confirms the no-claim review preserves production full-node readiness and deferred-surface boundaries. | Broad overclaim scanning is intentionally not implemented here. | Phase 88 owns REL-02, REL-03, and REL-04 broad deterministic claim guardrails. |

Required deterministic reviewer commands:

```bash
bun test scripts/check-phase87-release-readiness.test.ts
bun run scripts/check-phase87-release-readiness.ts
bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts
bun run scripts/check-phase88-deterministic-claim-guardrails.ts
bash scripts/verify.sh
```

## v1.9 Network Participation Evidence and Release Boundary

Surface id: `v1-9-network-participation-release-boundary`

This closeout is the current v1.9 release-review map for bounded network
participation evidence. It links BOUND-01 through BOUND-06 to canonical parity
roots, deterministic verification, opt-in UAT posture, residual risk, and the
no-claim or next gate. It reuses existing roots instead of creating a separate
release manifest.

| Requirement | Canonical evidence | Deterministic verification | Opt-in UAT posture | Residual risk | No-claim or next gate |
| --- | --- | --- | --- | --- | --- |
| BOUND-01 | `docs/parity/catalog/p2p.md`, `docs/parity/checklist.md`, `docs/parity/release-readiness.md`, `docs/parity/production-claim-boundary.md`, `docs/parity/support-matrix.md` | Plan 04 aggregate checker path: `bun run scripts/check-phase95-network-participation-release-boundary.ts`; current focused checks parse `docs/parity/index.json` and scan this root. | Reviewers use the runtime-guide loopback or synthetic inbound commands only when they deliberately opt in. | Broad release language can still drift until the aggregate checker is wired. | v1.9 does not claim transaction relay, compact block relay, mempool propagation, public inbound defaults, production service operation, or production full-node readiness. |
| BOUND-02 | `docs/parity/catalog/p2p.md`, `docs/parity/index.json`, and Knots anchors for `packages/bitcoin-knots/src/net.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/addrman.cpp`, `packages/bitcoin-knots/src/banman.cpp`, and `packages/bitcoin-knots/src/net_permissions.cpp` | Focused `rg` anchor checks in Plan 02; Plan 04 will make these aggregate checker assertions. | UAT remains source-built review evidence; Knots anchors are audit references, not live Knots process execution. | Future implementation can add new anchors and must update this surface. | Missing or changed anchors need a scoped parity-doc update before a broader claim. |
| BOUND-03 | Phase 90 through Phase 94 summaries, `scripts/verify.sh`, `docs/parity/production-claim-boundary.md`, and historical release-readiness sections | `bash scripts/verify.sh` remains the repo-native non-regression contract after Phase 95 checker wiring. | Public-network full-sync, soak, real service-manager, and live support-bundle collection remain optional operator evidence only. | Default verification cannot prove public-network reachability or production uptime. | Existing outbound sync, full-sync, soak, support-bundle, production no-claim, and release-boundary behavior must remain non-regressed before any stronger release gate. |
| BOUND-04 | `docs/operator/runtime-guide.md`, `docs/parity/checklist.md`, and Phase 90 through Phase 94 summaries | Plan 04 aggregate checker verifies repo-local Cargo and Bazel command forms. | Operators use copy-pasteable Cargo and Bazel loopback/synthetic commands; installed aliases are not the only path. | Local hosts can still lack ports, permissions, or build cache state. | Public-network listener exposure and CI remain future-scoped. |
| BOUND-05 | `packages/open-bitcoin-cli/src/operator/support/redaction.rs`, `packages/open-bitcoin-cli/src/operator/support/tests.rs`, `docs/parity/support-matrix.md`, and Phase 95 Plan 01 summary | Rust support tests and the Phase 95 aggregate checker guard redaction boundaries. | Support bundles are local redacted evidence for diagnosis, not public uploads or release validators by themselves. | New support fields can leak raw peer material unless the redaction tests and checker are kept current. | Automatic support-bundle upload and raw peer/payload/permission evidence remain out of scope. |
| BOUND-06 | `.planning/milestones/v1.9-REQUIREMENTS.md`, `.planning/milestones/v1.9-ROADMAP.md`, `.planning/milestones/v1.9-MILESTONE-AUDIT.md`, `.planning/phases/98-traceability-reconciliation/98-VERIFICATION.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, and this release matrix | `scripts/check-phase98-traceability-reconciliation.ts` verifies archived Phase 90 through Phase 98 traceability, selected verification notes, audit closure text, and verifier wiring after Phase 97. | Milestone audit and UAT reviewers should cite these roots instead of adding a parallel traceability manifest. | Phase 98 and the archived v1.9 milestone audit are now the canonical traceability closeout for Phase 98-owned requirements. | v1.9 requirement traceability stays exactly once across Phase 90 through Phase 98; new requirement IDs need a future roadmap update. |

Required deterministic reviewer commands for this closeout:

```bash
bun -e 'JSON.parse(await Bun.file("docs/parity/index.json").text()); console.log("index json ok")'
rg -n "v1-9-network-participation-release-boundary|BOUND-01|BOUND-06|Network Participation Evidence and Release Boundary" docs/parity/checklist.md docs/parity/catalog/p2p.md docs/parity/release-readiness.md
bash scripts/verify.sh
```

Phase 98 owns the final traceability reconciliation checker:
`scripts/check-phase98-traceability-reconciliation.ts` and
`scripts/check-phase98-traceability-reconciliation.test.ts`. Until the final
Phase 98 verification report lands, reviewers should treat those paths as the
next deterministic gate, not as a separate evidence manifest.

## v1.8 Release Readiness No-Claim Review

v1.8 is a boundary-setting milestone: it defines gates only and does not claim
production full-node readiness. It does not claim production service operation,
inbound serving, address relay, block serving, transaction relay, compact block
relay, production-funds wallet use or safety, migration apply mode, signed
packaging or package-manager distribution, Windows service integration, hosted
dashboards, GUI parity, public-network default checks, public-network CI,
release-blocking live sync, destructive repair, automatic support-bundle upload,
or broad production-node readiness.

Artifact existence, daemon startup, elapsed time, peer reachability, raw log
tail, service file existence, and support bundle path are context only. Release
reviewers must use named fields, unavailable reasons, canonical evidence roots,
and deterministic checker output before accepting any scoped claim.

Phase 88 owns REL-02, REL-03, and REL-04 broad deterministic claim guardrails.
Phase 87 deliberately adds the release-readiness checklist and narrow checker
only; it does not scan every documentation file for every possible overclaim.

## v1.7 Full-Sync Soak and Recovery Hardening Claim Boundary Matrix

| Surface | v1.7 Proven Claim | Accepted Evidence | Explicit Non-Claim | Required Next Milestone Or Deferred Gate | Requirement IDs |
| --- | --- | --- | --- | --- | --- |
| Phase 75 multi-day soak runner and evidence ledger | Operators can run explicit opt-in full-sync soaks with durable run identity, resumable ledger/report state, bounded stop conditions, and deterministic synthetic replay. | Phase 75 summaries, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `README.md`, and `scripts/check-phase75-soak-runner.ts`. | No default multi-day gate, inbound serving, relay, production-funds wallet use, migration apply mode, signed packaging, GUI, hosted dashboard, or broad production-node readiness claim. | Future production-node and release-policy milestones. | SOAK-01, SOAK-02, SOAK-03, SOAK-04 |
| Phase 76 disk and resource-bound enforcement | Operators can inspect disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle bounds before and during long soaks, with typed stop guidance before unsafe pressure. | Phase 76 summaries, `resource_bounds`, soak preflight/report evidence, support `Resource Bound Evidence`, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/index.json`, and `scripts/check-phase76-resource-bounds.ts`. | No public-network resource stress, unlimited unattended operation, hosted monitoring, raw support upload, full resource-governance parity, or production-node readiness claim. | Future production resource policy and signed comparable soak artifacts. | RES-05, RES-06, RES-07, RES-08 |
| Phase 77 corruption and lock recovery hardening | Operators can diagnose lock contention, stale-lock evidence, concurrent datadir use, corruption markers, schema mismatches, partial writes, unreadable stores, and backend-open failures with typed non-mutating recovery guidance. | Phase 77 summaries, `recovery_evidence`, lock-probe evidence, status/support/dashboard/soak projections, `docs/operator/runtime-guide.md`, `docs/architecture/storage-decision.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, and `scripts/check-phase77-corruption-lock-recovery.ts`. | No automatic destructive repair, lock cleanup, source datadir mutation, process scanning as required evidence, public-network default checks, or production-node readiness claim. | Future destructive repair, lock cleanup, source mutation, portable process attribution, public-network recovery UAT, and production-node readiness gates. | REC-05, REC-06, REC-07, REC-08 |
| Phase 78 progress guarantees and stall diagnosis | Soak progress is credited only for validated durable work or explicit stay-current evidence, and stalled paths expose last useful work, peer contribution, expected progress windows, thresholds, subsystem, cause, and next action. | Phase 78 summaries, shared status fields, soak/support/dashboard/live-smoke projections, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/index.json`, and `scripts/check-phase78-progress-guarantees.ts`. | No false-progress acceptance, public-network default checks, multi-day default gates, support-bundle forensics claim by itself, inbound serving, relay, production-wallet use, migration apply mode, packaging, GUI, hosted dashboards, or production-node readiness claim. | Future production-node progress policy and public-network UAT automation decisions. | PROG-01, PROG-02, PROG-03, PROG-04 |
| Phase 79 diagnostics and support-bundle forensics | Operators can inspect why a soak or support handoff passed, failed, or remained inconclusive through redacted local support forensics, timeline, checkpoint chain, narrative, evidence basis, confidence, next action, size bounds, and cross-surface consistency. | Phase 79 verification, `support_forensics`, support JSON/Markdown projections, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/index.json`, and `scripts/check-phase79-diagnostics-support-bundle.ts`. | No artifact-existence proof, raw-log proof, elapsed-time proof, hosted upload, automatic support-bundle upload, public-network default check, multi-day default gate, or production-node readiness claim. | Future signed/comparable support artifacts or hosted-support design. | DIAG-01, DIAG-02, DIAG-03, DIAG-04 |
| Phase 80 opt-in soak UAT and release boundaries | Reviewers can audit the v1.7 closeout through source-built opt-in UAT commands, existing parity roots, source breadcrumbs, support schema anchors, deterministic checkers, operator docs, and release-boundary wording. | `docs/operator/runtime-guide.md`, this matrix, `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/catalog/operator-runtime-release-hardening.md`, `docs/parity/source-breadcrumbs.json`, `scripts/check-parity-breadcrumbs.ts`, the Phase 75 through Phase 80 deterministic checkers, `scripts/verify.sh`, and `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md`. | No new evidence manifest, broad all-doc scanner, new v1.7 threat-model file, public-network default verification, release-blocking live sync, signed packaging, GUI, hosted dashboards, or broad production-node readiness claim. | Future release-engineering, production-node, packaging, hosted-operations, GUI, or public-network CI milestones. | VER-05, VER-06, VER-07, REL-04 |
| Default deterministic verification | Contributors can run the repo-native default verifier without internet access, public peers, real service managers, multi-day sleeps, current-tip timing, large disk allocation, or live release-blocking gates. | `scripts/verify.sh`, Phase 75 through Phase 80 deterministic checkers, checker fixture tests, `scripts/check-parity-breadcrumbs.ts --check`, benchmark smoke checks, Bazel smoke builds, Rust tests, coverage, and panic-site checks. | No public-network CI, current-tip SLA, real service-manager proof, process-table proof, public-network default checks, multi-day wall-clock gate, large-disk stress gate, or release-blocking live sync. | Future release-policy decision if public-network or long-run automation becomes default. | VER-05, REL-04 |
| Parity roots, source breadcrumbs, support schema anchors, deterministic checkers, and operator docs | Reviewers can audit v1.7 evidence from existing roots instead of a new evidence registry. New first-party Rust source/test traceability remains under `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts --check`; support and soak schema anchors remain typed in first-party code and guarded by deterministic checkers. | `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/source-breadcrumbs.json`, `scripts/check-parity-breadcrumbs.ts`, Phase 75 through Phase 80 checker scripts, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, and support/soak typed projections. | No duplicate manifest-driven evidence registry, no broad all-doc release scanner, no unchecked Rust source/test addition, and no claim that support bundles are release validators by themselves. | Future evidence-manifest design only if a later milestone deliberately chooses it. | VER-07, REL-04 |
| Explicit deferred production-node surfaces | v1.7 explicitly preserves deferred production-adjacent scope while hardening the opt-in soak and recovery workflow. | This matrix, `docs/parity/deviations-and-unknowns.md`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/catalog/operator-runtime-release-hardening.md`, `docs/parity/catalog/p2p.md`, and `docs/operator/runtime-guide.md`. | v1.7 does not claim inbound serving, address relay, block serving, transaction relay, compact block relay, production-funds wallet safety, migration apply mode, signed packaging, Windows service support, GUI parity, hosted dashboards, public-network default checks, public-network CI, release-blocking live sync, automatic support-bundle upload, destructive repair, or broad production-node readiness. | Future PNODE, wallet-production, migration-apply, packaging, Windows service, GUI, hosted dashboard, public-network CI, support-upload, and destructive-repair milestones. | REL-04 |

Final v1.7 traceability covers all 24 milestone requirement IDs: SOAK-01,
SOAK-02, SOAK-03, SOAK-04, RES-05, RES-06, RES-07, RES-08, REC-05, REC-06,
REC-07, REC-08, PROG-01, PROG-02, PROG-03, PROG-04, DIAG-01, DIAG-02,
DIAG-03, DIAG-04, VER-05, VER-06, VER-07, and REL-04.

Required deterministic reviewer commands:

```bash
bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts
bun run scripts/check-parity-breadcrumbs.ts --check
bash scripts/verify.sh
```

Operator UAT commands remain explicit opt-in workflows in
[`docs/operator/runtime-guide.md`](../operator/runtime-guide.md). Generated
soak reports, support bundles, daemon logs, metrics stores, compatibility
reports, live-mainnet reports, and local datadirs stay local and out of git.

## v1.6 Full-Sync Completion Claim Boundary Matrix

| Surface | v1.6 Proven Claim | Accepted Evidence | Explicit Non-Claim | Required Next Milestone Or Deferred Gate | Requirement IDs |
| --- | --- | --- | --- | --- | --- |
| Explicit opt-in full-sync completion | `open-bitcoind` can be explicitly opted into source-built mainnet full-sync review, and evidence can show validated active-chain progress to the best-known peer tip. | Phase 68 through Phase 73 verification, `docs/operator/runtime-guide.md`, `docs/parity/threat-model-v1.6.md`, this matrix, and `scripts/check-v1.6-release-boundaries.ts`. | No broad production-node readiness, release-blocking live sync, public-network CI, packaged-service guarantee, or timing-threshold release gate. | Future production-node and release-policy milestones. | REL-01, REL-02, REL-03 |
| Active-chain validation and durable persistence | Sync progress only counts after consensus validation and durable connection to the active chain. | Phase 68 evidence, durable UTXO/undo, block-index, runtime metadata, active-chain status fields. | No block serving, assumeutxo, assumevalid, pruning, snapshot bootstrap, or shortcut validation claim. | Future production-node chainstate policy. | SYNC-01, SYNC-02, SYNC-03, SYNC-04, REL-01 |
| Tip tracking and stay-current operation | Operators can inspect best-known tip source, freshness, peer agreement, current/stale/recovering state, and stay-current progress. | Phase 69 evidence, shared status contract, runtime guide UAT matrix. | No current-tip timing SLA, hidden tip oracle, or release-blocking public-network freshness gate. | Future release-policy decision. | TIP-01, TIP-02, TIP-03, REL-03 |
| Reorg, peer rotation, and no-progress recovery | Operators can inspect cumulative-work reorg outcomes, peer-attributed failures, retry/backoff, no-progress causes, and next actions. | Phase 70 evidence, `sync.latest_reorg`, `sync.no_progress_diagnosis`, `sync.no_progress_next_action`. | No inbound peer governance, address relay, peer banning, transaction relay, or compact block relay claim. | Future peer-governance and relay milestones. | REC-01, REC-02, REC-03, REC-04, REL-01, REL-02 |
| Resource bounds and durable restart/resume | Long sync attempts have documented/tested bounds and same-datadir recovery evidence. | Phase 71 evidence, resource-pressure fields, restart/resume matrix, storage-pressure recovery guidance. | No unlimited unattended operation, production resource policy, or automatic repair guarantee. | Future production resource governance. | RES-01, RES-02, RES-03, RES-04, REL-01 |
| Observability and support evidence | CLI status, dashboard, RPC, metrics, logs, live-smoke, and support bundles share one full-sync truth contract and redacted support evidence. | Phase 72 evidence, `OpenBitcoinStatusSnapshot`, `support-evidence.json`, `support-evidence.md`, architecture docs. | No hosted dashboards, remote administration, raw support upload, production-funds wallet safety, or support-bundle-as-release-validator claim. | Future hosted operations and support-artifact design. | OBS-01, OBS-02, OBS-03, OBS-04, REL-03 |
| Opt-in UAT and deterministic verification | Public-mainnet full-sync, stay-current, restart/resume, and support-bundle UAT commands are copy-pasteable and remain outside default verification. | Phase 73 evidence, runtime-guide UAT matrix, `scripts/check-phase73-uat-verification.ts`, `scripts/check-parity-breadcrumbs.ts`. | No public-network CI, release-blocking live sync, manual-peer default gate, or real service-manager default gate. | Future release-policy decision. | VER-01, VER-02, VER-03, VER-04, REL-02, REL-03 |
| Parity roots, README, and operator docs | Reviewers can find the current v1.6 threat model, release-readiness matrix, machine root, human checklist, README posture, runtime-guide evidence interpretation, and catalog boundaries. | `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `README.md`, `docs/operator/runtime-guide.md`, catalog pages. | Prior v1.3, v1.4, and v1.5 docs remain historical evidence, not the current v1.6 claim. | Future milestone roots when scope expands. | REL-01, REL-02, REL-03 |
| Inbound serving and address relay | No shipped v1.6 claim. | Deferred-scope rows in this matrix, `deviations-and-unknowns.md`, `catalog/p2p.md`. | v1.6 does not claim inbound serving or address relay. | Future PNODE-01/PNODE-02. | REL-02 |
| Block serving, transaction relay, and compact block relay | No shipped v1.6 claim. | Deferred-scope rows in this matrix, `deviations-and-unknowns.md`, `catalog/p2p.md`. | v1.6 does not claim block serving, transaction relay, or compact block relay. | Future PNODE-02/PNODE-03. | REL-02 |
| Production-funds wallet safety | No shipped v1.6 claim. | Deferred-scope rows in this matrix and runtime-guide limitations. | v1.6 does not claim production-funds wallet safety. | Future wallet production milestone. | REL-02 |
| Migration apply mode | No shipped v1.6 claim. | Dry-run migration docs, deferred-scope rows, parity deviations. | v1.6 does not claim migration apply mode, source-service cutover, or source-datadir mutation. | Future migration apply safety design. | REL-02 |
| Signed packaging and Windows service support | No shipped v1.6 claim. | Source-built install docs and deferred-scope rows. | v1.6 does not claim signed packaging, OS distribution, or Windows service support. | Future packaging milestones. | REL-02 |
| GUI parity and hosted dashboards | No shipped v1.6 claim. | Headless and terminal-first docs. | v1.6 does not claim GUI parity or hosted dashboards. | Future product-surface milestones. | REL-02 |

## v1.7 Phase 77 Recovery-Hardening Evidence Anchor

Within the full v1.7 closeout above, Phase 77 remains a scoped
recovery-hardening evidence anchor. It documents REC-05, REC-06, REC-07, and
REC-08 around typed lock and corruption diagnosis from `recovery_evidence`,
probe-only lock evidence, status/support/dashboard/soak projections, and
operator guidance.

| Surface | Phase 77 Scoped Claim | Accepted Evidence | Explicit Non-Claim | Required Next Milestone Or Deferred Gate | Requirement IDs |
| --- | --- | --- | --- | --- | --- |
| Corruption and lock recovery hardening | Operators can diagnose lock contention, stale-lock evidence, concurrent datadir use, corruption markers, schema mismatches, partial writes, unreadable stores, and backend-open failures with typed recovery guidance. | `packages/open-bitcoin-node/src/recovery.rs`, `packages/open-bitcoin-node/src/storage/lock_probe.rs`, `packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs`, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/storage-decision.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, and `scripts/check-phase77-corruption-lock-recovery.ts`. | No automatic destructive repair, lock cleanup, source datadir mutation, process scanning as required evidence, public-network default checks, or production-node readiness. | Future destructive repair, lock cleanup, source mutation, portable process attribution, public-network recovery UAT, and production-node readiness gates. | REC-05, REC-06, REC-07, REC-08 |

Final v1.6 traceability covers all 26 milestone requirement IDs:
SYNC-01, SYNC-02, SYNC-03, SYNC-04, TIP-01, TIP-02, TIP-03, REC-01,
REC-02, REC-03, REC-04, RES-01, RES-02, RES-03, RES-04, OBS-01, OBS-02,
OBS-03, OBS-04, VER-01, VER-02, VER-03, VER-04, REL-01, REL-02, and REL-03.

Required deterministic reviewer commands:

```bash
bun run scripts/check-v1.6-release-boundaries.ts
bun run scripts/check-phase73-uat-verification.ts
bash scripts/verify.sh
```

Operator UAT commands remain the Phase 73 matrix in
[`docs/operator/runtime-guide.md`](../operator/runtime-guide.md). Generated
live-mainnet reports, support bundles, daemon logs, metrics stores,
compatibility reports, and local datadirs stay local and out of git.

## v1.5 Unattended Operation Claim Boundary Matrix

| Surface | v1.5 Proven Claim | Accepted Evidence | Explicit Non-Claim | Required Next Milestone Or Deferred Gate | Requirement IDs |
| --- | --- | --- | --- | --- | --- |
| Unattended mainnet sync loop | `open-bitcoind` can be explicitly opted into bounded mainnet IBD review with durable pause/resume/shutdown and stop-reason evidence. | Phase 60 verification, durable sync status, runtime guide commands. | No default-on public-network operation or broad production full-node support. | Future PRODNODE-01 production-node milestone. | REL-01, REL-02 |
| Resource bounds and recovery states | Operators can inspect sync resource pressure, retry/backoff state, recovery category, recovery action, and next steps. | Phase 61 verification, `sync.resource_pressure`, `sync.recovery_category`, support evidence. | No guarantee of unlimited unattended operation or production resource policy. | Future resource-governance policy. | REL-01 |
| Disk and resource-bound enforcement | Operators can inspect typed `resource_bounds` for disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle evidence; soak preflight refuses stop-required or unavailable required evidence before ledger mutation and reports `resource_stop` source evidence. | Phase 76 verification, `resource_bounds`, soak reports, support `Resource Bound Evidence`, deterministic checker. | No public-network resource stress, unlimited unattended operation, hosted monitoring, raw support upload, broad production-node readiness, or full resource-governance parity. | Future production resource policy and signed comparable soak artifacts. | RES-05, RES-06, RES-07, RES-08 |
| Long-run truth surfaces | Status, dashboard, RPC, metrics, logs, live smoke, and support evidence share lifecycle, phase, target, progress, recovery, pressure, peer, and block evidence fields. | Phase 62 verification, `OpenBitcoinStatusSnapshot`, metrics/log docs. | No hosted dashboard, remote administration, or external monitoring product claim. | Future hosted operations design. | REL-01 |
| User-level service supervision | Operators can preview and manage launchd/systemd user services for `open-bitcoind` with selected datadir/config paths. | Phase 63 verification, service preview/install/status docs. | No signed packaging, machine-wide service support, Windows service support, or uptime SLA. | PKG-01 and PKG-02. | REL-01, REL-02 |
| Same-datadir service restart/resume | Operators can review same-datadir restart/resume evidence through `service.restart_resume`. | Phase 64 verification, status JSON, runtime guide UAT. | No automatic recovery-loop policy or mandatory fresh post-restart network progress in default verification. | Future PRODNODE-01 recovery gate. | REL-01 |
| Redacted support evidence | Operators can generate local support bundles with compact redacted status, metrics, log, service, restart/resume, and compatibility summaries. | Phase 65 verification, `support-evidence.json`, `support-evidence.md`. | No hosted support upload, raw log bundle, public release validator, or wallet-secret export. | Future artifact validator or hosted support design. | REL-01, REL-02 |
| Compatibility wrapper reports | Operators can generate local compatibility transcript reports through `open-bitcoin compatibility harness`. | Phase 66 verification, `compatibility-harness-report.json`, `compatibility-harness-report.md`. | No live public-peer proof, inbound serving, transaction relay, or compact block relay. | Future public-peer probing and relay milestones. | REL-01, REL-02 |
| Threat model and release boundary docs | Reviewers can inspect current v1.5 STRIDE/ASVS coverage, checklist roots, and machine-readable parity roots without promoting historical v1.3/v1.4 claims. | `docs/parity/threat-model-v1.5.md`, `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, `docs/parity/README.md`. | v1.3/v1.4 docs remain historical evidence and are not promoted into current v1.5 claims. | Future milestone threat model and parity roots when scope expands. | REL-01, REL-02, REL-04 |
| Deterministic verification | Default verification remains deterministic, public-network-free, and real-service-manager-free while checking v1.5 boundaries. | `bash scripts/verify.sh`, `scripts/check-v1.5-release-boundaries.ts`, parity docs. | No public-network CI gate, default live-smoke run, manual peer probing, `--restart-after-progress`, real `systemctl --user`, or real `launchctl`. | Future release-policy decision if live evidence becomes a stronger gate. | REL-03, REL-04 |
| Inbound serving | No shipped v1.5 claim. | Deferred-surface rows in this matrix, `deviations-and-unknowns.md`, `catalog/p2p.md`. | v1.5 does not claim inbound serving or address advertisement. | Future PRODNODE-02. | REL-02 |
| Transaction relay | No shipped v1.5 claim. | Deferred-surface rows in this matrix, `deviations-and-unknowns.md`, `catalog/p2p.md`. | v1.5 does not claim transaction relay or mempool propagation behavior. | Future PRODNODE-03. | REL-02 |
| Compact block relay | No shipped v1.5 claim. | Deferred-surface rows in this matrix, `deviations-and-unknowns.md`, `catalog/p2p.md`. | v1.5 does not claim compact block relay. | Future relay parity milestone. | REL-02 |
| Production-funds wallet use | No shipped v1.5 claim. | Deferred-surface rows in this matrix and runtime-guide known limitations. | v1.5 does not claim production-funds wallet use. | Future WALPROD-01 threat model and parity evidence. | REL-02 |
| Migration apply mode | No shipped v1.5 claim. | Dry-run migration docs, deferred-surface rows, parity deviations. | v1.5 does not claim migration apply mode, source-service cutover, or source-datadir mutation. | Future MIGAPPLY-01 safety design. | REL-02 |
| Packaging distribution | No shipped v1.5 claim. | Source-built install docs, deferred-surface rows. | v1.5 does not claim packaging, signed installers, or canonical OS distribution. | Future PKG-01. | REL-02 |
| Hosted dashboard | No shipped v1.5 claim. | Local dashboard docs, deferred-surface rows. | v1.5 does not claim hosted dashboard or public dashboard operation. | Future hosted operations design. | REL-02 |
| GUI | No shipped v1.5 claim. | Headless and terminal-first docs. | v1.5 does not claim GUI parity with the reference Qt app. | Future GUI milestone. | REL-02 |
| Windows service support | No shipped v1.5 claim. | Source-built local service docs, deferred-surface rows. | v1.5 does not claim Windows service support or certification. | Future PKG-02. | REL-02 |

Required deterministic reviewer commands:

```bash
bun run scripts/check-v1.5-release-boundaries.ts
bash scripts/verify.sh
```

Operator UAT commands that intentionally remain outside deterministic default
verification:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status live-smoke --network mainnet --extended --manual-peer 127.0.0.1:8333 --output /tmp/open-bitcoin-live-smoke
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- service restart --restart-after-progress --evidence-dir /tmp/open-bitcoin-service-uat
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status live-smoke -- --network mainnet --extended --manual-peer 127.0.0.1:8333 --output /tmp/open-bitcoin-live-smoke
```

These UAT commands require an operator-controlled environment. They are
evidence-gathering commands, not release preconditions for deterministic
verification.

## v1.4 Operator Evidence Claim Boundary Matrix

| Surface | v1.4 Proven Claim | Accepted Evidence | Explicit Non-Claim | Required Next Milestone Or Deferred Gate | Requirement IDs |
| --- | --- | --- | --- | --- | --- |
| Outbound peer compatibility | Open Bitcoin can distinguish compatible outbound peers, incompatible peers, idle peers, failed peers, and useful-contribution peers in deterministic and opt-in live evidence. | Compatibility harness evidence, durable peer telemetry, live-smoke peer outcomes, support evidence, `scripts/run-live-mainnet-smoke.ts`. | No inbound serving, address advertisement, peer eviction, or ban-policy production claim. | Future PRODNODE-02 inbound-serving and peer-governance milestone. | OBS-01, SEC-01, SEC-02 |
| Header progress | Opt-in live smoke can record the first validated header-height increase with before/after fresh status. | `result.firstHeaderProgress`, live-smoke JSON/Markdown, `OpenBitcoinStatusSnapshot`, runtime guide commands. | Header progress is not connected block progress, full IBD completion, or production readiness. | Future full-IBD completion milestone if the release claim expands. | OBS-01, OBS-03, SEC-01 |
| Downloaded block progress | Opt-in live smoke can record downloaded best-chain block-body progress separately from connected chainstate. | `result.firstBlockProgress`, `final_status.downloadedBlockHeight`, durable status snapshots. | Downloaded-only progress is not connected chainstate progress and remains diagnosable as awaiting connection when applicable. | Future block-connect and full-IBD evidence gate. | OBS-01, OBS-03, SEC-01 |
| Connected block progress | Opt-in live smoke can record connected block progress when a validated block reaches active chainstate. | `result.firstBlockProgress`, `final_status.connectedBlockHeight`, RPC-facing blockchain info, status/dashboard/support evidence. | Connected first-block progress is not full chain sync or unattended long-run convergence. | Future full-IBD and production-node evidence gates. | OBS-01, OBS-03, SEC-01 |
| Same-datadir restart/resume evidence | Operators can request a same-datadir restart review and inspect durable before/after progress continuity. | `result.restartResumeEvidence`, `result.restartResumeEvidence.recoveryDiagnosis.category`, Cargo and Bazel `sync status --format json` commands. | No service-manager restart policy, automatic recovery loop, or unattended production-node operation claim. | Future PRODNODE-01 service-supervision and long-run recovery milestone. | OBS-01, OBS-03, SEC-01 |
| Support evidence | Operators can generate a redacted local v1.4 support bundle summarizing live-smoke, peer, status, metrics/log, config, and store-health evidence. | `support-evidence.json`, `support-evidence.md`, support-bundle Cargo/Bazel commands, Phase 59 support projection summary. | Support bundles are not release validators, public-mainnet proof by themselves, hosted uploads, or raw report archives. | Future artifact validator or hosted support design. | OBS-02, OBS-03, SEC-01 |
| Threat model and release boundary docs | Reviewers can inspect current v1.4 STRIDE/ASVS coverage, human checklist roots, and machine-readable parity roots without rewriting v1.3 history. | `docs/parity/threat-model-v1.4.md`, `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, `docs/parity/README.md`. | v1.3 docs remain historical evidence and are not promoted into current v1.4 claims. | Future milestone threat model and parity roots when scope expands. | SEC-01, SEC-02 |
| Deterministic verification | Default verification remains deterministic and public-network-free. | `bash scripts/verify.sh`, `bash scripts/test-run-live-mainnet-smoke.sh`, `scripts/verify.sh` review, parity docs. | No public-network CI gate, default live-smoke run, or `--restart-after-progress` default verification claim. | Future release-policy decision if live evidence becomes a stronger gate. | SEC-03 |
| Inbound serving | No shipped v1.4 claim. | Deferred-surface rows in this matrix, `deviations-and-unknowns.md`, `catalog/p2p.md`. | v1.4 does not claim inbound serving or address advertisement. | Future PRODNODE-02. | SEC-02 |
| Transaction relay | No shipped v1.4 claim. | Deferred-surface rows in this matrix, `deviations-and-unknowns.md`, `catalog/p2p.md`. | v1.4 does not claim transaction relay or mempool propagation behavior. | Future PRODNODE-03. | SEC-02 |
| Production-funds wallet use | No shipped v1.4 claim. | Deferred-surface rows in this matrix and runtime-guide known limitations. | v1.4 does not claim production-funds wallet use. | Future WALPROD-01 threat model and parity evidence. | SEC-02 |
| Migration apply mode | No shipped v1.4 claim. | Dry-run migration docs, deferred-surface rows, parity deviations. | v1.4 does not claim migration apply mode, source-service cutover, or source-datadir mutation. | Future MIGAPPLY-01 safety design. | SEC-02 |
| Packaging | No shipped v1.4 claim. | Source-built install docs, deferred-surface rows. | v1.4 does not claim packaging, signed installers, or canonical OS distribution. | Future PKG-01. | SEC-02 |
| Hosted dashboard | No shipped v1.4 claim. | Local dashboard docs, deferred-surface rows. | v1.4 does not claim hosted dashboard or public dashboard operation. | Future hosted operations design. | SEC-02 |
| GUI | No shipped v1.4 claim. | Headless and terminal-first docs. | v1.4 does not claim GUI parity with the reference Qt app. | Future GUI milestone. | SEC-02 |
| Windows service support | No shipped v1.4 claim. | Source-built local service docs, deferred-surface rows. | v1.4 does not claim Windows service support or certification. | Future PKG-02. | SEC-02 |
| Unattended production-node operation | No shipped v1.4 claim. | Runtime-guide limitations, threat model, deferred-surface rows. | v1.4 does not claim unattended production-node operation. | Future PRODNODE-01 long-run service milestone. | SEC-02 |

## v1.3 Release Claim Boundary Matrix

| Surface | v1.3 Proven Claim | Accepted Evidence | Explicit Non-Claim | Future Gate | Requirements / Phases |
| --- | --- | --- | --- | --- | --- |
| Public-mainnet sync evidence | Source-built, opt-in live evidence can show validated header/block progress, restart/resume progress, or a diagnosed environment/network blocker. | `bash scripts/verify.sh`, live-smoke JSON/Markdown, support evidence JSON/Markdown, `OpenBitcoinStatusSnapshot`, Phase 50 UAT. | v1.3 does not add public-network checks to default verification. | Phase 50 evidence closeout. | PROOF-06, SEC-01, SEC-02, Phase 49, Phase 50 |
| Phase 50 live evidence closeout | Reviewers can accept observed progress or a diagnosed blocker when required evidence is present. | Typed no-progress cause, endpoint outcomes, status snapshots, next operator action, live-smoke report paths. | v1.3 does not treat DNS/TCP reachability or support-bundle existence alone as successful sync proof. | Phase 50 UAT. | PROOF-03, PROOF-04, PROOF-05, SEC-03 |
| Outbound public peer resilience | Existing daemon sync evidence distinguishes failed, waiting, stalled, connected, and useful-contribution peer states. | Phase 42, Phase 43, and Phase 44 summaries; live-smoke endpoint outcomes and peer contribution rows. | v1.3 does not claim inbound serving and address advertisement. | Future PRODNODE-02 phase. | PEER-01 through PEER-04, SEC-01 |
| Runtime resource bounds and durable recovery | Existing status and docs expose bounded runtime caps, separated durable progress, restart recovery, invalid-data rejection, and recovery guidance. | Phase 45 and Phase 46 summaries, runtime guide, status snapshot contract, support evidence. | v1.3 does not claim unattended production-node operation. | Future PRODNODE-01 phase with long-run evidence. | NODE-01 through NODE-05, SEC-01 |
| Operator RPC controls | Local status, sync pause/resume, dashboard, and support commands use the shared status truth surface and documented credential sources. | Runtime guide, `OpenBitcoinStatusSnapshot`, support evidence, Phase 47 summary. | v1.3 does not claim remote hosted administration, public RPC control, or broad ACL management. | Future remote-operator/auth scope. | OBS-01, OBS-02, SEC-01 |
| Redacted support evidence | Operators can generate local redacted support evidence with config paths, status snapshot, store health, redaction metadata, and schema v2 `result.*` live-smoke summary fields. | `support-evidence.json`, `support-evidence.md`, Phase 48 support summary, Phase 52 deterministic support-summary cleanup. | support bundles are local redacted evidence, not release validators or public-mainnet proof by themselves. | Future artifact validator or hosted support design. | OBS-03, OBS-04, SEC-01 |
| Inbound serving and address advertisement | No shipped v1.3 claim. | Deferred-surface docs, parity checklist, threat model boundary matrix. | v1.3 does not claim inbound serving and address advertisement. | Future PRODNODE-02 phase. | SEC-02 |
| Transaction relay and mempool propagation | No shipped v1.3 claim. | Deferred-surface docs, parity checklist, threat model boundary matrix. | v1.3 does not claim transaction relay or mempool propagation behavior. | Future PRODNODE-03 phase. | SEC-02 |
| Production-funds wallet use | No shipped v1.3 claim. | Deferred-surface docs, parity checklist, threat model boundary matrix. | v1.3 does not claim production-funds wallet use. | Future WALPROD-01 threat model and parity evidence. | SEC-02 |
| Migration apply mode and source datadir mutation | No shipped v1.3 claim. | Drop-in audit docs, migration dry-run docs, threat model boundary matrix. | v1.3 does not claim migration apply mode, source-service cutover, or source datadir mutation. | Future MIGAPPLY-01 phase. | SEC-02 |
| Packaging or signed installers | No shipped v1.3 claim. | Source-built install docs and deferred-surface docs. | v1.3 does not claim packaging or signed installer readiness. | Future PKG-01 phase. | SEC-02 |
| Hosted/public dashboard | No shipped v1.3 claim. | Local operator dashboard docs and deferred-surface docs. | v1.3 does not claim a hosted/public dashboard. | Future hosted operations design. | SEC-02 |
| GUI parity | No shipped v1.3 claim. | Headless and terminal-first scope docs. | v1.3 does not claim GUI parity with the reference Qt app. | Future GUI milestone. | SEC-02 |
| Unattended production-node operation | No shipped v1.3 claim. | Deferred-surface docs, runtime guide limitations, threat model boundary matrix. | v1.3 does not claim unattended production-node operation. | Future PRODNODE-01 phase. | SEC-02 |

## Phase 50 Evidence Acceptance Contract

Phase 50 can close through either observed header/block/restart-resume progress
or a diagnosed environment/network blocker.

Observed-progress evidence must include:

- live-smoke JSON and Markdown reports showing header or block progress;
- peer endpoint, source, timestamp, and before/after status snapshots;
- restart/resume evidence from the same datadir when restart/resume is the
  evidence path;
- local support evidence when useful for review context.

Diagnosed-blocker evidence must include:

- typed no-progress cause;
- endpoint outcomes;
- status snapshots;
- next operator action.

Required local commands:

```bash
bash scripts/verify.sh
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet \
  --timeout-seconds=60 --poll-seconds=5 --manual-peer=HOST[:PORT]
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support \
  --include-live-smoke-report=packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support
```

Reviewer artifact paths:

- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`
- `support-evidence.json`
- `support-evidence.md`
- `OpenBitcoinStatusSnapshot`

public-network checks remain opt-in and outside `bash scripts/verify.sh`.
support bundles are local redacted evidence, not release validators, and they
must be reviewed with the live-smoke report, status snapshots, or
diagnosed-blocker evidence.

## Phase 50 Evidence Closeout

Phase 50 closed through diagnosed blocker evidence recorded in
[`50-UAT.md`](../../.planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md).
The selected closeout report is
`packages/target/live-mainnet-smoke-reports/phase50-manual-peer-absolute/open-bitcoin-live-mainnet-smoke.json`.

The selected report recorded `result.status=no_progress`,
`result.maybeNoProgressCause=handshake_failure`, 79 manual-peer endpoint
outcomes, 24 durable status snapshots, and a next operator action to inspect
daemon stderr/endpoint outcomes or retry with a different reachable manual
peer. It did not record validated header or block progress, so Phase 50 does
not claim restart/resume success.

Generated live-smoke and support-bundle reports remain local artifacts outside
git. The committed UAT summarizes the artifact paths, selected report fields,
support-bundle status snapshot, requirement verdicts, and next operator action.

Phase 51 closes the fresh-status integration gap found during milestone audit:
the live-smoke runner now polls `openbitcoinsyncstatus` during the daemon run,
and the offline smoke regression proves progress and diagnosed-blocker snapshots
come from fresh daemon sync-control metadata. See
[`51-01-SUMMARY.md`](../../.planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md)
for the implementation closeout.

Phase 52 closes the deterministic support-summary and daemon preflight wording
debt found during milestone audit. Future support bundles summarize schema v2
`result.status`, `result.progressDetected`, `result.maybeNoProgressCause`,
`result.nextAction`, `result.headerDelta`, and `result.blockDelta` without
embedding raw live-smoke input, daemon stdout/stderr tails, raw options, raw
status snapshots, or endpoint tables. Future `open-bitcoind` preflight output
states that durable-store preflight opened and that enabled startup runs the
explicit opt-in bounded mainnet sync worker while preserving the non-claims for
unattended production-node operation and packaged-service readiness. This
deterministic cleanup does not add a new successful live-mainnet progress claim.

## Phase 53 Evidence Refresh

Phase 53 refreshes the opt-in live evidence after the Phase 51 fresh-status fix
and Phase 52 support-summary cleanup. The selected Phase 53 report is
`packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json`,
summarized in
[`53-UAT.md`](../../.planning/phases/53-live-evidence-refresh/53-UAT.md).

Phase 53 closed through fresh diagnosed blocker evidence, not successful
header, block, restart/resume, or contribution progress. The selected report is
schema v2, records `commands.status` with `openbitcoinsyncstatus`, and contains
36 fresh daemon sync-control snapshots. It recorded
`result.status=no_progress`, `result.progressDetected=false`,
`result.headerDelta=0`, `result.blockDelta=0`,
`result.maybeNoProgressCause=handshake_failure`, 205 endpoint outcomes, and 68
runtime peer rows with zero accepted header/block contribution. The next
operator action is to inspect daemon stderr and endpoint outcomes or retry with
a different reachable manual peer.

This closes the remaining D-01 and D-03 audit debt as accepted environmental
no-progress evidence: the old Phase 44 skipped live contribution UAT and the
historical Phase 50 `getblockchaininfo` snapshot caveat are superseded by a new
fresh-status report. It does not add a successful live-mainnet progress claim.

## Intentional Deferrals

[`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md) is the
current deferral and risk register. [`docs/parity/index.json`](index.json)
records no intentional in-scope external behavior deviations beyond the already
documented migration differences, but it preserves deferred surfaces for
review.

Current v1.4 deferrals include:

- inbound serving and address advertisement
- transaction relay and mempool propagation
- production-funds wallet use
- migration apply mode, source-service cutover, or source datadir mutation
- packaging or signed release installation flows
- Windows service support
- hosted/public dashboard work
- GUI parity
- unattended production-node operation
- public-network sync as part of the default local verification contract
- checked-in live-mainnet report fixtures or timing-threshold release gates

Relevant catalog and audit docs:

- [`docs/parity/catalog/rpc-cli-config.md`](catalog/rpc-cli-config.md) records
  deferred RPC, CLI, config, auth, and operator ergonomics.
- [`docs/parity/catalog/verification-harnesses.md`](catalog/verification-harnesses.md)
  records deferred harness and fuzzing work.
- [`docs/parity/catalog/drop-in-audit-and-migration.md`](catalog/drop-in-audit-and-migration.md)
  records the current dry-run-only migration posture.
- [`docs/parity/catalog/operator-runtime-release-hardening.md`](catalog/operator-runtime-release-hardening.md)
  records packaged-install, Windows-service, hosted-dashboard, and
  optional-public-network boundaries that remain outside the current shipped
  claim.
- [`docs/parity/benchmarks.md`](benchmarks.md) states that benchmark reports are
  audit and trend evidence, not release timing gates.

## Verification Evidence

Use these commands and artifacts to prove the current state:

```bash
bash scripts/verify.sh
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet
bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports
bun scripts/check-benchmark-report.ts --report=packages/target/benchmark-reports/open-bitcoin-bench-smoke.json
```

Evidence links:

- [`scripts/verify.sh`](../../scripts/verify.sh) runs deterministic local
  verification and must not run live public-network sync.
- [`scripts/check-v1.3-release-boundaries.ts`](../../scripts/check-v1.3-release-boundaries.ts)
  checks that Phase 49 docs and parity roots keep PROOF-06, SEC-01, and SEC-02
  linked without public-network access.
- [`scripts/check-v1.4-release-boundaries.ts`](../../scripts/check-v1.4-release-boundaries.ts)
  checks that Phase 59 docs and parity roots keep OBS-01, OBS-02, OBS-03,
  SEC-01, SEC-02, and SEC-03 linked without public-network access.
- [`scripts/run-live-mainnet-smoke.ts`](../../scripts/run-live-mainnet-smoke.ts)
  launches the explicit live-mainnet review flow, polls durable sync status,
  and writes local JSON plus Markdown evidence reports.
- [`scripts/check-benchmark-report.ts`](../../scripts/check-benchmark-report.ts)
  enforces the smoke report schema, required benchmark groups, required Phase 22
  case ids, and durability metadata.
- [`scripts/check-panic-sites.sh`](../../scripts/check-panic-sites.sh) scans
  first-party production Rust code for unclassified panic-like sites.
- [`docs/parity/checklist.md`](checklist.md) mirrors the checklist root from
  [`docs/parity/index.json`](index.json).

## Security Analysis Audit

Phase 41 is the v1.2 planning-security closeout gate. It reviewed tracked
`*-SECURITY.md` files from active and archived planning directories, active
v1.2 plan threat models, summary threat flags, and residual-risk sections.

Result: the reviewed corpus has `threats_open: 0` and `needs_phase_count: 0`.
The only remaining security-relevant items were deferred product-scope claims:
production-node operation, production-funds wallet use, inbound peer serving,
transaction relay, and packaged-service hardening.

Phase 49 adds the v1.3 scoped threat model in
[`docs/parity/threat-model-v1.3.md`](threat-model-v1.3.md). It does not claim a
formal security certification, and it does not expand runtime behavior.

## Benchmark Evidence

Benchmark smoke evidence is generated under
`packages/target/benchmark-reports` and is intentionally not checked into git.

Reviewer paths:

- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.json`
- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.md`

[`docs/parity/benchmarks.md`](benchmarks.md) records the benchmark groups, local
commands, report schema, and Knots mapping policy. Benchmark reports remain
threshold-free audit and trend evidence.

## Live Mainnet Smoke Evidence

Live mainnet smoke evidence is generated under
`packages/target/live-mainnet-smoke-reports` and is intentionally not checked
into git.

Reviewer paths:

- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`

Live-smoke reports record endpoint outcomes, typed no-progress causes, status
snapshots, peer contribution rows, daemon output tails, and next operator
action. They remain explicit operator evidence, not a default verification
gate.

## Reviewer Inspection Checklist

Before a release decision, inspect:

- [`docs/parity/threat-model-v1.4.md`](threat-model-v1.4.md) for the current
  v1.4 threat model, ASVS L1 mapping, release boundary matrix, and requirement
  traceability.
- [`docs/parity/threat-model-v1.3.md`](threat-model-v1.3.md) for the v1.3
  historical threat model, release boundary matrix, and requirement
  traceability.
- [`docs/parity/index.json`](index.json) for machine-readable checklist and
  audit roots.
- [`docs/parity/checklist.md`](checklist.md) for current status, evidence,
  known gaps, and suspected unknowns.
- [`docs/operator/runtime-guide.md`](../operator/runtime-guide.md) for the
  operator-facing workflow.
- [`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md) for
  deferred surfaces and suspected unknowns.
- [`scripts/verify.sh`](../../scripts/verify.sh) and
  [`scripts/check-v1.3-release-boundaries.ts`](../../scripts/check-v1.3-release-boundaries.ts)
  for the deterministic verification contract.
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`
  for the generated live-smoke JSON report.
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`
  for the generated live-smoke Markdown report.
- `support-evidence.json` for redacted support evidence.
- `support-evidence.md` for the support evidence human summary.
- [`docs/parity/benchmarks.md`](benchmarks.md) and local benchmark reports when
  reviewing runtime benchmark evidence.
