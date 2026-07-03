# v2.1 Pitfalls Research - Block Serving and Compact Block Relay Boundary

**Defined:** 2026-07-03
**Milestone:** v2.1 Block Serving and Compact Block Relay Boundary

## Pitfalls And Preventive Rules

### Accidental Public Serving Default

**Risk:** Adding block serving and compact block announcements can look like a public-serving launch.

**Preventive rule:** Every serving and compact-relay effect must pass explicit activation and peer eligibility. Docs and status must keep public-default and production-readiness claims separate.

**Warning sign:** Tests prove a block can be served, but no test proves default-disabled behavior.

### Serving Blocks Outside The Safe Boundary

**Risk:** A peer can request old, side-chain, pruned, unavailable, or not-fully-validated blocks and receive data that should be suppressed or bounded.

**Preventive rule:** Classify block status before storage reads. Only serve blocks inside the documented active-chain or recent-valid boundary, and produce typed suppress/fallback evidence for other states.

**Warning sign:** Block serving code reads from `blocks_by_hash` or durable storage before an eligibility decision exists.

### Leaking Prune Height Or Peer Details

**Risk:** Serving decisions can reveal prune depth, raw peer endpoints, permission strings, or raw transaction lists through logs, metrics, support bundles, or status JSON.

**Preventive rule:** Use low-cardinality labels and sanitize details. Public support evidence may count outcomes and capability states, not dump peer identities or transaction payloads.

**Warning sign:** A metric label or support field contains a block hash plus peer endpoint or raw transaction identifier list.

### Codec And Differential Index Bugs

**Risk:** BIP152 differential indexes, prefilled transaction indexes, and six-byte short IDs are easy to parse incorrectly.

**Preventive rule:** Keep codec logic isolated and fixture-tested against Knots functional-test style examples. Reject non-canonical, overflowing, trailing, or out-of-bounds payloads before policy.

**Warning sign:** `getblocktxn` indexes are treated as absolute in one path and differential in another.

### Short ID Collision Or Hash-Table DoS

**Risk:** Compact-block short IDs can collide or be shaped to create expensive reconstruction paths.

**Preventive rule:** Bound short ID counts, detect duplicate/collision cases, cap partial state, and fall back to full block fetch when reconstruction is suspicious or ambiguous.

**Warning sign:** Reconstruction loops through unbounded mempool entries without an early exit, cap, or failure status.

### Partial Reconstruction Mutates Chainstate

**Risk:** A partially reconstructed compact block could update local chain or durable progress before validation succeeds.

**Preventive rule:** Partial reconstruction is volatile. Only a complete block that passes the existing validation/connect path may affect chainstate or durable block evidence.

**Warning sign:** Partial compact-block state is written to durable store or status reports a connected block before validation.

### Incorrect `blocktxn` Matching

**Risk:** Duplicate, unexpected, or wrong-peer `blocktxn` responses can complete the wrong partial block or leave stale in-flight state.

**Preventive rule:** Match `blocktxn` by block hash, peer, and current partial state. Treat duplicate or unexpected responses as typed protocol outcomes and clear/fallback deliberately.

**Warning sign:** A `blocktxn` response can be processed when no `getblocktxn` was sent.

### Fallback Storms

**Risk:** Failed reconstruction can create repeated full-block requests or duplicate in-flight entries.

**Preventive rule:** Reuse existing in-flight block caps and cleanup. Fallback should preserve one clear owner per block/peer attempt and emit bounded evidence.

**Warning sign:** The same block appears multiple times in per-peer requested blocks after one compact-block failure.

### Compact Relay Scope Creep

**Risk:** Implementing BIP152 can accidentally activate package relay, bloom/filter serving, compact filters, or broad public relay claims.

**Preventive rule:** Add deterministic no-claim checkers and explicit deferred requirements for adjacent protocol surfaces.

**Warning sign:** Docs mention package relay, filters, or public serving without "deferred", "not claimed", or a future-milestone qualifier.

### Public-Network Verification Drift

**Risk:** Compact-block relay tempts public-network UAT as a default verifier.

**Preventive rule:** Keep `bash scripts/verify.sh` deterministic. Public-network review may exist only as opt-in UAT evidence.

**Warning sign:** A pre-commit, CI, or default script requires public peers, long wall-clock soak, service-manager state, or production deployment.

## Phase Placement

- Activation/default safety belongs in the first phase.
- Codec and differential-index risks belong before runtime integration.
- Reconstruction and `blocktxn` matching should be implemented before public operator evidence claims success.
- Documentation/no-claim guardrails belong in the final milestone closure phase and should be verified by scripts.
