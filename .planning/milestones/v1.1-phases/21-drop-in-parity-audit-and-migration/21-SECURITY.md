---
phase: 21
slug: drop-in-parity-audit-and-migration
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-04
generated_by: gsd-secure-phase
lifecycle_mode: yolo
phase_lifecycle_id: 21-2026-04-27T23-11-20
generated_at: 2026-05-04T10:09:06Z
---

# Phase 21 - Security

Per-phase security contract for Phase 21 drop-in parity audit and migration.
This audit verifies the security-relevant controls declared by the Phase 21
plans and summaries. It does not broaden into an unrelated vulnerability scan.

## Scope

This audit covers Phase 21 migration planning, the operator `migrate plan`
surface, migration notice synchronization with the parity ledger, the drop-in
audit catalog, and contributor-facing documentation. Phase 21 remains a
read-only, dry-run planning surface: it does not mutate source datadirs, disable
source services, import external wallets, or claim full drop-in replacement.

The Phase 21 plans do not contain literal `<threat_model>` blocks, and the
Phase 21 summaries do not contain `## Threat Flags` sections. The threat
register below is derived from the explicit security-sensitive must-haves and
task requirements in `21-01-PLAN.md` through `21-03-PLAN.md`.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Operator CLI input -> migration planner | User-controlled migration arguments choose the source datadir and output mode. | `migrate plan`, `--source-datadir`, target config flags, and output format. |
| Detection evidence -> rendered plan | Local Core or Knots config, cookie, wallet, datadir, and service metadata becomes terminal or JSON output. | Source paths, product confidence, uncertainty labels, service paths, wallet metadata, and cookie paths. |
| Source install -> Open Bitcoin target environment | Source evidence must stay read-only while target guidance remains separate. | Source `bitcoin.conf`, source datadir, source service definitions, source wallet paths, and target Open Bitcoin config/datadir paths. |
| Runtime migration notices -> parity ledger | Embedded migration deviation notices must stay aligned with machine-readable parity records. | Notice ids, summaries, severities, and docs paths. |
| Audit docs -> operator expectations | README, architecture, parity catalog, and ledger text shape user decisions about replacement readiness. | Dry-run scope, deferred mutation surfaces, intentional differences, and non-claims. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status | Evidence |
|-----------|----------|-----------|-------------|------------|--------|----------|
| T-21-01 | Tampering | `open-bitcoin migrate plan` source handling | mitigate | Keep migration planning dry-run only and route the command through rendering without apply/write branches; source config, datadir, service, and wallet actions are read-only checks, deferred work, or target-write previews. | closed | `packages/open-bitcoin-cli/src/operator/migration.rs`; `packages/open-bitcoin-cli/src/operator/migration/planning.rs`; `packages/open-bitcoin-cli/tests/operator_binary.rs` |
| T-21-02 | Information Disclosure | migration output rendering | mitigate | Render source paths and uncertainty but never cookie contents or raw wallet bytes. Binary tests preserve source files byte-for-byte and assert secret strings are absent from stdout. | closed | `packages/open-bitcoin-cli/src/operator/migration/tests.rs`; `packages/open-bitcoin-cli/tests/operator_binary.rs` |
| T-21-03 | Spoofing / Integrity | source selection and ambiguity handling | mitigate | Represent selected and manual-review-required outcomes explicitly. Ambiguous, partial, or unsupported source detections require manual review and point operators to `--source-datadir` instead of guessing. | closed | `packages/open-bitcoin-cli/src/operator/migration/planning.rs`; `packages/open-bitcoin-cli/src/operator/migration/tests.rs` |
| T-21-04 | Repudiation / Information Integrity | migration deviation notices and parity docs | mitigate | Embed runtime migration notices and keep their ids/summaries synchronized with `docs/parity/index.json`; document dry-run boundaries and intentional differences in the parity catalog, README, architecture docs, checklist, and deviations rollup. | closed | `packages/open-bitcoin-cli/tests/operator_flows.rs`; `docs/parity/index.json`; `docs/parity/catalog/drop-in-audit-and-migration.md`; `README.md`; `docs/architecture/cli-command-architecture.md` |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| Dedicated migration CLI contract | `open-bitcoin migrate plan --help` exposes `--source-datadir` under the operator `migrate plan` command. | verified |
| No source-mutation branch | `execute_migration_command` only handles `MigrationCommand::Plan`, builds a plan, renders it, and returns output; mutation-capable onboarding/service paths remain separate. | verified |
| Explanation-first dry-run plan | The migration renderer prints `Migration plan (dry run only)`, source-selection status, target environment, benefits, tradeoffs, unsupported surfaces, rollback expectations, backup requirements, grouped actions, and relevant deviation notices. | verified |
| Read-only source evidence | Action groups classify source datadir, config, cookie, service, and wallet evidence as read-only checks, manual steps, or deferred work; the binary test asserts seeded source config, cookie, and wallet bytes remain unchanged. | verified |
| Secret redaction | Unit and binary tests assert rendered output contains cookie paths but not cookie contents or raw wallet bytes. | verified |
| Ambiguity remains explicit | Planner tests cover multiple detections, explicit source selection, and unsupported explicit source paths, with manual-review-required outcomes instead of implicit selection. | verified |
| Service ownership ambiguity remains manual | Service association tests ensure only confidently matched service definitions are included, while ambiguous service ownership is surfaced as manual review. | verified |
| Runtime parity notices stay auditable | `migration_deviation_notices_match_parity_index` checks embedded migration notices against `docs/parity/index.json`. | verified |
| Contributor docs avoid overclaiming parity | README, CLI architecture docs, parity catalog, checklist, deviations rollup, and parity index preserve dry-run-only language and deferred surfaces such as service cutover, source-datadir mutation, external-wallet import or restore, and full drop-in replacement claims. | verified |
| Repo guardrails still pass | The repo-native verification contract passes, including hooks, LOC freshness, parity breadcrumbs, pure-core checks, panic-site classification, Cargo checks, benchmark smoke, Bazel smoke build, and coverage checks. | verified |

## Open Threats

No open threats.

## Accepted Risks Log

No accepted risks.

## Unregistered Flags

None. The Phase 21 summary files do not contain `## Threat Flags` sections.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-04 | 4 | 4 | 0 | Codex |

## Verification Evidence

| Command / Input | Result |
|-----------------|--------|
| Targeted scan for `<threat_model>`, `## Threat Flags`, `STRIDE`, and threat terms across `21-01-PLAN.md` through `21-03-SUMMARY.md`. | No literal threat model or summary threat flags found; four security-sensitive requirements derived from plan must-haves and task criteria. |
| Source review of `packages/open-bitcoin-cli/src/operator/migration.rs` and `packages/open-bitcoin-cli/src/operator/migration/planning.rs`. | Confirmed plan-only dispatch, explanation-first rendering, read-only/deferred action groups, manual-review outcomes, and embedded migration notices. |
| `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- migrate plan --help` | Passed; help exposes `open-bitcoin migrate plan` and `--source-datadir`. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_migrate_plan -- --nocapture` | Passed; 2 migration binary tests passed, covering dry-run behavior, source-byte preservation, custom source selection, and secret-safe output. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::migration::tests:: -- --nocapture` | Passed; 8 migration unit tests passed, covering explanation/action groups, redaction, ambiguity, service association, explicit source selection, and JSON deviations. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows migration_deviation_notices_match_parity_index -- --nocapture` | Passed; runtime migration notice ids and summaries match the parity ledger. |
| `rg -n "open-bitcoin migrate plan\|migrate plan --source-datadir\|drop-in-audit-and-migration\|mig-dry-run-only-switch-over\|mig-jsonc-open-bitcoin-settings\|mig-managed-wallet-backup-format\|full drop-in\|full replacement\|dry-run" README.md docs/architecture/cli-command-architecture.md docs/parity/catalog/drop-in-audit-and-migration.md docs/parity/index.json docs/parity/deviations-and-unknowns.md docs/parity/checklist.md` | Passed; docs and parity records expose the migration command, audit page, intentional differences, dry-run-only scope, and non-claim language. |
| `cargo fmt --all --manifest-path packages/Cargo.toml` | Passed. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed. |
| `bash scripts/verify.sh` | Passed. |

## Standards Inputs

Materially applied local `AGENTS.md`, `AGENTS.bright-builds.md`,
`standards-overrides.md`, Bright Builds pinned standards for verification and
testing, and the `gsd-secure-phase` workflow. ASVS Level: 1.

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-04
