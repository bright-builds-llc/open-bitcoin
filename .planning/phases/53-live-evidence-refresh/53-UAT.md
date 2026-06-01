# Phase 53 Live Evidence Refresh UAT

**Date:** 2026-06-01
**Outcome:** `fresh diagnosed blocker evidence`
**Selected closeout mode:** `satisfied-by-fresh-diagnosed-blocker`

## Commands Run

Default live-smoke attempt:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-mainnet-datadir \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase53-default \
  --timeout-seconds=180 \
  --poll-seconds=5 \
  --min-free-gib=1
```

The default run wrote schema v2 JSON and Markdown reports and diagnosed
`handshake_failure` without header or block progress. The post-command wrapper
used `status` as a zsh variable, so only the wrapper exit-code capture failed;
the generated report and stderr were preserved and inspected.

Same-datadir manual-peer retry:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-mainnet-datadir \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase53-manual-peer \
  --timeout-seconds=180 \
  --poll-seconds=5 \
  --min-free-gib=1 \
  --manual-peer=dnsseed.bluematt.me:8333 \
  --manual-peer=seed.bitcoin.jonasschnelli.ch:8333
```

Manual-peer retry exit code: `1`. The nonzero exit is accepted UAT evidence
because the runner wrote a fresh schema v2 diagnosed-blocker report.

Support bundle:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-mainnet-datadir \
  support bundle \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-support \
  --include-live-smoke-report=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json
```

Support bundle exit code: `0`.

Bazel support-bundle equivalent for operators:

```bash
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-mainnet-datadir \
  support bundle \
  --output-dir=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase53-support \
  --include-live-smoke-report=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json
```

## Local Artifacts

Generated live-smoke and support-bundle reports are local review artifacts and
are not checked into git.

| Artifact | Path | Status |
| --- | --- | --- |
| Default live-smoke JSON | `packages/target/live-mainnet-smoke-reports/phase53-default/open-bitcoin-live-mainnet-smoke.json` | Generated, fallback evidence |
| Default live-smoke Markdown | `packages/target/live-mainnet-smoke-reports/phase53-default/open-bitcoin-live-mainnet-smoke.md` | Generated, fallback evidence |
| Selected manual-peer live-smoke JSON | `packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json` | Generated, selected |
| Selected manual-peer live-smoke Markdown | `packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.md` | Generated, selected |
| Support evidence JSON | `packages/target/phase53-support/support-evidence.json` | Generated |
| Support evidence Markdown | `packages/target/phase53-support/support-evidence.md` | Generated |
| Same-datadir store | `packages/target/phase53-mainnet-datadir` | Reused across default and manual-peer attempts |

## Selected Closeout Report

Selected report:
`packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json`

Selected report generated at `2026-06-01T03:46:03Z`.

| Field | Value |
| --- | --- |
| `schema_version` | `2` |
| `result.status` | `no_progress` |
| `result.progressDetected` | `false` |
| `result.headerDelta` | `0` |
| `result.blockDelta` | `0` |
| `result.maybeNoProgressCause` | `handshake_failure` |
| `result.nextAction` | `Inspect daemon stderr and peer endpoint outcomes; retry with a different manual peer if the endpoint accepts TCP but does not complete the Bitcoin handshake.` |
| Manual peers | `dnsseed.bluematt.me:8333`, `seed.bitcoin.jonasschnelli.ch:8333` |
| Snapshot count | `36` |
| Endpoint outcome count | `205` |
| Runtime peer row count | `68` |
| Runtime peer contribution row count | `0` |

## Fresh Status Evidence

The selected report's `commands.status` array contains
`openbitcoinsyncstatus`, so Phase 53 supersedes the historical Phase 50
`getblockchaininfo` snapshot caveat with fresh daemon sync-control snapshots.

First snapshot:

| Field | Value |
| --- | --- |
| `capturedAtUnixSeconds` | `1780285382` |
| `headerHeight` | `0` |
| `blockHeight` | `0` |
| `outboundPeers` | `0` |
| `lifecycle` | `active` |
| `phase` | `steady_state` |
| `maybeLastError` | `null` |

Last snapshot:

| Field | Value |
| --- | --- |
| `capturedAtUnixSeconds` | `1780285557` |
| `headerHeight` | `0` |
| `blockHeight` | `0` |
| `outboundPeers` | `0` |
| `lifecycle` | `active` |
| `phase` | `steady_state` |
| `maybeLastError` | `sync I/O failure: inspect peer connectivity` |

Final durable status:

| Field | Value |
| --- | --- |
| `headerHeight` | `0` |
| `blockHeight` | `0` |
| `messagesProcessed` | `77` |
| `outboundPeers` | `0` |
| `lifecycle` | `active` |
| `phase` | `steady_state` |
| `maybeLastError` | `sync I/O failure: inspect peer connectivity` |

## Endpoint Outcomes

The selected manual-peer report recorded 205 endpoint outcomes:

| State | Count |
| --- | ---: |
| `resolved` | 68 |
| `connected` | 8 |
| `skipped` | 67 |
| `failed` | 62 |

Concrete selected-report endpoint rows:

| Stage | Source | Address | Resolved Endpoint | State | Error |
| --- | --- | --- | --- | --- | --- |
| `preflight` | `manual_peer` | `dnsseed.bluematt.me:8333` | `82.39.40.73:8333` | `resolved` | `Unavailable` |
| `preflight` | `manual_peer` | `dnsseed.bluematt.me:8333` | `82.39.40.73:8333` | `connected` | `Unavailable` |
| `preflight` | `manual_peer` | `dnsseed.bluematt.me:8333` | `117.212.69.134:8333` | `resolved` | `Unavailable` |
| `preflight` | `manual_peer` | `dnsseed.bluematt.me:8333` | `117.212.69.134:8333` | `skipped` | `skipped after 1 TCP attempt(s) for this source` |

## Runtime Peer Contributions

The selected report recorded 68 runtime peer rows: 62 `failed` rows and
6 `stalled` rows. No row recorded accepted header or block contribution, so the
Phase 53 closeout is not progress evidence and does not claim useful peer
contribution success.

Representative stalled peer rows:

| Peer | Endpoint | State | Failure Reason | Headers | Blocks | Capabilities |
| --- | --- | --- | --- | ---: | ---: | --- |
| `seed.bitcoin.jonasschnelli.ch:8333` | `139.162.179.171:8333` | `stalled` | `stall` | 0 | 0 | `services=3077 start_height=0 wtxidrelay=false prefers_headers=false user_agent=/Satoshi:27.0.0/` |
| `seed.bitcoin.jonasschnelli.ch:8333` | `149.106.35.164:8333` | `stalled` | `stall` | 0 | 0 | `services=0 start_height=-1 wtxidrelay=false prefers_headers=false user_agent=` |
| `seed.bitcoin.jonasschnelli.ch:8333` | `34.48.38.29:8333` | `stalled` | `stall` | 0 | 0 | `services=1037 start_height=951931 wtxidrelay=false prefers_headers=false user_agent=/Satoshi:25.1.0/` |

Representative failed peer rows:

| Peer | Endpoint | State | Failure Reason | Headers | Blocks | Error |
| --- | --- | --- | --- | ---: | ---: | --- |
| `dnsseed.bluematt.me:8333` | `79.191.94.121:8333` | `failed` | `network` | 0 | 0 | `sync network failure: inspect peer connectivity` |
| `dnsseed.bluematt.me:8333` | `184.161.137.147:8333` | `failed` | `network` | 0 | 0 | `sync network failure: inspect peer connectivity` |
| `dnsseed.bluematt.me:8333` | `50.47.238.125:8333` | `failed` | `network` | 0 | 0 | `sync network failure: inspect peer connectivity` |

Phase 44 Test 4 is superseded by this fresh-status report because the live
network did not allow contribution observation in this environment, but the
selected report preserves runtime peer rows, stalled/failed states, zero
contribution counters, typed no-progress cause, and next operator action.

## Same-Datadir Evidence

The same datadir, `packages/target/phase53-mainnet-datadir`, was reused for the
default attempt and the manual-peer retry. Restart/resume is
`satisfied-by-fresh-diagnosed-blocker`, not `satisfied-by-progress`: the second
valid invocation produced 36 fresh-status snapshots and coherent durable
metadata, but it did not observe header or block progress.

## Support Bundle Evidence

Support evidence was generated successfully:

| Artifact | Path |
| --- | --- |
| `support-evidence.json` | `packages/target/phase53-support/support-evidence.json` |
| `support-evidence.md` | `packages/target/phase53-support/support-evidence.md` |

The support bundle summarized schema v2 nested `result` fields from the
selected report:

| Field | Value |
| --- | --- |
| `status` | `no_progress` |
| `progressDetected` | `false` |
| `maybeNoProgressCause` | `handshake_failure` |
| `nextAction` | `Inspect daemon stderr and peer endpoint outcomes; retry with a different manual peer if the endpoint accepts TCP but does not complete the Bitcoin handshake.` |
| `headerDelta` | `0` |
| `blockDelta` | `0` |

The support bundle remains local redacted reviewer context. The selected
live-smoke JSON remains the authoritative source for endpoint rows, fresh
status snapshots, and runtime peer rows.

## Requirement Verdicts

| Requirement | Verdict | Evidence |
| --- | --- | --- |
| `PEER-03` | `satisfied-by-fresh-diagnosed-blocker` | The selected report did not observe accepted useful contribution, but it recorded 68 runtime peer rows with failed/stalled states, zero contribution counters, typed cause `handshake_failure`, and next action. |
| `PROOF-03` | `satisfied-by-fresh-diagnosed-blocker` | The selected report did not observe a validated header-height increase, but it recorded fresh `openbitcoinsyncstatus` snapshots, endpoint outcomes, typed cause, and next action. |
| `PROOF-04` | `satisfied-by-fresh-diagnosed-blocker` | The selected report did not observe a validated block connection; it explicitly recorded `result.blockDelta=0`, `result.status=no_progress`, and next action. |
| `PROOF-05` | `satisfied-by-fresh-diagnosed-blocker` | The same datadir was reused for the manual-peer retry, producing coherent fresh-status snapshots and durable metadata without claiming restart/resume progress. |
| `OBS-02` | `satisfied-by-fresh-diagnosed-blocker` | The selected report's per-poll snapshots used `openbitcoinsyncstatus` and showed `lifecycle=active`, `phase=steady_state`, outbound peers, latest error, and progress heights consistently with final status. |
| `SEC-03` | `satisfied-by-fresh-diagnosed-blocker` | UAT records the public-mainnet blocker with typed cause, endpoint outcomes, status snapshots, support evidence, and a concrete next operator action. |

## Debt Closeout

| Debt | Verdict | Evidence |
| --- | --- | --- |
| `D-01` Phase 44 optional public-network UAT skipped | `closed-by-fresh-diagnosed-blocker` | Phase 53 supersedes the old skipped `handshake_failure` run with a new schema v2 report generated after the Phase 51 fresh-status fix. The selected report records runtime peer rows and explains why contribution observation was not possible in this environment. |
| `D-03` Historical Phase 50 selected report caveat | `closed-by-fresh-diagnosed-blocker` | Phase 53 supersedes the old `getblockchaininfo` snapshot caveat with a selected report whose per-poll command is `openbitcoinsyncstatus`. Historical Phase 50 artifact paths remain preserved. |

## Next Operator Action

Retry the live-mainnet smoke with a different reachable manual peer or inspect
daemon stderr and endpoint outcomes for the selected report. The most specific
observed blocker is `handshake_failure`: eight preflight TCP endpoint checks
connected, but runtime peers ended as failed or stalled, accepted zero headers
and blocks, and final durable status reported zero outbound peers plus
`sync I/O failure: inspect peer connectivity`.
