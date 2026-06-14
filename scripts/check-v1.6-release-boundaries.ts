#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-6-full-sync-completion-release-boundaries";
const REQUIRED_REQUIREMENTS = ["REL-01", "REL-02", "REL-03"] as const;
const ALL_V1_6_REQUIREMENTS = [
  "SYNC-01",
  "SYNC-02",
  "SYNC-03",
  "SYNC-04",
  "TIP-01",
  "TIP-02",
  "TIP-03",
  "REC-01",
  "REC-02",
  "REC-03",
  "REC-04",
  "RES-01",
  "RES-02",
  "RES-03",
  "RES-04",
  "OBS-01",
  "OBS-02",
  "OBS-03",
  "OBS-04",
  "VER-01",
  "VER-02",
  "VER-03",
  "VER-04",
  "REL-01",
  "REL-02",
  "REL-03",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/threat-model-v1.6.md",
  "docs/parity/release-readiness.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/README.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "scripts/check-v1.6-release-boundaries.ts",
  "scripts/verify.sh",
  ".planning/phases/73-opt-in-uat-and-deterministic-verification/73-VERIFICATION.md",
] as const;
const DEFERRED_SURFACE_TEXT = [
  "production-node",
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet safety",
  "migration apply mode",
  "signed packaging",
  "Windows service support",
  "GUI parity",
  "hosted dashboards",
  "public-network CI",
  "release-blocking live sync",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl --user",
  "launchctl",
  "-openbitcoinsync=mainnet-ibd",
  "current tip",
  "release-blocking live sync",
] as const;

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ParitySurface = {
  name?: unknown;
  status?: unknown;
};

type ParityIndex = {
  audit?: unknown;
  checklist?: {
    surfaces?: unknown;
  };
  surfaces?: unknown;
};

function readText(relativePath: string): string {
  return readFileSync(path.join(REPO_ROOT, relativePath), "utf8");
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

function requireAllContains(text: string, needles: readonly string[], label: string): void {
  for (const needle of needles) {
    requireContains(text, needle, label);
  }
}

function readJoined(relativePaths: readonly string[]): string {
  return relativePaths.map((relativePath) => readText(relativePath)).join("\n");
}

function parseParityIndex(): ParityIndex {
  return JSON.parse(readText("docs/parity/index.json")) as ParityIndex;
}

function checklistSurfaces(index: ParityIndex): ChecklistSurface[] {
  const maybeSurfaces = index.checklist?.surfaces;
  if (!Array.isArray(maybeSurfaces)) {
    throw new Error("docs/parity/index.json checklist.surfaces must be an array");
  }

  return maybeSurfaces as ChecklistSurface[];
}

function requireChecklistSurface(index: ParityIndex): ChecklistSurface {
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

function requireTopLevelSurface(index: ParityIndex): void {
  if (!Array.isArray(index.surfaces)) {
    throw new Error("docs/parity/index.json surfaces must be an array");
  }

  const matchingSurfaces = (index.surfaces as ParitySurface[]).filter(
    (surface) => surface.name === SURFACE_ID,
  );
  if (matchingSurfaces.length !== 1) {
    throw new Error(
      `expected exactly one top-level surface with name ${SURFACE_ID}, found ${matchingSurfaces.length}`,
    );
  }

  const [surface] = matchingSurfaces;
  if (surface.status !== "done") {
    throw new Error(`${SURFACE_ID} top-level status must be done`);
  }
}

function verifyParityIndex(index: ParityIndex): void {
  requireTopLevelSurface(index);

  const surface = requireChecklistSurface(index);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireArrayIncludes(surface.requirements, `${SURFACE_ID}.requirements`, requirement);
  }
  for (const evidencePath of REQUIRED_EVIDENCE) {
    requireArrayIncludes(surface.evidence, `${SURFACE_ID}.evidence`, evidencePath);
  }

  const auditText = JSON.stringify(index.audit ?? {});
  requireAllContains(
    auditText,
    [
      "v1_6_threat_model",
      "v1_6_release_boundaries",
      "threat-model-v1.6.md",
      "release-readiness.md",
      ...REQUIRED_REQUIREMENTS,
    ],
    "docs/parity/index.json audit",
  );
}

function verifyChecklist(): void {
  const checklist = readText("docs/parity/checklist.md");

  requireAllContains(
    checklist,
    [SURFACE_ID, ...REQUIRED_REQUIREMENTS, "threat-model-v1.6.md"],
    "docs/parity/checklist.md",
  );
}

function verifyThreatModel(): void {
  const threatModel = readText("docs/parity/threat-model-v1.6.md");

  requireAllContains(
    threatModel,
    [
      "STRIDE Threat Register",
      "ASVS L1 Mapping",
      "OWASP ASVS v5.0.0",
      "Release Boundary Matrix",
      "Requirements Traceability",
      "V16-TM-01",
      "V16-TM-02",
      "V16-TM-03",
      "V16-TM-04",
      "V16-TM-05",
      "V16-TM-06",
      "V16-TM-07",
      "V16-TM-08",
      ...REQUIRED_REQUIREMENTS,
      ...DEFERRED_SURFACE_TEXT,
    ],
    "docs/parity/threat-model-v1.6.md",
  );
}

function verifyReleaseReadiness(): void {
  const releaseReadiness = readText("docs/parity/release-readiness.md");

  requireAllContains(
    releaseReadiness,
    [
      "v1.6 Full-Sync Completion Claim Boundary Matrix",
      "scripts/check-v1.6-release-boundaries.ts",
      "Phase 68 through Phase 73",
      "Final v1.6 traceability covers all 26 milestone requirement IDs",
      ...ALL_V1_6_REQUIREMENTS,
      ...DEFERRED_SURFACE_TEXT,
    ],
    "docs/parity/release-readiness.md",
  );
}

function verifyHumanDocs(): void {
  const docsText = readJoined([
    "docs/parity/README.md",
    "docs/parity/deviations-and-unknowns.md",
    "docs/parity/catalog/p2p.md",
    "docs/parity/catalog/chainstate.md",
    "docs/parity/catalog/operator-runtime-release-hardening.md",
    "README.md",
    "docs/operator/runtime-guide.md",
  ]);

  requireAllContains(
    docsText,
    [
      SURFACE_ID,
      "threat-model-v1.6.md",
      "explicit opt-in full-sync completion",
      "v1.6 release boundary",
      "support-evidence.json",
      "support bundles",
      "best-known-tip",
      "stay-current",
      ...DEFERRED_SURFACE_TEXT,
    ],
    "v1.6 human docs",
  );
}

function verifyPhaseEvidence(): void {
  const phase73Verification = readText(
    ".planning/phases/73-opt-in-uat-and-deterministic-verification/73-VERIFICATION.md",
  );
  requireAllContains(
    phase73Verification,
    ["status: passed", "VER-01", "VER-02", "VER-03", "VER-04"],
    "Phase 73 verification",
  );
}

function verifyVerifyScript(): void {
  const verifyScript = readText("scripts/verify.sh");

  requireContains(
    verifyScript,
    "bun run scripts/check-v1.6-release-boundaries.ts",
    "scripts/verify.sh",
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh");
  }
}

function main(): void {
  const index = parseParityIndex();

  verifyParityIndex(index);
  verifyChecklist();
  verifyThreatModel();
  verifyReleaseReadiness();
  verifyHumanDocs();
  verifyPhaseEvidence();
  verifyVerifyScript();

  console.log("validated v1.6 release boundary parity roots");
}

main();
