import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v2-1-runtime-timing-evidence-integrity";
export const REQUIREMENTS = ["HARD-02", "HARD-03", "HARD-04"] as const;
export const PHASE122_CHECK =
  "bun run scripts/check-phase122-compact-relay-peer-completion.ts";
export const PHASE123_TEST =
  "bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts";
export const PHASE123_CHECK =
  "bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts";
export const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";

export const TARGET_FILES = [
  "packages/open-bitcoin-node/src/sync/types.rs",
  "packages/open-bitcoin-node/src/sync/tcp.rs",
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-node/src/sync/session.rs",
  "packages/open-bitcoin-node/src/sync/block_reconcile.rs",
  "packages/open-bitcoin-node/src/sync/block_response.rs",
  "packages/open-bitcoin-node/src/lib.rs",
  "packages/open-bitcoin-bench/src/runtime_fixtures.rs",
  "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
  "packages/open-bitcoin-node/src/metrics/block_relay.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/status/block_relay_evidence.rs",
  "packages/open-bitcoin-rpc/src/context.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener/tests.rs",
  "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs",
  "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
  "packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs",
  "packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs",
  "scripts/check-phase121-block-relay-metrics-log-runtime.ts",
  "docs/architecture/operator-observability.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
] as const;

export type TargetFile = (typeof TARGET_FILES)[number];
export type CheckOptions = { rootDir?: string };
export type ParityIndex = {
  surfaces?: unknown[];
  checklist?: { surfaces?: unknown[] };
};
export type NamedSurface = { name?: string; status?: string };
export type ChecklistSurface = {
  id?: string;
  title?: string;
  status?: string;
  requirements?: string[];
  evidence?: string[];
  rationale?: string;
  upstream?: { sources?: string[]; tests?: string[] };
  known_gaps?: string[];
  suspected_unknowns?: string[];
};
export type BreadcrumbManifest = {
  groups?: Array<{ files?: string[]; breadcrumbs?: string[] }>;
};
