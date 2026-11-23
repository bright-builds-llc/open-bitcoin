import { isDeepStrictEqual } from "node:util";
import { decodeBlock, sha256 } from "../phase167-historical-corpus.ts";
import { BITS, proof } from "../phase167-regtest-serialization.ts";
import { REGTEST_CORPUS } from "./regtest-case.ts";
import { readRegtestHistory } from "./regtest-history.ts";
import { summarizeCoins, type Coin } from "./state.ts";

export const CHECKPOINT_ID = "v2-6-crec-04";
export const CHECKPOINT_FORMAT = "open-bitcoin-crec-04-v1";
export const CHECKPOINT_SOURCE =
  "packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/observation/recovery.rs";

const CASES = [
  { name: "before-body", exit: 75, height: 1 },
  { name: "before-chain-meta", exit: 75, height: null },
  { name: "after-basic-commit", exit: 75, height: 2 },
  { name: "inconsistent-coins-tip", exit: 0, height: null },
  { name: "inconsistent-header-work", exit: 0, height: null },
] as const;

type RecordValue = Record<string, unknown>;
const HEX256 = /^[a-f0-9]{64}$/;

function object(value: unknown): RecordValue | null {
  return value && typeof value === "object" && !Array.isArray(value) ? value as RecordValue : null;
}

function hash(value: unknown): boolean {
  return typeof value === "string" && HEX256.test(value);
}

export function readCheckpointCorpus(readText: (path: string) => string) {
  const history = readRegtestHistory(readText);
  const coins: Coin[] = [];
  const expected = history.blocks.slice(0, 2).map((block) => {
    const { transactions } = decodeBlock(Buffer.from(block.blockHex, "hex"), block.hash);
    if (transactions.length !== 1 || !transactions[0].coinbase) {
      throw new Error("Checkpoint fixture must use the canonical coinbase-only prefix");
    }
    const tx = transactions[0];
    for (const [vout, output] of tx.outputs.entries()) {
      coins.push({ txid: tx.txid, vout, sats: output.value, script: output.script, height: block.height, coinbase: true });
    }
    const digest = summarizeCoins(coins, block.height);
    return {
      tip: { height: block.height, hash: block.hash },
      chainwork: ((BigInt(block.height) + 1n) * proof(BITS)).toString(16).padStart(64, "0"),
      coins_sha256: digest.sha256, coins_count: digest.count, basic_height: block.height,
    };
  });
  return {
    corpusSha256: history.corpusSha256,
    fixtureSha256: sha256(readText(CHECKPOINT_SOURCE)),
    expected,
  };
}

export function checkCheckpointReceipt(value: unknown, readText: (path: string) => string): string[] {
  const record = object(value);
  const corpus = readCheckpointCorpus(readText);
  const cli = object(record?.cli_build);
  if (!record || record.format !== CHECKPOINT_FORMAT || record.status !== "passed"
    || record.network !== "regtest" || record.corpus !== REGTEST_CORPUS || record.corpus_sha256 !== corpus.corpusSha256
    || record.fixture_sha256 !== corpus.fixtureSha256 || !hash(record.native_log_sha256)
    || typeof cli?.source_commit !== "string" || !/^[a-f0-9]{40}$/.test(cli.source_commit)
    || !hash(cli.binary_sha256) || !hash(cli.build_log_sha256)
    || !isDeepStrictEqual(cli.cargo_options, ["--locked", "--manifest-path", "packages/Cargo.toml",
      "-p", "open-bitcoin-cli", "--bin", "open-bitcoin"])) {
    return [`${CHECKPOINT_ID}: recovery receipt lacks bound inputs, test source or operator build`];
  }
  if (!Array.isArray(record.cases) || record.cases.length !== CASES.length) {
    return [`${CHECKPOINT_ID}: incomplete checkpoint case matrix`];
  }
  const failures: string[] = [];
  for (const [index, expected] of CASES.entries()) {
    const observed = object(record.cases[index]);
    const operator = object(observed?.operator);
    const matchesCase = observed?.name === expected.name && observed.fixture_exit_status === expected.exit;
    if (expected.height === null) {
      if (!matchesCase || observed?.outcome !== "refused" || observed.source_unchanged !== true
        || Object.hasOwn(observed, "state") || operator?.error !== "chain_compatibility_refused"
        || typeof operator.exit_status !== "number" || !Number.isInteger(operator.exit_status) || operator.exit_status <= 0) {
        failures.push(`${CHECKPOINT_ID}: invalid refused-source observation for ${expected.name}`);
      }
      continue;
    }
    const state = corpus.expected[expected.height - 1];
    if (!matchesCase || observed?.outcome !== "proved_prefix"
      || !isDeepStrictEqual(observed.state, state)
      || !isDeepStrictEqual(operator, { exit_status: 0, tip: state.tip, chainwork: state.chainwork })) {
      failures.push(`${CHECKPOINT_ID}: invalid proved prefix for ${expected.name}`);
    }
  }
  return failures;
}
