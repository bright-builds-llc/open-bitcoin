#!/usr/bin/env bash
set -euo pipefail

install_restart_fixtures() {
	local tmp_dir="$1"
cat >"$tmp_dir/mock-restart-hash-mismatch-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

counter_file="${OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE:?}"
count=0
if [[ -f "$counter_file" ]]; then
	count="$(cat "$counter_file")"
fi

if [[ "$count" -eq 0 ]]; then
	echo 1 >"$counter_file"
	header_height=0
	block_height=0
	downloaded_block_height=0
	connected_block_height=0
	hash="null"
elif [[ "$count" -eq 1 ]]; then
	echo 2 >"$counter_file"
	header_height=1
	block_height=1
	downloaded_block_height=1
	connected_block_height=1
	hash="4444444444444444444444444444444444444444444444444444444444444444"
else
	header_height=1
	block_height=1
	downloaded_block_height=1
	connected_block_height=1
	hash="5555555555555555555555555555555555555555555555555555555555555555"
fi

cat <<JSON
{
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": $header_height,
            "block_height": $block_height,
            "downloaded_block_height": $downloaded_block_height,
            "connected_block_height": $connected_block_height,
            "maybe_downloaded_block_hash": $([[ "$hash" == "null" ]] && echo null || echo "\"$hash\""),
            "maybe_connected_block_hash": $([[ "$hash" == "null" ]] && echo null || echo "\"$hash\""),
            "messages_processed": 4
          }
        },
        "lifecycle": {
          "state": "available",
          "value": "active"
        },
        "phase": {
          "state": "available",
          "value": "header_sync"
        },
        "last_successful_progress_unix_seconds": {
          "state": "available",
          "value": 1777225600
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
            "outbound": 1
          }
        },
        "recent_peers": {
          "state": "available",
          "value": []
        }
      },
      "updated_at_unix_seconds": 1777225600
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-restart-hash-mismatch-status.sh"

cat >"$tmp_dir/mock-restart-second-status-fails.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

counter_file="${OPEN_BITCOIN_LIVE_SMOKE_COUNTER_FILE:?}"
count=0
if [[ -f "$counter_file" ]]; then
	count="$(cat "$counter_file")"
fi

if [[ "$count" -eq 0 ]]; then
	echo 1 >"$counter_file"
	header_height=0
	block_height=0
	downloaded_block_height=0
	connected_block_height=0
	hash=null
elif [[ "$count" -eq 1 ]]; then
	echo 2 >"$counter_file"
	header_height=1
	block_height=1
	downloaded_block_height=1
	connected_block_height=1
	hash="6666666666666666666666666666666666666666666666666666666666666666"
else
	echo "post-restart status unavailable" >&2
	exit 1
fi

cat <<JSON
{
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": $header_height,
            "block_height": $block_height,
            "downloaded_block_height": $downloaded_block_height,
            "connected_block_height": $connected_block_height,
            "maybe_downloaded_block_hash": $([[ "$hash" == "null" ]] && echo null || echo "\"$hash\""),
            "maybe_connected_block_hash": $([[ "$hash" == "null" ]] && echo null || echo "\"$hash\""),
            "messages_processed": 4
          }
        },
        "lifecycle": {
          "state": "available",
          "value": "active"
        },
        "phase": {
          "state": "available",
          "value": "header_sync"
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
            "outbound": 1
          }
        },
        "recent_peers": {
          "state": "available",
          "value": []
        }
      },
      "updated_at_unix_seconds": 1777225700
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-restart-second-status-fails.sh"

}
