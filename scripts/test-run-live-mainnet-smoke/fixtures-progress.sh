#!/usr/bin/env bash
set -euo pipefail
install_progress_fixtures() { local tmp_dir="$1"
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
}
