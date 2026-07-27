import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE83_REPO_ROOT";
export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const SURFACE_ID = "v1-8-support-matrix-issue-evidence";
export const PHASE83_REQUIREMENTS = ["SUP-01", "SUP-02", "SUP-03", "SUP-04"] as const;
export const SUPPORT_TERMS = ["supported", "preview", "opt-in UAT", "unsupported", "deferred"] as const;
export const FORBIDDEN_MATURITY_LABELS = [
  "best-effort",
  "beta",
  "production-grade",
  "production-ish",
  "partial production",
  "community-supported",
  "GA",
  "certified",
  "validated",
  "fully supported",
] as const;
export const MATRIX_COLUMNS = [
  "Environment family",
  "Support term",
  "Evidence basis",
  "Default verification",
  "Opt-in UAT / manual validation",
  "Residual risk",
  "Next gate",
] as const;
export const PLACEHOLDER_MATRIX_VALUES = [
  "evidence basis",
  "default verification",
  "opt-in UAT evidence",
  "opt-in UAT / manual validation",
  "residual risk",
  "next gate",
  "todo",
  "tbd",
  "n/a",
] as const;
export const REQUIRED_ENVIRONMENT_FAMILIES = [
  "source-built install and repo verification",
  "repo-local operator command forms through Cargo and Bazel",
  "local deterministic runtime, status, config, RPC, and support-bundle surfaces",
  "operator dashboard and shipped operator convenience surfaces",
  "public-network mainnet activation, full-sync, stay-current, and soak evidence",
  "storage/datadir resource-bound evidence and recovery diagnosis",
  "live storage pressure and long-run resource behavior",
  "launchd/systemd service-supervision previews",
  "real launchd/systemd service-manager lifecycle",
  "migration dry-run",
  "migration apply, source service mutation, and source datadir rewrite",
  "support bundle and support forensics",
  "wallet current non-production slice",
  "production-funds wallet use and safety",
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "signed packaging or package-manager distribution",
  "Windows service integration",
  "hosted dashboards and GUI parity",
  "automatic support-bundle upload",
  "destructive repair",
  "public-network default checks, public-network CI, and release-blocking live sync",
  "broad production-node readiness",
] as const;
export const REQUIRED_ISSUE_EVIDENCE = [
  "smallest useful redacted evidence set",
  "Unavailable: <reason>",
  "support-evidence.json",
  "support-evidence.md",
  "Relevant command output",
  "Bounded redacted logs",
  "configuration summary",
  "Service state",
  "resource-bound or resource-pressure evidence",
  "recovery/progress evidence",
  "sync status evidence",
  "version, commit, Rust, Cargo, Bun, and Bazel context",
  "Platform details",
  "exact repo-local command",
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
export const RESIDUAL_RISK_SURFACES = [
  "dashboard pseudoterminal/raw-input repaint and input behavior",
  "closeout without a dedicated milestone audit artifact",
  "diagnosed-blocker closeout and fresh status supersession",
  "planning traceability correction during archive prep",
  "public-network full-sync, stay-current, and soak evidence",
  "real service-manager lifecycle evidence",
  "multi-day wall-clock soak evidence",
  "support-bundle forensics",
  "recovery diagnosis versus destructive repair",
  "production-scope non-claims",
] as const;
export const REQUIRED_EVIDENCE = [
  "docs/parity/support-matrix.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/wallet.md",
  "docs/parity/catalog/drop-in-audit-and-migration.md",
  "scripts/verify.sh",
] as const;
export const TARGET_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/support-matrix.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/wallet.md",
  "docs/parity/catalog/drop-in-audit-and-migration.md",
  "scripts/verify.sh",
] as const;
export const HUMAN_POINTER_FILES = TARGET_FILES.filter((file) => file !== "docs/parity/index.json");
export const PHASE82_CHECKER_COMMAND = "bun run scripts/check-phase82-production-claim-boundary.ts";
export const PHASE83_TEST_COMMAND =
  "bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts";
export const PHASE83_CHECKER_COMMAND =
  "bun run scripts/check-phase83-support-matrix-issue-evidence.ts";
export const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-" + "smoke",
  "system" + "ctl",
  "launch" + "ctl",
  "sleep " + "259200",
  "automatic support-bundle upload" + " --",
  "destructive repair" + " --",
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

export type MatrixTable = {
  header: string[];
  rows: MatrixRow[];
};

export type MatrixRow = {
  cells: string[];
  environmentFamily: string;
  supportTerm: string;
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
