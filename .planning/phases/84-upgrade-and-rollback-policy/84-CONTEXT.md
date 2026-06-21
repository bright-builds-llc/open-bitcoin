---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 84-2026-06-21T21-33-46
generated_at: 2026-06-21T21:33:46.234Z
---

# Phase 84: Upgrade and Rollback Policy - Context

**Gathered:** 2026-06-21
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 84 documents source-built upgrade, rollback, backup, and state/schema
compatibility expectations for operators and contributors. Operators should be
able to prepare for an upgrade, classify compatibility outcomes, choose between
upgrade, retry, rollback, backup-then-rebuild, or stop-and-escalate guidance,
and recover from failed upgrades without hidden source datadir, wallet, service,
or config mutation.

This phase should not implement migration apply mode, destructive repair,
automatic backup or restore, signed packaging, automatic update channels,
production-funds wallet support, broad production service guarantees, or public
network/default-verification expansion. It should preserve the Phase 82
production claim boundary and the Phase 83 support matrix while adding a narrow
upgrade-policy surface and deterministic drift checks.
</domain>

<decisions>
## Implementation Decisions

### Upgrade Preflight Checklist

- **D-01:** Create one canonical upgrade and rollback policy under `docs/parity/`
  for source-built installs. The policy should be operator-facing, quiet, and
  explicit that v1.8 defines upgrade boundaries rather than a production
  full-node readiness claim.
- **D-02:** The pre-upgrade checklist must cover: current source revision or
  commit, repo-local verification status, binary provenance from Cargo or
  Bazel, Open Bitcoin JSONC config path, `bitcoin.conf` path, selected datadir,
  datadir ownership and free-space review, current sync/status evidence,
  support-bundle evidence when available, service state, wallet scope, and
  backup location.
- **D-03:** Operator commands in the policy must prefer repo-local forms:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. Do not rely
  only on an installed `open-bitcoin` alias.
- **D-04:** The checklist should distinguish review-only evidence from mutation.
  Collecting status, support bundles, config summaries, or service state is
  acceptable; source datadir, wallet, service, and config mutation requires a
  future scoped plan.

### State And Schema Compatibility Outcomes

- **D-05:** Reuse the existing recovery vocabulary instead of inventing upgrade
  labels. The policy should map `clean_shutdown`, `unclean_shutdown`,
  `incompatible_schema`, `store_corruption`, `storage_lock_contention`,
  `schema_mismatch`, `corruption_marker`, `corrupt_record`, `partial_write`,
  `unreadable_namespace`, `backend_open_failure`, and the action classes
  `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, and
  `stop_and_escalate`.
- **D-06:** Compatibility guidance should be decision-table based: when the
  evidence points to safe retry, retry the source-built run; when it points to
  read-only inspection, inspect and preserve evidence; when it points to
  backup-then-rebuild, preserve a backup before rebuild; when it points to
  stop-and-escalate, stop and attach redacted evidence.
- **D-07:** A schema or storage compatibility outcome is not proven by daemon
  startup, elapsed time, peer reachability, raw logs, or report existence alone.
  The policy must require field-level evidence and unavailable-field reasons
  where applicable.
- **D-08:** The policy should explicitly separate Open Bitcoin-owned durable
  store state from external Core/Knots source datadirs and wallets. External
  state remains high-value input and must not be rewritten as part of rollback
  policy.

### Rollback And Failed-Upgrade Boundary

- **D-09:** Failed-upgrade guidance should prioritize evidence preservation:
  stop the attempted upgraded process, record exact command and commit, collect
  redacted local evidence, preserve backups, and avoid repeated mutation until
  compatibility class is understood.
- **D-10:** Rollback guidance should be source-built and local-first: return to
  the previous checked-out source revision or known binary, use the same
  explicit datadir and config paths, verify with repo-local commands, and
  record the rollback evidence. Do not imply package-manager rollback, signed
  release channels, or automatic update behavior.
- **D-11:** The policy must not recommend hidden mutation of source datadirs,
  external wallets, service files, launchd/systemd state, `bitcoin.conf`, or
  Open Bitcoin JSONC config. Any mutation guidance must be explicit, future
  gated, and outside this phase unless already supported by existing docs.
- **D-12:** Destructive repair stays deferred. `backup_then_rebuild` is evidence
  and operator-decision guidance, not permission for automated destructive
  rebuild or repair.

### Release Readiness And Verification Drift Checks

- **D-13:** Link the upgrade policy from the production claim boundary, support
  matrix, release-readiness document, deviations register, parity README,
  parity checklist, parity index, README, runtime guide, and relevant parity
  catalogs without creating a second support matrix.
- **D-14:** Add a narrow Phase 84 deterministic checker only for upgrade-policy
  drift: required policy sections, exact support terms, backup/rollback
  boundaries, forbidden hidden-mutation language, required repo-local command
  forms, canonical links, and verifier wiring.
- **D-15:** The checker must be deterministic, public-network-free,
  real-service-manager-free, multi-day-free, and Bun-backed like the Phase 82
  and Phase 83 checkers. It must not become the broad all-doc production claim
  scanner owned by Phase 88.
- **D-16:** Final closeout should run `bash scripts/verify.sh`. Focused Bun
  checker/test commands may be used during iteration, but the phase verification
  evidence should cite the repo-native verifier.

### the agent's Discretion

- The planner may split work into canonical policy content, parity/root link
  updates, deterministic checker and fixture tests, verifier wiring, and
  closeout evidence.
- The executor may decide whether the upgrade policy is a new
  `docs/parity/upgrade-and-rollback-policy.md` file or a narrower filename with
  the same canonical role, as long as all entrypoints link to one source of
  truth.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update parity breadcrumbs for any new first-party Rust source or test files
  under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 84 goal, dependency on Phase 82, success
  criteria, and v1.8 phase sequencing.
- `.planning/REQUIREMENTS.md` - UPG-01 through UPG-04, v1.8 future
  requirements, and out-of-scope table.
- `.planning/PROJECT.md` - active v1.8 boundary-setting posture, core value,
  production-claim constraints, and current project state.
- `.planning/STATE.md` - current milestone state and accumulated decisions.
- `AGENTS.md` - repo-local verification, UAT command, parity breadcrumb, GSD,
  generated artifact, and workflow rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - local standards override registry.
- `standards/core/architecture.md` - functional-core and illegal-state rules.
- `standards/core/code-shape.md` - code shape, script, and naming rules.
- `standards/core/local-guidance.md` - local guidance and merge-safe task
  artifact rules.
- `standards/core/verification.md` - sync-first and repo-native verification
  requirements.
- `standards/core/testing.md` - unit-test and Arrange/Act/Assert expectations.
- `standards/languages/rust.md` - Rust module, option naming, invariant, and
  verification guidance.
- `standards/languages/typescript-javascript.md` - Bun/TS automation guidance.

### Locked v1.8 Boundary And Support Decisions

- `.planning/phases/82-production-claim-boundary/82-CONTEXT.md` - locked
  production vocabulary, evidence-gate model, deferred-surface inventory, and
  documentation/verification posture.
- `.planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md` -
  locked support matrix, issue-evidence expectations, residual-risk posture,
  and contributor update boundaries.
- `docs/parity/production-claim-boundary.md` - exact support terms,
  claim-to-evidence matrix, and deferred production-adjacent surfaces.
- `docs/parity/support-matrix.md` - current support classification and
  issue-evidence policy that Phase 84 must not broaden.
- `docs/parity/release-readiness.md` - v1.8 handoff and historical v1.3 through
  v1.7 release-boundary evidence.
- `docs/parity/deviations-and-unknowns.md` - deferred-surface register,
  destructive repair boundary, migration apply boundary, and wallet non-claims.
- `scripts/check-phase82-production-claim-boundary.ts` and
  `scripts/check-phase82-production-claim-boundary.test.ts` - Phase 82 checker
  and fixture-test pattern.
- `scripts/check-phase83-support-matrix-issue-evidence.ts` and
  `scripts/check-phase83-support-matrix-issue-evidence.test.ts` - Phase 83
  checker and fixture-test pattern.

### Upgrade, Recovery, Storage, Wallet, Migration, And Operator Evidence

- `docs/operator/runtime-guide.md` - source-built operator workflows, support
  bundles, recovery diagnosis, service state, opt-in UAT, and repo-local Cargo
  and Bazel command forms.
- `docs/architecture/status-snapshot.md` - recovery states, progress evidence,
  status fields, compatibility categories, and unavailable-field policy.
- `docs/architecture/operator-observability.md` - support evidence,
  recovery/progress interpretation, logs, metrics, and operator status
  semantics.
- `docs/architecture/storage-decision.md` - Fjall storage decision, schema
  versioning, restart behavior, and recovery action classes.
- `docs/architecture/cli-command-architecture.md` - migration and wallet
  command boundaries, backup expectations, and compatibility parser separation.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - resource,
  recovery, support-bundle, service, and release-hardening evidence catalog.
- `docs/parity/catalog/drop-in-audit-and-migration.md` - dry-run migration
  boundary, rollback expectations, and source datadir/wallet mutation limits.
- `docs/parity/catalog/wallet.md` - managed-wallet backup and production-funds
  non-claim boundaries.
- `docs/parity/catalog/chainstate.md` - chainstate release boundaries and
  production-node non-claims.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/checklist.md` - human-readable parity checklist root.
- `docs/parity/README.md` - parity entrypoint.
- `README.md` - contributor/operator entrypoint requiring a Phase 84 pointer
  without duplicating the full policy.
- `scripts/verify.sh` - repo-native verification contract.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `docs/parity/production-claim-boundary.md` already defines the exact support
  terms, allowed claim, not-yet-allowed claims, and deferred future gates.
- `docs/parity/support-matrix.md` already classifies storage/datadir recovery,
  migration dry-run, service manager, wallet, destructive repair, and public
  network surfaces by support term.
- `docs/operator/runtime-guide.md` already contains command forms, recovery
  state vocabulary, support-bundle collection, service state, and opt-in UAT
  patterns that the upgrade policy can link instead of duplicating.
- `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`,
  and `docs/architecture/storage-decision.md` already define the recovery and
  compatibility terms that should drive the policy decision tables.
- Phase 82 and Phase 83 checkers provide the nearest deterministic Bun checker
  and test-file patterns for a narrow Phase 84 checker.

### Established Patterns

- Current v1.8 docs extend the boundary and support-policy surface without
  turning historical v1.3 through v1.7 evidence into production support.
- Default verification remains deterministic, local, public-network-free,
  real-service-manager-free, timing-stable, and short-running.
- Operator-facing examples use explicit repo-local Cargo and Bazel commands.
- Evidence is field-specific. Raw artifact existence, elapsed time, peer
  reachability, daemon startup, or raw logs are not enough by themselves.
- Source datadirs, source service state, configs, and wallets are high-value
  user data. Current migration and recovery surfaces are dry-run or diagnostic
  unless a future phase explicitly designs mutation.

### Integration Points

- Add one canonical Phase 84 policy under `docs/parity/`.
- Link it from `README.md`, `docs/operator/runtime-guide.md`,
  `docs/parity/production-claim-boundary.md`, `docs/parity/support-matrix.md`,
  `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`,
  `docs/parity/README.md`, `docs/parity/checklist.md`,
  `docs/parity/index.json`, and relevant parity catalog pages.
- Add `scripts/check-phase84-upgrade-rollback-policy.ts` and a matching
  `.test.ts` if planning confirms deterministic drift checks are needed.
- Wire the Phase 84 checker into `scripts/verify.sh` near the Phase 82 and
  Phase 83 v1.8 checks.
- Refresh `docs/metrics/lines-of-code.md` if hooks or verification regenerate
  it.
</code_context>

<specifics>
## Specific Ideas

- Keep the policy table-driven and operator-actionable: evidence observed,
  compatibility class, allowed next action, forbidden hidden mutation, and
  escalation evidence.
- Prefer pointers over duplication for long recovery vocabulary already defined
  in architecture and runtime docs.
- Use the repo-local Cargo and Bazel command style from existing lessons and
  AGENTS.md whenever the policy tells an operator to collect evidence.
</specifics>

<deferred>
## Deferred Ideas

- Migration apply mode, destructive repair, package-manager rollback, signed
  release channels, automatic update channels, production-funds wallet support,
  and automatic backup/restore remain future scoped milestones.
</deferred>

---

*Phase: 84-upgrade-and-rollback-policy*
*Context gathered: 2026-06-21*
