import { decodeBlock, sha256 } from "../phase167-historical-corpus.ts";
import {
  BITS, ZERO, blockHash, display, encodeBlock, heightScript, proof, seal, type Body, type Tx,
} from "../phase167-regtest-serialization.ts";
import type { DaemonRpc } from "./daemon.ts";
import { readRegtestHistory, type RegtestBlock, type RegtestTip } from "./regtest-history.ts";
import { basicRow, observeKnotsCoins, SPEND_STATE_OBSERVER, summarizeCoins, type Coin } from "./state.ts";

export const SPEND_FORMAT = "open-bitcoin-crec-02-v1";
const DOUBLE_SPEND_REASON = "bad-txns-inputs-missingorspent";
const SUBSIDY = 5_000_000_000n;
type TimedBlock = RegtestBlock & { time: number };

function generate(parent: TimedBlock, tag: string, maybeSpend?: { coin: Coin; fee: bigint }) {
  const height = parent.height + 1;
  const time = parent.time + 600;
  const transactions: Tx[] = [{
    version: 1, lock_time: height,
    inputs: [{ txid: ZERO, vout: 0xffffffff, script: heightScript(height) + tag, sequence: 0xffffffff, witness: [] }],
    outputs: [{ sats: (SUBSIDY + (maybeSpend?.fee ?? 0n)).toString(), script: "51" }],
  }];
  if (maybeSpend) {
    const { coin, fee } = maybeSpend;
    transactions.push({
      version: 2, lock_time: 0,
      inputs: [{ txid: display(coin.txid), vout: coin.vout, script: "", sequence: 0xffffffff, witness: [] }],
      outputs: [{ sats: (BigInt(coin.sats) - fee).toString(), script: tag }],
    });
  }
  const body: Body = seal({
    header: { version: 4, parent: display(parent.hash), merkle: ZERO, time, bits: BITS, nonce: 0 },
    transactions,
  });
  return { height, hash: display(blockHash(body)), blockHex: encodeBlock(body).toString("hex"), time };
}

function decode(block: RegtestBlock) {
  return decodeBlock(Buffer.from(block.blockHex, "hex"), block.hash);
}

function outpoint(txid: string, vout: number) {
  return `${txid}:${vout}`;
}

function addBlockCoins(coins: Map<string, Coin>, block: RegtestBlock) {
  for (const tx of decode(block).transactions) {
    for (const input of tx.coinbase ? [] : tx.inputs) {
      const key = outpoint(input.txid, input.vout);
      const coin = coins.get(key);
      if (!coin || (coin.coinbase && block.height - coin.height < 100)) throw new Error(`Missing or immature coin ${key}`);
      coins.delete(key);
    }
    for (const [vout, output] of tx.outputs.entries()) {
      const key = outpoint(tx.txid, vout);
      if (coins.has(key)) throw new Error(`Unexpected duplicate coin ${key}`);
      coins.set(key, { txid: tx.txid, vout, sats: output.value, script: output.script, coinbase: tx.coinbase, height: block.height });
    }
  }
}

export function readSpendCorpus(readText: (path: string) => string) {
  const history = readRegtestHistory(readText);
  const prefix = history.blocks.slice(0, 100);
  const common = prefix[prefix.length - 1];
  const coin = decode(prefix[0]).transactions[0];
  const base = new Map<string, Coin>();
  for (const block of prefix) addBlockCoins(base, block);
  const funding = base.get(outpoint(coin.txid, 0));
  if (!funding || !funding.coinbase || funding.height !== 1 || funding.script !== "51") {
    throw new Error("Expected mature height-one first outpoint at the fork");
  }
  const parent = { ...common, time: decode(common).time };
  const a1 = generate(parent, "51", { coin: funding, fee: 1_000n });
  const b1 = generate(parent, "52", { coin: funding, fee: 2_000n });
  const bad = generate(b1, "53", { coin: funding, fee: 3_000n });
  const b2 = generate(b1, "52");
  const a2 = generate(a1, "51");
  const a3 = generate(a2, "51");
  const blocks = [a1, b1, bad, b2, a2, a3];
  if (new Set(blocks.map(({ hash }) => hash)).size !== blocks.length) throw new Error("Spend fork identities must be distinct");
  const aSpend = decode(a1).transactions[1].txid;
  const bSpend = decode(b1).transactions[1].txid;

  const runs = [
    { name: "original", start: { height: 0, hash: history.genesisHash }, end: a1,
      schedule: [{ block: a1, reason: null, active: a1 }, { block: b1, reason: "inconclusive", active: a1 }],
      active: [a1] },
    { name: "reject", start: a1, end: a1,
      schedule: [{ block: bad, reason: DOUBLE_SPEND_REASON, active: a1 }], active: [a1] },
    { name: "replacement", start: a1, end: b2,
      schedule: [{ block: b2, reason: null, active: b2 }], active: [b1, b2] },
    { name: "reconnect", start: b2, end: a3,
      schedule: [{ block: a2, reason: "inconclusive", active: b2 }, { block: a3, reason: null, active: a3 }],
      active: [a1, a2, a3] },
  ].map((run) => {
    const coins = new Map(base);
    for (const block of run.active) addBlockCoins(coins, block);
    return { ...run, utxos: summarizeCoins([...coins.values()], run.end.height),
      chainwork: ((BigInt(run.end.height) + 1n) * proof(BITS)).toString(16).padStart(64, "0") };
  });
  return {
    genesisHash: history.genesisHash, corpusSha256: history.corpusSha256, prefix, runs,
    spendSha256: sha256(JSON.stringify(blocks.map(({ blockHex }) => blockHex))),
    observerSha256: sha256(readText(SPEND_STATE_OBSERVER)),
    spendFacts: { funding: { txid: funding.txid, vout: funding.vout }, original_txid: aSpend, replacement_txid: bSpend },
  };
}

/** Use the actually retained replacement output for a new post-reopen spend. */
export function readRecoveryCorpus(readText: (path: string) => string) {
  const spend = readSpendCorpus(readText);
  const replacement = spend.runs[2];
  const coins = new Map<string, Coin>();
  for (const block of [...spend.prefix, ...replacement.active]) addBlockCoins(coins, block);
  const key = outpoint(spend.spendFacts.replacement_txid, 0);
  const funding = coins.get(key);
  if (!funding || funding.coinbase || funding.height !== 101) throw new Error("Missing replacement output for recovery");
  const resumed = generate(replacement.end, "53", { coin: funding, fee: 1_000n });
  addBlockCoins(coins, resumed);
  return {
    spend, replacement, resumed, funding: key,
    resumeSha256: sha256(Buffer.from(resumed.blockHex, "hex")),
    resumedCoins: summarizeCoins([...coins.values()], resumed.height),
    resumedWork: ((BigInt(resumed.height) + 1n) * proof(BITS)).toString(16).padStart(64, "0"),
  };
}

export type SpendCorpus = ReturnType<typeof readSpendCorpus>;
export type SpendRun = SpendCorpus["runs"][number];

export async function observeSpends(
  rpc: DaemonRpc, corpus: SpendCorpus, run: SpendRun, logs: string, name: "knots" | "open-bitcoin",
) {
  if (run.name === "original") {
    for (const block of corpus.prefix) await rpc.submit(block);
  }
  const steps = [];
  let finalTip: RegtestTip = run.start;
  for (const { block, reason, active } of run.schedule) {
    const result = await rpc.call("submitblock", [block.blockHex]);
    const info = await rpc.call("getblockchaininfo") as { chain: string; blocks: number; bestblockhash: string };
    if (result !== reason || info.chain !== "regtest" || info.blocks !== active.height || info.bestblockhash !== active.hash) {
      throw new Error(`${run.name} at ${block.height}: unexpected result or tip ${JSON.stringify({ result, active: info.bestblockhash })}`);
    }
    finalTip = { height: info.blocks, hash: info.bestblockhash };
    steps.push({ hash: block.hash, submitblock: result, active: finalTip });
  }
  const basic = basicRow(finalTip.hash, await rpc.call("getblockfilter", [finalTip.hash, "basic"]));
  return { name: run.name, final_tip: finalTip, steps, basic,
    ...(name === "knots" ? await observeKnotsCoins(rpc, logs, finalTip, `crec02-${run.name}.csv`) : {}) };
}
