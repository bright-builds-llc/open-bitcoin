import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE90_REPO_ROOT";
export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v1-9-inbound-listener-admission-policy";
export const AUDIT_KEY = "v1_9_inbound_listener_admission_policy";
export const PHASE88_CHECKER_COMMAND =
  "bun run scripts/check-phase88-deterministic-claim-guardrails.ts";
export const PHASE90_TEST_COMMAND =
  "bun test scripts/check-phase90-inbound-listener-admission.test.ts";
export const PHASE90_CHECKER_COMMAND =
  "bun run scripts/check-phase90-inbound-listener-admission.ts";
export const PHASE90_REQUIREMENTS = [
  "INB-01",
  "INB-02",
  "INB-03",
  "INB-04",
  "INB-05",
] as const;
export const TARGET_FILES = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
] as const;
export const REQUIRED_EVIDENCE = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/source-breadcrumbs.json",
] as const;
export const REQUIRED_UAT_COMMANDS = [
  {
    label: "Cargo daemon startup",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
      "-openbitcoininbound=1",
      "-openbitcoinlisten=127.0.0.1:18444",
    ],
  },
  {
    label: "Bazel daemon startup",
    required: [
      "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
      "-openbitcoininbound=1",
      "-openbitcoinlisten=127.0.0.1:18444",
    ],
  },
  {
    label: "Cargo getnetworkinfo",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --",
      "getnetworkinfo",
    ],
  },
  {
    label: "Bazel getnetworkinfo",
    required: [
      "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --",
      "getnetworkinfo",
    ],
  },
  {
    label: "Cargo openbitcoinnetworkstatus",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --",
      "openbitcoinnetworkstatus",
    ],
  },
  {
    label: "Bazel openbitcoinnetworkstatus",
    required: [
      "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --",
      "openbitcoinnetworkstatus",
    ],
  },
  {
    label: "Cargo status JSON",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
      "--format json",
      "status",
    ],
  },
  {
    label: "Bazel status JSON",
    required: [
      "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
      "--format json",
      "status",
    ],
  },
  {
    label: "Cargo support bundle",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
      "support bundle --output-dir=/tmp/open-bitcoin-inbound-support",
    ],
  },
  {
    label: "Bazel support bundle",
    required: [
      "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
      "support bundle --output-dir=/tmp/open-bitcoin-inbound-support",
    ],
  },
] as const;
export const REQUIRED_EVIDENCE_LABELS = [
  "openbitcoinnetworkstatus",
  "openbitcoininbound",
  "openbitcoinlisten",
  "inbound.allow_public",
  "OpenBitcoinStatusSnapshot.peers.inbound",
  "inbound_listener_state",
  "inbound_preflight_reason",
  "bound_endpoint",
  "admission_reject_reason",
  "reserved_slot",
  "connections_in",
  "connections_out",
] as const;
export const REQUIRED_CATALOG_ANCHORS = [
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/test/functional/p2p_handshake.py",
] as const;
export const REQUIRED_BREADCRUMB_MAPPINGS = [
  {
    label: "network-inbound-admission",
    files: [
      "packages/open-bitcoin-network/src/inbound.rs",
      "packages/open-bitcoin-network/src/inbound/tests.rs",
    ],
    breadcrumbs: [
      "packages/bitcoin-knots/src/net.cpp",
      "packages/bitcoin-knots/src/net_processing.cpp",
      "packages/bitcoin-knots/test/functional/p2p_handshake.py",
    ],
  },
  {
    label: "rpc-inbound-listener",
    files: [
      "packages/open-bitcoin-rpc/src/inbound_listener.rs",
      "packages/open-bitcoin-rpc/src/inbound_listener/tests.rs",
    ],
    breadcrumbs: [
      "packages/bitcoin-knots/src/net.cpp",
      "packages/bitcoin-knots/src/net_processing.cpp",
    ],
  },
  {
    label: "node-status-contract",
    files: [
      "packages/open-bitcoin-node/src/status/inbound.rs",
      "packages/open-bitcoin-node/src/status/inbound/tests.rs",
    ],
    breadcrumbs: [],
  },
  {
    label: "cli-operator-onboarding-contracts",
    files: ["packages/open-bitcoin-cli/src/operator/status/render/inbound.rs"],
    breadcrumbs: [],
  },
  {
    label: "cli-operator-support-bundles",
    files: ["packages/open-bitcoin-cli/src/operator/support/render/inbound.rs"],
    breadcrumbs: [],
  },
] as const;
export const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "test-run-live-mainnet-smoke",
  "nc -z",
  "curl ",
  "0.0.0.0",
  "[::]",
  "-openbitcoinlisten=::",
  "systemctl",
  "launchctl",
  "sleep 259200",
  "sleep 86400",
  "transaction relay",
  "compact block relay",
  "mempool propagation",
  "permission classes",
  "address relay",
  "eviction",
  "ban policy",
  "DoS governance",
] as const;
export const PUBLIC_DEFAULT_CLAIMS = [
  "supports public inbound by default",
  "public inbound by default",
  "public inbound serving by default",
  "public listener defaults are supported",
] as const;
export const PRODUCTION_READY_CLAIMS = [
  "Open Bitcoin is production full-node ready.",
  "Open Bitcoin has production full-node readiness.",
  "v1.9 proves production full-node readiness.",
  "production full-node readiness is supported",
] as const;
export const ALLOWED_SCOPE_TERMS = [
  "does not",
  "do not",
  "not a",
  "not part of",
  "without",
  "outside",
  "opt-in",
  "deferred",
  "future",
  "disabled by default",
  "remains",
  "remain",
  "no-claim",
  "non-claim",
] as const;
export const COMMAND_PREFIXES = Array.from(
  new Set(REQUIRED_UAT_COMMANDS.map((command) => command.required[0])),
);

export type AuditEntry = {
  evidence?: unknown;
  path?: unknown;
  requirements?: unknown;
  status?: unknown;
};

export type BreadcrumbIndex = {
  groups?: unknown;
};

export type BreadcrumbGroup = {
  breadcrumbs?: unknown;
  files?: unknown;
  label?: unknown;
};

export type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

export type ParityIndex = {
  audit?: Record<string, unknown>;
  checklist?: {
    surfaces?: unknown;
  };
  surfaces?: unknown;
};

export type ParitySurface = {
  name?: unknown;
  status?: unknown;
};

export type TargetFile = (typeof TARGET_FILES)[number];
