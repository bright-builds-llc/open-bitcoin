#!/usr/bin/env bun

import { execFileSync } from "node:child_process";
import { existsSync, lstatSync, readFileSync, realpathSync } from "node:fs";
import path from "node:path";

// Shallow CI checkouts may omit tag objects; the fixed label and both commit identities remain authoritative.
const BASELINE_TAG = "29.3.knots20260210";
const BASELINE_COMMIT = "a9aee730466ac67d35a3c03ee24676be5e045878";
// A changed cardinality requires an explicit new requirement approval, not silent ledger drift.
const REQUIREMENT_COUNT = 30;
const CLASSES = new Set([
  "source_derived",
  "seeded_historical",
  "local_product",
  "knots_daemon_differential",
]);
const STATUSES = new Set(["planned", "in_progress", "done", "deferred", "out_of_scope"]);

type RecordValue = Record<string, unknown>;

/** Gitlink and checked-out Knots identities used to check the declared baseline. */
export type BaselineFacts = {
  gitlinkCommit: string;
  checkoutCommit: string;
};

/** Pure validation inputs, with file access supplied by the caller. */
export type LedgerInputs = {
  requirementsText: string;
  index: unknown;
  baseline: BaselineFacts;
  isRegularFile: (relativePath: string) => boolean;
  readTextFile: (relativePath: string) => string;
};

function isRecord(value: unknown): value is RecordValue {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function strings(value: unknown): string[] | null {
  return Array.isArray(value) && value.every((item) => typeof item === "string")
    ? value
    : null;
}

function validRelativePath(value: string): boolean {
  return value.length > 0 && value.trim() === value && !value.includes("\\")
    && !path.posix.isAbsolute(value) && path.posix.normalize(value) === value
    && !value.split("/").some((part) => part === "." || part === "..");
}

function checkFiles(
  label: string,
  maybePaths: unknown,
  isRegularFile: LedgerInputs["isRegularFile"],
  failures: string[],
): string[] {
  const paths = strings(maybePaths);
  if (!paths) {
    failures.push(`${label}: expected an array of paths`);
    return [];
  }
  const seen = new Set<string>();
  for (const relativePath of paths) {
    if (!validRelativePath(relativePath)) {
      failures.push(`${label}: unsafe or non-normalized path ${JSON.stringify(relativePath)}`);
      continue;
    }
    if (seen.has(relativePath)) failures.push(`${label}: duplicate path ${relativePath}`);
    seen.add(relativePath);
    if (!isRegularFile(relativePath)) failures.push(`${label}: missing or non-regular file ${relativePath}`);
  }
  return paths;
}

function requirementIds(text: string, failures: string[]): string[] {
  const entries = [...text.matchAll(/^- \*\*([A-Z]+-\d{2})\*\*: (.+)$/gm)];
  if (/^- \[[ x]\] \*\*[A-Z]+-\d{2}\*\*:/m.test(text)) {
    failures.push("requirements: completion checkboxes belong in the parity ledger");
  }
  if (entries.length !== REQUIREMENT_COUNT) {
    failures.push(`requirements: expected ${REQUIREMENT_COUNT} approved descriptions; found ${entries.length}`);
  }
  const seen = new Set<string>();
  for (const [, id, description] of entries) {
    if (seen.has(id)) failures.push(`requirements: duplicate ID ${id}`);
    if (!description.trim()) failures.push(`requirements: empty description for ${id}`);
    seen.add(id);
  }
  return entries.map(([, id]) => id);
}

/** Validate active v2.6 ownership and evidence without reading GSD lifecycle state. */
export function checkParityLedger(inputs: LedgerInputs): string[] {
  const failures: string[] = [];
  const ids = requirementIds(inputs.requirementsText, failures);
  const knownIds = new Set(ids);
  const index = inputs.index;
  if (!isRecord(index)) return [...failures, "ledger: expected a JSON object"];

  if (index.baseline !== BASELINE_TAG) failures.push(`ledger: baseline must be ${BASELINE_TAG}`);
  if (inputs.baseline.gitlinkCommit !== BASELINE_COMMIT) failures.push("Knots gitlink differs from the approved commit");
  if (inputs.baseline.checkoutCommit !== BASELINE_COMMIT) failures.push("Knots checkout differs from the approved commit");

  const checklist = index.checklist;
  if (!isRecord(checklist) || !Array.isArray(checklist.surfaces)) {
    return [...failures, "ledger: checklist.surfaces must be an array"];
  }

  const owners = new Map<string, string[]>();
  const surfaceIds = new Set<string>();
  for (const value of checklist.surfaces) {
    if (!isRecord(value) || typeof value.id !== "string" || !value.id.startsWith("v2-6-")) continue;
    const id = value.id;
    if (surfaceIds.has(id)) failures.push(`ledger: duplicate v2.6 surface ${id}`);
    surfaceIds.add(id);
    if (typeof value.status !== "string" || !STATUSES.has(value.status)) {
      failures.push(`${id}: invalid status`);
    }
    const requirements = strings(value.requirements);
    if (!requirements || requirements.length === 0) {
      failures.push(`${id}: expected nonempty requirements`);
    } else {
      for (const requirement of requirements) {
        if (!knownIds.has(requirement)) failures.push(`${id}: unknown requirement ${requirement}`);
        owners.set(requirement, [...(owners.get(requirement) ?? []), id]);
      }
    }

    const evidence = strings(value.evidence);
    const classes = strings(value.evidence_classes);
    if (!evidence) failures.push(`${id}: evidence must be an array`);
    if (!classes) failures.push(`${id}: evidence_classes must be an array`);
    if (classes) {
      const seen = new Set<string>();
      for (const evidenceClass of classes) {
        if (!CLASSES.has(evidenceClass)) failures.push(`${id}: unknown evidence class ${evidenceClass}`);
        if (seen.has(evidenceClass)) failures.push(`${id}: duplicate evidence class ${evidenceClass}`);
        seen.add(evidenceClass);
      }
    }

    if (value.status === "planned") {
      if (evidence?.length) failures.push(`${id}: planned surface cannot claim evidence`);
      if (classes?.length) failures.push(`${id}: planned surface cannot claim an evidence class`);
    }
    if (value.status === "done") {
      if (!evidence?.length) failures.push(`${id}: done surface needs evidence`);
      if (!classes?.length) failures.push(`${id}: done surface needs an evidence class`);
      if (!evidence?.some((file) => !file.startsWith(".planning/"))) {
        failures.push(`${id}: done surface needs evidence outside historical GSD records`);
      }
      const upstream = value.upstream;
      const sources = isRecord(upstream) ? strings(upstream.sources) : null;
      if (!sources?.length) {
        failures.push(`${id}: done surface needs Knots source anchors`);
      } else {
        for (const source of sources) {
          if (!source.startsWith("packages/bitcoin-knots/")) {
            failures.push(`${id}: upstream source is outside pinned Knots: ${source}`);
          }
        }
        checkFiles(`${id} upstream`, sources, inputs.isRegularFile, failures);
      }
    }
    if (evidence?.length) checkFiles(`${id} evidence`, evidence, inputs.isRegularFile, failures);

    if (classes?.includes("knots_daemon_differential")) {
      const receipt = value.differential_receipt;
      if (typeof receipt !== "string" || !evidence?.includes(receipt)) {
        failures.push(`${id}: actual Knots differential requires a referenced receipt`);
      } else {
        if (!/^docs\/parity\/receipts\/[a-z0-9][a-z0-9._-]*\.json$/.test(receipt)) {
          failures.push(`${id}: actual Knots differential receipt must be a parity JSON receipt`);
        }
        checkFiles(`${id} differential receipt`, [receipt], inputs.isRegularFile, failures);
        if (validRelativePath(receipt) && inputs.isRegularFile(receipt)) {
          try {
            const record = JSON.parse(inputs.readTextFile(receipt)) as unknown;
            if (!isRecord(record) || record.source_commit !== BASELINE_COMMIT
              || record.binary_sha256 !== value.knots_binary_sha256 || record.status !== "passed") {
              failures.push(`${id}: actual Knots differential receipt lacks pinned source, binary and passed outcome`);
            }
          } catch (error) {
            if (!(error instanceof SyntaxError)) throw error;
            failures.push(`${id}: actual Knots differential receipt is invalid JSON`);
          }
        }
      }
      if (typeof value.knots_binary_sha256 !== "string" || !/^[0-9a-f]{64}$/.test(value.knots_binary_sha256)) {
        failures.push(`${id}: actual Knots differential requires a binary SHA-256`);
      }
    }
  }

  for (const id of ids) {
    const mapped = owners.get(id) ?? [];
    if (mapped.length !== 1) failures.push(`requirements: ${id} must have exactly one v2.6 surface; found ${mapped.length}`);
  }
  return failures;
}

function git(root: string, args: string[]): string {
  return execFileSync("git", ["-C", root, ...args], { encoding: "utf8" }).trim();
}

function regularFile(root: string, relativePath: string): boolean {
  const full = path.resolve(root, relativePath);
  if (!full.startsWith(`${root}${path.sep}`) || !existsSync(full)) return false;
  const real = realpathSync(full);
  return real.startsWith(`${root}${path.sep}`) && lstatSync(full).isFile();
}

/** Check the current checkout, including the materialized pinned Knots submodule. */
export function checkCurrentParityLedger(root = git(process.cwd(), ["rev-parse", "--show-toplevel"])): string[] {
  const gitlink = git(root, ["ls-files", "--stage", "--", "packages/bitcoin-knots"]);
  const maybeGitlink = /^160000 ([0-9a-f]{40}) 0\tpackages\/bitcoin-knots$/.exec(gitlink)?.[1];
  const knotsRoot = path.join(root, "packages/bitcoin-knots");
  const facts: BaselineFacts = {
    gitlinkCommit: maybeGitlink ?? "missing",
    checkoutCommit: git(knotsRoot, ["rev-parse", "HEAD"]),
  };
  const index = JSON.parse(readFileSync(path.join(root, "docs/parity/index.json"), "utf8")) as unknown;
  const requirementsText = readFileSync(path.join(root, "docs/parity/requirements.md"), "utf8");
  return checkParityLedger({ requirementsText, index, baseline: facts,
    isRegularFile: (relativePath) => regularFile(root, relativePath),
    readTextFile: (relativePath) => readFileSync(path.join(root, relativePath), "utf8") });
}

if (import.meta.main) {
  try {
    const failures = checkCurrentParityLedger();
    if (failures.length) {
      for (const failure of failures) console.error(`parity-ledger: ${failure}`);
      process.exitCode = 1;
    } else {
      console.log("Parity ledger verified: 30 v2.6 requirements have exactly one surface each.");
    }
  } catch (error) {
    console.error(`parity-ledger: ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 1;
  }
}
