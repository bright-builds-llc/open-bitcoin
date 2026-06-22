# Operator Runtime Guide

This guide describes the current source-built operator workflow for Open
Bitcoin on macOS and Linux. It is intentionally conservative: the runtime is
source-built, service integration is local-machine only, migration remains
dry-run only, and release readiness stays evidence-based rather than
timing-threshold based. The current v1.8 production claim boundary is
[`docs/parity/production-claim-boundary.md`](../parity/production-claim-boundary.md):
it defines `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`,
and it is not a production full-node readiness claim. Public-network, real
service-manager, multi-day, and release-blocking checks remain opt-in unless
future phases change the contract.
The v1.8 support matrix and issue-evidence checklist live at
[`docs/parity/support-matrix.md`](../parity/support-matrix.md); they classify
the current support terms and show the local evidence expected for issue
reports. The matrix confirms public-network, real service-manager, multi-day, and release-blocking checks remain opt-in unless a later scoped gate changes the contract.
The v1.8 source-built upgrade and rollback policy lives at
[`docs/parity/upgrade-and-rollback-policy.md`](../parity/upgrade-and-rollback-policy.md);
it provides the pre-upgrade checklist, failed-upgrade guidance, rollback
guidance, and no hidden source datadir, wallet, service, or config mutation
boundary.

Use this guide for the practical workflow. Use
[`docs/architecture/config-precedence.md`](../architecture/config-precedence.md),
[`docs/architecture/status-snapshot.md`](../architecture/status-snapshot.md),
and [`docs/parity/`](../parity/) when you need the lower-level contracts and
audit record.

## Install From Source

The current install path is source-built. From the repo root:

```bash
git submodule update --init --recursive
bun --version  # should match .bun-version
bash scripts/install-git-hooks.sh
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
```

Bun is required as the pinned runtime for repo-owned TypeScript automation used
by `bash scripts/verify.sh`. This repository does not have a `package.json`, so
there is no `bun install` step. The hook installer is safe to rerun, and
`bash scripts/verify.sh` will self-heal the local repo hook configuration
outside CI when `core.hooksPath` is missing or wrong.

Before making release or operator claims on a checkout, run the repo-native
verification contract:

```bash
bun run scripts/check-v1.6-release-boundaries.ts
bash scripts/verify.sh
```

That verification path stays offline by default. It runs formatting, linting,
builds, tests, parity-breadcrumb checks, bounded smoke benchmarks, and Bazel
smoke targets without requiring public-network sync.

For local iteration, `bash scripts/verify.sh --fast` keeps the offline Bun
boundary checks, architecture checks, formatting, clippy, and Cargo tests while
skipping benchmark smoke, Bazel smoke, and coverage. For runtime diagnosis of
the full gate, use `bash scripts/verify.sh --profile`; it runs the same strict
contract as `bash scripts/verify.sh` and prints per-step timings. The repo
pre-commit hook intentionally continues to run the strict default verifier.

For release-boundary review, start with the v1.8 production claim boundary
[`docs/parity/production-claim-boundary.md`](../parity/production-claim-boundary.md),
the support matrix
[`docs/parity/support-matrix.md`](../parity/support-matrix.md), the source-built
upgrade and rollback policy
[`docs/parity/upgrade-and-rollback-policy.md`](../parity/upgrade-and-rollback-policy.md),
and
[`docs/parity/release-readiness.md`](../parity/release-readiness.md). The v1.8
boundary defines gate vocabulary and future evidence requirements only.
Historical v1.6 and v1.7 sections preserve source-built, explicit opt-in
full-sync completion and soak evidence; reviewers should inspect validated
active-chain progress, best-known-tip freshness, stay-current state,
restart/resume continuity, no-progress guidance, support evidence, and the UAT
matrices below before accepting any scoped sync evidence claim.

## Binaries

The current source build exposes three relevant binaries:

- `open-bitcoind` for the current local JSON-RPC server runtime
- `open-bitcoin-cli` for the baseline-compatible RPC client path
- `open-bitcoin` for Open Bitcoin-owned operator workflows such as onboarding,
  status, service management, dashboard, migration planning, and managed-wallet
  helpers

`open-bitcoind` now has an explicit mainnet sync activation path with a
daemon-owned bounded sync loop. When enabled, daemon startup opens the selected
durable store, constructs `DurableSyncRuntime`, and runs the explicit opt-in
bounded mainnet sync worker while keeping truthful durable sync state available
to status, dashboard, RPC, and operator CLI control surfaces. This is still an
operator-ready review workflow, not unattended production-node operation and not
a packaged-service guarantee.

You can run them directly from `packages/target/{debug,release}/` after
building or through `cargo run`.

## Datadir And Config Ownership

Open Bitcoin keeps baseline-compatible settings in `bitcoin.conf` and
Open Bitcoin-only settings in `open-bitcoin.jsonc`.

The precedence order is:

`CLI flags > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`

The intended split is:

- `bitcoin.conf`: baseline-compatible node and RPC settings
- `open-bitcoin.jsonc`: onboarding answers, service settings, dashboard options,
  migration metadata, metrics and logging paths, storage settings, and sync
  knobs
- cookie files: RPC auth fallback only

The onboarding and migration flows should not write Open Bitcoin-only keys into
`bitcoin.conf`. See
[`docs/architecture/config-precedence.md`](../architecture/config-precedence.md)
for the stricter contract language.

## Mainnet Sync Activation

Mainnet sync activation is disabled by default. It can be enabled only for the
mainnet chain through Open Bitcoin-owned config or an `open-bitcoind` CLI
override.

JSONC form:

```jsonc
{
  "sync": {
    "network_enabled": true,
    "mode": "mainnet-ibd",
    "manual_peers": ["198.51.100.10:8333"],
    "dns_seeds": ["seed.bitcoin.sipa.be", "dnsseed.bluematt.me"],
    "target_outbound_peers": 2,
    "target_header_height": 144,
    "max_messages_per_peer": 64,
    "max_rounds": 8,
    "max_blocks_in_flight_per_peer": 16,
    "max_blocks_in_flight_total": 64
  }
}
```

Daemon CLI form:

```bash
mkdir -p /tmp/open-bitcoin-mainnet

cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \
  -datadir=/tmp/open-bitcoin-mainnet \
  -openbitcoinsync=mainnet-ibd \
  -server=1
```

Important boundaries:

- `-openbitcoinsync=mainnet-ibd` is an Open Bitcoin-only daemon flag; do not put
  it in `bitcoin.conf`.
- If the JSONC file is not at `<datadir>/open-bitcoin.jsonc`, pass the explicit
  `-openbitcoinconf=/path/to/open-bitcoin.jsonc` flag.
- `sync.manual_peers` configures explicit outbound peers as `host` or
  `host:port`; IPv6 literals should use bracket form such as
  `[2001:db8::7]:8333`.
- `sync.dns_seeds` overrides the default mainnet seed list. Set it to an empty
  array if you want manual peers only for deterministic or controlled testing.
- `sync.target_outbound_peers` caps how many successful outbound peer slots a
  sync round tries to satisfy before moving on. Sync status reports this target
  separately from the observed outbound peer count.
- `sync.target_header_height` optionally stops bounded daemon sync once the
  best validated header height reaches that target. Use it for smoke-sized
  header convergence runs; omit it for normal opt-in bounded mainnet review.
- `sync.max_messages_per_peer`, `sync.max_rounds`,
  `sync.max_blocks_in_flight_per_peer`, and
  `sync.max_blocks_in_flight_total` override the bounded runtime defaults. Each
  value must be greater than zero.
- `sync.network_enabled = true` without `sync.mode = "mainnet-ibd"` is rejected
  so partial config does not accidentally activate public-network behavior.
- Activation is rejected on `-regtest`, `-signet`, or `-testnet`; this Phase 35
  path is only for mainnet IBD bootstrap.
- The daemon now keeps the explicit opt-in bounded mainnet sync worker active
  when mainnet sync is enabled, while the normal local RPC server continues to
  serve operator and wallet requests. This is not unattended production-node
  operation.
- `open-bitcoin status`, `open-bitcoin dashboard`, `open-bitcoin sync status`,
  and RPC `getblockchaininfo` read the same durable sync truth for header
  height, downloaded block height, connected block height, progress signal,
  estimated lag, last successful progress, lifecycle, recovery guidance, and
  last error.
- `open-bitcoin sync pause` and `open-bitcoin sync resume` toggle the durable
  pause flag without requiring operators to inspect or edit internal store
  files directly.
- Use `bun run scripts/run-live-mainnet-smoke.ts --datadir=PATH` for explicit
  live-mainnet review evidence. It is opt-in, writes local reports, and stays
  outside the default `bash scripts/verify.sh` gate.

### Unattended review loop policy

`open-bitcoind` now runs an unattended review loop only after explicit mainnet
sync activation through `sync.network_enabled = true` and
`sync.mode = "mainnet-ibd"` or the daemon-only
`-openbitcoinsync=mainnet-ibd` override. After RPC binds, each daemon wake runs
one bounded `sync_until_idle` cycle, persists durable status, and then waits
before the next cycle.

The loop preserves explicit stop reasons in durable status and structured sync
evidence. Operator-visible reasons include `target_header_reached`,
`no_progress`, `max_rounds_reached`, `operator_paused`,
`shutdown_requested`, storage failure, resource limit, peer failure, and
`retry_backoff` waiting peers. Retry sleeps use at least
`max(sync.retry_backoff_ms, 1000ms)` between cycles, so failing peers, waiting
peers, and repeated no-progress cycles do not hot-loop.

`open-bitcoin sync pause`, `open-bitcoin sync resume`, and clean daemon
shutdown preserve durable state and next-action guidance for later review.
This remains extended operator review readiness, not a production-node,
inbound-serving, relay, production-funds wallet, migration-apply, or packaging
claim.

### Live-smoke block-progress evidence

The live-mainnet smoke report now writes `result.firstBlockProgress` when the
fresh status snapshots observe downloaded or connected block progress. The
object includes `kind: "downloaded" | "connected"`, `height`, `blockHash`,
`observedAtUnixSeconds`, `before`, `after`, `maybePeer`, `maybeSource`, and
`maybeResolvedEndpoint`.

Phase 57 pass evidence requires `kind: "connected"` and a connected block
height increase in the before/after durable status snapshots. downloaded-only evidence
is still useful because it proves the daemon received a best-chain block body,
but it is diagnosed as `awaiting_blocks` until active chainstate advances.
Header-only progress is also retained in `result.firstHeaderProgress` while
remaining a Phase 57 `awaiting_blocks` no-progress result.

Block-specific no-progress causes use these operator actions:

- `awaiting_blocks`: keep the daemon running or retry with peers that can
  deliver and validate block bodies.
- `peer_notfound`: retry with a different peer or more peers when the selected
  peer reports the requested block as unavailable.
- `malformed_block`: inspect peer diagnostics and retry with a different peer;
  malformed block payloads are rejected and uncredited.
- `invalid_block`: inspect validation diagnostics and retry with another peer
  before trusting the block response.
- `duplicate_or_disconnected_block`: review peer outcomes for duplicate,
  disconnected, or non-extending block responses, then retry with peers
  advertising best-chain data. Durable peer reason `disconnected_block` maps to
  this no-credit diagnosis.
- `resource_limit`: raise the configured block in-flight or sync-loop bounds
  for the explicit review run, or reduce competing load.

### Same-datadir restart/resume evidence

Use `--restart-after-progress` when you want the live-smoke runner to prove a
same-datadir restart boundary instead of only proving progress inside one daemon
process:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=/tmp/open-bitcoin-mainnet \
  --manual-peer=HOST:8333 \
  --restart-after-progress \
  --timeout-seconds=180 \
  --poll-seconds=10
```

After the report is written, inspect the same datadir directly through the
repo-local Cargo and Bazel operator commands:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json
```

Pass evidence for the restart run is:

- `result.status` is `passed`
- `result.restartResumeEvidence.restartStatus` is `completed`
- `result.restartResumeEvidence.sameDatadir.requestedPathMatched` is `true`
- `result.restartResumeEvidence.sameDatadir.resolvedPathMatched` is `true`
- `beforeRestart` and `afterRestart` preserve or increase header, downloaded
  block, and connected block heights
- downloaded and connected block hashes remain stable when heights do not move
  after restart
- `duplicateConnectVerdict` is `no_duplicate_connect_observed` or has a clear
  reason for `unavailable`
- `maybePostRestartProgressDelta` may be zero; fresh post-restart progress is
  stronger evidence but is not required when durable same-datadir resume is
  confirmed and the report includes a typed post-restart blocker

Recovery diagnosis categories use the same Phase 61 labels as
`sync.recovery_category`: `clean_shutdown`, `unclean_shutdown`,
`incompatible_schema`, `store_corruption`, `storage_lock_contention`,
`storage_backend_failure`, `resource_exhaustion`, `invalid_peer_data`,
`public_network_unreachable`, and `operator_cancellation`. Storage categories
outrank peer/network guidance so operators repair or preserve the datadir before
retrying network experiments.

### Runtime resource bounds

The sync loop has a bounded public-network resource envelope:

- Header sync keeps at most one `getheaders` request in flight per peer, and
  each decoded `headers` message is capped at the Bitcoin protocol maximum of
  2000 headers.
- Block download is capped by `sync.max_blocks_in_flight_per_peer` per peer and
  `sync.max_blocks_in_flight_total` across the runtime.
- Peer reads are capped by `sync.max_messages_per_peer` per peer per sync
  attempt, and daemon work is capped by `sync.max_rounds` per background wake.
- Durable sync progress, metrics, and status writes happen synchronously through
  the store adapter. There is no unbounded durable-write queue.
- Metrics keep a bounded day-scale window by default: 30 second interval, 2880
  samples per series, and 24 hours maximum age.
- Structured logs keep bounded files by default: daily rotation, 14 files, 14
  days, and 268435456 bytes total.

Active resource bounds are reported through `sync.resource_pressure` using
these `SyncResourcePressure` fields: `blocks_in_flight`,
`max_header_requests_in_flight_per_peer`, `max_headers_per_message`,
`max_blocks_in_flight_per_peer`, `max_blocks_in_flight_total`,
`max_messages_per_peer`, `max_sync_rounds`, `outbound_peers`, and
`target_outbound_peers`. Retry state, peer outcomes, metrics samples,
structured logs, and support evidence remain bounded by config or retention
policies or by compact summaries, and Phase 61 adds no unbounded retained
arrays. There is no retry queue: peer retry state is keyed by resolved endpoint and bounded by candidate peers/outbound target per cycle. The storage-write bound is: durable storage writes are synchronous adapter calls with no queued write backlog.

Inspect the active bounds through the shared status surface:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json
```

### Durable recovery status

Durable sync status separates progress states that can diverge during restart
or recovery:

- `sync_progress.header_height` is the best validated header height.
- `sync_progress.downloaded_block_height` is the highest contiguous best-chain
  block body currently available in the durable store.
- `sync_progress.connected_block_height` is the active chainstate height.
- `sync_progress.validated_active_chain_height` is explicit progress credit for
  blocks that passed consensus validation, connected to active chainstate, and
  were durably persisted. It matches connected chainstate height.
- `sync_progress.maybe_validated_active_chain_hash` is the connected
  active-chain tip hash when available.
- `sync_progress.maybe_validated_active_chain_work` is the connected
  active-chain tip cumulative work as a decimal string when available.
- `sync_progress.block_height` remains a compatibility alias for connected
  height.
- `sync.progress_signal` summarizes the latest useful sync signal:
  `header_progress`, `block_progress`, `waiting_for_peers`, `peer_failures`,
  `awaiting_blocks`, or `steady`.
- `sync.last_successful_progress_unix_seconds` records the most recent accepted
  header or block contribution time when known.
- `sync.lag` is the estimated count-based lag between known validated headers
  and connected chainstate, not a wall-clock ETA.
- `sync.last_error` records the latest durable runtime or peer failure when one
  was observed.
- `sync.recovery_category` reports the stable machine label for the current
  recovery state: `clean_shutdown`, `unclean_shutdown`, `incompatible_schema`,
  `store_corruption`, `storage_lock_contention`, `storage_backend_failure`,
  `resource_exhaustion`, `invalid_peer_data`,
  `public_network_unreachable`, or `operator_cancellation`.
- `sync.recovery_action` reports the highest-priority human next-action text.
  Storage recovery metadata wins over peer guidance because incompatible,
  corrupt, locked, or backend-failed stores must be handled before retrying
  sync.

### Phase 69 tip and stay-current fields

Phase 69 adds typed tip evidence and stay-current state to the same durable sync
status object. These fields explain whether the node is still catching up, is
current at the best-known validated tip, has stale tip evidence, is recovering,
or made no useful progress. They do not replace the Phase 68 progress counters:
operators should still inspect `sync_progress.header_height`,
`sync_progress.downloaded_block_height`, `sync_progress.connected_block_height`,
`sync_progress.validated_active_chain_height`,
`sync_progress.maybe_validated_active_chain_hash`, and
`sync_progress.maybe_validated_active_chain_work` when diagnosing partial
downloads or partial active-chain connection.

- `sync.best_known_tip.source` identifies the evidence source. The current
  runtime projection uses the durable validated header store.
- `sync.best_known_tip.height` is the best-known validated header height.
- `sync.best_known_tip.block_hash` is the best-known validated header hash.
- `sync.best_known_tip.work` is cumulative work for that validated header tip,
  encoded as a decimal string.
- `sync.best_known_tip.block_time_unix_seconds` is the header timestamp for the
  best-known tip.
- `sync.best_known_tip.observed_at_unix_seconds` is the local observation time
  used to classify freshness.
- `sync.best_known_tip.freshness` is `fresh` or `stale` under the configured
  deterministic freshness threshold.
- `sync.best_known_tip.peer_agreement` is a bounded per-peer evidence list that
  classifies peers as agreeing with the best-known tip, behind it, disagreeing
  with it, or providing no usable tip evidence.
- `sync.stay_current` is the shared machine label for the current stay-current
  state.
- `sync.stay_current_next_action` is bounded operator guidance for the
  non-recovery stay-current states.

`sync.stay_current` uses these exact labels:

- `initial_catch_up`: the runtime has useful header or block progress, but
  connected active-chain progress has not yet reached the best-known validated
  tip.
- `current_at_best_known_tip`: the connected active-chain height, hash, and work
  match the fresh best-known validated tip.
- `stale_tip`: best-known tip evidence or peer evidence is older than the
  configured freshness threshold.
- `recovering`: existing recovery context, such as storage or restart recovery,
  owns the operator guidance.
- `no_progress`: the runtime did not observe useful stay-current progress and
  does not have enough fresh current-at-tip evidence.

Metrics and structured logs use the same progress vocabulary. The bounded
metrics history records `header_height`, `downloaded_block_height`,
`connected_block_height`, and the compatibility `sync_height`; structured sync
summary log records include the same heights, progress signal, and last
successful progress timestamp. Treat downloaded-only block bodies as recovery
diagnostics; only validated active-chain fields are progress credit for durable
connection.

### Phase 70 reorg, peer, and no-progress fields

Phase 70 adds bounded branch/reorg evidence and shared no-progress diagnosis to
the durable sync status object. These fields explain whether a better branch is
waiting on block bodies, whether a reorg was durably persisted, which peer
state is blocking useful progress, and which bounded operator action applies.
They do not add public-network checks to default verification, and they do not
claim inbound serving, transaction relay, compact block relay, production-funds
wallet use, migration apply mode, packaging, GUI, hosted dashboard, or broad
production-node readiness.

Branch competition and reorg status use these exact fields:

- `sync.latest_reorg`: bounded evidence for the latest active-chain reorg when
  one has been recorded.
- `sync.latest_reorg.common_ancestor_height`: active-chain height shared by the
  old and replacement branches.
- `sync.latest_reorg.common_ancestor_hash`: block hash at that common ancestor.
- `sync.latest_reorg.disconnected_count`: number of active-chain blocks
  disconnected during the transition.
- `sync.latest_reorg.connected_count`: number of replacement branch blocks
  connected during the transition.
- `sync.latest_reorg.final_active_height`: connected active-chain height after
  the transition.
- `sync.latest_reorg.final_active_hash`: connected active-chain hash after the
  transition.
- `sync.latest_reorg.fully_persisted`: whether the reorg transition and final
  active-chain snapshot were persisted.
- `sync.reconcile_progress`: current branch/reorg reconciliation evidence, such
  as waiting for replacement branch bodies or a persisted reorg.

No-progress status uses `sync.no_progress_diagnosis` and
`sync.no_progress_next_action`. The diagnosis is derived from shared sync
evidence, not renderer-specific strings. Current labels are:

- `current_at_best_known_tip`: connected active-chain evidence matches the fresh
  best-known validated tip.
- `behind_awaiting_headers`: the runtime needs peer header evidence or another
  configured peer.
- `awaiting_block_bodies`: validated headers are ahead of durable block bodies
  or active-chain connection.
- `stale_inflight_cleanup`: in-flight block work is being cleared or reassigned
  after no useful progress.
- `peer_backoff`: a peer is waiting under endpoint-keyed retry backoff.
- `peer_stalled`: a peer stalled without useful progress.
- `peer_failures_exhausted`: the current bounded cycle exhausted peer attempts
  without useful progress.
- `branch_competition_awaiting_bodies`: a better branch is known, but required
  replacement block bodies are still missing.
- `recovering_from_reorg_or_storage`: recovery context owns guidance before a
  new progress claim is made.
- `storage_or_resource_blocked`: storage health or configured resource limits
  must be addressed before retrying sync.

When `sync.no_progress_diagnosis` is unavailable, consumers should render the
unavailable reason instead of inventing a local fallback. When it is available,
`sync.no_progress_next_action` is the bounded human guidance for that diagnosis.

### Phase 72 full-sync evidence and support verdicts

Phase 72 full-sync evidence and support verdicts align `open-bitcoin status`,
`open-bitcoin dashboard`, RPC durable sync status, bounded metrics, structured
logs, live-smoke reports, and support bundles around the same shared sync truth
contract. A support bundle is evidence only when its typed fields prove the
claim being made. Bundle existence, elapsed time, peer reachability, and daemon
startup are not sufficient proof without connected and validated active-chain
progress, best-known tip evidence, stay-current state, blocker diagnosis, reorg
or reconcile evidence, restart/resume checkpoints, or resource-pressure and
recovery evidence.

Support bundle verdict labels are:

- `sync_to_tip_proven`: connected and validated active-chain height, hash, and
  work match the best-known validated tip evidence.
- `stay_current_proven`: the same full-sync evidence is present and
  `stay_current` proves `current_at_best_known_tip`.
- `diagnosed_blocker`: progress is not proven, but the shared status evidence
  contains a typed blocker such as storage/resource pressure, no-progress
  diagnosis, recovery category, or reorg/reconcile blocker. For example,
  `resource_exhaustion` can pair with the next action
  `free storage and retry validation`.
- `inconclusive`: the bundle does not contain enough matching evidence to prove
  sync-to-tip, stay-current behavior, or a diagnosed blocker.

Use the repo-local support bundle command when you need shareable local
evidence for one selected datadir:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

The support output is intentionally compact. It may include
`validated_active_chain_height`, `maybe_validated_active_chain_hash`,
`maybe_validated_active_chain_work`, `best_known_tip`, `stay_current`,
`no_progress_diagnosis`, `no_progress_next_action`, `latest_reorg`,
`reconcile_progress`, `resource_pressure`, `peer_contribution`,
`latest_stop_reason`, and `evidence_verdict`. It must not copy raw daemon logs,
raw peer tables, endpoint tables, credential contents, wallet material, or raw
live-smoke reports into support artifacts.

Phase 72 adds observability and support evidence only. It does not add inbound
serving, address relay, block serving, transaction relay, compact block relay,
production-funds wallet claims, migration apply mode, signed packaging,
Windows service support, GUI, hosted dashboards, or broad production-node
readiness.

<!-- README impact reviewed: README.md already points operator preview readers to this runtime guide and describes explicit live-smoke evidence without making new production claims; packages/README.md is crate inventory only; docs/parity/README.md describes ledger structure rather than operator workflow. No README changes required for Phase 72. -->

### Phase 62 sync truth fields

`open-bitcoin status`, `open-bitcoin dashboard`, `open-bitcoin sync status`,
RPC sync status and warnings, bounded metrics, structured logs, and explicit
opt-in live-smoke snapshots read the same shared status and durable sync truth
projection. A reviewer should treat differences between those surfaces as drift
unless the field is intentionally unavailable and rendered as
`Unavailable: {reason}`.

Operator truth surfaces keep this field order when they show the Phase 62 sync
contract:

1. `lifecycle`
2. `phase`
3. `configured_targets`
4. `attempt_counters`
5. `progress_signal`
6. `last_successful_progress_unix_seconds`
7. `latest_stop_reason`
8. `last_error`
9. `recovery_category`
10. `recovery_action`
11. `resource_pressure`
12. `peer health`
13. `header_height`
14. `downloaded_block_height`
15. `maybe_downloaded_block_hash`
16. `connected_block_height`
17. `maybe_connected_block_hash`
18. `validated_active_chain_height`
19. `maybe_validated_active_chain_hash`
20. `maybe_validated_active_chain_work`
21. `messages_processed`
22. `headers_received`
23. `blocks_received`

Use these repo-local commands to inspect the selected datadir through the
focused sync status surface:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
```

public-network live-smoke evidence remains opt-in UAT. Use
`bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet`
only when you are intentionally collecting live-mainnet review evidence; it is
not part of `bash scripts/verify.sh`.

After a partial download or partial connect, restart the daemon or run a bounded
sync status check against the same datadir:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json
```

Recovery guidance intentionally distinguishes common cases:

- transient network or DNS failures: inspect peer connectivity or peer
  configuration, then retry after backoff;
- invalid data or network-magic mismatches: use a different peer or verify the
  configured Bitcoin network;
- incompatible or corrupt stores: follow the storage recovery action before
  retrying;
- resource exhaustion: raise the configured sync bounds or reduce competing
  load;
- intentional live-smoke cancellation: treat the generated report as cancelled
  evidence and run a new explicit smoke when needed.

## First Run And Onboarding

A common local workflow is:

```bash
mkdir -p /tmp/open-bitcoin-preview
cat > /tmp/open-bitcoin-preview/bitcoin.conf <<'EOF'
regtest=1
rpcconnect=127.0.0.1
rpcport=18443
rpcuser=preview
rpcpassword=preview
EOF

cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \
  -datadir=/tmp/open-bitcoin-preview
```

Then, from another shell:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview status --format human --no-color
```

Daemon-only CLI flags passed to `open-bitcoind` are not automatically
rediscoverable by later operator commands. `status` and `dashboard` need a
normal RPC auth source they can resolve from the selected datadir, such as the
datadir-local `bitcoin.conf` above or a discoverable `.cookie`.

To write the Open Bitcoin-owned JSONC config non-interactively:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview \
  --config=/tmp/open-bitcoin-preview/open-bitcoin.jsonc \
  onboard --non-interactive --approve-write --detect-existing
```

Important onboarding behaviors:

- `--approve-write` is required before onboarding writes files.
- `--detect-existing` asks the onboarding flow to inspect existing Core or Knots
  evidence in supported locations.
- `--force-overwrite` is available when a previously generated
  `open-bitcoin.jsonc` must be replaced deliberately.
- `--disable-metrics` and `--disable-logs` let operators opt out of those local
  runtime surfaces.
- `onboard` writes only `open-bitcoin.jsonc`; it intentionally does not create
  or update `bitcoin.conf`, so live status against a separately started daemon
  still needs baseline-compatible RPC auth outside onboarding.

## Service Lifecycle

Open Bitcoin has repo-owned user service integration for macOS `launchd` and
Linux `systemd`. These workflows are for explicit opt-in extended operator
review of the `open-bitcoind` daemon path. They are not part of
`bash scripts/verify.sh`, and default verification must not start, stop, or
restart a service or call public peers.

Generated launchd plist and systemd unit definitions supervise
`open-bitcoind`, not the `open-bitcoin` operator wrapper. launchd remains
user-level under `~/Library/LaunchAgents/org.open-bitcoin.node.plist`. systemd
remains user-level under
`~/.config/systemd/user/open-bitcoin-node.service`.

`service preview` is always side-effect-free. `service install` and
`service uninstall` are previews unless `--apply` is supplied. The selected
datadir and optional Open Bitcoin JSONC config path are rendered into the
generated service definition so operators can review exactly what would be
supervised before applying any change.

### Service command flow

Use the commands in this order when reviewing a service-managed daemon on a
local machine. Each command is shown with repo-local Cargo and Bazel forms.

1. Preview the generated service definition without side effects:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview
```

Review explicit config path handling during preview:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --config=/tmp/open-bitcoin-mainnet/open-bitcoin.jsonc service preview
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --config=/tmp/open-bitcoin-mainnet/open-bitcoin.jsonc service preview
```

2. Preview install output without writing service files:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install
```

Review explicit config path handling during install preview:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --config=/tmp/open-bitcoin-mainnet/open-bitcoin.jsonc service install
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --config=/tmp/open-bitcoin-mainnet/open-bitcoin.jsonc service install
```

3. Apply the service file install after reviewing the preview:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply
```

4. Start the user service:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start
```

5. Inspect service state:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
```

6. Restart the user service after reviewing current status:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
```

7. Stop the user service for safe shutdown:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop
```

8. Disable automatic user-service activation while keeping the service file:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable
```

9. Preview service file removal without side effects:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall
```

10. Apply service file removal after reviewing the preview:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply
```

### Manager commands and labels

The generated preview and action output surfaces the user-scope manager command
strings it would use, including:

- `systemctl --user start open-bitcoin-node.service`
- `systemctl --user stop open-bitcoin-node.service`
- `systemctl --user restart open-bitcoin-node.service`
- `launchctl bootstrap`
- `launchctl bootout`
- `launchctl kickstart -k`

The dashboard service action keys route through the same service command path:
`t start service`, `o stop service`, and `x restart service`.

Service lifecycle labels are exactly `unmanaged`, `installed-stopped`,
`running`, `failed`, `disabled`, and `unavailable-manager`. The service log path
is `<log_dir>/open-bitcoin.log` when the operator log directory is configured;
otherwise status reports an explicit unavailable reason.

`open-bitcoin service status`, `open-bitcoin status --format human`,
`open-bitcoin status --format json`, and dashboard output should agree on the
service manager, lifecycle label, installed/enabled/running evidence, service
file path, log path, diagnostics, and unavailable reasons. status/dashboard JSON
and human output preserve Phase 62 sync lifecycle, phase, configured targets,
attempt counters, latest stop reason, recovery category/action, resource
pressure, peer health, and downloaded/connected block evidence beside service
state.

### Safe operation notes

- Log inspection: read the service log path reported by `service status`.
  Do not copy RPC cookie contents, `rpcpassword`, or `rpcauth` values into
  support notes.
- Config path review: run `service preview` or `service install` with the
  explicit `--config=/tmp/open-bitcoin-mainnet/open-bitcoin.jsonc` form when the
  JSONC file is not at the datadir default path.
- Safe shutdown: run `service stop`, then run `service status` and
  `sync status --format json` against the same datadir before moving or
  archiving local evidence.
- Restart review: inspect `service status` before and after `service restart`.
  Treat the restart command output as lifecycle evidence, then inspect
  `service.restart_resume` through status JSON for the same selected datadir.
- Recovery next actions: use `sync.recovery_category`,
  `sync.recovery_action`, resource pressure, peer health, and block evidence to
  choose the next bounded retry, storage repair, peer change, or operator stop
  action.

Live service lifecycle checks and public-network mainnet checks are optional
UAT only. Keep them separate from deterministic default verification, and run
them only when intentionally reviewing a local service-managed daemon.

### Service-supervised restart/resume evidence

Phase 64 service restart review uses the same selected Open Bitcoin datadir for
the service definition, the restart action, and the status evidence. The restart
command remains an operator-initiated launchd/systemd action; durable resume
truth comes from `open-bitcoin status --format json` and
`open-bitcoin sync status --format json` after the restart.

Use matching repo-local Cargo and Bazel command forms for the same datadir:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
```

Interpret `service.restart_resume` from fields, not from elapsed time:

- `same_datadir` must be `true` to treat the evidence as same-datadir resume
  review.
- `prior_shutdown` reports `clean` when the prior daemon marked a clean
  shutdown and `unclean` when durable metadata shows interruption or recovery.
- `durable_progress` carries downloaded and connected block heights and hashes
  preserved from the selected datadir.
- `stale_inflight` is `cleared` when no stale block requests are present in the
  durable status evidence; `stale_requests_recorded` means review recovery
  guidance before continuing unattended review.
- `recovery_category` and `next_action` reuse the Phase 61 typed recovery
  vocabulary and storage-first recovery precedence.

### Phase 71 resource bounds and restart/resume proof

Phase 71 keeps bounded long-sync review deterministic by checking peers, in-flight blocks, request queues, retry maps, cache retention, synchronous storage writes, metrics retention, structured log retention, and support evidence compactness through source and hermetic tests. The same-datadir resume matrix: clean shutdown, unclean shutdown, mid-download interruption, mid-connect interruption, stale in-flight cleanup is covered without contacting public peers or restarting a real service manager.

Storage or resource blockers stay explicit operator evidence. `StorageRecoveryAction::FreeDisk` maps low-disk backend failures to `SyncRecoveryCategory::ResourceExhaustion` and the operator action is `Free disk space for the selected datadir, then retry sync.` Open Bitcoin does not automatically repair, prune, move, or mutate the selected datadir for that condition.

The deterministic proof points are
`phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network`
and
`phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight`.
Treat those as local verification evidence for bounded restart/resume behavior,
not as public-mainnet completion evidence.

The optional public-network restart smoke remains separate from default
verification:

```bash
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet --manual-peer=HOST:8333 --restart-after-progress --timeout-seconds=180 --poll-seconds=10
```

Do not add real `systemctl --user restart`, `launchctl kickstart`, or
`--restart-after-progress` public-network commands to `bash scripts/verify.sh`;
default verification stays deterministic and local.

## Status And Dashboard

`open-bitcoin status` is the shared operator summary surface. It can render in
human or JSON form and keeps stopped-node fields visible with explicit
`Unavailable` reasons where live runtime data is missing.

`open-bitcoin sync` is the focused control surface for daemon mainnet sync:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-preview sync status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-preview sync pause
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-preview sync resume
```

For live RPC bootstrap, `status` and `dashboard` reuse the selected datadir,
network, and normal RPC auth sources. A datadir-local `bitcoin.conf` is the
canonical way to make user/password auth rediscoverable for this workflow, and
a discoverable datadir-local `.cookie` works as well. If neither is available,
the command falls back to a stopped snapshot and emits a live-RPC bootstrap
warning.

If live RPC is unavailable, offline `sync status` may still read durable
metadata. Offline `sync pause` and `sync resume` refuse to write when durable
metadata indicates an unclean active, paused, recovering, or failed daemon sync
state; the refusal is an explicit second-writer conflict diagnostic. Use live
RPC or stop `open-bitcoind` cleanly before mutating sync control offline.

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview status --format json
```

`open-bitcoin dashboard` reuses the same shared status snapshot:

- on a TTY, it opens the interactive ratatui dashboard
- on a non-TTY, it falls back to a deterministic text snapshot
- with `--format json`, it emits the shared snapshot as JSON

Example:

```bash
open-bitcoin --network regtest --datadir=/tmp/open-bitcoin-preview dashboard --tick-ms 1000
```

Interpretation guidance:

- `Unavailable` means the collector chose to report absence explicitly instead
  of inventing a default value.
- Sync-focused status now includes lifecycle (`active`, `paused`,
  `recovering`, `failed`, or `stopped`), current phase, progress signal,
  estimated lag, last successful progress timestamp, resource pressure,
  recovery guidance, latest bounded reorg evidence, no-progress diagnosis and
  next action, and the last sync error when durable state is available.
- Recent peer telemetry can show peers as `connected`, `stalled`, `waiting`, or
  `failed`. A `waiting` peer with failure reason `retry_backoff` means the
  runtime is preserving the backoff window and trying other eligible peers when
  they are available.
- Per-peer `headers_received` and `blocks_received` are validation-gated
  contribution counters. `messages_processed` and last-activity timestamps show
  peer activity, but they do not by themselves mean the peer advanced useful
  sync progress. Idle, stalled, waiting, or failed peers with zero contribution
  are still useful diagnosis rows because they preserve state, attempts, last
  activity when available, and failure reason separately from useful progress.
- The `build` section stays compile-time truthful across supported local build
  paths: Cargo builds surface Cargo metadata, while Bazel builds surface the
  workspace version plus Bazel target and compilation-mode identifiers.
- `wallet.freshness` matters as much as `trusted_balance_sats`; a balance alone
  does not imply the wallet view is current.
- `dashboard` and `status` both surface the same node, config, service, sync,
  peer, mempool, wallet, log, metrics, health, and build sections.

For the shared data contract, see
[`docs/architecture/status-snapshot.md`](../architecture/status-snapshot.md).

## Migration Planning

Phase 21 added a read-only migration planner for existing Core or Knots
installations:

```bash
open-bitcoin --network regtest --datadir=/tmp/open-bitcoin-preview migrate plan \
  --source-datadir=/tmp/source/.bitcoin
```

The current migration contract is intentionally limited:

- it is explanation-first and dry-run only
- it detects existing installs, datadirs, configs, services, cookies, and wallet
  candidates
- with `--source-datadir`, it only shows concrete service review paths when a
  detected service definition can be tied to the selected source install;
  otherwise service cutover review stays explicit manual follow-up
- it explains backup requirements, rollback expectations, and intentional
  differences before any later cutover work
- it does not disable source services, mutate source datadirs, or rewrite
  external wallets

Use [`docs/parity/catalog/drop-in-audit-and-migration.md`](../parity/catalog/drop-in-audit-and-migration.md)
for the current audit matrix and explicit non-claims.

## Real-Sync Verification And Benchmarks

Open Bitcoin keeps benchmark evidence as reproducible local reports, not release
timing gates.

The sync runtime has durable peer/sync foundations, TCP transport coverage, and
an opt-in daemon-owned mainnet sync loop. Public-network operation is still an
explicit opt-in review surface, is not part of the default local verification
contract, and is not yet a production-node claim.

Use the repo-owned wrapper:

```bash
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet \
  --timeout-seconds=60 --poll-seconds=5 --manual-peer=HOST[:PORT]
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet \
  --manual-peer=HOST:8333 --restart-after-progress \
  --timeout-seconds=180 --poll-seconds=10
bash scripts/run-benchmarks.sh --smoke
bash scripts/run-benchmarks.sh --full --iterations 5
```

Live smoke behavior:

- `run-live-mainnet-smoke.ts` builds the current daemon and operator binaries,
  starts `open-bitcoind` with explicit `mainnet-ibd` activation, polls
  `open-bitcoin-cli openbitcoinsyncstatus` for fresh daemon sync-control
  status, and writes
  `open-bitcoin-live-mainnet-smoke.json` plus
  `open-bitcoin-live-mainnet-smoke.md` under
  `packages/target/live-mainnet-smoke-reports`.
- `--manual-peer=HOST[:PORT]` may be repeated. When manual peers are supplied
  without `--config`, the runner writes
  `open-bitcoin-live-mainnet-smoke.jsonc` in the selected output directory,
  sets `sync.manual_peers` to those exact values, disables DNS seeds for that
  deterministic run, sets `sync.target_outbound_peers = 1`, and passes the
  generated file to the daemon and final sync-status command. If you pass
  `--config`, put manual peers in that JSONC file instead of also passing
  `--manual-peer`.
- It fails early when the selected datadir does not exist, the optional config
  path is missing, the local clock is obviously wrong, or the selected disk
  path does not meet the configurable free-space floor.
- It records DNS seed and manual-peer endpoint outcomes in JSON and Markdown,
  including whether each preflight/runtime endpoint was resolved, connected,
  handshook, failed, or skipped. Preflight TCP checks are diagnostic and remain
  separate from daemon runtime peer telemetry.
- Its final report also includes runtime peer contribution rows from durable
  peer telemetry so support review can distinguish reachable or active peers
  from peers that actually supplied accepted headers or preserved blocks.
- When fresh daemon snapshots show validated header progress, the report records
  `result.firstHeaderProgress` with the before/after `openbitcoinsyncstatus`
  snapshots, observed timestamp, header delta, and the final peer
  endpoint/source that contributed accepted headers when available.
- It times out cleanly with typed no-progress guidance when outbound DNS or TCP
  access, handshake/capability checks, validation, storage, or runtime progress
  are insufficient.
- Operator cancellation is preserved as `status: cancelled` with
  `maybeNoProgressCause: operator_cancellation`, and the runner still writes
  partial evidence before exiting nonzero.
- With `--restart-after-progress`, the runner intentionally terminates the
  first daemon after observed header, downloaded-block, or connected-block
  progress, starts a second daemon with the same selected datadir, and writes
  compact restart proof under `result.restartResumeEvidence`.
- It terminates its own daemon process after collecting evidence; for longer
  manual review, launch `open-bitcoind` directly and use
  `open-bitcoin sync status|pause|resume`.

### Phase 73 opt-in public-mainnet UAT matrix

This matrix is the authoritative Phase 73 opt-in public-mainnet UAT command
matrix. Live public-mainnet work remains opt-in UAT outside
`bash scripts/verify.sh`, and repo-local Cargo and Bazel command forms are the
expected operator command forms for CLI-backed workflows. Each workflow below
separates evidence from non-proof so UAT artifacts are not misread as broader
release gates. Bundle existence, daemon startup, elapsed time, or peer reachability alone are not sync-to-tip proof.

| Workflow | Copy-paste commands | Evidence proves | Does not prove |
| --- | --- | --- | --- |
| Full-sync activation and review | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd`<br>`bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json` | The selected daemon path can opt into public-mainnet IBD, and durable status can report header, downloaded block, connected block, validated active-chain, best-known-tip, recovery, and resource-pressure evidence for the same datadir. | Daemon startup, peer reachability, or a running process does not prove sync-to-tip, stay-current operation, inbound serving, relay, production-wallet safety, packaging readiness, or broad production-node readiness. |
| Stay-current/status review | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json` | The shared operator status surfaces expose current-at-best-known-tip, stale-tip, recovering, no-progress, progress counters, peer agreement, and next-action evidence. | A single status snapshot, elapsed runtime, or reachable peer does not prove the node stayed current across a review window. |
| Same-datadir restart/resume review | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json` | Restart evidence can show same-datadir matching, preserved or advanced header/downloaded/connected heights, stable hashes when heights do not move, recovery category, and duplicate-connect verdicts. | A restart command completing does not prove durable resume, absence of stale in-flight work, sync-to-tip, or service-manager readiness. |
| Status-surface comparison | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet dashboard --tick-ms 1000`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet dashboard --tick-ms 1000`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status` | CLI status, sync status, dashboard, and service views can be reviewed for agreement on connected progress, best-known-tip freshness, recovery category, peer health, resource pressure, and next action. | Renderer agreement does not prove public-network progress unless validated active-chain fields and reviewed live evidence support the same claim. |
| Live-smoke report collection | `bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet`<br>`bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet --manual-peer=HOST:8333`<br>`bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet --manual-peer=HOST:8333 --restart-after-progress --timeout-seconds=180 --poll-seconds=10`<br>`bash scripts/test-run-live-mainnet-smoke.sh` is deterministic fixture validation, not public-network UAT. | The wrapper can capture opt-in public-network progress, typed no-progress blockers, peer contribution, endpoint outcomes, and optional same-datadir restart evidence in local reports. | A report file, elapsed timeout, preflight success, or endpoint reachability does not prove sync-to-tip unless the report fields show validated active-chain progress to the reviewed target. |
| Support-bundle collection | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support --include-live-smoke-report=packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json` | The bundle can collect redacted status, metrics/log availability, service evidence, support metadata, and summary-only live-smoke evidence for local review. | Bundle existence, copied files, or attached summaries do not prove sync-to-tip, production-node readiness, wallet safety, relay behavior, or safe migration apply behavior. |

### v1.6 release boundary

The v1.6 release boundary is a closeout around the Phase 73 matrix, not a
second authoritative command list. The accepted evidence is field-based:
connected and validated active-chain height/hash/work, best-known-tip
freshness, stay-current or stale/recovering state, same-datadir restart/resume
continuity, no-progress/reorg recovery guidance, resource pressure, peer
contribution, and redacted support evidence.

Use these deterministic checks before interpreting opt-in public-network
evidence:

```bash
bun run scripts/check-v1.6-release-boundaries.ts
bun run scripts/check-phase73-uat-verification.ts
bash scripts/verify.sh
```

The reviewer roots are
[`docs/parity/threat-model-v1.6.md`](../parity/threat-model-v1.6.md),
[`docs/parity/release-readiness.md`](../parity/release-readiness.md),
[`docs/parity/index.json`](../parity/index.json), and
[`docs/parity/checklist.md`](../parity/checklist.md). Generated live-mainnet
reports, support bundles, daemon logs, metrics stores, compatibility reports,
and local datadirs remain local artifacts outside git.

v1.6 does not claim inbound serving, address relay, block serving, transaction
relay, compact block relay, production-funds wallet safety, migration apply
mode, signed packaging, Windows service support, GUI parity, hosted dashboards,
public-network CI, release-blocking live sync, or broad production-node
readiness.

### Phase 75 multi-day soak runner

Phase 75 adds a bounded, explicit opt-in `open-bitcoin soak` workflow for
multi-day full-sync review. It records durable run identity and report state in
the selected Open Bitcoin datadir while keeping public-network and wall-clock
multi-day execution outside `bash scripts/verify.sh`.

Use these repo-local command forms to start a three-day elapsed-time soak:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak start --elapsed-time-seconds 259200 --checkpoint-interval-seconds 300 --target-height <target-height> --peer-policy daemon-configured --disk-budget-bytes 107374182400 --stop-condition elapsed-time
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak start --elapsed-time-seconds 259200 --checkpoint-interval-seconds 300 --target-height <target-height> --peer-policy daemon-configured --disk-budget-bytes 107374182400 --stop-condition elapsed-time
```

Use the same selected datadir and network when resuming, stopping, or
projecting reports for a run:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak resume --run-id <run-id> --checkpoint-interval-seconds 300
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak resume --run-id <run-id> --checkpoint-interval-seconds 300
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak stop --run-id <run-id> --reason operator-stop
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak stop --run-id <run-id> --reason operator-stop
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak report --run-id <run-id>
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak report --run-id <run-id>
```

The durable source of truth is <datadir>/soak/run-index.json plus <datadir>/soak/runs/<run_id>/events.jsonl.
JSON and Markdown reports, operator output, and support-bundle summaries are
projections from those files. Treat moved or stale reports as review artifacts,
not as current durable state.

The soak ledger records `started`, `checkpoint`, `resume`, `stop`, and
`verdict` events. Final outcomes are `clean_completion`,
`diagnosed_blocker`, `operator_stop`, `resource_stop`, `recovery_stop`, and
`unexpected_termination`. These labels belong to the soak evidence layer; they
wrap lower-level sync status, recovery, no-progress, support-verdict, and
process facts without redefining lower-level sync stop or recovery labels.

A soak run can prove bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence; it does not prove inbound serving, relay, production-funds wallet safety, migration apply mode, signed packages, GUI readiness, hosted dashboards, or broad production-node readiness.

### Phase 76 disk and resource-bound enforcement

Phase 76 extends the shared operator status snapshot with top-level
`resource_bounds` evidence. The resource-bound set is explicit: disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle. Each entry is
available with current usage, limit, unit, warning threshold, stop threshold,
state, and next action, or unavailable with a reason. Default warning and stop
thresholds are 80% and 95% of the relevant explicit budget.

Status and dashboard output render the shared `resource_bounds` contract; JSON
consumers should prefer the machine field over human strings:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir /path/to/open-bitcoin --network mainnet status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir /path/to/open-bitcoin --network mainnet status --format json
```

`soak start` performs resource-bound preflight before it writes a run index or
events ledger. Missing datadir evidence, unavailable required measurements, a
zero or invalid disk budget, or stop-required pressure refuses the new run
before ledger mutation. Warning pressure remains runnable and is recorded in
checkpoint evidence. When a running soak observes stop-required resource
pressure, the final outcome is `resource_stop`; the report preserves resource
bound state, next action, and source status evidence.

Support bundles include a compact `resource_bound_evidence` section and
Markdown `## Resource Bound Evidence` projection. The section records labels,
numeric usage, limits, units, next actions, and projected support-bundle
footprint only; it does not copy raw logs, raw stores, complete status payloads,
or unbounded peer tables.

### Phase 77 corruption and lock recovery hardening

Phase 77 adds diagnosis-only `recovery_evidence` to the shared status contract.
JSON consumers should read the top-level
`recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>` field when
diagnosing store locks, stale lock evidence, concurrent datadir use, corruption
markers, schema mismatches, partial writes, unreadable namespaces, backend open
failures, or resource pressure.

The stable recovery action classes are `safe_retry`,
`read_only_inspection`, `backup_then_rebuild`, and `stop_and_escalate`.
The stable causes are `schema_mismatch`, `corruption_marker`, `corrupt_record`,
`partial_write`, `unreadable_namespace`, `backend_open_failure`, `active_lock`,
`stale_lock_evidence`, `concurrent_datadir_use`, and `resource_pressure`.
The compatibility categories are `incompatible_schema`, `store_corruption`,
`storage_lock_contention`, `storage_backend_failure`, and
`resource_exhaustion`.

Use these repo-local commands to inspect one selected datadir without editing
store internals:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir <path> status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir <path> status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir <path> support bundle --output-dir <path>/support --format json
```

Interpret action classes conservatively:

- `safe_retry`: retry only after the named transient or resource condition has
  been addressed.
- `read_only_inspection`: inspect the selected datadir and status evidence
  without deleting lock artifacts or changing stores.
- `backup_then_rebuild`: preserve a backup before any rebuild, restore, or
  replacement workflow outside Phase 77.
- `stop_and_escalate`: stop normal operation and preserve evidence for review
  before retrying.

Phase 77 does not delete lock files, clear recovery markers, repair stores, compact stores, reindex stores, relocate datadirs, mutate source datadirs, scan OS process tables, or upload support bundles automatically.

### Phase 78 progress guarantees and stall diagnosis

Phase 78 adds progress-guarantee evidence to the shared sync status, soak
checkpoint/report, dashboard, support, and live-smoke projections. Operators
should read these machine fields together: `progress_credit`,
`last_useful_work`, `last_peer_contribution`, `expected_progress_window`,
`no_progress_threshold`, and `stall_diagnosis`.

`progress_credit` advances only for `validated_durable_active_chain` evidence
or explicit `current_at_best_known_tip` stay-current evidence. Headers, downloaded block bodies, peer messages, in-flight requests, retries, and report generation are evidence only and do not advance the credited progress watermark.

`last_useful_work` preserves the most recent credited active-chain or at-tip
evidence across later cycles that only wait, retry, or diagnose.
`last_peer_contribution` records the latest bounded peer contribution
separately, so a peer can explain headers, block bodies, messages, or failure
evidence without fabricating progress credit.

`expected_progress_window` and `no_progress_threshold` show the configured
retry/backoff and freshness window used to decide when waiting becomes
diagnosed no-progress evidence. `stall_diagnosis` identifies the current
subsystem and next action through labels such as
`storage_or_resource_pressure`, `at_tip_waiting`, `operator_stop`, and
`local_shutdown`.

Treat missing Phase 78 fields as explicit unavailable evidence. A soak report,
support summary, dashboard row, or live-smoke report is a compact projection of
the same shared status contract; it is not a separate source of progress truth.

### Phase 79 support bundle forensics

Phase 79 support bundles include typed `support_forensics` with `forensic timeline`,
`checkpoint chain`, `failure narrative`, `source evidence`, and `redaction facts`.
The same support-forensics facts are rendered in JSON and Markdown so reviewers
can compare the machine sidecar with `## Forensic Timeline`,
`## Checkpoint Chain`, and `## Failure Narrative` without reading raw logs.

Narrative fields are `verdict`, `likely_cause`, `evidence_basis`,
`next_action`, and `confidence`. Verdict outcomes are exactly `soak_stable`,
`blocker_diagnosed`, `inconclusive`, and `collection_failed`.

Checkpoint-chain evidence is ordering and truncation evidence, not authenticity,
not signing, and not an external trust root. It helps detect missing or reordered
local support-bundle events; it does not prove who produced the bundle or whether
the selected node is trustworthy.

support bundle existence, elapsed time, peer reachability, daemon startup, raw logs, or stale reports do not prove soak stability. Reviewers should quote the
typed `support_forensics` verdict, failure narrative, source evidence, redaction
facts, and checkpoint-chain validation result instead of trusting artifact
presence. Default verification remains public-network-free, service-manager-free,
short-running, and free of large disk allocations.

Use these repo-local support-bundle commands for UAT rather than relying on an
installed alias:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support --format json

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support --format json
```

### Phase 80 v1.7 opt-in soak UAT matrix

This matrix is the focused v1.7 opt-in soak and recovery UAT entrypoint. Keep
these workflows outside `bash scripts/verify.sh`: public-network, real
service-manager, multi-day wall-clock, large-disk, current-tip timing, and
release-blocking live-sync checks remain explicit operator UAT only. Use the
typed evidence named below when reviewing results; artifact presence, daemon
startup, peer reachability, elapsed time, raw logs, copied files, or stale
reports are not enough.

| Workflow | Repo-local Cargo commands | Repo-local Bazel commands | Evidence proves | Does not prove |
| --- | --- | --- | --- | --- |
| Multi-day soak lifecycle | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak start --elapsed-time-seconds 259200 --checkpoint-interval-seconds 300 --target-height <target-height> --peer-policy daemon-configured --disk-budget-bytes 107374182400 --stop-condition elapsed-time`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak resume --run-id <run-id> --checkpoint-interval-seconds 300`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak stop --run-id <run-id> --reason operator-stop`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak report --run-id <run-id>` | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak start --elapsed-time-seconds 259200 --checkpoint-interval-seconds 300 --target-height <target-height> --peer-policy daemon-configured --disk-budget-bytes 107374182400 --stop-condition elapsed-time`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak resume --run-id <run-id> --checkpoint-interval-seconds 300`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak stop --run-id <run-id> --reason operator-stop`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak report --run-id <run-id>` | The selected datadir can preserve durable run identity, start/resume/stop/report state, checkpoint evidence, typed final outcome, resource/recovery/progress evidence, and whether the run proved bounded opt-in soak behavior, durable resume evidence, or diagnosed blocker evidence. | Multi-day wall-clock elapsed time, report generation, peer reachability, daemon startup, or a clean CLI exit does not prove broad production-node readiness, inbound serving, relay, production-funds wallet safety, migration apply safety, signed packaging, GUI readiness, hosted dashboards, public-network CI, or release-blocking live sync. |
| Bounded recovery drill | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-recovery-drill status --format json`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-recovery-drill support bundle --output-dir=/tmp/open-bitcoin-recovery-support --format json` | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-recovery-drill status --format json`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-recovery-drill support bundle --output-dir=/tmp/open-bitcoin-recovery-support --format json` | Typed `recovery_evidence`, recovery action class, cause, next action, and related support evidence when present can show the selected datadir's diagnosis-only recovery state. | The drill does not repair stores, delete locks, prove source-datadir safety, prove process attribution, or authorize mutation. |
| Support-bundle generation | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support --format json` | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support --format json` | Redaction facts, typed `support_forensics`, forensic timeline, checkpoint chain, failure narrative, evidence basis, next action, confidence, and resource/support bounds can show what the local bundle captured and how reviewers should interpret it. | Bundle existence, copied files, raw logs, daemon startup, peer reachability, elapsed time, or stale reports do not prove soak stability. |
| Post-failure diagnosis | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak report --run-id <run-id>`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`<br>`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support --format json` | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet --network mainnet soak report --run-id <run-id>`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json`<br>`bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support --format json` | A typed final outcome, `support_forensics.verdict`, likely cause, evidence basis, next action, checkpoint-chain validation, resource/recovery/stall evidence, and explicit statement that the run proved soak stability, diagnosed a blocker, or stopped inconclusively can support post-failure review. | Post-failure diagnosis does not prove production-node readiness, current-tip timing SLAs, release-blocking live sync, public-network CI, inbound serving, relay, wallet safety, migration apply safety, packaging, GUI, hosted dashboards, automatic upload, or destructive repair. |

## v1.4 operator evidence closeout

Run the deterministic repo checks from the repo root before interpreting any
public-network evidence:

```bash
bun run scripts/check-v1.5-release-boundaries.ts
bash scripts/verify.sh
bash scripts/test-run-live-mainnet-smoke.sh
```

Use the opt-in live smoke commands only for local operator review. The manual
peer command checks fresh public-network progress:

```bash
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet --manual-peer=HOST:8333 --timeout-seconds=180 --poll-seconds=10
```

The same-datadir restart/resume review uses the same local datadir and adds the
restart boundary:

```bash
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet --manual-peer=HOST:8333 --restart-after-progress --timeout-seconds=180 --poll-seconds=10
```

Inspect the selected datadir through both repo-local operator command forms:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
```

Collect redacted local support evidence with either Cargo or Bazel. When a
live-smoke report exists, pass it as a summary-only input:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support --include-live-smoke-report=packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

Interpret v1.4 evidence from fields, not elapsed time or startup behavior:

- `result.status` must be read with `result.progressDetected`; a `passed`
  status is useful only when the relevant progress evidence is present.
- `result.firstHeaderProgress` proves the first validated header-height
  increase observed by the live-smoke runner.
- `result.firstBlockProgress` proves downloaded or connected block progress;
  Phase 57 pass evidence requires connected progress, while downloaded-only
  evidence remains `awaiting_blocks` until chainstate advances.
- `result.restartResumeEvidence` is required for same-datadir restart review.
  Its `result.restartResumeEvidence.recoveryDiagnosis.category` explains
  whether a failed or blocked run points to peer incompatibility, public-network
  reachability, invalid peer data, store corruption, store incompatibility,
  resource exhaustion, or intentional cancellation.
- `result.maybeNoProgressCause` and `result.nextAction` are the operator-facing
  diagnosis and follow-up path for non-passing reports.
- `final_status.headerHeight`, `final_status.downloadedBlockHeight`, and
  `final_status.connectedBlockHeight` are the final durable status counters to
  compare with the before/after evidence.
- `support-evidence.json` and `support-evidence.md` are redacted local support
  artifacts. Their existence confirms collection only; it does not prove sync
  success.

Peer reachability, elapsed time, support-bundle existence, and daemon startup
alone are not success criteria. A review should quote the specific fields above
and preserve the distinction between header progress, downloaded block
progress, connected block progress, restart/resume evidence, diagnosed blockers,
and the next operator action.

Generated live-smoke reports, support bundles, daemon logs, metrics stores, and
local datadirs remain local artifacts and are not checked into git. The docs may
name local paths such as `packages/target/live-mainnet-smoke-reports`,
`/tmp/open-bitcoin-support`, and `/tmp/open-bitcoin-mainnet`, but reviewers
should not commit environment-specific reports, bundles, logs, metrics stores,
or datadir contents. Credential evidence is metadata-only: the selected
credential source and cookie path/presence may be reported, but cookie contents,
`rpcpassword`, and `rpcauth` values are not support evidence.

### Compatibility harness operator wrapper

Use `open-bitcoin compatibility harness` when an operator needs stable local
compatibility evidence without calling Rust harness internals. The command runs
deterministic built-in transcript scenarios through the Phase 54
`open-bitcoin-network::evaluate_transcript` harness and labels the report with
the supplied peer endpoint. The endpoint is report context, not proof that a
public socket was contacted.

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network=mainnet \
  compatibility harness \
  --peer-endpoint=203.0.113.10:8333 \
  --scenario=service-bit-mismatch \
  --output-dir=/tmp/open-bitcoin-compatibility

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --network=mainnet \
  compatibility harness \
  --peer-endpoint=203.0.113.10:8333 \
  --scenario=service-bit-mismatch \
  --output-dir=/tmp/open-bitcoin-compatibility
```

The wrapper writes exactly these local files under the selected output
directory:

- `compatibility-harness-report.json`: machine-readable peer endpoint, network,
  scenario, negotiated capabilities, failing step, diagnosis, transcript
  summary, redaction boundaries, and next action
- `compatibility-harness-report.md`: human-readable review notes for the same
  facts

Supported deterministic scenarios and stable diagnosis values are `compatible`,
`version_rejected`, `network_mismatch`, `service_bit_mismatch`,
`unsupported_message_order`, `timeout`, `peer_disconnect`,
`malformed_payload`, and `local_configuration_failure`. Scenario flags use
kebab-case, for example `--scenario=network-mismatch`; report fields use
snake_case for stable JSON.

Compatibility harness reports omit raw wire payloads, daemon stdout/stderr
tails, RPC credentials, cookie contents, wallet private material, and unbounded
peer logs. They are opt-in local compatibility evidence outside default
verification; `bash scripts/verify.sh` checks the wrapper contract and docs but
does not contact public peers.

### v1.5 operator review

Use this sequence for v1.5 unattended-operation review. Start with deterministic
repo checks, then treat public-network and real service-manager commands as
opt-in UAT outside default verification:

```bash
bash scripts/verify.sh
bash scripts/test-run-live-mainnet-smoke.sh
```

Inspect the selected datadir through both repo-local sync status command forms:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json
```

Inspect the full operator status snapshot through both repo-local command forms:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
```

For service-managed review, inspect and restart only when the operator has
explicitly installed the user service for local review:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart
```

Collect support evidence after deterministic checks and any optional local UAT:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

Interpret the bundle from fields, not from elapsed time or bundle existence:

- `support-evidence.json` and `support-evidence.md` are local redacted evidence.
- `live_smoke.summary.finalStatus` carries compact header, downloaded block,
  connected block, `recoveryCategory`, and `resourcePressure` facts when a
  live-smoke report was attached.
- `restartResumeEvidence` summarizes same-datadir restart/recovery review from
  the attached live-smoke report when available.
- `status.service.restart_resume` carries service-scoped same-datadir,
  prior-shutdown, durable-progress, stale in-flight, recovery category, and
  next-action evidence when durable metadata exists.
- `status.metrics` and `status.logs` report bounded local evidence and explicit
  unavailable reasons; missing local evidence is diagnostic, not a serialization
  failure.
- Public-network long-run review, manual peers, `--restart-after-progress`, and
  real launchd/systemd actions remain opt-in UAT and are not part of
  `bash scripts/verify.sh`.

### Support Evidence Bundles

For a local support handoff, generate a redacted evidence bundle from the
operator CLI. Prefer an explicit output directory so the artifact is easy to
share or delete after review:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support
```

Use the issue-evidence checklist in
[`docs/parity/support-matrix.md`](../parity/support-matrix.md) when preparing
an issue report. Include the smallest useful redacted evidence set for the
selected datadir, or write `Unavailable: <reason>` for evidence that cannot be
provided.

If you already ran the live-mainnet smoke wrapper, attach its JSON report as a
summary-only input:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle \
  --output-dir=/tmp/open-bitcoin-support \
  --include-live-smoke-report=packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json
```

The command writes exactly these local files under the selected output
directory:

- `support-evidence.json`: machine-readable config-path evidence, the shared
  `OpenBitcoinStatusSnapshot`, store-health availability, redaction metadata,
  and an allowlisted live-smoke summary when supplied
- `support-evidence.md`: a compact human-readable index for the same evidence

Redaction boundaries:

- RPC cookie contents, `rpcpassword`, `rpcauth`, wallet private material, raw
  wallet files, and raw unbounded logs are not copied into the bundle.
- Credential evidence is metadata-only. Cookie paths and whether a cookie file
  was present may be reported, but cookie values are not read into the bundle.
- Live-smoke input is not embedded as a raw report. For schema v2 live-smoke
  reports, the support bundle copies only the allowlisted compact summary
  fields from `result` and `final_status`: progress status, no-progress cause,
  next action, first header/block progress, restart/resume evidence, recovery
  diagnosis category, header/downloaded/connected heights, Phase 77 recovery
  evidence (`recoveryEvidence`, `recoveryActionClass`, `recoveryCause`,
  `recoveryNextAction`, and `maybeRecoveryEvidenceUnavailableReason`), and the
  compact active-chain, best-tip, stay-current, no-progress, reorg, reconcile,
  resource-pressure, and peer-contribution final-status evidence. Older or
  hand-authored top-level report fields remain a compatibility fallback.
- Raw live-smoke input, daemon stdout/stderr tails, complete status payloads,
  raw options, and endpoint tables are not embedded in the support bundle.
- The support bundle is local evidence; it is not a production-node claim and
  does not make public-network sync part of `bash scripts/verify.sh`.

Benchmark modes:

- `--smoke` is the bounded local path used by `bash scripts/verify.sh`; it runs
  the benchmark binary in the debug profile and writes reports under
  `packages/target/benchmark-reports`
- `--full` uses a release build for deeper local inspection and trend review
- both modes remain threshold-free; correctness and reviewed evidence matter
  more than elapsed-time pass or fail numbers

The generated reports now record:

- the live-smoke command path, poll interval, timeout, and preflight outcome
- the live-smoke manual peers, generated config path when used, endpoint
  outcome table, typed no-progress cause, and suggested next action
- the final runtime peer contribution table, including peer state, accepted
  header/block counters, last activity, failure reason, and error fields
- the live-smoke status snapshots and daemon stderr/stdout tail for support
  review
- the benchmark mode and iteration count
- the binary profile (`debug` or `release`)
- the measurement focus, fixture type, and durability level for each case
- the relevant Knots benchmark names or source anchors when they exist

## Known Limitations

Open Bitcoin does not currently claim all of the following:

- packaged or signed release installation flows
- Windows service support
- production-node or production-funds readiness for unattended public-mainnet
  operation through `open-bitcoind`
- automatic migration apply, source-service cutover, or source-datadir mutation
- external-wallet import, restore, or rewrite
- public-network sync as part of the default local verification contract
- checked-in live-mainnet report fixtures or timing-threshold release gates
- public-network CI, release-blocking live sync, automatic support-bundle
  upload, destructive repair, or broad production-node readiness
- a hosted public dashboard or GUI parity with the reference Qt app

The parity ledger and deferred-surface record live under
[`docs/parity/`](../parity/). Start with:

- [`docs/parity/production-claim-boundary.md`](../parity/production-claim-boundary.md)
- [`docs/parity/index.json`](../parity/index.json)
- [`docs/parity/checklist.md`](../parity/checklist.md)
- [`docs/parity/deviations-and-unknowns.md`](../parity/deviations-and-unknowns.md)
