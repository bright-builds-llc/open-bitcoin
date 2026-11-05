import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { checkPhase167, loadBoundedInputs, loadInputs } from "./check-phase167-candidate-history.ts";
import { OFFLINE_COMMANDS, STEP_LABELS, VERIFY_BASE_SHA, VERIFY_FILE } from "./check-phase167-candidate-history/contracts.ts";
import { CONTROLLED_NATIVE_PREFIX, CONTROLLED_NATIVE_STEPS, CONTROLLED_NATIVE_SUFFIX, verificationHistoricalBase } from "./check-phase164-canonical-identity/historical-tooling.ts";

const source = (file: string) => readFileSync(resolve(import.meta.dir, "..", file), "utf8");
const real = loadInputs();
const SOURCE_MUTATIONS = [
  ["selected-halving", "packages/open-bitcoin-consensus/src/chain_spec/rules.rs", "CanonicalChain::Regtest => 150,", "CanonicalChain::Regtest => 151,"],
  ["coinbase-maturity", "packages/open-bitcoin-consensus/src/chain_spec/rules.rs", "pub const fn coinbase_maturity(&self) -> u32 {\n        100", "pub const fn coinbase_maturity(&self) -> u32 {\n        99"],
  ["exception-identity", "packages/open-bitcoin-consensus/src/candidate_history/rules.rs", ".maybe_script_flag_exception(facts.candidate_hash)", ".maybe_script_flag_exception(facts.parent_hash)"],
  ["deployment-additions", "packages/open-bitcoin-consensus/src/candidate_history/rules.rs", "flags |= addition;", "flags = addition;"],
  ["sigop-budget", "packages/open-bitcoin-consensus/src/block.rs", "sigop_budget.add_transaction(coinbase, &[], crate::ScriptVerifyFlags::NONE)?;", "let _ = coinbase;"],
  ["script-interpreter", "packages/open-bitcoin-consensus/src/transaction.rs", "verify_input_script(ScriptInputVerificationContext {", "skip_input_script(ScriptInputVerificationContext {"],
  ["script-parent-bridge", "packages/open-bitcoin-consensus/src/script.rs", "witness::verify_input_script(context)", "Ok(())"],
  ["witness-execution", "packages/open-bitcoin-consensus/src/script/witness.rs", "verify_top_stack_true(&stack)?;", "let _ = &stack;"],
  ["current-tip-context", "packages/open-bitcoin-chainstate/src/engine.rs", "self.selected_candidate_context(&self.active_chain, &block.header, &[], 0)?;", "self.selected_candidate_context(&[], &block.header, &[], 0)?;"],
  ["eleven-sample-mtp", "packages/open-bitcoin-chainstate/src/engine.rs", ".take(MEDIAN_TIME_PAST_WINDOW - usize::from(maybe_new_time.is_some()))", ".take(12)"],
  ["bip30-recheck", "packages/open-bitcoin-consensus/src/candidate_history/rules.rs", "if facts.height >= facts.network.bip30_recheck_height() {", "if facts.height > facts.network.bip30_recheck_height() {"],
  ["bip30-scan-gate", "packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs", "&& !history.rules().requires_bip30_scan()", "&& true"],
  ["lossless-overwrite", "packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs", "batch.overwritten.push(OverwrittenCoin {", "let _ = OverwrittenCoin {"],
  ["prior-coin-dto", "packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs", "transaction_index: index as u32,\n                outpoint,\n                previous_coin,\n                new_coin: Coin {", "transaction_index: index as u32,\n                outpoint,\n                new_coin: Coin {"],
  ["sealed-history-opener", "packages/open-bitcoin-node/src/sync/canonical_history.rs", "opened\n            .into_parts()", "opened\n            .raw_parts()"],
  ["signet-refusal", "packages/open-bitcoin-consensus/src/chain_spec.rs", "CanonicalChain::Mainnet | CanonicalChain::Testnet3 | CanonicalChain::Signet => {", "CanonicalChain::Mainnet | CanonicalChain::Testnet3 => {"],
  ["seeded-origin", "packages/open-bitcoin-consensus/src/candidate_history.rs", "history.origin = HistoryOrigin::IndependentlySeeded;", "history.origin = HistoryOrigin::GenesisLinked;"],
  ["fixture-promotion", "packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs", "if !self.work_authority.is_mechanical_fixture()\n            || history.origin() != HistoryOrigin::IndependentlySeeded", "if false\n            || history.origin() != HistoryOrigin::IndependentlySeeded"],
] as const;

test("current reviewed source registers every Phase167 offline command", () => {
  // Arrange
  const inputs = new Map(real);
  // Act
  const findings = checkPhase167(inputs);
  // Assert
  expect(findings).toEqual([]);
});

for (const [name, file, before, after] of SOURCE_MUTATIONS) {
  test(`current Phase167 source refuses ${name} mutation`, () => {
    // Arrange
    const original = source(file);
    expect(original.split(before)).toHaveLength(2);
    const inputs = new Map(real).set(file, original.replace(before, after));
    // Act
    const findings = checkPhase167(inputs);
    // Assert
    expect(findings).toContainEqual(expect.objectContaining({ file, rule: `behavior:${name}` }));
  });
}

test("current source refuses an unreviewed byte outside named behavior", () => {
  // Arrange
  const file = "packages/open-bitcoin-consensus/src/chain_spec/rules.rs";
  const inputs = new Map(real).set(file, source(file) + "\n");
  // Act
  const findings = checkPhase167(inputs);
  // Assert
  expect(findings).toContainEqual(expect.objectContaining({ file, rule: "source-pin" }));
});

test("historical manifest cannot silently drop one pinned required block", () => {
  // Arrange
  const file = "packages/testdata/phase167/manifest.json";
  const manifest = JSON.parse(source(file)) as { required: unknown[] };
  manifest.required.pop();
  const inputs = new Map(real).set(file, JSON.stringify(manifest));
  // Act
  const findings = checkPhase167(inputs);
  // Assert
  expect(findings).toContainEqual(expect.objectContaining({ file, rule: "corpus" }));
});

test("exception replacement must precede deployment additions", () => {
  // Arrange
  const file = "packages/open-bitcoin-consensus/src/candidate_history/rules.rs";
  const original = source(file);
  const before = ".maybe_script_flag_exception(facts.candidate_hash)";
  const after = "for (activation, addition) in [";
  expect(original.indexOf(before)).toBeLessThan(original.indexOf(after));
  const changed = original.replace(before, "REMOVED_EXCEPTION").replace(after, before).replace("REMOVED_EXCEPTION", after);
  // Act
  const findings = checkPhase167(new Map(real).set(file, changed));
  // Assert
  expect(findings).toContainEqual(expect.objectContaining({ file, rule: "behavior:exception-order" }));
});

test("missing named native Rust regression refuses", () => {
  // Arrange
  const file = "packages/open-bitcoin-consensus/src/candidate_history/rules/tests.rs";
  const before = "#[test]\nfn phase167_candidate_rules_exception_replacement_precedes_additions(";
  const original = source(file);
  expect(original.split(before)).toHaveLength(2);
  // Act
  const findings = checkPhase167(new Map(real).set(file, original.replace(before, before.replace("#[test]", "// disabled"))));
  // Assert
  expect(findings).toContainEqual(expect.objectContaining({ file, rule: "native-test" }));
});

test("fixed Knots source and exact input membership cannot drift", () => {
  // Arrange
  const file = "packages/bitcoin-knots/src/validation.cpp";
  // Act
  const pinned = checkPhase167(new Map(real).set(file, real.get(file)! + "\n"));
  const extended = checkPhase167(new Map(real).set("unreviewed/extra", "data"));
  // Assert
  expect(pinned).toContainEqual(expect.objectContaining({ file, rule: "knots-pin" }));
  expect(extended).toContainEqual(expect.objectContaining({ file: "<inventory>", rule: "inventory" }));
});

test("script child byte drift cannot hide behind an intact parent bridge", () => {
  // Arrange
  const file = "packages/open-bitcoin-consensus/src/script/legacy.rs";
  // Act
  const findings = checkPhase167(new Map(real).set(file, real.get(file)! + "\n"));
  // Assert
  expect(findings).toContainEqual(expect.objectContaining({ file, rule: "source-pin" }));
});

test("loader refuses oversized or nonfinite source bounds before reading", () => {
  // Arrange / Act / Assert
  expect(() => loadBoundedInputs(1024)).toThrow("bounded regular source input required");
  expect(() => loadBoundedInputs(Number.POSITIVE_INFINITY)).toThrow("invalid bounded source input limit");
});

test("historical manifest source membership and corpus hashes cannot drift", () => {
  // Arrange
  const file = "packages/testdata/phase167/manifest.json";
  const manifest = JSON.parse(source(file)) as { sources: unknown[] };
  manifest.sources.pop();
  // Act
  const inventory = checkPhase167(new Map(real).set(file, JSON.stringify(manifest)));
  const data = checkPhase167(new Map(real).set("packages/testdata/phase167/blocks.json", real.get("packages/testdata/phase167/blocks.json")! + "\n"));
  // Assert
  expect(inventory).toContainEqual(expect.objectContaining({ file, rule: "corpus" }));
  expect(data).toContainEqual(expect.objectContaining({ file: "packages/testdata/phase167/blocks.json", rule: "corpus" }));
});

const verifier = real.get(VERIFY_FILE)!;
const step = (index: number) => `run_step "${STEP_LABELS[index]}" ${OFFLINE_COMMANDS[index]}`;
function verifyMutation(before: string, after: string, rule: string): void {
  expect(verifier.split(before)).toHaveLength(2);
  const findings = checkPhase167(new Map(real).set(VERIFY_FILE, verifier.replace(before, after)));
  expect(findings).toContainEqual(expect.objectContaining({ file: VERIFY_FILE, rule }));
}

for (const [name, before, after, rule] of [
  ["missing selector", CONTROLLED_NATIVE_STEPS[0] + "\n", "", "native-stress"],
  ["duplicate selector", CONTROLLED_NATIVE_STEPS[0] + "\n", CONTROLLED_NATIVE_STEPS[0] + "\n" + CONTROLLED_NATIVE_STEPS[0] + "\n", "native-stress"],
  ["reordered selectors", CONTROLLED_NATIVE_STEPS[0] + "\n" + CONTROLLED_NATIVE_STEPS[1], CONTROLLED_NATIVE_STEPS[1] + "\n" + CONTROLLED_NATIVE_STEPS[0], "native-stress"],
  ["wrong harness threads", CONTROLLED_NATIVE_STEPS[0], CONTROLLED_NATIVE_STEPS[0].replace("--test-threads=16", "--test-threads=4"), "native-stress"],
  ["wrong CLI family", CONTROLLED_NATIVE_STEPS[8], CONTROLLED_NATIVE_STEPS[8].replace("phase167_cli", "phase167_cli_missing"), "native-stress"],
  ["missing stress block", CONTROLLED_NATIVE_PREFIX, "", "native-scope"],
  ["wrong complete-suite threads", "export RUST_TEST_THREADS=4", "export RUST_TEST_THREADS=16", "native-scope"],
  ["missing inherited-value restore", '  export RUST_TEST_THREADS="$saved_rust_test_threads"', "  unset RUST_TEST_THREADS", "native-scope"],
  ["missing absent-value restore", "  unset RUST_TEST_THREADS", "  :", "native-scope"],
  ["early restore", CONTROLLED_NATIVE_SUFFIX, "", "native-scope"],
] as const) {
  test(`native verifier refuses ${name}`, () => {
    // Arrange / Act / Assert: mutate one reviewed scheduling or restoration edge.
    verifyMutation(before, after, rule);
  });
}

for (const [kind, lineOf] of [
  ["declared", (index: number) => OFFLINE_COMMANDS[index]],
  ["executed", step],
] as const) {
  for (let index = 0; index < OFFLINE_COMMANDS.length; index++) {
    test(`${kind} Phase167 command ${index} refuses omission and duplication`, () => {
      // Arrange / Act / Assert
      const line = lineOf(index);
      const before = kind === "declared" ? "\n" + line + "\n" : line + "\n";
      const duplicate = kind === "declared" ? "\n" + line + "\n" + line + "\n" : line + "\n" + line + "\n";
      verifyMutation(before, kind === "declared" ? "\n" : "", "registration");
      verifyMutation(before, duplicate, "registration");
    });
  }
  test(`${kind} Phase167 commands refuse changed order`, () => {
    // Arrange / Act / Assert
    const first = lineOf(2), second = lineOf(3);
    verifyMutation(first + "\n" + second, second + "\n" + first, "registration");
  });
}

test("Phase167 native verifier refuses check-to-write and acquisition modes", () => {
  // Arrange / Act / Assert
  verifyMutation("\nbun run scripts/phase167-bip30-fixtures.ts --check\n", "\nbun run scripts/phase167-bip30-fixtures.ts --write\n", "offline");
  verifyMutation(step(4), step(4).replace("--check", "--write"), "offline");
  verifyMutation(step(0), step(0).replace("--check", "--acquire"), "offline");
});

test("Phase167 native verifier refuses a network fetch registration", () => {
  // Arrange / Act / Assert
  verifyMutation(step(7) + "\n", step(7) + "\nrun_step \"fetch Phase167 corpus\" curl https://example.invalid/data\n", "offline");
});

test("Phase167 verifier requires a real run_step instead of a commented decoy", () => {
  // Arrange / Act / Assert
  verifyMutation(step(6), "# " + step(6), "runner");
});

test("current verifier projects to the frozen original and changed additions refuse", () => {
  // Arrange
  const sha = (source: string) => createHash("sha256").update(source).digest("hex");
  // Act / Assert
  expect(sha(verificationHistoricalBase(verifier))).toBe(VERIFY_BASE_SHA);
  expect(() => verificationHistoricalBase(verifier.replace(step(5), "# skipped Phase167 Regtest test"))).toThrow();
});
