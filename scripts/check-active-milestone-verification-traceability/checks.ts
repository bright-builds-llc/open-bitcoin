import {
  existsSync,
  readFileSync,
  readdirSync,
  statSync,
} from "node:fs";
import path from "node:path";
import { DEFAULT_ROOT_DIR, ROADMAP_FILE, REQUIREMENTS_FILE, ARCHIVED_V21_FILES, ACTIVE_MILESTONE_HEADING, CheckActiveMilestoneVerificationTraceabilityOptions, ActiveRequirement, TraceabilityRow } from "./constants.ts";
import { readRequiredText, loadPhaseCorpora } from "./filesystem.ts";
import { parseActiveRoadmapPhases, parseActiveRequirements, parseTraceabilityRows, countValues } from "./parsing.ts";
import { activatedRequirementIds, lifecycleValidCoverage } from "./lifecycle.ts";

export function checkActiveMilestoneVerificationTraceability(
  maybeOptions: CheckActiveMilestoneVerificationTraceabilityOptions = {},
): string[] {
  const rootDir = path.resolve(
    maybeOptions.maybeRootDir ?? DEFAULT_ROOT_DIR,
  );
  const failures: string[] = [];
  const roadmap = readRequiredText(rootDir, ROADMAP_FILE, failures);
  if (
    !roadmap.includes(ACTIVE_MILESTONE_HEADING) &&
    ARCHIVED_V21_FILES.every((file) => existsSync(path.join(rootDir, file)))
  ) {
    return failures;
  }
  const requirements = readRequiredText(
    rootDir,
    REQUIREMENTS_FILE,
    failures,
  );
  const activePhases = parseActiveRoadmapPhases(roadmap, failures);
  const activeRequirements = parseActiveRequirements(
    requirements,
    failures,
  );
  const traceabilityRows = parseTraceabilityRows(requirements, failures);

  const ownedRequirementIds = verifyTraceabilityOwnership(
    activeRequirements,
    traceabilityRows,
    activePhases,
    failures,
  );

  const phaseCorpora = loadPhaseCorpora(rootDir, activePhases, failures);
  const activatedIds = activatedRequirementIds(
    phaseCorpora,
    ownedRequirementIds,
    failures,
  );
  verifyCompletedRequirementsActivated(
    activeRequirements,
    traceabilityRows,
    ownedRequirementIds,
    activatedIds,
    failures,
  );
  const coveredIds = new Set<string>();
  for (const corpus of phaseCorpora) {
    const ownerActivatedIds = new Set(
      traceabilityRows
        .filter(
          (row) =>
            row.phase === corpus.phase &&
            ownedRequirementIds.has(row.id) &&
            activatedIds.has(row.id),
        )
        .map((row) => row.id),
    );
    for (const id of lifecycleValidCoverage(
      [corpus],
      ownerActivatedIds,
      failures,
    )) {
      coveredIds.add(id);
    }
  }

  for (const requirement of activeRequirements) {
    const owner = traceabilityRows.find((row) => row.id === requirement.id);
    if (
      !ownedRequirementIds.has(requirement.id) ||
      !activatedIds.has(requirement.id) ||
      !requirement.checked ||
      owner?.status !== "Complete"
    ) {
      continue;
    }
    if (!coveredIds.has(requirement.id)) {
      failures.push(
        `activated requirement ${requirement.id} is missing lifecycle-valid active-phase verification coverage`,
      );
    }
  }

  return failures;
}

export function verifyTraceabilityOwnership(
  requirements: ActiveRequirement[],
  rows: TraceabilityRow[],
  activePhases: Set<number>,
  failures: string[],
): Set<string> {
  const ownedIds = new Set<string>();
  const checklistCounts = countValues(
    requirements.map((requirement) => requirement.id),
  );
  for (const requirement of requirements) {
    if (checklistCounts.get(requirement.id) !== 1) {
      continue;
    }
    const owners = rows.filter((row) => row.id === requirement.id);
    if (owners.length !== 1) {
      failures.push(
        `active requirement ${requirement.id} must have exactly one traceability row; found ${owners.length}`,
      );
      continue;
    }
    const owner = owners[0];
    if (owner && !activePhases.has(owner.phase)) {
      failures.push(
        `active requirement ${requirement.id} traceability owner Phase ${owner.phase} is not in the active roadmap`,
      );
      continue;
    }
    if (owner) {
      ownedIds.add(requirement.id);
    }
  }
  return ownedIds;
}

export function verifyCompletedRequirementsActivated(
  requirements: ActiveRequirement[],
  rows: TraceabilityRow[],
  ownedRequirementIds: Set<string>,
  activatedIds: Set<string>,
  failures: string[],
): void {
  for (const requirement of requirements) {
    if (!ownedRequirementIds.has(requirement.id)) {
      continue;
    }
    const owner = rows.find((row) => row.id === requirement.id);
    const traceabilityComplete = owner?.status === "Complete";
    if (requirement.checked !== traceabilityComplete) {
      failures.push(
        `active requirement ${requirement.id} has inconsistent checklist and traceability completion state`,
      );
      continue;
    }
    if (!requirement.checked || activatedIds.has(requirement.id)) {
      continue;
    }
    failures.push(
      `completed active requirement ${requirement.id} has no requirements-completed summary activation`,
    );
  }
}
