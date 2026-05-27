#!/usr/bin/env bun

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";

const SURFACE_ID = "v1-3-threat-model-release-boundaries";
const REQUIRED_REQUIREMENTS = ["PROOF-06", "SEC-01", "SEC-02"] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/threat-model-v1.3.md",
  "docs/parity/release-readiness.md",
] as const;

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ParityIndex = {
  audit?: unknown;
  checklist?: {
    surfaces?: unknown;
  };
};

function repoRoot(): string {
  return execFileSync("git", ["rev-parse", "--show-toplevel"], {
    cwd: process.cwd(),
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  }).trim();
}

function readText(repoRootPath: string, relativePath: string): string {
  return readFileSync(path.join(repoRootPath, relativePath), "utf8");
}

function requireContains(text: string, needle: string, label: string): void {
  if (!text.includes(needle)) {
    throw new Error(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(text: string, needle: string, label: string): void {
  if (text.includes(needle)) {
    throw new Error(`${label} must not contain: ${needle}`);
  }
}

function requireArrayIncludes(value: unknown, label: string, required: string): void {
  if (!Array.isArray(value)) {
    throw new Error(`${label} must be an array`);
  }
  if (!value.includes(required)) {
    throw new Error(`${label} missing required value: ${required}`);
  }
}

function parseParityIndex(repoRootPath: string): ParityIndex {
  return JSON.parse(readText(repoRootPath, "docs/parity/index.json")) as ParityIndex;
}

function checklistSurfaces(index: ParityIndex): ChecklistSurface[] {
  const maybeSurfaces = index.checklist?.surfaces;
  if (!Array.isArray(maybeSurfaces)) {
    throw new Error("docs/parity/index.json checklist.surfaces must be an array");
  }

  return maybeSurfaces as ChecklistSurface[];
}

function requireSurface(index: ParityIndex): ChecklistSurface {
  const matchingSurfaces = checklistSurfaces(index).filter(
    (surface) => surface.id === SURFACE_ID,
  );
  if (matchingSurfaces.length !== 1) {
    throw new Error(
      `expected exactly one checklist surface with id ${SURFACE_ID}, found ${matchingSurfaces.length}`,
    );
  }

  const [surface] = matchingSurfaces;
  if (surface.status !== "done") {
    throw new Error(`${SURFACE_ID} status must be done`);
  }

  return surface;
}

function verifyParityIndex(index: ParityIndex): void {
  const surface = requireSurface(index);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireArrayIncludes(surface.requirements, `${SURFACE_ID}.requirements`, requirement);
  }
  for (const evidencePath of REQUIRED_EVIDENCE) {
    requireArrayIncludes(surface.evidence, `${SURFACE_ID}.evidence`, evidencePath);
  }

  const auditText = JSON.stringify(index.audit ?? {});
  for (const requiredText of [
    "v1_3_threat_model",
    "v1_3_release_boundaries",
    "threat-model-v1.3.md",
    "release-readiness.md",
    ...REQUIRED_REQUIREMENTS,
  ]) {
    requireContains(auditText, requiredText, "docs/parity/index.json audit");
  }
}

function verifyDocs(repoRootPath: string): void {
  const checklist = readText(repoRootPath, "docs/parity/checklist.md");
  const readme = readText(repoRootPath, "docs/parity/README.md");
  const releaseReadiness = readText(repoRootPath, "docs/parity/release-readiness.md");
  const threatModel = readText(repoRootPath, "docs/parity/threat-model-v1.3.md");
  const deviations = readText(repoRootPath, "docs/parity/deviations-and-unknowns.md");
  const verifyScript = readText(repoRootPath, "scripts/verify.sh");

  requireContains(checklist, SURFACE_ID, "docs/parity/checklist.md");
  requireContains(readme, "threat-model-v1.3.md", "docs/parity/README.md");

  for (const requiredText of [
    "v1.3 Release Claim Boundary Matrix",
    "Phase 50 Evidence Acceptance Contract",
    "bash scripts/verify.sh",
    "bun run scripts/run-live-mainnet-smoke.ts",
    "support-evidence.json",
    "support-evidence.md",
  ]) {
    requireContains(releaseReadiness, requiredText, "docs/parity/release-readiness.md");
  }

  for (const requiredText of [
    "STRIDE Threat Register",
    "V13-TM-01",
    "V13-TM-02",
    "V13-TM-03",
    "V13-TM-04",
    "V13-TM-05",
    "V13-TM-06",
    "public peer input",
    "resource exhaustion",
    "storage corruption",
    "operator RPC controls",
    "log/report redaction",
    "live evidence handling",
  ]) {
    requireContains(threatModel, requiredText, "docs/parity/threat-model-v1.3.md");
  }

  for (const requiredText of [
    "inbound serving",
    "transaction relay",
    "production-funds",
    "migration apply mode",
    "packaging",
    "hosted/public dashboard",
    "GUI",
    "unattended production-node",
  ]) {
    requireContains(deviations, requiredText, "docs/parity/deviations-and-unknowns.md");
  }

  requireContains(
    verifyScript,
    "bun run scripts/check-v1.3-release-boundaries.ts",
    "scripts/verify.sh",
  );
  requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh");
}

function main(): void {
  const repoRootPath = repoRoot();
  const index = parseParityIndex(repoRootPath);

  verifyParityIndex(index);
  verifyDocs(repoRootPath);

  console.log("validated v1.3 release boundary parity roots");
}

main();
