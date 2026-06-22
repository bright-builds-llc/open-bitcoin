# Production Claim Boundary

Surface id: `v1-8-production-claim-boundary`

v1.8 is a boundary-setting milestone, not the production readiness milestone.
Its current allowed statement is limited to this release-control claim:
Open Bitcoin defines gates required before a future production full-node
readiness claim.

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
| Open Bitcoin supports production service operation. | `deferred` | not allowed yet | Runtime guide limitations and operator-runtime catalog | No default verifier may prove this in v1.8 | none | Service operation lacks production policy, packaging, and platform evidence | Future service-operation milestone with uptime, install, supervision, rollback, and platform gates |
| Open Bitcoin supports relay/inbound serving. | `deferred` | not allowed yet | P2P catalog and deferred-surface register | No default verifier may prove this in v1.8 | none | Inbound and relay policy are not production-scoped | Scoped P2P production milestone for inbound serving, address relay, block serving, transaction relay, and compact block relay |
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

## Deferred Production-Adjacent Surfaces

| Surface | Support term | Why deferred | Required future gate |
| --- | --- | --- | --- |
| inbound serving | `deferred` | Current P2P evidence is outbound review and does not prove production inbound policy. | Scoped P2P production milestone with inbound policy, resource, abuse, and UAT evidence. |
| address relay | `deferred` | Address-manager and relay governance are not production-scoped. | P2P address-relay milestone with privacy, poisoning, eviction, and parity evidence. |
| block serving | `deferred` | Current block evidence is validation/download-oriented, not serving policy. | Block-serving gate with serving correctness, resource bounds, peer policy, and production UAT. |
| transaction relay | `deferred` | Mempool relay behavior is not production-scoped. | Transaction-relay milestone with relay policy, DoS controls, and parity fixtures. |
| compact block relay | `deferred` | Compact-block protocol depth remains follow-up work. | Compact-block relay milestone with protocol fixtures, peer behavior, and recovery evidence. |
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
