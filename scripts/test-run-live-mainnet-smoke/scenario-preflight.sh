#!/usr/bin/env bash
set -euo pipefail

run_preflight_scenarios() {
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
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--output-dir="$output_dir" \
	--timeout-seconds=2junk >/dev/null 2>"$tmp_dir/invalid-timeout.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected invalid timeout smoke run to fail" >&2
	exit 1
fi

grep -q -- "--timeout-seconds must be a positive integer" "$tmp_dir/invalid-timeout.stderr"

set +e
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333junk \
	--output-dir="$output_dir" >/dev/null 2>"$tmp_dir/invalid-peer.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected invalid peer port smoke run to fail" >&2
	exit 1
fi

grep -q "invalid peer port" "$tmp_dir/invalid-peer.stderr"

injection_marker="$tmp_dir/command-injection-marker"
malicious_daemon="missing-daemon\"; touch \"$injection_marker\"; #"
set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$malicious_daemon" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--output-dir="$output_dir" >/dev/null 2>"$tmp_dir/command-injection.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected injected command smoke run to fail" >&2
	exit 1
fi
if [[ -e "$injection_marker" ]]; then
	echo "command existence preflight executed shell metacharacters" >&2
	exit 1
fi

set +e
OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$missing_datadir" \
	--output-dir="$output_dir" >/dev/null 2>"$tmp_dir/preflight.stderr"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
	echo "expected missing datadir smoke run to fail" >&2
	exit 1
fi

grep -q "requires an existing datadir" "$tmp_dir/preflight.stderr"
grep -q '"status": "preflight_failed"' "$report_json"
grep -q "Unavailable: no sync status snapshots captured" "$report_markdown"
assert_report_redacts_command_credentials "$report_json" "$report_markdown"

}
