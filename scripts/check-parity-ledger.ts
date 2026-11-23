#!/usr/bin/env bun

import { execFileSync } from "node:child_process";
import { existsSync, lstatSync, readFileSync, realpathSync } from "node:fs";
import path from "node:path";
import { isDeepStrictEqual } from "node:util";
import { readRegtestCase, REGTEST_CORPUS } from "./knots-regtest/regtest-case.ts";
import { readRegtestHistory, summarizeAcceptedTips, type RegtestTip } from "./knots-regtest/regtest-history.ts";
import { isRejectionState, normalizeRejection, readRejectionCorpus, REJECTION_NORMALIZATION,
  type RejectionRun } from "./knots-regtest/rejections.ts";
import { readStateCorpus, verifyState } from "./knots-regtest/state.ts";
import { QUALIFICATION_FORMAT } from "./knots-regtest/receipt.ts";
import { BRANCH_CLASSIFICATION, BRANCH_FORMAT, readBranches } from "./knots-regtest/branches.ts";

// Shallow CI checkouts may omit tag objects; the fixed label and both commit identities remain authoritative.
const BASELINE_TAG = "29.3.knots20260210";
export const BASELINE_COMMIT = "a9aee730466ac67d35a3c03ee24676be5e045878";
// A changed cardinality requires an explicit new requirement approval, not silent ledger drift.
const REQUIREMENT_COUNT = 30;
const CDIF_01 = "v2-6-cdif-01";
const CDIF_02 = "v2-6-cdif-02";
const CDIF_03 = "v2-6-cdif-03";
const CDIF_04 = "v2-6-cdif-04";
const CDIF_05 = "v2-6-cdif-05";
const CREC_01 = "v2-6-crec-01";
const SHA256 = /^[0-9a-f]{64}$/;
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
  checkoutTree: string;
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

function tipMatches(tip: unknown, height: number, hash: string): boolean {
  return isRecord(tip) && tip.height === height && tip.hash === hash;
}

function checkDifferentialBuild(
  id: string, record: RecordValue, inputs: Pick<LedgerInputs, "baseline">, failures: string[],
) {
  const knotsBuild = record.knots_build;
  const openBuild = record.open_bitcoin_build;
  if (record.format !== `open-bitcoin-${id.slice("v2-6-".length)}-v1` || record.source_tree !== inputs.baseline.checkoutTree
    || !isRecord(knotsBuild) || !isRecord(openBuild)
    || typeof knotsBuild.archive_sha256 !== "string" || !SHA256.test(knotsBuild.archive_sha256)
    || typeof knotsBuild.cmake_cache_sha256 !== "string" || !SHA256.test(knotsBuild.cmake_cache_sha256)
    || !strings(knotsBuild.configure_options)?.includes("-DBUILD_DAEMON=ON")
    || !strings(knotsBuild.build_options)?.includes("bitcoind")
    || typeof openBuild.source_commit !== "string" || !/^[0-9a-f]{40}$/.test(openBuild.source_commit)
    || typeof openBuild.binary_sha256 !== "string" || !SHA256.test(openBuild.binary_sha256)) {
    failures.push(`${id}: receipt lacks the pinned tree or daemon build inputs`);
  }
}

const PROCESS_FLAGS = {
  knots: ["-assumevalid=0", "-networkactive=0", "-listen=0"],
  open_bitcoin: ["-blockfilterindex=1"],
} as const;
type ProcessName = keyof typeof PROCESS_FLAGS;

function matchesProcess(
  result: unknown, name: ProcessName, genesisHash: string, finalTip: RegtestTip,
  initialTip = { height: 0, hash: genesisHash },
): result is RecordValue {
  if (!isRecord(result)) return false;
  const flags = strings(result.flags);
  return !!flags && ["-chain=regtest", "-rpcbind=127.0.0.1", ...PROCESS_FLAGS[name]].every((flag) => flags.includes(flag))
    && tipMatches(result.initial_tip, initialTip.height, initialTip.hash) && tipMatches(result.final_tip, finalTip.height, finalTip.hash)
    && result.peer_count === 0 && result.exit_status === 0;
}

function checkDifferentialProcesses(
  id: string, record: RecordValue, genesisHash: string, finalTip: RegtestTip,
  hasExpectedObservations: (result: RecordValue) => boolean, failures: string[],
) {
  for (const name of ["knots", "open_bitcoin"] as const) {
    const result = record[name];
    if (!matchesProcess(result, name, genesisHash, finalTip) || !hasExpectedObservations(result)) {
      failures.push(`${id}: receipt lacks the expected isolated ${name} process outcome`);
    }
  }
}

function matchesRegtestCorpus(record: RecordValue, corpusSha256: string) {
  return record.network === "regtest" && record.corpus === REGTEST_CORPUS && record.corpus_sha256 === corpusSha256;
}

function checkCDIF01Receipt(record: RecordValue, inputs: LedgerInputs, failures: string[]) {
  let evidence;
  try {
    evidence = readRegtestCase(inputs.readTextFile);
  } catch {
    failures.push(`${CDIF_01}: cannot read the canonical height-one Regtest input`);
    return;
  }
  if (!matchesRegtestCorpus(record, evidence.corpusSha256) || record.block_sha256 !== evidence.blockSha256) {
    failures.push(`${CDIF_01}: receipt does not match the canonical height-one Regtest input`);
  }
  checkDifferentialProcesses(CDIF_01, record, evidence.genesisHash, { height: 1, hash: evidence.blockHash },
    (result) => result.submitblock === null, failures);
}

function checkCDIF02Receipt(record: RecordValue, inputs: LedgerInputs, failures: string[]) {
  let history;
  try {
    history = readRegtestHistory(inputs.readTextFile);
  } catch {
    failures.push(`${CDIF_02}: cannot read canonical Regtest maturity and halving inputs`);
    return;
  }
  if (!matchesRegtestCorpus(record, history.corpusSha256)
    || !isDeepStrictEqual(record.maturity, history.monetaryFacts.maturity)
    || !isDeepStrictEqual(record.subsidy_boundaries, history.monetaryFacts.subsidy_boundaries)) {
    failures.push(`${CDIF_02}: receipt does not match the canonical Regtest monetary evidence`);
  }
  const expected = summarizeAcceptedTips(history.blocks);
  const finalTip = history.blocks[history.blocks.length - 1];
  checkDifferentialProcesses(CDIF_02, record, history.genesisHash, finalTip, (result) =>
    result.accepted_count === expected.accepted_count
    && result.accepted_tips_sha256 === expected.accepted_tips_sha256
    && isDeepStrictEqual(result.checkpoints, expected.checkpoints), failures);
}

function matchesRejectionCase(
  result: unknown, candidate: RejectionRun["cases"][number],
) {
  if (!isRecord(result) || result.name !== candidate.name || result.height !== candidate.height
    || result.candidate_hash !== candidate.hash || result.wire_sha256 !== candidate.wireSha256
    || !isRejectionState(result.before, candidate.height - 1, candidate.parentHash)
    || !isDeepStrictEqual(result.before, result.after)) return false;
  try {
    return normalizeRejection(result.submitblock) === candidate.expectedReason
      && result.normalized_reason === candidate.expectedReason;
  } catch {
    return false;
  }
}

function matchesRejectionCases(results: unknown[], candidates: RejectionRun["cases"]) {
  return results.length === candidates.length
    && candidates.every((candidate, index) => matchesRejectionCase(results[index], candidate));
}

function checkCDIF03Receipt(record: RecordValue, inputs: LedgerInputs, failures: string[]) {
  let corpus;
  try {
    corpus = readRejectionCorpus(inputs.readTextFile);
  } catch {
    failures.push(`${CDIF_03}: cannot read the canonical Regtest rejection inputs`);
    return;
  }
  if (!matchesRegtestCorpus(record, corpus.corpusSha256) || record.normalization !== REJECTION_NORMALIZATION) {
    failures.push(`${CDIF_03}: receipt does not match the Regtest rejection corpus or normalization`);
  }
  for (const name of ["knots", "open_bitcoin"] as const) {
    const process = record[name];
    if (!isRecord(process) || !Array.isArray(process.runs) || process.runs.length !== corpus.runs.length) {
      failures.push(`${CDIF_03}: receipt lacks the isolated ${name} runs`);
      continue;
    }
    for (const [index, group] of corpus.runs.entries()) {
      const result = process.runs[index];
      const finalTip = group.blocks[group.blocks.length - 1];
      if (!matchesProcess(result, name, corpus.genesisHash, finalTip) || result.name !== group.name
        || !strings(result.flags)?.includes("-blockfilterindex=1") || !Array.isArray(result.cases)
        || !matchesRejectionCases(result.cases, group.cases)) {
        failures.push(`${CDIF_03}: invalid ${name} rejection evidence for ${group.name}`);
      }
    }
  }
}

function checkCDIF04Receipt(record: RecordValue, inputs: LedgerInputs, failures: string[]) {
  let corpus;
  try {
    corpus = readStateCorpus(inputs.readTextFile);
  } catch {
    failures.push(`${CDIF_04}: cannot read canonical Regtest state inputs`);
    return;
  }
  if (!matchesRegtestCorpus(record, corpus.corpusSha256)
    || record.expectation_sha256 !== corpus.expectationSha256
    || record.open_bitcoin_observer_sha256 !== corpus.observerSha256) {
    failures.push(`${CDIF_04}: receipt does not match its state corpus or offline observer`);
  }
  checkDifferentialProcesses(CDIF_04, record, corpus.genesisHash, corpus.expected.tip, (result) => {
    if (!strings(result.flags)?.includes("-blockfilterindex=1")) return false;
    try {
      verifyState(result, corpus, CDIF_04);
      return true;
    } catch {
      return false;
    }
  }, failures);
}

export function checkQualificationRecord(
  record: RecordValue, inputs: Pick<LedgerInputs, "baseline" | "readTextFile">,
): string[] {
  const failures: string[] = [];
  checkDifferentialBuild(CDIF_05, record, inputs, failures);
  const { readTextFile } = inputs;
  let rejections, state;
  try {
    rejections = readRejectionCorpus(readTextFile);
    state = readStateCorpus(readTextFile);
  } catch {
    return [`${CDIF_05}: missing or invalid qualification inputs`];
  }
  if (record.format !== QUALIFICATION_FORMAT || record.status !== "passed" || record.source_commit !== BASELINE_COMMIT
    || record.source_tree !== inputs.baseline.checkoutTree
    || typeof record.binary_sha256 !== "string" || !SHA256.test(record.binary_sha256)
    || !isRecord(record.knots_build) || !/^Bitcoin Knots daemon version v29\.3(?:\.0)?\.knots20260210$/.test(String(record.knots_build.version))
    || !matchesRegtestCorpus(record, state.corpusSha256) || record.normalization !== REJECTION_NORMALIZATION
    || record.expectation_sha256 !== state.expectationSha256 || record.open_bitcoin_observer_sha256 !== state.observerSha256) {
    failures.push(`${CDIF_05}: qualification does not match its pinned oracle, corpus or observer`);
  }
  for (const name of ["knots", "open_bitcoin"] as const) {
    const daemon = record[name];
    if (!isRecord(daemon) || !Array.isArray(daemon.runs) || daemon.runs.length !== rejections.runs.length) {
      failures.push(`${CDIF_05}: missing or incomplete ${name} process runs`);
      continue;
    }
    for (const [index, expected] of rejections.runs.entries()) {
      const run = daemon.runs[index];
      const final = expected.blocks[expected.blocks.length - 1];
      if (!matchesProcess(run, name, rejections.genesisHash, final) || run.name !== expected.name
        || !strings(run.flags)?.includes("-blockfilterindex=1")
        || !isDeepStrictEqual(run.accepted, expected.blocks.map((block) => block.hash))
        || !Array.isArray(run.cases) || !matchesRejectionCases(run.cases, expected.cases)) {
        failures.push(`${CDIF_05}: incomplete or mismatched ${name} cases in ${expected.name}`);
      }
    }
    const final = daemon.runs.at(-1);
    try {
      if (!isRecord(final)) throw new Error("missing final observation");
      verifyState(final, state, name);
    } catch {
      failures.push(`${CDIF_05}: missing or mismatched ${name} final state`);
    }
  }
  return failures;
}

function checkCDIF05Receipt(record: RecordValue, inputs: LedgerInputs, failures: string[]) {
  failures.push(...checkQualificationRecord(record, inputs));
}

export function checkBranchRecord(
  record: RecordValue, inputs: Pick<LedgerInputs, "baseline" | "readTextFile">,
): string[] {
  const failures: string[] = [];
  checkDifferentialBuild(CREC_01, record, inputs, failures);
  const corpus = readBranches(inputs.readTextFile);
  if (record.format !== BRANCH_FORMAT || record.status !== "passed" || record.source_commit !== BASELINE_COMMIT
    || typeof record.binary_sha256 !== "string" || !SHA256.test(record.binary_sha256)
    || !matchesRegtestCorpus(record, corpus.corpusSha256) || record.branch_sha256 !== corpus.branchSha256) {
    failures.push(`${CREC_01}: receipt does not bind the pinned oracle and branch bytes`);
  }
  for (const name of ["knots", "open_bitcoin"] as const) {
    const process = record[name];
    if (!isRecord(process) || !Array.isArray(process.runs) || process.runs.length !== corpus.runs.length) {
      failures.push(`${CREC_01}: missing or incomplete ${name} branch runs`);
      continue;
    }
    for (const [index, expected] of corpus.runs.entries()) {
      const run = process.runs[index];
      const finalTip = expected.steps[expected.steps.length - 1].tip;
      const steps = expected.steps.map((step) => ({
        submitted: step.submit, hash: step.block.hash,
        submitblock: step.submit === step.active ? null : "inconclusive",
        active: { height: step.tip.height, hash: step.tip.hash },
      }));
      if (!matchesProcess(run, name, corpus.genesisHash, finalTip) || run.name !== expected.name
        || !isDeepStrictEqual(run.steps, steps)) {
        failures.push(`${CREC_01}: invalid ${name} branch observations for ${expected.name}`);
      }
    }
    const restart = process.restart;
    const active = corpus.blocks.a2;
    if (!isRecord(restart) || !tipMatches(restart.retained_tip, active.height, active.hash)) {
      failures.push(`${CREC_01}: ${name} did not retain the active branch at restart`);
      continue;
    }
    if (name === "open_bitcoin") {
      if (!matchesProcess(restart, name, corpus.genesisHash, active, active) || Object.hasOwn(restart, "ambiguous")) {
        failures.push(`${CREC_01}: invalid Open Bitcoin restart observation`);
      }
      continue;
    }
    const ambiguous = restart.ambiguous;
    const candidates = [corpus.blocks.b2.hash, corpus.blocks.c2.hash].sort();
    if (!isRecord(ambiguous) || ambiguous.classification !== BRANCH_CLASSIFICATION
      || !isDeepStrictEqual(ambiguous.candidates, candidates) || typeof ambiguous.selected !== "string"
      || !candidates.includes(ambiguous.selected) || ambiguous.chainwork !== corpus.tiedChainwork
      || !matchesProcess(restart, name, corpus.genesisHash, { height: active.height, hash: ambiguous.selected }, active)) {
      failures.push(`${CREC_01}: invalid Knots restart fallback classification`);
    }
  }
  return failures;
}

function checkCREC01Receipt(record: RecordValue, inputs: LedgerInputs, failures: string[]) {
  failures.push(...checkBranchRecord(record, inputs));
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

    const checkRegtestReceipt = id === CDIF_01 ? checkCDIF01Receipt : id === CDIF_02 ? checkCDIF02Receipt
      : id === CDIF_03 ? checkCDIF03Receipt : id === CDIF_04 ? checkCDIF04Receipt : id === CDIF_05 ? checkCDIF05Receipt : id === CREC_01 ? checkCREC01Receipt : null;
    if (checkRegtestReceipt && value.status === "done" && !classes?.includes("knots_daemon_differential")) {
      failures.push(`${id}: done runner requires actual Knots daemon evidence`);
    }
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
            } else if (checkRegtestReceipt) {
              if (id !== CDIF_05 && id !== CREC_01) checkDifferentialBuild(id, record, inputs, failures);
              checkRegtestReceipt(record, inputs, failures);
            }
          } catch (error) {
            if (!(error instanceof SyntaxError)) throw error;
            failures.push(`${id}: actual Knots differential receipt is invalid JSON`);
          }
        }
      }
      if (typeof value.knots_binary_sha256 !== "string" || !SHA256.test(value.knots_binary_sha256)) {
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
    checkoutTree: git(knotsRoot, ["rev-parse", "HEAD^{tree}"]),
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
