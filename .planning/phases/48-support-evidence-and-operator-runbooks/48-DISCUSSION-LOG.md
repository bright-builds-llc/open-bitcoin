# Phase 48: Support Evidence and Operator Runbooks - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-27T13:24:25.133Z
**Phase:** 48-support-evidence-and-operator-runbooks
**Mode:** Yolo
**Areas discussed:** Support evidence surface, redaction and safety, operator runbooks, integration with later phases

---

## Support Evidence Surface

| Option | Description | Selected |
|--------|-------------|----------|
| Dedicated support bundle command | Add an explicit operator CLI command that writes local JSON and Markdown evidence files. | yes |
| Documentation-only checklist | Tell operators to run existing commands manually and assemble artifacts themselves. | |
| Hosted upload flow | Push support data to a remote service. | |

**User's choice:** Dedicated support bundle command.
**Notes:** The recommended default keeps review local, auditable, and testable while avoiding hosted-service scope.

---

## Redaction And Safety

| Option | Description | Selected |
|--------|-------------|----------|
| Redaction-first bundle | Treat credential omission as part of the command contract and test it deterministically. | yes |
| Best-effort docs warning | Document that operators should review files manually before sharing. | |
| Raw artifact archive | Copy all available files into a bundle without filtering. | |

**User's choice:** Redaction-first bundle.
**Notes:** Cookie contents, RPC passwords, raw wallet data, and credential-like values must not appear in bundle output.

---

## Operator Runbooks

| Option | Description | Selected |
|--------|-------------|----------|
| Repo-local Cargo and Bazel commands | Make copy-pasteable repo commands primary, with installed aliases only as convenience. | yes |
| Installed binary examples only | Assume `open-bitcoin` is installed and on PATH. | |
| Conceptual docs only | Describe workflows without concrete commands. | |

**User's choice:** Repo-local Cargo and Bazel commands.
**Notes:** This carries forward the repo-local UAT command lesson and AGENTS.md guidance.

---

## Integration With Later Phases

| Option | Description | Selected |
|--------|-------------|----------|
| Stable inputs for Phases 49 and 50 | Produce evidence surfaces and docs without claiming later proof or threat-model outcomes. | yes |
| Close all v1.3 proof here | Include final public-mainnet progress and security closeout in Phase 48. | |
| Defer all evidence collection | Leave support bundle implementation to Phase 50. | |

**User's choice:** Stable inputs for Phases 49 and 50.
**Notes:** Phase 48 prepares support evidence and runbooks; Phase 49 and Phase 50 own release boundaries, threat review, and final live-mainnet proof.

---

## the agent's Discretion

- Exact command names, helper type names, bundle filenames, and Markdown section labels may follow existing operator CLI style.
- Planner may split code and docs work into one or more plans as long as OBS-03 and OBS-04 are covered.

## Deferred Ideas

None.
