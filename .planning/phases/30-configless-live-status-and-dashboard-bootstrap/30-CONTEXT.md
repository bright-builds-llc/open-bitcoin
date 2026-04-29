---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 30-2026-04-29T16-19-20
generated_at: 2026-04-29T16:19:20.795Z
---

# Phase 30: Configless Live Status and Dashboard Bootstrap - Context

**Gathered:** 2026-04-29
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Repair the shared operator live-RPC bootstrap so `open-bitcoin status` and
`open-bitcoin dashboard` stay truthful for the documented flag-only local
workflow when the selected datadir has no implicit `bitcoin.conf`. The fix must
preserve the shared status snapshot path, keep stopped-node behavior for
genuinely unavailable live RPC, and avoid reopening wallet/build-provenance
logic outside what this bootstrap repair needs. New operator RPC override flags,
broader config-model refactors, and unrelated dashboard UI work remain out of
scope.

</domain>

<decisions>
## Implementation Decisions

### Live RPC Bootstrap
- **D-01:** Treat a missing implicit `bitcoin.conf` as normal evidence, not as a
  hard stop. The status/dashboard runtime must still attempt live RPC bootstrap
  from the selected datadir, network, cookies, and defaults when the config
  path is only the implicit datadir-local file and it does not exist.
- **D-02:** Keep `resolve_startup_config()` as the authoritative RPC bootstrap
  engine. Phase 30 should repair the operator-side inputs passed into that
  startup path instead of inventing a separate status-only RPC config parser.
- **D-03:** Build startup arguments from `OperatorConfigResolution` with
  concrete precedence-aligned values: selected datadir always, selected network
  when known, and `-conf` only when the resolved `bitcoin.conf` path actually
  exists. Do not require an on-disk `bitcoin.conf` just to derive default RPC
  behavior.

### Shared Status And Dashboard Truth
- **D-04:** `open-bitcoin status` and `open-bitcoin dashboard` must continue to
  share the same `status_runtime_parts()` bootstrap and
  `collect_status_snapshot()` collector. No dashboard-only bootstrap fork is
  allowed.
- **D-05:** The repair must preserve Phase 24 behavior: wallet-specific RPC
  ambiguity or failure degrades wallet fields only, while node reachability
  remains tied to node-scoped RPC success.

### Fallback Semantics
- **D-06:** Keep the current stopped/unreachable fallback boundary. If live RPC
  cannot be bootstrapped because credentials are genuinely unavailable or the
  daemon is unreachable, status/dashboard should still fall back to explicit
  unavailable fields instead of crashing or inventing running-state data.
- **D-07:** The Phase 30 fix should be narrow: it closes the
  missing-implicit-config blocker without reworking service status, build
  provenance, or the dashboard presentation model.

### Verification And Documentation
- **D-08:** Add focused regression tests around the shared bootstrap helper and
  the documented flag-only workflow. Cover the absent-implicit-config case plus
  a counterexample that preserves stopped-node fallback when live RPC truly
  cannot start.
- **D-09:** Update operator-facing docs only where needed to keep the documented
  flag-only `open-bitcoind` -> `open-bitcoin status/dashboard` workflow
  truthful and aligned with the repaired runtime bootstrap.

### Claude's Discretion
- Helper names and exact function extraction are flexible as long as startup
  derivation stays small, pure where practical, and obviously shared by status
  and dashboard.
- The implementation may either add a small operator-to-startup conversion
  helper or lightly reshape the current `startup_config_for_status()` path,
  provided the behavior and tests stay clear.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and audit evidence
- `.planning/ROADMAP.md` - Phase 30 goal, dependencies, requirements, and
  success criteria.
- `.planning/REQUIREMENTS.md` - `OBS-01`, `DASH-01`, and `VER-07` traceability
  for this gap-closure phase.
- `.planning/PROJECT.md` - v1.1 operator-surface truthfulness, terminal-first
  scope, and verification expectations.
- `.planning/v1.1-MILESTONE-AUDIT-PHASE-29.md` - blocker `INT-v1.1-02` and
  broken flow `FLOW-v1.1-02`, including the exact evidence for the
  missing-implicit-config bootstrap defect.
- `.planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md` -
  original status/onboarding config-precedence and stopped-node status
  decisions.
- `.planning/phases/24-wallet-aware-live-status-and-build-provenance/24-CONTEXT.md`
  - locked wallet-aware live status and shared snapshot truth decisions.
- `.planning/phases/29-closeout-hygiene-and-build-provenance/29-CONTEXT.md` -
  recent operator truthfulness and compile-time provenance guardrails that must
  not regress.

### Runtime and config contracts
- `docs/operator/runtime-guide.md` - documented flag-only local workflow for
  daemon start, `status`, and `dashboard`.
- `docs/architecture/config-precedence.md` - operator config layering contract
  and ownership split between `open-bitcoin.jsonc` and `bitcoin.conf`.
- `docs/architecture/status-snapshot.md` - shared snapshot semantics,
  unavailable-field behavior, and truthful operator output expectations.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - shared
  `status_runtime_parts()` bootstrap and current `startup_config_for_status()`
  defect site.
- `packages/open-bitcoin-cli/src/operator/status.rs` - shared live/stopped
  snapshot collector and fallback semantics.
- `packages/open-bitcoin-cli/src/operator/dashboard/mod.rs` - dashboard reuse of
  the shared status snapshot.
- `packages/open-bitcoin-cli/src/operator/config.rs` - config-resolution
  evidence and existing runtime loading that already tolerates a missing
  implicit `bitcoin.conf`.
- `packages/open-bitcoin-cli/src/startup.rs` - authoritative CLI startup config
  resolution path used to derive RPC endpoint/auth.
- `packages/open-bitcoin-cli/src/operator/status/http.rs` - live RPC adapter
  that consumes the derived startup config.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - existing shared
  status tests and likely regression-test surface for this repair.

### Repo workflow and standards
- `AGENTS.md` - repo-local verification contract, parity breadcrumbs, and GSD
  workflow rules.
- `AGENTS.bright-builds.md` - sync-first, visible-truth, functional-core, and
  repo-native verification guidance.
- `standards-overrides.md` - local standards exception ledger (currently no
  substantive overrides recorded).
- [Bright Builds `standards/core/architecture.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md)
  - functional-core / imperative-shell and domain-type rules.
- [Bright Builds `standards/core/code-shape.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md)
  - early returns, `maybe` naming, and module-shape guidance.
- [Bright Builds `standards/core/verification.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md)
  - sync-first and repo-native verification rules.
- [Bright Builds `standards/core/testing.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md)
  - focused AAA unit-test expectations.
- [Bright Builds `standards/languages/rust.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md)
  - Rust-specific `let...else`, `maybe_`, and thin-adapter guidance.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `status_runtime_parts()` already constructs the shared status/bootstrap inputs
  used by both `open-bitcoin status` and `open-bitcoin dashboard`.
- `resolve_operator_config()` already resolves datadir, network, config-path
  evidence, and auth metadata without requiring an on-disk `bitcoin.conf`.
- `load_runtime_from_evidence()` in `operator/config.rs` already proves the
  repo's config loader can operate with `-datadir` and `-chain` alone, only
  adding `-conf` when the file exists.
- `CliStartupArgs` plus `resolve_startup_config()` already provide the
  authoritative RPC endpoint/auth derivation path the operator runtime should
  reuse.
- `collect_status_snapshot()` already has the correct shared fallback model for
  running, stopped, and unreachable snapshots once a live RPC client is or is
  not available.

### Established Patterns
- Operator truth repairs belong in shared typed helpers, not renderer-local
  branches or dashboard-only forks.
- Unavailable live data should surface explicit `Unavailable` reasons instead of
  silently collapsing fields or inventing defaults.
- Focused status tests are the preferred regression surface for operator
  bootstrap and snapshot truth behavior.
- Rust helpers should stay small, use early returns, and keep effectful
  filesystem/RPC work at the shell boundary.

### Integration Points
- The Phase 30 code change should land where `operator/runtime.rs` turns
  `OperatorConfigResolution` into `CliStartupConfig` for shared
  status/dashboard use.
- Tests will likely touch `operator/status/tests.rs` and possibly operator
  runtime tests that exercise the shared bootstrap helper.
- Operator docs should stay aligned with the same runtime/bootstrap logic used
  in code so the documented flag-only workflow remains evidence-backed.

</code_context>

<specifics>
## Specific Ideas

- Honor the exact documented local workflow in `docs/operator/runtime-guide.md`:
  start `open-bitcoind` with `-regtest` and `-datadir`, then run
  `open-bitcoin --network regtest --datadir=... status` or `dashboard` without
  first creating a `bitcoin.conf`.
- Prefer a single shared helper that says, in effect, "use the resolved datadir
  and network, and only pass `-conf` when that path actually exists," rather
  than duplicating config bootstrap rules in multiple places.
- Keep the repair honest about limits: if credentials or the daemon itself are
  truly unavailable, the output should still fall back to explicit unavailable
  fields rather than pretending the node is running.

</specifics>

<deferred>
## Deferred Ideas

- Adding explicit operator RPC override flags or a broader operator-side RPC
  config surface is a separate capability and stays out of scope for Phase 30.
- Persisting fully materialized runtime RPC config inside
  `OperatorConfigResolution` could be a later cleanup, but this phase only
  needs the narrow shared bootstrap repair.

</deferred>

---

*Phase: 30-configless-live-status-and-dashboard-bootstrap*
*Context gathered: 2026-04-29*
