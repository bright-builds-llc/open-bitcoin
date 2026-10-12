import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { CONTRACTS, type Contract } from "./check-phase162-integrated-filter-proof/contracts.ts";
import { checkPhase162IntegratedFilterProof, loadSources, assertionSpans, CHECK_FILES, MODULE_EDGES,
  NATIVE_COMMANDS, SHARED_FORMAT, factSpans, processOwnershipView, defaultRustTestAttributes,
  typeScriptFunction, typeScriptCallSpans, maskTypeScript } from "./check-phase162-integrated-filter-proof.ts";
import { CHECKPOINT_COUNTS, TS_CONTRACTS } from "./check-phase162-integrated-filter-proof/contracts.ts";
import { compact, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { maybeSelector } from "./check-phase160-peer-filter-serving.ts";
import { CHECKPOINT_CONTRACTS, PHASE162, PHASE162_DOC, PHASE162_EVIDENCE,
  PHASE162_IMPLEMENTATION, PHASE162_COMPLETE } from "./check-phase158-validated-reorg.ts";

const real = loadSources();
const sha = (source: string) => createHash("sha256").update(source).digest("hex");
const reports = [["VERIFICATION", "passed", "gsd-verifier"], ["NATIVE-EVIDENCE", "passed", "gsd-execute-phase"],
  ["REVIEW", "clean", "gsd-code-reviewer"], ["SECURITY", "verified", "gsd-security-auditor"]];

function pending(): Map<string, string> {
  const sources = new Map(real);
  sources.delete(PHASE162_DOC);
  for (const [name] of reports) sources.delete(PHASE162 + `162-${name}.md`);
  sources.set(".planning/REQUIREMENTS.md", sources.get(".planning/REQUIREMENTS.md")!
    .replace(/^- \[[ xX]\] \*\*(CFPR-02|CFGR-01|CFGR-02)\*\*:/gm, "- [ ] **$1**:")
    .replace(/(\|\s*(?:CFPR-02|CFGR-01|CFGR-02)\s*\|\s*Phase 162\s*\|)\s*(?:Pending|Complete)\s*\|/g, "$1 Pending |"));
  inventory(sources, owner => { owner.status = "in_progress"; owner.known_gaps = ["Phase162 scoped proof remains pending; v2.5 remains unaudited and unshipped."]; });
  return sources;
}
type Owner = { id: string; status: string; requirements: string[]; evidence: string[]; known_gaps: string[]; intentional_differences: string[] };
function inventory(sources: Map<string, string>, change: (owner: Owner, index: { baseline: string; checklist: { surfaces: Owner[] } }) => void): void {
  const index = JSON.parse(sources.get("docs/parity/index.json")!);
  change(index.checklist.surfaces.find((owner: Owner) => owner.id === "v2-5-integrated-retained-filter-proof"), index);
  sources.set("docs/parity/index.json", JSON.stringify(index));
}
function implementation(complete = false): Map<string, string> {
  const sources = pending();
  const context = sources.get(PHASE162 + "162-CONTEXT.md")!;
  const lifecycle = context.match(/^phase_lifecycle_id: (.+)$/m)![1];
  const timestamp = new Date().toISOString().replace(/\.\d{3}Z$/, "Z");
  const table = PHASE162_EVIDENCE.map(file => `| \`${file}\` | \`${sha(sources.get(file)!)}\` |`).join("\n");
  const sentence = complete ? PHASE162_COMPLETE : PHASE162_IMPLEMENTATION;
  sources.set(PHASE162_DOC, `---\nphase_lifecycle_id: ${lifecycle}\nlifecycle_mode: yolo\nimplementation_status: proven\ngenerated_at: ${timestamp}\n---\n${sentence}\n${table}\n`);
  inventory(sources, owner => {
    owner.status = complete ? "done" : "in_progress";
    owner.evidence.push(PHASE162_DOC);
    owner.known_gaps = [sentence];
  });
  if (!complete) return sources;
  sources.set(".planning/REQUIREMENTS.md", sources.get(".planning/REQUIREMENTS.md")!
    .replace(/^- \[ \] \*\*(CFPR-02|CFGR-01|CFGR-02)\*\*:/gm, "- [x] **$1**:")
    .replace(/(\|\s*(?:CFPR-02|CFGR-01|CFGR-02)\s*\|\s*Phase 162\s*\|)\s*Pending\s*\|/g, "$1 Complete |"));
  const digest = sha(PHASE162_EVIDENCE.map(file => `${file}:${sha(sources.get(file)!)}`).join("\n"));
  for (const [name, status, generator] of reports) {
    const file = PHASE162 + `162-${name}.md`;
    sources.set(file, `---\nstatus: ${status}\ngenerated_by: ${generator}\nphase_lifecycle_id: ${lifecycle}\nlifecycle_mode: yolo\ngenerated_at: ${timestamp}\nsource_digest: ${digest}\nlifecycle_validated: true\nthreats_open: 0\n---\n\`\`\`bash\nbash scripts/verify.sh\n\`\`\`\n`);
    inventory(sources, owner => { owner.evidence.push(file); });
  }
  return sources;
}

function rebindProof(sources: Map<string, string>): void {
  let proof = sources.get(PHASE162_DOC)!;
  for (const file of PHASE162_EVIDENCE) {
    const lines = proof.split("\n");
    const index = lines.findIndex(line => line.includes(`\`${file}\``));
    expect(index).toBeGreaterThanOrEqual(0);
    lines[index] = `| \`${file}\` | \`${sha(sources.get(file)!)}\` |`;
    proof = lines.join("\n");
  }
  sources.set(PHASE162_DOC, proof);
  const digest = sha(PHASE162_EVIDENCE.map(file => `${file}:${sha(sources.get(file)!)}`).join("\n"));
  for (const [name] of reports) {
    const file = PHASE162 + `162-${name}.md`;
    if (sources.has(file)) sources.set(file, sources.get(file)!.replace(/^source_digest: .+$/m, `source_digest: ${digest}`));
  }
}

function mutate(sources: Map<string, string>, file: string, edit: (source: string) => string): void {
  const before = sources.get(file)!;
  const after = edit(before);
  expect(after).not.toBe(before);
  sources.set(file, after);
}

const PROCESS = "packages/testdata/phase162/process.rs";
const UNIX = "#[cfg(all(test, unix))]";
test("original Unix view preserves length newlines functions and raw assertions", () => {
  // Arrange
  const source = real.get(PROCESS)!;
  const originalHash = sha(source);
  // Act
  const view = processOwnershipView(source);
  // Assert
  expect(view.length).toBe(source.length);
  expect([...view.matchAll(/\n/g)].map(match => match.index)).toEqual([...source.matchAll(/\n/g)].map(match => match.index));
  expect(sha(source)).toBe(originalHash);
  for (const row of CONTRACTS.filter(row => row.file === PROCESS)) {
    expect(maybeSelector(view, row.symbol)).toEqual(maybeSelector(source, row.symbol));
    expect(assertionSpans(view, row.symbol)).toEqual(assertionSpans(source, row.symbol));
  }
  expect(CONTRACTS.filter(row => row.file === PROCESS && row.kind === "test")).toHaveLength(9);
  expect(defaultRustTestAttributes(maybeSelector(view, "phase162_process_child")!.attributes)).toBe(false);
});
for (const change of ["remove", "different", "duplicate", "variant", "nest", "comment", "literal", "child"]) test(`Unix registration/ownership refuses ${change}`, () => {
  const sources = pending();
  mutate(sources, PROCESS, source => {
    if (change === "remove") return source.replace(UNIX, "");
    if (change === "different") return source.replace(UNIX, "#[cfg(test)]");
    if (change === "duplicate") return source + `\n${UNIX}\nmod tests {}\n`;
    if (change === "variant") return source.replace(UNIX, "#[cfg(all(test, unix, feature = \"invented\"))]");
    if (change === "nest") return source.replace(UNIX, `mod wrong_owner {\n${UNIX}`) + "\n}\n";
    if (change === "comment") return source.replace(UNIX, `/* ${UNIX} */`);
    if (change === "literal") return source.replace(UNIX, `const FAKE: &str = r#\"${UNIX}\"#;`);
    return source.replace('#[ignore = "invoked only as a bounded subprocess control"]', "");
  });
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(PROCESS) && /ownership|registration/.test(failure))).toBe(true);
});
test("the exact Unix registration cannot normalize another source file", () => {
  const sources = pending();
  const file = CONTRACTS[0].file;
  mutate(sources, file, source => UNIX + "\n" + source);
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(file) && failure.includes("Unix ownership/registration"))).toBe(true);
});
test("Unix helper no-op and fake assertion cannot satisfy original process behavior", () => {
  const sources = pending();
  const body = maybeSelector(sources.get(PROCESS)!, "refused")!;
  const open = rustCode(sources.get(PROCESS)!).indexOf("{", body.start);
  mutate(sources, PROCESS, source => source.slice(0, open + 1) + " assert!(true); " + source.slice(body.end - 1));
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(`${PROCESS}:refused:`))).toBe(true);
});

const ORACLE = "scripts/phase162-filter-oracle.ts";
test("TS extraction preserves original offsets across braces escaped quotes and nested templates", () => {
  const source = 'const other = `brace } escaped \\` ${ {value: `nested ${"}" + `{${1}}`}`} }`;\n'
    + 'const quoted = "escaped \\\" { }";\nfunction owned(value: unknown): unknown { const text = `outer ${`inner ${ {x: "}"}.x }`}`; return value; }';
  const fn = typeScriptFunction(source, "owned");
  expect(source.slice(fn.start, fn.open)).toBe("function owned(value: unknown): unknown ");
  expect(source.slice(fn.end - 1, fn.end)).toBe("}");
  expect(compact(fn.body)).toBe("consttext=;returnvalue;");
  expect(maskTypeScript(source).length).toBe(source.length);
});
for (const source of [
  "// function owned() {}\nconst other = 1;", 'const text = "function owned() {}";',
  "const text = `function owned() {}`;", "function outer() { function owned() {} }",
  "function owned() {}\nfunction owned() {}", "function owned() {", "function owned() { const s = 'unterminated; }",
  "function owned() { const s = `unterminated ${1}; }", "const expression = function owned() {};",
]) test(`TS extraction refuses fake owner/syntax ${source}`, () => {
  expect(() => typeScriptFunction(source, "owned")).toThrow();
});

for (const row of TS_CONTRACTS) {
  for (const [index, anchor] of row.anchors.entries()) for (const state of ["pending", "implementation", "complete"]) test(`TS intended ${row.symbol} edge${index} ${state}`, () => {
    // Arrange
    const sources = state === "pending" ? pending() : implementation(state === "complete");
    mutate(sources, ORACLE, source => {
      const fn = typeScriptFunction(source, row.symbol);
      const offsets = [...fn.body].flatMap((character, index) => /\s/.test(character) ? [] : [index]);
      const start = compact(fn.body).indexOf(compact(anchor));
      expect(start).toBeGreaterThanOrEqual(0);
      const keyword = /^(?:const|let|if|for|of|while|return|throw|new|as|in)$/;
      const bodyStart = offsets[start];
      const originalCode = fn.body.slice(bodyStart, offsets[start + compact(anchor).length - 1] + 1);
      const token = [...originalCode.matchAll(/[A-Za-z_]\w*/g)].find(match => !keyword.test(match[0]))!;
      const begin = fn.open + 1 + bodyStart + token.index;
      const end = begin + token[0].length;
      return source.slice(0, begin) + "altered_raw_contract" + source.slice(end);
    });
    expect(() => new Bun.Transpiler({ loader: "ts" }).transformSync(sources.get(ORACLE)!)).not.toThrow();
    if (state !== "pending") rebindProof(sources);
    // Act / Assert
    const failures = checkPhase162IntegratedFilterProof(sources);
    expect(failures.some(failure => failure.includes(`${ORACLE}:${row.symbol}: actual raw-oracle caller/argument`))).toBe(true);
    expect(failures.some(failure => /source hash|source_digest/.test(failure))).toBe(false);
  });
  for (const [index, call] of row.calls.entries()) for (const state of ["pending", "implementation", "complete"]) test(`TS intended ${row.symbol} argument${index} ${state}`, () => {
    const sources = state === "pending" ? pending() : implementation(state === "complete");
    mutate(sources, ORACLE, source => {
      const span = typeScriptCallSpans(source, row.symbol)[index];
      expect(compact(span.raw)).toBe(call);
      return source.slice(0, span.start) + "altered_raw_caller(null)" + source.slice(span.end);
    });
    expect(() => new Bun.Transpiler({ loader: "ts" }).transformSync(sources.get(ORACLE)!)).not.toThrow();
    if (state !== "pending") rebindProof(sources);
    const failures = checkPhase162IntegratedFilterProof(sources);
    expect(failures.some(failure => failure.includes(`${ORACLE}:${row.symbol}: actual raw-oracle caller/argument`))).toBe(true);
    expect(failures.some(failure => /source hash|source_digest/.test(failure))).toBe(false);
  });
}

const TS_MUTATIONS = [
  ["validateRecipe", 'keys(recipe, ["recipe_id", "network_magic", "blocks"])', 'keys(recipe, ["recipe_id", "network_magic", "blocks", "filter", "header"])'],
  ["validateRecipe", 'keys(row, ["name", "previous_name", "raw_block"])', 'keys(row, ["name", "previous_name", "raw_block", "spent", "hash"])'],
  ["validateRecipe", "> LIMITS.input", "> Number.MAX_SAFE_INTEGER"],
  ["validateRecipe", "recipe.recipe_id !== RECIPE_ID", "false"],
  ["validateRecipe", "hex(recipe.network_magic, 4)", "hex(recipe.network_magic, 1)"],
  ["validateRecipe", "recipe.blocks.length > 1108", "recipe.blocks.length > 1109"],
  ["validateRecipe", "row.raw_block.length < 162", "row.raw_block.length < 0"],
  ["validateRecipe", "heights.has(row.name)", "false"],
  ["validateRecipe", "!heights.has(row.previous_name)", "false"],
  ["validateRecipe", "active > 1100", "active > 1101"],
  ["validateRecipe", "branches > 8", "branches > 9"],
  ["rawExpectations", "recipe.blocks.slice(start, start + LIMITS.cases)", "[]"],
  ["rawExpectations", "JSON.stringify({ cases: batch, nodes })", "JSON.stringify({ cases: batch, nodes: recipe })"],
  ["rawExpectations", "Buffer.byteLength(input) > LIMITS.input", "false"],
  ["rawExpectations", '"python3"', '"rust-result-provider"'],
  ["rawExpectations", '"--phase162-recipe"', '"--peer-wire-proof"'],
  ["rawExpectations", "root, input", "ROOT, JSON.stringify(recipe)"],
  ["rawExpectations", "result.cases.length !== batch.length", "false"],
  ["rawExpectations", "keys(returned, names)", "void(returned)"],
  ["rawExpectations", "validateNode(node)", "void(node)"],
  ["rawExpectations", "JSON.stringify(nodes[name]) !== JSON.stringify(node)", "false"],
  ["rawExpectations", "nodes = returned as Record<string, OracleNode>", "nodes = {} as Record<string, OracleNode>"],
  ["rawExpectations", "node.parent !== raw.previous_name", "false"],
  ["rawExpectations", "nodes[raw.previous_name].height + 1", "0"],
  ["rawExpectations", "validateCase(row, raw, node, nodes)", "row as OracleCase"],
  ["wireExpectations", "block_hash_raw: row.block_hash_raw", "block_hash_raw: (recipe as any).hash"],
  ["wireExpectations", "filter_data: row.encoded", "filter_data: (recipe as any).filter"],
  ["wireExpectations", "row.previous_name", "row.name"],
  ["wireExpectations", "range[0].predecessor_raw", "(recipe as any).header"],
  ["wireExpectations", "range.map(row => row.hash_raw)", "(recipe as any).hashes"],
  ["wireExpectations", "height > 0 && height % 1000 === 0", "height >= 0 && height % 999 === 0"],
  ["wireExpectations", "start += LIMITS.cases", "start += 1000000"],
  ["wireExpectations", '"--peer-wire-proof"', '"--phase162-recipe"'],
  ["wireExpectations", "root, JSON.stringify(corpus)", "ROOT, JSON.stringify(recipe)"],
  ["wireExpectations", "validateResult(output, corpus).wire", "JSON.stringify({cases:[]})"],
  ["wireExpectations", "if (!row.expected)", "if (false)"],
  ["wireExpectations", "row.expected.frame_hex.slice(8)", "row.expected.frame_hex.slice(24)"],
  ["deriveRecipe", "runner: Runner = runBounded", "runner: Runner = alternateRunner"],
  ["deriveRecipe", "validateRecipe(value)", "void(value)"],
  ["deriveRecipe", "if (pin !== PIN)", "if (false)"],
  ["deriveRecipe", '"--porcelain"', '"--short"'],
  ["deriveRecipe", "rawExpectations(value, root, runner)", "(value as any).cases"],
  ["deriveRecipe", "wireExpectations(value, cases, root, runner)", "(value as any).wire"],
  ["deriveRecipe", "raw_block: row.raw_block", "filter: (row as any).filter"],
  ["deriveRecipe", "Buffer.byteLength(JSON.stringify(result)) > LIMITS.stdout", "false"],
];
for (const [symbol, before, after] of TS_MUTATIONS) for (const state of ["pending", "implementation", "complete"]) test(`TS semantic ${symbol} ${before} ${state}`, () => {
  // Arrange
  const sources = state === "pending" ? pending() : implementation(state === "complete");
  mutate(sources, ORACLE, source => {
    const fn = typeScriptFunction(source, symbol);
    const owned = source.slice(fn.start, fn.end);
    expect(owned).toContain(before);
    return source.slice(0, fn.start) + owned.replace(before, after) + source.slice(fn.end);
  });
  expect(() => new Bun.Transpiler({ loader: "ts" }).transformSync(sources.get(ORACLE)!)).not.toThrow();
  if (state !== "pending") rebindProof(sources);
  // Act / Assert
  const failures = checkPhase162IntegratedFilterProof(sources);
  expect(failures.some(failure => failure.includes(`${ORACLE}:${symbol}: actual raw-oracle caller/argument`))).toBe(true);
  expect(failures.some(failure => /source hash|source_digest/.test(failure))).toBe(false);
});

const RUST_MUTATIONS: [string, string, string, string][] = [
  ["seed_count", "fixtures.rs", "let params = runtime.config().network.consensus_params();", "let mut params = runtime.config().network.consensus_params(); params.coinbase_maturity = 1;"],
  ["seed_count", "fixtures.rs", "FjallNodeStore::open", "MemoryChainstateStore::open"],
  ["denied_disconnect", "inbound.rs", "Ok(0) => break", "Ok(_) => break"],
  ["denied_disconnect", "inbound.rs", 'other => panic!("absent capability requires zero bytes then disconnect: {other:?}")', "_ => break"],
  ["denied_disconnect", "inbound.rs", '.expect("denial must physically disconnect before deadline")', ".ok()"],
  ["physical_session", "inbound.rs", "denied_disconnect(&stream).await;", "sentinel(&stream, magic, 162_999).await;"],
  ["physical_session", "inbound.rs", "break;", "continue;"],
  ["assert_phase162_inbound", "inbound.rs", "for exchange in exchanges.clone()", "for exchange in exchanges.clone().into_iter().take(1)"],
  ["phase162_inbound_no_grant_all_families", "inbound.rs", "WireNetworkMessage::GetCFCheckpt", "WireNetworkMessage::GetCFHeaders"],
  ["persist", "chainstate.rs", "self.flush_with_mode(", "self.save_snapshot(self.chainstate.snapshot())?; self.flush_with_mode("],
  ["persist", "chainstate.rs", "FlushMode::IfNeeded", "FlushMode::Always"],
  ["execute_flush_applying_plan_with_header_checkpoint", "flush_lifecycle.rs", "checkpoint(sink)?;", "let _ = checkpoint(sink);"],
  ["execute_flush_applying_plan_with_header_checkpoint", "flush_lifecycle.rs", "Result<(), StorageError>", "Result<(), String>"],
  ["save_trusted_header_snapshot", "validation_history.rs", "let headers =", "self.publish_validation_history()?; let headers ="],
  ["send_response", "filter_serving.rs", "session.send_encoded(frame.bytes(), magic)?;", "let receipt_before_write = frame.acknowledge_write();"],
  ["SocketFilterWriter::write", "filter_serving.rs", "self.queue.clear_pending_write();", "self.queue = &mut RuntimeQueuePressureState::default();"],
];
for (const [symbol, suffix, before, after] of RUST_MUTATIONS) test(`semantic Rust ${symbol}: ${before}`, () => {
  const sources = pending();
  const row = rows.find(row => row.symbol === symbol && row.file.endsWith(suffix))!;
  expect(row).toBeDefined();
  mutate(sources, row.file, source => {
    const fn = maybeSelector(source, symbol, row.kind === "production")!;
    const owned = source.slice(fn.start, fn.end);
    expect(owned).toContain(before);
    return source.slice(0, fn.start) + owned.replace(before, after) + source.slice(fn.end);
  });
  rejectsFunction(sources, row);
});
function anchorSpan(source: string, row: Contract, anchor: string): [number, number] {
  const body = maybeSelector(source, row.symbol, row.kind === "production")!;
  const offsets = [...body.body].flatMap((character, index) => /\s/.test(character) ? [] : [index]);
  const start = compact(body.body).indexOf(compact(anchor));
  expect(start).toBeGreaterThanOrEqual(0);
  const open = rustCode(source).indexOf("{", body.start) + 1;
  return [open + offsets[start], open + offsets[start + compact(anchor).length - 1] + 1];
}
function rejectsFunction(sources: Map<string, string>, row: Contract): void {
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(`${row.file}:${row.symbol}:`))).toBe(true);
}

test("genuine Pending bootstrap uses current executable roots without future proof", () => {
  const sources = pending();
  expect(sources.has(PHASE162_DOC)).toBe(false);
  expect(checkPhase162IntegratedFilterProof(sources)).toEqual([]);
});

test("actual checkout claim state and fixed current roots pass", () => {
  expect(checkPhase162IntegratedFilterProof(real)).toEqual([]);
});
test("hermetic implementation proof passes without future reports", () => {
  expect(checkPhase162IntegratedFilterProof(implementation())).toEqual([]);
});
test("hermetic formal proof requires all fresh source-bound reports", () => {
  expect(checkPhase162IntegratedFilterProof(implementation(true))).toEqual([]);
});

const rows: Contract[] = [...CONTRACTS, ...CHECKPOINT_CONTRACTS.map((row, index) => ({ ...row, kind: "production" as const, counts: CHECKPOINT_COUNTS[index], assertions: [], facts: [] }))];
for (const row of rows) {
  test(`executable selector remains ${row.file}:${row.symbol}`, () => {
    const sources = pending();
    mutate(sources, row.file, source => {
      const start = maybeSelector(source, row.symbol, row.kind === "production")!.start;
      return source.slice(0, start) + source.slice(start).replace(new RegExp(`\\bfn\\s+${row.symbol.split("::").at(-1)}\\b`), "fn removed_selector");
    });
    rejectsFunction(sources, row);
  });
  for (const [index, anchor] of row.anchors.entries()) test(`independent caller ${row.symbol} edge${index}: ${anchor}`, () => {
    const sources = pending();
    mutate(sources, row.file, source => {
      const [start, end] = anchorSpan(source, row, anchor);
      return source.slice(0, start) + "/* removed caller */" + source.slice(end);
    });
    rejectsFunction(sources, row);
  });
  for (const [index, assertion] of row.assertions.entries()) test(`independent assertion ${row.symbol} #${index}`, () => {
    const sources = pending();
    mutate(sources, row.file, source => {
      const span = assertionSpans(source, row.symbol, row.kind === "production")[index];
      expect(compact(span.raw)).toBe(compact(assertion));
      return source.slice(0, span.start) + "assert!(true)" + source.slice(span.end);
    });
    rejectsFunction(sources, row);
  });
  for (const [index, fact] of row.facts.entries()) test(`independent protocol matrix ${row.symbol} #${index}`, () => {
    const sources = pending();
    mutate(sources, row.file, source => {
      const span = factSpans(source, row.symbol, row.kind === "production")[index];
      expect(compact(span.raw)).toBe(compact(fact));
      return source.slice(0, span.start) + "/* removed protocol matrix */" + source.slice(span.end);
    });
    rejectsFunction(sources, row);
  });
  if (row.kind === "test") for (const attribute of ["#[ignore]", '#[ignore = "fake evidence"]', "#[cfg(any())]"]) test(`default test ownership ${row.symbol}: ${attribute}`, () => {
    const sources = pending();
    mutate(sources, row.file, source => {
      const start = maybeSelector(source, row.symbol)!.start;
      return source.slice(0, start) + attribute + "\n" + source.slice(start);
    });
    rejectsFunction(sources, row);
  });
}

for (const symbol of ["phase162_integrated_lifecycle", "denied_disconnect", "run", "partial_case", "socket_case", "send_response"]) {
  const row = rows.find(row => row.symbol === symbol)!;
  for (const disguise of ["return;", "#[cfg(any())] {", "if false {"]) test(`dead executable evidence ${symbol}: ${disguise}`, () => {
    const sources = pending();
    mutate(sources, row.file, source => {
      const fn = maybeSelector(source, symbol, row.kind === "production")!;
      const open = rustCode(source).indexOf("{", fn.start) + 1;
      return source.slice(0, open) + disguise + source.slice(open, fn.end - 1) + (disguise.endsWith("{") ? "}" : "") + source.slice(fn.end - 1);
    });
    rejectsFunction(sources, row);
  });
}
test("outbound physical prefix constant cannot move from31bytes", () => {
  const sources = pending();
  const file = "packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs";
  mutate(sources, file, source => source.replace("const PARTIAL: usize = 31;", "const PARTIAL: usize = 32;"));
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("exact31-byte constant"))).toBe(true);
});
for (const row of CONTRACTS.filter(row => /(?:peer_filter_transport|peer_filters)\/phase162\.rs$/.test(row.file) && ["run", "partial_case"].includes(row.symbol))) test(`manual CF write receipt refuses ${row.file}:${row.symbol}`, () => {
  const sources = pending();
  mutate(sources, row.file, source => {
    const fn = maybeSelector(source, row.symbol)!;
    const open = rustCode(source).indexOf("{", fn.start) + 1;
    return source.slice(0, open) + "frame.acknowledge_write();" + source.slice(open);
  });
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(`${row.file}:${row.symbol}: production executor alone`))).toBe(true);
});
for (const file of CHECK_FILES) test(`hermetic mandatory current root ${file}`, () => {
  const sources = pending();
  sources.delete(file);
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(file))).toBe(true);
});
for (const edge of MODULE_EDGES) test(`owning registration ${edge.parent}:${edge.symbol}`, () => {
  const sources = pending();
  mutate(sources, edge.parent, source => source.replace(new RegExp(`\\bmod\\s+${edge.symbol}\\s*;`), "/* removed owning declaration */"));
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(edge.parent) && failure.includes("registration"))).toBe(true);
});

for (const state of [false, true]) {
  for (const field of ["phase_lifecycle_id", "lifecycle_mode", "implementation_status", "generated_at"]) test(`published proof field ${field} complete=${state}`, () => {
    const sources = implementation(state);
    mutate(sources, PHASE162_DOC, source => source.replace(new RegExp(`^${field}: .+$`, "m"), `${field}: invalid`));
    expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("T-162-10"))).toBe(true);
  });
  for (const file of PHASE162_EVIDENCE) test(`independent proof hash binding ${file} complete=${state}`, () => {
    const sources = implementation(state);
    mutate(sources, PHASE162_DOC, source => source.replace(sha(sources.get(file)!), "0".repeat(64)));
    expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("source hash"))).toBe(true);
  });
}
for (const [name] of reports) for (const field of ["status", "generated_by", "phase_lifecycle_id", "lifecycle_mode", "generated_at", "source_digest"]) test(`formal ${name} field ${field}`, () => {
  const sources = implementation(true);
  const file = PHASE162 + `162-${name}.md`;
  mutate(sources, file, source => source.replace(new RegExp(`^${field}: .+$`, "m"), `${field}: invalid`));
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(`Phase 162 ${name} gate`))).toBe(true);
});
for (const [name] of reports) test(`formal report omission ${name}`, () => {
  const sources = implementation(true);
  sources.delete(PHASE162 + `162-${name}.md`);
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes(`Phase 162 ${name} gate`))).toBe(true);
});
for (const id of ["CFPR-02", "CFGR-01", "CFGR-02"]) {
  for (const complete of [false, true]) {
    for (const change of ["checkbox", "traceability", "missing"]) test(`shared requirement ${id} ${change} complete=${complete}`, () => {
      const sources = complete ? implementation(true) : pending();
      mutate(sources, ".planning/REQUIREMENTS.md", source => {
        if (change === "checkbox") return source.replace(new RegExp(`^- \\[[ x]\\] \\*\\*${id}\\*\\*:`, "m"), `- [${complete ? " " : "x"}] **${id}**:`);
        if (change === "missing") return source.replace(new RegExp(`^- \\[[ x]\\] \\*\\*${id}\\*\\*:.*$`, "m"), "");
        return source.replace(new RegExp(`(\\|\\s*${id}\\s*\\|\\s*Phase 162\\s*\\|)\\s*(Pending|Complete)\\s*\\|`), `$1 ${complete ? "Pending" : "Complete"} |`);
      });
      expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("requirement checkbox/traceability"))).toBe(true);
    });
  }
}
for (const change of ["duplicate", "wrong id", "wrong requirement", "wrong order", "wrong baseline", "premature owner", "missing source registration"]) test(`shared unique owner ${change}`, () => {
  const sources = pending();
  inventory(sources, (owner, index) => {
    if (change === "duplicate") index.checklist.surfaces.push({ ...owner });
    if (change === "wrong id") owner.id = "invented-owner";
    if (change === "wrong requirement") owner.requirements[0] = "UNRELATED-01";
    if (change === "wrong order") owner.requirements.reverse();
    if (change === "wrong baseline") index.baseline = "wrong baseline";
    if (change === "premature owner") owner.status = "done";
    if (change === "missing source registration") owner.evidence = owner.evidence.filter(file => file !== PHASE162_EVIDENCE[0]);
  });
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("T-162-10"))).toBe(true);
});
for (const [name, field, replacement] of [["VERIFICATION", "lifecycle_validated", "false"], ["SECURITY", "threats_open", "1"]]) test(`shared final ${name} ${field}`, () => {
  const sources = implementation(true);
  mutate(sources, PHASE162 + `162-${name}.md`, source => source.replace(new RegExp(`^${field}: .+$`, "m"), `${field}: ${replacement}`));
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("full native/security/lifecycle"))).toBe(true);
});
test("no future proof can be silently borrowed from the filesystem", () => {
  const sources = implementation();
  sources.delete(PHASE162_EVIDENCE[0]);
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("source hash") || failure.includes("required current source missing"))).toBe(true);
});
test("duplicate current proof source row refuses", () => {
  const sources = implementation();
  sources.set(PHASE162_DOC, sources.get(PHASE162_DOC)! + `| \`${PHASE162_EVIDENCE[0]}\` | \`${sha(sources.get(PHASE162_EVIDENCE[0])!)}\` |\n`);
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("source hash"))).toBe(true);
});
test("implementation without proof registration refuses", () => {
  const sources = implementation();
  inventory(sources, owner => { owner.evidence = owner.evidence.filter(file => file !== PHASE162_DOC); });
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("linked current-lifecycle"))).toBe(true);
});
for (const replacement of ["bash scripts/verify.sh --fast", "We ran bash scripts/verify.sh successfully.", "`bash scripts/verify.sh`"]) test(`shared full native predicate rejects ${replacement}`, () => {
  const sources = implementation(true);
  mutate(sources, PHASE162 + "162-NATIVE-EVIDENCE.md", source => source.replace("\nbash scripts/verify.sh\n", `\n${replacement}\n`));
  expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("full native/security/lifecycle"))).toBe(true);
});
for (const claim of ["BIP37 is supported.", "V0 serving is enabled.", "assumeutxo is proven.", "dual-chainstate is supported.", "archive-scale is ready.", "public defaults are enabled.", "public-network CI is complete.", "repair is available.", "import is available.", "download is enabled.", "production-readiness is established.", "funds-safety is guaranteed.", "v2.5 is shipped.", "Phase 162 proof is complete."]) {
  for (const state of ["pending", "implementation", "complete"]) test(`independent exclusion ${state}: ${claim}`, () => {
    const sources = state === "pending" ? pending() : implementation(state === "complete");
    inventory(sources, owner => { owner.known_gaps.push(claim); });
    expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("unsupported claim"))).toBe(true);
  });
}

for (const executed of [false, true]) {
  for (const command of [...NATIVE_COMMANDS.slice(1), SHARED_FORMAT]) {
    for (const mode of ["remove", "duplicate", "replace"]) test(`native ${executed ? "executed" : "documented"} ${mode} ${command}`, () => {
      const sources = pending();
      mutate(sources, "scripts/verify.sh", source => {
        const lines = source.split("\n");
        const index = lines.findIndex(line => (executed ? line.startsWith("run_step ") : !line.startsWith("run_step ")) && line.endsWith(command));
        expect(index).toBeGreaterThanOrEqual(0);
        if (mode === "remove") lines[index] = "# removed native stage";
        if (mode === "duplicate") lines.splice(index, 0, lines[index]);
        if (mode === "replace") lines[index] = lines[index].replace(command, command.includes("--edition") ? command.replace("--edition 2024", "--edition 2021") : command.includes("./scripts/") ? command.replace("./scripts/", "scripts/") : command.replace(".ts", ".removed.ts"));
        return lines.join("\n");
      });
      expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("scripts/verify.sh:"))).toBe(true);
    });
  }
  test(`native reordered ${executed ? "executed" : "documented"} checker/oracle`, () => {
    const sources = pending();
    mutate(sources, "scripts/verify.sh", source => {
      const lines = source.split("\n");
      const first = lines.findIndex(line => (executed ? line.startsWith("run_step ") : !line.startsWith("run_step ")) && line.endsWith(NATIVE_COMMANDS[1]));
      [lines[first], lines[first + 1]] = [lines[first + 1], lines[first]];
      return lines.join("\n");
    });
    expect(checkPhase162IntegratedFilterProof(sources).some(failure => failure.includes("ordered Phase161/162"))).toBe(true);
  });
}
