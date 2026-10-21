#!/usr/bin/env bun
import { closeSync, constants, fstatSync, lstatSync, openSync, readSync } from "node:fs";
import { resolve } from "node:path";
import { checkVectors } from "./phase166-genesis-vectors.ts";
import { compact, hasAssertions, hasOrderedCode, maybeFunction, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { regions } from "./check-phase164-canonical-identity/regions.ts";
import { CHECKPOINT, CLI, CONTRACTS, DEPTH_GUARDS, ENTRY_GUARDS, EXTRA_FILES, FACTORY, NODE, OPENER, REGISTRATIONS, RPC, TESTS, VECTOR } from "./check-phase166-canonical-bootstrap/contracts.ts";
export { DAEMON, OBSERVER } from "./check-phase166-canonical-bootstrap/contracts.ts";

export type Finding = { file: string; rule: string; detail: string };
export const INPUT_FILES = [...new Set([...CONTRACTS.map(row => row.file), ...TESTS.map(row => row[0]), ...REGISTRATIONS.map(row => row[0]), ...EXTRA_FILES, VECTOR])];
const MAX_BYTES = 512_000;

/** Read only this finite source set; symlinks, non-files and oversized inputs refuse. */
export function loadInputs(maybeRoot?: string): Map<string, string> {
  const root = maybeRoot ?? resolve(import.meta.dir, "..");
  return new Map(INPUT_FILES.map(file => [file, readBounded(resolve(root, file))]));
}

function readBounded(file: string): string {
  if (!lstatSync(file).isFile()) throw new Error("required regular source input");
  const descriptor = openSync(file, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    if (!fstatSync(descriptor).isFile()) throw new Error("required regular source input");
    const buffer = Buffer.alloc(MAX_BYTES + 1);
    let size = 0;
    while (size < buffer.length) {
      const count = readSync(descriptor, buffer, size, buffer.length - size, null);
      if (!count) break;
      size += count;
    }
    if (size > MAX_BYTES) throw new Error("source input exceeds fixed bound");
    return buffer.subarray(0, size).toString("utf8");
  } finally { closeSync(descriptor); }
}

type Fail = (file: string, rule: string, detail: string) => void;

/** Conservative current wiring guard plus independent vectors; never grants stored authority. */
export async function checkPhase166(inputs: ReadonlyMap<string, string>): Promise<readonly Finding[]> {
  const findings: Finding[] = [];
  const fail: Fail = (file, rule, detail) => findings.push({ file, rule, detail });
  for (const file of INPUT_FILES) {
    const maybeValue = inputs.get(file);
    if (!maybeValue || Buffer.byteLength(maybeValue) > MAX_BYTES) fail(file, "input", "required bounded source input missing or oversized");
  }
  if (findings.length) return findings;
  const text = (file: string) => inputs.get(file)!;
  for (const contract of CONTRACTS) {
    const source = text(contract.file);
    const maybeBody = maybeFunction(source, contract.symbol, true);
    const raw = maybeBody ? source.slice(maybeBody.start, maybeBody.end) : "";
    if (!maybeBody || /\bcfg\s*\(/.test(maybeBody.attributes) || hasUnconditionalReturn(maybeBody.body)
      || !hasOrderedCode(maybeBody.body, contract.anchors.map(rustCode))
      || contract.anchors.some(anchor => anchor.includes('"') && !literalAnchor(raw, anchor))
      || (contract.maybeTopLevel ?? []).some(anchor => !atDepthOnce(maybeBody.body, anchor, 0))
      || (contract.maybeForbidden ?? []).some(anchor => compact(maybeBody.body).includes(compact(anchor)))) {
      fail(contract.file, contract.rule, `${contract.symbol}: reachable ordered executable boundary required`);
    }
  }
  for (const [file, symbol, rule, guard, depth] of DEPTH_GUARDS) {
    const maybeBody = maybeFunction(text(file), symbol, true);
    const raw = maybeBody ? text(file).slice(maybeBody.start, maybeBody.end) : "";
    if (!maybeBody || !atDepthOnce(maybeBody.body, guard, depth)
      || (guard.includes('"') && !literalAnchor(raw, guard))) fail(file, rule, `${symbol}: complete guard predicate/body at required depth`);
  }
  for (const [file, symbol, rule, first] of ENTRY_GUARDS) {
    const body = maybeFunction(text(file), symbol, true)?.body ?? "";
    if (!compact(body).startsWith(compact(rustCode(first)))) fail(file, rule, `${symbol}: mandatory first proof/dispatch operation`);
  }
  for (const [file, anchor] of REGISTRATIONS) {
    if (!literalAnchor(text(file), anchor, true)) fail(file, "registration", "required executable module/worker wiring changed");
  }
  for (const [file, symbol] of TESTS) {
    const maybeTest = maybeFunction(text(file), symbol);
    const attributes = compact(maybeTest?.attributes ?? "");
    if (!maybeTest || !attributes.endsWith("#[test]") || /#\[(?:ignore|cfg\(any\(\)\))/.test(attributes)
      || !hasAssertions(text(file), maybeTest.body) || /#!\[/.test(rustCode(text(file)))) {
      fail(file, "behavior-registration", `${symbol}: enabled Rust behavioral assertions required; execution is separate`);
    }
  }
  checkAuthorityTypes(text, fail);
  checkOrderAndScope(text, fail);
  if (findings.length) return findings;
  try { await checkVectors(JSON.parse(text(VECTOR))); }
  catch { fail(VECTOR, "vectors", "independent pinned complete-genesis/proof/BASIC/evidence-class reproduction failed"); }
  return findings;
}

/** A literal must have executable surrounding tokens at the same offsets, never a comment decoy. */
function literalAnchor(source: string, anchor: string, rejectExtraCfg = false): boolean {
  const pattern = anchor.trim().split(/\s+/).map(part => part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("\\s*");
  const masked = rustCode(source);
  const expected = compact(rustCode(anchor));
  return expected.length > 0 && [...source.matchAll(new RegExp(pattern, "g"))].some(match => {
    const prefix = masked.slice(0, match.index).replace(/\bpub(?:\([^)]*\))?\s*$/, "");
    const attributes = prefix.match(/(?:#\[[^\]]*\]\s*)+$/)?.[0] ?? "";
    return compact(masked.slice(match.index, match.index + match[0].length)) === expected
      && (!rejectExtraCfg || !/\bcfg\s*\(/.test(attributes));
  });
}

function hasUnconditionalReturn(body: string): boolean {
  let depth = 0;
  for (let index = 0; index < body.length; index++) {
    const char = body[index];
    depth += Number(char === "{") - Number(char === "}");
    if (depth === 0 && /^return\b/.test(body.slice(index)) && !/\w/.test(body[index - 1] ?? "")) return true;
  }
  return false;
}

function atDepthOnce(body: string, anchor: string, expectedDepth: number): boolean {
  const code = compact(body);
  const selected = compact(rustCode(anchor));
  const index = code.indexOf(selected);
  if (index < 0 || code.indexOf(selected, index + selected.length) >= 0) return false;
  let depth = 0;
  for (const char of code.slice(0, index)) depth += Number(char === "{") - Number(char === "}");
  return depth === expectedDepth;
}

function checkAuthorityTypes(text: (file: string) => string, fail: Fail): void {
  const factory = compact(rustCode(text(FACTORY)));
  if (!factory.includes("opened:OpenedCanonicalGenesis,")) fail(FACTORY, "sealed-signature", "factory consumes the exclusive capability by value");
  for (const [file, name, fields] of [
    [OPENER, "OpenedCanonicalGenesis", ["store:FjallNodeStore", "recovered:RecoveredCanonicalGenesis"]],
    [CONSENSUS_PROOF, "CanonicalGenesisProof", ["spec:ChainSpec", "block:Block", "work:ProofWork"]],
  ] as const) {
    const maybeStruct = rustCode(text(file)).match(new RegExp(`\\bstruct\\s+${name}\\s*\\{([^}]*)\\}`));
    if (!maybeStruct || /\bpub\b/.test(maybeStruct[1]) || fields.some(field => !compact(maybeStruct[1]).includes(field))) {
      fail(file, "sealed-fields", "authority fields must remain private and complete");
    }
  }
  const lease = text(NODE + "storage/fjall_store/open_lease.rs");
  const maybeAcquire = regions(lease).filter(row => row.symbol === "acquire")[1];
  if (!maybeAcquire || !hasOrderedCode(maybeAcquire.code, ["normalize_source(source)?", "validate_lock_metadata(&opened)?;",
    "validate_lock_metadata(&selected)?;", "file.try_lock()", "normalize_source(&source)? != source", "_file: file"])
    || /\breturn\s+Ok\s*\(/.test(maybeAcquire.code)) {
    fail(NODE + "storage/fjall_store/open_lease.rs", "application-lock", "retained validated application lease required");
  }
}

const CONSENSUS_PROOF = "packages/open-bitcoin-consensus/src/chain_spec/bootstrap.rs";

function checkOrderAndScope(text: (file: string) => string, fail: Fail): void {
  // Only these fixed functions have earned early-success branches. Their complete
  // branch bodies are also bound above, so an added conditional success refuses.
  for (const [file, symbol, count, rule] of [
    [NODE + "storage/canonical_bootstrap/proof.rs", "validate_basic", 2, "stored-basic"],
    [NODE + "sync/canonical_genesis/basic.rs", "resolve_startup_mode", 1, "saved-basic-owner"],
    ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs", "open_runtime_store", 1, "daemon-sealed-opener"],
  ] as const) {
    const body = maybeFunction(text(file), symbol, true)?.body ?? "";
    if ([...body.matchAll(/\breturn\s+Ok\s*\(/g)].length !== count) fail(file, rule, "only the fixed guarded early-success branches are allowed");
  }
  for (const [file, call] of [[RPC + "bin/open-bitcoind.rs", "FjallNodeStore::maybe_run_inspection_worker()"],
    [CLI + "bin/open-bitcoin.rs", "FjallNodeStore::maybe_run_inspection_worker()"]]) {
    const maybeMain = maybeFunction(text(file), "main", true);
    if (!maybeMain || compact(maybeMain.attributes).includes("#[tokio::main]")
      || compact(maybeMain.body).split(call).length !== 2) {
      fail(file, file.includes("open-bitcoind") ? "pre-tokio-dispatch" : "pre-parser-dispatch", "exact pre-runtime worker dispatch required");
    }
  }
  const maybeApplication = maybeFunction(text(RPC + "bin/open-bitcoind.rs"), "run_application", true);
  if (!maybeApplication || !compact(maybeApplication.attributes).includes("#[tokio::main]")) {
    fail(RPC + "bin/open-bitcoind.rs", "daemon-wiring", "Tokio belongs to the delegated application entry");
  }
  for (const [file, symbol, first, second, rule] of [
    [CHECKPOINT, "save_validated_chain_meta", "decode_chain_meta(&bytes)", "control.invalidate_append()?;", "checkpoint-counts"],
    [NODE + "chainstate/fjall_store/canonical_genesis.rs", "from_canonical_genesis", "recovered.check(&store)", "initialize_configured(", "recovery-after-proof"],
  ]) {
    const body = compact(maybeFunction(text(file), symbol, true)?.body ?? "");
    if (body.indexOf(compact(first)) < 0 || body.indexOf(compact(second)) <= body.indexOf(compact(first))
      || body.split(compact(second)).length !== 2) fail(file, rule, "proof/decode must precede the single initialization/invalidation call");
  }
  const walletFile = RPC + "context/wallet_state.rs";
  const wallet = rustCode(text(walletFile));
  const unavailable = "WalletState::Unavailable=>Err(unavailable_wallet()),";
  if (compact(wallet).split(unavailable).length !== 15
    || !hasOrderedCode(maybeFunction(text(walletFile), "selected_wallet_from_store", true)?.body ?? "", [
      "if matches!(self.wallet_state, WalletState::Unavailable)", "return Err(unavailable_wallet());", "load_wallet_registry(store)?",
    ])) fail(walletFile, "wallet-unavailable", "all unavailable-wallet refusals must precede registry recovery");
  for (const [file, call] of [[CLI + "operator/wallet.rs", "FjallNodeStore::open_ordinary(data_dir, selected_chain)"],
    [CLI + "operator/wallet/name_resolution.rs", "FjallNodeStore::open_ordinary(data_dir, selected_chain)"],
    [CLI + "operator/soak/runtime/chain_compatibility.rs", "FjallNodeStore::open_ordinary(datadir, chain)"]]) {
    if (!compact(rustCode(text(file))).includes(compact(call))) fail(file, "ordinary-scope", "wallet/soak ordinary authority gate required");
  }
}

if (import.meta.main) {
  try {
    const findings = await checkPhase166(loadInputs());
    if (findings.length) {
      console.error("Phase166 canonical bootstrap boundary failed:\n" + findings.map(row => `${row.file}: ${row.rule}: ${row.detail}`).join("\n"));
      process.exit(1);
    }
    console.log(`Phase166: ${INPUT_FILES.length} bounded current inputs; semantic wiring and independently reproduced genesis vectors. Rust behavior remains separate; no ordinary history or daemon-differential qualification.`);
  } catch {
    console.error("Phase166 required bounded input unavailable; no acceptance or receipt generated.");
    process.exit(1);
  }
}
