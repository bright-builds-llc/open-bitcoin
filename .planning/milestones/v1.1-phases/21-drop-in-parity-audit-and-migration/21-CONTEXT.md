---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 21-2026-04-27T23-11-20
generated_at: 2026-04-27T23:11:20.765Z
---

# Phase 21: Drop-In Parity Audit and Migration - Context

**Gathered:** 2026-04-27
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Make Open Bitcoin's replacement story evidence-based, explicit, and safe for operators who already have Bitcoin Core or Bitcoin Knots installs. This phase audits the real drop-in surface, adds a dry-run migration planner, and records intentional differences in the parity ledger. It does not execute automatic migration, rewrite source datadirs, import external wallets, copy source wallet files, or claim full replacement parity before the audit evidence exists.

</domain>

<decisions>
## Implementation Decisions

### Migration command surface
- **D-01:** Add a new operator-owned `open-bitcoin migrate plan` command instead of extending `onboard`. `onboard` remains the first-run Open Bitcoin config wizard; migration is a separate source-install inspection and planning workflow.
- **D-02:** Phase 21 stays dry-run only. The migration command may explain what would happen and what remains manual, but it must not mutate existing Core/Knots files, service definitions, wallet data, or permissions.
- **D-03:** Existing global `--datadir`, `--config`, and `--network` flags continue to describe the Open Bitcoin target environment. Source-install selection uses migration-specific input, with `--source-datadir` as the explicit selector when the detected source is ambiguous.

### Planner behavior and safety
- **D-04:** Reuse Phase 17 detection evidence and Phase 20 wallet inspection instead of inventing a new scanner. When product family, service manager, or wallet format is uncertain, the planner must say so explicitly and fall back to manual-review actions instead of guessing.
- **D-05:** The migration planner must explain benefits, tradeoffs, unsupported surfaces, rollback expectations, and backup requirements before the action list. This explanation is part of the plan output, not a hidden doc-only behavior.
- **D-06:** The dry-run plan must enumerate the relevant action classes for the selected source install: config, datadir/files, service, wallet, logs/auth, and operator follow-up. If Phase 21 intentionally leaves an action manual or unsupported, the plan still lists that as an explicit no-op or manual-review item.
- **D-07:** Output must remain support-oriented and secret-safe. Paths and uncertainty are visible; cookie contents, RPC secrets, raw wallet bytes, and similar source data are never rendered.

### Parity audit and deviation tracking
- **D-08:** Phase 21 adds a new parity catalog entry for the drop-in audit/migration slice rather than scattering the audit across unrelated pages. The page acts as an evidence matrix across CLI, RPC, config, datadir layout, service behavior, wallet behavior, sync, logging, and operator docs.
- **D-09:** Intentional migration-relevant differences are stored canonically in `docs/parity/index.json` under `deviations[]`, summarized in `docs/parity/deviations-and-unknowns.md`, and linked from the new Phase 21 parity catalog page.
- **D-10:** Migration output surfaces only the relevant deviations for the selected dry-run plan. The CLI reads structured deviation metadata from `docs/parity/index.json`; it does not parse Markdown at runtime and does not invent a second deviation registry.

### Command and architecture boundaries
- **D-11:** Keep the operator clap tree authoritative for the Open Bitcoin-owned migration flow. `open-bitcoin-cli` remains the baseline-compatible RPC client path and must not be repurposed for migration planning.
- **D-12:** Keep migration decisions in pure planner helpers where practical, and keep filesystem, environment, and CLI rendering in the runtime shell. This preserves the functional-core and dry-run patterns already used by onboarding, service, and wallet flows.

### the agent's Discretion
- Exact human output section labels, JSON field ordering, and helper names are discretionary if dry-run semantics, redaction, and explicit uncertainty remain stable.
- `onboard` may add a light pointer toward `open-bitcoin migrate plan` when detections exist, but the migration workflow must not collapse back into onboarding questions or writes.
- The parity audit may record a conservative "manual only for now" migration action when the current codebase cannot yet support a safe automatic equivalent.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope
- `.planning/ROADMAP.md` - Phase 21 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` - `CLI-07`, `WAL-08`, `MIG-01`, `MIG-02`, `MIG-03`, `MIG-04`, and `MIG-05`.
- `.planning/PROJECT.md` - v1.1 operator-runtime constraints and the explicit dry-run-first migration posture.
- `.planning/STATE.md` - current milestone state and current-phase focus.
- `.planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md` - operator command boundary, config ownership, and read-only detection decisions.
- `.planning/phases/18-service-lifecycle-integration/18-CONTEXT.md` - dry-run-first service plan patterns and privilege transparency.
- `.planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md` - external-wallet inspection, backup-export, and read-only migration boundary decisions.

### Architecture and parity docs
- `docs/architecture/cli-command-architecture.md` - operator vs compatibility-path routing.
- `docs/architecture/config-precedence.md` - target config ownership and `bitcoin.conf` compatibility boundary.
- `docs/parity/index.json` - machine-readable parity ledger and deviation root.
- `docs/parity/README.md` - parity-ledger conventions.
- `docs/parity/catalog/README.md` - parity catalog conventions.
- `docs/parity/deviations-and-unknowns.md` - human-readable deviation and residual-risk rollup.
- `docs/parity/catalog/rpc-cli-config.md` - existing CLI/RPC/config parity story.
- `docs/parity/catalog/wallet.md` - current external-wallet and backup behavior.

### Existing implementation to extend directly
- `packages/open-bitcoin-cli/src/operator.rs`
- `packages/open-bitcoin-cli/src/operator/runtime.rs`
- `packages/open-bitcoin-cli/src/operator/detect.rs`
- `packages/open-bitcoin-cli/src/operator/onboarding.rs`
- `packages/open-bitcoin-cli/src/operator/service.rs`
- `packages/open-bitcoin-cli/src/operator/wallet.rs`
- `packages/open-bitcoin-cli/tests/operator_binary.rs`
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs`

### Bitcoin Knots baseline
- `packages/bitcoin-knots/doc/files.md`
- `packages/bitcoin-knots/doc/init.md`
- `packages/bitcoin-knots/doc/managing-wallets.md`
- `packages/bitcoin-knots/src/common/args.cpp`
- `packages/bitcoin-knots/src/common/config.cpp`
- `packages/bitcoin-knots/src/httprpc.cpp`
- `packages/bitcoin-knots/src/rpc/server.cpp`
- `packages/bitcoin-knots/contrib/init/org.bitcoin.bitcoind.plist`
- `packages/bitcoin-knots/contrib/init/bitcoind.service`

### Standards
- `AGENTS.md`
- `AGENTS.bright-builds.md`
- `standards-overrides.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable assets
- Phase 17 already detects existing Core/Knots datadirs, configs, cookies, service definitions, and wallet candidates read-only through `operator/detect.rs`.
- Phase 17 onboarding already follows a plan-then-apply split and preserves `bitcoin.conf` as a read-only compatibility surface.
- Phase 18 service commands already establish the dry-run preview contract and privilege-transparent action rendering.
- Phase 20 wallet work already classifies external wallet candidates for backup and migration planning and rejects overlap between managed-wallet backup export and detected external wallet destinations.

### Established patterns
- Operator-owned workflows default to explicit previews and require clear approval before any mutation-capable branch is eligible.
- Runtime shells gather environment and filesystem evidence once in `operator/runtime.rs`, then dispatch to focused modules.
- New Rust files under the scoped packages need parity breadcrumb comments and `docs/parity/source-breadcrumbs.json` coverage.
- Human and JSON operator output should be stable, quiet, support-oriented, and explicit about unavailable or uncertain evidence.

### Integration points
- `open-bitcoin migrate plan` should slot into the existing operator clap tree and runtime dispatch branch.
- A pure migration planner can consume `DetectedInstallation`, `WalletCandidate`, and target config inputs directly, then render human or JSON output without touching source bytes.
- Runtime deviation surfacing should read `docs/parity/index.json` once and map relevant records into the plan output, not create a parallel registry.

</code_context>

<specifics>
## Specific Ideas

- Prefer the smallest honest migration surface: a conservative dry-run planner is better than an apply path that overclaims safety.
- Keep source-install ambiguity visible. If the scanner cannot prove which install the operator means, the planner should stop short of actionable mutation and tell the operator how to narrow the target.
- Treat migration as an operator support workflow first: evidence, warnings, manual checkpoints, and linked deviations matter more than polished wizard theatrics.

</specifics>

<deferred>
## Deferred Ideas

- Automatic file rewrite, service switch-over, or wallet import/export from Core/Knots sources.
- Mutation-capable migration apply mode.
- Full drop-in replacement claim before the Phase 21 audit and Phase 22 hardening close.
- Windows migration/service guidance.
- Full upstream wallet migration semantics or direct reuse of Knots migration code paths.

</deferred>

---

*Phase: 21-drop-in-parity-audit-and-migration*
*Context gathered: 2026-04-27*
