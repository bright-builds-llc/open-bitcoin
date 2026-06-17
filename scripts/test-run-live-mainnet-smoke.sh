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
output_dir="packages/target/live-mainnet-smoke-reports"
mkdir -p "$existing_datadir" "$output_dir"
rm -f "$output_dir/open-bitcoin-live-mainnet-smoke.json" \
	"$output_dir/open-bitcoin-live-mainnet-smoke.md" \
	"$output_dir/open-bitcoin-live-mainnet-smoke.jsonc"

bun run scripts/run-live-mainnet-smoke.ts --help | grep -q "Usage:"
if rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh >/dev/null; then
	echo "scripts/verify.sh must not invoke opt-in public-network live-smoke commands" >&2
	exit 1
fi

assert_report_redacts_command_credentials() {
	local report_json="$1"
	local report_markdown="$2"
	if rg -n "rpcpassword=|rpcauth=|Authorization|Bearer|Basic|__cookie__" "$report_json" "$report_markdown" >/dev/null; then
		echo "live-smoke reports must redact command credentials" >&2
		exit 1
	fi
}

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
        "configured_targets": {
          "state": "available",
          "value": {
            "target_outbound_peers": 4,
            "maybe_target_header_height": 840200
          }
        },
        "attempt_counters": {
          "state": "available",
          "value": {
            "attempted_peers": 1,
            "connected_peers": 1,
            "failed_peers": 0,
            "max_sync_rounds": 8
          }
        },
        "progress_signal": {
          "state": "available",
          "value": "waiting_for_peers"
        },
        "latest_stop_reason": {
          "state": "unavailable",
          "value": {
            "reason": "no stop reason recorded"
          }
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
        },
        "recovery_category": {
          "state": "unavailable",
          "value": {
            "reason": "no recovery category recorded"
          }
        },
        "recovery_action": {
          "state": "unavailable",
          "value": {
            "reason": "no recovery action required"
          }
        },
        "resource_pressure": {
          "state": "available",
          "value": {
            "blocks_in_flight": 0,
            "max_header_requests_in_flight_per_peer": 1,
            "max_headers_per_message": 2000,
            "max_blocks_in_flight_per_peer": 16,
            "max_blocks_in_flight_total": 64,
            "max_messages_per_peer": 64,
            "max_sync_rounds": 8,
            "outbound_peers": 1,
            "target_outbound_peers": 4
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
        "configured_targets": {
          "state": "available",
          "value": {
            "target_outbound_peers": 4,
            "maybe_target_header_height": 840200
          }
        },
        "attempt_counters": {
          "state": "available",
          "value": {
            "attempted_peers": 3,
            "connected_peers": 2,
            "failed_peers": 1,
            "max_sync_rounds": 8
          }
        },
        "progress_signal": {
          "state": "available",
          "value": "header_progress"
        },
        "latest_stop_reason": {
          "state": "available",
          "value": {
            "label": "target_header_reached",
            "message": "sync header target reached"
          }
        },
        "last_successful_progress_unix_seconds": {
          "state": "available",
          "value": 1777225005
        },
        "progress_credit": {
          "state": "available",
          "value": {
            "kind": "validated_durable_active_chain",
            "credited_validated_active_chain_height": 1,
            "credited_validated_active_chain_hash": "1111111111111111111111111111111111111111111111111111111111111111",
            "credited_validated_active_chain_work": "1",
            "source_unix_seconds": 1777225005,
            "rejected_activity": [
              {
                "kind": "header_download",
                "observed_count": 2,
                "reason": "headers alone do not prove durable progress"
              }
            ]
          }
        },
        "expected_progress_window": {
          "state": "available",
          "value": {
            "retry_backoff_seconds": 30,
            "max_sync_rounds": 8,
            "expected_progress_window_seconds": 300,
            "tip_freshness_threshold_seconds": 600
          }
        },
        "no_progress_threshold": {
          "state": "available",
          "value": {
            "threshold_seconds": 300,
            "elapsed_since_last_useful_work_seconds": 5,
            "state": "within_window",
            "evaluated_at_unix_seconds": 1777225010
          }
        },
        "last_useful_work": {
          "state": "available",
          "value": {
            "kind": "validated_durable_active_chain",
            "credited_validated_active_chain_height": 1,
            "credited_validated_active_chain_hash": "1111111111111111111111111111111111111111111111111111111111111111",
            "credited_validated_active_chain_work": "1",
            "source_unix_seconds": 1777225005,
            "rejected_activity": []
          }
        },
        "last_peer_contribution": {
          "state": "available",
          "value": {
            "peer": "127.0.0.1:8333",
            "maybe_resolved_endpoint": "127.0.0.1:8333",
            "kind": "headers_and_blocks",
            "messages_processed": 4,
            "headers_received": 2,
            "blocks_received": 1,
            "maybe_last_activity_unix_seconds": 1777225005,
            "maybe_failure_reason_label": null
          }
        },
        "stall_diagnosis": {
          "state": "available",
          "value": {
            "stalled_subsystem": "at_tip_waiting",
            "confidence": "high",
            "evidence_basis": ["validated_active_chain", "fresh_tip"],
            "next_action": "No operator action required.",
            "maybe_no_progress_diagnosis": "current_at_best_known_tip",
            "maybe_recovery_category": null,
            "maybe_latest_stop_reason_label": "target_header_reached",
            "source_unix_seconds": 1777225010
          }
        },
        "last_error": {
          "state": "unavailable",
          "value": {
            "reason": "no sync error recorded"
          }
        },
        "recovery_category": {
          "state": "available",
          "value": "invalid_peer_data"
        },
        "recovery_action": {
          "state": "available",
          "value": "Retry sync after peer backoff."
        },
        "resource_pressure": {
          "state": "available",
          "value": {
            "blocks_in_flight": 2,
            "max_header_requests_in_flight_per_peer": 1,
            "max_headers_per_message": 2000,
            "max_blocks_in_flight_per_peer": 16,
            "max_blocks_in_flight_total": 64,
            "max_messages_per_peer": 64,
            "max_sync_rounds": 8,
            "outbound_peers": 1,
            "target_outbound_peers": 4
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
        "configured_targets": {
          "state": "available",
          "value": {
            "target_outbound_peers": 4,
            "maybe_target_header_height": 840200
          }
        },
        "attempt_counters": {
          "state": "available",
          "value": {
            "attempted_peers": 3,
            "connected_peers": 1,
            "failed_peers": 2,
            "max_sync_rounds": 8
          }
        },
        "progress_signal": {
          "state": "available",
          "value": "waiting_for_peers"
        },
        "latest_stop_reason": {
          "state": "unavailable",
          "value": {
            "reason": "no stop reason recorded"
          }
        },
        "last_error": {
          "state": "unavailable",
          "value": {
            "reason": "no sync error recorded"
          }
        },
        "recovery_category": {
          "state": "available",
          "value": "public_network_unreachable"
        },
        "recovery_action": {
          "state": "available",
          "value": "Retry with a reachable manual peer."
        },
        "resource_pressure": {
          "state": "available",
          "value": {
            "blocks_in_flight": 0,
            "max_header_requests_in_flight_per_peer": 1,
            "max_headers_per_message": 2000,
            "max_blocks_in_flight_per_peer": 16,
            "max_blocks_in_flight_total": 64,
            "max_messages_per_peer": 64,
            "max_sync_rounds": 8,
            "outbound_peers": 0,
            "target_outbound_peers": 4
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

cat >"$tmp_dir/mock-unavailable-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cat <<'JSON'
{
  "metadata": {
    "maybe_sync_state": {
      "sync": {
        "sync_progress": {
          "state": "unavailable",
          "value": {
            "reason": "fixture sync progress unavailable"
          }
        },
        "lifecycle": {
          "state": "available",
          "value": "active"
        },
        "phase": {
          "state": "available",
          "value": "status_unavailable_fixture"
        },
        "configured_targets": {
          "state": "available",
          "value": {
            "target_outbound_peers": 4,
            "maybe_target_header_height": 840200
          }
        },
        "attempt_counters": {
          "state": "available",
          "value": {
            "attempted_peers": 1,
            "connected_peers": 0,
            "failed_peers": 0,
            "max_sync_rounds": 8
          }
        },
        "progress_signal": {
          "state": "unavailable",
          "value": {
            "reason": "fixture progress signal unavailable"
          }
        },
        "latest_stop_reason": {
          "state": "unavailable",
          "value": {
            "reason": "fixture stop reason unavailable"
          }
        },
        "last_error": {
          "state": "unavailable",
          "value": {
            "reason": "fixture sync error unavailable"
          }
        },
        "recovery_category": {
          "state": "unavailable",
          "value": {
            "reason": "fixture recovery category unavailable"
          }
        },
        "recovery_action": {
          "state": "unavailable",
          "value": {
            "reason": "fixture recovery action unavailable"
          }
        },
        "resource_pressure": {
          "state": "unavailable",
          "value": {
            "reason": "fixture resource pressure unavailable"
          }
        }
      },
      "peers": {
        "peer_counts": {
          "state": "unavailable",
          "value": {
            "reason": "fixture peer counts unavailable"
          }
        },
        "recent_peers": {
          "state": "available",
          "value": []
        }
      },
      "updated_at_unix_seconds": 1777225150
    },
    "sync_control": {
      "paused": false
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-unavailable-status.sh"

cat >"$tmp_dir/mock-unavailable-final-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cat <<'JSON'
{
  "maybe_sync_state": {
    "sync": {
      "sync_progress": {
        "state": "unavailable",
        "value": {
          "reason": "fixture sync progress unavailable"
        }
      },
      "lifecycle": {
        "state": "available",
        "value": "active"
      },
      "phase": {
        "state": "available",
        "value": "status_unavailable_fixture"
      },
      "configured_targets": {
        "state": "available",
        "value": {
          "target_outbound_peers": 4,
          "maybe_target_header_height": 840200
        }
      },
      "attempt_counters": {
        "state": "available",
        "value": {
          "attempted_peers": 1,
          "connected_peers": 0,
          "failed_peers": 0,
          "max_sync_rounds": 8
        }
      },
      "progress_signal": {
        "state": "unavailable",
        "value": {
          "reason": "fixture progress signal unavailable"
        }
      },
      "latest_stop_reason": {
        "state": "unavailable",
        "value": {
          "reason": "fixture stop reason unavailable"
        }
      },
      "last_error": {
        "state": "unavailable",
        "value": {
          "reason": "fixture sync error unavailable"
        }
      },
      "recovery_category": {
        "state": "unavailable",
        "value": {
          "reason": "fixture recovery category unavailable"
        }
      },
      "recovery_action": {
        "state": "unavailable",
        "value": {
          "reason": "fixture recovery action unavailable"
        }
      },
      "resource_pressure": {
        "state": "unavailable",
        "value": {
          "reason": "fixture resource pressure unavailable"
        }
      }
    },
    "peers": {
      "peer_counts": {
        "state": "unavailable",
        "value": {
          "reason": "fixture peer counts unavailable"
        }
      },
      "recent_peers": {
        "state": "available",
        "value": []
      }
    },
    "updated_at_unix_seconds": 1777225150
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-unavailable-final-status.sh"

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
          "header_height": 840004,
          "block_height": 840004,
          "downloaded_block_height": 840004,
          "connected_block_height": 840004,
          "validated_active_chain_height": 840004,
          "maybe_downloaded_block_hash": null,
          "maybe_connected_block_hash": null,
          "maybe_validated_active_chain_hash": "1111111111111111111111111111111111111111111111111111111111111111",
          "maybe_validated_active_chain_work": "840005",
          "headers_received": 2,
          "blocks_received": 1,
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
      "configured_targets": {
        "state": "available",
        "value": {
          "target_outbound_peers": 4,
          "maybe_target_header_height": 840200
        }
      },
      "attempt_counters": {
        "state": "available",
        "value": {
          "attempted_peers": 3,
          "connected_peers": 3,
          "failed_peers": 1,
          "max_sync_rounds": 8
        }
      },
      "progress_signal": {
        "state": "available",
        "value": "waiting_for_peers"
      },
      "latest_stop_reason": {
        "state": "available",
        "value": {
          "label": "target_header_reached",
          "message": "sync header target reached"
        }
      },
      "last_error": {
        "state": "unavailable",
        "value": {
          "reason": "no sync error recorded"
        }
      },
      "recovery_category": {
        "state": "available",
        "value": "public_network_unreachable"
      },
      "recovery_action": {
        "state": "available",
        "value": "Retry with a reachable manual peer."
      },
      "resource_pressure": {
        "state": "available",
        "value": {
          "blocks_in_flight": 0,
          "max_header_requests_in_flight_per_peer": 1,
          "max_headers_per_message": 2000,
          "max_blocks_in_flight_per_peer": 16,
          "max_blocks_in_flight_total": 64,
          "max_messages_per_peer": 64,
          "max_sync_rounds": 8,
          "outbound_peers": 0,
          "target_outbound_peers": 4
        }
      },
      "best_known_tip": {
        "state": "available",
        "value": {
          "source": "header_store",
          "height": 840004,
          "block_hash": "1111111111111111111111111111111111111111111111111111111111111111",
          "work": "840005",
          "block_time_unix_seconds": 1777224990,
          "observed_at_unix_seconds": 1777225005,
          "freshness": "fresh",
          "peer_agreement": []
        }
      },
      "stay_current": {
        "state": "available",
        "value": "current_at_best_known_tip"
      },
      "stay_current_next_action": {
        "state": "available",
        "value": "Continue monitoring best-known tip freshness."
      },
      "no_progress_diagnosis": {
        "state": "available",
        "value": "current_at_best_known_tip"
      },
      "no_progress_next_action": {
        "state": "available",
        "value": "No operator action required."
      },
      "progress_credit": {
        "state": "available",
        "value": {
          "kind": "validated_durable_active_chain",
          "credited_validated_active_chain_height": 840004,
          "credited_validated_active_chain_hash": "1111111111111111111111111111111111111111111111111111111111111111",
          "credited_validated_active_chain_work": "840005",
          "source_unix_seconds": 1777225005,
          "rejected_activity": [
            {
              "kind": "header_download",
              "observed_count": 2,
              "reason": "headers alone do not prove durable progress"
            }
          ]
        }
      },
      "expected_progress_window": {
        "state": "available",
        "value": {
          "retry_backoff_seconds": 30,
          "max_sync_rounds": 8,
          "expected_progress_window_seconds": 300,
          "tip_freshness_threshold_seconds": 600
        }
      },
      "no_progress_threshold": {
        "state": "available",
        "value": {
          "threshold_seconds": 300,
          "elapsed_since_last_useful_work_seconds": 5,
          "state": "within_window",
          "evaluated_at_unix_seconds": 1777225010
        }
      },
      "last_useful_work": {
        "state": "available",
        "value": {
          "kind": "current_at_best_known_tip",
          "credited_validated_active_chain_height": 840004,
          "credited_validated_active_chain_hash": "1111111111111111111111111111111111111111111111111111111111111111",
          "credited_validated_active_chain_work": "840005",
          "source_unix_seconds": 1777225005,
          "rejected_activity": []
        }
      },
      "last_peer_contribution": {
        "state": "available",
        "value": {
          "peer": "198.51.100.10:8333",
          "maybe_resolved_endpoint": "198.51.100.10:8333",
          "kind": "headers_and_blocks",
          "messages_processed": 4,
          "headers_received": 2,
          "blocks_received": 1,
          "maybe_last_activity_unix_seconds": 1777225100,
          "maybe_failure_reason_label": null
        }
      },
      "stall_diagnosis": {
        "state": "available",
        "value": {
          "stalled_subsystem": "at_tip_waiting",
          "confidence": "high",
          "evidence_basis": ["validated_active_chain", "fresh_tip"],
          "next_action": "No operator action required.",
          "maybe_no_progress_diagnosis": "current_at_best_known_tip",
          "maybe_recovery_category": null,
          "maybe_latest_stop_reason_label": "target_header_reached",
          "source_unix_seconds": 1777225010
        }
      },
      "latest_reorg": {
        "state": "available",
        "value": {
          "common_ancestor_height": 840000,
          "common_ancestor_hash": "0000000000000000000000000000000000000000000000000000000000000000",
          "disconnected_count": 0,
          "connected_count": 4,
          "final_active_height": 840004,
          "final_active_hash": "1111111111111111111111111111111111111111111111111111111111111111",
          "fully_persisted": true
        }
      },
      "reconcile_progress": {
        "state": "available",
        "value": {
          "state": "extended_active_chain",
          "details": {
            "connected_count": 4,
            "final_active_height": 840004,
            "final_active_hash": "1111111111111111111111111111111111111111111111111111111111111111"
          }
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
  },
  "recovery_evidence": {
    "state": "available",
    "value": {
      "category": "storage_lock_contention",
      "action_class": "read_only_inspection",
      "cause": "stale_lock_evidence",
      "evidence_basis": ["lock_probe"],
      "maybe_affected_namespace": null,
      "maybe_affected_path": "/tmp/open-bitcoin/LOCK",
      "next_action": "Inspect the datadir read-only and avoid deleting lock artifacts automatically.",
      "compatibility_action": {
        "state": "unavailable",
        "value": {
          "reason": "no compatibility recovery action recorded"
        }
      }
    }
  }
}
JSON
EOF
chmod +x "$tmp_dir/mock-final-status.sh"
sed '/"validated_active_chain_height":/d' "$tmp_dir/mock-final-status.sh" >"$tmp_dir/mock-final-status-missing-validated-height.sh"
chmod +x "$tmp_dir/mock-final-status-missing-validated-height.sh"

cat >"$tmp_dir/mock-peer-failure-final-status.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

failure_reason="${OPEN_BITCOIN_LIVE_SMOKE_FAILURE_REASON:?}"
last_error="${OPEN_BITCOIN_LIVE_SMOKE_LAST_ERROR:-}"
outbound_peers="${OPEN_BITCOIN_LIVE_SMOKE_OUTBOUND_PEERS:-0}"
recovery_category="${OPEN_BITCOIN_LIVE_SMOKE_RECOVERY_CATEGORY:-invalid_peer_data}"
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
          "headers_received": 0,
          "blocks_received": 0,
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
      "configured_targets": {
        "state": "available",
        "value": {
          "target_outbound_peers": 8,
          "maybe_target_header_height": null
        }
      },
      "attempt_counters": {
        "state": "available",
        "value": {
          "attempted_peers": 3,
          "connected_peers": 1,
          "failed_peers": 2,
          "max_sync_rounds": 25
        }
      },
      "progress_signal": {
        "state": "available",
        "value": "peer_failures"
      },
      "latest_stop_reason": {
        "state": "available",
        "value": {
          "label": "no_progress",
          "message": "sync made no progress before timeout"
        }
      },
      "recovery_category": {
        "state": "available",
        "value": "$recovery_category"
      },
      "recovery_action": {
        "state": "available",
        "value": "Retry sync after peer backoff."
      },
      "resource_pressure": {
        "state": "available",
        "value": {
          "blocks_in_flight": 3,
          "max_header_requests_in_flight_per_peer": 2,
          "max_headers_per_message": 2000,
          "max_blocks_in_flight_per_peer": 16,
          "max_blocks_in_flight_total": 64,
          "max_messages_per_peer": 128,
          "max_sync_rounds": 25,
          "outbound_peers": 1,
          "target_outbound_peers": 8
        }
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

report_json="$output_dir/open-bitcoin-live-mainnet-smoke.json"
report_markdown="$output_dir/open-bitcoin-live-mainnet-smoke.md"
generated_config="$output_dir/open-bitcoin-live-mainnet-smoke.jsonc"
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
bun --eval 'const report = await Bun.file(process.argv[1]).json(); if (report.result.firstHeaderProgress.before.headerHeight !== 0 || report.result.firstHeaderProgress.after.headerHeight !== 1) throw new Error("firstHeaderProgress headerHeight evidence missing"); if (report.result.firstHeaderProgress.before.progressSignal !== "waiting_for_peers" || report.result.firstHeaderProgress.after.progressSignal !== "header_progress") throw new Error("firstHeaderProgress progressSignal evidence missing"); if (report.snapshots[0].progressSignal !== "waiting_for_peers") throw new Error("snapshot progressSignal evidence missing"); if (report.snapshots.at(-1).progressCreditKind !== "validated_durable_active_chain" || report.snapshots.at(-1).expectedProgressWindowSeconds !== 300 || report.snapshots.at(-1).stalledSubsystem !== "at_tip_waiting") throw new Error("phase78 snapshot evidence missing"); if (report.final_status.configuredTargets.targetOutboundPeers !== 4 || report.final_status.configuredTargets.maybeTargetHeaderHeight !== 840200) throw new Error("final configuredTargets evidence missing"); if (report.final_status.attemptCounters.attemptedPeers !== 3 || report.final_status.attemptCounters.connectedPeers !== 3 || report.final_status.attemptCounters.failedPeers !== 1 || report.final_status.attemptCounters.maxSyncRounds !== 8) throw new Error("final attemptCounters evidence missing"); if (report.final_status.latestStopReason.label !== "target_header_reached") throw new Error("latestStopReason evidence missing"); if (report.final_status.recoveryAction !== "Retry with a reachable manual peer.") throw new Error("recoveryAction evidence missing"); if (report.final_status.recoveryEvidence?.category !== "storage_lock_contention" || report.final_status.recoveryActionClass !== "read_only_inspection" || report.final_status.recoveryCause !== "stale_lock_evidence" || report.final_status.recoveryNextAction !== "Inspect the datadir read-only and avoid deleting lock artifacts automatically." || report.final_status.maybeRecoveryEvidenceUnavailableReason !== null) throw new Error("phase77 recovery evidence missing"); if (report.final_status.resourcePressure.targetOutboundPeers !== 4) throw new Error("resourcePressure evidence missing"); if (report.final_status.progressCreditKind !== "validated_durable_active_chain" || report.final_status.progressCreditHeight !== 840004 || report.final_status.progressCreditSourceUnixSeconds !== 1777225005 || report.final_status.expectedProgressWindowSeconds !== 300 || report.final_status.noProgressThresholdState !== "within_window" || report.final_status.noProgressThresholdSeconds !== 300 || report.final_status.lastUsefulWorkKind !== "current_at_best_known_tip" || report.final_status.lastUsefulWorkHeight !== 840004 || report.final_status.lastPeerContribution?.kind !== "headers_and_blocks" || report.final_status.stalledSubsystem !== "at_tip_waiting" || report.final_status.stallConfidence !== "high" || report.final_status.stallEvidenceBasis.join(",") !== "validated_active_chain,fresh_tip" || report.final_status.stallNextAction !== "No operator action required.") throw new Error("phase78 live-smoke final status evidence missing"); if (report.final_status.validatedActiveChainHeight !== 840004 || report.final_status.maybeValidatedActiveChainHash !== "1111111111111111111111111111111111111111111111111111111111111111" || report.final_status.maybeValidatedActiveChainWork !== "840005" || report.final_status.bestKnownTip?.freshness !== "fresh" || report.final_status.stayCurrent !== "current_at_best_known_tip" || report.final_status.stayCurrentNextAction !== "Continue monitoring best-known tip freshness." || report.final_status.noProgressDiagnosis !== "current_at_best_known_tip" || report.final_status.noProgressNextAction !== "No operator action required." || report.final_status.latestReorg?.fullyPersisted !== true || report.final_status.reconcileProgress?.state !== "extended_active_chain" || report.final_status.peerContribution?.connected !== 3 || report.final_status.peerContribution?.failed !== 1) throw new Error("phase72 live-smoke final status evidence missing"); if (report.result.firstBlockProgress.before.downloadedBlockHeight !== 0 || report.result.firstBlockProgress.after.connectedBlockHeight !== 1) throw new Error("firstBlockProgress downloadedBlockHeight/connectedBlockHeight evidence missing"); if ("stdoutTail" in report.daemon || "stderrTail" in report.daemon) throw new Error("daemon tails persisted in JSON");' "$report_json"

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
bun --eval 'const report = await Bun.file(process.argv[1]).json(); if (report.final_status.validatedActiveChainHeight !== null) throw new Error("missing validated active-chain height was synthesized"); if (report.final_status.maybeValidatedActiveChainHeightUnavailableReason !== "validated active-chain height unavailable") throw new Error("missing validated active-chain height reason missing"); if (report.final_status.connectedBlockHeight !== 840004) throw new Error("connected block height evidence was lost");' "$missing_validated_height_json"
grep -q "Validated active-chain height: Unavailable: validated active-chain height unavailable" "$missing_validated_height_markdown"

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
bun --eval 'const report = await Bun.file(process.argv[1]).json(); if (typeof report.result.restartResumeEvidence.recoveryDiagnosis.category !== "string") throw new Error("restartResumeEvidence recoveryDiagnosis category missing");' "$report_json"
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
grep -q "Unavailable: no sync status snapshots captured" "$report_markdown"
assert_report_redacts_command_credentials "$report_json" "$report_markdown"

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
