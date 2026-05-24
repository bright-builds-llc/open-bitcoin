# Requirements: Open Bitcoin

**Defined:** 2026-05-24
**Milestone:** v1.3 Public Mainnet Sync Proof and Node Hardening
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v1.3 Requirements

Requirements for this milestone. Each maps to roadmap phases.

### Public Mainnet Proof

- [x] **PROOF-01**: Operator can run an opt-in live-mainnet smoke command with explicit datadir, timeout, polling interval, and optional manual peers, and the command fails fast when local prerequisites are missing.
- [x] **PROOF-02**: Operator can distinguish no-progress causes in the live smoke report, including DNS resolution failure, TCP connection failure, handshake failure, unsupported peer capability, validation failure, storage failure, timeout, and operator cancellation.
- [ ] **PROOF-03**: Reviewer can inspect a live smoke report that records the first observed validated header-height increase with peer endpoint, source, timestamp, and before/after durable status.
- [ ] **PROOF-04**: Reviewer can inspect a live smoke report that records the first validated block connection beyond genesis or a configured checkpoint, or an explicit diagnosis when block progress was not reached.
- [ ] **PROOF-05**: Operator can interrupt and restart the same public-mainnet sync datadir and see durable before/after evidence that header, block, and runtime metadata progress resume coherently.
- [ ] **PROOF-06**: Reviewer can validate v1.3 live-mainnet evidence with documented acceptance criteria and repo-local commands without adding public-network checks to the default `bash scripts/verify.sh` gate.

### Peer Connectivity and Lifecycle Hardening

- [x] **PEER-01**: Operator can preflight DNS seeds and manual peers and see which endpoints resolved, connected, handshook, failed, or were skipped before or during opt-in mainnet sync.
- [x] **PEER-02**: Daemon sync enforces bounded outbound peer counts and rotates unhealthy peers with stable backoff, stall, and retry reasons.
- [ ] **PEER-03**: Daemon sync records per-peer header and block contribution so idle or failing peers are not reported as useful sync progress.
- [x] **PEER-04**: Daemon sync handles mixed peer failures, disconnects, timeouts, invalid data, and peer replacement without corrupting durable state or exiting unexpectedly.

### Node Runtime and Data Integrity Hardening

- [ ] **NODE-01**: Daemon sync maintains documented resource bounds for in-flight headers, in-flight blocks, durable writes, metrics retention, and log retention during long public-network runs.
- [ ] **NODE-02**: Daemon sync survives restart after partial downloads, partial validation, or partial connect work without duplicating block connects or losing validated progress.
- [ ] **NODE-03**: Daemon sync rejects invalid headers and blocks with peer attribution and recovery guidance without advancing the active chain.
- [ ] **NODE-04**: Operator pause, resume, stop, and status flows leave coherent durable status and do not create second-writer store conflicts.
- [ ] **NODE-05**: Operator recovery guidance distinguishes transient network failures, incompatible stores, corrupt stores, resource exhaustion, and intentional cancellation.

### Operator Evidence and Observability

- [ ] **OBS-01**: Operator can inspect JSON sync status that reports current phase, outbound peer count, peer outcomes, best header height, best block height, progress signal, estimated lag, last successful progress, and last error.
- [ ] **OBS-02**: Operator-facing status, dashboard, metrics, structured logs, and RPC-facing blockchain info stay consistent and never imply full sync before validated chainstate reaches the selected tip.
- [ ] **OBS-03**: Operator can generate a redacted support evidence bundle or equivalent report containing relevant config sources, command versions, sync status, peer outcomes, recent logs, metrics, store health, and live smoke artifacts.
- [ ] **OBS-04**: Operator docs provide copy-pasteable repo-local Cargo and Bazel commands, manual-peer examples, disk/network expectations, troubleshooting steps, and pass/fail interpretation for v1.3 evidence.

### Security and Release Claim Boundaries

- [ ] **SEC-01**: Reviewer can inspect a v1.3 threat model covering public peer input, resource exhaustion, storage corruption, operator RPC controls, log/report redaction, and live evidence handling.
- [ ] **SEC-02**: Reviewer can inspect refreshed parity and release-readiness docs that distinguish v1.3 proven public-mainnet sync evidence from deferred inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging, and unattended production-node claims.
- [ ] **SEC-03**: UAT records either successful public-mainnet header and block progress evidence or a diagnosed environment/network blocker with enough detail for the next operator action.

## Future Requirements

Deferred to future milestones. Tracked but not in the current roadmap.

### Production Node Scope

- **PRODNODE-01**: Operator can run Open Bitcoin as an unattended production full node with documented service supervision, restart policy, upgrade behavior, and long-run resource expectations.
- **PRODNODE-02**: Operator can accept inbound peers with documented address advertisement, ban/eviction policy, peer permissions, and resource governance.
- **PRODNODE-03**: Operator can use transaction relay and mempool propagation behavior that is parity-reviewed against the pinned Knots baseline.

### Packaging and Platform Scope

- **PKG-01**: Operator can install signed or packaged releases through a canonical distribution path instead of building from source.
- **PKG-02**: Operator can install and supervise Open Bitcoin as a Windows service.

### Wallet and Migration Scope

- **WALPROD-01**: Operator can use production-funds wallet flows with fresh wallet threat modeling and parity evidence.
- **MIGAPPLY-01**: Operator can perform an explicit, backup-aware migration apply mode that may mutate source services, datadirs, or wallet formats only after a dedicated safety design.

## Out of Scope

Explicitly excluded from v1.3. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Inbound peer serving and address advertisement | v1.3 focuses on proving outbound public-mainnet sync progress and hardening the existing opt-in daemon path. |
| Compact block relay and transaction relay | Relay behavior needs its own parity and resource-governance milestone after sync proof is stronger. |
| Production-funds wallet use | Wallet risk expands the threat model beyond public sync evidence and remains deferred. |
| Migration apply mode or source datadir mutation | Existing Core or Knots data remains high-value user data; v1.3 does not change the dry-run-only migration posture. |
| Signed packages or OS-native production service certification | Packaging and unattended service claims require separate platform and release-engineering work. |
| Making public-network checks part of `bash scripts/verify.sh` | Default verification must remain deterministic unless a future milestone deliberately changes that contract. |
| Hosted/public dashboard or Qt GUI work | The milestone stays headless and terminal-first. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| PROOF-01 | Phase 42 | Complete |
| PROOF-02 | Phase 42 | Complete |
| PROOF-03 | Phase 50 | Pending |
| PROOF-04 | Phase 50 | Pending |
| PROOF-05 | Phase 50 | Pending |
| PROOF-06 | Phase 49 | Pending |
| PEER-01 | Phase 42 | Complete |
| PEER-02 | Phase 43 | Complete |
| PEER-03 | Phase 44 | Pending |
| PEER-04 | Phase 43 | Complete |
| NODE-01 | Phase 45 | Pending |
| NODE-02 | Phase 46 | Pending |
| NODE-03 | Phase 46 | Pending |
| NODE-04 | Phase 45 | Pending |
| NODE-05 | Phase 46 | Pending |
| OBS-01 | Phase 47 | Pending |
| OBS-02 | Phase 47 | Pending |
| OBS-03 | Phase 48 | Pending |
| OBS-04 | Phase 48 | Pending |
| SEC-01 | Phase 49 | Pending |
| SEC-02 | Phase 49 | Pending |
| SEC-03 | Phase 50 | Pending |

**Coverage:**
- v1.3 requirements: 22 total
- Mapped to phases: 22
- Unmapped: 0

---
*Requirements defined: 2026-05-24*
*Last updated: 2026-05-24 after Phase 43 completion*
