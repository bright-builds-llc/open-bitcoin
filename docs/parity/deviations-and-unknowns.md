# Deviations And Unknowns

This page collects current deviations, known gaps, suspected unknowns, deferred
surfaces, and folded todo history for audit review. It summarizes existing
parity artifacts and completed planning follow-ups without adding new
implementation scope.

## Intentional Deviations

`docs/parity/index.json` now records these intentional in-scope migration
differences from Bitcoin Knots `29.3.knots20260210`:

- `mig-jsonc-open-bitcoin-settings`: Open Bitcoin-only onboarding, service,
  dashboard, migration, metrics, logging, storage, and sync settings live in
  `open-bitcoin.jsonc` rather than being written into `bitcoin.conf`.
- `mig-dry-run-only-switch-over`: Phase 21 migration is dry-run only and does
  not disable source services, rewrite source datadirs, or perform automatic
  switch-over.
- `mig-managed-wallet-backup-format`: Open Bitcoin exports managed-wallet
  backups as repo-owned JSON and does not copy or rewrite external
  `wallet.dat` files.

See [`catalog/drop-in-audit-and-migration.md`](catalog/drop-in-audit-and-migration.md)
for the evidence matrix behind those differences.

## Deferred Surfaces

The Phase 8 operator catalog records these deferred RPC, CLI, and config
surfaces:

- Richer send RPC ergonomics beyond the current deterministic build and sign
  extensions.
- Peer-info views and the human net-info dashboard that depends on them.
- Multi-endpoint wallet selection semantics.
- Remote-operator auth and ACL features such as rpcauth and whitelist handling.
- Wait-for-daemon, daemon supervision, and broader process-control helpers.

The Phase 9 verification catalog records these deferred harness and fuzzing
surfaces:

- Building or spawning the vendored Knots daemon from the shared harness.
- Translating the full upstream Python functional suite.
- Adding a dedicated cargo-fuzz or libFuzzer runtime beyond deterministic
  generated property tests.

These are deferred by scope decision, not silent omissions.

The Phase 21 migration audit adds these explicit deferred migration surfaces:

- automatic source-service disable, uninstall, or replacement
- source-datadir mutation or in-place cutover
- external-wallet import, restore, or rewrite
- any full drop-in replacement claim beyond the current dry-run audit evidence

The Phase 22 release-hardening slice keeps these additional surfaces out of the
current shipped claim:

- packaged or signed release installation flows beyond the current source-built
  path
- Windows service support
- production-node operation through `open-bitcoind`; Phase 40 adds explicit
  opt-in live smoke reports, and v1.5 later adds explicit opt-in unattended
  operator review, but neither milestone makes a broader production-readiness
  claim
- public-network sync as part of the default local verification contract
- hosted or public dashboard work beyond the local terminal dashboard
- timing-threshold benchmark gates that would fail or pass a release on elapsed
  numbers alone

### v1.3 Threat Model And Release Boundaries

The v1.3 threat model and release-boundary surface supports `PROOF-06`,
`SEC-01`, and `SEC-02` without expanding the shipped claim. v1.3 proves only a
source-built, opt-in, local-evidence public-mainnet sync review path or a
diagnosed environment/network blocker path.

The following remain explicit non-claims until separate future gates exist:

- inbound serving and address advertisement
- transaction relay and mempool propagation
- production-funds wallet use
- migration apply mode, source-service cutover, or source-datadir mutation
- packaging or signed release installation flows
- hosted/public dashboard work
- GUI parity
- unattended production-node readiness

The Phase 48 support bundle remains local redacted evidence only. It does not
make support tooling a release validator and does not add public-network checks
to `bash scripts/verify.sh`.

### v1.4 Operator Evidence And Release Boundaries

The v1.4 threat model and release-readiness roots support `OBS-01`, `OBS-02`,
`OBS-03`, `SEC-01`, `SEC-02`, and `SEC-03` without expanding the shipped claim.
v1.4 proves opt-in outbound IBD evidence only: outbound peer compatibility,
header progress, downloaded block progress, connected block progress,
same-datadir restart/resume evidence, redacted support evidence, or a typed
diagnosed blocker with next operator action.

The following remain explicit non-claims until separate future gates exist:

- inbound serving and address advertisement
- transaction relay and mempool propagation
- production-funds wallet use
- migration apply mode, source-service cutover, or source-datadir mutation
- packaging or signed release installation flows
- hosted dashboard or public dashboard operation
- GUI parity
- Windows service support
- unattended production-node operation

Generated live-smoke reports, support bundles, daemon logs, metrics stores, and
local datadirs stay local. Support evidence remains redacted review context; it
does not make support tooling a release validator, hosted upload path, or
default public-network verification gate.

### v1.5 Unattended Operation And Release Boundaries

The v1.5 threat model and release-readiness roots support `REL-01`, `REL-02`,
`REL-03`, and `REL-04` without expanding the shipped claim beyond source-built,
explicit opt-in extended unattended mainnet operator review readiness. v1.5
proves bounded daemon loop behavior, resource bounds, recovery states, long-run
truth surfaces, user-level service supervision, same-datadir service
restart/resume evidence, redacted support evidence, compatibility wrapper
reports, deterministic local verification, and auditable parity roots.

The following remain explicit non-claims until separate future gates exist:

- broad production-node operation or uptime guarantees
- inbound serving and address advertisement
- transaction relay and mempool propagation
- compact block relay
- production-funds wallet use
- migration apply mode, source-service cutover, or source-datadir mutation
- packaging or signed release installation flows
- hosted dashboard or public dashboard operation
- GUI parity
- Windows service support
- public-network CI or default live-smoke verification

Public-network live-smoke, manual-peer probing, `--restart-after-progress`,
`systemctl --user`, and `launchctl` remain explicit operator UAT outside
`bash scripts/verify.sh`. Generated live-smoke reports, compatibility reports,
support bundles, daemon logs, metrics stores, and local datadirs stay local.

### v1.6 Mainnet Full-Sync Completion And Release Boundaries

The v1.6 threat model and release-readiness roots support `REL-01`, `REL-02`,
and `REL-03` without expanding the shipped claim beyond source-built, explicit
opt-in full-sync completion evidence. v1.6 proves validated active-chain
progress to the best-known peer tip, durable restart/resume state,
stay-current review, reorg/no-progress/recovery handling, resource bounds,
shared status evidence, redacted support bundles, opt-in UAT commands,
deterministic local verification, and auditable parity roots.

The following remain explicit non-claims until separate future gates exist:

- broad production-node readiness or uptime guarantees
- inbound serving and address relay
- block serving
- transaction relay and mempool propagation
- compact block relay
- production-funds wallet safety
- migration apply mode, source-service cutover, or source-datadir mutation
- signed packaging or release installation flows
- Windows service support
- GUI parity
- hosted dashboards or public dashboard operation
- public-network CI or release-blocking live sync

Public-network full-sync, manual-peer probing, `--restart-after-progress`,
support-bundle collection, `systemctl --user`, and `launchctl` remain explicit
operator UAT outside `bash scripts/verify.sh`. Generated live-mainnet reports,
support bundles, daemon logs, metrics stores, compatibility reports, and local
datadirs stay local.

### v1.7 Full-Sync Soak and Recovery Hardening

The v1.7 release-readiness roots support `VER-05`, `VER-06`, `VER-07`, and
`REL-04` without expanding the shipped claim beyond source-built, explicit
opt-in full-sync soak and recovery hardening. v1.7 proves durable multi-day soak
evidence, disk and resource bounds, recovery diagnosis, progress guarantees,
stall diagnosis, support-bundle forensics, opt-in UAT commands, deterministic
release-boundary checks, and auditable parity roots.

The following remain explicit non-claims until separate future gates exist:

- inbound serving
- address relay
- block serving
- transaction relay
- compact block relay
- production-funds wallet use or safety
- migration apply mode, source-service cutover, or source-datadir mutation
- signed packaging or release installation flows
- Windows service support
- GUI parity
- hosted dashboards or public dashboard operation
- public-network default checks
- public-network CI
- release-blocking live sync
- automatic support-bundle upload
- destructive repair of corrupted stores or locks
- broad production-node readiness or uptime guarantees

Public-network full-sync and multi-day soak UAT remain explicit operator
workflows outside `bash scripts/verify.sh`. Support bundles, soak reports,
compatibility reports, daemon logs, metrics stores, live-mainnet reports, and
local datadirs stay local unless an operator explicitly shares redacted
artifacts.

## Suspected Unknowns

Current catalog entries preserve these review targets:

- Deprecated or ambiguous hex acceptance may still matter at some future
  user-facing boundaries.
- Additional serializer parameter contexts may need explicit typed Rust
  boundaries as disk and networking adapters grow.
- Address-codec, protocol, and witness edge cases may need more repo-owned
  fixtures as new surfaces become public.
- Cache-flush, multi-manager, and long-lived runtime policy behavior may need
  clearer ownership between pure core and shell adapters.
- Future peer governance, discovery, address relay, HD descriptors, multisig,
  PSBT, send, peer-info, and multi-endpoint semantics need scoped parity
  decisions before they are claimed complete.
- Future Knots-backed harness work must decide which upstream functional cases
  are translated into Rust and which are wrapped around a managed baseline
  process.
- A future apply-mode migration phase must decide how closely service cutover,
  datadir mutation, and wallet-import behavior should mirror Knots once those
  destructive paths become in scope.

## Folded Todo Audit

### AI-Agent-Friendly CLI Surface

The pending CLI todo is folded into audit evidence only. Phase 8 already proves
non-interactive CLI execution, stable JSON output for get-info, explicit exit
code failures, duplicate named-parameter rejection, and no hidden stdin wait on
normal invocations.

Evidence to inspect:

- [Phase 8 verification](../../.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md)
- [RPC, CLI, and config catalog](catalog/rpc-cli-config.md)
- [Original completed todo](../../.planning/todos/completed/2026-04-18-ai-agent-friendly-cli-surface.md)

Residual risk: broader command discovery, schema introspection, and dedicated
agent affordances remain future design work, not Phase 10 implementation scope.

### Panic And Illegal-State Exposure

The former panic and illegal-state sweep was completed as Phase 11. The phase
inventoried first-party production panic-like sites, replaced reachable crash
paths with typed failures, and added a repo-owned guard for future changes.

Evidence to inspect:

- [Phase 11 context](../../.planning/phases/11-panic-and-illegal-state-hardening/11-CONTEXT.md)
- [Phase 11 inventory](../../.planning/phases/11-panic-and-illegal-state-hardening/11-INVENTORY.md)
- [Original completed todo](../../.planning/todos/completed/2026-04-18-sweep-panics-and-illegal-states.md)
- [Repo verification contract](../../scripts/verify.sh)
- [Production panic-site guard](../../scripts/check-panic-sites.sh)

Residual risk: future production `expect`, `unwrap`, `panic!`, `unreachable!`,
`todo!`, or `unimplemented!` sites must either be fixed or added to the narrow
allowlist with a local invariant rationale. The allowlist is empty at Phase 11
close.

## Follow-Up Triggers

Update this page when:

- `docs/parity/index.json` gains a new intentional deviation.
- A deferred RPC, CLI, config, harness, fuzzing, or benchmark surface becomes
  in scope.
- A suspected unknown is resolved by evidence and can move into a catalog entry,
  deviation record, or release-readiness note.
- A folded todo becomes active implementation work in a dedicated phase.
- Phase 11's panic-site allowlist gains an entry that needs audit review.
