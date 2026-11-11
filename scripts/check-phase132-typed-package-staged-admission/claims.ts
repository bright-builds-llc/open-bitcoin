import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { NEGATED_BOUNDARY_VERB, WITHOUT_BOUNDARY_VERB, DEFERRED_BOUNDARY_PREDICATE, NEGATED_SUPPORT_BOUNDARY_PREDICATE, OUTSIDE_SCOPE_BOUNDARY_PREDICATE, PREFIXED_DEFERRED_BOUNDARY } from "./constants.ts";
import { readTarget } from "./filesystem.ts";

export function hasExplicitClaimBoundary(clause: string, claim: string): boolean {
  const claimIndex = clause.indexOf(claim);
  if (claimIndex === -1) return false;

  const before = clause.slice(0, claimIndex);
  const after = clause.slice(claimIndex + claim.length);
  return (
    NEGATED_BOUNDARY_VERB.test(before) ||
    WITHOUT_BOUNDARY_VERB.test(before) ||
    PREFIXED_DEFERRED_BOUNDARY.test(before) ||
    DEFERRED_BOUNDARY_PREDICATE.test(after) ||
    NEGATED_SUPPORT_BOUNDARY_PREDICATE.test(after) ||
    OUTSIDE_SCOPE_BOUNDARY_PREDICATE.test(after)
  );
}

export function claimClauses(paragraph: string): string[] {
  const sentences = paragraph.match(/[^.!?]+(?:[.!?]+|$)/g) ?? [];
  return sentences.flatMap((sentence) => {
    const clauses = sentence.split(
      /\s*(?:;|—|\||\b(?:but|however|whereas|while|although|though|yet)\b)\s*/i,
    );
    return clauses.flatMap((clause) =>
      clause.split(/\r?\n(?=\s*(?:[-*+]|\d+\.)\s)/)
    );
  });
}

export function checkDeterministicScope(repoRoot: string, failures: string[]): void {
  const checker = readTarget(
    repoRoot,
    "scripts/check-phase132-typed-package-staged-admission.ts",
  );
  const forbidden = [
    "fetch" + "(",
    "Bun." + "spawn",
    "node:" + "child_process",
    "http" + "://",
    "https" + "://",
  ];
  if (forbidden.some((needle) => checker.includes(needle))) {
    failures.push(
      "P132 deterministic scope: checker must remain local and network-free",
    );
  }
}
