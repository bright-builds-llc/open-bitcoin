#!/usr/bin/env bun
import { readFileSync, statSync } from "node:fs";
import { resolve } from "node:path";
import { checkProjection, OUTPUT as BIP30_FIXTURES } from "./phase167-bip30-fixtures.ts";
import { check, compactTarget, decodeBlock, decodeHeader, LIMITS, PIN, sha256 } from "./phase167-historical-corpus.ts";

export const RECEIPT = "docs/parity/fixtures/cint-04-mainnet-audit.json";
const ROOT = resolve(import.meta.dir, "..");
const DATA = "packages/testdata/phase167";
const RULES = "docs/parity/fixtures/phase167-rule-expectations.json";
const RECEIPT_LIMIT = 128 * 1024;

type BlockRow = { network: string; height: number; hash: string; purpose: string; sha256: string; raw: string };
type HeaderRow = { network: string; height: number; hash: string; raw: string };
type Coin = {
  txid: string; vout: number; value: string; script: string;
  coinbase: boolean; created_height: number; created_mtp: number;
};
type Envelope = {
  network: string; hash: string; parent: string; candidate_history: string[];
  parent_window: string[]; expiry_window: string[]; expiry_mtp: number; difficulty: string[];
  external_utxos: Coin[]; overwrite_original: string[]; expected_flags: string[];
  bip34: { kind: "absent-parent-below-height" } | { kind: "independent-seeded-anchor"; hash: string };
};
type Previous = { network: string; txid: string; block_hash: string; proof: string; mtp_window: string[] };
type Context = {
  envelopes: Envelope[]; headers: HeaderRow[]; transactions: Previous[];
  responses: { url: string; sha256: string }[];
};
type Manifest = {
  blocks_sha256: string; contexts_sha256: string; expectations_sha256: string;
  sources: { file: string; sha256: string }[];
};

function need<T>(maybeValue: T | undefined, name: string): T {
  if (maybeValue === undefined) throw new Error(`Missing verified ${name}`);
  return maybeValue;
}

function boundedFile(root: string, path: string, limit: number): Buffer {
  const file = resolve(root, path);
  if (statSync(file).size > limit) throw new Error(`Audit input exceeds byte limit: ${path}`);
  const bytes = readFileSync(file);
  if (bytes.length > limit) throw new Error(`Audit input exceeds byte limit: ${path}`);
  return bytes;
}

function checkedInput(root: string, path: string, digest: string): Buffer {
  const bytes = boundedFile(root, path, LIMITS.corpus);
  if (sha256(bytes) !== digest) throw new Error(`Audit input changed after validation: ${path}`);
  return bytes;
}

function summarizeWindow(hashes: string[], headers: Map<string, HeaderRow>) {
  const window = hashes.map(hash => need(headers.get(hash), "header"));
  const first = need(window[0], "window start");
  const last = need(window.at(-1), "window end");
  return {
    count: window.length,
    first: { height: first.height, hash: first.hash },
    last: { height: last.height, hash: last.hash },
    ordered_headers_sha256: sha256(JSON.stringify(window.map(row => [row.height, row.hash, row.raw]))),
  };
}

function summarizeCoins(coins: Coin[], previous: Map<string, Previous>, proofs: Map<string, string>) {
  const records = [...coins].sort((a, b) => a.txid === b.txid ? a.vout - b.vout : a.txid < b.txid ? -1 : 1);
  const commitments = records.map(coin => {
    const source = need(previous.get(coin.txid), "creating transaction");
    return [coin.txid, coin.vout, coin.value, coin.script, coin.coinbase, coin.created_height,
      coin.created_mtp, source.block_hash, source.mtp_window, need(proofs.get(source.proof), "creating inclusion proof")];
  });
  return {
    count: records.length,
    coinbase_inputs: records.filter(coin => coin.coinbase).length,
    outpoints_and_creation_proofs_sha256: sha256(JSON.stringify(commitments)),
  };
}

function pinnedRules(root: string, manifest: Manifest) {
  const source = (file: string) => checkedInput(root, `packages/bitcoin-knots/${file}`,
    need(manifest.sources.find(row => row.file === file), "Knots rule source").sha256).toString();
  const params = source("src/kernel/chainparams.cpp").split("class CMainParams :")[1]?.split("class CTestNetParams :")[0];
  if (params === undefined) throw new Error("Missing pinned Mainnet parameters");
  const height = (name: string) => Number(need(params.match(new RegExp(`^\\s*consensus\\.${name} = (\\d+);`, "m"))?.[1], name));
  return {
    activation_heights: {
      bip34: height("BIP34Height"), dersig: height("BIP66Height"), cltv: height("BIP65Height"),
      csv: height("CSVHeight"), segwit: height("SegwitHeight"),
    },
    bip34_hash: need(params.match(/^\s*consensus\.BIP34Hash = uint256\{"([0-9a-f]{64})"\};/m)?.[1], "BIP34 hash"),
    bip30_recheck_height: Number(need(source("src/validation.cpp").match(/static constexpr int BIP34_IMPLIES_BIP30_LIMIT = (\d+);/)?.[1], "BIP30 recheck height")),
  };
}

export function buildMainnetAudit(root = ROOT) {
  const manifestPath = `${DATA}/manifest.json`;
  const manifestBytes = boundedFile(root, manifestPath, RECEIPT_LIMIT);
  check(root);
  const bip30 = checkProjection(root);
  if (!boundedFile(root, manifestPath, RECEIPT_LIMIT).equals(manifestBytes)) {
    throw new Error("Historical manifest changed during validation");
  }
  const manifest: Manifest = JSON.parse(manifestBytes.toString());
  const rules = pinnedRules(root, manifest);
  const { blocks }: { blocks: BlockRow[] } = JSON.parse(checkedInput(root, `${DATA}/blocks.json`, manifest.blocks_sha256).toString());
  const contexts: Context = JSON.parse(checkedInput(root, `${DATA}/contexts.json`, manifest.contexts_sha256).toString());
  checkedInput(root, RULES, manifest.expectations_sha256);

  const mainnet = new Map(blocks.filter(row => row.network === "mainnet").map(row => [row.hash, row]));
  const envelopes = new Map(contexts.envelopes.filter(row => row.network === "mainnet").map(row => [row.hash, row]));
  const headers = new Map(contexts.headers.filter(row => row.network === "mainnet").map(row => [row.hash, row]));
  const previous = new Map(contexts.transactions.filter(row => row.network === "mainnet").map(row => [row.txid, row]));
  const proofs = new Map(contexts.responses.map(row => [row.url, row.sha256]));
  if (mainnet.size !== 6 || envelopes.size !== 6) throw new Error("Expected six verified Mainnet cases");

  const cases = [...mainnet.values()].sort((a, b) => a.height - b.height).map(row => {
    const context = need(envelopes.get(row.hash), "Mainnet case envelope");
    const body = decodeBlock(Buffer.from(row.raw, "hex"), row.hash);
    if (BigInt(`0x${row.hash}`) > compactTarget(body.bits)) throw new Error(`Candidate PoW mismatch at ${row.height}`);
    const matchedBip34 = context.bip34.kind === "independent-seeded-anchor";
    if (matchedBip34 && context.bip34.hash !== rules.bip34_hash) throw new Error("Mainnet BIP34 anchor differs from pinned rules");
    const parentTimes = context.parent_window.map(hash => {
      const header = need(headers.get(hash), "parent MTP header");
      return decodeHeader(Buffer.from(header.raw, "hex"), hash).time;
    }).sort((a, b) => a - b);
    return {
      height: row.height, hash: row.hash, purpose: row.purpose, raw_block_sha256: row.sha256,
      parent: context.parent, parent_median_time: parentTimes[Math.floor(parentTimes.length / 2)],
      ancestry: summarizeWindow(context.candidate_history, headers),
      expiry: { median_time: context.expiry_mtp, ...summarizeWindow(context.expiry_window, headers) },
      difficulty: { bits: body.bits.toString(16).padStart(8, "0"), ...summarizeWindow(context.difficulty, headers) },
      external_inputs: summarizeCoins(context.external_utxos, previous, proofs),
      activation: { bip34: context.bip34, script_flags: context.expected_flags },
      bip30_scan_expected: row.height >= rules.bip30_recheck_height || (!matchedBip34 && context.overwrite_original.length === 0),
      overwrite_originals: context.overwrite_original.map(hash => {
        const original = need(mainnet.get(hash), "BIP30 original");
        const coinbase = decodeBlock(Buffer.from(original.raw, "hex"), hash).transactions[0];
        return { height: original.height, hash, raw_block_sha256: original.sha256, coinbase_txid: coinbase.txid };
      }),
    };
  });

  return {
    schema: 1, source_pin: PIN, network: "mainnet", evidence_class: "seeded_historical",
    scope: "finite seeded candidate contexts and referenced outputs",
    rules,
    inputs: {
      [manifestPath]: sha256(manifestBytes), [`${DATA}/blocks.json`]: manifest.blocks_sha256,
      [`${DATA}/contexts.json`]: manifest.contexts_sha256, [RULES]: manifest.expectations_sha256,
      [BIP30_FIXTURES]: bip30.digest,
    },
    cases,
  };
}

export function checkMainnetReceipt(receipt: string, expected: ReturnType<typeof buildMainnetAudit>): void {
  if (receipt !== JSON.stringify(expected, null, 2) + "\n") {
    throw new Error("Mainnet historical audit differs from verified case prerequisites");
  }
}

if (import.meta.main) {
  try {
    const argument = Bun.argv[2];
    if (Bun.argv.length !== 3 || (argument !== "--check" && argument !== "--print")) {
      throw new Error("Usage: bun run scripts/check-cint-04-mainnet.ts --check|--print");
    }
    const audit = buildMainnetAudit();
    if (argument === "--print") console.log(JSON.stringify(audit, null, 2));
    else {
      checkMainnetReceipt(boundedFile(ROOT, RECEIPT, RECEIPT_LIMIT).toString(), audit);
      console.log(`CINT-04 checked ${audit.cases.length} finite seeded Mainnet contexts against pinned Knots ${audit.source_pin}`);
    }
  } catch (error) {
    console.error(String(error));
    process.exitCode = 1;
  }
}
