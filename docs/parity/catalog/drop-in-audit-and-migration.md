# Drop-In Audit And Migration

This entry tracks the Phase 21 audit slice for Open Bitcoin's replacement story
against the pinned Bitcoin Knots `29.3.knots20260210` baseline. It is
intentionally operator-facing: the goal is to show what a current Core or Knots
operator can expect today, what remains read-only or manual, and which
intentional differences the migration planner must surface before any later
cutover step.

## Coverage

- drop-in audit coverage for CLI, RPC, config ownership, datadir layout, service
  behavior, wallet handling, sync or logs expectations, and operator docs
- `open-bitcoin migrate plan` dry-run planning over detected Core or Knots
  installs
- explanation-first migration output covering benefits, tradeoffs, unsupported
  surfaces, rollback expectations, and backup requirements
- migration-relevant intentional difference tracking through
  `docs/parity/index.json`

## Knots sources

- [`packages/bitcoin-knots/doc/files.md`](../../../packages/bitcoin-knots/doc/files.md)
- [`packages/bitcoin-knots/doc/init.md`](../../../packages/bitcoin-knots/doc/init.md)
- [`packages/bitcoin-knots/doc/managing-wallets.md`](../../../packages/bitcoin-knots/doc/managing-wallets.md)
- [`packages/bitcoin-knots/src/common/args.cpp`](../../../packages/bitcoin-knots/src/common/args.cpp)
- [`packages/bitcoin-knots/src/common/config.cpp`](../../../packages/bitcoin-knots/src/common/config.cpp)
- [`packages/bitcoin-knots/src/httprpc.cpp`](../../../packages/bitcoin-knots/src/httprpc.cpp)
- [`packages/bitcoin-knots/src/rpc/server.cpp`](../../../packages/bitcoin-knots/src/rpc/server.cpp)
- [`packages/bitcoin-knots/contrib/init/org.bitcoin.bitcoind.plist`](../../../packages/bitcoin-knots/contrib/init/org.bitcoin.bitcoind.plist)
- [`packages/bitcoin-knots/contrib/init/bitcoind.service`](../../../packages/bitcoin-knots/contrib/init/bitcoind.service)

## First-party implementation

- [`packages/open-bitcoin-cli/src/operator/migration.rs`](../../../packages/open-bitcoin-cli/src/operator/migration.rs)
- [`packages/open-bitcoin-cli/src/operator/detect.rs`](../../../packages/open-bitcoin-cli/src/operator/detect.rs)
- [`packages/open-bitcoin-cli/src/operator/onboarding.rs`](../../../packages/open-bitcoin-cli/src/operator/onboarding.rs)
- [`packages/open-bitcoin-cli/src/operator/service.rs`](../../../packages/open-bitcoin-cli/src/operator/service.rs)
- [`packages/open-bitcoin-cli/src/operator/wallet.rs`](../../../packages/open-bitcoin-cli/src/operator/wallet.rs)
- [`packages/open-bitcoin-cli/tests/operator_binary.rs`](../../../packages/open-bitcoin-cli/tests/operator_binary.rs)
- [`docs/architecture/cli-command-architecture.md`](../../architecture/cli-command-architecture.md)

## Audit Matrix

| Surface | Baseline expectation | Open Bitcoin current behavior | Evidence | Migration impact | Deviation ids |
| --- | --- | --- | --- | --- | --- |
| CLI and operator routing | Bitcoin or Knots keep `bitcoin-cli`-style transport parsing separate from node-owned operational helpers. | `open-bitcoin-cli` remains the baseline-compatible RPC client path, while `open-bitcoin` owns the operator migration planner as an Open Bitcoin-specific workflow. | `packages/open-bitcoin-cli/src/operator.rs`, `packages/open-bitcoin-cli/src/operator/runtime.rs`, `packages/open-bitcoin-cli/tests/operator_binary.rs` | Operators use a dedicated migration planner instead of expecting `bitcoin-cli` parity for migration. | `mig-dry-run-only-switch-over` |
| RPC and auth evidence | Local RPC uses explicit auth and config or cookie discovery, but migration docs and tools must not leak secrets. | Migration planning reports config and cookie paths as read-only evidence only and never prints cookie contents or raw credentials. | `packages/open-bitcoin-cli/src/operator/detect.rs`, `packages/open-bitcoin-cli/src/operator/migration.rs`, `packages/open-bitcoin-cli/tests/operator_binary.rs` | Source auth material stays operator-visible by path, not by value. | None |
| Config ownership | Baseline-compatible node settings live in `bitcoin.conf`; operator guidance depends on accurate config ownership. | Open Bitcoin keeps Open Bitcoin-only settings in `open-bitcoin.jsonc` and treats source `bitcoin.conf` as read-only migration input. | `docs/architecture/config-precedence.md`, `packages/open-bitcoin-cli/src/operator/onboarding.rs`, `packages/open-bitcoin-cli/src/operator/migration.rs` | The migration plan tells operators to review the source `bitcoin.conf` but place Open Bitcoin-only settings in `open-bitcoin.jsonc`. | `mig-jsonc-open-bitcoin-settings` |
| Datadir layout | Existing datadirs are high-value user data and should not be mutated implicitly. | The planner treats detected source datadirs as read-only evidence and directs the operator toward a separate Open Bitcoin target datadir. | `packages/open-bitcoin-cli/src/operator/detect.rs`, `packages/open-bitcoin-cli/src/operator/migration.rs`, `packages/open-bitcoin-cli/tests/operator_binary.rs` | Cutover remains review-first; no in-place datadir rewrite or source-data mutation exists in Phase 21. | `mig-dry-run-only-switch-over` |
| Service behavior | Existing launchd or systemd definitions influence operator expectations around restart and ownership. | Migration planning inspects detected service definitions and renders them as manual cutover review items. Open Bitcoin does not disable or replace source services automatically in Phase 21. | `packages/open-bitcoin-cli/src/operator/service.rs`, `packages/open-bitcoin-cli/src/operator/migration.rs`, `packages/open-bitcoin-cli/tests/operator_binary.rs` | Operators must validate service ownership and disable source supervisors manually during any future cutover. | `mig-dry-run-only-switch-over` |
| Wallet handling | Existing wallets are high-value data and wallet migration semantics are sensitive. | Phase 20 already added read-only external-wallet inspection; Phase 21 layers migration planning on top and keeps external wallet import, restore, copy, and rewrite out of scope. | `docs/parity/catalog/wallet.md`, `packages/open-bitcoin-cli/src/operator/wallet.rs`, `packages/open-bitcoin-cli/src/operator/migration.rs` | The plan enumerates each detected external wallet candidate and requires verified upstream backups before any later migration work. | `mig-managed-wallet-backup-format` |
| Sync, logs, and runtime expectations | Operators need honest statements about what is or is not preserved during switchover. | Phase 21 keeps migration evidence-scoped. It explains rollback expectations, preserves source paths as review inputs, and avoids claiming full runtime cutover parity for deferred sync or service behavior. | `packages/open-bitcoin-cli/src/operator/migration.rs`, `.planning/ROADMAP.md`, `.planning/PROJECT.md` | Migration output explicitly frames deferred sync and cutover behavior as manual or future work. | `mig-dry-run-only-switch-over` |
| Operator docs | Drop-in claims must be evidence-based rather than implied by implementation drift. | The migration planner, README pointer, architecture page, and parity ledger all describe the same dry-run-first scope with explicit deviations. | `README.md`, `docs/architecture/cli-command-architecture.md`, `docs/parity/index.json`, `.planning/milestones/v1.1-phases/21-drop-in-parity-audit-and-migration/21-VERIFICATION.md` | Operators can audit the current replacement story before treating Open Bitcoin as a safe cutover target. | `mig-jsonc-open-bitcoin-settings`, `mig-dry-run-only-switch-over`, `mig-managed-wallet-backup-format` |

## Intentional Differences

- `mig-jsonc-open-bitcoin-settings`: Open Bitcoin-owned settings stay in
  `open-bitcoin.jsonc` rather than being written into `bitcoin.conf`.
- `mig-dry-run-only-switch-over`: Phase 21 migration is dry-run only. It does
  not disable source services, rewrite source datadirs, or perform automatic
  switch-over.
- `mig-managed-wallet-backup-format`: Open Bitcoin exports managed-wallet
  backups as repo-owned JSON and does not copy or rewrite external
  `wallet.dat` files.

## Known Gaps

- automatic service cutover or daemon handoff
- source-datadir mutation or in-place migration
- external-wallet import, restore, rewrite, or Core-compatible migration
- any full drop-in replacement claim beyond the evidence captured here

## Follow-Up Triggers

Update this entry when:

- a later phase adds migration apply mode or automatic source-data mutation
- source-service cutover becomes automated instead of manual-review-only
- external-wallet import, restore, or rewrite becomes supported
- new intentional migration differences are added to `docs/parity/index.json`
