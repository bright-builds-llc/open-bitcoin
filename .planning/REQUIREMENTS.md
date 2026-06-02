# Requirements: Open Bitcoin

**Defined:** 2026-06-02
**Milestone:** v1.4 Mainnet IBD Convergence and Peer Compatibility
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v1.4 Requirements

Requirements for this milestone. Each maps to roadmap phases.

### Peer Compatibility and Protocol Diagnosis

- [x] **COMPAT-01**: Reviewer can compare Open Bitcoin outbound handshake and early sync message behavior against the pinned Knots baseline for `version`, `verack`, `sendheaders`, `wtxidrelay`, `getheaders`, and `getdata` flows.
- [x] **COMPAT-02**: Operator can run a deterministic compatibility harness or equivalent scripted peer check that reproduces handshake or early-protocol failures with a precise failing step.
- [ ] **COMPAT-03**: Daemon sync can complete the outbound handshake with a reachable manual or DNS peer that accepts a baseline-compatible Knots outbound connection, without weakening existing duplicate-version, malformed-message, or wrong-network rejections.
- [x] **COMPAT-04**: Operator-facing peer diagnostics distinguish version rejection, network magic mismatch, service-bit mismatch, unsupported message order, timeout, peer disconnect, malformed payload, and local configuration failure.
- [ ] **COMPAT-05**: Daemon sync uses compatibility diagnosis to skip or replace incompatible peers without crediting useful progress or corrupting durable state.

### Header IBD Progress

- [ ] **HDR-01**: Operator can run an opt-in live-mainnet smoke command that records the first observed validated header-height increase with peer endpoint, source, timestamp, and before/after fresh daemon sync status.
- [ ] **HDR-02**: Daemon sync continues header locator and `getheaders` rounds across multiple batches until it reaches the configured smoke target, current tip estimate, timeout, or a typed diagnosed blocker.
- [ ] **HDR-03**: Header progress is durably persisted and visible through `openbitcoinsyncstatus` after daemon restart or status polling.
- [ ] **HDR-04**: Deterministic tests cover multi-batch public-mainnet-like header sync, including accepted headers, rejected headers, and no-progress diagnosis.

### Block Download and Connect Progress

- [ ] **BLK-01**: Daemon sync requests, tracks, and bounds in-flight block downloads for selected validated headers without exceeding documented v1.4 resource limits.
- [ ] **BLK-02**: Daemon sync validates and connects the first non-genesis block or configured checkpoint-adjacent block in the opt-in live-smoke path when reachable peers provide the required data.
- [ ] **BLK-03**: Live-smoke evidence records the first validated block connection with peer endpoint, block hash, height, timestamp, and before/after durable status, or records a typed diagnosis when block progress is not reached.
- [ ] **BLK-04**: Missing, `notfound`, malformed, invalid, duplicate, or disconnected block responses are peer-attributed and do not advance active chainstate or create duplicate connect work.

### Restart and Resume Evidence

- [ ] **RESUME-01**: Operator can interrupt and restart the same v1.4 public-mainnet datadir after observed header or block progress and see sync resume from durable state without duplicating block connects.
- [ ] **RESUME-02**: Live-smoke reporting can capture same-datadir before/after restart evidence for header height, block height, runtime phase, peer outcomes, and latest progress timestamp.
- [ ] **RESUME-03**: Recovery guidance distinguishes transient peer incompatibility, public-network unreachability, invalid peer data, store corruption, store incompatibility, resource exhaustion, and intentional cancellation.

### Operator Evidence and Observability

- [ ] **OBS-01**: Operator-facing status, dashboard, metrics, structured logs, RPC-facing blockchain info, and live-smoke snapshots agree on current header height, block height, peer compatibility state, progress signal, and latest error.
- [ ] **OBS-02**: Operator can generate a redacted v1.4 support bundle or equivalent evidence packet that summarizes compatibility diagnostics, selected live-smoke reports, peer outcomes, status snapshots, metrics, logs, config sources, and store health without embedding raw sensitive data.
- [ ] **OBS-03**: Operator docs provide copy-pasteable repo-local Cargo and Bazel commands for deterministic checks, manual-peer live smoke, same-datadir restart/resume review, support evidence collection, and pass/fail interpretation.

### Security and Release Claim Boundaries

- [ ] **SEC-01**: Reviewer can inspect a v1.4 threat-model update covering public peer compatibility handling, header/block input, resource bounds, restart/resume evidence, report redaction, and operator-facing live evidence.
- [ ] **SEC-02**: Reviewer can inspect refreshed parity and release-readiness docs that distinguish v1.4 opt-in outbound IBD progress from deferred inbound serving, transaction relay, production-funds wallet use, migration apply mode, packaging, hosted dashboard, GUI work, and unattended production-node claims.
- [ ] **SEC-03**: Default repo verification remains deterministic; public-network checks stay opt-in and are documented as UAT evidence rather than part of `bash scripts/verify.sh`.

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

Explicitly excluded from v1.4. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Inbound peer serving and address advertisement | v1.4 focuses on outbound peer compatibility and opt-in IBD progress before accepting inbound peers. |
| Compact block relay and transaction relay | Relay behavior needs a separate parity and resource-governance milestone after outbound IBD convergence is proven. |
| Production-funds wallet use | Wallet risk expands the threat model beyond public sync convergence and remains deferred. |
| Migration apply mode or source datadir mutation | Existing Core or Knots data remains high-value user data; v1.4 does not change the dry-run-only migration posture. |
| Signed packages or OS-native production service certification | Packaging and unattended service claims require separate platform and release-engineering work. |
| Making public-network checks part of `bash scripts/verify.sh` | Default verification must remain deterministic; live-mainnet checks stay opt-in UAT evidence. |
| Hosted/public dashboard or Qt GUI work | The milestone stays headless and terminal-first. |
| Broad production-node claim | v1.4 may prove opt-in outbound IBD progress, but it does not claim unattended production operation. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| COMPAT-01 | Phase 54 | Complete |
| COMPAT-02 | Phase 54 | Complete |
| COMPAT-03 | Phase 55 | Pending |
| COMPAT-04 | Phase 54 | Complete |
| COMPAT-05 | Phase 55 | Pending |
| HDR-01 | Phase 56 | Pending |
| HDR-02 | Phase 56 | Pending |
| HDR-03 | Phase 56 | Pending |
| HDR-04 | Phase 56 | Pending |
| BLK-01 | Phase 57 | Pending |
| BLK-02 | Phase 57 | Pending |
| BLK-03 | Phase 57 | Pending |
| BLK-04 | Phase 57 | Pending |
| RESUME-01 | Phase 58 | Pending |
| RESUME-02 | Phase 58 | Pending |
| RESUME-03 | Phase 58 | Pending |
| OBS-01 | Phase 59 | Pending |
| OBS-02 | Phase 59 | Pending |
| OBS-03 | Phase 59 | Pending |
| SEC-01 | Phase 59 | Pending |
| SEC-02 | Phase 59 | Pending |
| SEC-03 | Phase 59 | Pending |

**Coverage:**
- v1.4 requirements: 22 total
- Mapped to phases: 22
- Unmapped: 0

---
*Requirements defined: 2026-06-02*
*Last updated: 2026-06-02 after v1.4 roadmap creation*
