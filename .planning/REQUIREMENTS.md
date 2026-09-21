# Requirements: Open Bitcoin

**Defined:** 2026-09-21
**Milestone:** v2.4 Prune-Mode Product Behavior
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v2.4 Requirements

Requirements for Knots-aligned prune on the single active chainstate. Block and undo removal uses the existing Fjall payload keys. The `blk`/`rev` file difference is a documented parity difference, not a second block store. Each requirement maps to exactly one roadmap phase.

### Wallet Cutover

- [ ] **SNAP-01**: Wallet rescan reads durable coins and payload-present blocks, and does not treat leftover snapshot bytes as chain truth.

### Prune Policy

- [ ] **PRUN-01**: Operator can disable prune, select manual-only prune, or set an automatic target of at least 550 MiB.
- [ ] **PRUN-02**: Automatic prune keeps the last 288 blocks and does not start before the network prune-after height.
- [ ] **PRUN-03**: Manual prune refuses a target inside the 288-block keep window.

### Unlink

- [ ] **UNLK-01**: A prune of a height removes that height's block payload and undo together.
- [ ] **UNLK-02**: The node records have-pruned only after that delete is durable.
- [ ] **UNLK-03**: Restart after an interrupted prune finishes or refuses the partial delete without inventing blocks or reindexing.

### Locks

- [ ] **LOCK-01**: A prune lock keeps the locked height range, plus a 10-block buffer, from deletion.
- [ ] **LOCK-02**: Operator can list and set prune locks.

### Serving

- [ ] **SERV-01**: Prune mode advertises `NODE_NETWORK_LIMITED` and does not advertise full `NODE_NETWORK`.
- [ ] **SERV-02**: A request for a block older than the limited serve window is refused.
- [ ] **SERV-03**: A request for a block whose payload prune removed is not served.

### Labels

- [ ] **LABL-01**: Status and RPC report `Pruned` only when have-pruned is set and the payload is gone; a missing payload without prune stays `Unavailable`.

### Operator Evidence

- [ ] **OPER-01**: Operator can read pruned, prune height, automatic pruning, and prune target from RPC, CLI, and dashboard.
- [ ] **OPER-02**: Operator can request a manual prune when prune mode is on.
- [ ] **OPER-03**: Support evidence reports prune counts and the last prune height without raw storage paths.

### Guardrails

- [ ] **GRD-01**: Parity docs cite the pinned Knots prune anchors, including the Fjall key versus `blk`/`rev` file difference, and checkers still reject archive serving, assumeutxo, BIP37, public defaults, and production-readiness claims.

## Future Requirements

Deferred beyond v2.4 and not mapped to the current roadmap.

### Prune Startup Optimization

- **FUT-27**: Node applies a temporary prune target during initial block download (`-pruneduringinit`).

### Later Node Capabilities

- **FUT-19**: Node claims archive-node or production-scale historical serving.
- **FUT-20**: Node serves compact filters (BIP157/158). BIP37 bloom serving stays excluded.
- **FUT-21**: Node supports assumeutxo, assumevalid, or a second chainstate.
- **FUT-22**: Node imports or exports Knots/Core LevelDB `chainstate/` files as a live compatibility path.
- **FUT-23**: Node performs automatic destructive reindex or coins repair.

### Broader Operation Claims

- **FUT-24**: Node enables public serving or relay by default.
- **FUT-25**: Default CI or release blocking depends on public-network runs.
- **FUT-26**: Node claims production full-node readiness, production service operation, or production-funds wallet safety.

## Out of Scope

| Feature | Reason |
| --- | --- |
| Knots `blk`/`rev` flat-file block store | v2.3 already stores payloads as Fjall keys. v2.4 matches prune behavior on that store and records the file-layout difference. |
| Archive-node or production-scale historical serving | Keeping and serving all history is the opposite operator problem from prune. |
| assumeutxo, assumevalid, or a second chainstate | v2.4 keeps the single active chainstate. |
| BIP37 bloom serving | Privacy and DoS surface. Compact filters are a later milestone. |
| Compact-filter serving in this milestone | Needs prune locks first, then its own retention design. |
| LevelDB `chainstate/` import or export | Different engine and schema. Migration stays dry-run-first. |
| Automatic destructive reindex | Interrupted prune stays fail-closed. Repair stays explicit. |
| `-txindex` combined with prune | The pinned Knots help marks those modes incompatible. |
| Public serving or relay defaults | Activation stays opt-in. |
| Public-network CI as a release gate | Default verification stays hermetic. |
| Production readiness or production-funds wallet claims | Prune does not satisfy those evidence gates. |
| New production crates, LevelDB, RocksDB, or rust-bitcoin | Extend Fjall 3.1.4 and the existing first-party crates. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
| --- | --- | --- |
| SNAP-01 | Phase 146 | Pending |
| PRUN-01 | Phase 147 | Pending |
| PRUN-02 | Phase 147 | Pending |
| PRUN-03 | Phase 147 | Pending |
| LOCK-01 | Phase 147 | Pending |
| UNLK-01 | Phase 148 | Pending |
| UNLK-02 | Phase 148 | Pending |
| UNLK-03 | Phase 148 | Pending |
| SERV-01 | Phase 149 | Pending |
| SERV-02 | Phase 149 | Pending |
| SERV-03 | Phase 149 | Pending |
| LABL-01 | Phase 149 | Pending |
| OPER-01 | Phase 150 | Pending |
| OPER-02 | Phase 150 | Pending |
| OPER-03 | Phase 150 | Pending |
| LOCK-02 | Phase 150 | Pending |
| GRD-01 | Phase 151 | Pending |

**Coverage:**
- v2.4 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0

---
*Requirements defined: 2026-09-21*
*Last updated: 2026-09-21 after v2.4 roadmap creation*
