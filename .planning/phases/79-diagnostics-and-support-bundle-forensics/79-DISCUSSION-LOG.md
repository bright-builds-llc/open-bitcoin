# Phase 79: Diagnostics and Support Bundle Forensics - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md - this log preserves the
> alternatives considered.

**Date:** 2026-06-17T13:53:04.288Z
**Phase:** 79-Diagnostics and Support Bundle Forensics
**Mode:** Yolo
**Areas discussed:** Forensic timeline and checkpoint chain, Shared diagnostic contract, Failure narrative and final verdict, Redaction/size bounds/deterministic verification

---

## Forensic Timeline And Checkpoint Chain

| Option | Description | Selected |
| --- | --- | --- |
| Bundle-time reconstruction from logs/RPC/status snapshots | Small surface and useful fallback for older runs, but weak causality and fragile redaction. | |
| Structured soak event journal | Deterministic, redacted, fixture-friendly chronology from typed events. | |
| Structured journal plus hash-linked checkpoints | Adds compact ordering/truncation evidence for support-bundle forensics. | Yes |
| External telemetry traces/events | More interoperability, but more dependencies and harder deterministic redaction. | |

**Selected default:** Structured journal plus hash-linked checkpoints, with
fallback reconstruction only for incomplete or older evidence.

**Notes:** The checkpoint chain is integrity evidence for ordering and missing
events, not authenticity. Signing and external telemetry are out of scope.

---

## Shared Diagnostic Contract

| Option | Description | Selected |
| --- | --- | --- |
| Extend `OpenBitcoinStatusSnapshot` with typed diagnostic verdicts | Best when the fields are live or durable node truth shared by all surfaces. | Yes, when status truth |
| Add a support-forensics sidecar derived from status | Best for bundle-only provenance, redaction, paths, source counts, and comparison metadata. | Yes, when bundle-specific |
| External observability taxonomy | Useful later, but adds surface area and high-cardinality risk. | |
| Renderer-local adapters with golden tests | Small immediate change, but violates the no renderer-local reclassification goal. | |

**Selected default:** Preserve `OpenBitcoinStatusSnapshot` as runtime truth and
add a support-forensics sidecar only for bundle-specific facts.

**Notes:** Renderers should format shared typed values, not reclassify them from
strings.

---

## Failure Narrative And Final Verdict

| Option | Description | Selected |
| --- | --- | --- |
| Compact domain verdict plus four-field narrative | Matches operator triage: likely cause, evidence basis, next action, confidence. | Yes |
| CI-style conclusion taxonomy | Familiar but too generic and risks pass/fail production-claim confusion. | |
| Postmortem-style narrative | Rich but too verbose for default support bundles. | |
| Findings-only analysis with no final verdict | Conservative, but fails DIAG-03's final-verdict requirement. | |

**Selected default:** Compact domain verdict plus four-field narrative.

**Notes:** Recommended outcome labels are `soak_stable`,
`blocker_diagnosed`, `inconclusive`, and `collection_failed`. Wording must not
overclaim root cause from partial evidence.

---

## Redaction, Size Bounds, And Deterministic Verification

| Option | Description | Selected |
| --- | --- | --- |
| Typed support-bundle contract checker in `scripts/verify.sh` | Deterministic, repo-native, and checks redaction, field anchors, ordering, and verifier wiring. | Yes |
| Canonical generated bundle fixture plus normalized manifest | Strong artifact proof but more golden-output churn. | Partial, only where focused Rust tests need fixtures |
| Runtime CLI integration probe | Useful for file-write usability but weaker semantic proof alone. | Partial, only if needed by support tests |
| Opt-in live/soak forensic validation | Useful UAT, but non-deterministic and outside default verification. | Deferred |

**Selected default:** Focused Rust support-bundle tests plus a deterministic
Phase 79 Bun checker wired into `bash scripts/verify.sh`.

**Notes:** Default proof must not require public peers, real service managers,
multi-day wall-clock waits, or large disk allocation.

---

## the agent's Discretion

- Planner may decide exact plan count and split, provided every DIAG
  requirement has deterministic coverage and the final support narrative
  remains redacted and scoped.
- Executor may introduce compact pure types for timeline events,
  checkpoint-chain evidence, narrative verdicts, confidence, and
  missing-evidence markers.
- Executor may choose whether the checkpoint hash uses a new helper or existing
  JSON canonicalization patterns, as long as tests prove deterministic output.

## Deferred Ideas

- External telemetry export and OpenTelemetry adoption.
- Cryptographically signed support or soak evidence artifacts.
- Opt-in multi-day soak UAT closeout and v1.7 release-boundary wording.
- Production-node readiness, inbound serving, relay, wallet safety, migration
  apply mode, packaging, GUI, hosted dashboards, and scheduled public soak
  monitors.
