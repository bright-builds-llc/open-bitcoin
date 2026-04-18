---
created: 2026-04-18T01:41:24.140Z
title: Sweep magic numbers for clarity
area: general
files:
  - packages/
---

## Problem

As the codebase grows, some numeric literals are likely being used inline in
ways that obscure intent. Even when the values are correct, magic numbers can
make protocol logic, validation rules, and policy behavior harder to audit,
harder to maintain, and easier to misuse. In a Bitcoin implementation, this is
especially risky because many constants carry precise protocol, consensus, or
policy meaning that should be obvious at the call site.

At the same time, not every literal needs extraction. Blindly replacing all
numbers with constants can add indirection instead of clarity.

## Solution

Run a focused clarity pass over the codebase looking for magic numbers whose
meaning is not obvious from nearby context.

Approach hints:
- Search for repeated or semantically important numeric literals in non-test
  code first, especially in consensus, policy, codec, networking, wallet, and
  adapter boundaries.
- Extract named constants, helper constructors, or local variables only when
  doing so makes the underlying meaning clearer.
- Prefer names that encode the protocol, policy, or domain meaning of the
  value rather than generic “DEFAULT” or “LIMIT” names.
- Keep obviously self-explanatory literals inline when extraction would reduce
  readability.
- Where several related numeric values express one domain concept, consider
  grouping them into a typed config or domain model instead of scattering
  constants.
