# Requirements: Open Bitcoin

**Defined:** 2026-08-29
**Milestone:** v2.3 Chainstate Durability and Historical Serving
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v2.3 Requirements

Requirements for disk-backed coins, cache-flush policy, fuller chainstate-manager behavior, and honest stored-block availability. Each requirement maps to exactly one roadmap phase.

### Coins Database and Cache

- [ ] **COIN-01**: Node persists spendable UTXOs as per-outpoint durable coin records with coins best-block and interrupted-flush markers, not as live snapshot-blob truth.
- [ ] **CACHE-01**: Node overlays an in-memory DIRTY/FRESH coins cache so connect and disconnect do not persist the whole UTXO set after every block.

### Flush Policy and Recovery

- [ ] **FLUSH-01**: Node flushes coins using IfNeeded, Periodic, and Always policy, including disk-space refusal, from injected cache, time, and disk facts.
- [ ] **FLUSH-02**: A mid-flush crash is recovered by interrupted-flush replay using stored undo and block bodies, or fails closed without inventing a consistent tip.

### Chainstate Manager

- [ ] **MGR-01**: One manager owns coins-database init, health-check, cache init, and CanFlush-style readiness for the single active chainstate.
- [ ] **MGR-02**: After restart, tip and UTXO view come from durable coins best-block, not a leftover snapshot blob.
- [ ] **MGR-03**: Flush and recovery decisions are a typed pure-core state machine; adapters perform I/O.

### Honest Availability

- [ ] **HAVL-01**: Node serves or reports a stored block as Available only when the payload bytes are present.
- [ ] **HAVL-02**: When the payload is absent, the node refuses cleanly with Unavailable and does not emit Pruned unless prune mode actually deleted files.
- [ ] **HAVL-03**: Operator evidence distinguishes payload_present, index_known, and validated_on_active_chain.

### Operator Evidence

- [ ] **CSOBS-01**: Status, RPC, CLI, dashboard, metrics, logs, and support expose flush, recovery, and have-bytes vs do-not using sanitized low-cardinality fields.
- [ ] **CSOBS-02**: Operator evidence reports cache-size state (OK / LARGE / CRITICAL) and last flush reason.
- [ ] **CSOBS-03**: A coins disk-read error fails closed as a typed storage or recovery error, not as spent or missing.

### Parity and Release Boundaries

- [ ] **CSVFY-01**: Parity roots cite pinned Knots coins, flush, manager, and serve-path anchors, or document intentional differences.
- [ ] **CSVFY-02**: Deterministic no-claim guardrails keep prune/archive modes, assumeutxo, compact filters, public defaults, and production readiness out of the v2.3 claim.

## Future Requirements

Deferred beyond v2.3 and not mapped to the current roadmap.

### Prune, Archive, and Light-Client Serving

- **FUT-18**: Node supports prune-mode product behavior, including height windows, file unlinking, prune locks, and `NODE_NETWORK_LIMITED` serving limits.
- **FUT-19**: Node claims archive-node or production-scale historical serving.
- **FUT-20**: Node serves compact filters or BIP37 bloom filters.

### Sync Shortcuts and Foreign Schemas

- **FUT-21**: Node supports assumeutxo, assumevalid, or IBD snapshot shortcuts, including dual-chainstate cache split.
- **FUT-22**: Node imports or exports Knots/Core LevelDB `chainstate/` files as a live compatibility path.
- **FUT-23**: Node performs automatic destructive reindex or coins repair.

### Broader Operation Claims

- **FUT-24**: Node enables public serving or relay by default with production-scale support evidence.
- **FUT-25**: Default CI or release blocking depends on public-network historical-serving or long-chain flush runs.
- **FUT-26**: Node claims production full-node readiness, production service operation, or production-funds wallet safety.

## Out of Scope

| Feature | Reason |
| --- | --- |
| Prune-mode product behavior | v2.3 is storage-first; honest availability must exist before prune can delete files. |
| Archive-node or production-scale historical serving | Honesty about stored payloads is not an archive or public-default claim. |
| assumeutxo, assumevalid, or IBD snapshot shortcuts | This milestone is durability, not a sync-speed shortcut. |
| Dual snapshot/IBD chainstate | A single active chainstate is the v2.3 manager boundary. |
| Compact-filter or BIP37 serving | Separate protocol, privacy, and DoS surface. |
| Knots/Core `chainstate/` binary compatibility | Different engine and schema; datadir mutation stays dry-run-first and deferred. |
| Automatic destructive reindex or coins repair | Recovery stays fail-closed and diagnostic. |
| Public serving or relay defaults | Existing activation and claim boundaries remain in force. |
| Production readiness or production-funds wallet claims | Durable coins and honest availability do not independently satisfy those gates. |
| New production crates, LevelDB, or rust-bitcoin | Extend Fjall `3.1.4` and first-party types. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
| --- | --- | --- |
| COIN-01 | — | Pending |
| CACHE-01 | — | Pending |
| FLUSH-01 | — | Pending |
| FLUSH-02 | — | Pending |
| MGR-01 | — | Pending |
| MGR-02 | — | Pending |
| MGR-03 | — | Pending |
| HAVL-01 | — | Pending |
| HAVL-02 | — | Pending |
| HAVL-03 | — | Pending |
| CSOBS-01 | — | Pending |
| CSOBS-02 | — | Pending |
| CSOBS-03 | — | Pending |
| CSVFY-01 | — | Pending |
| CSVFY-02 | — | Pending |

**Coverage:**
- v2.3 requirements: 15 total
- Mapped to phases: 0
- Unmapped: 15

---
*Requirements defined: 2026-08-29*
*Last updated: 2026-08-29 after initial definition*
