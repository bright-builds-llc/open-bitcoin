# Phase 149: Limited Serving and Honest Pruned Labels - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-27
**Phase:** 149-limited-serving-and-honest-pruned-labels
**Mode:** Yolo
**Areas discussed:** Service advertisement, Limited serve window, Honest labels, Wire refusal, Serving scope

---

## Service advertisement

| Option | Description | Selected |
|--------|-------------|----------|
| Limited only while prune mode is on | Manual or automatic prune advertises `NETWORK_LIMITED \| WITNESS` and drops `NETWORK`. Disabled keeps `NETWORK \| WITNESS`. | ✓ |
| Limited only after the first delete | Keep full `NETWORK` until have-pruned becomes true. | |
| Always advertise both bits | Match Knots full-node defaults by adding `NETWORK_LIMITED` even when prune is off. | |

**User's choice:** Limited only while prune mode is on (recommended default)
**Notes:** SERV-01 is about prune mode. Co-advertising the limited bit on full nodes is deferred to the Phase 151 parity note. Research: BIP 159 bit 10 is `1 << 10`; Knots prune mode does not add `NODE_NETWORK`.

## Limited serve window

| Option | Description | Selected |
|--------|-------------|----------|
| 288 plus the Knots +2 race buffer | Refuse block bodies whose tip distance is greater than 288 + 2, even if bytes remain. Delete keep window stays 288. | ✓ |
| Exact 288, same as the prune keep window | One constant for both delete and serve. | |
| Serve any bytes still on disk | Advertise limited service but answer historical `getdata` until the payload is gone. | |

**User's choice:** 288 plus the Knots +2 race buffer (recommended default)
**Notes:** BIP 159 says a limited node should not serve deeper than 288, so prune depth is not fingerprinted. Milestone `STACK.md` records the Knots check as 288 + 2. Window applies only in prune mode.

## Honest labels

| Option | Description | Selected |
|--------|-------------|----------|
| Pruned only when have-pruned is set and the payload is gone | Missing payload without that flag stays `Unavailable`. Present payload stays `Available`. Unknown hashes stay not-found. | ✓ |
| Pruned whenever prune mode is configured and bytes are missing | Treat config as the label source. | |
| Keep Pruned reserved | Leave the Phase 143 unused variant in place. | |

**User's choice:** Pruned only when have-pruned is set and the payload is gone (recommended default)
**Notes:** Matches LABL-01 and Phase 148's rule that have-pruned is not inferred from config.

## Wire refusal

| Option | Description | Selected |
|--------|-------------|----------|
| NotFound, plus disconnect ordinary out-of-window peers | No new wire message. Download-permission peers are refused without disconnect. In-window misses do not disconnect. | ✓ |
| NotFound only | Same refusal for every miss, no disconnect. | |
| New pruned wire error | Distinct message for pruned versus unavailable. | |

**User's choice:** NotFound, plus disconnect ordinary out-of-window peers (recommended default)
**Notes:** Phase 143 already refuses with not-found. Knots functional coverage disconnects ordinary peers who request blocks older than the limited window.

## Serving scope

| Option | Description | Selected |
|--------|-------------|----------|
| Block bodies and compact block bodies only | Extend `managed_block_serve_input`. Leave headers alone. No historical inventory outside the window. No BIP37 or compact filters. | ✓ |
| Also limit header serving | Apply the same window to `getheaders`. | |
| Operator surfaces in this phase | Add prune RPC fields and manual prune now. | |

**User's choice:** Block bodies and compact block bodies only (recommended default)
**Notes:** Operator surfaces stay in Phase 150. Parity docs and no-claim checkers stay in Phase 151.

## Claude's Discretion

- Module placement for the pure window predicate versus shell wiring.
- Exact tip-distance comparison, provided it cites the Knots 288 + 2 check and does not widen the delete keep window.
- How status and RPC render the existing `Pruned` label.

## Deferred Ideas

- Phase 150 operator prune surfaces and support evidence.
- Phase 151 full-node `NETWORK_LIMITED` co-advertisement and parity docs.
- BIP37, compact filters, archive serving, and remote limited-peer download policy.
