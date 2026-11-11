#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { COMPLETE_NATIVE_SUITES, CONTROLLED_NATIVE_PREFIX, CONTROLLED_NATIVE_STEPS, CONTROLLED_NATIVE_SUFFIX } from "./check-phase164-canonical-identity/historical-tooling.ts";
import { BEHAVIOR_ROWS, CURRENT_PINS, KNOTS_PINS, NATIVE_TESTS, OFFLINE_COMMANDS, REQUIRED_BLOCKS, STEP_LABELS, VERIFY_FILE } from "./check-phase167-candidate-history/contracts.ts";

export type Finding = { file: string; rule: string; detail: string };
export const INPUT_FILES = [VERIFY_FILE, ...CURRENT_PINS.map(row => row[0]), ...KNOTS_PINS.map(row => row[0])];
export const MAX_INPUT_BYTES = 128 * 1024 * 1024;
const MAX_TOTAL_BYTES = 256 * 1024 * 1024;
const sha = (source: string) => createHash("sha256").update(source).digest("hex");

function checkSource(inputs: ReadonlyMap<string, string>, findings: Finding[]): void {
  const expected = new Set(INPUT_FILES);
  if (expected.size !== INPUT_FILES.length || inputs.size !== expected.size || [...inputs.keys()].some(file => !expected.has(file))) {
    findings.push({ file: "<inventory>", rule: "inventory", detail: "exact Phase167 source and corpus membership required" });
  }
  for (const [file, digest, kind] of CURRENT_PINS) {
    const current = inputs.get(file);
    if (current === undefined || sha(current) !== digest) {
      findings.push({ file, rule: kind === "corpus" ? "corpus" : "source-pin", detail: "reviewed fixed source bytes required" });
    }
  }
  for (const [file, digest] of KNOTS_PINS) {
    if (sha(inputs.get(file) ?? "") !== digest) {
      findings.push({ file, rule: "knots-pin", detail: "pinned Knots source bytes required" });
    }
  }
  for (const [name, file, needle] of BEHAVIOR_ROWS) {
    if ((inputs.get(file) ?? "").split(needle).length !== 2) {
      findings.push({ file, rule: `behavior:${name}`, detail: "unique reviewed executable seam required" });
    }
  }
  const ruleSource = inputs.get("packages/open-bitcoin-consensus/src/candidate_history/rules.rs") ?? "";
  if (ruleSource.indexOf(".maybe_script_flag_exception(facts.candidate_hash)") < 0
    || ruleSource.indexOf("for (activation, addition) in [") < 0
    || ruleSource.indexOf(".maybe_script_flag_exception(facts.candidate_hash)") > ruleSource.indexOf("for (activation, addition) in [")) {
    findings.push({ file: "packages/open-bitcoin-consensus/src/candidate_history/rules.rs", rule: "behavior:exception-order", detail: "exact exception replacement precedes deployment additions" });
  }
  for (const [file, name] of NATIVE_TESTS) {
    if (!(inputs.get(file) ?? "").includes(`#[test]\nfn ${name}(`)) {
      findings.push({ file, rule: "native-test", detail: `named executable Rust regression ${name} required` });
    }
  }
  for (const file of CURRENT_PINS.filter(row => row[2] === "corpus-script").map(row => row[0])) {
    if (!(inputs.get(file) ?? "").includes('"--check"')) {
      findings.push({ file, rule: "offline-mode", detail: "read-only --check mode required" });
    }
  }
  for (const file of CURRENT_PINS.filter(row => row[0].endsWith(".test.ts")).map(row => row[0])) {
    if (!(inputs.get(file) ?? "").includes("test(")) {
      findings.push({ file, rule: "bun-test", detail: "nonempty Bun behavior registration required" });
    }
  }
  checkCorpus(inputs, findings);
}

function checkCorpus(inputs: ReadonlyMap<string, string>, findings: Finding[]): void {
  const file = "packages/testdata/phase167/manifest.json";
  try {
    const manifest = JSON.parse(inputs.get(file) ?? "") as Record<string, unknown>;
    const sources = KNOTS_PINS.map(([path, digest]) => ({ file: path.replace("packages/bitcoin-knots/", ""), sha256: digest }));
    const expectedLinks = [
      ["blocks_sha256", "packages/testdata/phase167/blocks.json"],
      ["contexts_sha256", "packages/testdata/phase167/contexts.json"],
      ["expectations_sha256", "docs/parity/fixtures/phase167-rule-expectations.json"],
    ] as const;
    if (manifest.schema !== 1 || manifest.pin !== "a9aee730466ac67d35a3c03ee24676be5e045878"
      || manifest.evidence_class !== "independently-seeded-historical"
      || JSON.stringify(manifest.sources) !== JSON.stringify(sources)
      || JSON.stringify(manifest.required) !== JSON.stringify(REQUIRED_BLOCKS)
      || expectedLinks.some(([key, target]) => manifest[key] !== sha(inputs.get(target) ?? ""))) {
      findings.push({ file, rule: "corpus", detail: "fixed source pin, required block and corpus link membership required" });
    }
  } catch {
    findings.push({ file, rule: "corpus", detail: "bounded pinned manifest JSON required" });
  }
}

/** Pure current registration policy; it does not run or qualify native checks. */
export function checkPhase167(inputs: ReadonlyMap<string, string>): Finding[] {
  const findings: Finding[] = [];
  checkSource(inputs, findings);
  const verify = inputs.get(VERIFY_FILE);
  if (verify === undefined) return [{ file: VERIFY_FILE, rule: "input", detail: "reviewed verifier source required" }];
  const actual = verify.split("\n").filter(line => line.startsWith("run_step "));
  const executed = actual.map(line => line.replace(/^run_step "[^"]+" /, ""));
  const phase167 = (line: string) => line.includes("scripts/phase167-") || line.includes("scripts/check-phase167-");
  const index = executed.indexOf("bun run scripts/check-phase166-canonical-bootstrap.ts");
  if (index < 0 || OFFLINE_COMMANDS.some((command, offset) => executed[index + offset + 1] !== command)
    || OFFLINE_COMMANDS.some(command => executed.filter(line => line === command).length !== 1)
    || executed.filter(phase167).length !== OFFLINE_COMMANDS.length) {
    findings.push({ file: VERIFY_FILE, rule: "registration", detail: "executed Phase167 offline commands must follow Phase166 exactly once and in order" });
  }
  if (STEP_LABELS.some((label, index) => actual.filter(line => line === `run_step "${label}" ${OFFLINE_COMMANDS[index]}`).length !== 1)) {
    findings.push({ file: VERIFY_FILE, rule: "runner", detail: "exact executable run_step labels and commands required" });
  }
  const verifyLines = verify.split("\n");
  const stressLines = verifyLines.filter(line => line.startsWith("run_step ") && line.includes("--test-threads="));
  const client = 'run_step "cargo test CLI RPC client binary" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --all-features';
  if (CONTROLLED_NATIVE_STEPS.length !== 7 || stressLines.length !== 7
    || stressLines.some((line, index) => line !== CONTROLLED_NATIVE_STEPS[index])
    || CONTROLLED_NATIVE_STEPS.some(line => verifyLines.filter(actualLine => actualLine === line).length !== 1)
    || verifyLines.indexOf(CONTROLLED_NATIVE_STEPS[6]) >= verifyLines.indexOf(COMPLETE_NATIVE_SUITES.split("\n")[0])) {
    findings.push({ file: VERIFY_FILE, rule: "native-stress", detail: "exact once ordered 16-thread node selectors before complete native suites required" });
  }
  if (verify.split(CONTROLLED_NATIVE_PREFIX).length !== 2
    || verify.split(CONTROLLED_NATIVE_SUFFIX).length !== 2
    || !verify.includes(CONTROLLED_NATIVE_PREFIX + COMPLETE_NATIVE_SUITES + client + "\n" + CONTROLLED_NATIVE_SUFFIX)) {
    findings.push({ file: VERIFY_FILE, rule: "native-scope", detail: "four-thread complete native suites must restore the caller environment before coverage" });
  }
  if (actual.some(line => /--write|--acquire|\b(?:curl|wget|fetch|git clone)\b/.test(line))) {
    findings.push({ file: VERIFY_FILE, rule: "offline", detail: "native verification cannot write, acquire or fetch Phase167 inputs" });
  }
  return findings;
}

export function loadInputs(): Map<string, string> {
  return loadBoundedInputs(MAX_INPUT_BYTES);
}

/** Optional lower bound is for focused tests; production never raises the reviewed limit. */
export function loadBoundedInputs(maxBytes: number): Map<string, string> {
  if (!Number.isSafeInteger(maxBytes) || maxBytes < 1 || maxBytes > MAX_INPUT_BYTES) {
    throw new Error("invalid bounded source input limit");
  }
  let totalBytes = 0;
  const rows: [string, string][] = [];
  for (const file of INPUT_FILES) {
    const path = resolve(import.meta.dir, "..", file);
    const info = lstatSync(path);
    totalBytes += info.size;
    if (!info.isFile() || info.size > maxBytes || totalBytes > MAX_TOTAL_BYTES) {
      throw new Error(`${file}: bounded regular source input required`);
    }
    const content = readFileSync(path, "utf8");
    if (Buffer.byteLength(content) !== info.size) throw new Error(`${file}: source changed during bounded read`);
    rows.push([file, content]);
  }
  return new Map(rows);
}

if (import.meta.main) {
  try {
    const findings = checkPhase167(loadInputs());
    if (findings.length) {
      console.error("Phase167 candidate history guard failed:\n" + findings.map(row => `${row.file}: ${row.rule}: ${row.detail}`).join("\n"));
      process.exitCode = 1;
    } else console.log("Phase167 candidate history source and offline verifier guard passed.");
  } catch (error) {
    console.error(error);
    process.exitCode = 1;
  }
}
