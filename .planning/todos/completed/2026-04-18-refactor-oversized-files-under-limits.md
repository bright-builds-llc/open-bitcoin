---
created: 2026-04-18T01:43:58.758Z
title: Refactor oversized files under limits
area: general
files:
  - AGENTS.md:1
  - AGENTS.bright-builds.md:1
  - packages/
---

## Problem

The repo guidance and Bright Builds rules both treat oversized files as a code
quality smell, but the codebase has likely accumulated files that now exceed
those thresholds. The repo-level guidance says to consider splitting files once
they grow beyond roughly 300-500 lines, and the Bright Builds rules treat files
over roughly 628 lines as refactor triggers.

When large files are left in place, they become harder to review, harder to
reason about, and more likely to mix unrelated concerns. That is especially
costly in this project because the architecture depends on keeping pure-core
logic explicit, auditable, and structurally simple.

## Solution

Run a codebase sweep for files that violate or materially pressure the file-size
guidance, then refactor the worst offenders into smaller modules where doing so
improves cohesion and readability.

Approach hints:
- Identify files above the Bright Builds refactor trigger first, then review
  files in the 300-500 line range where the local AGENTS guidance already
  suggests splitting.
- Prefer refactors that separate responsibilities cleanly rather than slicing
  files arbitrarily just to hit a number.
- Preserve functional-core versus imperative-shell boundaries while extracting
  modules.
- Keep public APIs and file layout comprehensible after the split so the change
  increases clarity instead of scattering logic.
- Add or update verification around the touched code paths so the refactors do
  not silently change behavior.
