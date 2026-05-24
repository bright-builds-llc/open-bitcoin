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

network_fixture="$tmp_dir/network-preflight.json"
cat >"$network_fixture" <<'JSON'
[
  {
    "address": "127.0.0.1:8333",
    "attemptedAtUnixSeconds": 1,
    "host": "127.0.0.1",
    "maybeError": null,
    "maybeFailureCause": null,
    "maybeResolvedEndpoint": "127.0.0.1:8333",
    "port": 8333,
    "source": "manual_peer",
    "stage": "preflight",
    "state": "connected"
  },
  {
    "address": "seed.bitcoin.sipa.be",
    "attemptedAtUnixSeconds": 1,
    "host": "seed.bitcoin.sipa.be",
    "maybeError": "manual peers supplied; generated config disables DNS seeds",
    "maybeFailureCause": null,
    "maybeResolvedEndpoint": null,
    "port": 8333,
    "source": "dns_seed",
    "stage": "preflight",
    "state": "skipped"
  }
]
JSON

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

cat >"$tmp_dir/mock-stalled-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cat <<'JSON'
{
  "headers": 0,
  "blocks": 0,
  "initialblockdownload": true,
  "warnings": ""
}
JSON
EOF
chmod +x "$tmp_dir/mock-stalled-status.sh"

cat >"$tmp_dir/mock-final-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cat <<'JSON'
{
  "maybe_sync_state": {
    "sync": {
      "sync_progress": {
        "state": "available",
        "value": {
          "header_height": 0,
          "block_height": 0,
          "messages_processed": 0
        }
      },
      "lifecycle": {
        "state": "available",
        "value": "active"
      },
      "phase": {
        "state": "available",
        "value": "steady_state"
      },
      "last_error": {
        "state": "unavailable",
        "value": {
          "reason": "no sync error recorded"
        }
      }
    },
    "peers": {
      "peer_counts": {
        "state": "available",
        "value": {
          "outbound": 0
        }
      },
      "recent_peers": {
        "state": "available",
        "value": [
          {
            "peer": "127.0.0.1:8333",
            "source": "manual",
            "state": "failed",
            "network": "mainnet",
            "attempts": 1,
            "maybe_resolved_endpoint": {
              "state": "available",
              "value": "127.0.0.1:8333"
            },
            "capabilities": {
              "state": "unavailable",
              "value": {
                "reason": "peer capabilities unavailable"
              }
            },
            "headers_received": 0,
            "blocks_received": 0,
            "maybe_last_activity_unix_seconds": {
              "state": "unavailable",
              "value": {
                "reason": "peer last activity unavailable"
              }
            },
            "failure_reason": {
              "state": "available",
              "value": "connect"
            },
            "error": {
              "state": "available",
              "value": "connection refused"
            }
          }
        ]
      }
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-final-status.sh"

counter_file="$tmp_dir/status-counter"

OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN="$tmp_dir/mock-daemon.sh" \
OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN="$tmp_dir/mock-status.sh" \
OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE="$network_fixture" \
OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1 \
OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE="$counter_file" \
bun run scripts/run-live-mainnet-smoke.ts \
	--datadir="$existing_datadir" \
	--manual-peer=127.0.0.1:8333 \
	--output-dir="$output_dir" \
	--timeout-seconds=3 \
	--poll-seconds=1 >/dev/null

report_json="$output_dir/open-bitcoin-live-mainnet-smoke.json"
report_markdown="$output_dir/open-bitcoin-live-mainnet-smoke.md"
generated_config="$output_dir/open-bitcoin-live-mainnet-smoke.jsonc"
grep -q '"status": "passed"' "$report_json"
grep -q '"progressDetected": true' "$report_json"
grep -q '"manualPeers": \[' "$report_json"
grep -q '"network_preflight"' "$report_json"
grep -q '"state": "connected"' "$report_json"
grep -q '"dns_seeds": \[\]' "$generated_config"
grep -q "Network Endpoint Outcomes" "$report_markdown"
grep -q "manual_peer" "$report_markdown"
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
grep -q "typed no-progress cause: tcp_connection_failure" "$tmp_dir/no-progress.stderr"

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
	--poll-seconds=1 >/dev/null 2>"$tmp_dir/cancel.stderr" &
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
