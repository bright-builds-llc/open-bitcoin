import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { CLI_MODULES, copiedInputs, expectCopiedAdapterCase } from "./check-phase165-shared-exact-work/test-support.ts";
import { SOURCE_CONTRACTS } from "./check-phase164-canonical-identity/test-modules.ts";
import { affirmativeScopeClaims, checkCurrentClaims, currentQualificationClaims } from "./check-phase164-canonical-identity/claims.ts";
import { checkPhase164, loadInputs as loadIdentityInputs } from "./check-phase164-canonical-identity.ts";
import { checkPhase165, loadInputs as loadWorkInputs } from "./check-phase165-shared-exact-work.ts";
import { CURRENT_TEMP_PREPARATION, historicalToolingView, legacyToolingBase, verificationHistoricalBase } from "./check-phase164-canonical-identity/historical-tooling.ts";
import { HISTORICAL_TOOLING_INVERSE, maybeRestoreReviewedSource } from "./check-phase164-canonical-identity/contracts.ts";
import { VERIFY_BASE_SHA } from "./check-phase165-shared-exact-work/contracts.ts";
import { createHash } from "node:crypto";
import { reviewedOwnership } from "./check-phase164-canonical-identity.ts";
import { regions } from "./check-phase164-canonical-identity/regions.ts";
import { checkPhase166, INPUT_FILES, loadInputs } from "./check-phase166-canonical-bootstrap.ts";
import { CHECKPOINT, CLI, CONSENSUS, CORE, DAEMON, FACTORY, INSPECTION, NODE, OBSERVER, OPENER, RPC, VECTOR } from "./check-phase166-canonical-bootstrap/contracts.ts";

const real = loadInputs();
const PROCESS = "packages/open-bitcoin-rpc/tests/phase165_work_startup.rs";
const ORIGINAL_PROCESS_IDENTITY = "ac6c248cd76324da26c5bf3850718df60d4fb790a13288ebfbc6eccc2e8272a3";

// Literal forms and statuses inventoried independently from the original 164/165 guards.
const legacyClaimFamilies = [
  { owner: 164, subjects: ["Bootstrap", "Reopen", "Whole-chain parity", "Production", "Knots qualification"],
    statuses: ["qualified", "supported", "ready", "proven", "complete", "accepted"] },
  { owner: 165, subjects: ["Phase 166", "Phase 167", "Phase 168", "Canonical bootstrap", "Canonical reopen", "Restart tie parity", "Deterministic restart ordering", "Reopen order", "Deterministic reopen tie parity", "Knots daemon", "Actual Knots differential qualification", "Pinned Knots qualification", "Actual pinned Knots daemon differential qualification"],
    statuses: ["qualified", "proven", "passed", "delivered", "complete", "established", "supported", "accepted", "deterministic", "ready", "achieved", "available"] },
] as const;
for (const family of legacyClaimFamilies) for (const subject of family.subjects) for (const status of family.statuses) {
  test(`pure original ${family.owner} subject/status union refuses ${subject} is ${status}`, () => {
    expect(currentQualificationClaims(`${subject} is ${status}.`).length).toBeGreaterThan(0);
  });
}
for (const status of ["supported", "available", "proved"]) test(`pure standalone reopen restriction remains ${status}`, () => {
  expect(currentQualificationClaims(`Reopen is ${status}.`).length).toBeGreaterThan(0);
});
test("pure original whole-chain subject retains its natural space spelling", () => {
  for (const status of ["qualified", "supported", "ready", "proven", "complete", "accepted"]) {
    expect(currentQualificationClaims(`Whole chain parity is ${status}.`).length).toBeGreaterThan(0);
  }
});
for (const subject of ["bootstrap", "reopen", "whole-chain parity", "production", "Knots qualification"]) {
  for (const status of ["Qualified", "Supported", "Ready", "Proven", "Complete", "Accepted"]) {
    test(`pure original164 prefix-status order refuses ${status} ${subject}`, () => {
      expect(currentQualificationClaims(`${status} ${subject}.`).length).toBeGreaterThan(0);
    });
  }
}
for (const [claim, allowed] of [
  ["No proven reopen.", true], ["Not yet supported production.", true],
  ["Never qualified Knots qualification.", true], ["Without accepted whole-chain parity.", true],
  ["Once a proven reopen.", true], ["Not proof for proven reopen.", true],
  ["Proven reopen remains deferred.", true],
  ["No ordinary history is supported and supported production.", false],
  ["Bootstrap is not supported but proven reopen.", false],
  ["The setup runs if configured while proven reopen.", false],
  ["No blockers remain and proven reopen.", false],
] as const) test(`pure original164 prefix ownership and denial: ${claim}`, () => {
  expect(currentQualificationClaims(claim).length === 0).toBe(allowed);
});
for (const [claim, refused] of [
  ["Phase 168\n\nis complete.", true], ["## Phase 168\n\nComplete.", true],
  ["Proven canonical bootstrap.", false], ["Supported Knots daemon qualification.", false],
  ["## Current Phase 166 canonical genesis boundary\n\nCanonical genesis bootstrap and reopen are supported for all four selectors.", false],
  ["Unfinished preceding prose\n\n## Current Phase 166 canonical genesis boundary\n\nCanonical genesis bootstrap and reopen are supported for all four selectors.", false],
  ["## Current Phase 166 canonical genesis boundary\n\nComplete.", true],
] as const) test(`pure original165 flattening and following-predicate ownership: ${claim}`, () => {
  expect(affirmativeScopeClaims(claim).length > 0).toBe(refused);
});
test("original164 helper preprocessing keeps paragraphs and semicolons but retains but-cooccurrence", () => {
  expect(currentQualificationClaims("Reopen\n\nsupported.")).toEqual([]);
  expect(currentQualificationClaims("Reopen; supported.")).toEqual([]);
  expect(currentQualificationClaims("Reopen but supported.").length).toBeGreaterThan(0);
});
for (const [file, claim] of [
  ["docs/parity/v2-6-canonical-chain-identity.md", "Reopen; supported."],
  ["docs/parity/v2-6-canonical-chain-identity.md", "Proven reopen."],
  ["README.md", "Supported production."], ["docs/parity/support-matrix.md", "Proven reopen."],
] as const) test(`original164 actual document ordering/preprocessing entrypoint ${file}: ${claim}`, () => {
  // Arrange
  const inputs = loadIdentityInputs();
  inputs.set(file, inputs.get(file)! + "\n\n## Current Phase 166 canonical genesis boundary\n\n" + claim + "\n");
  const findings: { file: string; rule: string; detail: string }[] = [];
  // Act / Assert
  checkCurrentClaims(file => inputs.get(file)!, (file, rule, detail) => findings.push({ file, rule, detail }));
  expect(findings.some(row => row.file === file && row.rule === "claim")).toBe(true);
});

const CURRENT_GENESIS_HEADING = "## Current Phase 166 canonical genesis boundary";
for (const [claim, allowed] of [
  ...["supported", "available", "proved"].flatMap(status => [
    [`Phase 166 canonical genesis bootstrap and reopen are ${status} for all four selectors`, true],
    [`Canonical bootstrap and reopen are ${status}`, false], [`Ordinary history is ${status}`, false], [`Reopen is ${status}`, false],
  ] as const),
  ["Reopen\n\nsupported", true], ["Reopen; supported", true], ["Reopen but supported", false], ["Supported production", false],
] as const) test(`current166 section coverage actual existing README: ${claim}`, () => {
  // Arrange: replace the real current paragraph, preserving all surrounding document bytes.
  const inputs = loadIdentityInputs(), file = "README.md", original = inputs.get(file)!;
  const before = "Canonical genesis bootstrap and reopen are supported for all four selectors.";
  expect(original.split(CURRENT_GENESIS_HEADING)).toHaveLength(2);
  expect(original.split(before)).toHaveLength(2);
  const changed = original.replace(before, claim + ".");
  expect(changed).not.toBe(original);
  inputs.set(file, changed);
  const findings: { file: string; rule: string; detail: string }[] = [];
  // Act / Assert
  checkCurrentClaims(file => inputs.get(file)!, (file, rule, detail) => findings.push({ file, rule, detail }));
  expect(findings.some(row => row.file === file && row.rule === "claim")).toBe(!allowed);
});
for (const file of ["README.md", "docs/parity/support-matrix.md"]) for (const badIndex of [-1, 0, 1, 2]) {
  for (const claim of badIndex === -1 ? ["none"] : ["Supported production.", "Reopen is available.", "Ordinary history is proved.", "Canonical bootstrap and reopen are supported."]) {
    test(`current166 section coverage repeated ${file} section ${badIndex}: ${claim}`, () => {
      // Arrange: historical separators cannot hide a later current section or gain current authority.
      const inputs = loadIdentityInputs();
      const bounded = "Canonical genesis bootstrap and reopen are available for all four selectors.";
      const sections = [0, 1, 2].map(index => CURRENT_GENESIS_HEADING + "\n\n" + (index === badIndex ? claim : bounded));
      inputs.set(file, inputs.get(file)! + "\n\n" + sections.join("\n\n## Historical release\n\nSupported production.\n\n"));
      const findings: { file: string; rule: string; detail: string }[] = [];
      // Act
      checkCurrentClaims(file => inputs.get(file)!, (file, rule, detail) => findings.push({ file, rule, detail }));
      // Assert: precisely one offending section, or none when every current clause is bounded.
      expect(findings.filter(row => row.file === file && row.rule === "claim")).toHaveLength(badIndex === -1 ? 0 : 1);
    });
  }
}

test("current verifier inverse preserves the complete reviewed base and original historical snapshot", () => {
  // Arrange
  const inputs = loadWorkInputs();
  const source = inputs.get("scripts/verify.sh")!;
  // Act
  const projected = verificationHistoricalBase(source);
  const historical = historicalToolingView(inputs);
  // Assert: these are distinct existing generations; neither hash or snapshot is renewed.
  expect(createHash("sha256").update(projected).digest("hex")).toBe(VERIFY_BASE_SHA);
  expect(historical.get("scripts/verify.sh")).toBe(inputs.get("packages/testdata/phase164/historical-tooling/verify.txt"));
});

const NATIVE_SOURCE = "scripts/check-phase163-provenance/native.test.ts";
const TOOLING_SOURCE = "scripts/check-phase164-canonical-identity/historical-tooling.ts";
const NATIVE_MANIFEST_CURRENT_SHA = "5297898680a15de12f79b7e6c9c8a814fdd0276522bfd071b96cd7b3a6f7ac16";
test("shared copied writer preserves literal replacement tokens at semantic refusal boundaries", () => {
  // Arrange: inspect exact copied bytes before either imported guard and actual CLI execute.
  const inputs = loadWorkInputs(), file = TOOLING_SOURCE, source = inputs.get(file)!;
  const before = 'import { createHash } from "node:crypto";';
  const after = before + "\n// literal replacement $$ $& $` $'\n";
  expect(source.split(before)).toHaveLength(2);
  const offset = source.indexOf(before), expected = source.slice(0, offset) + after + source.slice(offset + before.length);
  const load = (root: string) => {
    expect(readFileSync(join(root, file), "utf8") === expected, "shared writer changed literal bytes").toBe(true);
    return loadWorkInputs(root);
  };
  // Act / Assert: the shared writer must preserve the mutant and its precise refusal boundary.
  expectCopiedAdapterCase(["unknown-tooling-byte", file, before, after, "paired current raw historical-tooling generation"], inputs, load,
    [[CLI_MODULES[2], checkPhase164], [CLI_MODULES[0], checkPhase165]]);
});
test("native timeout isolation restores exact manifest-current source and original tooling bytes", () => {
  // Arrange
  const inputs = loadIdentityInputs();
  const native = inputs.get(NATIVE_SOURCE)!;
  const hash = (source: string) => createHash("sha256").update(source).digest("hex");
  // Act
  const restored = legacyToolingBase(NATIVE_SOURCE, native);
  const maybeTooling = maybeRestoreReviewedSource(inputs.get(TOOLING_SOURCE)!, HISTORICAL_TOOLING_INVERSE);
  const historical = historicalToolingView(inputs);
  // Assert
  expect(restored).not.toBe(native);
  expect(hash(restored)).toBe(NATIVE_MANIFEST_CURRENT_SHA);
  expect(hash(maybeTooling!)).toBe("47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65");
  expect(Buffer.byteLength(maybeTooling!)).toBe(104671);
  expect(historical.get(NATIVE_SOURCE)).toBe(inputs.get("packages/testdata/phase164/historical-tooling/phase163-native-tests.txt"));
});

for (const [name, before, after] of [
  ["deadline", "timeout: 55_000", "timeout: 54_000"],
  ["broad selection", '`^${selected}$`', '`^actual `'],
  ["unrelated bytes", "afterEach(cleanupFixtures);", "afterEach(cleanupFixtures);\n"],
] as const) test(`native timeout isolation refuses unreviewed current native ${name}`, () => {
  // Arrange
  const inputs = loadIdentityInputs();
  const original = inputs.get(NATIVE_SOURCE)!;
  expect(original.split(before)).toHaveLength(2);
  const changed = original.replace(before, after);
  expect(changed).not.toBe(original);
  inputs.set(NATIVE_SOURCE, changed);
  // Act / Assert
  expect(legacyToolingBase(NATIVE_SOURCE, changed)).toBe(changed);
  expect(() => historicalToolingView(inputs)).toThrow(`${NATIVE_SOURCE}: unreviewed current tooling fingerprint`);
});

for (const mode of ["missing", "wrong destination", "altered current span"] as const) {
  test(`native timeout isolation refuses actual copied ${mode} mapping`, () => {
    // Arrange: run the changed mapping module itself, independently of current owner pins.
    const inputs = loadIdentityInputs();
    const source = inputs.get(TOOLING_SOURCE)!;
    const mappingRows = source.split("\n").filter(line => line.startsWith(`  ["${NATIVE_SOURCE}","${NATIVE_MANIFEST_CURRENT_SHA}",`));
    expect(mappingRows).toHaveLength(1);
    const mapping = mappingRows[0];
    const replacement = mode === "missing" ? ""
      : mode === "wrong destination" ? mapping.replace(NATIVE_MANIFEST_CURRENT_SHA, "0".repeat(64))
      : mapping.replace("selectedLayout ===", "selectedLayout !==");
    expect(replacement).not.toBe(mapping);
    const changed = source.replace(mapping, () => replacement);
    expect(maybeRestoreReviewedSource(changed, HISTORICAL_TOOLING_INVERSE)).toBeUndefined();
    expect(checkPhase164(new Map(inputs).set(TOOLING_SOURCE, changed)).some(row => row.rule === "history" && row.detail.includes("historical-tooling"))).toBe(true);
    const root = mkdtempSync(join(tmpdir(), "phase166-native-mapping-"));
    try {
      copiedInputs(root, inputs);
      writeFileSync(join(root, TOOLING_SOURCE), changed);
      // Act
      const result = spawnSync(process.execPath, ["-e",
        'import { loadInputs } from "./scripts/check-phase164-canonical-identity.ts"; import { historicalToolingView } from "./scripts/check-phase164-canonical-identity/historical-tooling.ts"; historicalToolingView(loadInputs());'],
      { cwd: root, encoding: "utf8", timeout: 10_000, maxBuffer: 256 * 1024 });
      // Assert: this exact native fingerprint proves the changed mapping executed.
      expect(result.error).toBeUndefined();
      expect(result.signal).toBeNull();
      expect(result.status, result.stderr).toBe(1);
      expect(result.stderr).toContain(`${NATIVE_SOURCE}: unreviewed current tooling fingerprint`);
      expect(result.stderr).not.toContain("input inventory");
    } finally { rmSync(root, { recursive: true }); }
  });
}

for (const [name, transform] of [
  ["missing", (source: string) => source.replace(CURRENT_TEMP_PREPARATION, "")],
  ["duplicate", (source: string) => source.replace(CURRENT_TEMP_PREPARATION, CURRENT_TEMP_PREPARATION.repeat(2))],
  ["nonphysical", (source: string) => source.replace("pwd -P", "pwd -L")],
  ["late", (source: string) => source.replace(CURRENT_TEMP_PREPARATION, "") + CURRENT_TEMP_PREPARATION],
] as const) test(`both current guards refuse ${name} temporary-directory preparation`, () => {
  // Arrange
  const inputs = loadWorkInputs();
  const source = inputs.get("scripts/verify.sh")!;
  const changed = transform(source);
  expect(changed).not.toBe(source);
  inputs.set("scripts/verify.sh", changed);
  // Act / Assert
  expect(checkPhase164(inputs).some(row => row.file === "scripts/verify.sh" && row.rule === "registration")).toBe(true);
  expect(checkPhase165(inputs).some(row => row.file === "scripts/verify.sh" && row.rule === "registration")).toBe(true);
});
test("verifier temporary-directory preparation resolves aliases and fails before fixture work on a missing parent", () => {
  // Arrange: execute only the actual pre-fixture shell prefix.
  const root = mkdtempSync(join(tmpdir(), "phase166-temp-parent-"));
  try {
    mkdirSync(join(root, "physical"));
    symlinkSync(join(root, "physical"), join(root, "alias"));
    const source = readFileSync("scripts/verify.sh", "utf8");
    const prefix = source.slice(0, source.indexOf("print_timings=0"));
    expect(prefix).toContain(CURRENT_TEMP_PREPARATION);
    // Act
    const actual = spawnSync("bash", ["-c", prefix + '\nprintf "%s" "$TMPDIR"'], { env: { ...process.env, TMPDIR: join(root, "alias") }, encoding: "utf8" });
    const refused = spawnSync("bash", ["-c", prefix + '\nprintf "FIXTURE_REACHED"'], { env: { ...process.env, TMPDIR: join(root, "missing") }, encoding: "utf8" });
    // Assert
    expect(actual.error).toBeUndefined();
    expect(actual.status).toBe(0);
    expect(actual.stdout).toBe(realpathSync(join(root, "physical")));
    expect(refused.error).toBeUndefined();
    expect(refused.status).not.toBe(0);
    expect(refused.stdout).toBe("");
  } finally { rmSync(root, { recursive: true }); }
});

for (const [name, before, after] of [
  ["closed physical preservation", "                    manifest(root),\n                    before,", "                    manifest(root),\n                    manifest(root),"],
  ["diagnostic redaction", 'assert!(!message.contains(secret), "{message}");', 'assert!(true, "{message}");'],
  ["ordinary history refusal", 'message.contains("scope=identity_only; ordinary_history=false")', 'message.contains("scope=genesis; ordinary_history=true")'],
  ["fresh selected genesis", 'assert_eq!(\n                info["result"]["bestblockhash"],\n                vector["block_hash_display"]\n            );', 'assert!(true);'],
  ["same-path independent reopen", "for _ in 0..2 {", "for _ in 0..1 {"],
  ["fresh test ownership", "#[test]\nfn phase166_existing_empty_local_genesis_starts_and_reopens_all_selectors()", "#[cfg(any())]\n#[test]\nfn phase166_existing_empty_local_genesis_starts_and_reopens_all_selectors()"],
] as const) test(`reviewed process contract refuses removed or disabled ${name}`, () => {
  // Arrange: unchanged current source must first pass both independent current guards.
  const inputs = loadWorkInputs();
  const source = inputs.get(PROCESS)!;
  expect(source.split(before)).toHaveLength(2);
  expect(before).not.toBe(after);
  const changed = source.replace(before, after);
  expect(changed).not.toBe(source);
  expect(checkPhase164(inputs).filter(row => row.file === PROCESS)).toEqual([]);
  expect(checkPhase165(inputs).filter(row => row.file === PROCESS)).toEqual([]);
  inputs.set(PROCESS, changed);
  // Act / Assert
  expect(checkPhase164(inputs).some(row => row.file === PROCESS && row.rule === "boundary")).toBe(true);
  expect(checkPhase165(inputs).some(row => row.file === PROCESS && row.rule === "ownership")).toBe(true);
});

test("reviewed process contract refuses its original stale row in an isolated current checker", () => {
  // Arrange: only copied checker data changes; tracked contracts remain untouched.
  const root = mkdtempSync(join(tmpdir(), "phase166-stale-process-"));
  try {
    const inputs = loadWorkInputs();
    copiedInputs(root, inputs);
    const row = SOURCE_CONTRACTS.find(row => row[0] === PROCESS)!;
    expect(row[2]).not.toBe(ORIGINAL_PROCESS_IDENTITY);
    const file = join(root, "scripts/check-phase164-canonical-identity/test-modules.ts");
    const source = readFileSync(file, "utf8");
    expect(source.split(row[2])).toHaveLength(2);
    writeFileSync(file, source.replace(row[2], ORIGINAL_PROCESS_IDENTITY));
    // Act
    const result = spawnSync(process.execPath, [join(root, "scripts/check-phase165-shared-exact-work.ts")], { encoding: "utf8", timeout: 10_000 });
    // Assert: the exact stale literal-bearing identity refuses independently of runtime behavior.
    expect(result.error).toBeUndefined();
    expect(result.status).toBe(1);
    expect(result.stderr).toContain(PROCESS + ": ownership: reviewed literal-bearing owner changed");
    expect(result.stderr).not.toContain("input inventory");
  } finally { rmSync(root, { recursive: true }); }
});

for (const [claim, allowed] of [
  ["Canonical genesis bootstrap and reopen are supported for all four selectors", true],
  ["Phase 166 canonical genesis bootstrap and reopen are supported for all four selectors", true],
  ["Exact root identity grants no canonical bootstrap/reopen or accepted history", true],
  ["Source-derived vectors do not claim actual Knots daemon qualification or ordinary accepted canonical history", true],
  ["The suite can run once a built Knots daemon path is available", true],
  ["The suite can run once a built Knots daemon path is available and ordinary history is proved", false],
  ["Unrelated setup waits if configured while canonical bootstrap is available", false],
  ["No ordinary history is proved and canonical bootstrap is available", false],
  ["Canonical genesis bootstrap is not supported and actual Knots daemon differential qualification is proved", false],
  ["Unrelated setup runs when configured while ordinary history is proved", false],
  ["Unrelated setup asks whether it is ready while actual Knots daemon differential qualification is proved", false],
  ["Canonical bootstrap and reopen acceptance is complete", false],
  ["Canonical genesis bootstrap is supported and ordinary history is supported", false],
  ["Canonical genesis bootstrap is supported and Phase 168 is qualified", false],
  ["Actual pinned Knots daemon differential qualification is proven", false],
  ["Canonical genesis bootstrap and reopen are supported for all four selectors and actual pinned Knots daemon differential qualification is proven", false],
] as const) test(`current claims require the same bounded genesis clause: ${claim}`, () => {
  // Arrange: a current statement cannot inherit scope or denial from historical context.
  const identity = loadIdentityInputs();
  const file = "docs/parity/v2-6-canonical-chain-identity.md";
  identity.set(file, identity.get(file)! + "\n\n" + claim + ".\n");
  const findings: { file: string; rule: string; detail: string }[] = [];
  // Act
  checkCurrentClaims(file => identity.get(file)!, (file, rule, detail) => findings.push({ file, rule, detail }));
  const work = loadWorkInputs();
  work.set(file, identity.get(file)!);
  const workFindings = checkPhase165(work);
  // Assert: stale code rows are separate from the claim classification.
  expect(findings.some(row => row.file === file && row.rule === "claim")).toBe(!allowed);
  expect(workFindings.some(row => row.file === file && row.rule === "claim")).toBe(!allowed);
});

for (const status of ["supported", "available", "proved"]) for (const file of ["README.md", "docs/parity/support-matrix.md", "docs/parity/v2-6-canonical-chain-identity.md"]) {
  for (const [claim, allowed] of [
    [`Canonical genesis bootstrap and reopen are ${status} for all four selectors`, true],
    [`Canonical bootstrap and reopen are ${status}`, false],
    [`Ordinary history is ${status}`, false],
    [`Reopen is ${status}`, false],
  ] as const) test(`current heading qualification vocabulary ${file}: ${claim}`, () => {
    // Arrange: status and subject share this current clause, independent of historical denials.
    const identity = loadIdentityInputs();
    const source = identity.get(file)! + "\n\n## Current Phase 166 canonical genesis boundary\n\n" + claim + ".\n";
    identity.set(file, source);
    const findings: { file: string; rule: string; detail: string }[] = [];
    // Act
    checkCurrentClaims(file => identity.get(file)!, (file, rule, detail) => findings.push({ file, rule, detail }));
    const work = loadWorkInputs();
    work.set(file, source);
    const workFindings = checkPhase165(work);
    // Assert
    expect(findings.some(row => row.file === file && row.rule === "claim")).toBe(!allowed);
    expect(workFindings.some(row => row.file === file && row.rule === "claim")).toBe(!allowed);
  });
}

for (const status of ["supported", "available", "proved"]) for (const file of ["docs/parity/v2-6-shared-exact-work.md", "docs/parity/index.json"]) {
  for (const subject of ["Canonical bootstrap", "Reopen", "Ordinary history", "Actual pinned Knots daemon differential qualification"]) {
    test(`fixed work/JSON claim entrypoint refuses ${file}: ${subject} is ${status}`, () => {
      // Arrange: each JSON string owns its assertion independently of another string's denial.
      const inputs = loadWorkInputs();
      const claim = `${subject} is ${status}.`;
      if (file.endsWith(".json")) {
        const catalog = JSON.parse(inputs.get(file)!);
        const owner = catalog.checklist.surfaces.find((row: { id: string }) => row.id === "v2-6-canonical-chain-identity");
        owner.known_gaps.unshift("No canonical bootstrap is supported.");
        owner.intentional_differences.push(claim);
        inputs.set(file, JSON.stringify(catalog));
      } else inputs.set(file, inputs.get(file)! + "\n\n" + claim + "\n");
      // Act / Assert
      expect(checkPhase165(inputs).some(row => row.file === file && row.rule === "claim")).toBe(true);
      if (file.endsWith(".json")) {
        const findings: { file: string; rule: string; detail: string }[] = [];
        checkCurrentClaims(file => inputs.get(file)!, (file, rule, detail) => findings.push({ file, rule, detail }));
        expect(findings.some(row => row.file === file && row.rule === "claim")).toBe(true);
      }
    });
  }
}

test("original identity and work claim ownership preserves actual historical production-source facts", () => {
  // Arrange: this is an existing Phase165 source-preservation fact, not current production qualification.
  const source = readFileSync("docs/parity/v2-6-shared-exact-work.md", "utf8");
  const fact = "Historical snapshots, manifests, inverses, copiers, test assertions and production\nRust retain their complete reviewed bytes.";
  expect(source).toContain(fact);
  // Act / Assert: original164 owned generic production only in its identity sections;165 did not.
  expect(affirmativeScopeClaims(fact)).toEqual([]);
  expect(currentQualificationClaims("Production is complete.").length).toBeGreaterThan(0);
  expect(currentQualificationClaims("Whole-chain parity is accepted.").length).toBeGreaterThan(0);
});

test("current Phase165 copied membership refuses an unapproved sixth input before source-root I/O", () => {
  // Arrange: copy actual current modules, then change their exported read-set membership.
  const root = mkdtempSync(join(tmpdir(), "phase166-reader-membership-"));
  try {
    copiedInputs(root, new Map());
    const file = join(root, "scripts/check-phase164-canonical-identity/test-modules.ts");
    const source = readFileSync(file, "utf8");
    const anchor = "export const SOURCE_FILES = [";
    expect(source.split(anchor)).toHaveLength(2);
    const changed = source.replace(anchor, anchor + '\n  "unapproved-sixth-input.rs",');
    expect(changed).not.toBe(source);
    writeFileSync(file, changed);
    const moduleUrl = new URL("file://" + join(root, "scripts/check-phase165-shared-exact-work.ts")).href;
    // Act: the absent root makes accidental source I/O distinguishable from inventory refusal.
    const result = spawnSync(process.execPath, ["-e", `const module = await import(${JSON.stringify(moduleUrl)}); try { module.loadInputs(${JSON.stringify(join(root, "absent-source-root"))}); process.exit(2); } catch (error) { console.log(error.message); }`], { encoding: "utf8", timeout: 10_000 });
    // Assert
    expect(result.error).toBeUndefined();
    expect(result.status).toBe(0);
    expect(result.stderr).toBe("");
    expect(result.stdout).toBe("fixed input inventory changed before any source read\n");
  } finally { rmSync(root, { recursive: true }); }
});
const LEASE = NODE + "storage/fjall_store/open_lease.rs";
const lease = real.get(LEASE)!;
const unixDeclaration = "#[cfg(unix)]\n    pub(super) fn acquire(source:";
const helperDeclaration = "#[cfg(test)]\nimpl super::FjallNodeStore {";

test("exact relocated Unix and test-impl ownership retain their real mechanical regions", () => {
  // Arrange
  const entries = regions(lease);
  const acquire = entries.filter(row => row.symbol === "acquire");
  const helper = entries.find(row => row.symbol === "open_without_ensure_schema_for_test")!;
  // Act / Assert
  expect(reviewedOwnership(LEASE, lease, acquire[1].start, "acquire")).toBe(true);
  expect(reviewedOwnership(LEASE, lease, acquire[0].start, "acquire")).toBe(false);
  expect(reviewedOwnership(LEASE, lease, helper.start, helper.symbol)).toBe(true);
  expect(helper.test).toBe(false);
  expect(reviewedOwnership("wrong.rs", lease, acquire[1].start, "acquire")).toBe(false);
  expect(reviewedOwnership(LEASE, lease, acquire[1].start, "wrong")).toBe(false);
});
for (const [symbol, before, after] of [
  ...["", "#[cfg(not(unix))]", "#[cfg(any())]", "#[cfg(unix)]\n    #[cfg(any())]", "#[cfg(unix)]\n    #[cfg(unix)]"].map(cfg =>
    ["acquire", unixDeclaration, `${cfg}\n    pub(super) fn acquire(source:`]),
  ...["", "#[cfg(any())]", "#[cfg(test)]\n#[cfg(any())]", "#[cfg(test)]\n#[cfg(test)]"].map(cfg =>
    ["open_without_ensure_schema_for_test", helperDeclaration, `${cfg}\nimpl super::FjallNodeStore {`]),
  ["open_without_ensure_schema_for_test", "    pub(crate) fn open_without_ensure_schema_for_test(", "    #[cfg(any())]\n    pub(crate) fn open_without_ensure_schema_for_test("],
] as readonly (readonly [string, string, string])[]) {
  test(`relocated ownership refuses missing, extra or false cfg: ${symbol}:${after}`, () => {
    // Arrange
    expect(lease.split(before)).toHaveLength(2);
    expect(before).not.toBe(after);
    const changed = lease.replace(before, after);
    expect(changed).not.toBe(lease);
    const owner = regions(changed).filter(row => row.symbol === symbol)[symbol === "acquire" ? 1 : 0];
    // Act / Assert
    expect(reviewedOwnership(LEASE, changed, owner.start, symbol)).toBe(false);
  });
}
const mutants: readonly (readonly [string, string, string, string, string])[] = [
  ["complete body proof", CONSENSUS + "chain_spec/bootstrap.rs", "spec.check_genesis_block(body)?;", "let _ignored = spec;", "genesis-proof"],
  ["proof constructor cannot be cfg-disabled", CONSENSUS + "chain_spec/bootstrap.rs", "    pub fn prove(", "    #[cfg(any())]\n    pub fn prove(", "genesis-proof"],
  ["unreachable proof cannot qualify", CONSENSUS + "chain_spec/bootstrap.rs", "spec.check_genesis_block(body)?;", "return Ok(Self { spec, block: body.clone(), work: ProofWork::ZERO }); spec.check_genesis_block(body)?;", "genesis-proof"],
  ["conditional early success cannot skip proof", CONSENSUS + "chain_spec/bootstrap.rs", "spec.check_genesis_block(body)?;", "if true { return Ok(Self { spec, block: body.clone(), work: ProofWork::ZERO }); } spec.check_genesis_block(body)?;", "genesis-proof"],
  ["internally recomputed work", CONSENSUS + "chain_spec/bootstrap.rs", "crate::block_proof(body.header.bits)", "crate::block_proof(0x207f_ffff)", "genesis-proof"],
  ["complete coins view", CORE + "engine/canonical_genesis.rs", "coins.complete_snapshot()?", "coins.partial_snapshot()?", "complete-coins"],
  ["no spendable genesis outputs", CORE + "engine/canonical_genesis.rs", "|| !snapshot.coins.is_empty()", "|| false", "complete-coins"],
  ["coins heads refusal", CORE + "engine/canonical_genesis.rs", "|| !snapshot.head_blocks.is_empty()", "|| false", "complete-coins"],
  ["coins B binding", CORE + "engine/canonical_genesis.rs", "snapshot.maybe_best_block != Some(proof.hash())", "snapshot.maybe_best_block.is_none()", "complete-coins"],
  ["actual stored complete body", NODE + "storage/canonical_bootstrap/proof.rs", "CanonicalGenesisProof::prove(selected.spec(), &records.body)", "CanonicalGenesisProof::prove(selected.spec(), &selected.spec().genesis_block())", "stored-authority"],
  ["genesis-only ancestry", NODE + "storage/canonical_bootstrap/proof.rs", "records.headers.len() != 1", "records.headers.is_empty()", "stored-authority"],
  ["stored exact accumulation", NODE + "storage/canonical_bootstrap/proof.rs", "records.active[0] != position", "false", "stored-authority"],
  ["stored BASIC owner", NODE + "storage/canonical_bootstrap/proof.rs", "matches!(owner, IndexLifecycle::Active { .. }) && maybe_lock_protection.is_none()", "false", "stored-basic"],
  ["stored BASIC full body and no undo", NODE + "storage/canonical_bootstrap/proof.rs", "BasicFilterInputs::from_historical(genesis.block(), position, None)", "BasicFilterInputs::from_historical(&fixture, position, None)", "stored-basic"],
  ["strict explicit counts", NODE + "storage/canonical_bootstrap/records.rs", "maybe_counts.ok_or(Refusal::MissingRecord)?", "maybe_counts.unwrap_or_default()", "strict-records"],
  ["no C or H inventory", NODE + "storage/canonical_bootstrap/records.rs", '"coins" => key == "B",', '"coins" => true,', "no-genesis-coins"],
  ["live instance binding", OPENER, "!Arc::ptr_eq(&self.publication, &store.filter_publication)", "false", "store-generation"],
  ["consuming capability recheck", OPENER, "self.recovered.check(&self.store)?;", "let _ignored = &self.recovered;", "consuming-owner"],
  ["atomic SyncAll publication", OPENER, "store.db.batch().durability(Some(FjallPersistMode::SyncAll))", "store.db.batch().durability(None)", "atomic-publication"],
  ["uncertain commit refuses capability", OPENER, "batch.commit().map_err(|_| Refusal::CommitUncertain)?;", "let _ignored = batch.commit();", "atomic-publication"],
  ["pre-original detached proof", OPENER, "prove_detached(records, selected)", "prove_fixture(records, selected)", "pre-effect-proof"],
  ["retained application lease", OPENER, "open_lease::OperationLease::acquire(path)", "open_lease::OperationLease::unlocked(path)", "atomic-publication"],
  ["last-handle lease retention", NODE + "storage/fjall_store/opening.rs", "_lease: Arc::new(lease)", "_lease: Arc::new(dummy_lease)", "retained-store-lock"],
  ["backend path bound to lease", NODE + "storage/fjall_store/opening.rs", "Database::builder(lease.source())", "Database::builder(unproved_path)", "retained-store-lock"],
  ["application lock is mandatory", NODE + "storage/fjall_store/open_lease.rs", "file.try_lock()", "file.metadata()", "application-lock"],
  ["exclusive capability fields", OPENER, "pub struct OpenedCanonicalGenesis {\n    store: FjallNodeStore,", "pub struct OpenedCanonicalGenesis {\n    pub store: FjallNodeStore,", "sealed-fields"],
  ["parent engine lock", INSPECTION, "let _engine_lock = lock_source(source)?;", "let _engine_lock = ();", "cleanup-before-proof"],
  ["process isolation", INSPECTION, "worker::recover,", "worker::raw_recover,", "isolated-recovery"],
  ["source recheck", INSPECTION, ".recheck(source)", ".skip_recheck(source)", "cleanup-before-proof"],
  ["checked scratch cleanup", INSPECTION, "scratch.cleanup()?;", "let _ignored = scratch.cleanup();", "cleanup-before-proof"],
  ["cleanup cannot be conditional", INSPECTION, "scratch.cleanup()?;", "if false { scratch.cleanup()?; }", "cleanup-before-proof"],
  ["child exit status", NODE + "storage/fjall_store/inspection/worker.rs", "if !status.success() || !delivered", "if !delivered", "child-result"],
  ["child reaping", NODE + "storage/fjall_store/inspection/worker.rs", "match child.wait()", "match child.try_wait()", "child-reaping"],
  ["proof before recovery", NODE + "chainstate/fjall_store/canonical_genesis.rs", "let proof = recovered", "initialize_configured(&store, FlushPolicyTime::from_unix_seconds(0), FlushPolicyTime::from_unix_seconds(0), 0, false, u64::MAX, mode)?; let proof = recovered", "recovery-after-proof"],
  ["saved active BASIC earns owner", NODE + "sync/canonical_genesis/basic.rs", "return Ok(BasicFilterStartupMode::Enabled);", "return Ok(BasicFilterStartupMode::PreserveSaved);", "saved-basic-owner"],
  ["checkpoint full ancestry", CHECKPOINT, "stored.as_slice() == active_chain", "stored.last() == active_chain.last()", "checkpoint-counts"],
  ["checkpoint malformed propagation", CHECKPOINT, ".transpose()?", ".transpose().unwrap_or_default()", "checkpoint-counts"],
  ["checkpoint before invalidation", CHECKPOINT, "// Counts describe the entire stored ancestry.", "control.invalidate_append()?; // Counts describe the entire stored ancestry.", "checkpoint-counts"],
  ["no inferred empty checkpoint counts", CHECKPOINT, "encode_chain_meta(active_chain, maybe_counts.as_ref())?", "encode_chain_meta(active_chain, Some(&HashMap::new()))?", "checkpoint-counts"],
  ["activation history gate", DAEMON, ".require_ordinary_history()", ".ordinary_history()", "activation-before-open"],
  ["activation relay default boundary", DAEMON, "|| runtime.relay.enabled", "|| false", "activation-before-open"],
  ["raw daemon opener substitution", DAEMON, "FjallNodeStore::open_canonical_genesis(data_dir, chain)", "FjallNodeStore::open(data_dir)", "daemon-sealed-opener"],
  ["daemon borrowed runtime substitution", DAEMON, "ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)", "ManagedRpcContext::from_runtime_config_with_network_handle(runtime, borrowed, None)", "daemon-consumption"],
  ["daemon dispatch cardinality", RPC + "bin/open-bitcoind.rs", "    run_application()", "    run_application()?; FjallNodeStore::maybe_run_inspection_worker(); Ok(())", "pre-tokio-dispatch"],
  ["daemon dispatch moved after Tokio", RPC + "bin/open-bitcoind.rs", "    if let Some(code) = FjallNodeStore::maybe_run_inspection_worker() {\n        std::process::exit(code);\n    }\n    run_application()", "    run_application()?; if let Some(code) = FjallNodeStore::maybe_run_inspection_worker() { std::process::exit(code); } Ok(())", "pre-tokio-dispatch"],
  ["CLI dispatch cardinality", CLI + "bin/open-bitcoin.rs", "    let cli = OperatorCli::parse();", "    let cli = OperatorCli::parse(); open_bitcoin_node::FjallNodeStore::maybe_run_inspection_worker();", "pre-parser-dispatch"],
  ["CLI dispatch moved after parser", CLI + "bin/open-bitcoin.rs", "    if let Some(code) = open_bitcoin_node::FjallNodeStore::maybe_run_inspection_worker() {\n        std::process::exit(code);\n    }\n    let cli = OperatorCli::parse();", "    let cli = OperatorCli::parse(); if let Some(code) = open_bitcoin_node::FjallNodeStore::maybe_run_inspection_worker() { std::process::exit(code); }", "pre-parser-dispatch"],
  ["qualified genesis excludes ordinary maintenance", RPC + "bin/open-bitcoind.rs", "let maybe_checkpoint_worker = if canonical_genesis", "let maybe_checkpoint_worker = if false", "genesis-maintenance"],
  ["RPC selected params", FACTORY, "consensus_params(opened.genesis())", "config.sync.runtime.network.consensus_params()", "sealed-rpc-factory"],
  ["RPC wallet unavailable", FACTORY, "WalletState::Unavailable,", "WalletState::Local(Wallet::default()),", "sealed-rpc-factory"],
  ["RPC borrowed capability", FACTORY, "opened: OpenedCanonicalGenesis,", "opened: &OpenedCanonicalGenesis,", "sealed-signature"],
  ["RPC recovery before proof", FACTORY, "preflight(config, &opened)?;", "recover_mempool_snapshot_from_store_handle(&store)?; preflight(config, &opened)?;", "sealed-rpc-factory"],
  ["RPC wrong selected network", FACTORY, "opened.genesis().chain() != chain", "false", "rpc-selected-history"],
  ["RPC library worker registration", RPC + "lib.rs", "open_bitcoin_node::register_inspection_worker!();", "", "registration"],
  ["RPC worker cannot be cfg-disabled", RPC + "lib.rs", "#[cfg(test)]\nopen_bitcoin_node::register_inspection_worker!();", "#[cfg(any())]\n#[cfg(test)]\nopen_bitcoin_node::register_inspection_worker!();", "registration"],
  ["qualification module cannot be cfg-disabled", NODE + "storage/canonical_bootstrap/tests.rs", '#[path = "qualification_tests.rs"]\nmod qualification_tests;', '#[cfg(any())]\n#[path = "qualification_tests.rs"]\nmod qualification_tests;', "registration"],
  ["public proof module cannot be cfg-disabled", CONSENSUS + "chain_spec.rs", "pub mod bootstrap;", "#[cfg(any())]\npub mod bootstrap;", "registration"],
  ["only Status gains observation", CLI + "operator/runtime/support.rs", "matches!(args.command, SyncCommand::Status)", "true", "status-only"],
  ["CLI existing-only opener", OBSERVER, "FjallNodeStore::reopen_canonical_genesis_existing(data_dir, selected_chain)", "FjallNodeStore::open_canonical_genesis(data_dir, selected_chain)", "existing-observation"],
  ["CLI capability consumption", OBSERVER, "DurableSyncRuntime::open_canonical_genesis_configured(", "DurableSyncRuntime::open_configured(", "existing-observation"],
  ["reviewed predicate family: explicit counts", NODE + "storage/canonical_bootstrap/records.rs", "if !counts.is_empty() {", "if !counts.is_empty() && false {", "strict-records"],
  ["reviewed predicate family: inventory refusal", NODE + "storage/canonical_bootstrap/records.rs", "if !supported {", "if !supported && false {", "no-genesis-coins"],
  ["reviewed predicate family: BASIC projection", NODE + "storage/canonical_bootstrap/proof.rs", "!= genesis.hash()\n    {", "!= genesis.hash() && false\n    {", "stored-basic"],
  ["reviewed predicate family: uncertain child retention", INSPECTION, "if !reaped {", "if !reaped && false {", "cleanup-before-proof"],
  ["reviewed predicate family: delivered result", NODE + "storage/fjall_store/inspection/worker.rs", "if !status.success() || !delivered {", "if !status.success() || !delivered && false {", "child-result"],
  ["reviewed predicate family: selected runtime", NODE + "sync/canonical_genesis.rs", "if opened.genesis().chain() != selected {", "if opened.genesis().chain() != selected && false {", "runtime-owner"],
  ["reviewed predicate family: saved BASIC recovery", NODE + "sync/canonical_genesis/basic.rs", "store.resolved_basic_filter_indexing_after_startup()? == Some(true)\n    {", "store.resolved_basic_filter_indexing_after_startup()? == Some(true) && false\n    {", "saved-basic-owner"],
  ["reviewed predicate family: capability early success", OPENER, "self.recovered.check(&self.store)?;", "if true { return Ok((self.store, self.recovered)); } self.recovered.check(&self.store)?;", "consuming-owner"],
  ["reviewed predicate family: activation branch", DAEMON, "if activation {", "if activation && false {", "activation-before-open"],
  ["reviewed predicate family: checkpoint counts", CHECKPOINT, "if stored.as_slice() == active_chain {", "if stored.as_slice() == active_chain || true {", "checkpoint-counts"],
  ["predicate audit: unavailable wallet branch", RPC + "context/wallet_state.rs", "if matches!(self.wallet_state, WalletState::Unavailable) {", "if matches!(self.wallet_state, WalletState::Unavailable) && false {", "wallet-unavailable"],
  ["predicate audit: Status branch", CLI + "operator/runtime/support.rs", "if matches!(args.command, SyncCommand::Status) {", "if matches!(args.command, SyncCommand::Status) && false {", "status-only"],
  ["predicate audit: storage-free no-trigger branch", DAEMON, "if !has_durable_trigger {", "if !has_durable_trigger && false {", "daemon-sealed-opener"],
  ["predicate audit: exact serialized body", NODE + "storage/canonical_bootstrap/records.rs", "!= body_bytes {", "!= body_bytes && false {", "strict-records"],
  ["predicate audit: BASIC early success", NODE + "storage/canonical_bootstrap/proof.rs", "let maybe_locks = rows.get(fjall_store::PRUNE_LOCKS_KEY.as_bytes());", "if true { return Ok(()); } let maybe_locks = rows.get(fjall_store::PRUNE_LOCKS_KEY.as_bytes());", "stored-basic"],
  ["predicate audit: saved BASIC early success", NODE + "sync/canonical_genesis/basic.rs", "    if matches!(requested, BasicFilterStartupMode::PreserveSaved)", "    if true { return Ok(requested); } if matches!(requested, BasicFilterStartupMode::PreserveSaved)", "saved-basic-owner"],
  ["predicate audit: lock early success", LEASE, "file.try_lock().map_err(|error| match error {", "if true { return Ok(Self { source, _file: file }); } file.try_lock().map_err(|error| match error {", "application-lock"],
  ["predicate audit: daemon dispatch branch", RPC + "bin/open-bitcoind.rs", "if let Some(code) = FjallNodeStore::maybe_run_inspection_worker() {", "if let Some(code) = FjallNodeStore::maybe_run_inspection_worker() && false {", "pre-tokio-dispatch"],
  ["predicate audit: CLI dispatch branch", CLI + "bin/open-bitcoin.rs", "if let Some(code) = open_bitcoin_node::FjallNodeStore::maybe_run_inspection_worker() {", "if let Some(code) = open_bitcoin_node::FjallNodeStore::maybe_run_inspection_worker() && false {", "pre-parser-dispatch"],
];

for (const [file, start, rule] of [
  [CORE + "engine/canonical_genesis.rs", "if snapshot.maybe_best_block != Some(proof.hash())", "complete-coins"],
  [NODE + "storage/canonical_bootstrap/proof.rs", "if records.identity_hash != genesis.hash()", "stored-authority"],
  [NODE + "storage/canonical_bootstrap/proof.rs", "if records.headers[0] != expected_header", "stored-authority"],
  [OPENER, "if !Arc::ptr_eq(&self.publication, &store.filter_publication)", "store-generation"],
  [OPENER, "if store.db.seqno() != self.sequence", "store-generation"],
  [FACTORY, "if opened.genesis().chain() != chain", "rpc-selected-history"],
  [FACTORY, "if config.sync.is_enabled()", "rpc-selected-history"],
] as const) {
  test(`conditional guard bypass refuses ${file}:${start}`, async () => {
    // Arrange: wrap the actual entire condition; every old predicate token remains.
    const source = real.get(file)!;
    expect(source.split(start)).toHaveLength(2);
    const offset = source.indexOf(start);
    const before = source.slice(offset, source.indexOf("{", offset) + 1);
    const after = `if false && (${before.slice(3, -1).trim()}) {`;
    expect(source.split(before)).toHaveLength(2);
    expect(before).not.toBe(after);
    const changed = source.replace(before, after);
    expect(changed).not.toBe(source);
    // Act / Assert
    expect((await checkPhase166(new Map(real).set(file, changed))).some(row => row.file === file && row.rule === rule)).toBe(true);
  });
}

test("current bounded source and independently reproduced vectors pass", async () => {
  expect(await checkPhase166(real)).toEqual([]);
}, 30_000);

for (const [name, file, before, after, rule] of mutants) {
  test(`semantic mutant refuses ${name}`, async () => {
    // Arrange
    const original = real.get(file)!;
    expect(original.split(before)).toHaveLength(2);
    expect(before).not.toBe(after);
    const changed = original.replace(before, after);
    expect(changed).not.toBe(original);
    const inputs = new Map(real).set(file, changed);
    // Act
    const findings = await checkPhase166(inputs);
    // Assert
    expect(findings.some(row => row.file === file && row.rule === rule)).toBe(true);
  });
}

for (const field of ["body", "proof", "filter", "predecessor", "evidence-class"]) {
  test(`independent frozen vector rejects changed ${field}`, async () => {
    // Arrange
    const value = JSON.parse(real.get(VECTOR)!);
    if (field === "body") value.genesis[0].block_hex = "00";
    if (field === "proof") value.genesis[0].proof_le = "ff".repeat(32);
    if (field === "filter") value.genesis[0].basic.encoded = "00";
    if (field === "predecessor") value.genesis[0].basic.predecessor_raw = "01".repeat(32);
    if (field === "evidence-class") value.evidence_class = "actual-daemon-differential";
    // Act
    const findings = await checkPhase166(new Map(real).set(VECTOR, JSON.stringify(value)));
    // Assert
    expect(findings.some(row => row.file === VECTOR && row.rule === "vectors")).toBe(true);
  }, 30_000);
}

test("missing required input refuses visibly", async () => {
  // Arrange
  const inputs = new Map(real);
  inputs.delete(OPENER);
  // Act
  const findings = await checkPhase166(inputs);
  // Assert
  expect(findings.some(row => row.file === OPENER && row.rule === "input")).toBe(true);
});

test("bounded inventory has unique fixed membership", () => {
  expect(new Set(INPUT_FILES).size).toBe(INPUT_FILES.length);
  expect(INPUT_FILES.length).toBeLessThan(60);
  expect(INPUT_FILES.some(file => file.includes(".planning/"))).toBe(false);
});

test("import is silent and does not run the CLI or oracle", () => {
  // Arrange / Act
  const result = spawnSync(process.execPath, ["-e", 'await import("./scripts/check-phase166-canonical-bootstrap.ts");'],
    { cwd: new URL("..", import.meta.url), encoding: "utf8", timeout: 10_000 });
  // Assert
  expect(result.error).toBeUndefined();
  expect(result.status).toBe(0);
  expect(result.stdout).toBe("");
  expect(result.stderr).toBe("");
});
