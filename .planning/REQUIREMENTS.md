# Requirements: Open Bitcoin v1.8 Production Full-Node Readiness Boundary

**Defined:** 2026-06-20
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v1.8 Requirements

### Production Boundary

- [ ] **PROD-01**: Operator can read a production full-node readiness definition that separates supported, preview, opt-in UAT, unsupported, and deferred surfaces.
- [ ] **PROD-02**: Release reviewer can trace each allowed production-related statement to an explicit evidence gate, current status, and verification source.
- [ ] **PROD-03**: Contributor can tell which evidence is required before Open Bitcoin may claim production full-node readiness.
- [ ] **PROD-04**: Operator-facing docs explicitly preserve deferred status for inbound serving, relay, production-funds wallet use, migration apply mode, signed packaging, hosted dashboards, GUI parity, public-network CI, destructive repair, and automatic support-bundle upload.

### Support Boundaries

- [ ] **SUP-01**: Operator can identify which source-built install, runtime, network, storage, and service-supervision environments are supported, preview, opt-in UAT, best-effort, or unsupported.
- [ ] **SUP-02**: Operator can identify the support information expected for issue reports, including redacted support bundles, logs, configuration summaries, service state, resource evidence, and sync status evidence.
- [ ] **SUP-03**: Contributor can update the support matrix without accidentally broadening production, wallet, relay, migration, packaging, hosted-dashboard, GUI, or CI claims.
- [ ] **SUP-04**: Release reviewer can see residual risks and manual validation surfaces carried forward from v1.1 through v1.7 before approving v1.8 release language.

### Upgrade Policy

- [x] **UPG-01**: Operator can follow a pre-upgrade checklist covering backups, source-built binaries, config files, datadir ownership, service state, and current sync evidence.
- [x] **UPG-02**: Operator can understand state and schema compatibility expectations, including when upgrade, retry, rollback, backup-then-rebuild, or stop-and-escalate guidance applies.
- [x] **UPG-03**: Operator can follow rollback and failed-upgrade guidance without hidden source datadir, wallet, service, or config mutation.
- [x] **UPG-04**: Contributor can run deterministic checks that fail when upgrade policy docs, rollback boundaries, or backup expectations drift out of the release-readiness contract.

### Runbooks and Service Expectations

- [ ] **RUN-01**: Operator can follow a production-boundary preflight runbook before long-running source-built node operation.
- [ ] **RUN-02**: Operator can follow long-run operation, monitoring, no-progress diagnosis, recovery, and escalation runbooks using existing v1.3 through v1.7 evidence surfaces.
- [ ] **RUN-03**: Operator can collect a redacted support-bundle timeline and identify what evidence is sufficient for support triage.
- [x] **SVC-01**: Operator can distinguish source-built daemon operation from launchd/systemd supervision, packaged-service distribution, service-manager availability, and unsupported production-service claims.
- [x] **SVC-02**: Operator can verify service lifecycle, restart/resume, log, metric, resource-bound, and recovery expectations through repo-local Cargo and Bazel command forms.

### Release Readiness and Guardrails

- [x] **REL-01**: Release reviewer can use a v1.8 release-readiness checklist that maps every production-boundary requirement to docs, UAT, deterministic checks, and residual risk.
- [ ] **REL-02**: Deterministic verification fails if release docs claim production full-node readiness without the required v1.8 evidence gates.
- [ ] **REL-03**: Deterministic verification fails if docs imply deferred surfaces are production-ready, including inbound serving, relay, production-funds wallet use, migration apply mode, signed packaging, hosted dashboards, GUI parity, public-network CI, destructive repair, or automatic support-bundle upload.
- [ ] **REL-04**: Default `bash scripts/verify.sh` runs the v1.8 release-boundary checker while keeping public-network, real service-manager, and multi-day checks opt-in.
- [x] **REL-05**: Contributor-facing README and parity docs point to the v1.8 boundary docs, support policy, upgrade policy, runbooks, and release-readiness checklist.
- [x] **REL-06**: Release reviewer can verify that v1.8 ends with a truthful no-claim boundary unless all production readiness gates are explicitly satisfied by a future milestone.

## Future Requirements

Deferred to later milestones. Tracked but not in the current roadmap.

### Production Capability Expansion

- **PROD-FUT-01**: Operator can rely on an evidence-backed production full-node readiness claim after all defined gates pass.
- **PROD-FUT-02**: Operator can use inbound peer serving, address relay, block serving, transaction relay, compact block relay, and mempool relay as supported production surfaces.
- **PROD-FUT-03**: Operator can use production-funds wallet workflows under explicitly audited safety boundaries.
- **PROD-FUT-04**: Operator can apply migration workflows to existing Core or Knots datadirs through explicit backup-aware apply mode.

### Distribution and Operations

- **DIST-FUT-01**: Operator can install signed release artifacts or package-manager builds with documented provenance and upgrade channels.
- **DIST-FUT-02**: Operator can use Windows service integration with support expectations comparable to launchd/systemd.
- **DIST-FUT-03**: Release reviewer can treat public-network full-sync or soak checks as release-blocking CI when the project deliberately changes the verification contract.
- **DIST-FUT-04**: Operator can opt into automatic support-bundle upload through an audited privacy and consent model.

## Out of Scope

Explicitly excluded from v1.8. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Default production full-node readiness claim | v1.8 defines gates and guardrails before such a claim is allowed. |
| Inbound peer serving, address relay, block serving, transaction relay, compact block relay, and mempool relay | These require separate P2P capability and public-network evidence milestones. |
| Production-funds wallet safety | Wallet runtime exists, but production-funds support needs a separate threat model, audit, and evidence milestone. |
| Migration apply mode or destructive repair | Existing datadirs and wallets remain high-value user data; v1.8 keeps mutation dry-run-first and explicitly deferred. |
| Signed packaging, package-manager distribution, or automatic update channels | v1.8 covers source-built support boundaries and upgrade policy, not distribution infrastructure. |
| Hosted dashboards, GUI parity, or public marketing launch | The project remains headless and terminal-first for this milestone. |
| Public-network, real service-manager, or multi-day checks in default verification | Default verification must remain deterministic; these checks stay opt-in UAT unless a future milestone changes the contract. |
| Automatic support-bundle upload | v1.8 may define manual support evidence expectations, but upload, retention, consent, and privacy automation require separate design. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| PROD-01 | Phase 82 | Pending |
| PROD-02 | Phase 82 | Pending |
| PROD-03 | Phase 82 | Pending |
| PROD-04 | Phase 82 | Pending |
| SUP-01 | Phase 83 | Pending |
| SUP-02 | Phase 83 | Pending |
| SUP-03 | Phase 83 | Pending |
| SUP-04 | Phase 83 | Pending |
| UPG-01 | Phase 84 | Complete |
| UPG-02 | Phase 84 | Complete |
| UPG-03 | Phase 84 | Complete |
| UPG-04 | Phase 84 | Complete |
| RUN-01 | Phase 85 | Pending |
| RUN-02 | Phase 85 | Pending |
| RUN-03 | Phase 85 | Pending |
| SVC-01 | Phase 86 | Complete |
| SVC-02 | Phase 86 | Complete |
| REL-01 | Phase 87 | Complete |
| REL-02 | Phase 88 | Pending |
| REL-03 | Phase 88 | Pending |
| REL-04 | Phase 88 | Pending |
| REL-05 | Phase 87 | Complete |
| REL-06 | Phase 87 | Complete |

**Coverage:**
- v1.8 requirements: 23 total
- Mapped to phases: 23
- Unmapped: 0

---
*Requirements defined: 2026-06-20*
*Last updated: 2026-06-22 after completing Phase 84 upgrade and rollback policy*
