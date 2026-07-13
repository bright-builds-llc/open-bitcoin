# Phase 119: Compact Receive Mempool Candidate Injection - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-13
**Phase:** 119-compact-receive-mempool-candidate-injection
**Mode:** Yolo
**Areas discussed:** Receive Candidate Supply Seam, Mempool And Extra Sources, Mempool Removal Lifecycle Hook, Verification And Parity

---

## Receive Candidate Supply Seam

| Option | Description | Selected |
|--------|-------------|----------|
| Shell intercept + facts into download API | ManagedPeerNetwork gathers candidates/extras and calls handle_compact_block_download (or facts-aware entry) without coupling PeerManager to mempool | ✓ |
| Embed mempool into PeerManager handle_message | Network crate depends on mempool and fills facts inside message_dispatch | |
| Leave empty default; only test injection | Keep production empty facts; cover candidates only in unit tests | |

**User's choice:** [auto] Shell intercept + facts into download API (recommended default)
**Notes:** Closes audit seam while preserving Phase 114 D-08 (no mempool crate in network). Matches Phase 118 pattern of shell owning effectful wiring.

---

## Mempool And Extra Sources

| Option | Description | Selected |
|--------|-------------|----------|
| Current mempool view + Knots-shaped bounded extra buffer | Live mempool candidates plus dedicated bounded extras in node shell | ✓ |
| Mempool only; skip extras this phase | Only feed mempool; leave extras empty until later | |
| Unbounded recent-block history as extras | Scan arbitrary historical blocks for candidates | |

**User's choice:** [auto] Current mempool view + Knots-shaped bounded extra buffer (recommended default)
**Notes:** RCN-02 requires mempool plus bounded extra/recent inputs. Unbounded history risks archive-like claims and GOV-05 bleed.

---

## Mempool Removal Lifecycle Hook

| Option | Description | Selected |
|--------|-------------|----------|
| Hook from mempool lifecycle removals into compact partial state | Call on_mempool_transaction_removed / PeerManager forwarder on connect and other mempool-exit paths | ✓ |
| Hook only on block connect confirmed removals | Skip evict/expire paths | |
| Defer lifecycle hook to Phase 120 | Only inject candidates now | |

**User's choice:** [auto] Hook from mempool lifecycle removals into compact partial state (recommended default)
**Notes:** GOV-04 audit explicitly cites missing lifecycle call; Phase 120 is timeout/misbehavior, not this hook.

---

## Verification And Parity

| Option | Description | Selected |
|--------|-------------|----------|
| Runtime tests for inject + outcomes + lifecycle; breadcrumbs; verify.sh | Prove live path, typed outcomes, cleanup; keep package/filter/public defaults untouched | ✓ |
| Unit-only network tests with empty production path | Leave receive_message empty-facts | |
| Require public-network CI for RCN-02 | Add public compact-relay CI gate | |

**User's choice:** [auto] Runtime tests for inject + outcomes + lifecycle; breadcrumbs; verify.sh (recommended default)
**Notes:** Matches gap-closure verification posture from Phase 118; public-network stays opt-in UAT.

---

## Auto-selection Log

```
[auto-select] Selected all gray areas: Receive Candidate Supply Seam, Mempool And Extra Sources, Mempool Removal Lifecycle Hook, Verification And Parity.
[auto] Receive Candidate Supply Seam — Q: "Where should live CompactBlock receive get candidates?" → Selected: "Shell intercept + facts into download API" (recommended default)
[auto] Mempool And Extra Sources — Q: "What supplies mempool and extras?" → Selected: "Current mempool view + Knots-shaped bounded extra buffer" (recommended default)
[auto] Mempool Removal Lifecycle Hook — Q: "How should mempool removals clear partial compact state?" → Selected: "Hook from mempool lifecycle removals into compact partial state" (recommended default)
[auto] Verification And Parity — Q: "What verification bar closes the gap?" → Selected: "Runtime tests for inject + outcomes + lifecycle; breadcrumbs; verify.sh" (recommended default)
```
