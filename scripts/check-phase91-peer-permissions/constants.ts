import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE91_REPO_ROOT";
export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v1-9-peer-permissions-connection-classes";
export const AUDIT_KEY = "v1_9_peer_permissions_connection_classes";
export const PHASE90_CHECKER_COMMAND =
  "bun run scripts/check-phase90-inbound-listener-admission.ts";
export const PHASE91_TEST_COMMAND =
  "bun test scripts/check-phase91-peer-permissions.test.ts";
export const PHASE91_CHECKER_COMMAND = "bun run scripts/check-phase91-peer-permissions.ts";
export const REQUIRED_PERMISSION_TOKENS =
  "in,noban,forceinbound,download,addr,relay,forcerelay,mempool,bloomfilter,blockfilters";
export const PHASE91_REQUIREMENTS = ["PERM-01", "PERM-02", "PERM-03", "PERM-04"] as const;
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
    label: "Cargo permission daemon startup",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
      "-openbitcoininbound=1",
      "-openbitcoinlisten=127.0.0.1:18444",
      "-openbitcoinreservedslots=1",
      `-openbitcoininboundpermissionclass=operator_loopback@127.0.0.1=${REQUIRED_PERMISSION_TOKENS}`,
    ],
  },
  {
    label: "Bazel permission daemon startup",
    required: [
      "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
      "-openbitcoininbound=1",
      "-openbitcoinlisten=127.0.0.1:18444",
      "-openbitcoinreservedslots=1",
      `-openbitcoininboundpermissionclass=operator_loopback@127.0.0.1=${REQUIRED_PERMISSION_TOKENS}`,
    ],
  },
  {
    label: "Cargo permission network status",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --",
      "openbitcoinnetworkstatus",
    ],
  },
  {
    label: "Bazel permission network status",
    required: [
      "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --",
      "openbitcoinnetworkstatus",
    ],
  },
  {
    label: "Cargo permission status JSON",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
      "status --format json",
    ],
  },
  {
    label: "Bazel permission status JSON",
    required: [
      "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
      "status --format json",
    ],
  },
  {
    label: "Cargo permission support bundle",
    required: [
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
      "support bundle --output-dir=/tmp/open-bitcoin-permission-support",
    ],
  },
  {
    label: "Bazel permission support bundle",
    required: [
      "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
      "support bundle --output-dir=/tmp/open-bitcoin-permission-support",
    ],
  },
] as const;
export const REQUIRED_EVIDENCE_LABELS = [
  "inbound.permission_classes",
  "openbitcoininboundpermissionclass",
  "literal IP",
  "CIDR ranges",
  "hostnames",
  "endpoint-shaped values",
  "OpenBitcoinStatusSnapshot.peers.inbound",
  "permission_class",
  "permissioned_inbound_peers",
  "protected_inbound_peers",
  "active_permission_effects",
  "inactive_permission_effects",
  "latest_permission_decision",
  "inactive_relay",
  "inactive_forcerelay",
  "inactive_mempool",
  "inactive_bloomfilter",
  "inactive_blockfilters",
] as const;
export const REQUIRED_METRICS = [
  "InboundPermissionedAdmitCount",
  "InboundProtectedAdmitCount",
  "InboundInactivePermissionEffectCount",
  "InboundPermissionValidationFailureCount",
] as const;
export const REQUIRED_CATALOG_ANCHORS = [
  "packages/bitcoin-knots/src/net_permissions.h",
  "packages/bitcoin-knots/src/net_permissions.cpp",
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/test/functional/p2p_permissions.py",
] as const;
export const REQUIRED_BREADCRUMB_MAPPINGS = [
  {
    label: "network-peer-permissions",
    files: ["packages/open-bitcoin-network/src/inbound/permissions.rs"],
    breadcrumbs: [
      "packages/bitcoin-knots/src/net_permissions.h",
      "packages/bitcoin-knots/src/net_permissions.cpp",
      "packages/bitcoin-knots/test/functional/p2p_permissions.py",
    ],
  },
] as const;
export const FORBIDDEN_VERIFY_STRINGS = [
  "public-network",
  "service-manager",
  "multi-day",
  "whitebind",
  "whitelist",
  "nc -z",
  "curl ",
  "0.0.0.0",
  "[::]",
  "systemctl",
  "launchctl",
  "sleep 259200",
  "sleep 86400",
] as const;
export const FORBIDDEN_UNSCOPED_CLAIMS = [
  "transaction relay support",
  "compact block relay support",
  "mempool propagation support",
  "BIP37 serving support",
  "BIP37 bloom serving support",
  "compact filter serving support",
  "compact-filter serving support",
  "full address relay support",
  "public inbound by default",
  "production full-node readiness",
  "accepts Knots whitelist",
  "accepts Knots whitebind",
  "whitelist compatibility is supported",
  "whitebind compatibility is supported",
  "silently accepts whitelist",
  "silently accepts whitebind",
  "all activates transaction relay",
  "all activates compact block relay",
  "all activates mempool propagation",
] as const;
export const FORBIDDEN_SUPPORT_RAW_DETAILS = [
  "operator_loopback",
  "peer_id=",
  "127.0.0.1:",
  "rpc_password",
  "rpcpassword",
  "cookie=",
  REQUIRED_PERMISSION_TOKENS,
] as const;
export const ALLOWED_SCOPE_TERMS = [
  "does not",
  "do not",
  "not a",
  "not part of",
  "not silently accepted",
  "not support evidence",
  "must not",
  "without",
  "outside",
  "rejected",
  "reject",
  "deferred",
  "future",
  "diagnostic evidence only",
  "inactive",
  "redacted",
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
  upstream?: {
    sources?: unknown;
    tests?: unknown;
  };
};

export type BreadcrumbGroup = {
  breadcrumbs?: unknown;
  files?: unknown;
  label?: unknown;
};

export type BreadcrumbIndex = {
  groups?: unknown;
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
