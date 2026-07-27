#!/usr/bin/env bash
set -euo pipefail

initialize_live_smoke_harness() {
	local tmp_root="$1"
	if [[ "$tmp_root" == "" || "$tmp_root" != /* ]]; then
		echo "live-smoke harness requires an absolute temporary root" >&2
		exit 1
	fi
	mkdir -p "$tmp_root/existing-datadir" "$tmp_root/live-mainnet-smoke-reports"
}

assert_report_redacts_command_credentials() {
	local report_json="$1"
	local report_markdown="$2"
	if rg -n "rpcpassword=|rpcauth=|Authorization|Bearer|Basic|__cookie__" "$report_json" "$report_markdown" >/dev/null; then
		echo "live-smoke reports must redact command credentials" >&2
		exit 1
	fi
}
