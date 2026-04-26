# Operator Observability Contracts

## Default metrics retention

Metrics history defaults to a 30 seconds sampling interval, 2880 samples per series, and a 24 hours maximum age. The intent is to give status and dashboard consumers a bounded day-scale window without creating unbounded runtime storage.

Required metric kinds are sync height, header height, peer count, mempool transactions, wallet trusted balance in sats, disk usage bytes, RPC health, and service restarts.

No metric or log retention contract may require public network access. Default verification must remain hermetic; live-network telemetry belongs behind explicit opt-in tests or operator runtime paths.

## Default log retention

Structured logs default to daily rotation, 14 files, 14 days, and 268435456 bytes of total retained log data. Rolling file creation is not retention pruning. Phase 16 must implement pruning separately from any rolling file writer and must test max-file, max-age, and byte-cap behavior.

Status and dashboard consumers must read these contracts instead of inventing renderer-local retention windows.

## Phase Boundaries

Phase 13 defines serializable contracts only. It must not install a tracing subscriber, create a file appender, write metric samples, prune log files, or render dashboard graphs. Runtime writers and readers are Phase 16 responsibilities.
