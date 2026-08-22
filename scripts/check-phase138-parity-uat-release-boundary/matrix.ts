import { COMPOSITION_FILE, COMPOSITION_SYMBOL } from "./constants.ts";

export type MatrixMethod = "pinned-Knots" | "fake-clock" | "graph-oracle" | "failure-injection";
export type MatrixBehavior = "package" | "rolling-fee" | "pressure" | "expiry" | "recovery" | "retry";

export type MatrixCell = {
  method: MatrixMethod;
  behavior: MatrixBehavior;
  file: string;
  symbol: string;
};

export const MATRIX_CELLS: readonly MatrixCell[] = [
  {
    method: "pinned-Knots",
    behavior: "package",
    file: "packages/open-bitcoin-mempool/src/package/tests/empty_package_is_rejected.rs",
    symbol: "package_fingerprint_matches_knots_fixed_vector",
  },
  {
    method: "fake-clock",
    behavior: "package",
    file: "scripts/check-phase132-typed-package-staged-admission.ts",
    symbol: "PACK-01",
  },
  {
    method: "graph-oracle",
    behavior: "package",
    file: "packages/open-bitcoin-mempool/src/pool/tests/prospective_oracle_cases.rs",
    symbol: "generated_graph_recomputation_oracle_covers_twenty_five_sparse_additions",
  },
  {
    method: "failure-injection",
    behavior: "package",
    file: "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs",
    symbol: "every_injected_preflight_failure_preserves_the_complete_aggregate",
  },
  {
    method: "pinned-Knots",
    behavior: "rolling-fee",
    file: "packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs",
    symbol: "rolling_fee_decay_twelve_hour_halflife_at_high_occupancy",
  },
  {
    method: "fake-clock",
    behavior: "rolling-fee",
    file: "packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs",
    symbol: "rolling_fee_decay_does_not_run_before_block_after_bump",
  },
  {
    method: "graph-oracle",
    behavior: "rolling-fee",
    file: "packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs",
    symbol: "sustained_pressure_oracle_agrees_across_fill_trim_block_decay_expiry_refill_reorg",
  },
  {
    method: "failure-injection",
    behavior: "rolling-fee",
    file: "packages/open-bitcoin-mempool/src/fee/rolling.rs",
    symbol: "set_rolling_fee_rate_updates_inject_seam",
  },
  {
    method: "pinned-Knots",
    behavior: "pressure",
    file: "packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs",
    symbol: "accounted_capacity_trim_evicts_until_usage_within_capacity",
  },
  {
    method: "fake-clock",
    behavior: "pressure",
    file: "packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs",
    symbol: "sustained_pressure_oracle_agrees_across_fill_trim_block_decay_expiry_refill_reorg",
  },
  {
    method: "graph-oracle",
    behavior: "pressure",
    file: "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs",
    symbol: "generated_steps",
  },
  {
    method: "failure-injection",
    behavior: "pressure",
    file: "packages/open-bitcoin-mempool/src/pool/tests/prospective_failure_cases.rs",
    symbol: "missing_pressure_victim_is_rejected",
  },
  {
    method: "pinned-Knots",
    behavior: "expiry",
    file: "packages/open-bitcoin-mempool/src/pool/tests/expiry_cases.rs",
    symbol: "expiry_removes_aged_entry_and_descendants",
  },
  {
    method: "fake-clock",
    behavior: "expiry",
    file: "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases/lifecycle_cases.rs",
    symbol: "expiry_uses_injected_time_without_sleeping",
  },
  {
    method: "graph-oracle",
    behavior: "expiry",
    file: "packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs",
    symbol: "Expiry",
  },
  {
    method: "failure-injection",
    behavior: "expiry",
    file: "packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs",
    symbol: "expiry_preparation_is_pure_and_orders_descendants_before_ancestors",
  },
  {
    method: "pinned-Knots",
    behavior: "recovery",
    file: "packages/open-bitcoin-node/src/network/tests/recovery_cases.rs",
    symbol: "recovery_topology_orders_parent_before_child_independent_of_stored_order",
  },
  {
    method: "fake-clock",
    behavior: "recovery",
    file: "packages/open-bitcoin-node/src/network/tests/recovery_cases/staging.rs",
    symbol: "prepare_mempool_recovery_at",
  },
  {
    method: "graph-oracle",
    behavior: "recovery",
    file: "packages/open-bitcoin-node/src/network/tests/recovery_cases.rs",
    symbol: "recovery_topology_orders_parent_before_child_independent_of_stored_order",
  },
  {
    method: "failure-injection",
    behavior: "recovery",
    file: "packages/open-bitcoin-node/src/network/tests/recovery_cases/staging.rs",
    symbol: "every_injected_recovery_install_validation_failure_preserves_the_exact_aggregate",
  },
  {
    method: "pinned-Knots",
    behavior: "retry",
    file: "packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs",
    symbol: "retry_cycle_length_is_ten_minutes_plus_injected_jitter",
  },
  {
    method: "fake-clock",
    behavior: "retry",
    file: "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/retry.rs",
    symbol: "retry_worker_fires_maintenance_tick_on_elapsed_without_sleep",
  },
  {
    method: "graph-oracle",
    behavior: "retry",
    file: "packages/open-bitcoin-node/src/network/tests/maintenance_tick_cases.rs",
    symbol: "maintenance_tick_walks_only_unbroadcast_members",
  },
  {
    method: "failure-injection",
    behavior: "retry",
    file: "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/retry.rs",
    symbol: "retry_worker_jitter_unavailable_does_not_use_silent_constant_jitter",
  },
] as const;

export const COMPOSITION_CELL = {
  file: COMPOSITION_FILE,
  symbol: COMPOSITION_SYMBOL,
} as const;

export function cellLabel(cell: Pick<MatrixCell, "behavior" | "method">): string {
  return `${cell.behavior}/${cell.method}`;
}
