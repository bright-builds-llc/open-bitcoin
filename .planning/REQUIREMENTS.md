# Requirements: Open Bitcoin

**Defined:** 2026-06-05
**Milestone:** v1.5 Unattended Mainnet Node Operation Readiness
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v1.5 Requirements

Requirements for this milestone. Each maps to roadmap phases.

### Unattended Daemon Sync Loop

- [ ] **LOOP-01**: Operator can start `open-bitcoind` with an explicit opt-in mainnet sync setting that runs repeated sync cycles without requiring an interactive command after the daemon binds RPC.
- [ ] **LOOP-02**: Daemon sync enforces documented stop conditions for configured header or block targets, operator pause or shutdown, sustained no-progress diagnosis, resource exhaustion, storage failure, and incompatible peer exhaustion.
- [ ] **LOOP-03**: Daemon sync applies bounded retry and backoff policy across peer, network, and protocol failures without hot-looping, unbounded peer creation, or crediting failed peers with useful progress.
- [ ] **LOOP-04**: Operator can pause, resume, and cleanly shut down the unattended sync loop while preserving durable state and explicit next-action guidance.

### Long-Run Observability and Support Evidence

- [ ] **OBS-01**: Operator-facing status, dashboard, RPC sync status, metrics, structured logs, and live-smoke snapshots agree on unattended loop phase, configured targets, attempt counters, latest progress, latest stop reason, peer health, and downloaded or connected block evidence.
- [ ] **OBS-02**: Metrics and structured logs retain bounded long-run samples and cycle summaries without unbounded growth, while preserving enough evidence to diagnose progress, waiting, retry, stop, and recovery states.
- [ ] **OBS-03**: Operator can generate a redacted v1.5 support bundle that summarizes long-run sync cycles, service state, restart/recovery evidence, peer outcomes, progress counters, stop reasons, metrics, logs, and config sources without embedding credentials or raw local report artifacts.
- [ ] **OBS-04**: Operator docs provide copy-pasteable repo-local Cargo and Bazel commands for deterministic checks, opt-in long-run review, service-based review, support bundle collection, and pass/fail interpretation.

### Service Lifecycle Hardening

- [ ] **SVC-01**: Operator can preview, install, start, stop, restart, and inspect launchd or systemd supervision for the opt-in unattended daemon workflow without implying a broad production-node claim.
- [ ] **SVC-02**: Service status distinguishes unmanaged, installed-stopped, running, failed, disabled, and unavailable-manager states while preserving shared sync truth fields.
- [ ] **SVC-03**: Daemon restart under service supervision reopens durable sync state, reports clean versus unclean prior shutdown, and resumes bounded sync work without duplicating block connection or in-flight requests.
- [ ] **SVC-04**: Service runbooks explain log locations, config paths, safe shutdown, restart review, and recovery actions for launchd and systemd operators.

### Resource Bounds and Recovery Behavior

- [ ] **RR-01**: Unattended sync enforces documented bounds for outbound peers, in-flight headers or blocks, retry queues, storage writes, metrics samples, structured logs, and support evidence size.
- [ ] **RR-02**: Recovery handling distinguishes clean shutdown, unclean shutdown, incompatible schema, store corruption, storage lock contention, resource exhaustion, invalid peer data, public-network unreachability, and operator cancellation.
- [ ] **RR-03**: Same-datadir restart tests cover extended loop recovery without duplicate block requests, duplicate block connects, corrupted active chainstate, or lost progress counters.
- [ ] **RR-04**: Operator-visible errors and recovery guidance stay typed, actionable, and consistent across status, logs, support bundles, and docs.

### Compatibility Harness Operator Wrapper

- [ ] **COMPAT-01**: Operator can run the Phase 54 public-peer compatibility harness through a documented CLI or repo script wrapper instead of invoking the Rust harness path directly.
- [ ] **COMPAT-02**: Compatibility wrapper output includes a stable JSON and Markdown report with peer endpoint, network, negotiated capabilities, failing step, diagnosis, transcript summary, and redaction boundaries.
- [ ] **COMPAT-03**: Compatibility wrapper diagnostics align with daemon peer-replacement behavior and release-boundary docs for version rejection, network mismatch, service-bit mismatch, unsupported message order, timeout, peer disconnect, malformed payload, and local configuration failure.

### Release Boundaries and Verification

- [ ] **REL-01**: Reviewer can inspect refreshed v1.5 threat-model and release-readiness docs covering unattended sync loop behavior, service supervision, long-run evidence, resource bounds, recovery states, support redaction, and compatibility wrapper output.
- [ ] **REL-02**: Parity docs distinguish v1.5 extended operator-review readiness from deferred inbound serving, transaction relay, compact block relay, production-funds wallet use, migration apply mode, packaging distribution, hosted dashboard, GUI work, and broad production-node claims.
- [ ] **REL-03**: Default repo verification remains deterministic; public-network long-run and service checks stay opt-in UAT evidence rather than part of `bash scripts/verify.sh`.
- [ ] **REL-04**: Release-boundary checks fail deterministically when v1.5 docs or parity roots omit the unattended-operation claim boundaries.

## Future Requirements

Deferred to future milestones. Tracked but not in the current roadmap.

### Production Node Scope

- **PRODNODE-01**: Operator can run Open Bitcoin as an explicitly supported production full node with documented uptime expectations, upgrade policy, public compatibility evidence, and operational support boundaries.
- **PRODNODE-02**: Operator can accept inbound peers with documented address advertisement, ban and eviction policy, peer permissions, resource governance, and parity evidence.
- **PRODNODE-03**: Operator can use transaction relay, compact block relay, and mempool propagation behavior that is parity-reviewed against the pinned Knots baseline.

### Packaging and Platform Scope

- **PKG-01**: Operator can install signed or packaged releases through a canonical distribution path instead of building from source.
- **PKG-02**: Operator can install and supervise Open Bitcoin as a Windows service.

### Wallet and Migration Scope

- **WALPROD-01**: Operator can use production-funds wallet flows with fresh wallet threat modeling, backup guidance, and parity evidence.
- **MIGAPPLY-01**: Operator can perform an explicit, backup-aware migration apply mode that may mutate source services, datadirs, or wallet formats only after a dedicated safety design.

## Out of Scope

Explicitly excluded from v1.5. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Inbound peer serving and address advertisement | v1.5 hardens unattended outbound sync review before accepting inbound peer responsibilities. |
| Compact block relay and transaction relay | Relay behavior needs a separate parity and resource-governance milestone after unattended sync operation is stable. |
| Production-funds wallet use | Wallet risk expands the threat model beyond node operation readiness and remains deferred. |
| Migration apply mode or source datadir mutation | Existing Core or Knots data remains high-value user data; v1.5 does not change the dry-run-only migration posture. |
| Signed packages or broad distribution polish | v1.5 may harden launchd/systemd behavior, but packaging and release distribution need separate platform work. |
| Making public-network checks part of `bash scripts/verify.sh` | Default verification must remain deterministic; live-mainnet and long-run checks stay opt-in UAT evidence. |
| Hosted/public dashboard or Qt GUI work | The milestone stays headless and terminal-first. |
| Broad production-node claim | v1.5 targets extended unattended operator review, not a supported production full-node claim. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| LOOP-01 | Phase 60 | Pending |
| LOOP-02 | Phase 60 | Pending |
| LOOP-03 | Phase 60 | Pending |
| LOOP-04 | Phase 60 | Pending |
| OBS-01 | Phase 62 | Pending |
| OBS-02 | Phase 62 | Pending |
| OBS-03 | Phase 65 | Pending |
| OBS-04 | Phase 65 | Pending |
| SVC-01 | Phase 63 | Pending |
| SVC-02 | Phase 63 | Pending |
| SVC-03 | Phase 64 | Pending |
| SVC-04 | Phase 63 | Pending |
| RR-01 | Phase 61 | Pending |
| RR-02 | Phase 61 | Pending |
| RR-03 | Phase 64 | Pending |
| RR-04 | Phase 61 | Pending |
| COMPAT-01 | Phase 66 | Pending |
| COMPAT-02 | Phase 66 | Pending |
| COMPAT-03 | Phase 66 | Pending |
| REL-01 | Phase 67 | Pending |
| REL-02 | Phase 67 | Pending |
| REL-03 | Phase 67 | Pending |
| REL-04 | Phase 67 | Pending |

**Coverage:**
- v1.5 requirements: 23 total
- Mapped to phases: 23
- Unmapped: 0

---
*Requirements defined: 2026-06-05*
*Last updated: 2026-06-05 after v1.5 roadmap creation*
