#!/usr/bin/env bash
set -euo pipefail

install_failure_fixtures() {
	local tmp_dir="$1"
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
}
