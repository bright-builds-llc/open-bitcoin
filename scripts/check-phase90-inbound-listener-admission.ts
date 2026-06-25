#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE90_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-9-inbound-listener-admission-policy";
const AUDIT_KEY = "v1_9_inbound_listener_admission_policy";
const PHASE88_CHECKER_COMMAND =
  "bun run scripts/check-phase88-deterministic-claim-guardrails.ts";
const PHASE90_TEST_COMMAND =
  "bun test scripts/check-phase90-inbound-listener-admission.test.ts";
const PHASE90_CHECKER_COMMAND =
  "bun run scripts/check-phase90-inbound-listener-admission.ts";
const PHASE90_REQUIREMENTS = [
  "INB-01",
  "INB-02",
  "INB-03",
  "INB-04",
  "INB-05",
] as const;
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
const REQUIRED_EVIDENCE_LABELS = [
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
const REQUIRED_CATALOG_ANCHORS = [
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/test/functional/p2p_handshake.py",
] as const;
const REQUIRED_BREADCRUMB_MAPPINGS = [
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
const FORBIDDEN_VERIFY_STRINGS = [
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
const PUBLIC_DEFAULT_CLAIMS = [
  "supports public inbound by default",
  "public inbound by default",
  "public inbound serving by default",
  "public listener defaults are supported",
] as const;
const PRODUCTION_READY_CLAIMS = [
  "Open Bitcoin is production full-node ready.",
  "Open Bitcoin has production full-node readiness.",
  "v1.9 proves production full-node readiness.",
  "production full-node readiness is supported",
] as const;
const ALLOWED_SCOPE_TERMS = [
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
const COMMAND_PREFIXES = Array.from(
  new Set(REQUIRED_UAT_COMMANDS.map((command) => command.required[0])),
);

type AuditEntry = {
  evidence?: unknown;
  path?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type BreadcrumbIndex = {
  groups?: unknown;
};

type BreadcrumbGroup = {
  breadcrumbs?: unknown;
  files?: unknown;
  label?: unknown;
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

export function checkPhase90InboundListenerAdmission(
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
  const expected = JSON.stringify(PHASE90_REQUIREMENTS);
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
    requireNormalizedContains(corpus, label, "inbound evidence label", failures);
  }
}

function verifyParityDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  const p2pText = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklistText = texts.get("docs/parity/checklist.md") ?? "";
  requireContains(p2pText, SURFACE_ID, "docs/parity/catalog/p2p.md", failures);
  requireContains(checklistText, SURFACE_ID, "docs/parity/checklist.md", failures);
  for (const requirement of PHASE90_REQUIREMENTS) {
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
        failures.push(`source breadcrumb mapping missing required Phase 90 file: ${file}`);
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
  for (const command of [PHASE90_TEST_COMMAND, PHASE90_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "test Phase 90 inbound listener admission checker" ${PHASE90_TEST_COMMAND}`,
    "verifier-order",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 90 inbound listener admission" ${PHASE90_CHECKER_COMMAND}`,
    "verifier-order",
    failures,
  );
  verifyVerifierOrder(executableText, failures);
  verifyVerifierBoundary(executableText, failures);
}

function verifyVerifierOrder(executableText: string, failures: string[]): void {
  const phase88Index = executableText.indexOf(PHASE88_CHECKER_COMMAND);
  const phase90TestIndex = executableText.indexOf(PHASE90_TEST_COMMAND);
  const phase90CheckerIndex = executableText.indexOf(PHASE90_CHECKER_COMMAND);
  const pureCoreIndex = executableText.indexOf("bash scripts/check-pure-core-deps.sh");
  const orderValid =
    phase88Index !== -1 &&
    phase90TestIndex > phase88Index &&
    phase90CheckerIndex > phase90TestIndex &&
    pureCoreIndex > phase90CheckerIndex;

  if (!orderValid) {
    failures.push(
      "verifier-order requires executed Phase 90 test and checker after Phase 88 and before pure-core checks",
    );
  }
}

function verifyVerifierBoundary(executableText: string, failures: string[]): void {
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 90 command text: ${forbidden}`);
    }
  }
}

function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "docs/parity/source-breadcrumbs.json" || file === "scripts/verify.sh") {
      continue;
    }

    for (const unit of contextUnits(text)) {
      verifyNoPublicDefaultClaim(file, unit, failures);
      verifyNoProductionReadinessClaim(file, unit, failures);
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

function verifyNoPublicDefaultClaim(
  file: string,
  unit: string,
  failures: string[],
): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  for (const claim of PUBLIC_DEFAULT_CLAIMS) {
    if (normalizedLower(unit).includes(claim.toLowerCase())) {
      failures.push(`Phase 90 no-claim boundary public inbound default claim in ${file}: ${unit}`);
    }
  }
}

function verifyNoProductionReadinessClaim(
  file: string,
  unit: string,
  failures: string[],
): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  for (const claim of PRODUCTION_READY_CLAIMS) {
    if (normalizeWhitespace(unit).includes(normalizeWhitespace(claim))) {
      failures.push(`Phase 90 no-claim boundary production full-node readiness claim in ${file}: ${unit}`);
    }
  }
}

function isScopedAllowance(unit: string): boolean {
  const lower = normalizedLower(unit);
  return ALLOWED_SCOPE_TERMS.some((term) => lower.includes(term.toLowerCase()));
}

if (import.meta.main) {
  const failures = checkPhase90InboundListenerAdmission();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 90 inbound listener admission evidence");
  }
}
