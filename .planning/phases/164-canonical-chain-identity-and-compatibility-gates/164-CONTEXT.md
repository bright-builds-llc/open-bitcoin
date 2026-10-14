---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: 2026-10-14T01:59:33.519Z
---

# Phase 164: Canonical Chain Identity and Compatibility Gates - Context

**Gathered:** 2026-10-13
**Status:** Ready for planning
**Mode:** Yolo; recommended answers auto-selected under the requested strict-push workflow.

<domain>
## Phase Boundary

Bind existing selectors to pinned canonical roots, refuse incompatible durable state before mutation, and expose truthful supported/refused validation scopes. Requirements: CBOT-01, CBOT-04, CVLD-05.
</domain>

<decisions>
## Implementation Decisions

### Canonical root identity

- **D-01:** Only exact pinned genesis header/body for mainnet, testnet3, signet and regtest; reject any other null-parent root.
- **D-02:** Use one pure existing-network specification with inspectable genesis and support facts reused by adapters.

### Compatibility refusal

- **D-03:** Inspect before schema/coins migration, initialization, validation-history recovery, prune resume and BASIC writes across ordinary daemon/operator paths.
- **D-04:** Fail closed with a bounded typed diagnostic when root/network proof is absent, contradictory or noncanonical; preserve prior durable records and provide a seam for Phase 165 work classification.
- **D-05:** Describe the reason and safe separate-datadir/review steps; no automatic repair, conversion or destructive reindex.

### Validation support and evidence

- **D-06:** Identity selection is separate from validation support. Classify all four existing modes and refuse unsupported ordinary history before acceptance, especially Signet without challenge validation.
- **D-07:** Only implemented and verified capabilities; missing canonical bootstrap, exact work and candidate-history prerequisites remain explicit gates until Phases 165–167.
- **D-08:** Keep synthetic/source-derived fixture evidence explicitly separate from ordinary canonical runtime acceptance; do not rewrite historical receipts or silently bypass production gates.

### Agent Discretion

Choose small typed modules, exact diagnostics, storage inspection mechanics and focused tests after research; preserve first-party domain ownership, one active Fjall chainstate, existing dependency set and default-off public networking.
</decisions>

<canonical-refs>
## Canonical References

Downstream agents must read these before planning or implementation.

- .planning/ROADMAP.md — Phase 164 criteria and later-phase ownership.
- .planning/REQUIREMENTS.md — CBOT-01, CBOT-04, CVLD-05 and evidence contract.
- .planning/PROJECT.md — approved milestone scope and retained limits.
- .planning/research/SUMMARY.md — compatibility and canonical-authority pitfalls.
- .planning/research/ARCHITECTURE.md — proposed pure-core and shell seams.
- .planning/research/PITFALLS.md — mutation and unsupported-network risks.
- packages/bitcoin-knots/src/kernel/chainparams.cpp — exact canonical genesis definitions for the four existing networks at a9aee730466ac67d35a3c03ee24676be5e045878.
- packages/bitcoin-knots/src/validation.cpp — canonical chain and contextual validation ordering.
- AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/core/architecture.md, standards/core/verification.md, standards/core/testing.md, standards/languages/rust.md — local instructions and applicable standards.
</canonical-refs>

<code-context>
## Existing Code Insights

- FjallNodeStore::open in packages/open-bitcoin-node/src/storage/fjall_store.rs opens all namespaces, recovers validation history and ensures/migrates schema before runtime construction. A runtime-only gate is too late.
- Existing runtime initialization subsequently recovers coins, configures BASIC and resumes prune intent; all require accepted compatibility inspection first.
- Reuse existing codecs, network selectors, header store, managed chainstate and storage records rather than introducing a production crate or dependency.
- Existing synthetic fixtures and source-derived evidence remain narrow historical inputs.
</code-context>

<specifics>
## Specific Ideas

Compare exact pinned header fields, genesis transaction/body and hash for all four networks. Refusal tests must demonstrate unchanged existing records and absence of initialization, migration, pruning or index publication. Use explicit repo-local Cargo/Bazel commands in operator guidance.
</specifics>

<deferred>
## Deferred Ideas

Phase 165 owns exact work and legacy synthetic-work policy. Phase 166 owns canonical durable bootstrap, exact-work reopen and genesis BASIC. Phase 167 owns candidate-history consensus rules. Actual differential qualification belongs to Phase 168. No new networks, automatic repair, public-default activation or production-readiness claim.
</deferred>
