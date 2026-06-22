#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE86_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-8-service-operation-expectations";
const AUDIT_KEY = "v1_8_service_operation_expectations";
const PHASE86_REQUIREMENTS = ["SVC-01", "SVC-02"] as const;
const SERVICE_DOC_PATH = "docs/parity/service-operation-expectations.md";
const TABLE_HEADER =
  "Service surface | Support term | What evidence proves | Cargo command evidence | Bazel command evidence | Default verification | Opt-in UAT | Residual risk | Next gate";
const PHASE85_CHECKER_COMMAND =
  "bun run scripts/check-phase85-operator-runbooks.ts";
const PHASE86_TEST_COMMAND =
  "bun test scripts/check-phase86-service-operation-expectations.test.ts";
const PHASE86_CHECKER_COMMAND =
  "bun run scripts/check-phase86-service-operation-expectations.ts";
const TARGET_FILES = [
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
const HUMAN_POINTER_FILES = TARGET_FILES.filter(
  (file) =>
    file !== SERVICE_DOC_PATH &&
    file !== "docs/parity/index.json" &&
    file !== "scripts/verify.sh",
);
const REQUIRED_EVIDENCE = [
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
const REQUIRED_HEADINGS = [
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
const SUPPORT_TERMS = [
  "supported",
  "preview",
  "opt-in UAT",
  "unsupported",
  "deferred",
] as const;
const SERVICE_SURFACES = [
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
const SERVICE_COMMANDS = [
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
const FIELD_EVIDENCE_TERMS = [
  "service file existence",
  "daemon startup",
  "elapsed time",
  "raw log tail",
  "public peer reachability",
  "support bundle path",
  "expected fields and unavailable reasons",
  "Unavailable: <reason>",
] as const;
const SERVICE_FIELDS = [
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
const LIFECYCLE_LABELS = [
  "unmanaged",
  "installed-stopped",
  "running",
  "failed",
  "disabled",
  "unavailable-manager",
] as const;
const RESTART_RESUME_FIELDS = [
  "same_datadir",
  "prior_shutdown",
  "durable_progress",
  "stale_inflight",
  "recovery_category",
  "next_action",
] as const;
const PROOF_SIGNALS = [
  "service file existence",
  "daemon startup",
  "elapsed time",
  "raw log tail",
  "public peer reachability",
  "support bundle path",
] as const;
const SENSITIVE_EVIDENCE_TERMS = [
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
const FORBIDDEN_DOC_PERMISSION_STRINGS = [
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
const FORBIDDEN_VERIFY_STRINGS = [
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

type AuditEntry = {
  evidence?: unknown;
  path?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ParityIndex = {
  audit?: Record<string, unknown>;
  checklist?: {
    surfaces?: unknown;
  };
  surfaces?: unknown;
};

type ParitySurface = {
  name?: unknown;
  status?: unknown;
};

export function checkPhase86ServiceOperationExpectations(
  maybeRepoRoot = process.env.OPEN_BITCOIN_PHASE86_REPO_ROOT,
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyServiceDoc(texts.get(SERVICE_DOC_PATH) ?? "", failures);
  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanRoots(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function normalizeEvidenceText(text: string): string {
  return text
    .replaceAll("&lt;", "<")
    .replaceAll("&gt;", ">")
    .replace(/\s+/g, " ")
    .trim();
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!normalizeEvidenceText(text).includes(normalizeEvidenceText(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
  }
}

function requireNotNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (normalizeEvidenceText(text).includes(normalizeEvidenceText(needle))) {
    failures.push(`${label} must not duplicate required text: ${needle}`);
  }
}

function requireArrayIncludes(
  value: unknown,
  label: string,
  required: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} parity root must be an array`);
    return;
  }
  if (!value.includes(required)) {
    failures.push(`${label} parity root missing required value: ${required}`);
  }
}

function requireExactRequirements(value: unknown, label: string, failures: string[]): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} parity root requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const expected = JSON.stringify(PHASE86_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`${label} parity root requirements mismatch: expected ${expected}, got ${actual}`);
  }
}

function sectionBetween(text: string, heading: string): string {
  const startIndex = text.indexOf(heading);
  if (startIndex === -1) {
    return "";
  }

  const nextHeadingIndex = text.indexOf("\n## ", startIndex + heading.length);
  if (nextHeadingIndex === -1) {
    return text.slice(startIndex);
  }
  return text.slice(startIndex, nextHeadingIndex);
}

function verifyServiceDoc(text: string, failures: string[]): void {
  for (const heading of REQUIRED_HEADINGS) {
    requireContains(text, heading, "service expectations headings", failures);
  }
  requireContains(text, SURFACE_ID, "service expectations surface", failures);
  for (const term of SUPPORT_TERMS) {
    requireContains(text, term, "support terms", failures);
  }
  requireNormalizedContains(
    text,
    "generated launchd/systemd definitions supervise `open-bitcoind`, not the `open-bitcoin` operator wrapper.",
    "service expectations scope",
    failures,
  );
  requireContains(
    text,
    "`service preview` is always side-effect-free.",
    "service expectations scope",
    failures,
  );
  requireNormalizedContains(
    text,
    "`service install` and `service uninstall` are previews unless `--apply` is supplied.",
    "service expectations scope",
    failures,
  );

  verifyClassification(text, failures);
  verifyCommandEvidence(text, failures);
  verifyFieldEvidence(text, failures);
  verifyRestartResume(text, failures);
  verifyDefaultBoundary(text, failures);
  verifySensitiveEvidence(text, failures);
  verifyNoProofPromotion(text, failures);
}

function verifyClassification(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Service Surface Classification");
  if (section === "") {
    failures.push("service classification missing required section");
    return;
  }

  requireNormalizedContains(section, TABLE_HEADER, "service classification", failures);
  for (const surface of SERVICE_SURFACES) {
    requireContains(section, surface, "service classification", failures);
  }
}

function verifyCommandEvidence(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Repo-Local Command Evidence");
  if (section === "") {
    failures.push("command evidence missing required section");
    return;
  }

  for (const command of SERVICE_COMMANDS) {
    requireNormalizedContains(section, command, "command evidence", failures);
  }
}

function verifyFieldEvidence(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Field-Based Evidence Rules");
  if (section === "") {
    failures.push("field-based evidence missing required section");
    return;
  }

  for (const term of FIELD_EVIDENCE_TERMS) {
    requireNormalizedContains(section, term, "field-based evidence", failures);
  }
  for (const field of SERVICE_FIELDS) {
    requireContains(section, field, "field-based evidence", failures);
  }
  for (const label of LIFECYCLE_LABELS) {
    requireContains(section, label, "lifecycle labels", failures);
  }
}

function verifyRestartResume(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Restart Resume Evidence");
  if (section === "") {
    failures.push("restart/resume evidence missing required section");
    return;
  }

  for (const field of RESTART_RESUME_FIELDS) {
    requireContains(section, field, "restart/resume evidence", failures);
  }
  requireContains(section, "same selected datadir", "restart/resume evidence", failures);
  requireContains(section, "do not prove durable resume", "restart/resume evidence", failures);
}

function verifyDefaultBoundary(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Default Verification And Opt-In UAT Boundaries");
  if (section === "") {
    failures.push("default verifier boundary missing required section");
    return;
  }

  requireContains(
    section,
    "Default bash scripts/verify.sh remains deterministic, public-network-free, real-service-manager-free, and multi-day-free.",
    "default verifier boundary",
    failures,
  );
  for (const phrase of [
    "public-network live smoke",
    "real service-manager commands",
    "long wall-clock sleeps",
    "package-manager service commands",
    "Windows service workflows",
    "automatic support-bundle upload",
    "production service ownership checks",
    "broad production-node readiness checks",
    "opt-in UAT",
  ]) {
    requireNormalizedContains(section, phrase, "default verifier boundary", failures);
  }
}

function verifySensitiveEvidence(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Sensitive Evidence Boundaries");
  if (section === "") {
    failures.push("sensitive evidence missing required section");
    return;
  }

  for (const term of SENSITIVE_EVIDENCE_TERMS) {
    requireContains(section, term, "sensitive evidence", failures);
  }
}

function verifyNoProofPromotion(text: string, failures: string[]): void {
  const lowerText = normalizeEvidenceText(text).toLowerCase();
  for (const signal of PROOF_SIGNALS) {
    for (const proofPattern of [
      `${signal} as proof`,
      `${signal} is proof`,
      `${signal} proves`,
      `${signal} alone proves`,
    ]) {
      if (lowerText.includes(proofPattern)) {
        failures.push(`field-based evidence must not treat ${signal} as proof`);
      }
    }
  }

  for (const forbidden of FORBIDDEN_DOC_PERMISSION_STRINGS) {
    if (lowerText.includes(forbidden.toLowerCase())) {
      failures.push(`sensitive evidence or default boundary must not permit: ${forbidden}`);
    }
  }
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`parity root index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
  verifyAuditEntry(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("parity root surfaces must be an array");
    return;
  }

  const surface = parsed.surfaces.find((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`parity root surfaces missing done ${SURFACE_ID}`);
  }
}

function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const checklistSurfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(checklistSurfaces)) {
    failures.push("parity root checklist.surfaces must be an array");
    return;
  }

  const surface = checklistSurfaces.find((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`parity root checklist missing done ${SURFACE_ID}`);
  }
  requireExactRequirements(surface?.requirements, `${SURFACE_ID}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(surface?.evidence, `${SURFACE_ID}.evidence`, evidence, failures);
  }
}

function verifyAuditEntry(parsed: ParityIndex, failures: string[]): void {
  const auditEntry = parsed.audit?.[AUDIT_KEY] as AuditEntry | undefined;
  if (auditEntry?.path !== "service-operation-expectations.md" || auditEntry.status !== "done") {
    failures.push(`parity root audit.${AUDIT_KEY} is missing or incomplete`);
    return;
  }
  requireExactRequirements(auditEntry.requirements, `audit.${AUDIT_KEY}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(auditEntry.evidence, `audit.${AUDIT_KEY}.evidence`, evidence, failures);
  }
}

function verifyHumanRoots(texts: Map<string, string>, failures: string[]): void {
  for (const file of HUMAN_POINTER_FILES) {
    requireContains(texts.get(file) ?? "", "service-operation-expectations.md", file, failures);
    requireNotNormalizedContains(texts.get(file) ?? "", TABLE_HEADER, file, failures);
  }

  requireContains(
    texts.get("README.md") ?? "",
    "docs/parity/service-operation-expectations.md",
    "README.md",
    failures,
  );
  requireContains(
    texts.get("docs/operator/runtime-guide.md") ?? "",
    "../parity/service-operation-expectations.md",
    "docs/operator/runtime-guide.md",
    failures,
  );

  const releaseReadiness = texts.get("docs/parity/release-readiness.md") ?? "";
  for (const phrase of [
    SURFACE_ID,
    "SVC-01",
    "SVC-02",
    "source-built daemon operation",
    "launchd/systemd preview",
    "opt-in real service lifecycle UAT",
    "restart/resume fields",
    "repo-local Cargo/Bazel commands",
    "production-service non-claims",
  ]) {
    requireContains(releaseReadiness, phrase, "docs/parity/release-readiness.md", failures);
  }

  const catalog = texts.get("docs/parity/catalog/operator-runtime-release-hardening.md") ?? "";
  for (const phrase of [
    "Phase 86 service operation expectations",
    SURFACE_ID,
    "SVC-01",
    "SVC-02",
    "source-built daemon operation",
    "launchd/systemd preview",
    "opt-in real service lifecycle UAT",
    "restart/resume fields",
    "repo-local Cargo/Bazel commands",
    "production-service non-claims",
  ]) {
    requireContains(catalog, phrase, "docs/parity/catalog/operator-runtime-release-hardening.md", failures);
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [PHASE86_TEST_COMMAND, PHASE86_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  for (const command of [PHASE86_TEST_COMMAND, PHASE86_CHECKER_COMMAND]) {
    requireContains(executableText, command, "verifier-order", failures);
  }

  const phase85Index = executableText.indexOf(PHASE85_CHECKER_COMMAND);
  const phase86TestIndex = executableText.indexOf(PHASE86_TEST_COMMAND);
  const phase86CheckerIndex = executableText.indexOf(PHASE86_CHECKER_COMMAND);
  const orderValid =
    phase85Index !== -1 &&
    phase86TestIndex > phase85Index &&
    phase86CheckerIndex > phase86TestIndex;

  if (!orderValid) {
    failures.push("verifier-order requires executed Phase 86 test and checker after Phase 85 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 86 default command text: ${forbidden}`);
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase86ServiceOperationExpectations();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 86 service operation expectations");
  }
}
