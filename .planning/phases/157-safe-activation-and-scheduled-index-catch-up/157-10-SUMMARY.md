---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "10"
subsystem: verification
tags: [bun, rust, basic-filters, parity, source-provenance, mutation-controls]
requires:
  - phase: 157-07
    provides: Measured ordinary bounded turn and consumed production policy
  - phase: 157-08
    provides: Actual offline daemon activation and maintenance
  - phase: 157-09
    provides: Actual paired loss, accepted failures and ordinary retention
provides:
  - Decision-indexed ordinary source, behavior, claim and resource regression gates
  - Exact scoped option/history/resource documentation and Cargo/Bazel UAT
  - Historical configured pre-prune guards and recovered constructor assertions
  - Matching registry entries for all 32 new Rust paths and default verifier stages
affects: [phase157-verification, phase158, phase159, phase160, phase161, phase162]
tech-stack:
  added: []
  patterns: [bounded-source-contracts, independent-snapshot-mutations, existing-rust-masking]
key-files:
  created:
    - scripts/check-phase157-index-catch-up.ts
    - scripts/check-phase157-index-catch-up.test.ts
    - scripts/check-phase157-index-catch-up/contracts.ts
    - scripts/check-phase157-index-catch-up/rust-evidence.ts
    - .planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-UAT.md
  modified:
    - docs/parity/index.json
    - docs/parity/catalog/basic-compact-filters.md
    - docs/parity/checklist.md
    - docs/parity/source-breadcrumbs.json
    - README.md
    - scripts/verify.sh
    - scripts/check-phase155-filter-index.ts
    - scripts/check-phase155-filter-index.test.ts
    - packages/open-bitcoin-node/src/sync/tests/restart_chainstate/persist_and_hydrate.rs
key-decisions:
  - Keep the activation surface in_progress and requirements unactivated pending root native/security/formal closure.
  - Preserve inherited PreserveSaved recovery/protection semantics; full suffix preflight belongs to configured Enabled fresh/saved activation.
  - Retain exact default-off scalar/list semantics and document empty-store refusal plus omitted double-negative warning.
  - Support scoped done only with lifecycle-matched passing verifier and zero-open-threat security report metadata.
requirements-completed: [CFAC-01, CFAC-02, CFIX-01]
requirements-addressed: [CFAC-01, CFAC-02, CFIX-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:50:41Z
duration: 22min closing verification/documentation window
completed: 2026-10-05
commits: []
git_finalization: pending root whole-phase native verification and strict finalization
---

# Phase 157 Plan 10: Scoped parity and native regression gates Summary

**Actual offline BASIC option/startup/turn/accepted-state contracts now have exact resource and refusal documentation, complete new-source provenance, and 556 independent native source/claim mutation controls.**

## Performance and Scope

- **4/4 tasks complete**, across the **14 declared paths** plus this summary.
- The explicitly clock-observed closing verification/documentation window is
  **2026-10-05T21:28:18Z–2026-10-05T21:50:41Z**, approximately **22 minutes**.
  Earlier read-only preparation and uninstrumented initial implementation are
  excluded; this is not a fabricated total implementation duration.
- The root released execution only after final Plans 08/09 summaries/self-checks.
  Final Plan 07 measurements and actual source supplied the policy; no resource
  numbers or runtime results were inferred from constants alone.
- One focused helper owned only the two Phase 155 scripts and restart assertions.
  Other agents' production/test changes were preserved. No production logic,
  dependency/crate, schema, public endpoint or network activation was added.
- **No staging, task/TDD/metadata commit, push, STATE/ROADMAP/REQUIREMENTS/config
  change or requirement activation occurred.** Root owns every such operation.

## Task Outcomes

### Task 1: Real ordinary source and claim guard

The four small Bun files contain one D-01–D-14 decision inventory, narrow
ordinary function/body checks, named assertions/helper resolution, module
registration and independent filesystem snapshots. They reuse Phase 156's
comment/literal masker and offset conventions; no parser framework or
dependency was added. Private/public/qualified test modules and test-only
functions cannot replace an ordinary caller.

Contracts follow resolver/loader, actual durable daemon selection, configured
pre-prune initialization, first turn, concrete Fjall maintenance adapter,
elapsed worker and accepted pre-persist handoff. Captured store/generation/
revision/frontier identity, monotonic budget tightening, admitted undo work,
body pre-decode admission, generation accounting, append predecessor checks
and exact-tip safe publication remain required. Known full-fence/forest/
projection/ancestor scans are rejected in the ordinary turn and selected
bounded helpers.

Every published normal/singleton table field and consumed production default
has an independent control. Each parity string/subject is checked separately;
unrelated deferred prose or another subject's negation cannot authorize peer,
RPC, runtime-reorg, full-sync, production/funds or milestone-shipped claims.
Source checks supplement executed Rust/native evidence: they do not execute
Rust, prove arbitrary call graphs or establish performance.

The guard supports current `in_progress` and subsequent scoped `done`.
Done requires same-lifecycle `gsd-verifier` passed status and
`gsd-secure-phase` zero-open-threat frontmatter. Synthetic fixture metadata
tests that gate only; it is not reported as formal runtime/security proof.

### Task 2: Exact scoped documentation and UAT

The parity ledger/catalog/checklist/README and UAT record CLI-last/config-first
scalar selection, source priority, merged named lists, negation resets/revival,
active network/includeconf behavior and the complete tested truth table.
Single/double dash, bare and equals forms are the guarantee; separate next-token
values refuse and JSONC has no index field. V0/type 2 remains excluded.

Explicit BASIC or zero with an existing datadir selects durable inspection.
Omission without another durable trigger stays transient; every actual durable
daemon open applies Enabled or Disabled before prune resume. Indexing does not
activate sync, DNS/peers, P2P inbound listeners, relay or filter service bit 6.
Existing authenticated local JSON-RPC server behavior remains unchanged.

Two deliberate differences are explicit: enabled empty history refuses
validated genesis history required because no complete canonical genesis/body
bootstrap installer exists; pinned double-negative selection omits the warning.
Linked SyncNetwork/consensus and body-bound input sources identify the actual
retained validated-genesis prerequisite. Operator commands use existing Open
Bitcoin history, not a Knots datadir/import or invented repair/bootstrap path.

Docs separate startup-wide recovery/suffix preflight from eight-block admitted
turns; accepted, processed, initial completion, later lag and safe durability;
missing body versus non-genesis undo; stronger covering protection; and exact
durable-tip release. Normal Periodic/Always remains the coins/deletion owner.
The measurement link records complete reservations/actual observations, legal
989,871-byte singleton and 16/128/512-prefix evidence. Reservations do not
claim RSS/comparison counts, every consensus-legal output representation,
physical disk capacity or bounded hardware/fsync latency. The existing
10,000-byte representation limit, map refusal, 384 MiB retained-fact cap and
conservative sustained-growth retention are explicit.

Actual Plan 09 evidence earns 714 paired deletes with 1,002 continuously
validated active blocks plus 584 codec-valid nonactive volume bodies. Raw
namespace equality is distinguished from streamed large-volume commitments.
All v2.4 advisories and Phase 158–162 exclusions remain intact. Copy-pasteable
timing-wrapped Cargo/Bazel daemon and existing status forms have verified target
spellings; no new index status field, RPC or peer response is promised.

### Task 3: Historical startup guards and Rust assertions

The observed frozen-source Phase 155 baseline was **45 passed / 8 failed**
among 53 tests, with **four live findings**. Besides stale initialize/constructor
routing, the old publication literal `128` had become named consumed ceilings.

The guard now selects `initialize_configured`, checks unconditional mandatory
dispatch after coins recovery and before locks/resume/readiness/cache, verifies
PreserveSaved wrappers and configured forwarding, and follows the actual
sealed `ManagedChainstate::from_recovered_chainstate` constructor. PreserveSaved
retains independent full-index recovery and strong pre-prune intent protection.
Enabled independently requires fresh/saved full suffix preflight before
mutation or Active return, with lexical-depth controls for conditional bypass.
No new compatibility history behavior or production edit was introduced.

Forty added route/preflight/ordering/comment/literal/test-module controls bring
Phase 155 to **93 passing tests**. Phase 156 remains unchanged and passing.
Both old Rust constructor assertions now reference the actual recovered
constructor; the existing behavioral reopen/hydration controls pass without
fake legacy calls or comments.

### Task 4: Source provenance and default verification

Root alone added exact intent-to-add inventory entries; this executor staged
nothing. All **32 new Phase 157 Rust paths** now have one mapped group matching
their immediate pinned source blocks: index/base/index/blockfilterindex/
blockstorage, accepted-state validation, or init/args/settings as appropriate.
No unsupported none breadcrumb was introduced. The normal checker verifies
**997 tracked first-party Rust files**.

The live and explicit-file mutation stages execute after Phase 156 in the
default verifier and its help text. Existing Rust, coverage, benchmark, Bazel
and policy stages remain intact.

## Verification and RED/GREEN Evidence

Final applicable checks pass; counts overlap and must not be summed as distinct
runtime-test coverage.

| Check                                                               | Actual result                                                         |
| ------------------------------------------------------------------- | --------------------------------------------------------------------- |
| Task 1 initial RED, before checker module existed                   | Exit failure; 0 passed, 1 failed, 1 import error                      |
| Final Phase 157 explicit mutation file, frozen shutdown source      | **556 passed, 0 failed, 1,470 assertions; 50.53s**                    |
| Phase 157 live source/claim/provenance guard                        | **Passed**                                                            |
| Phase 155 final explicit mutation file                              | **93 passed, 0 failed, 183 assertions**                               |
| Phase 155 live source/parity guard                                  | **Passed**                                                            |
| Unchanged Phase 156 explicit mutation file                          | **84 passed, 0 failed, 151 assertions**                               |
| Phase 156 live source/parity guard                                  | **Passed**                                                            |
| Timing-wrapped node restart/hydrate Rust controls                   | **5 passed**, 1,248 filtered out                                      |
| Scoped Rustfmt for restart assertions                               | **Passed**                                                            |
| Standard breadcrumb `--check`                                       | **997 Rust files verified**                                           |
| Managed `bright-builds-check.ts all`                                | **Zero findings**, 1,320 files scanned                                |
| `bash -n`, `shfmt -i 2 -d scripts/verify.sh`, owned diff whitespace | **Passed**                                                            |
| Installed mdformat 1.0.0 + GFM 1.0.0/frontmatter 2.1.2              | Catalog/UAT passed; new README/checklist blocks canonically formatted |

The first mutation implementation removed only one of repeated equivalent
anchors, so five cases did not remove the intended full boundary. Scoped
all-occurrence removal corrected the test control; the 334-case suite passed,
followed by 470 and 511 expanded passing suites. Later two positive fixtures
correctly refused while production shutdown test strengthening and ledger
links were still being synchronized. Final snapshots run only after production
and test freeze and pass all 556. These are source-control/test-construction
discoveries, not claimed runtime bug RED evidence.

Whole README/checklist formatting already differed before these edits. The
installed formatter was probed and check mode run first; only the new sections
were normalized. Unrelated existing Markdown was preserved. The canonical shell
style uses two-space indentation; that scoped formatter check passes.

## Independent Shutdown Finding and Integration

Root independent daemon review found WR-01: a retained coins/index shutdown
error could return before retry/mempool settlements. Root's separate GSD fix
owns the production files, not this executor. Its frozen
`settle_daemon_shutdown` eagerly evaluates sync, coins, retry, checkpoint and
serve results, settles every callback before returning the first failure,
and writes the clean marker only after complete success.

The closing guard now requires those actual outer callbacks, eager ordering,
first-error/clean-marker gates and all four named controls. Independent
early-`coins()?` and real retry-join removal mutations fail. The concrete
regression uses a genuine accepted successor, backlog, missing future body,
real CoinsFlushWorker thread/signal/join, successful Always fence, real retry
and mempool settlement, withheld clean marker and retained Active protection.

The fixer reports **4/4 targeted controls**, **63/63 inherited daemon controls**
(before test-only harness strengthening), strict normal RPC Clippy and scoped
Rustfmt; Phase 135 live and **88/88 mutations** pass. These are handoff results
from the production owner, not Cargo commands executed by this Plan 10
executor. Root independent rereview, security, full native and formal
verification remain **pending**; no shutdown/phase closure is asserted here.

## Decisions, Simplification and Deviations

AGENTS and its Bright Builds sidecar, placeholder-only overrides, architecture,
code-shape, testing, verification, local-guidance and Rust/Bun standards
materially informed this work. Both active lessons were completely loaded
within **7,188 bytes / 2,397 estimated tokens**; no archive or new audit trigger
was used. No project skill index exists.

The simplification pass reused the established masker, kept declarative
contracts separate from traversal, bounded diagnostics to 40 categories/paths,
used existing snapshots and small helpers, and avoided a dependency or custom
parser framework. New guard files are **195/327/146/48 lines** at final check,
within the managed file-length gate.

- Root clarified that PreserveSaved must preserve inherited protection/recovery;
  Enabled owns the full required-suffix preflight. An initial proposed extra
  compatibility preflight contract was removed before final verification.
- **[Rule 1 / historical verification]** The additional stale Phase 155
  publication-number anchor follows real named consumed limits; no production
  literal or fake call was added.
- **[Rule 2 / closing regression evidence]** Root's WR-01 fix required additional
  outer shutdown source/test contracts and mutations in the four already-owned
  guard files. Production repair and its separate review report remain root
  owned; no scope expansion to production occurred.
- Strict wrapper overrides per-task commits and all shared state/requirements
  updates. The GSD CLI roadmap format limitation is root documented/handled;
  no global CLI was changed here.

## Task Commits

Tasks 1–4 and summary metadata are **pending root strict Git finalization**.
`commits: []`; no hashes, content staging or push are claimed.

## Threat Review and Known Limits

T-157-29 is addressed by independent string/subject claims, actual numeric
default/table controls, earned source/test/measurement links and explicit
fresh-start/diagnostic differences. T-157-30 is addressed by ordinary-source
masking, concrete callers and registered assertions/helpers. T-157-31 covers
known full-scan regressions, total ledger/defaults, executed measurements and
default stages. T-157-32 diagnostics contain bounded categories/fixed paths,
not script/credential/history payload dumps. T-157-33 retains unconditional
configured pre-prune protection and independent PreserveSaved/Enabled controls.

No blocking production stub or new production endpoint/auth/schema trust
surface was found. New guard reads stay within the established repository
source/metadata evidence boundary. Synthetic incomplete/empty mutation
fixtures are intentional negative controls, not shipped data stubs. Source
contracts do not prove arbitrary Rust safety/performance; software faults and
closed reopen do not establish hardware power loss, public-mainnet, archive
scale, production readiness or funds safety.

## Root Handoff and Next Readiness

All four targeted closing tasks are complete and source is frozen. Keep the
ledger `in_progress` and `requirements-completed: []`. Root must complete final
guard/docs review and daemon WR-01 rereview, security audit, default
`bash scripts/verify.sh`, formal lifecycle/goal verification and strict
commit/push. Root can then change only the earned scoped ledger status and
requirement activation. Phase 157 does not complete v2.5 or activate deferred
158–162 products. No user setup/authentication gate was required.

## Self-Check: PASSED

All fourteen declared paths and this summary exist. Exact lifecycle metadata,
two standalone frontmatter delimiters and empty requirement activation were
checked. Final frozen-source evidence is 556/93/84 mutation controls, five
focused Rust restart controls, three live guards, 997 breadcrumbed Rust paths,
zero managed findings and passing applicable scoped style/shell/diff checks.
Stub/threat scans found no blocking stub or new production trust surface.
Commit-existence checks are inapplicable under explicit root deferral; no hash
or full native/phase completion is asserted. Root review/security/formal/native
and strict Git gates remain pending.

## Root completion review 2026-10-05T23:21:26.518Z

CFAC-01/02 and CFIX-01 are activated only after the complete native contract,91-file final source review,33/33 security closure,34/34 formal goal report and valid lifecycle. The scoped ledger is done; milestone v2.5 and Phases158–162 remain pending. Final guards include568 controls plus independently reviewed historical123/128/134 corrections. All Git task/TDD/metadata work remains consolidated in root's normal hook-enforced saving commit and upstream transport; no pre-commit hash is invented.
