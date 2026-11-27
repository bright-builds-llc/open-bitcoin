import { copyFileSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { sha256 } from "../phase167-historical-corpus.ts";
import type { DaemonRpc } from "./daemon.ts";
import { loggedCommand, OBSERVER_TIMEOUT_MS } from "./process.ts";
import { readRegtestHistory, type RegtestTip } from "./regtest-history.ts";

export const CLOSED_STATE_OBSERVER = "packages/open-bitcoin-rpc/tests/cdif04_closed_state.rs";
export const MAINNET_STATE_OBSERVER = "packages/open-bitcoin-rpc/tests/mainnet_prefix_closed_state.rs";
export const SPEND_STATE_OBSERVER = "packages/open-bitcoin-rpc/tests/crec02_closed_state.rs";
const STATE_FIXTURE = "docs/parity/fixtures/phase167-regtest-expectations.json";
const COIN_FIELDS = ["txid", "vout", "value", "coinbase", "height", "scriptPubKey"];
const HEX256 = /^[a-f0-9]{64}$/;
const MAX_ROWS = 4096;

type BasicRow = [hash: string, filter: string, header: string];
export type Coin = { txid: string; vout: number; sats: string; script: string; coinbase: boolean; height: number };

function hexBytes(value: unknown): value is string {
  return typeof value === "string" && value.length <= 65_536
    && value.length % 2 === 0 && /^[a-f0-9]*$/.test(value);
}

function natural(value: unknown, max: number): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0 && value <= max;
}

export function summarizeCoins(input: unknown, tipHeight: number) {
  if (!Array.isArray(input) || input.length > MAX_ROWS) throw new Error("UTXO projection outside row bounds");
  const rows: [string, number, string, string, boolean, number][] = input.map((value: unknown) => {
    const row = value as Partial<Coin> | null;
    if (typeof row?.txid !== "string" || !HEX256.test(row.txid) || !natural(row.vout, 0xffffffff)
      || typeof row.sats !== "string" || !/^(0|[1-9][0-9]{0,15})$/.test(row.sats)
      || !hexBytes(row.script) || typeof row.coinbase !== "boolean" || !natural(row.height, tipHeight)) {
      throw new Error("Invalid UTXO projection row");
    }
    return [row.txid, row.vout, row.sats, row.script, row.coinbase, row.height];
  });
  rows.sort((a, b) => a[0].localeCompare(b[0]) || a[1] - b[1]);
  if (rows.some(([txid, vout], index) => index > 0 && txid === rows[index - 1][0] && vout === rows[index - 1][1])) {
    throw new Error("Duplicate projected outpoint");
  }
  const sats = rows.reduce((sum, row) => sum + BigInt(row[2]), 0n);
  return { count: rows.length, total_sats: sats.toString(), sha256: sha256(JSON.stringify(rows)) };
}

export function summarizeKnotsCoins(csv: string, tipHeight: number) {
  const document = csv.trimEnd();
  const rows = document ? document.split("\n") : [];
  if (rows.length > MAX_ROWS) throw new Error("Knots ASCII dump exceeded row bounds");
  return summarizeCoins(rows.map((line) => {
    const [txid, vout, sats, coinbase, height, script, ...extra] = line.split(",");
    if (extra.length || !/^[0-9]+$/.test(vout ?? "") || !/^[0-9]+$/.test(height ?? "")
      || (coinbase !== "0" && coinbase !== "1")) {
      throw new Error("Invalid Knots ASCII dump row");
    }
    return { txid, vout: Number(vout), sats, coinbase: coinbase === "1", height: Number(height), script };
  }), tipHeight);
}

function basicSummary(rows: BasicRow[]) {
  return { count: rows.length, tip_header: rows[rows.length - 1][2], sha256: sha256(JSON.stringify(rows)) };
}

export function basicRow(hash: string, value: unknown): BasicRow {
  const row = value as { filter?: unknown; header?: unknown } | null;
  if (!hexBytes(row?.filter) || row.filter.length === 0 || typeof row.header !== "string" || !HEX256.test(row.header)) {
    throw new Error(`Invalid BASIC response for ${hash}`);
  }
  return [hash, row.filter, row.header];
}

export function checkWork(value: unknown): string {
  if (typeof value !== "string" || !HEX256.test(value)) throw new Error("Invalid exact chainwork");
  return value;
}

export function readStateCorpus(readText: (path: string) => string) {
  const history = readRegtestHistory(readText);
  const fixture = readText(STATE_FIXTURE);
  const { rows } = JSON.parse(fixture) as { rows: { block_hash_display: string; chain_work_hex: string;
    utxos: unknown; basic: { encoded: string; header_display: string } }[] };
  const tips = [{ height: 0, hash: history.genesisHash }, ...history.blocks];
  if (!Array.isArray(rows) || rows.length !== tips.length || rows.some((row, index) => row.block_hash_display !== tips[index].hash)) {
    throw new Error("Regtest expected state has a different canonical chain");
  }
  const last = rows[rows.length - 1];
  const { height, hash } = tips[tips.length - 1];
  return {
    ...history,
    expectationSha256: sha256(fixture),
    observerSha256: sha256(readText(CLOSED_STATE_OBSERVER)),
    expected: {
      tip: { height, hash },
      chainwork: checkWork(last.chain_work_hex),
      utxos: summarizeCoins((last.utxos as Coin[]).map((row) => ({
        ...row, txid: Buffer.from(row.txid, "hex").reverse().toString("hex"),
      })), height),
      basic: basicSummary(rows.map((row) => basicRow(row.block_hash_display, {
        filter: row.basic.encoded, header: row.basic.header_display,
      }))),
    },
  };
}

export type StateCorpus = ReturnType<typeof readStateCorpus>;

export async function observeLiveState(
  rpc: DaemonRpc, corpus: StateCorpus, run: string, name: "knots" | "open-bitcoin",
) {
  let finalTip: RegtestTip = { height: 0, hash: corpus.genesisHash };
  for (const block of corpus.blocks) {
    finalTip = await rpc.submit(block);
    if (block.height % 50 === 0) console.log(`CDIF-04: ${name} accepted height ${block.height}`);
  }
  return observeStateAtTip(rpc, corpus, run, name, finalTip);
}

export async function observeStateAtTip(
  rpc: DaemonRpc, corpus: StateCorpus, run: string, name: "knots" | "open-bitcoin", finalTip: RegtestTip,
) {
  const rows: BasicRow[] = [];
  for (const hash of [corpus.genesisHash, ...corpus.blocks.map((block) => block.hash)]) {
    rows.push(basicRow(hash, await rpc.call("getblockfilter", [hash, "basic"])));
  }
  writeFileSync(join(run, `${name}-basic.json`), JSON.stringify(rows), { flag: "wx", mode: 0o600 });
  const basic = basicSummary(rows);
  if (name === "open-bitcoin") return { final_tip: finalTip, basic };

  return { final_tip: finalTip, basic, ...await observeKnotsCoins(rpc, run, finalTip, "cdif04-utxos.csv") };
}

export async function observeKnotsCoins(rpc: DaemonRpc, run: string, finalTip: RegtestTip, fileName: string) {
  const chain = await rpc.call("getblockchaininfo") as { chainwork?: unknown };
  const csv = join(rpc.datadir, fileName);
  const dumped = await rpc.call("dumptxoutset", [csv, "latest", { format: COIN_FIELDS, show_header: false }]) as {
    base_hash?: unknown; base_height?: unknown; coins_written?: unknown;
  };
  if (dumped.base_hash !== finalTip.hash || dumped.base_height !== finalTip.height || statSync(csv).size > 1_048_576) {
    throw new Error("Knots dump failed to bind its bounded UTXOs to the active tip");
  }
  const utxos = summarizeKnotsCoins(readFileSync(csv, "utf8"), finalTip.height);
  if (utxos.count !== dumped.coins_written) throw new Error("Knots dump count disagrees with its UTXO rows");
  copyFileSync(csv, join(run, "knots-utxos.csv"));
  return { chainwork: checkWork(chain.chainwork), utxos };
}

export async function observeClosedState(root: string, datadir: string, run: string, corpus: StateCorpus) {
  return observeClosedCoins(root, datadir, run, corpus.expected.tip, corpus.observerSha256, "cdif04");
}

export async function observeClosedCoins(
  root: string, datadir: string, run: string, expectedTip: RegtestTip, observerSha256: string,
  observer: "cdif04" | "crec02" | "mainnet_prefix",
) {
  const sources = { cdif04: CLOSED_STATE_OBSERVER, crec02: SPEND_STATE_OBSERVER, mainnet_prefix: MAINNET_STATE_OBSERVER };
  const source = sources[observer];
  const entry = observer === "mainnet_prefix" ? "observe_closed_mainnet_state" : "observe_closed_regtest_state";
  const flag = `OPEN_BITCOIN_${observer.toUpperCase()}`;
  const target = join(run, "open-bitcoin-closed-utxos.json");
  const log = join(run, "open-bitcoin-closed-state.log");
  const command = [process.execPath, "run", "scripts/command-timings.ts", "run", "--key", `${observer}-closed-state`, "--",
    "cargo", "test", "--locked", "--manifest-path", "packages/Cargo.toml", "-p", "open-bitcoin-rpc",
    "--test", `${observer}_closed_state`, entry, "--", "--ignored", "--exact"];
  await loggedCommand(root, command, log, {
    ...process.env, [`${flag}_DATADIR`]: datadir, [`${flag}_OUTPUT`]: target,
  }, OBSERVER_TIMEOUT_MS);
  if (sha256(readFileSync(join(root, source))) !== observerSha256 || statSync(target).size > 1_048_576) {
    throw new Error("Open Bitcoin observer source changed or report exceeded evidence bounds");
  }
  const observed = JSON.parse(readFileSync(target, "utf8")) as { tip: unknown; chainwork: unknown; utxos: unknown };
  if (!isDeepStrictEqual(observed.tip, expectedTip)) throw new Error("Open Bitcoin closed tip disagrees with live RPC");
  return { chainwork: checkWork(observed.chainwork), utxos: summarizeCoins(observed.utxos, expectedTip.height) };
}

export function verifyState(result: Record<string, unknown>, corpus: StateCorpus, name: string) {
  for (const [field, expected] of Object.entries(corpus.expected)) {
    const key = field === "tip" ? "final_tip" : field;
    if (!isDeepStrictEqual(result[key], expected)) throw new Error(`${name}: unexplained ${field} mismatch against canonical Regtest facts`);
  }
}
