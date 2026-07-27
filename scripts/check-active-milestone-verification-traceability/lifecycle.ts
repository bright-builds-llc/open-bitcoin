import type {
  Artifact,
  LifecycleIdentity,
  PhaseCorpus,
} from "./constants.ts";
import {
  containsRequirementToken,
  exactScalar,
  parseRequirementsCompleted,
  requireExactScalar,
} from "./parsing.ts";

export function activatedRequirementIds(
  corpora: PhaseCorpus[],
  ownedRequirementIds: Set<string>,
  failures: string[],
): Set<string> {
  const activated = new Set<string>();

  for (const corpus of corpora) {
    for (const summary of corpus.summaries) {
      if (summary.frontmatter === null) {
        continue;
      }
      verifyArtifactLifecycle(summary, corpus.lifecycle, failures);
      for (const id of parseRequirementsCompleted(summary, failures)) {
        if (ownedRequirementIds.has(id)) {
          activated.add(id);
        }
      }
    }
  }
  return activated;
}

export function lifecycleValidCoverage(
  corpora: PhaseCorpus[],
  activatedIds: Set<string>,
  failures: string[],
): Set<string> {
  const covered = new Set<string>();

  for (const corpus of corpora) {
    for (const verification of corpus.verifications) {
      const lifecycleValid = verifyVerificationLifecycle(
        verification,
        corpus.lifecycle,
        failures,
      );
      if (!lifecycleValid) {
        continue;
      }
      for (const id of activatedIds) {
        if (containsRequirementToken(verification.text, id)) {
          covered.add(id);
        }
      }
    }
  }
  return covered;
}

export function verifyArtifactLifecycle(
  artifact: Artifact,
  expected: LifecycleIdentity | null,
  failures: string[],
): boolean {
  if (artifact.frontmatter === null || expected === null) {
    return false;
  }
  const maybeActual = parseLifecycleIdentity(
    artifact.frontmatter,
    artifact.relativePath,
    failures,
  );
  if (maybeActual === null) {
    return false;
  }
  let valid = true;
  if (maybeActual.mode !== expected.mode) {
    failures.push(
      `${artifact.relativePath} lifecycle_mode does not match its phase CONTEXT`,
    );
    valid = false;
  }
  if (maybeActual.phaseLifecycleId !== expected.phaseLifecycleId) {
    failures.push(
      `${artifact.relativePath} phase_lifecycle_id does not match its phase CONTEXT`,
    );
    valid = false;
  }
  return valid;
}

export function verifyVerificationLifecycle(
  artifact: Artifact,
  expected: LifecycleIdentity | null,
  failures: string[],
): boolean {
  const lifecycleValid = verifyArtifactLifecycle(
    artifact,
    expected,
    failures,
  );
  if (artifact.frontmatter === null) {
    return false;
  }
  const statusValid = requireExactScalar(
    artifact.frontmatter,
    "status",
    "passed",
    artifact.relativePath,
    failures,
  );
  const validationValid = requireExactScalar(
    artifact.frontmatter,
    "lifecycle_validated",
    "true",
    artifact.relativePath,
    failures,
  );
  return lifecycleValid && statusValid && validationValid;
}

export function parseLifecycleIdentity(
  frontmatter: string,
  relativePath: string,
  failures: string[],
): LifecycleIdentity | null {
  const maybeMode = exactScalar(
    frontmatter,
    "lifecycle_mode",
    relativePath,
    failures,
  );
  const maybePhaseLifecycleId = exactScalar(
    frontmatter,
    "phase_lifecycle_id",
    relativePath,
    failures,
  );
  if (maybeMode === null || maybePhaseLifecycleId === null) {
    return null;
  }
  return { mode: maybeMode, phaseLifecycleId: maybePhaseLifecycleId };
}
