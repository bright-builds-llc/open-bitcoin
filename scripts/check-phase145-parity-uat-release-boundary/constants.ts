import path from "node:path";

export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");

export const D14_SENTENCE =
  "disk-backed per-outpoint coins, typed cache-flush policy, fuller chainstate-manager behavior for the single active chainstate, and honest stored-block availability that serves or reports a stored block only when the payload bytes are present";

export const D14_REQUIRED_FILES = ["README.md", "docs/operator/runtime-guide.md"] as const;

export const CLOSEOUT_SURFACE = "v2-3-parity-roots-and-no-claim-guardrails";
export const PHASE139_SURFACE = "v2-3-coins-view-cache-contract";
export const CSVFY_IDS = ["CSVFY-01", "CSVFY-02"] as const;

export const CAN_FLUSH_TO_DISK = "CanFlushToDisk";
export const CHECK_BLOCK_DATA_AVAILABILITY = "CheckBlockDataAvailability";

export const PHASE117_CHECK = "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
export const PHASE138_CHECK = "bun run scripts/check-phase138-parity-uat-release-boundary.ts";
export const PHASE144_TEST =
  "bun test scripts/check-phase144-operator-flush-availability-evidence.test.ts";
export const PHASE144_CHECK =
  "bun run scripts/check-phase144-operator-flush-availability-evidence.ts";
export const PHASE145_TEST = "bun test scripts/check-phase145-parity-uat-release-boundary.test.ts";
export const PHASE145_CHECK = "bun run scripts/check-phase145-parity-uat-release-boundary.ts";

export const PHASE117_CHECK_STEP =
  `run_step "check Phase 117 parity UAT release boundary" ${PHASE117_CHECK}`;
export const PHASE138_CHECK_STEP =
  `run_step "check Phase 138 parity UAT release boundary" ${PHASE138_CHECK}`;
export const PHASE144_TEST_STEP =
  `run_step "test Phase 144 operator flush and availability evidence checker" ${PHASE144_TEST}`;
export const PHASE144_CHECK_STEP =
  `run_step "check Phase 144 operator flush and availability evidence" ${PHASE144_CHECK}`;
export const PHASE145_TEST_STEP =
  `run_step "test Phase 145 parity UAT release boundary checker" ${PHASE145_TEST}`;
export const PHASE145_CHECK_STEP =
  `run_step "check Phase 145 parity UAT release boundary" ${PHASE145_CHECK}`;

export const UAT_PACKAGE =
  ".planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md";
export const HISTORICAL_PHASE138_DIR =
  ".planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails";

export const REQUIREMENTS_BY_SURFACE = {
  "v2-3-coins-view-cache-contract": ["CACHE-01"],
  "v2-3-pure-flush-policy-and-typed-decisions": ["FLUSH-01", "MGR-03"],
  "v2-3-durable-fjall-coins-adapter": ["COIN-01", "CSOBS-03"],
  "v2-3-manager-flush-lifecycle-and-restart": ["MGR-01", "MGR-02", "FLUSH-02"],
  "v2-3-honest-stored-block-availability": ["HAVL-01", "HAVL-02", "HAVL-03"],
  "v2-3-operator-flush-availability-evidence": ["CSOBS-01", "CSOBS-02"],
  "v2-3-parity-roots-and-no-claim-guardrails": ["CSVFY-01", "CSVFY-02"],
} as const;

export const REQUIRED_TOP_LEVEL_NAMES = [
  "v2-3-coins-view-cache-contract",
  "v2-3-pure-flush-policy-and-typed-decisions",
  "v2-3-durable-fjall-coins-adapter",
  "v2-3-manager-flush-lifecycle-and-restart",
  "v2-3-honest-stored-block-availability",
  "v2-3-operator-flush-availability-evidence",
  CLOSEOUT_SURFACE,
] as const;

export const CLAIM_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/checklist.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
] as const;

export const REQUIRED_DOC_FILES = [
  ...CLAIM_FILES,
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
  UAT_PACKAGE,
] as const;

export function allV23RequirementIds(): string[] {
  return Object.values(REQUIREMENTS_BY_SURFACE).flatMap((requirements) => [...requirements]);
}

export const REQUIRED_UAT_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-chainstate-durability-support",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-chainstate-durability-support",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-preview",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview getblockchaininfo",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview getblockchaininfo",
  PHASE145_TEST,
  PHASE145_CHECK,
  "bash scripts/verify.sh",
] as const;

export const STORED_BLOCK_PRESENCE_GROUP = "node-stored-block-presence";
export const STORED_BLOCK_PRESENCE_ANCHOR =
  "packages/bitcoin-knots/src/node/blockstorage.cpp";

export const REQUIRED_BREADCRUMB_GROUPS = [
  "chainstate-engine",
  "node-coins-adapter",
  "node-chainstate-adapter",
  "node-network-chainstate-durability-evidence",
  STORED_BLOCK_PRESENCE_GROUP,
] as const;

export const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/coins.h",
  "packages/bitcoin-knots/src/coins.cpp",
  "packages/bitcoin-knots/src/validation.cpp",
  "packages/bitcoin-knots/src/node/chainstate.cpp",
  "packages/bitcoin-knots/src/node/blockstorage.cpp",
] as const;

export const DENIED_OVERCLAIMS = [
  "prune-mode",
  "archive-node",
  "production-scale historical serving",
  "assumeutxo",
  "assumevalid",
  "ibd snapshot",
  "compact-filter",
  "bip37",
  "public serving or relay by default",
  "public-network ci",
  "production full-node readiness",
  "production service operation",
  "production-funds",
  "leveldb chainstate",
  "destructive reindex",
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

export const FORBIDDEN_RUN_STEP_TOKENS = [
  "public-network",
  "wall-clock",
  "run-live-mainnet-smoke",
  "command-timings.ts",
] as const;

export type ClaimFile = (typeof CLAIM_FILES)[number];
export type RequiredDocFile = (typeof REQUIRED_DOC_FILES)[number];
export type SurfaceId = keyof typeof REQUIREMENTS_BY_SURFACE;
