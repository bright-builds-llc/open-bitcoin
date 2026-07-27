import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

function compareNames(left: string, right: string): number {
  if (left < right) {
    return -1;
  }
  if (left > right) {
    return 1;
  }
  return 0;
}

function descendantFiles(directory: string): string[] {
  const files: string[] = [];
  const entries = readdirSync(directory, { withFileTypes: true }).sort((left, right) =>
    compareNames(left.name, right.name),
  );

  for (const entry of entries) {
    const entryPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      files.push(...descendantFiles(entryPath));
      continue;
    }
    if (entry.isFile()) {
      files.push(entryPath);
    }
  }

  return files;
}

function gitTracksPaths(repoRoot: string, relativePaths: string[]): boolean {
  const gitResult = Bun.spawnSync(["git", "ls-files", "--error-unmatch", "--", ...relativePaths], {
    cwd: repoRoot,
    stderr: "pipe",
    stdout: "pipe",
  });
  return gitResult.success;
}

function trackedDescendantFiles(repoRoot: string, directory: string): string[] {
  const candidates = descendantFiles(directory);
  const relativePaths = candidates.map((candidate) =>
    path.relative(repoRoot, candidate).replaceAll(path.sep, "/"),
  );
  if (relativePaths.length === 0 || gitTracksPaths(repoRoot, relativePaths)) {
    return candidates;
  }

  return candidates.filter((_, index) => gitTracksPaths(repoRoot, [relativePaths[index]]));
}

/**
 * Reads a stable source root together with its same-named module tree.
 *
 * Rust `foo.rs` + `foo/`, Rust `tests.rs` + `tests/`, TypeScript roots, and
 * shell roots all use the same extension-free directory convention. Child
 * files are traversed recursively in deterministic bytewise path order.
 */
export function readSourceCorpus(repoRoot: string, relativePath: string): string {
  const rootPath = path.join(repoRoot, relativePath);
  const extension = path.extname(relativePath);
  const childDirectory = extension === "" ? rootPath : rootPath.slice(0, -extension.length);
  const corpusPaths = [rootPath];

  if (childDirectory !== rootPath && existsSync(childDirectory)) {
    corpusPaths.push(...trackedDescendantFiles(repoRoot, childDirectory));
  }

  return corpusPaths
    .map((sourcePath) => readSourceRoot(repoRoot, path.relative(repoRoot, sourcePath)))
    .join("\n");
}

/** Reads only the named stable root without following its module directory. */
export function readSourceRoot(repoRoot: string, relativePath: string): string {
  return readFileSync(path.join(repoRoot, relativePath), "utf8");
}
