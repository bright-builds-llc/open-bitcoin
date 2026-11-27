#!/usr/bin/env bun
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { BASELINE_COMMIT } from "./check-parity-ledger.ts";
import { currentQualificationInputs, QUALIFICATION_INPUT_FORMAT } from "./knots-regtest/qualification-inputs.ts";
import { MAINNET_STATE_OBSERVER } from "./knots-regtest/state.ts";
import { MAINNET_GENESIS, MAINNET_NEXT_BLOCK, MAINNET_PREFIX_CORPUS, MAINNET_PREFIX_RECEIPT, PREFIX_LIMIT_REASON, readMainnetPrefix } from "./mainnet-prefix.ts";
import { sha256 } from "./phase167-historical-corpus.ts";

const ROOT = resolve(import.meta.dir, "..");
const COMMIT = /^[a-f0-9]{40}$/;
const SHA256 = /^[a-f0-9]{64}$/;
const KNOTS_FLAGS = ["-chain=main", "-assumevalid=0", "-networkactive=0", "-listen=0", "-listenonion=0", "-discover=0", "-dnsseed=0", "-fixedseeds=0", "-natpmp=0"];
const LOCAL_FLAGS = ["-chain=main", "-server=1", "-rpcbind=127.0.0.1", "-blockfilterindex=0"];
const SURFACE = "mainnet-offline-prefix";

function ensure(condition: unknown, message: string): asserts condition {
  if (!condition) throw new Error(`MAINNET-PREFIX-02: ${message}`);
}

function coinSummary(value: { count: number; total_sats: string; sha256: string }) {
  return Number.isInteger(value.count) && value.count > 0 && value.count <= 4096
    && /^[1-9][0-9]*$/.test(value.total_sats) && SHA256.test(value.sha256);
}

export function checkMainnetPrefix(root: string) {
  const read = (file: string) => readFileSync(resolve(root, file), "utf8");
  const corpus = readMainnetPrefix(root);
  const row = JSON.parse(read(MAINNET_PREFIX_RECEIPT));
  const tree = execFileSync("git", ["-C", resolve(root, "packages/bitcoin-knots"), "rev-parse", "HEAD^{tree}"],
    { encoding: "utf8", timeout: 5000 }).trim();
  const index = JSON.parse(read("docs/parity/index.json"));
  const owners = index.checklist.surfaces.filter((owner: { id: string; requirements: string[] }) =>
    owner.id === SURFACE || owner.requirements.some(id => id === "MAINNET-PREFIX-01" || id === "MAINNET-PREFIX-02"));
  const owner = owners[0];
  ensure(owners.length === 1 && owner.id === SURFACE && owner.status === "done"
    && isDeepStrictEqual(owner.requirements, ["MAINNET-PREFIX-01", "MAINNET-PREFIX-02"])
    && owner.differential_receipt === MAINNET_PREFIX_RECEIPT
    && owner.evidence.includes(MAINNET_PREFIX_RECEIPT)
    && owner.evidence_classes.includes("knots_daemon_differential"),
  "expected one completed owner with actual daemon evidence");

  ensure(row.format === "open-bitcoin-mainnet-prefix-v2" && row.status === "passed"
    && row.network === "mainnet" && row.network_enabled === false
    && isDeepStrictEqual(row.corpus, MAINNET_PREFIX_CORPUS) && row.corpus_sha256 === corpus.corpusSha256
    && row.boundary_input === MAINNET_NEXT_BLOCK && row.boundary_sha256 === corpus.boundarySha256
    && isDeepStrictEqual(row.retarget, corpus.retarget),
  "receipt or corpus binding mismatch");
  ensure(row.source_commit === BASELINE_COMMIT && row.source_tree === tree && SHA256.test(row.binary_sha256)
    && SHA256.test(row.open_bitcoin_build?.binary_sha256)
    && row.open_bitcoin_build.cargo_options.includes("--release")
    && row.open_bitcoin_observer_sha256 === sha256(read(MAINNET_STATE_OBSERVER)),
  "oracle, product, or closed-state observer identity mismatch");
  const binding = row.qualification_inputs;
  ensure(binding?.format === QUALIFICATION_INPUT_FORMAT
    && COMMIT.test(binding.source_commit) && binding.source_commit === row.open_bitcoin_build.source_commit
    && binding.sha256 === currentQualificationInputs(root),
  "actual qualification is stale or unbound");

  for (const result of [row.knots, row.open_bitcoin]) {
    ensure(result.exit_status === 0 && result.peer_count === 0
      && isDeepStrictEqual(result.initial_tip, { height: 0, hash: MAINNET_GENESIS })
      && isDeepStrictEqual(result.final_tip, corpus.finalTip) && result.chainwork === corpus.chainwork
      && result.bits === corpus.retarget.bits && result.previous_chainwork === corpus.retarget.previous_chainwork
      && result.accepted_count === corpus.accepted.accepted_count
      && result.accepted_tips_sha256 === corpus.accepted.accepted_tips_sha256
      && coinSummary(result.utxos),
    "a daemon did not complete the pinned Mainnet prefix");
  }
  const reopened = row.open_bitcoin.reopened;
  ensure(KNOTS_FLAGS.every(flag => row.knots.flags.includes(flag))
    && isDeepStrictEqual(row.open_bitcoin.flags, LOCAL_FLAGS) && isDeepStrictEqual(reopened.flags, LOCAL_FLAGS)
    && row.knots.previous_bits === corpus.retarget.previous_bits
    && row.open_bitcoin.boundary_rejection === PREFIX_LIMIT_REASON && reopened.exit_status === 0
    && reopened.peer_count === 0 && isDeepStrictEqual(reopened.initial_tip, corpus.finalTip)
    && reopened.chainwork === corpus.chainwork && isDeepStrictEqual(row.open_bitcoin.utxos, row.knots.utxos),
  "peer isolation, height bound, or durable state comparison failed");
  return corpus.finalTip;
}

if (import.meta.main) {
  try {
    const tip = checkMainnetPrefix(ROOT);
    console.log(`MAINNET-PREFIX-02 verified actual pinned Knots and reopened Open Bitcoin at height ${tip.height}, ${tip.hash}`);
  } catch (error) {
    console.error(String(error));
    process.exitCode = 1;
  }
}
