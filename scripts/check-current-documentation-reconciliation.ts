#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const FILES = {
  architecture: ".planning/ARCHITECTURE.md",
  catalog: "docs/parity/catalog/rpc-cli-config.md",
  conventions: ".planning/CONVENTIONS.md",
  deviations: "docs/parity/deviations-and-unknowns.md",
  methodSource: "packages/open-bitcoin-rpc/src/method.rs",
  productionBoundary: "docs/parity/production-claim-boundary.md",
  readme: "README.md",
  releaseReadiness: "docs/parity/release-readiness.md",
  supportMatrix: "docs/parity/support-matrix.md",
  verifier: "scripts/verify.sh",
} as const;
const EXTENSION_METHODS = new Set([
  "openbitcoinnetworkstatus",
  "openbitcoinsyncstatus",
  "openbitcoinsyncpause",
  "openbitcoinsyncresume",
  "buildtransaction",
  "buildandsigntransaction",
]);
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
const RECONCILIATION_TEST =
  "bun test scripts/check-current-documentation-reconciliation.test.ts";
const RECONCILIATION_CHECK =
  "bun run scripts/check-current-documentation-reconciliation.ts";
const VISIBLE_SEQUENCE = [
  PHASE117_TEST,
  PHASE117_CHECK,
  RECONCILIATION_TEST,
  RECONCILIATION_CHECK,
].join("\n");
const EXECUTABLE_SEQUENCE = [
  `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`,
  `run_step "check Phase 117 parity UAT release boundary" ${PHASE117_CHECK}`,
  `run_step "test current documentation reconciliation checker" ${RECONCILIATION_TEST}`,
  `run_step "check current documentation reconciliation" ${RECONCILIATION_CHECK}`,
].join("\n");

type CorpusKey = keyof typeof FILES;
type Corpus = Map<CorpusKey, string>;

export function checkCurrentDocumentationReconciliation(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(maybeRepoRoot ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const corpus = loadCorpus(repoRoot, failures);

  verifyArchivedProjectState(corpus, failures);
  verifyCurrentRuntimeBoundaries(corpus, failures);
  verifyRelayClassifications(corpus, failures);
  verifyCatalogMethodSet(corpus, failures);
  verifyWalletCatalog(corpus, failures);
  verifyVerifierWiring(corpus.get("verifier") ?? "", failures);

  return failures.sort();
}

function loadCorpus(repoRoot: string, failures: string[]): Corpus {
  const corpus: Corpus = new Map();
  for (const [key, relativeFile] of Object.entries(FILES) as [CorpusKey, string][]) {
    try {
      corpus.set(key, readFileSync(path.join(repoRoot, relativeFile), "utf8"));
    } catch {
      failures.push(`missing reconciliation input: ${relativeFile}`);
      corpus.set(key, "");
    }
  }
  return corpus;
}

function verifyArchivedProjectState(corpus: Corpus, failures: string[]): void {
  const readmeCurrent = sectionBefore(corpus.get("readme") ?? "", "## Parity At A Glance");
  verifyArchivedStateText("README archived milestone state", readmeCurrent, failures);
  requireAll(
    "README final audit counts",
    normalized(readmeCurrent),
    ["39/39 requirements", "20/20 phases", "13/13 integration links", "11/11 end-to-end flows"],
    failures,
  );

  for (const [key, label] of [
    ["architecture", "architecture archived milestone state"],
    ["conventions", "conventions archived milestone state"],
  ] as const) {
    const current = sectionBefore(corpus.get(key) ?? "", "## Architectural Shape");
    const fallbackCurrent = sectionBefore(corpus.get(key) ?? "", "## Parity And Evidence");
    verifyArchivedStateText(label, current || fallbackCurrent, failures);
  }

  const releaseSection = markdownSection(
    corpus.get("releaseReadiness") ?? "",
    "## v2.1 Block Serving and Compact Block Relay Boundary",
  );
  verifyArchivedStateText(
    "release-readiness archived milestone state",
    releaseSection,
    failures,
  );
  requireAll(
    "release-readiness final audit evidence",
    normalized(releaseSection),
    [
      "39/39 requirements",
      "20/20 phases",
      "13/13 integration links",
      "11/11 end-to-end flows",
      "v2.1-milestone-audit.md",
    ],
    failures,
  );
}

function verifyArchivedStateText(label: string, text: string, failures: string[]): void {
  const value = normalized(text);
  const required = [
    "v2.1 shipped and was archived on 2026-07-22",
    "/gsd-new-milestone",
  ];
  const forbidden = [
    "/gsd-complete-milestone v2.1",
    "archive-ready",
    "archive ready",
    "pending completion",
    "pending the completion",
  ];
  if (required.some((needle) => !value.includes(needle))) {
    failures.push(`${label}: missing shipped/archive date or next-milestone route`);
  }
  if (forbidden.some((needle) => value.includes(needle))) {
    failures.push(`${label}: contains pre-archive completion language`);
  }
}

function verifyCurrentRuntimeBoundaries(corpus: Corpus, failures: string[]): void {
  const architecture = normalized(
    markdownSection(corpus.get("architecture") ?? "", "## Sync Boundary"),
  );
  requireAll(
    "architecture current sync boundary",
    architecture,
    [
      "explicit opt-in full-sync path",
      "bounded, explicit, default-off inbound serving",
      "v2.0 transaction relay",
      "v2.1 block serving",
      "v2.1 compact-block relay",
      "not public defaults",
    ],
    failures,
  );
  rejectAny(
    "architecture current sync boundary",
    architecture,
    ["later v1.2", "pre-v1.2"],
    failures,
  );

  const conventions = normalized(
    markdownSection(corpus.get("conventions") ?? "", "## Operator Surface"),
  );
  requireAll(
    "conventions current operator boundary",
    conventions,
    [
      "full-sync",
      "bounded inbound",
      "transaction-relay",
      "block-serving",
      "compact-block-relay",
      "must not imply public defaults",
    ],
    failures,
  );
  rejectAny(
    "conventions current operator boundary",
    conventions,
    ["later v1.2", "pre-v1.2"],
    failures,
  );
}

function verifyRelayClassifications(corpus: Corpus, failures: string[]): void {
  verifyTableTerm(
    corpus.get("supportMatrix") ?? "",
    "transaction relay",
    "preview",
    "support-matrix transaction relay must be preview",
    failures,
  );

  verifyRelayBoundaryTable(
    corpus.get("productionBoundary") ?? "",
    "production boundary",
    "Open Bitcoin provides bounded, explicit, default-off v2.0 transaction relay and mempool participation.",
    failures,
  );
  verifyRelayBoundaryTable(
    corpus.get("deviations") ?? "",
    "deviation register",
    "bounded, explicit, default-off v2.0 transaction relay",
    failures,
  );
}

function verifyRelayBoundaryTable(
  text: string,
  label: string,
  boundedRow: string,
  failures: string[],
): void {
  verifyTableTerm(
    text,
    boundedRow,
    "preview",
    `${label} bounded v2.0 relay must be preview`,
    failures,
  );
  verifyTableTerm(
    text,
    "public/default or production transaction relay beyond the bounded v2.0 path",
    "deferred",
    `${label} broader relay must be deferred`,
    failures,
  );
}

function verifyCatalogMethodSet(corpus: Corpus, failures: string[]): void {
  const rustMethods = extractSupportedMethodNames(corpus.get("methodSource") ?? "");
  const catalog = corpus.get("catalog") ?? "";
  const baselineMethods = extractBulletCodeValues(
    catalog,
    "supported baseline-backed RPC methods:",
  );
  const extensionMethods = extractBulletCodeValues(
    catalog,
    "supported Open Bitcoin extension RPC methods:",
  );
  const catalogMethods = new Set([...baselineMethods, ...extensionMethods]);

  if (rustMethods.size !== 20) {
    failures.push(`SupportedMethod enum must expose exactly 20 serde names (found ${rustMethods.size})`);
  }
  reportSetDifference(rustMethods, catalogMethods, "missing", failures);
  reportSetDifference(catalogMethods, rustMethods, "extra", failures);

  const expectedBaseline = new Set([...rustMethods].filter((name) => !EXTENSION_METHODS.has(name)));
  if (!setsEqual(baselineMethods, expectedBaseline)) {
    failures.push("catalog baseline-backed supported-method list has incorrect grouping");
  }
  if (!setsEqual(extensionMethods, EXTENSION_METHODS)) {
    failures.push("catalog extension supported-method list has incorrect grouping");
  }
}

function verifyWalletCatalog(corpus: Corpus, failures: string[]): void {
  const catalog = corpus.get("catalog") ?? "";
  const supported = normalized(markdownSection(catalog, "## Supported behaviors"));
  requireAll(
    "catalog implemented wallet routing subset",
    supported,
    ["`sendtoaddress` is baseline-backed", "-rpcwallet=<name>", "/wallet/<name>"],
    failures,
  );

  const deferred = normalized(markdownSection(catalog, "## Deferred surfaces"));
  requireAll(
    "catalog narrow wallet deferrals",
    deferred,
    ["richer `send` rpc semantics", "loadwallet", "unloadwallet", "listwallets"],
    failures,
  );
  if (/deferred\s+`?sendtoaddress`?/.test(deferred)) {
    failures.push("catalog must not defer sendtoaddress wholesale");
  }
  if (/deferred\s+`?(?:rpcwallet|-rpcwallet)`?/.test(deferred)) {
    failures.push("catalog must not defer -rpcwallet wholesale");
  }
}

function verifyVerifierWiring(verifyText: string, failures: string[]): void {
  const visible = between(
    verifyText,
    ": <<'VERIFY_COMMAND_ORDER'\n",
    "\nVERIFY_COMMAND_ORDER",
  );
  if (!visible.includes(VISIBLE_SEQUENCE)) {
    failures.push("verifier visible reconciliation order must immediately follow Phase 117");
  }
  if (!verifyText.includes(EXECUTABLE_SEQUENCE)) {
    failures.push("verifier executable reconciliation order must immediately follow Phase 117");
  }
}

function extractSupportedMethodNames(source: string): Set<string> {
  const start = source.indexOf("pub enum SupportedMethod {");
  if (start < 0) return new Set();
  const end = source.indexOf("\n}", start);
  if (end < 0) return new Set();
  return new Set(
    [...source.slice(start, end).matchAll(/#\[serde\(rename = "([^"]+)"\)\]/g)].map(
      (match) => match[1] ?? "",
    ),
  );
}

function extractBulletCodeValues(text: string, label: string): Set<string> {
  const start = text.indexOf(`- ${label}`);
  if (start < 0) return new Set();
  const maybeEnd = text.indexOf("\n- ", start + 2);
  const block = text.slice(start, maybeEnd < 0 ? text.length : maybeEnd);
  return new Set([...block.matchAll(/`([a-z0-9]+)`/g)].map((match) => match[1] ?? ""));
}

function verifyTableTerm(
  text: string,
  firstCell: string,
  expectedTerm: string,
  failure: string,
  failures: string[],
): void {
  const maybeRow = parseTableRows(text).find((row) => row[0] === firstCell);
  if (maybeRow?.[1]?.replaceAll("`", "") !== expectedTerm) failures.push(failure);
}

function parseTableRows(text: string): string[][] {
  return text
    .split("\n")
    .filter((line) => line.startsWith("|") && line.endsWith("|"))
    .map((line) => line.slice(1, -1).split("|").map((cell) => cell.trim()))
    .filter((row) => row.some((cell) => !/^:?-+:?$/.test(cell)));
}

function reportSetDifference(
  expected: Set<string>,
  actual: Set<string>,
  kind: "extra" | "missing",
  failures: string[],
): void {
  const difference = [...expected].filter((name) => !actual.has(name)).sort();
  if (difference.length > 0) {
    failures.push(`catalog supported-method set mismatch: ${kind} ${difference.join(", ")}`);
  }
}

function setsEqual(left: Set<string>, right: Set<string>): boolean {
  return left.size === right.size && [...left].every((value) => right.has(value));
}

function requireAll(
  label: string,
  text: string,
  needles: readonly string[],
  failures: string[],
): void {
  const missing = needles.filter((needle) => !text.includes(needle));
  if (missing.length > 0) failures.push(`${label}: missing ${missing.join(", ")}`);
}

function rejectAny(
  label: string,
  text: string,
  needles: readonly string[],
  failures: string[],
): void {
  const found = needles.filter((needle) => text.includes(needle));
  if (found.length > 0) failures.push(`${label}: contains ${found.join(", ")}`);
}

function markdownSection(text: string, heading: string): string {
  const start = text.indexOf(heading);
  if (start < 0) return "";
  const next = text.indexOf("\n## ", start + heading.length);
  return text.slice(start, next < 0 ? text.length : next);
}

function sectionBefore(text: string, heading: string): string {
  const end = text.indexOf(heading);
  return end < 0 ? "" : text.slice(0, end);
}

function between(text: string, startMarker: string, endMarker: string): string {
  const start = text.indexOf(startMarker);
  if (start < 0) return "";
  const bodyStart = start + startMarker.length;
  const end = text.indexOf(endMarker, bodyStart);
  return end < 0 ? "" : text.slice(bodyStart, end);
}

function normalized(text: string): string {
  return text.replace(/^>\s?/gm, "").replace(/\s+/g, " ").trim().toLowerCase();
}

if (import.meta.main) {
  const failures = checkCurrentDocumentationReconciliation();
  if (failures.length > 0) {
    console.error("Current documentation reconciliation check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Current documentation reconciliation validated.");
}
