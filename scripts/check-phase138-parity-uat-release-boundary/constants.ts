import path from "node:path";

export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");

export const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
export const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
export const PHASE138_TEST =
  "bun test scripts/check-phase138-parity-uat-release-boundary.test.ts";
export const PHASE138_CHECK =
  "bun run scripts/check-phase138-parity-uat-release-boundary.ts";
export const RECONCILIATION_TEST =
  "bun test scripts/check-current-documentation-reconciliation.test.ts";
export const RECONCILIATION_CHECK =
  "bun run scripts/check-current-documentation-reconciliation.ts";

export const PHASE117_TEST_STEP = `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`;
export const PHASE117_CHECK_STEP = `run_step "check Phase 117 parity UAT release boundary" ${PHASE117_CHECK}`;
export const PHASE138_TEST_STEP = `run_step "test Phase 138 parity UAT release boundary checker" ${PHASE138_TEST}`;
export const PHASE138_CHECK_STEP = `run_step "check Phase 138 parity UAT release boundary" ${PHASE138_CHECK}`;
export const RECONCILIATION_TEST_STEP = `run_step "test current documentation reconciliation checker" ${RECONCILIATION_TEST}`;
export const RECONCILIATION_CHECK_STEP = `run_step "check current documentation reconciliation" ${RECONCILIATION_CHECK}`;

export const D21_SENTENCE =
  "bounded local-package APIs, same-peer 1P1C assembly over ordinary transaction messages, ordinary transaction fanout, and initial-broadcast-retry of locally submitted unbroadcast members";

export const CLOSEOUT_SURFACE = "v2-2-parity-uat-release-boundary";
export const SNAPSHOT_TOP_LEVEL_NAME = "v2 snapshot schema, checkpointing, and recovery";
export const SNAPSHOT_CHECKLIST_ID = "v2-2-snapshot-schema-checkpointing-recovery";
export const PHASE132_SURFACE = "v2-2-typed-package-staged-admission";

export const UAT_PACKAGE =
  ".planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/138-UAT.md";

export const REQUIREMENTS_BY_SURFACE = {
  "v2-2-resource-time-fee-primitives": ["FEEP-01", "FEEP-02", "FEEP-03", "FEEP-04", "FEEP-05"],
  "v2-2-rolling-fee-expiry-pressure": ["PRESS-01", "PRESS-02", "PRESS-03", "PRESS-04", "PRESS-05"],
  "v2-2-typed-package-staged-admission": [
    "PACK-01",
    "PACK-02",
    "PACK-03",
    "PACK-04",
    "PACK-05",
    "PACK-06",
    "PACK-07",
  ],
  "v2-2-package-aware-download-orphan-bridge": ["PPKG-01", "PPKG-02", "PPKG-03"],
  "v2-2-authoritative-cross-cache-lifecycle-integration": [
    "MPLIFE-01",
    "MPLIFE-02",
    "MPLIFE-03",
    "MPLIFE-04",
  ],
  "v2-2-snapshot-schema-checkpointing-recovery": ["MPDUR-01", "MPDUR-02", "MPDUR-03", "MPDUR-04"],
  "v2-2-receive-independent-maintenance-and-transport-receipts": [
    "PPKG-04",
    "IBR-01",
    "IBR-02",
    "IBR-03",
    "IBR-04",
  ],
  "v2-2-rpc-and-sanitized-operator-evidence": ["MPOBS-01", "MPOBS-02", "MPOBS-03"],
  "v2-2-parity-uat-release-boundary": ["MPVFY-01", "MPVFY-02", "MPVFY-03", "MPVFY-04"],
} as const;

export const MPVFY_IDS = ["MPVFY-01", "MPVFY-02", "MPVFY-03", "MPVFY-04"] as const;

export const REQUIRED_TOP_LEVEL_NAMES = [
  "v2-2-resource-time-fee-primitives",
  "v2-2-rolling-fee-expiry-pressure",
  "v2-2-typed-package-staged-admission",
  "v2-2-package-aware-download-orphan-bridge",
  "v2-2-authoritative-cross-cache-lifecycle-integration",
  SNAPSHOT_TOP_LEVEL_NAME,
  "v2-2-receive-independent-maintenance-and-transport-receipts",
  "v2-2-rpc-and-sanitized-operator-evidence",
  CLOSEOUT_SURFACE,
] as const;

export const CLAIM_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/checklist.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
] as const;

export const REQUIREMENTS_FILE = ".planning/REQUIREMENTS.md";

export const REQUIRED_DOC_FILES = [
  ...CLAIM_FILES,
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
  "scripts/check-benchmark-report.ts",
  "packages/open-bitcoin-bench/src/cases/mempool.rs",
  UAT_PACKAGE,
  REQUIREMENTS_FILE,
] as const;

export function allV22RequirementIds(): string[] {
  return Object.values(REQUIREMENTS_BY_SURFACE).flatMap((requirements) => [...requirements]);
}

export const D21_REQUIRED_FILES = ["README.md", "docs/operator/runtime-guide.md"] as const;

export const RUNNABLE_CARGO_FILTERS = [
  "cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib retry",
  "cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib maintenance_tick",
  "cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib package_fanout",
  "cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib retry_worker",
] as const;

export const REQUIRED_UAT_COMMANDS = [
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -datadir=/tmp/open-bitcoin-mainnet testmempoolaccept '["<hex>"]'`,
  `bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -datadir=/tmp/open-bitcoin-mainnet testmempoolaccept '["<hex>"]'`,
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -datadir=/tmp/open-bitcoin-mainnet submitpackage '["<hex>"]'`,
  `bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -datadir=/tmp/open-bitcoin-mainnet submitpackage '["<hex>"]'`,
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet package dry-run --hex '<hex>'",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet package dry-run --hex '<hex>'",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet package submit --hex '<hex>'",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet package submit --hex '<hex>'",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  PHASE138_TEST,
  PHASE138_CHECK,
  "bash scripts/verify.sh",
] as const;

export const REQUIRED_BREADCRUMB_GROUPS = [
  "mempool-resource-accounting",
  "mempool-entry-context",
  "mempool-lifecycle",
  "mempool-package-policy",
  "mempool-package-parity-closure",
  "mempool-staged-admission",
  "mempool-package-fee-policy",
  "network-initial-broadcast-retry-inputs",
  "node-initial-broadcast-retry",
  "node-package-admission-bridge",
  "node-local-package-admission",
  "node-mempool-recovery-topology",
  "node-mempool-checkpoint-coordinator",
  "rpc-package-projection",
  "cli-operator-package",
  "bench-mempool-policy",
] as const;

export const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/txmempool.cpp",
  "packages/bitcoin-knots/src/policy/packages.cpp",
  "packages/bitcoin-knots/src/rpc/mempool.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
] as const;

export const DENIED_OVERCLAIMS = [
  "general package wire",
  "arbitrary multi-parent",
  "whole-mempool rebroadcast",
  "public relay by default",
  "production relay",
  "guaranteed propagation",
  "public-network ci",
  "production full-node readiness",
  "production-funds",
  "package relay",
] as const;

export const POSITIVE_PATTERNS = [
  /\bsupports\b/,
  /\bprovides?\b/,
  /\benables?\b/,
  /\badds?\b/,
  /\bimplements?\b/,
  /\bships?\b/,
  /\bproves?\b/,
  /\bis supported\b/,
  /\bis enabled\b/,
  /\bis ready\b/,
] as const;

export const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "is not",
  "are not",
  "not a ",
  "must not",
  "not required",
  "not supported",
  "not run",
  "without claiming",
  "without making",
  "without turning",
  "without broadening",
  "outside",
  "out of scope",
  "deferred",
  "remain deferred",
  "remains deferred",
  "future",
  "future-gated",
  "no claim",
  "optional uat",
] as const;

export const FORBIDDEN_BENCH_TOKENS = ["SUSTAINED_PRESSURE_MAX_ELAPSED", "Instant::now"] as const;
export const REQUIRED_BENCH_TOKEN = "SUSTAINED_PRESSURE_TRIM_CYCLES";
export const THRESHOLD_FREE = "threshold_free";

export const FORBIDDEN_RUN_STEP_TOKENS = [
  "public-network",
  "wall-clock",
  "run-live-mainnet-smoke",
  "command-timings.ts",
] as const;

export const COMPOSITION_FILE =
  "packages/open-bitcoin-node/src/network/tests/recovery_cases/restart_composition.rs";
export const COMPOSITION_SYMBOL =
  "recovery_restart_preserves_local_package_unbroadcast_remints_retry_and_keeps_injected_install_failure_inert";

export type ClaimFile = (typeof CLAIM_FILES)[number];
export type RequiredDocFile = (typeof REQUIRED_DOC_FILES)[number];
export type SurfaceId = keyof typeof REQUIREMENTS_BY_SURFACE;
