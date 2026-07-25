# Phase 132: Typed Package Vocabulary and Staged Admission - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-25
**Phase:** 132-typed-package-vocabulary-and-staged-admission
**Mode:** Yolo
**Areas discussed:** Package shape, identity, and ordered outcomes; dry-run, partial acceptance, and staged commit; effective-fee and final policy boundaries

## Package shape, identity, and ordered outcomes

| Option | Description | Selected |
| --- | --- | --- |
| Opaque package refinements + index-aligned report | Prove common and submission-only shapes once, preserve request order, and encode package/member/effective-fee result invariants in types. | ✓ |
| Single opaque bounded package + runtime mode checks | Centralize common checks with fewer types, but keep submission applicability and result combinations fallible. | |
| Raw vector + wtxid-keyed result map | Translate Knots mechanically, but retain invalid packages and lose request ordering unless an extra sidecar is maintained. | |

**User's choice:** Auto-selected the recommended opaque-refinement and index-aligned-report contract.
**Notes:** This matches the existing checked lifecycle identity contract and gives Phases 133 and 137 proven invariants without repeated validation.

## Dry-run, partial acceptance, and staged commit

| Option | Description | Selected |
| --- | --- | --- |
| Typed staged overlay + checked sub-deltas + one guarded live apply | Preserve individual-first results in a prospective view, discard dry-runs, and compose coherent committed facts only after final trim. | ✓ |
| Whole-mempool clone simulation + swap | Reuse current mutation code but impose full-mempool copying and state-field drift risk. | |
| Sequential live changesets | Mirror Knots procedurally but complicate dry-run isolation, effect buffering, and aggregate lifecycle truth. | |

**User's choice:** Auto-selected the recommended typed staged overlay.
**Notes:** Open Bitcoin already prepares single-transaction state prospectively; the chosen approach extracts that seam and adds base-state guarding instead of building a second authority.

## Effective-fee and final policy boundaries

| Option | Description | Selected |
| --- | --- | --- |
| Mode-specific typed staged pipelines | Implement the full unchanged PACK-06/PACK-07 contract with explicit ordering and invalid policy combinations excluded by type. | ✓ |
| Single staged engine with sealed profiles | Share more orchestration but recreate a capability matrix whose checks can leak between modes. | |
| Narrow profile with typed unsupported outcomes | Ship less policy now, but require changing PACK-07 and leave unowned parity gaps. | |

**User's choice:** Auto-selected the recommended full PACK-06/PACK-07 pipeline.
**Notes:** Requirements remain unchanged, so limited package RBF, TRUC, ephemeral dust, witness aliases, reconsiderability, pressure trim, and final-membership rewriting are Phase 132 obligations.

## the agent's Discretion

- Exact type, method, and module names.
- Internal overlay storage and base-state token mechanics.
- Exact package/member rejection enum decomposition.
- Private helper sharing between dry-run and submission.

## Deferred Ideas

- Peer package assembly — Phase 133.
- Cross-cache lifecycle projection — Phase 134.
- RPC and sanitized operator evidence — Phase 137.
