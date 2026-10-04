# Phase 154: BASIC Generation and Commitment Parity - Discussion Log

Audit trail only; downstream agents use CONTEXT.md.

**Date:** 2026-10-03 CDT
**Mode:** Yolo; one recommendation pass, auto-selected defaults.

| Area / Question                            | Selected recommendation                              | Alternatives considered                                         | Rationale                                                            |
| ------------------------------------------ | ---------------------------------------------------- | --------------------------------------------------------------- | -------------------------------------------------------------------- |
| Historical inputs: what is authoritative?  | Complete validated undo/spent scripts                | Current coins reconstruction; caller-supplied unchecked scripts | Preserve historical and same-block spend behavior.                   |
| Missing evidence: what outcome?            | Explicit fail-closed error                           | Empty scripts; partial filter                                   | Missing inputs must not invent a successful commitment.              |
| Encoding: which filter rules?              | Exact pinned BASIC/type 0                            | Knots V0; general filter product                                | BASIC-only requirement and independent vectors define scope.         |
| Commitment API: how to represent identity? | Typed raw-byte identities and explicit predecessor   | Display-hex hashing; implicit current tip                       | Prevent byte reversal and branch substitution.                       |
| Proof: how to obtain expected bytes?       | Pinned vectors plus independent Knots oracle         | Self-derived Rust expected values                               | Parity must have independent evidence.                               |
| Layering: where does generation live?      | Existing pure crates and narrow validated-input seam | New crate/dependencies; runtime-owned generation                | Preserve functional core and dependency policy.                      |
| UI hint: what UI is needed?                | None for this phase                                  | New dashboard/GUI                                               | Protocol header wording causes the hint; operator UI belongs to 161. |

No scope additions. Implementation details remain agent discretion. Source and standards inputs are listed in CONTEXT.md.
