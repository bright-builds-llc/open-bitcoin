import { copyFileSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { sha256 } from "../phase167-historical-corpus.ts";
import type { DaemonRpc } from "./daemon.ts";
import { readRegtestHistory, type RegtestTip } from "./regtest-history.ts";

export const CLOSED_STATE_OBSERVER = "packages/open-bitcoin-rpc/tests/cdif04_closed_state.rs";
const STATE_FIXTURE = "docs/parity/fixtures/phase167-regtest-expectations.json";
const COIN_FIELDS = ["txid", "vout", "value", "coinbase", "height", "scriptPubKey"];
const HEX256 = /^[a-f0-9]{64}$/;
const MAX_ROWS = 4096;

type BasicRow = [hash: string, filter: string, header: string];
type Coin = { txid: string; vout: number; sats: string; script: string; coinbase: boolean; height: number };

function hexBytes(value: unknown): value is string {
  return typeof value === "string" && value.length <= 65_536
    && value.length % 2 === 0 && /^[a-f0-9]*$/.test(value);
}

function natural(value: unknown, max: number): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0 && value <= max;
}

function coinSummary(input: unknown, tipHeight: number) {
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
  return coinSummary(rows.map((line) => {
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

function basicRow(hash: string, value: unknown): BasicRow {
  const row = value as { filter?: unknown; header?: unknown } | null;
  if (!hexBytes(row?.filter) || row.filter.length === 0 || typeof row.header !== "string" || !HEX256.test(row.header)) {
    throw new Error(`Invalid BASIC response for ${hash}`);
  }
  return [hash, row.filter, row.header];
}

function checkWork(value: unknown): string {
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
      utxos: coinSummary((last.utxos as Coin[]).map((row) => ({
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
  const rows: BasicRow[] = [];
  for (const hash of [corpus.genesisHash, ...corpus.blocks.map((block) => block.hash)]) {
    rows.push(basicRow(hash, await rpc.call("getblockfilter", [hash, "basic"])));
  }
  writeFileSync(join(run, `${name}-basic.json`), JSON.stringify(rows), { flag: "wx", mode: 0o600 });
  const basic = basicSummary(rows);
  if (name === "open-bitcoin") return { final_tip: finalTip, basic };

  const chain = await rpc.call("getblockchaininfo") as { chainwork?: unknown };
  const csv = join(rpc.datadir, "cdif04-utxos.csv");
  const dumped = await rpc.call("dumptxoutset", [csv, "latest", { format: COIN_FIELDS, show_header: false }]) as {
    base_hash?: unknown; base_height?: unknown; coins_written?: unknown;
  };
  if (dumped.base_hash !== finalTip.hash || dumped.base_height !== finalTip.height || statSync(csv).size > 1_048_576) {
    throw new Error("Knots dump failed to bind its bounded UTXOs to the active tip");
  }
  const utxos = summarizeKnotsCoins(readFileSync(csv, "utf8"), finalTip.height);
  if (utxos.count !== dumped.coins_written) throw new Error("Knots dump count disagrees with its UTXO rows");
  copyFileSync(csv, join(run, "knots-utxos.csv"));
  return { final_tip: finalTip, basic, chainwork: checkWork(chain.chainwork), utxos };
}

export async function observeClosedState(root: string, datadir: string, run: string, corpus: StateCorpus) {
  const target = join(run, "open-bitcoin-closed-utxos.json");
  const log = join(run, "open-bitcoin-closed-state.log");
  const command = [process.execPath, "run", "scripts/command-timings.ts", "run", "--key", "cdif04-closed-state", "--",
    "cargo", "test", "--locked", "--manifest-path", "packages/Cargo.toml", "-p", "open-bitcoin-rpc",
    "--test", "cdif04_closed_state", "observe_closed_regtest_state", "--", "--ignored", "--exact"];
  const child = Bun.spawn(command, { cwd: root, stderr: "pipe", stdout: "pipe",
    env: { ...process.env, OPEN_BITCOIN_CDIF04_DATADIR: datadir, OPEN_BITCOIN_CDIF04_OUTPUT: target } });
  const [stdout, stderr, status] = await Promise.all([
    new Response(child.stdout).text(),
    new Response(child.stderr).text(),
    child.exited,
  ]);
  writeFileSync(log, stdout + stderr, { flag: "wx", mode: 0o600 });
  if (status !== 0) throw new Error(`Open Bitcoin closed-state observer failed; see ${log}`);
  if (sha256(readFileSync(join(root, CLOSED_STATE_OBSERVER))) !== corpus.observerSha256 || statSync(target).size > 1_048_576) {
    throw new Error("Open Bitcoin observer source changed or report exceeded evidence bounds");
  }
  const observed = JSON.parse(readFileSync(target, "utf8")) as { tip: unknown; chainwork: unknown; utxos: unknown };
  if (!isDeepStrictEqual(observed.tip, corpus.expected.tip)) throw new Error("Open Bitcoin closed tip disagrees with live RPC");
  return { chainwork: checkWork(observed.chainwork), utxos: coinSummary(observed.utxos, corpus.expected.tip.height) };
}

export function verifyState(result: Record<string, unknown>, corpus: StateCorpus, name: string) {
  for (const [field, expected] of Object.entries(corpus.expected)) {
    const key = field === "tip" ? "final_tip" : field;
    if (!isDeepStrictEqual(result[key], expected)) throw new Error(`${name}: unexplained ${field} mismatch against canonical Regtest facts`);
  }
}
