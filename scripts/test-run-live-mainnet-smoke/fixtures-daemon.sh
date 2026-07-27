#!/usr/bin/env bash
set -euo pipefail

install_daemon_fixtures() {
	local tmp_dir="$1"
	local network_fixture="$tmp_dir/network-preflight.json"
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
}
