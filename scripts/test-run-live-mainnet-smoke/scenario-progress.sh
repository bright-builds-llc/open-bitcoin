#!/usr/bin/env bash
set -euo pipefail

run_happy_progress_scenarios() {
	local tmp_dir="$1"
	local existing_datadir="$tmp_dir/existing-datadir"
	local missing_datadir="$tmp_dir/missing-datadir"
	local output_dir="$tmp_dir/live-mainnet-smoke-reports"
	local network_fixture="$tmp_dir/network-preflight.json"
	local counter_file="$tmp_dir/status-counter"
	local report_json="$output_dir/open-bitcoin-live-mainnet-smoke.json"
	local report_markdown="$output_dir/open-bitcoin-live-mainnet-smoke.md"
	local generated_config="$output_dir/open-bitcoin-live-mainnet-smoke.jsonc"
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_FINAL_STATUS_BIN="$tmp_dir/mock-final-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=3 \
	--poll-seconds=1 >/dev/null

assert_report_redacts_command_credentials "$report_json" "$report_markdown"
grep -q '"status": "passed"' "$report_json"
grep -q '"progressDetected": true' "$report_json"
grep -q '"restartResumeEvidence": null' "$report_json"
grep -q '"firstBlockProgress": {' "$report_json"
grep -q '"kind": "connected"' "$report_json"
grep -q '"height": 1' "$report_json"
grep -q '"blockHash": "1111111111111111111111111111111111111111111111111111111111111111"' "$report_json"
grep -q '"downloadedBlockHeight": 1' "$report_json"
grep -q '"connectedBlockHeight": 1' "$report_json"
grep -q '"maybeLastSuccessfulProgressUnixSeconds": 1777225005' "$report_json"
grep -q '"openbitcoinsyncstatus"' "$report_json"
grep -q '"lifecycle": "active"' "$report_json"
grep -q '"phase": "header_sync"' "$report_json"
grep -q '"outboundPeers": 1' "$report_json"
grep -q '"configuredTargets": {' "$report_json"
grep -q '"targetOutboundPeers": 4' "$report_json"
grep -q '"maybeTargetHeaderHeight": 840200' "$report_json"
grep -q '"attemptCounters": {' "$report_json"
grep -q '"attemptedPeers": 3' "$report_json"
grep -q '"connectedPeers": 2' "$report_json"
grep -q '"failedPeers": 1' "$report_json"
grep -q '"maxSyncRounds": 8' "$report_json"
grep -q '"progressSignal": "header_progress"' "$report_json"
grep -q '"latestStopReason": {' "$report_json"
grep -q '"label": "target_header_reached"' "$report_json"
grep -q '"maybeLatestStopReasonUnavailableReason": "no stop reason recorded"' "$report_json"
grep -q '"recoveryCategory": "invalid_peer_data"' "$report_json"
grep -q '"recoveryAction": "Retry sync after peer backoff."' "$report_json"
grep -q '"recoveryEvidence": {' "$report_json"
grep -q '"recoveryActionClass": "read_only_inspection"' "$report_json"
grep -q '"recoveryCause": "stale_lock_evidence"' "$report_json"
grep -q '"recoveryNextAction": "Inspect the datadir read-only and avoid deleting lock artifacts automatically."' "$report_json"
grep -q '"maybeRecoveryEvidenceUnavailableReason": null' "$report_json"
grep -q '"resourcePressure": {' "$report_json"
grep -q '"targetOutboundPeers": 4' "$report_json"
grep -q '"paused": false' "$report_json"
grep -q '"updatedAtUnixSeconds": 1777225005' "$report_json"
grep -q '"manualPeers": \[' "$report_json"
grep -q '"network_preflight"' "$report_json"
grep -q '"state": "connected"' "$report_json"
grep -q '"dns_seeds": \[\]' "$generated_config"
grep -q "Network Endpoint Outcomes" "$report_markdown"
grep -q "manual_peer" "$report_markdown"
grep -q "Header delta: 1" "$report_markdown"
grep -q "First block progress" "$report_markdown"
grep -q "Signal | Configured Targets | Attempts" "$report_markdown"
grep -q "Latest Stop Reason" "$report_markdown"
grep -q "Validated active-chain height:" "$report_markdown"
grep -q "Validated active-chain hash:" "$report_markdown"
grep -q "Validated active-chain work:" "$report_markdown"
grep -q "Recovery action class: read_only_inspection" "$report_markdown"
grep -q "Recovery cause: stale_lock_evidence" "$report_markdown"
grep -q "Recovery next action: Inspect the datadir read-only and avoid deleting lock artifacts automatically." "$report_markdown"
grep -q "Best-known tip:" "$report_markdown"
grep -q "Stay-current:" "$report_markdown"
grep -q "No-progress diagnosis:" "$report_markdown"
grep -q "Progress credit:" "$report_markdown"
grep -q "No-progress threshold:" "$report_markdown"
grep -q "Last useful work:" "$report_markdown"
grep -q "Last peer contribution:" "$report_markdown"
grep -q "Stalled subsystem:" "$report_markdown"
grep -q "Latest reorg:" "$report_markdown"
grep -q "Reconcile progress:" "$report_markdown"
grep -q "Daemon Output Summary" "$report_markdown"
bun run scripts/test-run-live-mainnet-smoke/assert-report.ts progress "$report_json"

rm -f "$counter_file"
missing_validated_height_output_dir="$tmp_dir/missing-validated-height-output"
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_FINAL_STATUS_BIN="$tmp_dir/mock-final-status-missing-validated-height.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$missing_validated_height_output_dir" \
	--timeout-seconds=3 \
	--poll-seconds=1 >/dev/null

missing_validated_height_json="$missing_validated_height_output_dir/open-bitcoin-live-mainnet-smoke.json"
missing_validated_height_markdown="$missing_validated_height_output_dir/open-bitcoin-live-mainnet-smoke.md"
bun run scripts/test-run-live-mainnet-smoke/assert-report.ts missing-validated-height "$missing_validated_height_json"
grep -q "Validated active-chain height: Unavailable: validated active-chain height unavailable" "$missing_validated_height_markdown"

}

run_downloaded_and_header_progress_scenarios() {
	local tmp_dir="$1"
	local existing_datadir="$tmp_dir/existing-datadir"
	local missing_datadir="$tmp_dir/missing-datadir"
	local output_dir="$tmp_dir/live-mainnet-smoke-reports"
	local network_fixture="$tmp_dir/network-preflight.json"
	local counter_file="$tmp_dir/status-counter"
	local report_json="$output_dir/open-bitcoin-live-mainnet-smoke.json"
	local report_markdown="$output_dir/open-bitcoin-live-mainnet-smoke.md"
	local generated_config="$output_dir/open-bitcoin-live-mainnet-smoke.jsonc"
rm -f "$counter_file"
set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-downloaded-only-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=2 \
	--poll-seconds=1 >/dev/null 2>"$tmp_dir/downloaded-only.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected downloaded-only smoke run to report no_progress" >&2
	exit 1
fi

grep -q '"status": "no_progress"' "$report_json"
grep -q '"firstBlockProgress": {' "$report_json"
grep -q '"kind": "downloaded"' "$report_json"
grep -q '"height": 1' "$report_json"
grep -q '"blockHash": "2222222222222222222222222222222222222222222222222222222222222222"' "$report_json"
grep -q '"maybeNoProgressCause": "awaiting_blocks"' "$report_json"
grep -q "Downloaded block progress was observed" "$tmp_dir/downloaded-only.stderr"

rm -f "$counter_file"
set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-header-only-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=2 \
	--poll-seconds=1 >/dev/null 2>"$tmp_dir/header-only.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected header-only smoke run to report no_progress" >&2
	exit 1
fi

grep -q '"status": "no_progress"' "$report_json"
grep -q '"firstHeaderProgress": {' "$report_json"
grep -q '"firstBlockProgress": null' "$report_json"
grep -q '"maybeNoProgressCause": "awaiting_blocks"' "$report_json"
grep -q "Header progress was observed" "$tmp_dir/header-only.stderr"

}
