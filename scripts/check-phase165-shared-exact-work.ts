#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { closeSync, constants, fstatSync, lstatSync, openSync, readSync } from "node:fs";
import { resolve } from "node:path";
import { INPUT_FILES as IDENTITY_INPUTS, historicalFixtureAdapter, HISTORICAL_TOOLING_INVERSE, maybeRestoreReviewedSource } from "./check-phase164-canonical-identity.ts";
import { regions } from "./check-phase164-canonical-identity/regions.ts";
import { CLAIM_FILES, checkCurrentClaims } from "./check-phase164-canonical-identity/claims.ts";
import { historicalToolingView, MANIFEST_FILE, CURRENT_VERIFY_ADDITIONS, verificationHistoricalBase } from "./check-phase164-canonical-identity/historical-tooling.ts";
import { compact, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { supportedOwnership } from "./check-phase159-filter-rpcs/evidence.ts";
import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";

export type Finding = { file: string; rule: string; detail: string };
type Fail = (file: string, rule: string, detail: string) => void;
export const MAX_FILE_BYTES = 512_000;
const INPUT_INVENTORY_SHA = "625975bfe720b4d08d05429a31a0c2dd0d76cb044c0675617754ffa4cc1ce6bb";
export const INPUT_FILES = [...new Set([
  ...IDENTITY_INPUTS, ...SOURCE_CONTRACTS.map(row => row[0]),
  ...HISTORICAL_IDENTITIES.map(row => row[0]), CORPUS_FILE, WORK_CLAIM_FILE,
])];

/** Fixed regular inputs only: no corpus traversal, writes, network or checkout fallback. */
export function loadInputs(maybeRoot?: string): Map<string, string> {
  assertInputInventory(INPUT_FILES);
  const root = maybeRoot ?? resolve(import.meta.dir, "..");
  return new Map(INPUT_FILES.map(file => [file, readBounded(resolve(root, file))]));
}

/** Manifest data cannot broaden the reviewed read set, even before source validation. */
export function assertInputInventory(files: readonly string[]): void {
  if (sha(JSON.stringify(files)) !== INPUT_INVENTORY_SHA) {
    throw new Error("fixed input inventory changed before any source read");
  }
}

function readBounded(file: string): string {
  if (!lstatSync(file).isFile()) throw new Error(`${file}: regular input required`);
  const descriptor = openSync(file, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    if (!fstatSync(descriptor).isFile()) throw new Error(`${file}: regular input required`);
    const bytes = Buffer.alloc(MAX_FILE_BYTES + 1);
    let length = 0;
    while (length < bytes.length) {
      const count = readSync(descriptor, bytes, length, bytes.length - length, null);
      if (count === 0) break;
      length += count;
    }
    if (length > MAX_FILE_BYTES) throw new Error(`${file}: fixed byte limit exceeded`);
    return bytes.subarray(0, length).toString("utf8");
  } finally { closeSync(descriptor); }
}

const sha = (source: string) => createHash("sha256").update(source).digest("hex");

/** Literal-bearing reviewed source identity, excluding only leading masked commentary. */
export function sourceIdentity(source: string): string {
  const start = rustCode(source).search(/\S/);
  return sha(start < 0 ? "" : source.slice(start));
}

/** Pure source/evidence policy. Passing does not execute Rust or qualify any daemon. */
export function checkPhase165(inputs: ReadonlyMap<string, string>): readonly Finding[] {
  const findings: Finding[] = [];
  const fail: Fail = (file, rule, detail) => findings.push({ file, rule, detail });
  for (const file of INPUT_FILES) {
    const maybeSource = inputs.get(file);
    if (!maybeSource || Buffer.byteLength(maybeSource) > MAX_FILE_BYTES) {
      fail(file, "input", "required fixed bounded input missing or oversized");
    }
  }
  if (findings.length) return findings;
  const text = (file: string) => inputs.get(file) ?? "";
  for (const [file, rule, digest] of SOURCE_CONTRACTS) {
    if (sourceIdentity(text(file)) !== digest) {
      fail(file, rule, "reviewed literal-bearing owner changed; source review and actual Rust verification required");
    }
  }
  checkEnabledTests(text, fail);
  checkCorpus(text(CORPUS_FILE), fail);
  const historyBound = checkHistory(inputs, fail);
  if (historyBound) checkCurrentClaims(text, fail);
  checkWorkClaims(text(WORK_CLAIM_FILE), fail);
  for (const file of [...CLAIM_FILES, WORK_CLAIM_FILE]) {
    for (const prose of claimTexts(file, text(file), fail)) {
      for (const claim of affirmativeScopeClaims(prose)) {
        if (claim.phase) fail(file, "claim", "later bootstrap/context/pinned-daemon qualification remains deferred");
        else if (file === WORK_CLAIM_FILE) fail(file, "claim", "unearned daemon, restart-tie or canonical acceptance claim");
      }
    }
  }
  checkRegistration(text("scripts/verify.sh"), fail);
  return findings;
}

/** JSON fields are separate prose; statuses in another row cannot qualify a phase. */
function claimTexts(file: string, source: string, fail: Fail): string[] {
  if (!file.endsWith(".json")) return [source];
  let value: unknown;
  try { value = JSON.parse(source); }
  catch { fail(file, "input", "invalid JSON claim input"); return []; }
  const strings: string[] = [];
  const collect = (value: unknown): void => {
    if (typeof value === "string") strings.push(value);
    else if (Array.isArray(value)) value.forEach(collect);
    else if (value !== null && typeof value === "object") Object.values(value).forEach(collect);
  };
  collect(value);
  return strings;
}

function checkWorkClaims(source: string, fail: Fail): void {
  for (const fact of WORK_CLAIM_FACTS) {
    if (!source.includes(fact)) fail(WORK_CLAIM_FILE, "claim", `required bounded work fact: ${fact}`);
  }
}

/** Every known subject owns only its following predicates, never unrelated context. */
function affirmativeScopeClaims(source: string): { phase: boolean }[] {
  const claims: { phase: boolean }[] = [];
  const subjects = /\b(?:Phase\s*16[678]|canonical(?=[^.!?;]*\b(?:bootstrap|reopen)\b)|(?:deterministic\s+)?(?:restart|reopen)(?=[^.!?;]*\b(?:tie|ordering|order)\b)|(?:actual\s+)?(?:pinned[- ]?)?Knots(?=[^.!?;]*\b(?:daemon|differential|qualification)\b))\b/gi;
  for (const sentence of source.replaceAll("\n", " ").split(/[.!?;](?:\s|$)|\b(?:but|however)\b/i)) {
    const plain = sentence.replace(/[`*_#>]/g, "");
    const targets = [...plain.matchAll(subjects)];
    let phaseScope = false;
    let inheritedNo = false;
    for (let index = 0; index < targets.length; index++) {
      const target = targets[index];
      const previous = targets[index - 1];
      const connector = previous ? plain.slice(previous.index + previous[0].length, target.index) : "";
      phaseScope = /^Phase\b/i.test(target[0]) || (phaseScope && !/\b(?:is|are|was|were|has|have|will|can|does|do)\b/i.test(connector));
      const predicate = plain.slice(target.index + target[0].length, targets[index + 1]?.index ?? plain.length);
      const attachedNo = (inheritedNo && !/\b(?:is|are|was|were|has|have|will|can|does|do)\b/i.test(connector)) || /(?:^|,)\s*(?:no|never|without)\s+(?:(?:actual|deterministic|whole-chain|production-service|production-funds|public-default|networking|and|or)[,\s-]*)*$/i.test(plain.slice(0, target.index));
      inheritedNo = attachedNo;
      for (const status of predicate.matchAll(/\b(?:qualified|proven|passed|delivered|complete|established|supported|accepted|deterministic|ready|achieved|available)\b/gi)) {
        const before = predicate.slice(0, status.index);
        const localPrefix = before.split(/\band\b|,/i).at(-1) ?? "";
        const denied = /\b(?:not|never)\s*(?:yet\s*)?$/i.test(localPrefix)
          || /\bnot\s+(?:an?\s+)?(?:evidence|proof|claim)\s+for\s+(?:a\s+)?$/i.test(localPrefix)
          || /\b(?:only after|until)\b/i.test(localPrefix)
          || /\b(?:remains?|is|are)\s+deferred\b/i.test(localPrefix)
          || (attachedNo && !/\band\b|,/i.test(before));
        if (!denied) claims.push({ phase: phaseScope });
      }
    }
  }
  return claims;
}

/** Exact immediate attribute range; replacing it leaves async/public modifiers untouched. */
export function maybeTestDeclaration(source: string, functionStart: number): { start: number; end: number; registration: string } | undefined {
  return declarationFromCode(source, rustCode(source), functionStart);
}

function declarationFromCode(source: string, code: string, functionStart: number): { start: number; end: number; registration: string } | undefined {
  let prefix = code.slice(0, functionStart);
  while (true) {
    const previous = prefix;
    prefix = prefix.replace(/\b(?:pub(?:\s*\([^)]*\))?|async|const|unsafe|extern)\s*$/, "");
    if (prefix === previous) break;
  }
  const attributes = prefix.match(/(?:#\s*\[[^\]]*\]\s*)+$/)?.[0] ?? "";
  const start = prefix.length - attributes.length;
  const asyncDeclared = /\basync\b/.test(code.slice(prefix.length, functionStart));
  const registrations: string[] = [];
  for (const match of attributes.matchAll(/#\s*\[([^\]]*)\]/g)) {
    const raw = source.slice(start + match.index, start + match.index + match[0].length);
    const item = compact(raw.replace(/^#\s*\[|\]$/g, ""));
    if (item === "test" && !asyncDeclared) registrations.push(raw);
    else if (asyncDeclared && (item === "tokio::test" || (item.startsWith("tokio::test(") && item.endsWith(")") && validTokioOptions(item.slice(12, -1))))) registrations.push(raw);
  }
  return registrations.length === 1 ? { start, end: prefix.length, registration: registrations[0] } : undefined;
}

/** Bounded actual Tokio options; unsupported names, duplicate fields and invalid pairs refuse. */
function validTokioOptions(options: string): boolean {
  const fields = options.split(",");
  if (fields.at(-1) === "") fields.pop();
  const values = new Map<string, string>();
  for (const field of fields) {
    const maybePair = /^(flavor|worker_threads|start_paused)=(.+)$/.exec(field);
    if (!maybePair || values.has(maybePair[1])) return false;
    values.set(maybePair[1], maybePair[2]);
  }
  const flavor = values.get("flavor") ?? '"current_thread"';
  if (![ '"current_thread"', '"multi_thread"' ].includes(flavor)) return false;
  const maybeWorkers = values.get("worker_threads");
  const maybePaused = values.get("start_paused");
  return (maybeWorkers === undefined || (/^[1-9][0-9]*$/.test(maybeWorkers) && flavor === '"multi_thread"'))
    && (maybePaused === undefined || (/^(?:true|false)$/.test(maybePaused) && flavor === '"current_thread"'));
}

function checkEnabledTests(text: (file: string) => string, fail: Fail): void {
  for (const [file, count] of TEST_OWNERS) {
    const source = text(file);
    const code = rustCode(source);
    try {
      const enabled = regions(source).filter(fn => {
        const maybeDeclaration = declarationFromCode(source, code, fn.start);
        if (!maybeDeclaration) return false;
        const attributes = code.slice(maybeDeclaration.start, maybeDeclaration.end);
        const inner = code.match(/^\s*(?:#!\s*\[[^\]]*\]\s*)*/)?.[0] ?? "";
        return !/#\s*!?\s*\[\s*ignore\b/.test(attributes + inner)
          && supportedOwnership(source, fn.start, true);
      });
      if (enabled.length !== count) fail(file, "ownership", "actual enabled test declarations and enclosing file/module cfg required");
    } catch (error) {
      fail(file, "ownership", error instanceof Error ? error.message : String(error));
    }
  }
}

function checkCorpus(source: string, fail: Fail): void {
  // Reviewed independent expected facts are immutable; never regenerated with block_proof.
  if (sha(source) !== CORPUS_SHA) fail(CORPUS_FILE, "corpus", "independent numeric expectations/source pin changed");
  try {
    const corpus = JSON.parse(source);
    if (corpus.source_pin.commit !== "a9aee730466ac67d35a3c03ee24676be5e045878"
      || corpus.evidence_class !== "source-derived-arithmetic"
      || corpus.proof_vectors.length !== 11 || corpus.addition_vectors.length !== 6) {
      fail(CORPUS_FILE, "corpus", "fixed independent source-derived corpus required");
    }
  } catch { fail(CORPUS_FILE, "corpus", "invalid independent corpus JSON"); }
}

function checkHistory(inputs: ReadonlyMap<string, string>, fail: Fail): boolean {
  let maybeHistoricalAdapter: string | undefined;
  try { maybeHistoricalAdapter = historicalFixtureAdapter(inputs); }
  catch (error) { fail("scripts/check-phase164-canonical-identity/fixture-source.ts", "history", error instanceof Error ? error.message : String(error)); }
  for (const [file, digest] of CURRENT_BRIDGE_DATA) {
    if (sha(inputs.get(file) ?? "") !== digest) fail(file, "bridge-data", "reviewed current legacy-contract bridge data changed; original captures are separate");
  }
  for (const [file, digest] of HISTORICAL_IDENTITIES) {
    const historical = file === "scripts/check-phase164-canonical-identity/fixture-source.ts" ? maybeHistoricalAdapter
      : file === "scripts/check-phase164-canonical-identity/historical-tooling.ts"
        ? maybeRestoreReviewedSource(inputs.get(file) ?? "", HISTORICAL_TOOLING_INVERSE) : inputs.get(file);
    if (sha(historical ?? "") !== digest) fail(file, "history", "original receipt or reviewed historical adapter changed");
  }
  try { historicalToolingView(inputs); }
  catch (error) { fail(MANIFEST_FILE, "history", error instanceof Error ? error.message : String(error)); }
  const file = "packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs";
  const source = inputs.get(file) ?? "";
  if (Buffer.byteLength(source) !== 20_688
    || sha(source) !== "e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b") {
    fail(file, "history", "mechanically adapted preservation source changed; original 20556-byte 3d004f91 attribution is separate");
  }
  return maybeHistoricalAdapter !== undefined;
}

function checkRegistration(source: string, fail: Fail): void {
  try {
    const lines = source.split("\n");
    if (CURRENT_VERIFY_ADDITIONS.some(line => lines.filter(actual => actual === line).length !== 1)
      || sha(verificationHistoricalBase(source)) !== VERIFY_BASE_SHA) {
      fail("scripts/verify.sh", "registration", "exact default run_step/declared pairs after164 and before68, with every old byte/order retained");
    }
  } catch (error) {
    fail("scripts/verify.sh", "registration", error instanceof Error ? error.message : String(error));
  }
}

if (import.meta.main) {
  try {
    const findings = checkPhase165(loadInputs());
    if (findings.length) {
      console.error(findings.map(row => `${row.file}: ${row.rule}: ${row.detail}`).join("\n"));
      process.exit(1);
    }
    console.log(`Phase165 source guard: ${SOURCE_CONTRACTS.length} reviewed owners; ${TEST_OWNERS.length} executable test owners. Source-derived arithmetic and local refusal evidence only; no runtime acceptance or pinned-Knots differential qualification.`);
  } catch (error) {
    console.error(`Phase165 fixed input failure: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
