# v2.1 Feature Research - Block Serving and Compact Block Relay Boundary

**Defined:** 2026-07-03
**Milestone:** v2.1 Block Serving and Compact Block Relay Boundary

## Scope Decision

v2.1 should move Open Bitcoin from transaction relay and inbound participation into bounded block serving plus a compact block relay boundary. The milestone should let eligible peers request and receive validated blocks, negotiate compact-block behavior, reconstruct compact blocks from local mempool state, request missing transactions, fall back safely, and expose truthful operator evidence.

The milestone should not make Open Bitcoin a broad public relay default or production-grade serving node. It is a controlled boundary milestone that proves the behavior is internally coherent, resource bounded, parity-traceable, and observable.

## Feature Categories

### Block Serving Eligibility And Requests

**Table stakes:**

- Explicit activation keeps block serving off or narrowly scoped until configured.
- Eligible peer classes are deterministic across outbound, inbound, manual, protected, and permissioned peers.
- `getdata` requests for block, witness block, and compact block inventory are classified before storage reads.
- The node serves only validated and available blocks inside the documented active-chain or recent-valid boundary.
- Old, side-chain, pruned, unknown, unavailable, or unvalidated blocks produce suppress/fallback evidence instead of accidental serving.
- Request caps, write backpressure, and in-flight limits remain active under bursty inbound peers.

**Differentiators:**

- Serving decisions share a policy shape with transaction relay, making v2.0/v2.1 operator evidence comparable.
- Support bundles can show bounded serving outcomes without exposing raw peer endpoints or raw transaction lists.

### BIP152 Wire Messages And Negotiation

**Table stakes:**

- `sendcmpct` version 2 is encoded, decoded, and tracked per peer.
- Unsupported compact-block versions are ignored or rejected according to the documented Knots-compatible boundary.
- `cmpctblock` supports header, nonce, 6-byte short IDs, and prefilled transaction differential indexes.
- `getblocktxn` and `blocktxn` support differential indexes and witness transaction serialization.
- Per-peer high-bandwidth and low-bandwidth compact-block preferences are tracked separately from transaction relay.
- Compact-block announcements require activation, peer negotiation, relevant header state, and block availability.

**Differentiators:**

- The default status can say compact-block capability exists but public compact relay is not broadly enabled.
- Negotiation evidence can distinguish "we can serve compact blocks" from "we requested high-bandwidth compact announcements."

### Compact Block Reconstruction And Fallback

**Table stakes:**

- Compact block headers are validated before partial reconstruction state is accepted.
- Prefilled transaction ordering, transaction count limits, short ID collisions, duplicate matches, and malformed indexes are handled deterministically.
- Reconstruction uses current mempool state and bounded extra/recent block transaction state.
- Missing transaction indexes produce bounded `getblocktxn` requests.
- `blocktxn` responses are accepted only for expected in-flight partial blocks.
- Reconstructed blocks enter the existing block validation/connect path; partial reconstruction never mutates chainstate.
- Failed reconstruction, incomplete responses, stale requests, old/far blocks, and timeouts fall back to full block fetch or suppression.

**Differentiators:**

- Reconstruction can produce low-cardinality reasons such as `complete_from_mempool`, `missing_requested`, `fallback_full_block`, `collision_or_duplicate`, and `unexpected_blocktxn`.

### Resource Governance And Peer Policy

**Table stakes:**

- Compact relay participates in existing request caps, queue pressure, disconnect, misbehavior, ban/discourage, and peer cleanup rules.
- Partial compact-block state is bounded by peer and block.
- Duplicate `blocktxn`, unexpected `blocktxn`, out-of-bounds indexes, malformed compact blocks, and invalid headers do not leave stale in-flight state.
- Inbound peers cannot use compact block requests to bypass historical-serving or pruned-block boundaries.
- Restart or reconnect cleanup removes volatile compact-relay state.

**Differentiators:**

- The same deterministic verifier can prove resource governance for full block serving and compact relay without public network access.

### Operator Evidence, Parity, And Release Boundary

**Table stakes:**

- RPC/network status reports block serving and compact-block relay from one shared status contract.
- CLI, dashboard, metrics, logs, and support bundles render fixed labels.
- Parity docs cite Knots source and functional tests for every activated protocol surface.
- UAT uses repo-local Cargo and Bazel commands and keeps public-network review opt-in.
- README/operator docs state that v2.1 is bounded block serving plus compact-block boundary, not production public serving.
- Guardrails prevent package relay, bloom/filter serving, compact filter serving, public-serving defaults, and production-readiness claims from drifting in.

**Differentiators:**

- Requirements can explicitly separate "compact block relay boundary implemented" from future public relay operations.

## Anti-Features

- Package relay and cluster mempool policy are not part of this milestone.
- BIP37 bloom filters and compact filters are not activated.
- Public block or compact-block serving by default is not claimed.
- Public-network relay CI is not a default verifier gate.
- Production full-node readiness, production service operation, and production-funds wallet safety remain separate release gates.

## Suggested Build Order

1. Define block-serving activation, eligibility, and full-block request policy.
2. Add full-block serving shell behavior and status evidence.
3. Add BIP152 payload codecs and parity fixtures.
4. Add `sendcmpct` negotiation and compact block announcement policy.
5. Add compact block reconstruction from mempool and extra transaction inputs.
6. Add `getblocktxn`/`blocktxn`, fallback, in-flight cleanup, and validation handoff.
7. Add operator evidence, docs, parity, UAT, and no-claim guardrails.
