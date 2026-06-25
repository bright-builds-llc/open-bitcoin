#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE91_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-9-peer-permissions-connection-classes";
const AUDIT_KEY = "v1_9_peer_permissions_connection_classes";
const PHASE90_CHECKER_COMMAND =
  "bun run scripts/check-phase90-inbound-listener-admission.ts";
const PHASE91_TEST_COMMAND =
  "bun test scripts/check-phase91-peer-permissions.test.ts";
const PHASE91_CHECKER_COMMAND = "bun run scripts/check-phase91-peer-permissions.ts";
const REQUIRED_PERMISSION_TOKENS =
  "in,noban,forceinbound,download,addr,relay,forcerelay,mempool,bloomfilter,blockfilters";
const PHASE91_REQUIREMENTS = ["PERM-01", "PERM-02", "PERM-03", "PERM-04"] as const;
const TARGET_FILES = [
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
const REQUIRED_EVIDENCE = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/source-breadcrumbs.json",
] as const;
const REQUIRED_UAT_COMMANDS = [
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
const REQUIRED_EVIDENCE_LABELS = [
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
const REQUIRED_METRICS = [
  "InboundPermissionedAdmitCount",
  "InboundProtectedAdmitCount",
  "InboundInactivePermissionEffectCount",
  "InboundPermissionValidationFailureCount",
] as const;
const REQUIRED_CATALOG_ANCHORS = [
  "packages/bitcoin-knots/src/net_permissions.h",
  "packages/bitcoin-knots/src/net_permissions.cpp",
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/test/functional/p2p_permissions.py",
] as const;
const REQUIRED_BREADCRUMB_MAPPINGS = [
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
const FORBIDDEN_VERIFY_STRINGS = [
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
const FORBIDDEN_UNSCOPED_CLAIMS = [
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
const FORBIDDEN_SUPPORT_RAW_DETAILS = [
  "operator_loopback",
  "peer_id=",
  "127.0.0.1:",
  "rpc_password",
  "rpcpassword",
  "cookie=",
  REQUIRED_PERMISSION_TOKENS,
] as const;
const ALLOWED_SCOPE_TERMS = [
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
const COMMAND_PREFIXES = Array.from(
  new Set(REQUIRED_UAT_COMMANDS.map((command) => command.required[0])),
);

type AuditEntry = {
  evidence?: unknown;
  path?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: {
    sources?: unknown;
    tests?: unknown;
  };
};

type BreadcrumbGroup = {
  breadcrumbs?: unknown;
  files?: unknown;
  label?: unknown;
};

type BreadcrumbIndex = {
  groups?: unknown;
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

type TargetFile = (typeof TARGET_FILES)[number];

export function checkPhase91PeerPermissions(
  maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV],
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanDocs(texts, failures);
  verifySourceBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);
  verifySupportRedactionBoundary(texts, failures);

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

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function normalizeShellCommand(text: string): string {
  return normalizeWhitespace(text.replace(/\\\s*/g, " "));
}

function normalizedLower(text: string): string {
  return normalizeWhitespace(text).toLowerCase();
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
  if (!normalizedLower(text).includes(normalizedLower(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
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
  const expected = JSON.stringify(PHASE91_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`${label} parity root requirements mismatch: expected ${expected}, got ${actual}`);
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
  if (auditEntry?.path !== "catalog/p2p.md" || auditEntry.status !== "done") {
    failures.push(`parity root audit.${AUDIT_KEY} is missing or incomplete`);
    return;
  }
  requireExactRequirements(auditEntry.requirements, `audit.${AUDIT_KEY}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(auditEntry.evidence, `audit.${AUDIT_KEY}.evidence`, evidence, failures);
  }
  for (const anchor of [
    "packages/bitcoin-knots/src/net_permissions.h",
    "packages/bitcoin-knots/src/net_permissions.cpp",
    "packages/bitcoin-knots/src/net.cpp",
    "packages/bitcoin-knots/src/net_processing.cpp",
  ]) {
    requireArrayIncludes(
      auditEntry.upstream?.sources,
      `audit.${AUDIT_KEY}.upstream.sources`,
      anchor,
      failures,
    );
  }
  requireArrayIncludes(
    auditEntry.upstream?.tests,
    `audit.${AUDIT_KEY}.upstream.tests`,
    "packages/bitcoin-knots/test/functional/p2p_permissions.py",
    failures,
  );
}

function verifyHumanDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  verifyRuntimeGuideCommands(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  verifyEvidenceLabels(texts, failures);
  verifyParityDocs(texts, failures);
}

function verifyRuntimeGuideCommands(text: string, failures: string[]): void {
  const commandUnits = shellCommandUnits(text);
  for (const command of REQUIRED_UAT_COMMANDS) {
    const commandFound = commandUnits.some((unit) =>
      command.required.every((required) => unit.includes(normalizeShellCommand(required))),
    );
    if (!commandFound) {
      failures.push(`UAT command missing ${command.label}: ${command.required.join(" ")}`);
    }
  }
}

function shellCommandUnits(text: string): string[] {
  const units: string[] = [];
  let currentLines: string[] = [];

  for (const rawLine of text.replaceAll("\r\n", "\n").split("\n")) {
    const line = rawLine.trim();
    const lineStartsCommand = COMMAND_PREFIXES.some((prefix) => line.startsWith(prefix));

    if (lineStartsCommand) {
      pushCurrentShellCommandUnit(currentLines, units);
      currentLines = [line];
      continue;
    }

    if (currentLines.length === 0) {
      continue;
    }

    if (line.length === 0 || line.startsWith("```")) {
      pushCurrentShellCommandUnit(currentLines, units);
      currentLines = [];
      continue;
    }

    currentLines.push(line);
  }

  pushCurrentShellCommandUnit(currentLines, units);
  return units;
}

function pushCurrentShellCommandUnit(currentLines: string[], units: string[]): void {
  if (currentLines.length === 0) {
    return;
  }

  units.push(normalizeShellCommand(currentLines.join("\n")));
}

function verifyEvidenceLabels(texts: Map<TargetFile, string>, failures: string[]): void {
  const corpus = [
    texts.get("docs/operator/runtime-guide.md") ?? "",
    texts.get("docs/architecture/config-precedence.md") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
  ].join("\n");

  for (const label of REQUIRED_EVIDENCE_LABELS) {
    requireNormalizedContains(corpus, label, "Phase 91 evidence label", failures);
  }
  for (const metric of REQUIRED_METRICS) {
    requireContains(corpus, metric, "Phase 91 metric evidence", failures);
  }
  requireContains(corpus, REQUIRED_PERMISSION_TOKENS, "Phase 91 token evidence", failures);
}

function verifyParityDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  const p2pText = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklistText = texts.get("docs/parity/checklist.md") ?? "";
  requireContains(p2pText, SURFACE_ID, "docs/parity/catalog/p2p.md", failures);
  requireContains(checklistText, SURFACE_ID, "docs/parity/checklist.md", failures);
  for (const requirement of PHASE91_REQUIREMENTS) {
    requireContains(p2pText, requirement, "docs/parity/catalog/p2p.md", failures);
    requireContains(checklistText, requirement, "docs/parity/checklist.md", failures);
  }
  for (const anchor of REQUIRED_CATALOG_ANCHORS) {
    requireContains(p2pText, anchor, "docs/parity/catalog/p2p.md", failures);
  }
}

function verifySourceBreadcrumbs(text: string, failures: string[]): void {
  for (const mapping of REQUIRED_BREADCRUMB_MAPPINGS) {
    for (const file of mapping.files) {
      if (!text.includes(file)) {
        failures.push(`source breadcrumb mapping missing required Phase 91 file: ${file}`);
      }
    }
  }

  let parsed: BreadcrumbIndex;
  try {
    parsed = JSON.parse(text) as BreadcrumbIndex;
  } catch (error) {
    failures.push(`source breadcrumb JSON parse failed: ${String(error)}`);
    return;
  }

  if (!Array.isArray(parsed.groups)) {
    failures.push("source breadcrumb groups must be an array");
    return;
  }

  for (const mapping of REQUIRED_BREADCRUMB_MAPPINGS) {
    verifyBreadcrumbMapping(parsed.groups, mapping, failures);
  }
}

function verifyBreadcrumbMapping(
  groups: unknown[],
  mapping: (typeof REQUIRED_BREADCRUMB_MAPPINGS)[number],
  failures: string[],
): void {
  for (const file of mapping.files) {
    const maybeGroup = groups.find((entry) => {
      const group = entry as BreadcrumbGroup;
      return group.label === mapping.label && Array.isArray(group.files) && group.files.includes(file);
    }) as BreadcrumbGroup | undefined;
    if (maybeGroup === undefined) {
      failures.push(`source breadcrumb mapping missing ${mapping.label}: ${file}`);
      continue;
    }

    const actual = JSON.stringify(maybeGroup.breadcrumbs ?? []);
    const expected = JSON.stringify(mapping.breadcrumbs);
    if (actual !== expected) {
      failures.push(`source breadcrumb mapping mismatch for ${file}: expected ${expected}, got ${actual}`);
    }
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [PHASE91_TEST_COMMAND, PHASE91_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "test Phase 91 peer permissions checker" ${PHASE91_TEST_COMMAND}`,
    "verifier-order",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 91 peer permissions" ${PHASE91_CHECKER_COMMAND}`,
    "verifier-order",
    failures,
  );
  verifyVerifierOrder(executableText, failures);
  verifyVerifierBoundary(executableText, failures);
}

function verifyVerifierOrder(executableText: string, failures: string[]): void {
  const phase90Index = executableText.indexOf(PHASE90_CHECKER_COMMAND);
  const phase91TestIndex = executableText.indexOf(PHASE91_TEST_COMMAND);
  const phase91CheckerIndex = executableText.indexOf(PHASE91_CHECKER_COMMAND);
  const pureCoreIndex = executableText.indexOf("bash scripts/check-pure-core-deps.sh");
  const orderValid =
    phase90Index !== -1 &&
    phase91TestIndex > phase90Index &&
    phase91CheckerIndex > phase91TestIndex &&
    pureCoreIndex > phase91CheckerIndex;

  if (!orderValid) {
    failures.push(
      "verifier-order requires executed Phase 91 test and checker after Phase 90 and before pure-core checks",
    );
  }
}

function verifyVerifierBoundary(executableText: string, failures: string[]): void {
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 91 command text: ${forbidden}`);
    }
  }
}

function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "docs/parity/source-breadcrumbs.json" || file === "scripts/verify.sh") {
      continue;
    }

    for (const unit of contextUnits(text)) {
      verifyNoForbiddenClaim(file, unit, failures);
    }
  }
}

function verifySupportRedactionBoundary(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "docs/parity/source-breadcrumbs.json" || file === "scripts/verify.sh") {
      continue;
    }

    for (const unit of contextUnits(text)) {
      const lower = normalizedLower(unit);
      const supportContext =
        lower.includes("support bundle") ||
        (lower.includes("support") && lower.includes("evidence"));
      if (!supportContext || isScopedAllowance(unit)) {
        continue;
      }

      for (const rawDetail of FORBIDDEN_SUPPORT_RAW_DETAILS) {
        if (unit.includes(rawDetail)) {
          failures.push(`Phase 91 support redaction boundary raw detail in ${file}: ${unit}`);
        }
      }
    }
  }
}

function contextUnits(text: string): string[] {
  const units: string[] = [];
  for (const block of text.replaceAll("\r\n", "\n").split(/\n\s*\n/)) {
    const lines = block
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
    if (lines.length === 0) {
      continue;
    }

    const tableRows = lines.filter(
      (line) => line.startsWith("|") && !/^\|\s*-/.test(line),
    );
    if (tableRows.length > 0) {
      units.push(...tableRows.map(normalizeWhitespace));
      const prose = lines.filter((line) => !line.startsWith("|")).join(" ");
      units.push(...sentenceUnits(prose));
      continue;
    }

    units.push(...sentenceUnits(lines.join(" ")));
  }

  return units.map(normalizeWhitespace).filter((unit) => unit.length > 0);
}

function sentenceUnits(text: string): string[] {
  const normalized = normalizeWhitespace(text);
  if (normalized.length === 0) {
    return [];
  }

  return normalized.split(/(?<=[.!?])\s+(?=[A-Z`])/);
}

function verifyNoForbiddenClaim(file: string, unit: string, failures: string[]): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  for (const claim of FORBIDDEN_UNSCOPED_CLAIMS) {
    if (normalizedLower(unit).includes(claim.toLowerCase())) {
      failures.push(`Phase 91 no-claim boundary forbidden claim in ${file}: ${unit}`);
    }
  }
}

function isScopedAllowance(unit: string): boolean {
  const lower = normalizedLower(unit);
  return ALLOWED_SCOPE_TERMS.some((term) => lower.includes(term.toLowerCase()));
}

if (import.meta.main) {
  const failures = checkPhase91PeerPermissions();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 91 peer permissions evidence");
  }
}
