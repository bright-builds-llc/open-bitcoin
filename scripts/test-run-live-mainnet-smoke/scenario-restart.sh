#!/usr/bin/env bash
set -euo pipefail

run_restart_scenarios() {
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
daemon_counter_file="$tmp_dir/daemon-counter"
rm -f "$daemon_counter_file"
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_COUNTER_FILE="$daemon_counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=4 \
	--poll-seconds=1 \
	--restart-after-progress >/dev/null

if [[ "$(cat "$daemon_counter_file")" -ne 2 ]]; then
	echo "expected restart smoke run to start the mock daemon twice" >&2
	exit 1
fi

grep -q '"status": "passed"' "$report_json"
grep -q '"restartResumeEvidence": {' "$report_json"
grep -q '"restartStatus": "completed"' "$report_json"
grep -q '"requestedPathMatched": true' "$report_json"
grep -q '"resolvedPathMatched": true' "$report_json"
grep -q '"duplicateConnectVerdict": "no_duplicate_connect_observed"' "$report_json"
grep -q '"beforeRestart": {' "$report_json"
grep -q '"afterRestart": {' "$report_json"
grep -q '"maybePostRestartProgressDelta": {' "$report_json"
grep -q '"firstBlockProgress": {' "$report_json"
grep -q '"maybeLastSuccessfulProgressUnixSeconds": 1777225005' "$report_json"
grep -q '"daemon_sessions": \[' "$report_json"
grep -q "Restart/resume evidence" "$report_markdown"
grep -q "Daemon Sessions" "$report_markdown"
bun run scripts/test-run-live-mainnet-smoke/assert-report.ts restart "$report_json"
restart_evidence_json="$(bun run scripts/test-run-live-mainnet-smoke/assert-report.ts extract-restart-evidence "$report_json")"
for forbidden_restart_field in stdoutTail stderrTail endpoint_outcomes snapshots manualPeers; do
	if [[ "$restart_evidence_json" == *"$forbidden_restart_field"* ]]; then
		echo "restart evidence leaked forbidden field $forbidden_restart_field" >&2
		exit 1
	fi
done

rm -f "$counter_file"
set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-restart-hash-mismatch-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=4 \
	--poll-seconds=1 \
	--restart-after-progress >/dev/null 2>"$tmp_dir/restart-hash-mismatch.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected restart hash mismatch smoke run to fail" >&2
	exit 1
fi
grep -q '"restartStatus": "blocked_before_restart"' "$report_json"
grep -q '"duplicateConnectVerdict": "duplicate_connect_suspected"' "$report_json"
grep -q "Post-restart durable resume evidence did not preserve" "$tmp_dir/restart-hash-mismatch.stderr"

rm -f "$counter_file"
set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-restart-second-status-fails.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=4 \
	--poll-seconds=1 \
	--restart-after-progress >/dev/null 2>"$tmp_dir/restart-second-status-fails.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected second-session status failure smoke run to fail" >&2
	exit 1
fi
grep -q '"status": "runtime_failed"' "$report_json"
grep -q '"restartStatus": "blocked_before_restart"' "$report_json"
grep -q "Post-restart daemon session did not produce" "$tmp_dir/restart-second-status-fails.stderr"

}
