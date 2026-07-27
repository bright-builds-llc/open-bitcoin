import {
  existsSync,
  readFileSync,
  readdirSync,
  statSync,
} from "node:fs";
import path from "node:path";
import { PHASES_DIR, PhaseCorpus, Artifact } from "./constants.ts";
import { extractFrontmatter } from "./parsing.ts";
import { parseLifecycleIdentity } from "./lifecycle.ts";

export function readRequiredText(
  rootDir: string,
  relativePath: string,
  failures: string[],
): string {
  const absolutePath = path.join(rootDir, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required corpus file ${relativePath}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}

export function loadPhaseCorpora(
  rootDir: string,
  activePhases: Set<number>,
  failures: string[],
): PhaseCorpus[] {
  const phasesDir = path.join(rootDir, PHASES_DIR);
  if (!existsSync(phasesDir)) {
    failures.push(`missing required corpus directory ${PHASES_DIR}`);
    return [];
  }

  const directories = readdirSync(phasesDir)
    .filter((entry) => statSync(path.join(phasesDir, entry)).isDirectory())
    .sort();
  const corpora: PhaseCorpus[] = [];

  for (const phase of [...activePhases].sort((left, right) => left - right)) {
    const matches = directories.filter((entry) =>
      entry.startsWith(`${phase}-`),
    );
    if (matches.length > 1) {
      failures.push(
        `active Phase ${phase} resolves to multiple phase directories: ${matches.join(", ")}`,
      );
    }
    const maybeDirectory = matches[0];
    if (maybeDirectory === undefined) {
      continue;
    }
    corpora.push(
      loadPhaseCorpus(rootDir, phase, maybeDirectory, failures),
    );
  }

  return corpora;
}

export function loadPhaseCorpus(
  rootDir: string,
  phase: number,
  directory: string,
  failures: string[],
): PhaseCorpus {
  const relativeDirectory = path.join(PHASES_DIR, directory);
  const absoluteDirectory = path.join(rootDir, relativeDirectory);
  const entries = readdirSync(absoluteDirectory).sort();
  const summaryNames = entries.filter((entry) =>
    new RegExp(`^${phase}-\\d+-SUMMARY\\.md$`).test(entry),
  );
  const verificationNames = entries.filter((entry) =>
    new RegExp(`^${phase}-VERIFICATION\\.md$`).test(entry),
  );
  const contextPath = path.join(relativeDirectory, `${phase}-CONTEXT.md`);
  const hasArtifacts =
    summaryNames.length > 0 || verificationNames.length > 0;
  const contextText = existsSync(path.join(rootDir, contextPath))
    ? readFileSync(path.join(rootDir, contextPath), "utf8")
    : "";

  if (hasArtifacts && contextText === "") {
    failures.push(`active Phase ${phase} artifacts are missing ${contextPath}`);
  }

  const maybeContextFrontmatter = hasArtifacts
    ? extractFrontmatter(contextText, contextPath, failures)
    : null;
  const lifecycle =
    maybeContextFrontmatter === null
      ? null
      : parseLifecycleIdentity(
          maybeContextFrontmatter,
          contextPath,
          failures,
        );

  return {
    directory: relativeDirectory,
    lifecycle,
    phase,
    summaries: summaryNames.map((name) =>
      loadArtifact(rootDir, path.join(relativeDirectory, name), failures),
    ),
    verifications: verificationNames.map((name) =>
      loadArtifact(rootDir, path.join(relativeDirectory, name), failures),
    ),
  };
}

export function loadArtifact(
  rootDir: string,
  relativePath: string,
  failures: string[],
): Artifact {
  const text = readFileSync(path.join(rootDir, relativePath), "utf8");
  return {
    frontmatter: extractFrontmatter(text, relativePath, failures),
    relativePath,
    text,
  };
}
