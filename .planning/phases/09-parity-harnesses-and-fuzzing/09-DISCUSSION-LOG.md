# Phase 9: Parity Harnesses and Fuzzing - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-24T10:06:16.773Z
**Phase:** 9-parity-harnesses-and-fuzzing
**Mode:** Yolo
**Areas discussed:** Harness target contract, integration isolation, property coverage, CI reporting

---

## Harness Target Contract

| Option | Description | Selected |
| --- | --- | --- |
| Shared Rust `FunctionalCase` suite | One suite drives multiple target adapters through JSON-RPC-visible behavior. | yes |
| Separate Open Bitcoin and Knots tests | Faster to scaffold but risks drift between implementations. | no |
| Spawn Knots automatically | Stronger parity proof but requires a built baseline binary and heavier lifecycle work. | no |

**User's choice:** auto-selected shared Rust suite with optional external Knots endpoint.
**Notes:** Keeps `VER-03` honest without making every local verify depend on a built Knots daemon.

---

## Integration Isolation

| Option | Description | Selected |
| --- | --- | --- |
| Harness-owned sandbox helpers | Reusable unique temp dirs, port reservations, and process guards. | yes |
| Per-test ad hoc helpers | Less upfront code, more collision risk. | no |
| Global fixed test ports | Simple but not parallel-safe. | no |

**User's choice:** auto-selected harness-owned isolation helpers.
**Notes:** Directly supports `VER-04`.

---

## Property Coverage

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic property-style tests | Reproducible generated cases under normal `cargo test`. | yes |
| Add cargo-fuzz/libFuzzer now | Higher ceiling but larger toolchain and CI cost. | no |
| Only fixture tests | Keeps scope small but does not satisfy the property-style risk reduction. | no |

**User's choice:** auto-selected deterministic property-style tests.
**Notes:** Avoids new fuzz runtime dependency while satisfying `PAR-01` for the first slice.

---

## CI Reporting

| Option | Description | Selected |
| --- | --- | --- |
| Extend `scripts/verify.sh` and upload reports in CI | Keeps one repo-native verification contract. | yes |
| Add a separate CI job | More visible but duplicates workflow ownership. | no |
| Local-only reports | Less CI churn but weaker auditability. | no |

**User's choice:** auto-selected verify-path reporting.
**Notes:** Aligns with repo-local guidance.

## the agent's Discretion

- Exact module names and report formatting.
- Exact representative RPC cases in the first suite.
- Exact deterministic generator shape for property-style tests.

## Deferred Ideas

- Full managed Knots process lifecycle in Rust.
- Translating the complete upstream Python functional suite.
