---
phase: 91-peer-permissions-and-connection-classes
plan: 09
subsystem: docs-parity
tags: [docs, parity, p2p, peer-permissions, uat]

requires:
  - phase: 91-06
    provides: "Operator status permission rendering"
  - phase: 91-07
    provides: "Support-bundle permission evidence and redaction"
  - phase: 91-08
    provides: "Negative relay, mempool, filter, and compact-block safeguards"
provides:
  - "Architecture docs for Open Bitcoin-owned permission-class config and status evidence"
  - "Loopback permission-class UAT with repo-local Cargo and Bazel commands"
  - "Parity surface v1-9-peer-permissions-connection-classes for PERM-01 through PERM-04"
affects:
  - 91-10-deterministic-phase-checker-and-verifier-wiring
  - 95-network-participation-evidence-and-release-boundary

tech-stack:
  added: []
  patterns:
    - "Document active permission effects separately from inactive/deferred relay-like labels"
    - "Use repo-local Cargo and Bazel command forms for operator UAT"
    - "Keep parity roots explicit about Knots anchors and Open Bitcoin no-claim boundaries"

key-files:
  created:
    - .planning/phases/91-peer-permissions-and-connection-classes/91-09-SUMMARY.md
  modified:
    - docs/architecture/config-precedence.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/p2p.md
    - docs/parity/index.json
    - docs/parity/checklist.md

key-decisions:
  - "Phase 91 docs accept only Open Bitcoin-owned JSONC and CLI permission-class inputs."
  - "Operator UAT uses loopback commands and expects relay-like tokens to appear as inactive effects."
  - "Parity roots cite net_permissions, net.cpp, net_processing.cpp, and p2p_permissions.py while keeping whitelist/whitebind and relay behavior outside the claim."

requirements-completed: [PERM-01, PERM-02, PERM-03, PERM-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T19:01:22Z

duration: 4min
completed: 2026-06-25
---

# Phase 91 Plan 09: Operator Docs, Parity Roots, and UAT Commands Summary

**Phase 91 permission classes are now documented as Open Bitcoin-owned, loopback-reviewable, parity-rooted evidence with explicit inactive/deferred relay-like boundaries.**

## Accomplishments

- Documented `inbound.permission_classes`, literal-IP matching, and repeatable `-openbitcoininboundpermissionclass=<name>@<literal_ip>=<tokens>` CLI overrides.
- Added status and observability contracts for `permission_class`, permissioned/protected counts, active effects, inactive effects, latest permission decision, and fixed permission metrics.
- Added loopback permission-class UAT commands for Cargo and Bazel daemon startup, `openbitcoinnetworkstatus`, `open-bitcoin status --format json`, and redacted support bundles.
- Registered `v1-9-peer-permissions-connection-classes` in the P2P catalog, machine index, and checklist with PERM-01 through PERM-04 traceability.
- Preserved no-claim boundaries for Knots `whitelist`/`whitebind`, transaction relay, compact block relay, mempool propagation, BIP37, compact filters, full address relay, ban/misbehavior semantics, public inbound defaults, and production readiness.

## Task Commits

1. **Task 1: Update architecture docs for permission config and status** - `a84bafc` (`docs`)
2. **Task 2: Add operator UAT commands with Cargo and Bazel forms** - `a84bafc` (`docs`)
3. **Task 3: Register Phase 91 parity roots and no-claim boundary** - `a84bafc` (`docs`)

## Files Created/Modified

- `docs/architecture/config-precedence.md` - Documents JSONC/CLI permission-class ownership and literal-IP matching.
- `docs/architecture/status-snapshot.md` - Documents shared inbound permission status fields and inactive effects.
- `docs/architecture/operator-observability.md` - Documents permission metrics, support redaction, and inactive label semantics.
- `docs/operator/runtime-guide.md` - Adds loopback permission-class UAT commands with Cargo and Bazel forms.
- `docs/parity/catalog/p2p.md` - Registers Phase 91 parity roots and no-claim boundary.
- `docs/parity/index.json` - Adds the machine-readable Phase 91 surface and audit entry.
- `docs/parity/checklist.md` - Adds the human-readable Phase 91 surface row.

## Verification Results

- `rg -n "inbound.permission_classes|openbitcoininboundpermissionclass|literal IP|whitelist|whitebind" docs/architecture/config-precedence.md` - passed
- `rg -n "permission_class|active_permission_effects|inactive_permission_effects|latest_permission_decision" docs/architecture/status-snapshot.md docs/architecture/operator-observability.md` - passed
- `rg -n "relay|forcerelay|mempool|bloomfilter|blockfilters" docs/architecture/status-snapshot.md docs/architecture/operator-observability.md` - passed
- `rg -n "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --|bazel run //packages/open-bitcoin-rpc:open_bitcoind --|openbitcoininboundpermissionclass|openbitcoinnetworkstatus|support bundle --output-dir=/tmp/open-bitcoin-permission-support" docs/operator/runtime-guide.md` - passed
- `rg -n "inactive.*relay|inactive.*mempool|inactive.*bloom|inactive.*blockfilter" docs/operator/runtime-guide.md` - passed
- `rg -n "v1-9-peer-permissions-connection-classes|PERM-01|PERM-02|PERM-03|PERM-04|net_permissions.h|net_permissions.cpp|p2p_permissions.py|net.cpp|net_processing.cpp" docs/parity/catalog/p2p.md docs/parity/index.json docs/parity/checklist.md` - passed
- `node -e "const fs=require('fs'); const p=JSON.parse(fs.readFileSync('docs/parity/index.json','utf8')); const s=JSON.stringify(p); if(!s.includes('v1-9-peer-permissions-connection-classes')) process.exit(1)"` - passed
- `git diff --check -- docs/architecture/config-precedence.md docs/architecture/status-snapshot.md docs/architecture/operator-observability.md docs/operator/runtime-guide.md docs/parity/catalog/p2p.md docs/parity/index.json docs/parity/checklist.md` - passed

## Deviations from Plan

- None. The docs stayed within the planned architecture, operator, and parity files.

## Next Phase Readiness

Plan 91-10 can wire deterministic checks against the documented labels, UAT command forms, parity surface id, Knots anchors, source breadcrumbs, and no-claim boundaries.
