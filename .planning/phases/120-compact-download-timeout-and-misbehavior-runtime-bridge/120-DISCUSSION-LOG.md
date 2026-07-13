# Phase 120: Compact Download Timeout and Misbehavior Runtime Bridge - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-13
**Phase:** 120-compact-download-timeout-and-misbehavior-runtime-bridge
**Mode:** Yolo
**Areas discussed:** Timeout tick scheduling seam, Misbehavior escalation bridge, Volatile cleanup contract, Verification and scope isolation

---

## Timeout Tick Scheduling Seam

| Option | Description | Selected |
|--------|-------------|----------|
| ManagedPeerNetwork forwarder mirroring expire_transaction_requests | Thin shell → PeerManager::expire_compact_download_timeouts with caller-supplied now | ✓ |
| DurableSyncRuntime-only tick | Schedule expiry only inside DurableSyncRuntime persist/drive | |
| Background Tokio timer | Dedicated async timer thread as primary seam | |

**User's choice:** Auto-selected (yolo recommended default)
**Notes:** Keeps clock injection deterministic and matches existing transaction request expiry pattern.

---

## Misbehavior Escalation Bridge

| Option | Description | Selected |
|--------|-------------|----------|
| Map typed CompactBlockTxnMisbehavior to Disconnect/score via peer-policy | Non-empty PeerActions for GOV-02 cases | ✓ |
| Keep silent suppress only | Empty PeerAction list (current gap) | |
| Invent parallel compact-only ban book | Separate policy stack beside MisbehaviorPolicy | |

**User's choice:** Auto-selected (yolo recommended default)
**Notes:** Audit gap is suppress-only mapping; reuse existing MisbehaviorPolicy / Disconnect surfaces.

---

## Volatile Cleanup Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Preserve volatile-only cleanup; wire block-connect cleanup if still unwired | Timeout clears in-flight only; never chainstate/durable | ✓ |
| Expand cleanup into durable/chainstate paths | Broader than GOV-03 | |

**User's choice:** Auto-selected (yolo recommended default)
**Notes:** Aligns with Phase 115 D-10 and audit GOV-03 partial evidence.

---

## Verification And Scope Isolation

| Option | Description | Selected |
|--------|-------------|----------|
| Runtime proofs for tick + misbehavior; leave Phase 121 untouched | Deterministic verify.sh; no metrics projection | ✓ |
| Also project DurableSyncRuntime block-relay metrics/logs | Pulls OBS-03 into this phase | |

**User's choice:** Auto-selected (yolo recommended default)
**Notes:** Phase 121 owns OBS-03.

---

## Claude's Discretion

- Exact tick call-site within ManagedPeerNetwork / receive-drive helpers
- MisbehaviorKind / disconnect-reason mapping table per CompactBlockTxnMisbehavior variant
- Whether escalation also records through record_peer_policy_misbehavior
- Test clock advancement for forced expiry

## Deferred Ideas

- DurableSyncRuntime block-relay metrics/log projection — Phase 121
- Package relay / filters / public defaults / production claims — out of scope
