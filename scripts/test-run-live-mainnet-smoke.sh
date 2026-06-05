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

bun run scripts/run-live-mainnet-smoke.ts --help | grep -q "Usage:"

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
daemon_counter_file="${OPEN_BITCOIN_LIVE_SMOKE_DAEMON_COUNTER_FILE:-}"
if [[ "$daemon_counter_file" != "" ]]; then
	count=0
	if [[ -f "$daemon_counter_file" ]]; then
		count="$(cat "$daemon_counter_file")"
	fi
	echo $((count + 1)) >"$daemon_counter_file"
fi
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
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": 0,
            "block_height": 0,
            "downloaded_block_height": 0,
            "connected_block_height": 0,
            "maybe_downloaded_block_hash": null,
            "maybe_connected_block_hash": null,
            "messages_processed": 1
          }
        },
        "lifecycle": {
          "state": "available",
          "value": "active"
        },
        "phase": {
          "state": "available",
          "value": "waiting_for_headers"
        },
        "last_successful_progress_unix_seconds": {
          "state": "unavailable",
          "value": {
            "reason": "no successful progress recorded"
          }
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
      "updated_at_unix_seconds": 1777225000
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
	echo 1 >"$counter_file"
	exit 0
fi

cat <<'JSON'
{
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": 1,
            "block_height": 1,
            "downloaded_block_height": 1,
            "connected_block_height": 1,
            "maybe_downloaded_block_hash": "1111111111111111111111111111111111111111111111111111111111111111",
            "maybe_connected_block_hash": "1111111111111111111111111111111111111111111111111111111111111111",
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
          "value": 1777225005
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
      "updated_at_unix_seconds": 1777225005
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-status.sh"

cat >"$tmp_dir/mock-downloaded-only-status.sh" <<'EOF'
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
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": 0,
            "block_height": 0,
            "downloaded_block_height": 0,
            "connected_block_height": 0,
            "maybe_downloaded_block_hash": null,
            "maybe_connected_block_hash": null,
            "messages_processed": 1
          }
        },
        "lifecycle": {
          "state": "available",
          "value": "active"
        },
        "phase": {
          "state": "available",
          "value": "waiting_for_blocks"
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
      "updated_at_unix_seconds": 1777225200
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
	echo 1 >"$counter_file"
	exit 0
fi

cat <<'JSON'
{
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": 1,
            "block_height": 0,
            "downloaded_block_height": 1,
            "connected_block_height": 0,
            "maybe_downloaded_block_hash": "2222222222222222222222222222222222222222222222222222222222222222",
            "maybe_connected_block_hash": null,
            "messages_processed": 4
          }
        },
        "lifecycle": {
          "state": "available",
          "value": "active"
        },
        "phase": {
          "state": "available",
          "value": "awaiting_blocks"
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
      "updated_at_unix_seconds": 1777225205
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-downloaded-only-status.sh"

cat >"$tmp_dir/mock-header-only-status.sh" <<'EOF'
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
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": 0,
            "block_height": 0,
            "downloaded_block_height": 0,
            "connected_block_height": 0,
            "maybe_downloaded_block_hash": null,
            "maybe_connected_block_hash": null,
            "messages_processed": 1
          }
        },
        "lifecycle": {
          "state": "available",
          "value": "active"
        },
        "phase": {
          "state": "available",
          "value": "waiting_for_headers"
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
      "updated_at_unix_seconds": 1777225300
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
	echo 1 >"$counter_file"
	exit 0
fi

cat <<'JSON'
{
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": 1,
            "block_height": 0,
            "downloaded_block_height": 0,
            "connected_block_height": 0,
            "maybe_downloaded_block_hash": null,
            "maybe_connected_block_hash": null,
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
      "updated_at_unix_seconds": 1777225305
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-header-only-status.sh"

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

cat >"$tmp_dir/mock-stalled-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cat <<'JSON'
{
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "available",
          "value": {
            "header_height": 0,
            "block_height": 0,
            "downloaded_block_height": 0,
            "connected_block_height": 0,
            "maybe_downloaded_block_hash": null,
            "maybe_connected_block_hash": null,
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
          "value": []
        }
      },
      "updated_at_unix_seconds": 1777225100
    },
    "sync_control": {
      "paused": false
    }
  }
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
          "downloaded_block_height": 0,
          "connected_block_height": 0,
          "maybe_downloaded_block_hash": null,
          "maybe_connected_block_hash": null,
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
            "peer": "198.51.100.10:8333",
            "source": "manual",
            "state": "connected",
            "network": "mainnet",
            "attempts": 1,
            "maybe_resolved_endpoint": {
              "state": "available",
              "value": "198.51.100.10:8333"
            },
            "capabilities": {
              "state": "available",
              "value": "services=9 start_height=1 wtxidrelay=true prefers_headers=true user_agent=/open-bitcoin-test/"
            },
            "headers_received": 2,
            "blocks_received": 1,
            "maybe_last_activity_unix_seconds": {
              "state": "available",
              "value": 1777225100
            },
            "failure_reason": {
              "state": "unavailable",
              "value": {
                "reason": "peer healthy"
              }
            },
            "error": {
              "state": "unavailable",
              "value": {
                "reason": "peer healthy"
              }
            }
          },
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

cat >"$tmp_dir/mock-peer-failure-final-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

failure_reason="${OPEN_BITCOIN_LIVE_SMOKE_FAILURE_REASON:?}"
last_error="${OPEN_BITCOIN_LIVE_SMOKE_LAST_ERROR:-}"
outbound_peers="${OPEN_BITCOIN_LIVE_SMOKE_OUTBOUND_PEERS:-0}"
if [[ "$last_error" == "" ]]; then
	last_error_json='"last_error": {
        "state": "unavailable",
        "value": {
          "reason": "no sync error recorded"
        }
      }'
else
	last_error_json='"last_error": {
        "state": "available",
        "value": "'"$last_error"'"
      }'
fi
cat <<JSON
{
  "maybe_sync_state": {
    "sync": {
      "sync_progress": {
        "state": "available",
        "value": {
          "header_height": 0,
          "block_height": 0,
          "downloaded_block_height": 0,
          "connected_block_height": 0,
          "maybe_downloaded_block_hash": null,
          "maybe_connected_block_hash": null,
          "messages_processed": 1
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
      $last_error_json
    },
    "peers": {
      "peer_counts": {
        "state": "available",
        "value": {
          "outbound": $outbound_peers
        }
      },
      "recent_peers": {
        "state": "available",
        "value": [
          {
            "peer": "198.51.100.20:8333",
            "source": "manual",
            "state": "failed",
            "network": "mainnet",
            "attempts": 1,
            "maybe_resolved_endpoint": {
              "state": "available",
              "value": "198.51.100.20:8333"
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
              "state": "available",
              "value": 1777225400
            },
            "failure_reason": {
              "state": "available",
              "value": "$failure_reason"
            },
            "error": {
              "state": "available",
              "value": "$failure_reason fixture"
            }
          }
        ]
      }
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-peer-failure-final-status.sh"

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
restart_evidence_json="$(bun --eval 'const report = await Bun.file(process.argv[1]).json(); console.log(JSON.stringify(report.result.restartResumeEvidence));' "$report_json")"
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

recovery_cases=(
	"store_incompatibility|connect|storage schema mismatch during sync"
	"store_corruption|connect|storage corruption in headers during sync"
	"resource_exhaustion|resource_limit|"
	"invalid_peer_data|invalid_block|"
	"peer_incompatibility|invalid_magic|"
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
	if [[ "$expected_category" == "store_incompatibility" || "$expected_category" == "store_corruption" ]]; then
		grep -q "\"category\": \"$expected_category\"" "$report_json"
		grep -q "Inspect the datadir storage error" "$report_json"
	fi
done

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
grep -q '"category": "intentional_cancellation"' "$report_json"
