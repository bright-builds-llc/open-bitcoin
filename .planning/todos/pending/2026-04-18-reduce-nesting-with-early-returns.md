---
created: 2026-04-18T01:45:22.344Z
title: Reduce nesting with early returns
area: general
files:
  - AGENTS.md:1
  - AGENTS.bright-builds.md:1
  - packages/
---

## Problem

The repo AGENTS guidance and Bright Builds rules both push the codebase toward
minimal nesting and clearer control flow, with early returns preferred over
deeply nested branches. As the project grows, some files and functions are
likely drifting away from that style, especially in validation, parsing,
wallet, policy, or adapter code where branching complexity tends to accumulate.

When nesting grows too deep, code becomes slower to review, harder to audit,
and more error-prone because the happy path and failure paths get mixed
together. That makes it harder to preserve the project's goals around
clarity, type safety, and reference-grade maintainability.

## Solution

Run a focused codebase sweep for files and functions that violate or materially
pressure the nesting and early-return guidance, then refactor them toward
clearer control flow.

Approach hints:
- Search for functions with repeated nested `if`, `match`, or loop bodies where
  guard clauses or early returns would make the happy path clearer.
- Prioritize public APIs, reusable pure-core helpers, and adapter boundaries
  where clear failure handling matters most.
- Prefer small structural refactors that flatten control flow without changing
  behavior.
- Where nesting reflects a deeper modeling problem, consider extracting helper
  types, validated constructors, or narrower functions instead of only
  rearranging branches.
- Keep the result readable; do not force early returns where they make the flow
  more fragmented or less obvious.
