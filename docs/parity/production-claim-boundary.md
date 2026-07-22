# Production Claim Boundary

Surface id: `v1-8-production-claim-boundary`

v1.8 is a boundary-setting milestone, not the production readiness milestone.
Its current allowed statement is limited to this release-control claim:
Open Bitcoin defines gates required before a future production full-node
readiness claim.

v1.9 adds bounded opt-in inbound listener/admission, permission,
address-boundary, eviction/ban, and resource-governance evidence for loopback
or synthetic review through the existing parity roots. This boundary does not claim production full-node readiness. That evidence does not claim public inbound
defaults, transaction relay, compact block relay, mempool propagation, full
address relay, production-service operation, or production network
participation.

v2.1 provides bounded, explicit, default-off block serving and compact-block
relay with deterministic local evidence and optional public-network operator
review. This scoped capability does not authorize public serving or relay
defaults, archive-node or production-scale historical serving, production
service/deployment, production full-node readiness, or production-funds wallet
use.
Package relay, BIP37 bloom-filter serving, compact-filter serving, packaging,
GUI and hosted dashboards, migration apply mode, destructive repair, and
automatic support upload also remain deferred or unsupported.

For release review, use the v1.8 release-readiness checklist in
[`release-readiness.md`](release-readiness.md#v18-release-readiness-checklist).
It maps this boundary and the other current v1.8 requirements to canonical
evidence, deterministic checks, UAT or manual evidence, residual risk, and
no-claim or next-gate status.
For the v1.9 network participation boundary closeout, use
[`release-readiness.md`](release-readiness.md#v19-network-participation-evidence-and-release-boundary),
[`catalog/p2p.md`](catalog/p2p.md), the parity
[`checklist.md`](checklist.md), and the operator
[`runtime-guide.md`](../operator/runtime-guide.md).
For the v2.1 release boundary, use
[`release-readiness.md`](release-readiness.md#v21-block-serving-and-compact-block-relay-boundary),
the parity [`checklist.md`](checklist.md), and the operator
[`runtime-guide.md`](../operator/runtime-guide.md#phase-117-v21-release-boundary-review).
The v1.8 deterministic claim guardrails prevent overbroad
production-readiness and deferred-surface claims in the public release/operator
docs; they do not claim production full-node readiness.

## Support Terms

| Term | Definition |
| --- | --- |
| `supported` | Evidence-backed source-built behavior that default verification and documented UAT substantiate today. |
| `preview` | Shipped behavior without a support commitment. |
| `opt-in UAT` | Explicit operator-run evidence outside default verification. |
| `unsupported` | Local experimentation or historical compatibility without support expectation. |
| `deferred` | Not shipped or not safe to rely on until a future milestone names gates and evidence. |

## Claim-To-Evidence Matrix

| Statement | Support term | Current status | Evidence sources | Verification command | UAT status | Residual risk | Next required gate |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Open Bitcoin defines gates required before a future production full-node readiness claim. | `supported` | allowed | `docs/parity/production-claim-boundary.md`, `docs/parity/release-readiness.md`, `docs/parity/index.json`, and `bash scripts/verify.sh` | `bash scripts/verify.sh` | docs/parity verification only | Gates are defined but not yet satisfied | Phase 87 release-readiness checklist plus Phase 88 deterministic claim guardrails |
| Open Bitcoin has production full-node readiness. | `deferred` | not allowed yet | This boundary, `docs/parity/deviations-and-unknowns.md`, and historical v1.3 through v1.7 evidence | No default verifier may prove this in v1.8 | none | Future gates are unsatisfied | Future production-readiness milestone with scoped P2P, chainstate, wallet, operator, packaging, support, and release-policy evidence |
| Open Bitcoin supports production service operation. | `deferred` | not allowed yet | Runtime guide limitations, `docs/parity/service-operation-expectations.md`, and operator-runtime catalog | No default verifier may prove this in v1.8 | none | Service operation lacks production policy, packaging, and platform evidence | Future service-operation milestone with uptime, install, supervision, rollback, and platform gates |
| Open Bitcoin supports relay/inbound serving. | `deferred` | not allowed yet | P2P catalog, release-readiness closeout, support matrix, runtime guide, and deferred-surface register; this legacy broad-claim label applies only beyond the bounded preview paths | No default verifier may prove this in v1.8 or broad public relay or production inbound serving in v1.9 | bounded opt-in inbound and relay review only | This deferred row applies only to the broad public/default/production claim: public inbound defaults, production network participation, guaranteed public propagation, full address relay, and production full-node readiness remain unsatisfied. | Scoped P2P production milestone for public defaults, production inbound serving, address relay, relay policy, and long-run evidence |
| Open Bitcoin provides bounded, explicit, default-off v2.0 transaction relay and mempool participation. | `preview` | allowed within the named boundary | Phase 100 through 108 parity surfaces, `docs/parity/release-readiness.md`, and `docs/operator/runtime-guide.md` | `bash scripts/verify.sh` plus the Phase 106 checker pair and Phase 107/108 extension guards | optional public-network operator review | Public/default/production relay, guaranteed public propagation, service operation, and production readiness remain unsatisfied. | Separate scoped gates for public defaults, long-run network behavior, service operation, support, and production readiness |
| Open Bitcoin has bounded opt-in inbound listener/admission, permission, address-boundary, eviction/ban, and resource-governance evidence for v1.9 loopback or synthetic review. | `opt-in UAT` | allowed as bounded evidence only | `docs/operator/runtime-guide.md`, `docs/parity/catalog/p2p.md`, `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, and `docs/parity/index.json` | `bash scripts/verify.sh` remains the non-regression contract; Plan 04 owns the aggregate Phase 95 checker | loopback or synthetic review only | The evidence does not prove public inbound defaults, production-service operation, production network participation, relay, or production full-node readiness. | Future scoped milestones for public defaults, service operation, relay, packaging, and production readiness |
| Open Bitcoin provides bounded, explicit, default-off block serving and compact-block relay for v2.1 local review. | `preview` | allowed within the named boundary | Phase 110 through 117 parity surfaces, `docs/parity/release-readiness.md`, and `docs/operator/runtime-guide.md` | `bash scripts/verify.sh` plus the Phase 117 checker pair | optional public-network operator review | Public/default, archive-scale, service, deployment, and production-readiness gates remain unsatisfied. | Separate scoped gates for public defaults, archive behavior, service operation, packaging, support, and production readiness |
| Open Bitcoin has public serving or relay defaults, archive-node or production-scale historical serving, production-service operation, or production full-node readiness. | `deferred` | not allowed yet | P2P catalog, release-readiness closeout, support matrix, and deferred-surface register | No default verifier may prove these in v2.1 | none | Public exposure, archive scale, service operation, and production-readiness gates are unsatisfied. | Future scoped P2P, service, support, packaging, public-network, and production-readiness milestones |
| Open Bitcoin supports production wallet use. | `deferred` | not allowed yet | Wallet catalog and runtime guide limitations | No default verifier may prove this in v1.8 | none | Production-funds safety, backups, recovery, and threat model are incomplete | Wallet-production threat model, audit, UAT, and operator rollback gates |
| Open Bitcoin supports migration apply mode. | `deferred` | not allowed yet | Drop-in audit catalog and deviations register | No default verifier may prove this in v1.8 | none | Current migration is dry-run only | Migration apply safety design, backup, rollback, source-service, and source-datadir mutation gates |
| Open Bitcoin supports signed distribution. | `deferred` | not allowed yet | Runtime guide and operator-runtime catalog | No default verifier may prove this in v1.8 | none | Release signing, provenance, and package-manager policy are not complete | Release-engineering signing, provenance, reproducibility, and package-manager gates |
| Open Bitcoin supports hosted dashboards. | `deferred` | not allowed yet | Runtime guide and operator-runtime catalog | No default verifier may prove this in v1.8 | none | Hosted operations, auth, retention, and incident policy are undefined | Hosted-operations design, access-control, privacy, retention, and support gates |
| Open Bitcoin supports public-network CI. | `deferred` | not allowed yet | Release-readiness history and runtime guide limitations | No default verifier may prove this in v1.8 | none | Public-network checks are opt-in and environment-dependent | Public-network CI policy decision with flake budget, evidence retention, and release-blocking criteria |
| Open Bitcoin supports destructive repair. | `deferred` | not allowed yet | Recovery catalog rows and deviations register | No default verifier may prove this in v1.8 | none | Current recovery is diagnosis-only | Destructive-repair policy, backup, rollback, corruption-fixture, and operator-consent gates |
| Open Bitcoin supports automatic support upload. | `deferred` | not allowed yet | Runtime guide support-bundle boundaries and deviations register | No default verifier may prove this in v1.8 | none | Upload consent, privacy, retention, and redaction policy are not designed | Support-upload privacy, consent, retention, transport, and redaction gates |

Evidence is field- and gate-based. Artifact existence, daemon startup, peer
reachability, elapsed time, raw logs, or support bundle existence is not
sufficient proof by itself.

The source-built upgrade policy in
[`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md) is evidence
and boundary guidance for rollback, backup, state/schema compatibility, and
failed-upgrade handling. It does not satisfy the future gates for production
full-node readiness or authorize hidden source datadir, wallet, service, or
config mutation.

The operator runbook in [`operator-runbooks.md`](operator-runbooks.md) is
procedural evidence guidance for production-boundary preflight, long-run
monitoring, no-progress diagnosis, recovery/stop decisions, redacted
support-bundle timelines, and escalation evidence. It does not satisfy future
production-readiness gates, authorize hidden mutation, or promote any deferred
surface.

The service expectation document in
[`service-operation-expectations.md`](service-operation-expectations.md)
classifies source-built daemon operation, service preview, opt-in real
lifecycle UAT, and deferred production service claims without satisfying
production-service gates.

## Deferred Production-Adjacent Surfaces

| Surface | Support term | Why deferred | Required future gate |
| --- | --- | --- | --- |
| inbound serving | `deferred` | Broad inbound serving remains deferred; v1.9 documents bounded opt-in inbound review evidence only, not public exposure or production network participation. | Scoped P2P production milestone with inbound policy, resource, abuse, support, relay, and UAT evidence. |
| public inbound defaults and production network participation | `deferred` | v1.9 documents bounded opt-in inbound review evidence only; public exposure by default and production network participation are not scoped. | Scoped production-network milestone with public-default, resource, abuse, support, and UAT evidence. |
| full address relay | `deferred` | Address-manager and relay governance beyond Phase 92 bounded direct response evidence are not production-scoped. | P2P address-relay milestone with privacy, poisoning, eviction, and parity evidence. |
| public/default, archive-node, or production-scale block serving beyond the bounded v2.1 path | `deferred` | v2.1 provides a bounded default-off path only; public defaults and historical-serving scale are not scoped. | Block-serving gate with public-default policy, archive correctness, resource bounds, support policy, and production UAT. |
| public/default or production transaction relay beyond the bounded v2.0 path | `deferred` | v2.0 provides bounded, explicit, default-off relay and mempool participation only; guaranteed public propagation and production operation are not scoped. | Transaction-relay gate with public-default policy, long-run peer behavior, DoS controls, service support, and production evidence. |
| public/default or production compact-block relay beyond the bounded v2.1 path | `deferred` | v2.1 provides bounded default-off protocol, reconstruction, fallback, validation-handoff, and aggregate-evidence depth only. | Compact-block relay gate with public-default policy, long-run peer behavior, production support, and network evidence. |
| production-funds wallet use | `deferred` | Current wallet workflows are not approved for production funds. | Wallet-production threat model, backup/restore proof, audit, and operator UAT. |
| production-funds wallet safety | `deferred` | Key, signing, recovery, and support boundaries need a production wallet review. | Wallet-production safety audit with threat model, failure drills, and regression coverage. |
| migration apply mode | `deferred` | Migration remains dry-run and does not mutate source services or datadirs. | Migration apply safety design with backup, rollback, consent, and source mutation evidence. |
| signed packaging or package-manager distribution | `deferred` | The supported install path is source-built. | Release-engineering milestone for signing, provenance, reproducibility, and package-manager delivery. |
| Windows service integration | `deferred` | Current service work targets local macOS/Linux operator review. | Windows service milestone with install, supervision, rollback, and platform UAT. |
| hosted dashboards | `deferred` | Dashboard operation is local and terminal-first. | Hosted-operations design with auth, privacy, retention, monitoring, and incident gates. |
| GUI parity | `deferred` | GUI work is outside the current headless scope. | GUI milestone with parity mapping, accessibility, state management, and release UAT. |
| public-network default checks | `deferred` | Default verification must remain deterministic and local. | Release-policy decision with flake budget, opt-in boundary, and evidence retention rules. |
| public-network CI | `deferred` | Public-network CI would be environment-dependent and potentially flaky. | CI policy milestone with isolation, quotas, retention, and release-blocking criteria. |
| release-blocking live sync | `deferred` | Live sync remains opt-in evidence rather than a default gate. | Release-readiness milestone that defines blocking criteria, timing policy, and reviewer evidence. |
| automatic support-bundle upload | `deferred` | Support bundles are local redacted evidence only. | Support-upload design with consent, privacy, retention, redaction, and transport gates. |
| destructive repair | `deferred` | Recovery guidance is diagnostic and non-mutating. | Destructive-repair policy with backup, rollback, corruption fixtures, and explicit operator consent. |
| broad production-node readiness | `deferred` | Phase 82 defines gates only; the gates are not satisfied. | Future production-readiness milestone after all deferred surfaces have scoped evidence and Phase 88 guardrails. |

## Historical Evidence

v1.3 through v1.7 remain historical scoped evidence. They show source-built,
explicit opt-in public-mainnet, full-sync, soak, recovery, diagnostics, and
release-boundary review paths. They do not change any Phase 82 deferred support
term and should not be rewritten into current production support.
