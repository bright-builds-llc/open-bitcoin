import { existsSync, readFileSync, statSync } from "node:fs";
import path from "node:path";
import { resolveGapAuditSource } from "./planning-sources.ts";
import { loadPhaseCorpora } from "../check-active-milestone-verification-traceability/filesystem.ts";
import {
  activatedRequirementIds,
  lifecycleValidCoverage,
} from "../check-active-milestone-verification-traceability/lifecycle.ts";

const CLOSURES = {
  "SNAP-01": {
    phase: 152,
    originalPhase: 146,
    title: "Post-Prune Wallet Rescan Eligibility",
    directory: "152-post-prune-wallet-rescan-eligibility",
    gap: "INT-02",
  },
  "PRUN-01": {
    phase: 153,
    originalPhase: 147,
    title: "Automatic Prune Retention Integration",
    directory: "153-automatic-prune-retention-integration",
    gap: "INT-01",
  },
  "PRUN-02": {
    phase: 153,
    originalPhase: 147,
    title: "Automatic Prune Retention Integration",
    directory: "153-automatic-prune-retention-integration",
    gap: "INT-01",
  },
} as const;

/** Recognizes audited Pending work without treating it as milestone completion. */
export function pendingGapClosureRequirements(
  repoRoot: string,
  requirements: string,
  roadmap: string,
  failures: string[],
): Set<string> {
  const candidates = Object.entries(CLOSURES).filter(([id]) =>
    requirements.includes(`- [ ] **${id}**`),
  );
  if (candidates.length === 0) return new Set();
  const maybeAudit = maybeReadGapAudit(repoRoot, failures);
  if (!maybeAudit) return new Set();
  const pending = new Set<string>();
  for (const [id, closure] of candidates) {
    const matchingGaps = maybeAudit.requirements.filter(
      (gap) => isRecord(gap) && gap.id === id,
    );
    const maybeGap = matchingGaps[0];
    if (matchingGaps.length !== 1 || !isRecord(maybeGap)) continue;
    if (maybeGap.phase !== closure.originalPhase) continue;
    if (typeof maybeGap.status !== "string" ||
      !["partial", "unsatisfied", "orphaned"].includes(maybeGap.status)) continue;
    if (!hasIntegrationGap(maybeAudit.integration, closure.gap, id)) continue;
    if (!hasTraceabilityRow(requirements, id, closure.phase, "Pending")) continue;
    if (!hasTraceabilityRow(roadmap, id, closure.phase, "Pending")) continue;
    if (!roadmap.includes(`- [ ] **Phase ${closure.phase}: ${closure.title}**`)) continue;
    const sections = roadmap.split(/(?=^### Phase |^## )/m);
    const maybeSection = sections.find((section) =>
      section.startsWith(`### Phase ${closure.phase}: ${closure.title}\n`),
    );
    if (!maybeSection?.includes(`**Gap Closure**: ${closure.gap}`)) continue;
    const directory = path.join(repoRoot, ".planning/phases", closure.directory);
    if (!existsSync(directory) || !statSync(directory).isDirectory()) continue;
    pending.add(id);
  }
  return pending;
}

type GapAudit = { requirements: unknown[]; integration: unknown[] };

/** Reassigned completion must be proved by the new owner, not historical reports. */
export function checkGapClosureCompletion(
  repoRoot: string,
  requirements: string,
  roadmap: string,
  failures: string[],
  requireClosureEvidence = false,
): void {
  const hasClosureMetadata = requireClosureEvidence ||
    existsSync(path.join(repoRoot, resolveGapAuditSource(repoRoot))) ||
    Object.values(CLOSURES).some((closure) =>
      roadmap.includes(`### Phase ${closure.phase}: ${closure.title}`),
    );
  if (!hasClosureMetadata) return;
  for (const [id, closure] of Object.entries(CLOSURES)) {
    if (!requirements.includes(`- [x] **${id}**`)) continue;
    if (!hasTraceabilityRow(requirements, id, closure.phase, "Complete") ||
      !hasTraceabilityRow(roadmap, id, closure.phase, "Complete")) {
      failures.push(`${id} closure completion must map uniquely to Phase ${closure.phase}`);
      continue;
    }
    const corpora = loadPhaseCorpora(repoRoot, new Set([closure.phase]), failures);
    const activated = activatedRequirementIds(corpora, new Set([id]), failures);
    const covered = lifecycleValidCoverage(corpora, activated, failures);
    if (!activated.has(id) || !covered.has(id)) {
      failures.push(`${id} closure requires Phase ${closure.phase} summary and lifecycle-valid verification`);
    }
  }
}

function maybeReadGapAudit(repoRoot: string, failures: string[]): GapAudit | null {
  const auditPath = path.join(repoRoot, resolveGapAuditSource(repoRoot));
  if (!existsSync(auditPath)) return null;
  try {
    const text = readFileSync(auditPath, "utf8");
    const maybeFrontmatter = /^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/.exec(text)?.[1];
    if (!maybeFrontmatter) throw new Error("missing audit frontmatter");
    const parsed: unknown = Bun.YAML.parse(maybeFrontmatter);
    if (!isRecord(parsed) || parsed.milestone !== "v2.4" || parsed.status !== "gaps_found") {
      throw new Error("audit must identify v2.4 gaps_found");
    }
    const gaps = parsed.gaps;
    if (!isRecord(gaps) || !Array.isArray(gaps.requirements) || !Array.isArray(gaps.integration)) {
      throw new Error("audit must contain structured requirement and integration gaps");
    }
    return { requirements: gaps.requirements, integration: gaps.integration };
  } catch (error) {
    failures.push(`invalid pending gap-closure audit: ${String(error)}`);
    return null;
  }
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function hasIntegrationGap(gaps: unknown[], gapId: string, id: string): boolean {
  return gaps.some(
    (gap) => isRecord(gap) && gap.id === gapId &&
      Array.isArray(gap.requirements) && gap.requirements.includes(id),
  );
}

function hasTraceabilityRow(
  text: string,
  id: string,
  phase: number,
  status: "Pending" | "Complete",
): boolean {
  const rows = text.split("\n").filter((line) => new RegExp(`^\\|\\s*${id}\\s*\\|`).test(line));
  return rows.length === 1 && new RegExp(
    `^\\|\\s*${id}\\s*\\|\\s*Phase ${phase}\\s*\\|\\s*${status}\\s*\\|\\s*$`,
  ).test(rows[0] ?? "");
}
