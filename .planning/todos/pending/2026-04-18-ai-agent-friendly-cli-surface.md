---
created: 2026-04-18T01:37:17.534Z
title: AI-agent-friendly CLI surface
area: planning
files:
  - .planning/ROADMAP.md:210
  - .planning/REQUIREMENTS.md:53
  - .planning/REQUIREMENTS.md:54
  - .planning/REQUIREMENTS.md:55
---

## Problem

Phase 8 already owns the RPC, CLI, and headless operator surface, but the
current roadmap wording does not explicitly call out AI-agent-friendly CLI
behavior. If the CLI ends up optimized only for human operators, it will be
harder for autonomous agents and scripted workflows to discover commands,
understand failures, compose multi-step flows, and operate the node or wallet
reliably at scale. That would limit mass usability, integration quality, and
project adoption even if baseline-compatible CLI coverage lands.

## Solution

When planning and implementing Phase 8, extend the CLI design so agent-facing
automation is a first-class use case alongside human operators.

Approach hints:
- Prefer stable, machine-readable output modes for every important command,
  especially structured JSON with predictable field names and explicit exit
  codes.
- Make command discovery and self-description easy for agents: strong `--help`,
  consistent schemas, examples, and if practical command metadata or capability
  introspection.
- Keep flows non-interactive and scriptable by default, including flags for
  confirmation bypass, explicit network selection, deterministic formatting, and
  idempotent behavior where possible.
- Ensure error responses are structured and actionable instead of free-form
  text only.
- Consider whether some operator flows should have dedicated “agent mode”
  affordances, such as minimal output, schema-stable summaries, or explicit
  next-step hints.
