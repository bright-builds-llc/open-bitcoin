# Phase 107: Runtime Relay Activation and Download Eligibility Integration - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-07-03T02:54:26.453Z
**Phase:** 107-runtime-relay-activation-and-download-eligibility-integration
**Mode:** Yolo
**Areas discussed:** Runtime activation propagation, download eligibility gate, peer class matrix, operator evidence, deterministic guardrails

## Runtime Activation Propagation

| Option | Description | Selected |
|--------|-------------|----------|
| Thread `RuntimeConfig.relay` into managed runtime construction | Treat loaded runtime config as the source of truth and prove production construction does not fall back to defaults. | yes |
| Keep default construction only | Continue relying on `ManagedPeerNetwork::new` defaults and document activation as policy-only. | |
| Rework config names | Rename relay config while fixing runtime wiring. | |

**User's choice:** Selected recommended default in YOLO mode: thread `RuntimeConfig.relay` into managed runtime construction.
**Notes:** This directly closes the roadmap gap and preserves Phase 100's Open Bitcoin-owned config surface.

## Download Eligibility Gate

| Option | Description | Selected |
|--------|-------------|----------|
| Gate scheduling with Phase 100 eligibility and typed suppression evidence | Suppress inactive or ineligible peers before request scheduling while preserving duplicate, fallback, and cleanup behavior. | yes |
| Gate only in managed network translation | Let scheduler request first and drop messages later in the adapter. | |
| Gate only in docs/status | Leave request behavior unchanged and describe eligibility as aspirational. | |

**User's choice:** Selected recommended default in YOLO mode: gate scheduling with Phase 100 eligibility and typed suppression evidence.
**Notes:** The selected approach keeps the behavior testable in the functional core and avoids hidden stale state.

## Peer Class Matrix

| Option | Description | Selected |
|--------|-------------|----------|
| Cover enabled, disabled, outbound, inbound, manual, protected, and permissioned peers | Prove ordinary inbound and protected-only peers remain ineligible while scoped relay permission plus activation enables eligible behavior. | yes |
| Test only enabled versus disabled | Prove the activation bit but not peer-class eligibility. | |
| Reuse Phase 100 policy tests unchanged | Assume prior policy coverage is enough and skip runtime/scheduler matrix coverage. | |

**User's choice:** Selected recommended default in YOLO mode: cover enabled, disabled, outbound, inbound, manual, protected, and permissioned peers.
**Notes:** This guards ACT-01, ACT-02, INV-02, INV-03, DL-01, DL-02, and REL-03 together.

## Operator Evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse shared sanitized Phase 105 status vocabulary | Distinguish default-off, explicitly enabled, eligible, and ineligible relay states without leaking raw peer or transaction material. | yes |
| Add bespoke RPC fields | Add Open Bitcoin-only details directly to baseline-shaped RPC responses. | |
| Keep evidence internal only | Avoid updating status/UAT evidence after fixing runtime behavior. | |

**User's choice:** Selected recommended default in YOLO mode: reuse shared sanitized Phase 105 status vocabulary.
**Notes:** This preserves baseline RPC compatibility while making Open Bitcoin-specific relay truth visible.

## Deterministic Guardrails

| Option | Description | Selected |
|--------|-------------|----------|
| Add focused Phase 107 checker/tests and wire them into `scripts/verify.sh` | Fail locally when runtime activation is dropped or download eligibility gates are absent. | yes |
| Extend only existing checkers | Patch Phase 100/101/106 checks without a dedicated Phase 107 evidence root. | |
| Rely on Rust tests only | Skip doc/parity/checker coverage for the gap-closure phase. | |

**User's choice:** Selected recommended default in YOLO mode: add focused Phase 107 checker/tests and wire them into `scripts/verify.sh`.
**Notes:** A dedicated checker gives the gap-closure phase an auditable evidence root before archive.

## the agent's Discretion

- Exact type names, constructor names, scheduler action labels, and checker file names are left to the planner/executor.
- The implementation may place the eligibility gate inside the scheduler or immediately before scheduler entry if the resulting behavior is pure, testable, and produces typed evidence.

## Deferred Ideas

- Durable mempool relay state recovery belongs to Phase 108.
- Compact block relay, package relay, bloom/filter serving, public relay defaults, public-network CI, production service operation, production full-node readiness, production-funds wallet safety, GUI, hosted dashboards, packaging, installer, and migration apply mode remain deferred.
