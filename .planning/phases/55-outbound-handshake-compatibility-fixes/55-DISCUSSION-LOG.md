# Phase 55: Outbound Handshake Compatibility Fixes - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md -- this log preserves the alternatives considered.

**Date:** 2026-06-02T22:38:08.006Z
**Phase:** 55-Outbound Handshake Compatibility Fixes
**Mode:** Yolo
**Areas discussed:** Handshake Completion, Incompatible Peer Outcomes, Scope Controls

---

## Handshake Completion

| Option | Description | Selected |
|--------|-------------|----------|
| Complete handshake is connected | Treat local/remote `version` and `verack` completion as a connected peer even if no later message arrives. | yes |
| Require later headers or blocks | Keep waiting for later sync messages before considering the peer connected. | |
| Count any peer activity | Treat any received message as compatible progress. | |

**User's choice:** Auto-selected recommended approach.
**Notes:** This directly targets the Phase 53 `handshake_failure` blocker while preserving useful-progress accounting for headers and blocks.

---

## Incompatible Peer Outcomes

| Option | Description | Selected |
|--------|-------------|----------|
| Propagate typed failures | Convert peer-manager disconnect decisions and invalid peer data into sync failed outcomes with no progress credit. | yes |
| Keep silent disconnects | Disconnect internally and let the peer eventually stall. | |
| Treat disconnects as connected | Avoid warning operators when a peer is rejected. | |

**User's choice:** Auto-selected recommended approach.
**Notes:** This preserves duplicate-version, malformed-message, and wrong-network safeguards and lets the runtime try replacement peers.

---

## Scope Controls

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic runtime fix only | Add focused Rust behavior, tests, and parity docs; keep live smoke opt-in. | yes |
| Add operator command | Add a new CLI surface for compatibility checking. | |
| Add public-network verification gate | Require live mainnet checks in default verification. | |

**User's choice:** Auto-selected recommended approach.
**Notes:** Public-network checks remain outside `bash scripts/verify.sh` per repo and milestone decisions.

## the agent's Discretion

- Exact helper names and test placement.
- Whether the typed failure taxonomy uses an existing or new `PeerFailureReason`
  variant, as long as deterministic evidence remains clear.

## Deferred Ideas

- Phase 56 validated header convergence.
- Phase 57 block download/connect progress.
- Phase 58 restart/resume proof.
- Phase 59 support bundle and release boundary closeout.
