import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { resolvePlanningRequirementsSource } from "../source-corpus.ts";
import {
  ARCHIVED_V24_AUDIT,
  ARCHIVED_V24_REQUIREMENTS,
  ARCHIVED_V24_ROADMAP,
  AUDIT_FILE,
  REQUIREMENTS_FILE,
  ROADMAP_FILE,
  V24_REQUIREMENTS_MILESTONE_NEEDLE,
} from "./constants.ts";

/** Selects only the matching active v2.4 control file, otherwise its archive. */
export function resolvePlanningSource(repoRoot: string, file: string): string {
  if (file === REQUIREMENTS_FILE) {
    return resolvePlanningRequirementsSource(
      repoRoot, ARCHIVED_V24_REQUIREMENTS, V24_REQUIREMENTS_MILESTONE_NEEDLE,
    );
  }
  if (file !== ROADMAP_FILE) return file;
  const livePath = path.join(repoRoot, ROADMAP_FILE);
  if (existsSync(livePath) && /^## Active Milestone: v2\.4 /m.test(readFileSync(livePath, "utf8"))) {
    return ROADMAP_FILE;
  }
  if (existsSync(path.join(repoRoot, ARCHIVED_V24_ROADMAP))) return ARCHIVED_V24_ROADMAP;
  return ROADMAP_FILE;
}

/** The versioned audit moves with milestone controls while phase evidence stays put. */
export function resolveGapAuditSource(repoRoot: string): string {
  if (existsSync(path.join(repoRoot, AUDIT_FILE))) return AUDIT_FILE;
  return ARCHIVED_V24_AUDIT;
}

/** Rejects unrelated controls before they can satisfy historical v2.4 checks. */
export function checkPlanningSourceIdentity(file: string, text: string, failures: string[]): void {
  if (file === REQUIREMENTS_FILE && !text.includes(V24_REQUIREMENTS_MILESTONE_NEEDLE)) {
    failures.push("requirements source must identify milestone v2.4");
  }
  if (file === ROADMAP_FILE && !/^## Active Milestone: v2\.4 /m.test(text) &&
    !/^# Milestone v2\.4:/m.test(text)) {
    failures.push("roadmap source must identify milestone v2.4");
  }
}

/** Archived completion cannot silently lose its final milestone audit. */
export function checkArchivedAudit(repoRoot: string, failures: string[]): void {
  const auditPath = path.join(repoRoot, ARCHIVED_V24_AUDIT);
  if (!existsSync(auditPath)) {
    failures.push(`missing target file ${ARCHIVED_V24_AUDIT}`);
    return;
  }
  try {
    const text = readFileSync(auditPath, "utf8");
    const maybeFrontmatter = /^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/.exec(text)?.[1];
    const parsed: unknown = maybeFrontmatter ? Bun.YAML.parse(maybeFrontmatter) : null;
    if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed) ||
      !("milestone" in parsed) || parsed.milestone !== "v2.4" ||
      !("status" in parsed) || !["passed", "tech_debt"].includes(String(parsed.status))) {
      throw new Error("archived audit must identify completed v2.4");
    }
  } catch (error) {
    failures.push(`invalid archived v2.4 audit: ${String(error)}`);
  }
}
