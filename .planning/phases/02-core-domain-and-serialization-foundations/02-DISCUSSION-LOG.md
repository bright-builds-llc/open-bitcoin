# Phase 2: Core Domain and Serialization Foundations - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in `02-CONTEXT.md`; this log preserves the
> alternatives considered.

**Date:** 2026-04-11
**Phase:** 2-core-domain-and-serialization-foundations
**Mode:** Yolo
**Areas discussed:** Core library boundaries, Invariant handling, Fixture
strategy, Reference catalog shape

---

## Core library boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Keep everything in `open-bitcoin-core` | Fastest scaffold, but risks a monolith that later phases must unwind. | |
| Split by reusable pure-core boundaries | Keep domain values, codecs, and support artifacts separable where it improves reuse across later phases. | ✓ |
| Mirror Knots file layout directly | Familiar to upstream readers, but pulls Rust structure toward C++ source parity instead of domain clarity. | |

**User's choice:** Split by reusable pure-core boundaries
**Notes:** Yolo mode selected the recommended default because Phase 1 already
established a pure-core vs runtime split and Phase 2 must create reusable
libraries for later phases.

---

## Invariant handling

| Option | Description | Selected |
|--------|-------------|----------|
| Keep primitive aliases and validate at call sites | Minimal upfront design, but spreads validation across the codebase. | |
| Parse once into invariant-bearing domain types | Validate raw inputs at boundaries and keep later logic on checked types. | ✓ |
| Defer strict validation until consensus logic | Reduces early scope, but conflicts with `ARCH-03` and leaks unchecked primitives. | |

**User's choice:** Parse once into invariant-bearing domain types
**Notes:** Yolo mode selected the recommended default because the phase
explicitly exists to satisfy `ARCH-03` and build safer reusable core types.

---

## Fixture strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Handwrite new fixtures only | Simple to control, but loses baseline provenance and misses upstream edge cases. | |
| Reuse Knots vectors plus repo-owned golden tests | Preserve parity evidence while still allowing focused first-party fixtures. | ✓ |
| Defer serious fixture work to later parity phases | Keeps phase 2 lighter, but weakens `CONS-01` and early codec confidence. | |

**User's choice:** Reuse Knots vectors plus repo-owned golden tests
**Notes:** Yolo mode selected the recommended default because Phase 2 needs
byte-exact evidence now, even though full black-box harnesses land later.

---

## Reference catalog shape

| Option | Description | Selected |
|--------|-------------|----------|
| Keep catalog details as code TODOs | Low immediate overhead, but not auditable or planner-friendly. | |
| Extend `docs/parity/` with subsystem catalog artifacts | Keeps the existing machine-readable root index and adds auditable subsystem detail. | ✓ |
| Wait until the audit-readiness milestone | Defers documentation debt and hides unknowns during active implementation. | |

**User's choice:** Extend `docs/parity/` with subsystem catalog artifacts
**Notes:** Yolo mode selected the recommended default because `REF-03` belongs
to this phase and the repo already treats `docs/parity/` as the parity source
of truth.

---

## the agent's Discretion

- Exact crate names and internal module splits once the concrete file graph is
  clear.
- Which upstream fixtures are copied verbatim versus normalized into smaller
  repo-owned golden cases, provided provenance stays documented.

## Deferred Ideas

- Live black-box parity execution against a running Knots/Open Bitcoin pair —
  planned for Phase 9.
- Runtime behavior beyond the shared domain/codec layer — deferred to the later
  consensus, chainstate, networking, wallet, and interface phases.
