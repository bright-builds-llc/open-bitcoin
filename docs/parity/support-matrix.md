# Support Matrix And Issue Evidence

Surface id: `v1-8-support-matrix-issue-evidence`

Phase 83 is a support classification and issue-evidence policy, not a production full-node readiness claim. It turns the Phase 82 boundary into a
reader-facing classification surface for operators, contributors, and release
reviewers without widening any deferred production-adjacent surface.

Use this document with the Phase 82
[`production-claim-boundary.md`](production-claim-boundary.md) and the operator
[`runtime-guide.md`](../operator/runtime-guide.md). Artifact existence alone is not an evidence basis; support changes require the fields, verifier, residual risk, and next gate named below.
For operational issue-evidence collection, use the canonical
[`operator-runbooks.md`](operator-runbooks.md); it preserves these support
terms while routing operators through preflight, monitoring, no-progress,
recovery, redaction, and escalation evidence.
For service operation classification, command evidence, restart/resume fields,
and production-service non-claims, use the canonical
[`service-operation-expectations.md`](service-operation-expectations.md).
For release review, use the v1.8 release-readiness checklist in
[`release-readiness.md`](release-readiness.md#v18-release-readiness-checklist);
it points back to this support matrix rather than duplicating the table.
The v1.8 deterministic claim guardrails prevent overbroad
production-readiness and deferred-surface claims in the public release/operator
docs; they do not claim production full-node readiness.
v1.9 adds bounded opt-in inbound listener/admission, permission,
address-boundary, eviction/ban, and resource-governance review evidence through
the runtime guide and parity roots. It does not claim production full-node
readiness, public inbound defaults, transaction relay, compact block relay,
mempool propagation, full address relay, production-service operation, or
production network participation.

## Support Terms

The support terms are exactly the Phase 82 terms:

- `supported` - evidence-backed source-built behavior covered by default
  verification and documented review paths today.
- `preview` - shipped behavior available for operator review without a support
  commitment.
- `opt-in UAT` - explicit operator-run evidence outside default verification.
- `unsupported` - local experimentation or historical compatibility without a
  support expectation.
- `deferred` - not shipped, not safe to rely on, or not in scope until a future
  milestone names gates and evidence.

Do not add alternate support labels to this document. New rows and support-term
changes must preserve the Phase 82 vocabulary and link back to
[`production-claim-boundary.md`](production-claim-boundary.md).

## Support Matrix

| Environment family | Support term | Evidence basis | Default verification | Opt-in UAT / manual validation | Residual risk | Next gate |
| --- | --- | --- | --- | --- | --- | --- |
| source-built install and repo verification | `supported` | Source build path in [`runtime-guide.md`](../operator/runtime-guide.md), repo-native verifier, pinned Rust/Bun/Bazel context, and Phase 82 boundary docs. | `bash scripts/verify.sh` covers first-party formatting, linting, build, tests, parity breadcrumbs, bounded smoke checks, and Bazel smoke. | Operators may repeat source build and verifier on their platform, but no public-network or service-manager action is required. | Platform-specific toolchain drift can still block a local checkout. | Keep `runtime-guide.md`, `rust-toolchain.toml`, `.bun-version`, and verifier evidence current before changing the term. |
| repo-local operator command forms through Cargo and Bazel | `supported` | Repo-local command forms in [`runtime-guide.md`](../operator/runtime-guide.md) and AGENTS.md guidance for Cargo and Bazel UAT commands. | `bash scripts/verify.sh` keeps CLI and Bazel smoke paths exercised. | Operators can run documented Cargo and Bazel forms against their selected datadir for issue evidence. | Operator aliases or installed binaries can differ from repo-local forms. | Add concrete command evidence and verifier coverage before changing the command support term. |
| local deterministic runtime, status, config, RPC, and support-bundle surfaces | `supported` | [`docs/architecture/status-snapshot.md`](../architecture/status-snapshot.md), [`docs/architecture/operator-observability.md`](../architecture/operator-observability.md), [`docs/operator/runtime-guide.md`](../operator/runtime-guide.md), and `bash scripts/verify.sh`. | `bash scripts/verify.sh` covers deterministic local runtime, status, config, RPC, and support-bundle behavior without public-network requirements. | Operators may collect local status and support-bundle evidence for a selected datadir. | Missing local fields must be reported as unavailable rather than inferred as proof. | Promote only with a concrete evidence source, verifier command, residual-risk update, and next gate. |
| operator dashboard and shipped operator convenience surfaces | `preview` | Runtime-guide dashboard and operator convenience docs plus deterministic CLI/dashboard checks. | Default verification checks shipped local surfaces where deterministic. | Manual review is acceptable for terminal repaint, raw-input, and operator ergonomics. | UI behavior can vary by terminal, shell, and platform. | Add deterministic coverage or accepted manual review criteria before changing the term. |
| public-network mainnet activation, full-sync, stay-current, and soak evidence | `opt-in UAT` | Runtime guide public-network UAT commands, release-readiness history, and parity catalog entries for outbound sync, full-sync, stay-current, and soak evidence. | Not part of default verification; no public-network live check belongs in `bash scripts/verify.sh` for this phase. | Operators explicitly run public-network, full-sync, stay-current, or soak commands and preserve bounded local reports. | Peer availability, timing, resource pressure, and network conditions remain environment-dependent. | Future release-readiness policy with scoped reviewer acceptance and retention rules. |
| storage/datadir resource-bound evidence and recovery diagnosis | `supported` | Runtime guide resource-bound and recovery sections, status snapshot fields, operator observability docs, and Phase 76/77 parity evidence. | `bash scripts/verify.sh` covers deterministic resource-bound and diagnosis behavior. | Operators may attach redacted local resource or recovery evidence for the selected datadir. | Diagnosis does not imply destructive repair or source datadir mutation. | Separate destructive-repair, backup, rollback, and operator-consent gate. |
| live storage pressure and long-run resource behavior | `opt-in UAT` | Runtime-guide resource pressure and soak guidance plus local reports from explicit long-running operator review. | Not part of default verification; large-disk and long-wall-clock checks stay out of `bash scripts/verify.sh`. | Operators may run bounded long-run review with explicit disk, cache, peer, log, metric, and support-bundle limits. | Live pressure depends on workload, platform, disk, and peer behavior. | Future resource-governance gate with bounded fixtures and field evidence. |
| launchd/systemd service-supervision previews | `preview` | Runtime-guide service preview docs, [`service-operation-expectations.md`](service-operation-expectations.md), and operator-runtime parity catalog rows for source-built local service flows. | Deterministic docs and CLI checks only; default verification avoids real service-manager mutation. | Operators can inspect generated service intent without mutating a real supervisor. | Preview files do not prove real service ownership or lifecycle behavior. | Platform-specific service expectation docs and deterministic checker coverage. |
| real launchd/systemd service-manager lifecycle | `opt-in UAT` | Runtime-guide service UAT commands, [`service-operation-expectations.md`](service-operation-expectations.md), and v1.5/v1.7 service restart/resume evidence. | Not part of default verification; no real `launchctl` or `systemctl` action belongs in `bash scripts/verify.sh`. | Operators explicitly install or run local user services and capture status/restart evidence. | Host policy, privileges, paths, and supervisor state vary by machine. | Future service-operation milestone with uptime, rollback, and platform acceptance gates. |
| migration dry-run | `supported` | Drop-in audit and migration catalog, runtime guide migration boundary, and dry-run operator planning behavior. | Deterministic migration planning checks remain local and read-only. | Operators may inspect source installs and target plans without source mutation. | Dry-run output does not authorize switch-over or external wallet mutation. | Migration apply design with backup, rollback, and source-service consent evidence. |
| migration apply, source service mutation, and source datadir rewrite | `deferred` | Drop-in audit catalog and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.8. | Source services, source datadirs, and wallets are high-value data and must not be mutated implicitly. | Future migration-apply safety plan with backup, rollback, consent, and source mutation evidence. |
| support bundle and support forensics | `supported` | Runtime guide support-bundle section, operator observability docs, and Phase 79 local support-forensics evidence. | `bash scripts/verify.sh` covers deterministic redacted local support evidence behavior. | Operators may create `support-evidence.json` and `support-evidence.md` locally and share the smallest redacted useful subset. | Bundle existence, raw logs, daemon startup, peer reachability, or elapsed time alone do not prove the reported condition. | Add field-level evidence, redaction, and reviewer acceptance before changing the term. |
| wallet current non-production slice | `preview` | Wallet catalog, runtime guide limitations, and current managed-wallet/operator wallet docs. | Existing deterministic wallet checks cover the current non-production slice. | Operators may inspect non-production wallet behavior only inside documented bounds. | Current wallet work is not approved for production funds, advanced restore, or broad external-wallet mutation. | Wallet safety threat model, backup/restore proof, audit, and operator UAT gates. |
| production-funds wallet use and safety | `deferred` | Wallet catalog and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.8. | Key safety, backup, recovery, signing, and support policy remain incomplete. | Future wallet-production safety gate with threat model, recovery drills, and audit evidence. |
| inbound serving | `opt-in UAT` | Bounded opt-in inbound listener/admission, permission, address-boundary, eviction/ban, and resource-governance evidence in [`runtime-guide.md`](../operator/runtime-guide.md), [`catalog/p2p.md`](catalog/p2p.md), [`release-readiness.md`](release-readiness.md), [`checklist.md`](checklist.md), and [`index.json`](index.json). | `bash scripts/verify.sh` remains deterministic and public-network-free; Plan 04 owns the aggregate Phase 95 checker. | Operators can run the documented loopback or synthetic inbound Cargo/Bazel commands and preserve redacted local status/support evidence. | This evidence does not claim production full-node readiness, public inbound defaults, production network participation, transaction relay, compact block relay, mempool propagation, full address relay, or production-service operation. | Public-default inbound serving, production network participation, relay, service, packaging, and production-readiness gates need future scoped evidence. |
| public inbound defaults and production network participation | `deferred` | Phase 95 boundary docs and the Phase 82 deferred-surface boundary. | None in default verification. | None in v1.9. | Public listener exposure by default, service operation, abuse handling, support policy, and production acceptance gates are unsatisfied. | Scoped public-default and production-network gate with firewall, support, release, abuse, resource, packaging, and UAT evidence. |
| address relay | `deferred` | P2P catalog and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.9. | Full address relay beyond Phase 92 bounded direct response evidence is not production-scoped. | Address-relay gate with privacy, poisoning, eviction, and parity evidence. |
| block serving | `deferred` | Chainstate/P2P catalogs and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.8. | Current block evidence is validation/download-oriented, not serving policy. | Block-serving gate with serving correctness, resource bounds, peer policy, and UAT evidence. |
| transaction relay | `deferred` | P2P catalog, mempool parity scope, and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.8. | Mempool transaction relay behavior is not production-scoped. | Transaction-relay gate with relay policy, DoS controls, and parity fixtures. |
| compact block relay | `deferred` | P2P catalog and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.8. | Compact-block protocol depth remains future work. | Compact-block relay gate with protocol fixtures, peer behavior, and recovery evidence. |
| signed packaging or package-manager distribution | `deferred` | Runtime guide limitations, operator-runtime catalog, and Phase 82 boundary. | None in default verification. | None in v1.8. | Source-built install is the only current supported install path. | Release-engineering gate for signing, provenance, reproducibility, and package-manager delivery. |
| Windows service integration | `deferred` | Runtime guide limitations and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.8. | Current service work targets local macOS/Linux operator review. | Windows service gate with install, supervision, rollback, and platform UAT. |
| hosted dashboards and GUI parity | `deferred` | Runtime guide limitations and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.8. | Current dashboard work is terminal-first and local; GUI parity is outside current headless scope. | Hosted-operations and GUI milestones with auth, privacy, state, accessibility, and release UAT gates. |
| automatic support-bundle upload | `deferred` | Runtime guide support-bundle boundary and Phase 82 deferred-surface boundary. | None in default verification. | None in v1.8. | Upload consent, privacy, retention, transport, and redaction policy are not designed. | Support-upload gate with explicit consent, privacy, retention, transport, and redaction evidence. |
| destructive repair | `deferred` | Recovery catalog rows, runtime guide recovery diagnosis, and Phase 82 boundary. | None in default verification. | None in v1.8. | Current recovery is diagnosis-only and non-mutating. | Destructive-repair policy with backup, rollback, corruption fixtures, and explicit operator consent. |
| public-network default checks, public-network CI, and release-blocking live sync | `deferred` | Runtime guide limitations, release-readiness history, and Phase 82 boundary. | None in default verification; `bash scripts/verify.sh` remains deterministic and public-network-free. | None in v1.8 except explicit operator UAT outside the default gate. | Public-network conditions are environment-dependent and can make default verification flaky or costly. | Public-network CI policy with flake budget, isolation, quotas, retention, and release-blocking criteria. |
| broad production-node readiness | `deferred` | Phase 82 production claim boundary, deviations register, and historical v1.3 through v1.7 scoped evidence. | None in default verification may prove this in v1.8. | None in v1.8. | Future gates are unsatisfied across service, P2P serving/relay, wallet safety, migration apply, packaging, support, and release policy. | Future production-readiness milestone after scoped evidence and deterministic claim guardrails are complete. |

## Issue Evidence Checklist

Issue reports should include the smallest useful redacted evidence set. For each
item below, include the evidence or write `Unavailable: <reason>` so reviewers
can distinguish missing evidence from a passing condition.

- Redacted support bundle files `support-evidence.json` and
  `support-evidence.md` when available.
- Relevant command output, copied from the command that reproduced the issue.
- Bounded redacted logs, log paths, or compact log summaries.
- A configuration summary for the selected datadir, including relevant config
  paths and whether Open Bitcoin-only JSONC was used.
- Service state, including whether no service manager was involved.
- resource-bound or resource-pressure evidence from status, support evidence,
  or the affected run report.
- recovery/progress evidence, including recovery category/action, progress
  credit, stall diagnosis, or no-progress reason when applicable.
- sync status evidence, including header, downloaded block, connected block,
  best-known tip, stay-current, and latest stop reason when applicable.
- version, commit, Rust, Cargo, Bun, and Bazel context from the checkout and
  active toolchain.
- Platform details: OS, CPU architecture, filesystem, shell, terminal when UI
  behavior matters, and whether the run used Cargo or Bazel.
- The exact repo-local command that reproduced the issue.

Copy-pasteable support-bundle collection forms:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

```bash
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

A daemon start, elapsed time, peer reachability, raw log tail, local report file,
or support bundle path is not sufficient by itself. The report needs the fields
above, the evidence basis, and any unavailable reasons needed to interpret the
selected datadir.

### Do Not Attach

- wallet private material
- raw wallet files
- RPC cookies
- rpcpassword
- rpcauth
- raw datadirs
- unredacted logs
- raw unbounded logs
- full peer tables with sensitive local data
- automatic support-bundle upload

## Contributor Update Rules

Matrix edits are support-policy changes, not prose cleanup. A new row or support
term promotion requires all of the following:

- A concrete evidence source linked from the row.
- A verifier or opt-in UAT command that a reviewer can run or inspect.
- A residual-risk statement that says what still is not proven.
- A next gate that names the future condition required before another term
  change.

Deferred surfaces cannot be promoted by prose-only edits. Keep the Phase 82
boundary links and support vocabulary intact when editing this file, including
`docs/parity/production-claim-boundary.md`,
`docs/parity/upgrade-and-rollback-policy.md`,
`docs/parity/release-readiness.md`,
`docs/parity/deviations-and-unknowns.md`, parity roots, README,
`docs/operator/runtime-guide.md`, and `scripts/verify.sh`.

Upgrade-policy changes must preserve
[`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md) as the
canonical source-built upgrade and rollback policy and must not create a second
support matrix in this file.

Runbook changes must preserve [`operator-runbooks.md`](operator-runbooks.md) as
procedural evidence guidance and must not change any support term in this
matrix.

Service expectation changes must preserve
[`service-operation-expectations.md`](service-operation-expectations.md) as the
canonical source for service classification, command evidence, lifecycle
labels, restart/resume fields, and production-service non-claims.

The Phase 83 checker scope is intentionally narrow: support matrix rows,
issue-evidence checklist content, residual-risk table entries, canonical links,
and exact support terms; broad all-doc production-claim scanning remains Phase 88 scope, not Phase 83 scope.

## Carried-Forward Residual Risks And Manual Validation

This table is descriptive and gate-oriented; it does not convert residual risks into release blockers unless an existing source already classifies them as blockers.

| Milestone | Surface | Handling status | Latest evidence source | Current support effect | Next gate |
| --- | --- | --- | --- | --- | --- |
| v1.1 | dashboard pseudoterminal/raw-input repaint and input behavior | accepted manual validation surface | `.planning/milestones/v1.1-MILESTONE-AUDIT.md` | Keeps the operator dashboard and terminal interaction surface at `preview` where real terminal behavior is manually reviewed. | Deterministic pseudoterminal harness or accepted manual review checklist before any term change. |
| v1.2 | closeout without a dedicated milestone audit artifact | historical closeout context | `.planning/MILESTONES.md` and `.planning/RETROSPECTIVE.md` | v1.2 remains historical opt-in IBD closeout evidence, not a current production claim. | Preserve Phase 40 and Phase 41 closeout references until a future archive normalization pass exists. |
| v1.3 | diagnosed-blocker closeout and fresh status supersession | opt-in UAT evidence | `.planning/milestones/v1.3-MILESTONE-AUDIT.md` and `docs/parity/release-readiness.md` | Fresh diagnosed-blocker evidence supports source-built opt-in review only; it does not prove public-network success. | Future public-network evidence gate with reviewer acceptance criteria for observed progress or typed blockers. |
| v1.4 | planning traceability correction during archive prep | historical closeout context | `.planning/milestones/v1.4-MILESTONE-AUDIT.md` and `.planning/MILESTONES.md` | v1.4 implementation evidence remains accepted, while stale planning metadata is historical archive context. | Keep requirements, roadmap, and summary traceability synchronized before future archive steps. |
| v1.3-v1.7 | public-network full-sync, stay-current, and soak evidence | opt-in UAT evidence | `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`, and `.planning/milestones/v1.7-MILESTONE-AUDIT.md` | Public-network review remains `opt-in UAT`; default verification stays deterministic and public-network-free. | Release-policy gate with flake budget, evidence retention, reviewer acceptance, and Phase 88 guardrails. |
| v1.5-v1.7 | real service-manager lifecycle evidence | opt-in UAT evidence | `.planning/MILESTONES.md`, `docs/parity/release-readiness.md`, and `.planning/milestones/v1.7-MILESTONE-AUDIT.md` | Real launchd/systemd lifecycle review remains `opt-in UAT`; service previews remain `preview`. | Phase 86 service expectations plus platform-specific service UAT and rollback evidence. |
| v1.7 | multi-day wall-clock soak evidence | opt-in UAT evidence | `.planning/milestones/v1.7-MILESTONE-AUDIT.md`, `.planning/MILESTONES.md`, and `docs/parity/release-readiness.md` | Multi-day soak evidence remains opt-in operator review and does not become a default gate. | Future release-policy decision before any long-wall-clock or public-network default check is introduced. |
| v1.7 | support-bundle forensics | verified deterministic behavior | `.planning/milestones/v1.7-MILESTONE-AUDIT.md`, `docs/parity/release-readiness.md`, and `docs/architecture/operator-observability.md` | Support-bundle forensics are supported as local redacted evidence; bundle existence and upload do not prove the reported condition. | Field-level evidence, redaction, size-bound, and reviewer acceptance before any broader support workflow. |
| v1.7 | recovery diagnosis versus destructive repair | verified deterministic behavior plus deferred/non-claim | `.planning/milestones/v1.7-MILESTONE-AUDIT.md`, `docs/parity/deviations-and-unknowns.md`, and `docs/parity/production-claim-boundary.md` | Recovery diagnosis is `supported`; destructive repair remains `deferred`. | Destructive-repair policy with backup, rollback, corruption fixtures, and explicit operator consent. |
| v1.8 | production-scope non-claims | deferred/non-claim | `docs/parity/production-claim-boundary.md`, `docs/parity/deviations-and-unknowns.md`, and `.planning/PROJECT.md` | Production-node readiness, production-funds wallet use, inbound serving, relay, migration apply, packaging, hosted dashboards, GUI, support upload, destructive repair, public-network CI, and release-blocking live sync remain `deferred`. | Future scoped milestones plus Phase 87 release-readiness checklist and Phase 88 claim guardrails. |
