#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

import {
  PHASE124_SUMMARY_FILE,
  verifyPhase124CloseoutLifecycle,
} from "./check-phase124-milestone-closeout-lifecycle";
import {
  isPhase124GapClosureStage,
  verifyPhase124GapClosureStage,
} from "./check-phase124-milestone-gap-closure";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE123_TEST =
  "bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts";
const PHASE123_CHECK =
  "bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts";
const PHASE124_TEST =
  "bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts";
const PHASE124_CHECK =
  "bun run scripts/check-phase124-milestone-closeout-reconciliation.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
const ARCHIVE_ROUTE = "/gsd-complete-milestone v2.1";

const REQUIRED_FILES = [
  ".planning/REQUIREMENTS.md",
  ".planning/ROADMAP.md",
  ".planning/STATE.md",
  ".planning/v2.1-MILESTONE-AUDIT.md",
  ".planning/PROJECT.md",
  "README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "scripts/verify.sh",
] as const;
const HARDENING_REQUIREMENTS = [
  "HARD-01",
  "HARD-02",
  "HARD-03",
  "HARD-04",
] as const;
const RESOLVED_DEBT_IDS = [
  "DEBT-01-INBOUND-GETBLOCKTXN",
  "DEBT-02-PHASE112-TEST-VOCABULARY",
  "DEBT-03-SUCCESSFUL-BLOCK-WRITE-EVIDENCE",
  "DEBT-04-RECEIVE-INDEPENDENT-TIMEOUT",
  "DEBT-05-AUTHORITATIVE-RUNTIME-PROJECTION",
  "DEBT-06-MILESTONE-METADATA-RECONCILIATION",
] as const;
const CLAIM_TOPICS = [
  "public block serving by default",
  "public compact relay by default",
  "public compact-block relay by default",
  "archive-node",
  "archive node",
  "package relay",
  "filter serving",
  "public-network ci",
  "production full-node readiness",
  "production-funds wallet",
] as const;
const POSITIVE_CLAIM =
  /\b(supports|provides?|enables?|implements?|ships?|proves?|ready for)\b/;
const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "is not",
  "are not",
  "must not",
  "without",
  "outside",
  "out of scope",
  "deferred",
  "remain deferred",
  "remains deferred",
  "no claim",
  "no-claim",
  "guardrail",
  "before a future",
  "not allowed yet",
] as const;

type RequiredFile = (typeof REQUIRED_FILES)[number];
type TextCorpus = Map<RequiredFile, string>;
type CheckOptions = { rootDir?: string };
type RequirementEntry = { checked: boolean; id: string };
type TraceabilityEntry = { id: string; phase: number; status: string };

export function checkPhase124MilestoneCloseoutReconciliation(
  options: CheckOptions = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = loadCorpus(repoRoot, failures);
  const requirements = texts.get(".planning/REQUIREMENTS.md") ?? "";
  const roadmap = texts.get(".planning/ROADMAP.md") ?? "";
  const entries = parseRequirementEntries(requirements);
  const traceability = parseTraceabilityEntries(requirements);
  const maybeHard05 = entries.find((entry) => entry.id === "HARD-05");
  const finalStage = maybeHard05?.checked === true;
  const phaseComplete = roadmap.includes("- [x] **Phase 124:");
  const audit = texts.get(".planning/v2.1-MILESTONE-AUDIT.md") ?? "";
  const gapClosureStage = isPhase124GapClosureStage(roadmap, audit);

  if (gapClosureStage) {
    verifyPhase124GapClosureStage(repoRoot, requirements, roadmap, audit, failures);
    verifyPhase124CloseoutLifecycle(repoRoot, phaseComplete, failures);
  } else {
    verifyRequirementOwnership(entries, traceability, failures);
    verifyStageCounts(
      finalStage,
      phaseComplete,
      entries,
      traceability,
      requirements,
      roadmap,
      failures,
    );
    verifyRoadmapStage(finalStage, phaseComplete, roadmap, failures);
  }
  if (!gapClosureStage && finalStage) {
    verifyFinalAudit(audit, failures);
    verifyPhase124CloseoutLifecycle(repoRoot, phaseComplete, failures);
    if (phaseComplete) {
      verifyFinalRoute(texts, failures);
    } else {
      if (existsSync(path.join(repoRoot, PHASE124_SUMMARY_FILE))) {
        failures.push("P124 promoted pre-summary stage requires the current plan summary to be absent");
      }
      verifyPromotedRoute(texts.get(".planning/v2.1-MILESTONE-AUDIT.md") ?? "", failures);
    }
  }
  verifyNoClaimBoundary(texts, failures);
  verifyVerifierOrder(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function loadCorpus(repoRoot: string, failures: string[]): TextCorpus {
  const texts = new Map<RequiredFile, string>();
  for (const file of REQUIRED_FILES) {
    const absolutePath = path.join(repoRoot, file);
    if (!existsSync(absolutePath)) {
      failures.push(`P124 missing required corpus file: ${file}`);
      texts.set(file, "");
      continue;
    }
    texts.set(file, readFileSync(absolutePath, "utf8"));
  }
  return texts;
}

function parseRequirementEntries(text: string): RequirementEntry[] {
  return [...text.matchAll(/^- \[([ x])\] \*\*([A-Z]+-\d+)\*\*:/gm)].map(
    (match) => ({ checked: match[1] === "x", id: match[2] ?? "" }),
  );
}

function parseTraceabilityEntries(text: string): TraceabilityEntry[] {
  return [
    ...text.matchAll(
      /^\|\s*([A-Z]+-\d+)\s*\|\s*Phase\s+(\d+)\s*\|\s*(Complete|Pending)\s*\|$/gm,
    ),
  ].map((match) => ({
    id: match[1] ?? "",
    phase: Number(match[2]),
    status: match[3] ?? "",
  }));
}

function verifyRequirementOwnership(
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  failures: string[],
): void {
  requireExactNumber(entries.length, 39, "P124 requirement checklist total", failures);
  requireExactNumber(traceability.length, 39, "P124 traceability total", failures);
  const checklistIds = new Set(entries.map((entry) => entry.id));
  const traceabilityIds = new Set(traceability.map((entry) => entry.id));
  requireExactNumber(checklistIds.size, 39, "P124 unique checklist ownership", failures);
  requireExactNumber(traceabilityIds.size, 39, "P124 unique traceability ownership", failures);
  for (const entry of entries) {
    const owners = traceability.filter((candidate) => candidate.id === entry.id);
    if (owners.length !== 1) {
      failures.push(`P124 ${entry.id} must have exactly one traceability owner`);
    }
  }
  const hard05Owners = traceability.filter(
    (entry) => entry.id === "HARD-05" && entry.phase === 124,
  );
  if (hard05Owners.length !== 1) {
    failures.push("P124 HARD-05 must be owned exactly once by Phase 124");
  }
}

function verifyStageCounts(
  finalStage: boolean,
  phaseComplete: boolean,
  entries: RequirementEntry[],
  traceability: TraceabilityEntry[],
  requirements: string,
  roadmap: string,
  failures: string[],
): void {
  const expectedComplete = finalStage ? 39 : 38;
  const expectedPending = finalStage ? 0 : 1;
  const stage = finalStage ? "final" : "evidence-reconciled";
  requireExactNumber(
    entries.filter((entry) => entry.checked).length,
    expectedComplete,
    `P124 ${stage} checked requirement count`,
    failures,
  );
  requireExactNumber(
    traceability.filter((entry) => entry.status === "Complete").length,
    expectedComplete,
    `P124 ${stage} complete traceability count`,
    failures,
  );
  requireExactNumber(
    traceability.filter((entry) => entry.status === "Pending").length,
    expectedPending,
    `P124 ${stage} pending traceability count`,
    failures,
  );
  for (const requirement of HARDENING_REQUIREMENTS) {
    if (!entries.some((entry) => entry.id === requirement && entry.checked)) {
      failures.push(`P124 ${stage} requires checked ${requirement}`);
    }
  }
  const maybeHard05 = entries.find((entry) => entry.id === "HARD-05");
  if (!maybeHard05 || maybeHard05.checked !== finalStage) {
    failures.push(`P124 ${stage} HARD-05 checklist state is invalid`);
  }
  const maybeHard05Trace = traceability.find((entry) => entry.id === "HARD-05");
  const expectedHard05Status = finalStage ? "Complete" : "Pending";
  if (maybeHard05Trace?.status !== expectedHard05Status) {
    failures.push(`P124 ${stage} HARD-05 traceability must be ${expectedHard05Status}`);
  }
  for (const [text, label, completeLabel, projectedComplete, projectedPending] of [
    [requirements, "requirements", "Complete", expectedComplete, expectedPending],
    [roadmap, "roadmap", "Satisfied", phaseComplete ? 39 : 38, phaseComplete ? 0 : 1],
  ] as const) {
    for (const line of [
      "v2.1 requirements: 39 total",
      "Mapped to phases: 39",
      `${completeLabel}: ${projectedComplete}`,
      `Pending hardening and closeout: ${projectedPending}`,
      "Unmapped: 0",
    ]) {
      requireContains(text, line, `P124 ${stage} ${label} counts`, failures);
    }
  }
}

function verifyRoadmapStage(
  finalStage: boolean,
  phaseComplete: boolean,
  roadmap: string,
  failures: string[],
): void {
  const phase122 = phaseSection(roadmap, 122);
  const phase123 = phaseSection(roadmap, 123);
  const phase124 = phaseSection(roadmap, 124);
  requireContains(phase122, "**Plans:** 1/1 plans complete", "P124 Phase 122 evidence", failures);
  requireContains(phase123, "**Plans:** 7/7 plans complete", "P124 Phase 123 evidence", failures);
  requireAbsent(roadmap, "Plan Phase 122", "P124 stale Phase 122 route", failures);
  requireAbsent(roadmap, "/gsd-plan-phase 122", "P124 stale Phase 122 route", failures);
  if (phaseComplete) {
    if (!finalStage) {
      failures.push("P124 completed phase requires checked HARD-05");
    }
    requireContains(roadmap, "- [x] **Phase 124:", "P124 final phase state", failures);
    requireContains(phase124, "**Plans:** 2/2 plans complete", "P124 final plan state", failures);
    return;
  }
  const stage = finalStage ? "promoted pre-summary" : "intermediate";
  requireContains(roadmap, "- [ ] **Phase 124:", `P124 ${stage} phase state`, failures);
  if (finalStage) {
    requireContains(
      phase124,
      "**Plans:** 1/2 plans executed",
      "P124 promoted pre-summary plan state",
      failures,
    );
    return;
  }
  if (!/\*\*Plans:\*\* [01]\/2 plans executed/.test(phase124)) {
    failures.push("P124 intermediate Phase 124 plans must be 0/2 or 1/2 executed");
  }
}

function phaseSection(roadmap: string, phase: number): string {
  const marker = `#### Phase ${phase}:`;
  const start = roadmap.indexOf(marker);
  if (start === -1) return "";
  const end = roadmap.indexOf("\n#### Phase ", start + marker.length);
  return roadmap.slice(start, end === -1 ? roadmap.length : end);
}

function verifyFinalAudit(audit: string, failures: string[]): void {
  for (const needle of [
    "status: passed",
    'requirements: "39/39"',
    'phases: "15/15"',
    "tech_debt: []",
    "gaps:\n  requirements: []\n  integration: []\n  flows: []",
    "## Resolved Hardening Debt",
  ]) {
    requireContains(audit, needle, "P124 final canonical audit", failures);
  }
  for (const debtId of RESOLVED_DEBT_IDS) {
    requireExactNumber(
      countOccurrences(audit, debtId),
      1,
      `P124 resolved debt ledger ${debtId}`,
      failures,
    );
  }
}

function verifyFinalRoute(texts: TextCorpus, failures: string[]): void {
  for (const file of [
    ".planning/ROADMAP.md",
    ".planning/STATE.md",
    ".planning/v2.1-MILESTONE-AUDIT.md",
  ] as const) {
    const text = texts.get(file) ?? "";
    requireContains(text, ARCHIVE_ROUTE, `P124 final archive route ${file}`, failures);
    for (const staleRoute of ["/gsd-plan-phase", "/gsd-execute-phase"]) {
      requireAbsent(text, staleRoute, `P124 final stale route ${file}`, failures);
    }
  }
}

function verifyPromotedRoute(audit: string, failures: string[]): void {
  requireContains(audit, ARCHIVE_ROUTE, "P124 promoted archive route", failures);
  for (const staleRoute of ["/gsd-plan-phase", "/gsd-execute-phase"]) {
    requireAbsent(audit, staleRoute, "P124 promoted stale route", failures);
  }
}

function verifyNoClaimBoundary(texts: TextCorpus, failures: string[]): void {
  for (const file of [
    ".planning/PROJECT.md",
    ".planning/ROADMAP.md",
    ".planning/v2.1-MILESTONE-AUDIT.md",
    "README.md",
    "docs/parity/release-readiness.md",
    "docs/parity/production-claim-boundary.md",
  ] as const) {
    const text = (texts.get(file) ?? "").toLowerCase();
    for (const line of text.split("\n")) {
      const tableNoClaim = tableRowHasNoClaimStatus(line);
      const rowTopics = CLAIM_TOPICS.filter((topic) => line.includes(topic));
      for (const clause of claimClauses(line)) {
        for (const segment of dangerousClaimSegments(clause)) {
          if (!POSITIVE_CLAIM.test(segment)) continue;
          if (NO_CLAIM_MARKERS.some((marker) => segment.includes(marker))) continue;
          if (tableNoClaim && rowTopics.length === 1) continue;
          for (const topic of CLAIM_TOPICS) {
            if (segment.includes(topic)) {
              failures.push(`P124 no-claim boundary ${file} has positive claim: ${topic}`);
            }
          }
        }
      }
    }
  }
}

function tableRowHasNoClaimStatus(line: string): boolean {
  const trimmed = line.trim();
  if (!trimmed.startsWith("|") || !trimmed.endsWith("|")) return false;
  return trimmed
    .slice(1, -1)
    .split("|")
    .map((value) => value.trim().replaceAll("`", ""))
    .some((value) => ["deferred", "unsupported", "not allowed", "not allowed yet"].includes(value));
}

function claimClauses(line: string): string[] {
  const trimmed = line.trim();
  if (trimmed.startsWith("|") && trimmed.endsWith("|")) {
    return trimmed
      .slice(1, -1)
      .split("|")
      .map((value) => value.trim())
      .filter((value) => value !== "");
  }
  return trimmed.split(/(?<=[.!?])\s+/).filter((value) => value !== "");
}

function dangerousClaimSegments(clause: string): string[] {
  return clause
    .split(
      /\s*;\s*|,?\s+while\s+|,?\s+but\s+|\s+whereas\s+|\s+and\s+(?=(?:open bitcoin\s+)?(?:package relay|filter serving|public|archive|production))/,
    )
    .map((value) => value.trim())
    .filter((value) => value !== "");
}

function verifyVerifierOrder(verifyScript: string, failures: string[]): void {
  const expected = [
    PHASE123_TEST,
    PHASE123_CHECK,
    PHASE124_TEST,
    PHASE124_CHECK,
    PHASE117_TEST,
    PHASE117_CHECK,
  ];
  const visible = visibleCommandOrder(verifyScript);
  requireOrdered(visible, expected, "P124 visible verifier order", failures);
  const executableCommands = executableRunStepCommands(verifyScript);
  requireOrdered(
    executableCommands.join("\n"),
    expected,
    "P124 executable verifier order",
    failures,
  );
  requireExactNumber(
    countOccurrences(verifyScript, PHASE124_TEST),
    2,
    "P124 verifier mutation command count",
    failures,
  );
  requireExactNumber(
    countOccurrences(verifyScript, PHASE124_CHECK),
    2,
    "P124 verifier live command count",
    failures,
  );
  requireFinalPhaseChecker(visible, "P124 visible verifier final gate", failures);
  requireFinalPhaseChecker(
    executableCommands.join("\n"),
    "P124 executable verifier final gate",
    failures,
  );
}

function requireFinalPhaseChecker(text: string, label: string, failures: string[]): void {
  const phaseCommands = text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => /\bbun (?:test|run) scripts\/check-phase\d+/.test(line));
  if (!phaseCommands.at(-1)?.includes(PHASE117_CHECK)) {
    failures.push(`${label} must end with ${PHASE117_CHECK}`);
  }
}

function visibleCommandOrder(text: string): string {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = text.indexOf(marker);
  if (start === -1) return "";
  const bodyStart = start + marker.length;
  const end = text.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  return end === -1 ? "" : text.slice(bodyStart, end);
}

function executableRunStepCommands(text: string): string[] {
  const commands: string[] = [];
  let current = "";
  for (const rawLine of text.split("\n")) {
    const line = rawLine.trim();
    if (current === "" && !line.startsWith("run_step")) continue;
    current = `${current} ${line}`.trim();
    if (current.endsWith("\\")) {
      current = current.slice(0, -1).trim();
      continue;
    }
    commands.push(current.replace(/\s+/g, " "));
    current = "";
  }
  if (current !== "") commands.push(current.replace(/\s+/g, " "));
  return commands;
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) failures.push(`${label} missing ${needle}`);
}

function requireAbsent(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) failures.push(`${label} must not contain ${needle}`);
}

function requireExactNumber(
  actual: number,
  expected: number,
  label: string,
  failures: string[],
): void {
  if (actual !== expected) failures.push(`${label}: expected ${expected}, found ${actual}`);
}

function requireOrdered(
  text: string,
  needles: readonly string[],
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle, cursor + 1);
    if (index === -1) failures.push(`${label} missing or out of order ${needle}`);
    else cursor = index;
  }
}

function countOccurrences(text: string, needle: string): number {
  return text.split(needle).length - 1;
}

if (import.meta.main) {
  const failures = checkPhase124MilestoneCloseoutReconciliation();
  if (failures.length > 0) {
    console.error("Phase 124 milestone closeout reconciliation checker failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 124 milestone closeout reconciliation checker passed.");
}
