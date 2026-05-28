# Phase 50 Public Mainnet Evidence UAT

**Date:** 2026-05-28  
**Outcome:** `diagnosed blocker evidence`  
**Selected closeout mode:** `satisfied-by-diagnosed-blocker`

## Commands Run

Primary smoke attempt:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=packages/target/phase50-mainnet-datadir \
  --output-dir=packages/target/live-mainnet-smoke-reports/phase50-default \
  --timeout-seconds=120 \
  --poll-seconds=5 \
  --min-free-gib=1
```

Same-datadir manual-peer attempt with relative paths:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=packages/target/phase50-mainnet-datadir \
  --output-dir=packages/target/live-mainnet-smoke-reports/phase50-manual-peer \
  --timeout-seconds=120 \
  --poll-seconds=5 \
  --min-free-gib=1 \
  --manual-peer=seed.bitcoin.sipa.be:8333
```

Same-datadir manual-peer attempt with absolute paths:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase50-mainnet-datadir \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase50-manual-peer-absolute \
  --timeout-seconds=120 \
  --poll-seconds=5 \
  --min-free-gib=1 \
  --manual-peer=seed.bitcoin.sipa.be:8333
```

Support bundle:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase50-mainnet-datadir \
  support bundle \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase50-support \
  --include-live-smoke-report=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase50-manual-peer-absolute/open-bitcoin-live-mainnet-smoke.json
```

Deterministic verification commands for closeout:

```bash
bash scripts/verify.sh
cargo fmt --manifest-path packages/Cargo.toml --all
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support
```

## Local Artifacts

Generated live-smoke and support-bundle reports are local review artifacts and
are not checked into git.

| Artifact | Path | Status |
| --- | --- | --- |
| Default live-smoke JSON | `packages/target/live-mainnet-smoke-reports/phase50-default/open-bitcoin-live-mainnet-smoke.json` | Generated |
| Default live-smoke Markdown | `packages/target/live-mainnet-smoke-reports/phase50-default/open-bitcoin-live-mainnet-smoke.md` | Generated |
| Relative manual-peer live-smoke JSON | `packages/target/live-mainnet-smoke-reports/phase50-manual-peer/open-bitcoin-live-mainnet-smoke.json` | Generated, rejected as selected evidence because the daemon failed before snapshots due to a relative generated-config path |
| Relative manual-peer live-smoke Markdown | `packages/target/live-mainnet-smoke-reports/phase50-manual-peer/open-bitcoin-live-mainnet-smoke.md` | Generated, rejected as selected evidence |
| Selected manual-peer live-smoke JSON | `packages/target/live-mainnet-smoke-reports/phase50-manual-peer-absolute/open-bitcoin-live-mainnet-smoke.json` | Generated, selected |
| Selected manual-peer live-smoke Markdown | `packages/target/live-mainnet-smoke-reports/phase50-manual-peer-absolute/open-bitcoin-live-mainnet-smoke.md` | Generated, selected |
| Support evidence JSON | `packages/target/phase50-support/support-evidence.json` | Generated |
| Support evidence Markdown | `packages/target/phase50-support/support-evidence.md` | Generated |
| Same-datadir store | `packages/target/phase50-mainnet-datadir` | Reused across all attempts |

## Selected Closeout Report

Selected report:
`packages/target/live-mainnet-smoke-reports/phase50-manual-peer-absolute/open-bitcoin-live-mainnet-smoke.json`

Selected report generated at `2026-05-28T03:33:16Z`.

| Field | Value |
| --- | --- |
| `result.status` | `no_progress` |
| `result.progressDetected` | `false` |
| `result.headerDelta` | `0` |
| `result.blockDelta` | `0` |
| `result.maybeNoProgressCause` | `handshake_failure` |
| `result.nextAction` | `Inspect daemon stderr and peer endpoint outcomes; retry with a different manual peer if the endpoint accepts TCP but does not complete the Bitcoin handshake.` |
| Manual peers | `seed.bitcoin.sipa.be:8333` |
| Snapshot count | `24` |
| Endpoint outcome count | `79` |

Daemon stderr ended with:

```text
open-bitcoind mainnet sync preflight enabled: mode=mainnet-ibd, datadir="/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase50-mainnet-datadir", best_header_height=0, best_block_height=0; peer transport and unattended full IBD are not started by this phase.
```

## Endpoint Outcomes

The selected report recorded 79 manual-peer endpoint outcomes:

| State | Count |
| --- | ---: |
| `resolved` | 39 |
| `connected` | 1 |
| `skipped` | 39 |

Concrete selected-report endpoint rows:

| Stage | Source | Address | Resolved Endpoint | State | Error |
| --- | --- | --- | --- | --- | --- |
| `preflight` | `manual_peer` | `seed.bitcoin.sipa.be:8333` | `85.251.67.179:8333` | `resolved` | `Unavailable` |
| `preflight` | `manual_peer` | `seed.bitcoin.sipa.be:8333` | `85.251.67.179:8333` | `connected` | `Unavailable` |
| `preflight` | `manual_peer` | `seed.bitcoin.sipa.be:8333` | `24.48.3.247:8333` | `skipped` | `skipped after 1 TCP attempt(s) for this source` |

The default DNS-seed report recorded 332 endpoint outcomes and the same typed
no-progress cause, `handshake_failure`. The absolute manual-peer report is the
selected closeout artifact because it uses the same datadir, includes manual
peer evidence, avoids the relative generated-config path failure, and includes
status snapshots.

## Durable Status Evidence

Selected report first snapshot:

| Field | Value |
| --- | --- |
| `capturedAtUnixSeconds` | `1779939076` |
| `headerHeight` | `0` |
| `blockHeight` | `0` |
| `outboundPeers` | `0` |
| `lifecycle` | `synced` |
| `phase` | `rpc_getblockchaininfo` |
| `maybeLastError` | `null` |

Selected report last snapshot:

| Field | Value |
| --- | --- |
| `capturedAtUnixSeconds` | `1779939191` |
| `headerHeight` | `0` |
| `blockHeight` | `0` |
| `outboundPeers` | `0` |
| `lifecycle` | `synced` |
| `phase` | `rpc_getblockchaininfo` |
| `maybeLastError` | `null` |

Selected report final durable status:

| Field | Value |
| --- | --- |
| `headerHeight` | `0` |
| `blockHeight` | `0` |
| `messagesProcessed` | `0` |
| `outboundPeers` | `0` |
| `lifecycle` | `active` |
| `phase` | `steady_state` |
| `maybeLastError` | `null` |

## Restart Resume Evidence

The same datadir, `packages/target/phase50-mainnet-datadir`, was reused across
the default smoke attempt, relative manual-peer attempt, and absolute
manual-peer attempt.

Restart/resume is `satisfied-by-diagnosed-blocker`, not
`satisfied-by-progress`. The second valid same-datadir invocation produced 24
snapshots and coherent durable metadata, but it did not observe header or block
progress. The UAT therefore does not claim durable resume success. It records
same-datadir blocker evidence with stable heights at 0, no sync error, zero
outbound runtime peers, and next action to retry with a different manual peer
or inspect daemon/endpoint evidence.

## Support Bundle Evidence

Support evidence was generated successfully:

| Artifact | Path |
| --- | --- |
| `support-evidence.json` | `packages/target/phase50-support/support-evidence.json` |
| `support-evidence.md` | `packages/target/phase50-support/support-evidence.md` |

Support evidence status snapshot recorded:

| Field | Value |
| --- | --- |
| Node state | `stopped` |
| Datadir | `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase50-mainnet-datadir` |
| Sync network | `mainnet` |
| Header height | `0` |
| Downloaded block height | `0` |
| Connected block height | `0` |
| Messages processed | `0` |
| Outbound peers | `0` |
| Target outbound peers | `1` |
| Last successful progress | `unavailable: no successful sync progress recorded in this run` |

The support bundle recorded the selected live-smoke report path. Its
allowlisted live-smoke summary field reported `summary_fields_unavailable`, so
the selected live-smoke JSON remains the authoritative closeout evidence for
`result.status`, deltas, typed cause, endpoint outcomes, snapshots, and next
action.

## Requirement Verdicts

| Requirement | Verdict | Evidence |
| --- | --- | --- |
| `PROOF-03` | `satisfied-by-diagnosed-blocker` | The selected live-smoke report did not observe a validated header-height increase, but it recorded typed no-progress cause `handshake_failure`, endpoint outcomes, 24 before/after durable snapshots, and next action. |
| `PROOF-04` | `satisfied-by-diagnosed-blocker` | The selected report did not observe a validated block connection beyond genesis/checkpoint; it explicitly diagnosed no progress with `result.status=no_progress`, `result.blockDelta=0`, and next action. |
| `PROOF-05` | `satisfied-by-diagnosed-blocker` | The same datadir was reused for a second valid invocation, producing coherent durable metadata and snapshots without progress; no restart/resume success is claimed. |
| `SEC-03` | `satisfied-by-diagnosed-blocker` | UAT records the public-mainnet blocker with typed cause, endpoint outcomes, status snapshots, support evidence, and a concrete next operator action. |

## Next Operator Action

Retry the live-mainnet smoke with a different reachable manual peer or inspect
the daemon stderr and endpoint outcomes for the selected report. The most
specific observed blocker is `handshake_failure`; one TCP preflight endpoint
connected, but the daemon runtime still reported zero outbound peers and no
accepted headers or blocks.
