import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE86_REPO_ROOT";
export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v1-8-service-operation-expectations";
export const AUDIT_KEY = "v1_8_service_operation_expectations";
export const PHASE86_REQUIREMENTS = ["SVC-01", "SVC-02"] as const;
export const SERVICE_DOC_PATH = "docs/parity/service-operation-expectations.md";
export const TABLE_HEADER =
  "Service surface | Support term | What evidence proves | Cargo command evidence | Bazel command evidence | Default verification | Opt-in UAT | Residual risk | Next gate";
export const PHASE85_CHECKER_COMMAND =
  "bun run scripts/check-phase85-operator-runbooks.ts";
export const PHASE86_TEST_COMMAND =
  "bun test scripts/check-phase86-service-operation-expectations.test.ts";
export const PHASE86_CHECKER_COMMAND =
  "bun run scripts/check-phase86-service-operation-expectations.ts";
export const TARGET_FILES = [
  SERVICE_DOC_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
export const HUMAN_POINTER_FILES = TARGET_FILES.filter(
  (file) =>
    file !== SERVICE_DOC_PATH &&
    file !== "docs/parity/index.json" &&
    file !== "scripts/verify.sh",
);
export const REQUIRED_EVIDENCE = [
  SERVICE_DOC_PATH,
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/check-phase86-service-operation-expectations.ts",
  "scripts/check-phase86-service-operation-expectations.test.ts",
  "scripts/verify.sh",
] as const;
export const REQUIRED_HEADINGS = [
  "# Service Operation Expectations",
  "## Scope And Non-Claims",
  "## Support Terms",
  "## Service Surface Classification",
  "## Repo-Local Command Evidence",
  "## Field-Based Evidence Rules",
  "## Restart Resume Evidence",
  "## Default Verification And Opt-In UAT Boundaries",
  "## Sensitive Evidence Boundaries",
] as const;
export const SUPPORT_TERMS = [
  "supported",
  "preview",
  "opt-in UAT",
  "unsupported",
  "deferred",
] as const;
export const SERVICE_SURFACES = [
  "Direct source-built open-bitcoind operation",
  "Local status and support evidence",
  "launchd/systemd generated definition preview",
  "Real user-level launchd/systemd lifecycle",
  "Service-manager unavailable status",
  "Packaged service distribution",
  "Windows service integration",
  "Automatic updates",
  "Production service ownership and uptime guarantees",
  "Broad production full-node readiness",
] as const;
export const SERVICE_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd -server=1",
  "bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd -server=1",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support",
] as const;
export const FIELD_EVIDENCE_TERMS = [
  "service file existence",
  "daemon startup",
  "elapsed time",
  "raw log tail",
  "public peer reachability",
  "support bundle path",
  "expected fields and unavailable reasons",
  "Unavailable: <reason>",
] as const;
export const SERVICE_FIELDS = [
  "service.lifecycle",
  "service.log_path",
  "service.manager_command",
  "service.generated_service_file_path",
  "service.unavailable_reason",
  "resource_bounds",
  "sync.resource_pressure",
  "recovery_category",
  "recovery_action",
  "next_action",
  "support-evidence.json",
  "support-evidence.md",
] as const;
export const LIFECYCLE_LABELS = [
  "unmanaged",
  "installed-stopped",
  "running",
  "failed",
  "disabled",
  "unavailable-manager",
] as const;
export const RESTART_RESUME_FIELDS = [
  "same_datadir",
  "prior_shutdown",
  "durable_progress",
  "stale_inflight",
  "recovery_category",
  "next_action",
] as const;
export const PROOF_SIGNALS = [
  "service file existence",
  "daemon startup",
  "elapsed time",
  "raw log tail",
  "public peer reachability",
  "support bundle path",
] as const;
export const SENSITIVE_EVIDENCE_TERMS = [
  "wallet private material",
  "raw wallet files",
  "RPC cookies",
  "rpcpassword",
  "rpcauth",
  "raw datadirs",
  "unredacted logs",
  "raw unbounded logs",
  "automatic support-bundle upload",
  "production service ownership",
] as const;
export const FORBIDDEN_DOC_PERMISSION_STRINGS = [
  "default verification runs real service-manager commands",
  "default verification runs public-network live smoke",
  "default verification runs long wall-clock sleeps",
  "service file existence proves",
  "daemon startup proves",
  "elapsed time proves",
  "raw log tail proves",
  "public peer reachability proves",
  "support bundle path proves",
  "automatic support-bundle upload is supported",
  "automatic support-bundle upload is allowed",
  "production service ownership is supported",
] as const;
export const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 259200",
  "--restart-after-progress",
  "brew services",
  "Windows service",
  "automatic support-bundle upload",
  "production service ownership",
  "broad production-node readiness",
] as const;

export type AuditEntry = {
  evidence?: unknown;
  path?: unknown;
  requirements?: unknown;
  status?: unknown;
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
