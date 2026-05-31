# Phase 51: Live Smoke Fresh Status Integration - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-31T21:25:05.455Z  
**Phase:** 51-live-smoke-fresh-status-integration  
**Mode:** Yolo  
**Areas discussed:** fresh status source, deterministic proof, evidence amendment

---

## Fresh Status Source

| Option | Description | Selected |
| --- | --- | --- |
| Poll `openbitcoinsyncstatus` | Use the daemon sync-control RPC already backed by fresh runtime metadata. | yes |
| Refresh `getblockchaininfo` per request | Keep baseline method but change RPC context loading semantics. | no |
| Defer to final status only | Keep per-poll snapshots as-is and rely on post-run durable status. | no |

**User's choice:** Auto-selected `openbitcoinsyncstatus`.  
**Notes:** This is the narrowest fix for audit gap G-01 because the fresh
status path already exists and avoids widening baseline `getblockchaininfo`
semantics in this phase.

---

## Deterministic Proof

| Option | Description | Selected |
| --- | --- | --- |
| Update offline smoke regression | Make the mock status command return fresh sync-control metadata and assert report fields. | yes |
| Add live-network verification to `verify.sh` | Prove with public-mainnet execution by default. | no |
| Add a new test framework | Introduce a separate JS/TS test harness for one script. | no |

**User's choice:** Auto-selected offline smoke regression.  
**Notes:** This preserves the v1.3 rule that public-network smoke remains
explicit UAT, not default deterministic verification.

---

## Evidence Amendment

| Option | Description | Selected |
| --- | --- | --- |
| Amend Phase 50 UAT and parity roots | Preserve historical evidence, name the stale snapshot mismatch, and link Phase 51 fix. | yes |
| Check in generated live-smoke JSON | Commit local generated evidence fixtures. | no |
| Rewrite release docs broadly | Reframe unrelated v1.3 release sections. | no |

**User's choice:** Auto-selected targeted amendment.  
**Notes:** Phase 51 should close the audited integration gap without expanding
the release boundary or committing generated runtime artifacts.

## the agent's Discretion

- Helper names, local type aliases, and exact report amendment wording are left
  to the implementer, subject to repo style and Bright Builds code-shape rules.

## Deferred Ideas

None.
