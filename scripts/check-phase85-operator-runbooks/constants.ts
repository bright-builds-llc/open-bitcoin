import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE85_REPO_ROOT";
export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v1-8-operator-runbooks";
export const AUDIT_KEY = "v1_8_operator_runbooks";
export const PHASE85_REQUIREMENTS = ["RUN-01", "RUN-02", "RUN-03"] as const;
export const RUNBOOK_PATH = "docs/parity/operator-runbooks.md";
export const TABLE_HEADER = "Evidence to record | How to collect it | Mutation status | Escalation use";
export const PHASE84_CHECKER_COMMAND =
  "bun run scripts/check-phase84-upgrade-rollback-policy.ts";
export const PHASE85_TEST_COMMAND =
  "bun test scripts/check-phase85-operator-runbooks.test.ts";
export const PHASE85_CHECKER_COMMAND =
  "bun run scripts/check-phase85-operator-runbooks.ts";
export const TARGET_FILES = [
  RUNBOOK_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
export const HUMAN_POINTER_FILES = TARGET_FILES.filter(
  (file) => file !== RUNBOOK_PATH && file !== "docs/parity/index.json" && file !== "scripts/verify.sh",
);
export const REQUIRED_EVIDENCE = [
  RUNBOOK_PATH,
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
export const RUNBOOK_HEADINGS = [
  "# Operator Runbooks",
  "## Scope And Non-Claims",
  "## Production-Boundary Preflight",
  "## Long-Run Monitoring",
  "## No-Progress Diagnosis",
  "## Recovery And Stop Decisions",
  "## Escalation Evidence Thresholds",
  "## Support-Bundle Timeline",
  "## Privacy And Safety Boundaries",
] as const;
export const SUPPORT_TERMS = [
  "supported",
  "preview",
  "opt-in UAT",
  "unsupported",
  "deferred",
] as const;
export const PREFLIGHT_ITEMS = [
  "selected datadir",
  "source revision",
  "repo-local verification status",
  "Cargo or Bazel command form",
  "config paths",
  "current status evidence",
  "resource/disk review",
  "service state or unavailable reason",
  "wallet scope",
  "support-bundle availability",
] as const;
export const STATUS_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<path> status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=<path> status --format json",
] as const;
export const MONITORING_FIELDS = [
  "progress_credit",
  "last_useful_work",
  "last_peer_contribution",
  "expected_progress_window",
  "no_progress_threshold",
  "stall_diagnosis",
  "sync.no_progress_diagnosis",
  "sync.no_progress_next_action",
  "latest_stop_reason",
  "resource_bounds",
  "sync.resource_pressure",
  "recovery_evidence",
  "support_forensics",
] as const;
export const STRUCTURED_MONITORING_TERMS = [
  "structured logs",
  "metrics",
  "support-bundle summaries",
  "soak reports",
  "live-smoke reports",
  "checkpoint timeline",
  "stalled subsystem",
  "public-network opt-in",
  "stay-current opt-in",
  "multi-day soak opt-in",
] as const;
export const REQUIRED_INSUFFICIENT_SIGNALS = [
  "elapsed time",
  "daemon startup",
  "peer reachability",
  "raw log tail",
  "report existence",
  "support bundle existence",
] as const;
export const PROOF_SIGNALS = [
  "artifact existence",
  "elapsed time",
  "daemon startup",
  "peer reachability",
  "raw logs",
  "raw log tail",
  "report existence",
  "support bundle existence",
] as const;
export const ACTION_CLASSES = [
  "safe_retry",
  "read_only_inspection",
  "backup_then_rebuild",
  "stop_and_escalate",
] as const;
export const ESCALATION_THRESHOLDS = [
  "repeated no-progress with typed cause",
  "unavailable critical fields",
  "recovery class requiring stop/escalate",
  "resource pressure crossing documented bounds",
  "inconsistent status/support evidence",
  "failure to collect the minimum redacted support-bundle timeline",
] as const;
export const FORBIDDEN_BOUNDARY_TERMS = [
  "destructive repair",
  "source datadir mutation",
  "external wallet mutation",
  "service-manager mutation",
  "config rewrite",
  "automatic rebuild",
  "response timelines",
  "hosted support upload",
  "production service ownership",
] as const;
export const FORBIDDEN_PERMISSION_STRINGS = [
  "destructive repair is allowed",
  "source datadir mutation is allowed",
  "external wallet mutation is allowed",
  "service-manager mutation is allowed",
  "config rewrite is allowed",
  "automatic rebuild is allowed",
  "automatic support-bundle upload is supported",
  "automatic support-bundle upload is allowed",
] as const;
export const TIMELINE_LABELS = [
  "preflight evidence",
  "command start",
  "status snapshots",
  "progress or no-progress events",
  "resource/recovery events",
  "support-bundle collection",
  "operator action taken",
  "final status",
  "escalation decision",
] as const;
export const MINIMUM_BUNDLE_ITEMS = [
  "support-evidence.json",
  "support-evidence.md",
  "exact command output",
  "bounded log summary",
  "config summary",
  "service state or unavailable reason",
  "resource evidence",
  "recovery/progress evidence",
  "sync evidence",
  "version/toolchain context",
  "platform details",
  "exact repo-local reproduction command",
  "Unavailable: <reason>",
] as const;
export const SUPPORT_BUNDLE_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<path> support bundle --output-dir=<path>/support --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=<path> support bundle --output-dir=<path>/support --format json",
] as const;
export const FORBIDDEN_EVIDENCE_ITEMS = [
  "wallet private material",
  "raw wallet files",
  "RPC cookies",
  "rpcpassword",
  "rpcauth",
  "raw datadirs",
  "unredacted logs",
  "raw unbounded logs",
  "full peer tables with sensitive local data",
  "automatic support-bundle upload",
] as const;
export const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 259200",
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
