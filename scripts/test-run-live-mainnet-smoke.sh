#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
cd "$repo_root"

tmp_dir="$(mktemp -d)"
cleanup() {
	rm -rf "$tmp_dir"
}
trap cleanup EXIT

existing_datadir="$tmp_dir/existing-datadir"
missing_datadir="$tmp_dir/missing-datadir"
output_dir="$tmp_dir/output"
mkdir -p "$existing_datadir" "$output_dir"

cat >"$tmp_dir/mock-daemon.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
trap 'exit 0' TERM INT
while true; do
	sleep 1
done
EOF
chmod +x "$tmp_dir/mock-daemon.sh"

cat >"$tmp_dir/mock-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

counter_file="${OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE:?}"
count=0
if [[ -f "$counter_file" ]]; then
	count="$(cat "$counter_file")"
fi

if [[ "$count" -eq 0 ]]; then
	cat <<'JSON'
{
  "headers": 0,
  "blocks": 0,
  "initialblockdownload": true,
  "warnings": ""
}
JSON
	echo 1 >"$counter_file"
	exit 0
fi

cat <<'JSON'
{
  "headers": 1,
  "blocks": 0,
  "initialblockdownload": true,
  "warnings": ""
}
JSON
EOF
chmod +x "$tmp_dir/mock-status.sh"

counter_file="$tmp_dir/status-counter"

OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--output-dir="$output_dir" \
	--timeout-seconds=3 \
	--poll-seconds=1 >/dev/null

report_json="$output_dir/open-bitcoin-live-mainnet-smoke.json"
report_markdown="$output_dir/open-bitcoin-live-mainnet-smoke.md"
grep -q '"status": "passed"' "$report_json"
grep -q '"progressDetected": true' "$report_json"
grep -q "Header delta: 1" "$report_markdown"

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
