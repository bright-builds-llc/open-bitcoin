import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v1-9-network-participation-release-boundary";
export const PHASE94_TEST_COMMAND = "bun test scripts/check-phase94-dos-resource-governance.test.ts";
export const PHASE94_CHECKER_COMMAND = "bun run scripts/check-phase94-dos-resource-governance.ts";
export const PHASE95_TEST_COMMAND = "bun test scripts/check-phase95-network-participation-release-boundary.test.ts";
export const PHASE95_CHECKER_COMMAND = "bun run scripts/check-phase95-network-participation-release-boundary.ts";
export const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
export const REQUIRED_PHASE95_REQUIREMENTS = [
  "BOUND-01", "BOUND-02", "BOUND-03", "BOUND-04", "BOUND-05", "BOUND-06",
] as const;
export const PHASE_REQUIREMENTS = {
  "v1-9-inbound-listener-admission-policy": [
    "INB-01", "INB-02", "INB-03", "INB-04", "INB-05",
  ],
  "v1-9-peer-permissions-connection-classes": [
    "PERM-01", "PERM-02", "PERM-03", "PERM-04",
  ],
  "v1-9-address-advertisement-discovery-boundaries": [
    "ADDR-01", "ADDR-02", "ADDR-03", "ADDR-04",
  ],
  "v1-9-eviction-ban-misbehavior-policy": [
    "EVICT-01", "EVICT-02", "EVICT-03", "EVICT-04",
  ],
  "v1-9-dos-resource-governance": [
    "DOS-01", "DOS-02", "DOS-03", "DOS-04", "DOS-05",
  ],
  [SURFACE_ID]: REQUIRED_PHASE95_REQUIREMENTS,
} as const;
export const REQUIRED_V1_9_REQUIREMENTS = Object.values(PHASE_REQUIREMENTS).flat();
export const REQUIREMENT_PHASE_ASSIGNMENTS = {
  "INB-01": 98,
  "INB-02": 98,
  "INB-03": 98,
  "INB-04": 98,
  "INB-05": 97,
  "PERM-01": 91,
  "PERM-02": 91,
  "PERM-03": 91,
  "PERM-04": 91,
  "ADDR-01": 92,
  "ADDR-02": 92,
  "ADDR-03": 92,
  "ADDR-04": 92,
  "EVICT-01": 93,
  "EVICT-02": 93,
  "EVICT-03": 96,
  "EVICT-04": 96,
  "DOS-01": 94,
  "DOS-02": 94,
  "DOS-03": 96,
  "DOS-04": 97,
  "DOS-05": 94,
  "BOUND-01": 95,
  "BOUND-02": 95,
  "BOUND-03": 95,
  "BOUND-04": 95,
  "BOUND-05": 95,
  "BOUND-06": 98,
} as const;
export const ROADMAP_TRACEABILITY_ROWS = [
  { phase: 90, requirements: [] },
  { phase: 91, requirements: ["PERM-01", "PERM-02", "PERM-03", "PERM-04"] },
  { phase: 92, requirements: ["ADDR-01", "ADDR-02", "ADDR-03", "ADDR-04"] },
  { phase: 93, requirements: ["EVICT-01", "EVICT-02"] },
  { phase: 94, requirements: ["DOS-01", "DOS-02", "DOS-05"] },
  { phase: 95, requirements: ["BOUND-01", "BOUND-02", "BOUND-03", "BOUND-04", "BOUND-05"] },
  { phase: 96, requirements: ["EVICT-03", "EVICT-04", "DOS-03"] },
  { phase: 97, requirements: ["INB-05", "DOS-04"] },
  { phase: 98, requirements: ["INB-01", "INB-02", "INB-03", "INB-04", "BOUND-06"] },
] as const;
export const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net.cpp", "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/addrman.cpp", "packages/bitcoin-knots/src/banman.cpp",
  "packages/bitcoin-knots/src/net_permissions.cpp",
] as const;
export const REQUIRED_PHASE95_EVIDENCE = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/operator/runtime-guide.md",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase95-network-participation-release-boundary.ts",
  "scripts/check-phase95-network-participation-release-boundary.test.ts",
  "scripts/verify.sh",
  ".planning/milestones/v1.9-REQUIREMENTS.md",
  ".planning/milestones/v1.9-ROADMAP.md",
] as const;
export const REQUIRED_UAT_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind",
  "bazel run //packages/open-bitcoin-rpc:open_bitcoind",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin",
  "openbitcoinnetworkstatus",
  "status --format json",
  "support bundle --output-dir=/tmp/open-bitcoin-inbound-support",
] as const;
export const REQUIRED_SUPPORT_REDACTION_ROOTS = [
  "INBOUND_ENDPOINT_REDACTION_SAFEGUARD",
  "INBOUND_PERMISSION_REDACTION_SAFEGUARD",
  "INBOUND_ADDRESS_REDACTION_SAFEGUARD",
  "INBOUND_PEER_POLICY_REDACTION_SAFEGUARD",
  "INBOUND_RESOURCE_GOVERNANCE_REDACTION_SAFEGUARD",
  "inbound resource-governance evidence bounded/redacted",
  "redact_inbound_resource_governance_evidence",
  "redacted_resource_governance_evidence",
  "sanitized_resource_governance_text",
  "inbound_support_redacts_raw_phase94_resource_governance_material",
  "peer_id=",
  "raw_endpoint",
  "payload_bytes",
  "permission_string",
  "credential",
  "secret",
  "cookie=",
  "config=",
] as const;
export const FORBIDDEN_POSITIVE_CLAIMS = [
  "transaction relay support",
  "compact block relay support",
  "mempool propagation support",
  "full address relay support",
  "full address relay",
  "public inbound default",
  "public inbound defaults",
  "public inbound by default",
  "public-network ci",
  "production service operation",
  "production-service operation",
  "production full-node readiness",
] as const;
export const POSITIVE_CLAIM_VERBS = [
  "provides", "provide", "supports", "support", "adds", "add", "enables", "enable",
  "includes", "include", "ships", "ship", "has", "have",
] as const;
export const FORBIDDEN_VERIFY_STRINGS = [
  "openbitcoinlisten=0.0.0.0",
  "public-network CI",
  "production full-node readiness",
  "production service operation",
  "production-service operation",
  "systemctl",
  "launchctl",
  "service-manager",
  "sleep 259200",
  "sleep 86400",
  "--restart-after-progress",
  "run-live-mainnet-smoke",
] as const;
export const CLAIM_SCAN_FILES = [
  "README.md",
  "docs/parity/checklist.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/operator/runtime-guide.md",
] as const;
export const TARGET_FILES = [
  ".planning/milestones/v1.9-REQUIREMENTS.md",
  ".planning/milestones/v1.9-ROADMAP.md",
  "README.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/operator/runtime-guide.md",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/verify.sh",
] as const;

export type TargetFile = (typeof TARGET_FILES)[number];
export type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown };
};
export type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
export type ParitySurface = { name?: unknown; status?: unknown };
export type CheckPhase95Options = { rootDir?: string };
