# Phase 152: Post-Prune Wallet Rescan Eligibility - Discussion Log

Audit trail only; downstream agents use CONTEXT.md.

**Date:** 2026-10-02 CDT
**Mode:** Yolo (recommended decisions auto-selected by requested workflow)
**Lifecycle:** 152-2026-10-03T00-26-52

| Area | Alternatives considered | Selected and rationale |
| --- | --- | --- |
| Replacement scope | Incremental merge; full replacement with entry probes; probe all historical payloads | Full replacement with entry probes plus existing range gate; preserves current behavior and Phase 146 D-04 without refusing unrelated older coins. |
| Missing payload | Skip coin; refuse and preserve wallet; snapshot fallback | Refuse and preserve wallet; no silent omissions or invented authority. |
| Read error | Return only; persist Failed and return; retry indefinitely | Persist Failed then propagate; closes audit WR-02 and supports restart diagnosis. |
| Resume | Trust earlier probes; recheck each replacement | Recheck each replacement; prune can remove payloads between chunks. |
| Evidence | Source assertions; in-memory deletes; real paired deletion and reopen | Real paired deletion, both adapters, retained controls and default verifier. |

No user answers were solicited in yolo mode. Decisions carry forward locked
Phase 146 preferences and the approved Phase 152 audit closure scope.
External research cross-check: upstream Bitcoin Core's
[rescan RPC source](https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/transactions.cpp)
checks block availability before rescanning. The pinned local Knots source
and this repository's full replacement contract control implementation.

No matching pending todos were found. Automatic retention remains Phase 153.
