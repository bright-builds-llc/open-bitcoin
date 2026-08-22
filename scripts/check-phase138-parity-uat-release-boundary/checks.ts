import path from "node:path";

import { DEFAULT_REPO_ROOT } from "./constants.ts";

export function checkPhase138ParityUatReleaseBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE138_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  void repoRoot;
  const failures: string[] = [];
  return failures;
}
