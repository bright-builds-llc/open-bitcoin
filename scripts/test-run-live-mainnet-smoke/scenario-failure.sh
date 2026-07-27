#!/usr/bin/env bash
set -euo pipefail

run_recovery_diagnosis_scenarios() {
	local tmp_dir="$1"
	local existing_datadir="$tmp_dir/existing-datadir"
	local missing_datadir="$tmp_dir/missing-datadir"
	local output_dir="$tmp_dir/live-mainnet-smoke-reports"
	local network_fixture="$tmp_dir/network-preflight.json"
	local counter_file="$tmp_dir/status-counter"
	local report_json="$output_dir/open-bitcoin-live-mainnet-smoke.json"
	local report_markdown="$output_dir/open-bitcoin-live-mainnet-smoke.md"
	local generated_config="$output_dir/open-bitcoin-live-mainnet-smoke.jsonc"
recovery_cases=(
	"incompatible_schema|connect|storage schema mismatch during sync"
	"store_corruption|connect|storage corruption in headers during sync"
	"storage_lock_contention|connect|storage lock contention during sync"
	"storage_backend_failure|connect|storage backend unavailable during sync"
	"resource_exhaustion|resource_limit|"
	"invalid_peer_data|invalid_block|"
	"invalid_peer_data|invalid_magic|"
	"public_network_unreachable|connect|"
)

for recovery_case in "${recovery_cases[@]}"; do
	IFS='|' read -r expected_category failure_reason last_error <<<"$recovery_case"
	rm -f "$counter_file"
	OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
	OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-status.sh" \
	OPEN_BITCOIN_LIVE_SMOKE_FINAL_STATUS_BIN="$tmp_dir/mock-peer-failure-final-status.sh" \
	OPEN_BITCOIN_LIVE_SMOKE_FAILURE_REASON="$failure_reason" \
	OPEN_BITCOIN_LIVE_SMOKE_LAST_ERROR="$last_error" \
	OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
	OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
	OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
	bun run scripts/run-live-mainnet-smoke.ts \
		--datadir="$existing_datadir" \
		--manual-peer=127.0.0.1:8333 \
		--output-dir="$output_dir" \
		--timeout-seconds=4 \
		--poll-seconds=1 \
		--restart-after-progress >/dev/null

	grep -q "\"category\": \"$expected_category\"" "$report_json"
	grep -q "\"maybePeerFailureReason\": \"$failure_reason\"" "$report_json"
	if [[ "$expected_category" == "invalid_peer_data" ]]; then
		grep -q '"recoveryCategory": "invalid_peer_data"' "$report_json"
		grep -q '"resourcePressure": {' "$report_json"
		grep -q '"maxBlocksInFlightTotal": 64' "$report_json"
	fi
	if [[ "$expected_category" == "incompatible_schema" || "$expected_category" == "store_corruption" || "$expected_category" == "storage_lock_contention" || "$expected_category" == "storage_backend_failure" ]]; then
		grep -q "\"category\": \"$expected_category\"" "$report_json"
		grep -q "Inspect the datadir storage error" "$report_json"
	fi
done

}

run_failure_scenarios() {
	local tmp_dir="$1"
	local existing_datadir="$tmp_dir/existing-datadir"
	local missing_datadir="$tmp_dir/missing-datadir"
	local output_dir="$tmp_dir/live-mainnet-smoke-reports"
	local network_fixture="$tmp_dir/network-preflight.json"
	local counter_file="$tmp_dir/status-counter"
	local report_json="$output_dir/open-bitcoin-live-mainnet-smoke.json"
	local report_markdown="$output_dir/open-bitcoin-live-mainnet-smoke.md"
	local generated_config="$output_dir/open-bitcoin-live-mainnet-smoke.jsonc"
set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-stalled-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_FINAL_STATUS_BIN="$tmp_dir/mock-final-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=2 \
	--poll-seconds=1 >/dev/null 2>"$tmp_dir/no-progress.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected no-progress smoke run to fail" >&2
	exit 1
fi

grep -q '"status": "no_progress"' "$report_json"
grep -q '"maybeNoProgressCause": "tcp_connection_failure"' "$report_json"
grep -q '"phase": "steady_state"' "$report_json"
grep -q '"outboundPeers": 0' "$report_json"
grep -q '"headersReceived": 2' "$report_json"
grep -q '"blocksReceived": 1' "$report_json"
grep -q "Runtime Peer Contributions" "$report_markdown"
grep -q "typed no-progress cause: tcp_connection_failure" "$tmp_dir/no-progress.stderr"

set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-unavailable-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_FINAL_STATUS_BIN="$tmp_dir/mock-unavailable-final-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=2 \
	--poll-seconds=1 >/dev/null 2>"$tmp_dir/unavailable-status.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected unavailable-status smoke run to report no_progress" >&2
	exit 1
fi

grep -q '"status": "no_progress"' "$report_json"
grep -q '"maybeSyncProgressUnavailableReason": "fixture sync progress unavailable"' "$report_json"
grep -q '"maybePeerCountsUnavailableReason": "fixture peer counts unavailable"' "$report_json"
grep -q '"headerHeight": null' "$report_json"
grep -q '"blockHeight": null' "$report_json"
grep -q '"downloadedBlockHeight": null' "$report_json"
grep -q '"connectedBlockHeight": null' "$report_json"
grep -q '"headersReceived": null' "$report_json"
grep -q '"blocksReceived": null' "$report_json"
grep -q '"messagesProcessed": null' "$report_json"
grep -q '"outboundPeers": null' "$report_json"
grep -q '"maybeRecoveryEvidenceUnavailableReason": "recovery evidence unavailable"' "$report_json"
if rg -n '"(headerHeight|blockHeight|downloadedBlockHeight|connectedBlockHeight|headersReceived|blocksReceived|messagesProcessed|outboundPeers)": 0' "$report_json" >/dev/null; then
	echo "unavailable sync progress or peer fields must remain null instead of zero" >&2
	exit 1
fi
grep -q "Header height: Unavailable: fixture sync progress unavailable" "$report_markdown"
grep -q "Peer health: outbound_peers=Unavailable: fixture peer counts unavailable" "$report_markdown"
grep -q "Bounded counters: messages_processed=Unavailable: fixture sync progress unavailable" "$report_markdown"
grep -q "Recovery action class: Unavailable: recovery evidence unavailable" "$report_markdown"

peer_failure_cases=(
	"block_notfound peer_notfound"
	"malformed_block malformed_block"
	"invalid_block invalid_block"
	"duplicate_block duplicate_or_disconnected_block"
	"disconnected_block duplicate_or_disconnected_block"
	"non_extending_block duplicate_or_disconnected_block"
	"resource_limit resource_limit"
)

for peer_failure_case in "${peer_failure_cases[@]}"; do
	read -r peer_failure_reason expected_cause <<<"$peer_failure_case"
	set +e
	OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
	OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-stalled-status.sh" \
	OPEN_BITCOIN_LIVE_SMOKE_FINAL_STATUS_BIN="$tmp_dir/mock-peer-failure-final-status.sh" \
	OPEN_BITCOIN_LIVE_SMOKE_FAILURE_REASON="$peer_failure_reason" \
	OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
	OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
	bun run scripts/run-live-mainnet-smoke.ts \
		--datadir="$existing_datadir" \
		--manual-peer=127.0.0.1:8333 \
		--output-dir="$output_dir" \
		--timeout-seconds=2 \
		--poll-seconds=1 >/dev/null 2>"$tmp_dir/peer-$peer_failure_reason.stderr"
	status=$?
	set -e

	if [[ "$status" -eq 0 ]]; then
		echo "expected peer failure smoke run for $peer_failure_reason to fail" >&2
		exit 1
	fi

	grep -q '"status": "no_progress"' "$report_json"
	grep -q "\"maybeNoProgressCause\": \"$expected_cause\"" "$report_json"
	grep -q "\"maybeFailureReason\": \"$peer_failure_reason\"" "$report_json"
	grep -q "typed no-progress cause: $expected_cause" "$tmp_dir/peer-$peer_failure_reason.stderr"
	if [[ "$peer_failure_reason" == "disconnected_block" ]]; then
		grep -q '"maybeFailureReason": "disconnected_block"' "$report_json"
		grep -q '"maybeNoProgressCause": "duplicate_or_disconnected_block"' "$report_json"
	fi
done

set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-stalled-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_FINAL_STATUS_BIN="$tmp_dir/mock-final-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=30 \
	--poll-seconds=1 \
	--restart-after-progress >/dev/null 2>"$tmp_dir/cancel.stderr" &
cancel_pid=$!
sleep 3
kill -TERM "$cancel_pid"
wait "$cancel_pid"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected cancelled smoke run to fail" >&2
	exit 1
fi

grep -q '"status": "cancelled"' "$report_json"
grep -q '"maybeNoProgressCause": "operator_cancellation"' "$report_json"
grep -q '"category": "operator_cancellation"' "$report_json"
}
