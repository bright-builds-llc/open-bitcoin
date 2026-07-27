#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
helper_dir="$script_dir/test-run-live-mainnet-smoke"
cd "$repo_root"

source "$helper_dir/common.sh"
source "$helper_dir/fixtures-daemon.sh"
source "$helper_dir/fixtures-progress.sh"
source "$helper_dir/fixtures-restart.sh"
source "$helper_dir/fixtures-failure.sh"
source "$helper_dir/scenario-progress.sh"
source "$helper_dir/scenario-restart.sh"
source "$helper_dir/scenario-preflight.sh"
source "$helper_dir/scenario-failure.sh"

tmp_root="$(mktemp -d)"
cleanup() {
	if [[ "$tmp_root" == "" || "$tmp_root" == "/" ]]; then
		echo "refusing unsafe live-smoke harness cleanup target" >&2
		return 1
	fi
	rm -rf -- "$tmp_root"
}
trap cleanup EXIT

initialize_live_smoke_harness "$tmp_root"
install_daemon_fixtures "$tmp_root"
install_progress_fixtures "$tmp_root"
install_restart_fixtures "$tmp_root"
install_failure_fixtures "$tmp_root"

bun run scripts/run-live-mainnet-smoke.ts --help | grep -q "Usage:"
if rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh >/dev/null; then
	echo "scripts/verify.sh must not invoke opt-in public-network live-smoke commands" >&2
	exit 1
fi

run_happy_progress_scenarios "$tmp_root"
run_restart_scenarios "$tmp_root"
run_recovery_diagnosis_scenarios "$tmp_root"
run_downloaded_and_header_progress_scenarios "$tmp_root"
run_preflight_scenarios "$tmp_root"
run_failure_scenarios "$tmp_root"
