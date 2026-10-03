import path from "node:path";

export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");

export const V24_PRUNE_SENTENCE =
  "Knots-aligned prune on the single active chainstate deletes old block and undo payloads inside a height window by removing Fjall keys, advertises NODE_NETWORK_LIMITED, and reports Pruned only after a durable delete.";

export const V23_D14_SENTENCE =
  "disk-backed per-outpoint coins, typed cache-flush policy, fuller chainstate-manager behavior for the single active chainstate, and honest stored-block availability that serves or reports a stored block only when the payload bytes are present.";

export const FJALL_DIFFERENCE_SENTENCE =
  "Open Bitcoin removes paired Fjall block and undo keys for eligible heights and does not introduce a Knots blk/rev flat-file store.";

export const D14_REQUIRED_FILES = ["README.md", "docs/operator/runtime-guide.md"] as const;

export const CLOSEOUT_SURFACE = "v2-4-parity-roots-and-no-claim-guardrails";
export const WALLET_LEFTOVER_SURFACE = "v2-4-wallet-leftover-snapshot-cutover";
export const GRD01_ID = "GRD-01";
export const GRD01_CHECKBOX = "- [x] **GRD-01**";

export const PHASE138_CHECK = "bun run scripts/check-phase138-parity-uat-release-boundary.ts";
export const PHASE145_TEST = "bun test scripts/check-phase145-parity-uat-release-boundary.test.ts";
export const PHASE145_CHECK = "bun run scripts/check-phase145-parity-uat-release-boundary.ts";
export const PHASE151_TEST = "bun test scripts/check-phase151-parity-uat-release-boundary.test.ts";
export const PHASE151_CHECK = "bun run scripts/check-phase151-parity-uat-release-boundary.ts";

export const PHASE145_TEST_STEP =
  `run_step "test Phase 145 parity UAT release boundary checker" ${PHASE145_TEST}`;
export const PHASE145_CHECK_STEP =
  `run_step "check Phase 145 parity UAT release boundary" ${PHASE145_CHECK}`;
export const PHASE151_TEST_STEP =
  `run_step "test Phase 151 parity UAT release boundary checker" ${PHASE151_TEST}`;
export const PHASE151_CHECK_STEP =
  `run_step "check Phase 151 parity UAT release boundary" ${PHASE151_CHECK}`;

export const UAT_PACKAGE =
  ".planning/phases/151-parity-roots-and-no-claim-guardrails/151-UAT.md";
export const HISTORICAL_PHASE_DIR =
  ".planning/phases/145-parity-roots-and-no-claim-guardrails";

export const REQUIREMENTS_BY_SURFACE = {
  "v2-4-wallet-leftover-snapshot-cutover": ["SNAP-01"],
  "v2-4-pure-prune-policy-and-lock-windows": ["PRUN-01", "PRUN-02", "PRUN-03", "LOCK-01"],
  "v2-4-fjall-payload-unlink-and-have-pruned": ["UNLK-01", "UNLK-02", "UNLK-03"],
  "v2-4-limited-serving-and-honest-pruned-labels": ["SERV-01", "SERV-02", "SERV-03", "LABL-01"],
  "v2-4-operator-prune-surfaces-and-evidence": ["OPER-01", "OPER-02", "OPER-03", "LOCK-02"],
  "v2-4-parity-roots-and-no-claim-guardrails": ["GRD-01"],
} as const;

export const REQUIRED_TOP_LEVEL_NAMES = [
  "v2-4-wallet-leftover-snapshot-cutover",
  "v2-4-pure-prune-policy-and-lock-windows",
  "v2-4-fjall-payload-unlink-and-have-pruned",
  "v2-4-limited-serving-and-honest-pruned-labels",
  "v2-4-operator-prune-surfaces-and-evidence",
  CLOSEOUT_SURFACE,
] as const;

export const CLAIM_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/checklist.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
] as const;

export const CATALOG_FILES = [
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/rpc-cli-config.md",
] as const;

export const REQUIREMENTS_FILE = ".planning/REQUIREMENTS.md";
export const ROADMAP_FILE = ".planning/ROADMAP.md";
export const ARCHIVED_V24_REQUIREMENTS = ".planning/milestones/v2.4-REQUIREMENTS.md";
export const ARCHIVED_V24_ROADMAP = ".planning/milestones/v2.4-ROADMAP.md";
export const V24_REQUIREMENTS_MILESTONE_NEEDLE = "**Milestone:** v2.4 ";
export const AUDIT_FILE = ".planning/v2.4-MILESTONE-AUDIT.md";
export const ARCHIVED_V24_AUDIT = ".planning/milestones/v2.4-MILESTONE-AUDIT.md";

export const REQUIRED_DOC_FILES = [
  ...CLAIM_FILES,
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
  UAT_PACKAGE,
  REQUIREMENTS_FILE,
  ROADMAP_FILE,
] as const;

export function allV24RequirementIds(): string[] {
  return Object.values(REQUIREMENTS_BY_SURFACE).flatMap((requirements) => [...requirements]);
}

export function checkedV24RequirementIds(): string[] {
  return allV24RequirementIds().filter((id) => id !== GRD01_ID);
}

export const REQUIRED_UAT_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune run 1000",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- prune run 1000",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune lock list",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune lock set --name ibd --height-first 1 --height-last 2",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune lock clear --name ibd",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- prune lock list",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- prune lock set --name ibd --height-first 1 --height-last 2",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- prune lock clear --name ibd",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-preview",
  "bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir=/tmp/open-bitcoin-preview",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview getblockchaininfo",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview getblockchaininfo",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview pruneblockchain 1000",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview pruneblockchain 1000",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview listprunelocks",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview listprunelocks",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview setprunelock",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview setprunelock",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview clearprunelock",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview clearprunelock",
  PHASE151_TEST,
  PHASE151_CHECK,
  "bash scripts/verify.sh",
] as const;

export const BREADCRUMB_GROUP = "cli-operator-prune";
export const BREADCRUMB_ANCHOR = "packages/bitcoin-knots/src/rpc/blockchain.cpp";

export const REQUIRED_CLOSEOUT_SOURCES = [
  "packages/bitcoin-knots/src/validation.h",
  "packages/bitcoin-knots/src/validation.cpp",
  "packages/bitcoin-knots/src/node/blockmanager_args.cpp",
  "packages/bitcoin-knots/src/node/blockstorage.h",
  "packages/bitcoin-knots/src/node/blockstorage.cpp",
  "packages/bitcoin-knots/src/protocol.h",
  "packages/bitcoin-knots/src/init.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/rpc/blockchain.cpp",
] as const;

export const REQUIRED_KNOTS_SYMBOLS = [
  "MIN_BLOCKS_TO_KEEP",
  "MIN_DISK_SPACE_FOR_BLOCK_FILES",
  "GetPruneRange",
  "ParsePruneOption",
  "PruneLockInfo",
  "DoPruneLocksForbidPruning",
  "PRUNE_LOCK_BUFFER",
  "FindFilesToPrune",
  "UnlinkPrunedFiles",
  "m_have_pruned",
  "NODE_NETWORK_LIMITED",
  "pruneblockchain",
  "pruneheight",
  "automatic_pruning",
  "prune_target_size",
] as const;

export const DENIED_OVERCLAIMS = [
  "archive-node",
  "production-scale historical serving",
  "assumeutxo",
  "assumevalid",
  "second chainstate",
  "bip37",
  "compact-filter",
  "public serving or relay by default",
  "public-network ci",
  "production full-node readiness",
  "production service operation",
  "production-funds",
  "blk/rev",
  "pruneduringinit",
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

export const PUBLIC_NETWORK_NOT_RUN_LINE = "not run";

export type ClaimFile = (typeof CLAIM_FILES)[number];
export type RequiredDocFile = (typeof REQUIRED_DOC_FILES)[number];
export type SurfaceId = keyof typeof REQUIREMENTS_BY_SURFACE;
