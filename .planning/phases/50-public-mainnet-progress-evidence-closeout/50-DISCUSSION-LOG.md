# Phase 50: Public Mainnet Progress Evidence Closeout - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-28T03:09:46Z
**Phase:** 50-public-mainnet-progress-evidence-closeout
**Mode:** Yolo
**Areas discussed:** Evidence strategy, public-network attempt shape, restart/resume proof, support evidence, release closeout, verification posture

---

## Evidence Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Existing live-mainnet smoke runner plus phase UAT summary | Use the local JSON/Markdown live-smoke report as the source of truth and commit a concise phase evidence summary that points to generated artifacts. | yes |
| Commit generated live-smoke fixtures | Check the live report JSON/Markdown into git as release evidence. | |
| Add a new Phase 50 report format | Write a separate script or schema only for closeout evidence. | |
| Documentation-only closeout | Avoid running public-network commands and only describe the operator procedure. | |

**User's choice:** Auto-selected existing live-mainnet smoke runner plus phase UAT summary.
**Notes:** The selected path preserves the established boundary that live network artifacts are local opt-in evidence, not default tracked fixtures.

---

## Public-Network Attempt Shape

| Option | Description | Selected |
|--------|-------------|----------|
| Bounded default attempt with manual-peer retry if needed | Run the smoke with an isolated datadir, then retry with an explicit peer if discovery/connectivity prevents useful progress. | yes |
| Single short smoke attempt only | Run one brief attempt and close with whatever result appears. | |
| Long unattended sync attempt | Let the daemon run until block progress is observed regardless of duration. | |
| Manual peer only | Skip DNS/default discovery and always use a single configured peer. | |

**User's choice:** Auto-selected bounded default attempt with manual-peer retry if needed.
**Notes:** This balances real evidence collection with the requirement that the phase remain bounded and operator-controlled.

---

## Restart/Resume Proof

| Option | Description | Selected |
|--------|-------------|----------|
| Same-datadir second invocation | Reuse the same public-mainnet datadir and compare before/after durable status plus runtime metadata. | yes |
| Fresh datadir second invocation | Run a second smoke from a clean store and compare broad behavior only. | |
| Process-level interruption only | Kill and restart without recording durable before/after evidence. | |
| Skip restart/resume if first run passes | Treat initial progress as sufficient. | |

**User's choice:** Auto-selected same-datadir second invocation.
**Notes:** PROOF-05 is about durable resume coherence, so a fresh datadir would not satisfy the requirement.

---

## Diagnosed Blocker Fallback

| Option | Description | Selected |
|--------|-------------|----------|
| Accept progress-or-blocker closeout | If progress is not observed, close only with typed no-progress cause, endpoint outcomes, status snapshots, and next action. | yes |
| Require successful header and block progress in this environment | Keep rerunning until successful public-mainnet progress appears. | |
| Treat any no-progress run as phase failure | Stop without a closeout summary if the live network does not progress. | |
| Soften blocker language in docs | Record a generic network note without typed diagnostics. | |

**User's choice:** Auto-selected progress-or-blocker closeout.
**Notes:** This follows Phase 49's release-boundary decision and SEC-03 wording.

---

## Support Evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Generate optional support bundle embedding the selected smoke report | Use the Phase 48 support command after live-smoke capture and record the local bundle path. | yes |
| Skip support bundle | Rely on the live-smoke report alone. | |
| Expand support bundle schema | Add new Phase 50-only support evidence fields. | |
| Make support bundle a release validator | Treat support evidence generation as proof of readiness. | |

**User's choice:** Auto-selected optional support bundle embedding the selected smoke report.
**Notes:** The bundle remains redacted local evidence and does not broaden release claims.

---

## Verification Posture

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic verification plus opt-in UAT transcript | Run required repo checks and separately record public-network smoke evidence in phase UAT. | yes |
| Add public-network smoke to `scripts/verify.sh` | Make the default verification contract depend on public network access. | |
| Skip full repo verification for evidence-only work | Treat live-smoke UAT as sufficient. | |
| CI-only closeout | Defer evidence to hosted CI or a future manual job. | |

**User's choice:** Auto-selected deterministic verification plus opt-in UAT transcript.
**Notes:** Default verification remains public-network-free, while the phase evidence remains reviewable.

---

## the agent's Discretion

- Exact smoke timeout and poll interval for bounded attempts.
- Exact target-local datadir and report directory names.
- Whether release-readiness docs need a final evidence link after outcome inspection.
- Exact wording for the UAT/evidence artifact.

## Deferred Ideas

- Hosted/manual CI live-network evidence collection.
- New offline validator for live-smoke report schema outside the default verifier.
- Future production-node, relay, wallet, migration apply, packaging, hosted dashboard, and GUI release gates.
