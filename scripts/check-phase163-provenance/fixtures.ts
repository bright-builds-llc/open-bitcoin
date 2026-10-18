import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, constants, existsSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { CHECK_FILES as FILES158, PHASE162, PHASE162_DOC, PHASE162_EVIDENCE, PHASE162_COMPLETE,
  PHASE162_IMPLEMENTATION, checkPhase158ValidatedReorg, loadPhase162OptionalSources } from "../check-phase158-validated-reorg.ts";
import { CHECK_FILES as FILES162, checkPhase162IntegratedFilterProof, loadSources } from "../check-phase162-integrated-filter-proof.ts";
import { PHASE163, PHASE163_DOC, ORIGINAL_AUDITS, provenanceDigest,
  V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS, GAP_CLAIM, CLOSURE_CLAIM, READINESS_CLAIM, ARCHIVED_CLAIM } from "../check-phase163-provenance.ts";
import { v25FixtureSources } from "./fixture-controls.ts";

export const REPO = resolve(import.meta.dir, "../..");
export const ENTRIES = ["scripts/check-phase158-validated-reorg.ts", "scripts/check-phase162-integrated-filter-proof.ts"];
export const CURRENT_CLAIM_FILES = ["README.md", "packages/README.md", "docs/parity/v2-5-validated-reorg.md", PHASE163_DOC];
export const REPORT_NAMES = ["VERIFICATION", "NATIVE-EVIDENCE", "REVIEW", "SECURITY", "04-SUMMARY"];
export type Layout = "original-bootstrap" | "original-implementation" | "original-formal" | "gap-pending"
  | "functional-closure" | "audit-in-progress" | "final" | "overlap" | "archive-only" | "next-active";
export const LAYOUTS: Layout[] = ["original-bootstrap", "original-implementation", "original-formal", "gap-pending",
  "functional-closure", "audit-in-progress", "final", "overlap", "archive-only", "next-active"];
export const sha = (source: string) => createHash("sha256").update(source).digest("hex");
const ownedRoots = new Set<string>();
export const cleanedRoots: string[] = [];
const liveFiles = [...new Set([...PHASE162_EVIDENCE, PHASE162_DOC, PHASE162 + "162-CONTEXT.md", ...ENTRIES,
  "scripts/check-phase158-validated-reorg.test.ts", "scripts/check-phase163-provenance.ts", ...REPORT_NAMES.slice(0, 4).map(name => PHASE162 + `162-${name}.md`),
  ...V25_REQUIREMENTS, ...V25_ROADMAPS, ...V25_AUDITS, ...ORIGINAL_AUDITS])];
const optionalControls = new Set([...V25_REQUIREMENTS, ...V25_ROADMAPS, ...V25_AUDITS]);
function maybeLiveHash(file: string): string | undefined {
  const full = resolve(REPO, file);
  const maybeStat = lstatSync(full, { throwIfNoEntry: false });
  if (!maybeStat && optionalControls.has(file)) return undefined;
  if (!maybeStat?.isFile()) throw new Error(`regular live evidence file required: ${file}`);
  return sha(readFileSync(full, "utf8"));
}
const liveHashes = new Map(liveFiles.map(file => [file, maybeLiveHash(file)]));

const JSON_RUNTIME_EVIDENCE = "packages/testdata/phase164/historical-tooling/manifest.json";
type RuntimeGraph = { sources: Map<string, string>; evidenceImports: Set<string> };
const runtimeGraphs = new Map<string, RuntimeGraph>();

function relativeFile(root: string, file: string): string {
  const full = resolve(root, file), normalized = relative(root, full);
  if (!file || isAbsolute(file) || file.includes("\\") || normalized.startsWith("../") || normalized === ".."
    || normalized !== file || file.split("/").some(part => part === ".." || part === "." || !part)) {
    throw new Error(`invalid bounded fixture path: ${file}`);
  }
  return full;
}

function boundedPath(root: string, file: string): string {
  const full = relativeFile(root, file);
  const rootStat = lstatSync(root);
  if (!rootStat.isDirectory() || rootStat.isSymbolicLink()) throw new Error("regular bounded fixture root required");
  let cursor = root;
  for (const part of file.split("/")) {
    cursor = join(cursor, part);
    const maybeStat = lstatSync(cursor, { throwIfNoEntry: false });
    if (maybeStat?.isSymbolicLink()) throw new Error(`symlink bounded fixture path: ${file}`);
    if (maybeStat && cursor !== full && !maybeStat.isDirectory()) throw new Error(`non-directory bounded fixture component: ${file}`);
    if (maybeStat && cursor === full && !maybeStat.isFile()) throw new Error(`regular bounded fixture file required: ${file}`);
  }
  return full;
}

function regularModule(root: string, file: string): string {
  const full = boundedPath(root, file);
  const stat = lstatSync(full);
  if (!stat.isFile() || stat.size > 512_000 || !file.endsWith(".ts")) throw new Error(`regular bounded TypeScript module required: ${file}`);
  const source = readFileSync(full, "utf8");
  if (Buffer.byteLength(source) > 512_000) throw new Error(`runtime module byte bound exceeded: ${file}`);
  return source;
}

/** Check computed import calls too: scanImports deliberately lists only literal edges. */
function assertLiteralRuntimeCalls(source: string, file: string): void {
  function quoted(start: number): number {
    const quote = source[start];
    for (let index = start + 1; index < source.length; index++) {
      if (source[index] === "\\") { index++; continue; }
      if (source[index] === quote) return index + 1;
    }
    throw new Error(`unterminated runtime string: ${file}`);
  }
  function template(start: number): number {
    for (let index = start + 1; index < source.length; index++) {
      if (source[index] === "\\") { index++; continue; }
      if (source[index] === "`") return index + 1;
      if (source.startsWith("${", index)) index = code(index + 2, true) - 1;
    }
    throw new Error(`unterminated runtime template: ${file}`);
  }
  function code(start: number, closingBrace = false): number {
    let depth = 0;
    for (let index = start; index < source.length; index++) {
      const char = source[index];
      if (source.startsWith("//", index)) {
        const end = source.indexOf("\n", index + 2);
        index = end < 0 ? source.length : end;
        continue;
      }
      if (source.startsWith("/*", index)) {
        const end = source.indexOf("*/", index + 2);
        if (end < 0) throw new Error(`unterminated runtime comment: ${file}`);
        index = end + 1;
        continue;
      }
      if (char === '"' || char === "'") { index = quoted(index) - 1; continue; }
      if (char === "`") { index = template(index) - 1; continue; }
      if (char === "/" && /(?:^|[=(:,!&|?{;]|\breturn|\bthrow)$/.test(source.slice(start, index).trimEnd())) {
        let characterClass = false;
        for (index++; index < source.length; index++) {
          if (source[index] === "\\") { index++; continue; }
          if (source[index] === "[") characterClass = true;
          if (source[index] === "]") characterClass = false;
          if (source[index] === "/" && !characterClass) break;
        }
        continue;
      }
      if (char === "{") depth++;
      if (char === "}" && depth-- === 0 && closingBrace) return index + 1;
      if (char !== "i" && char !== "r") continue;
      if (index > start && /[\w$]/.test(source[index - 1])) continue;
      const maybeCall = source.slice(index).match(/^(?:import|require)\s*\(/);
      if (maybeCall && !/^\s*(["'])[^"'\\\r\n]*\1\s*\)/.test(source.slice(index + maybeCall[0].length))) {
        throw new Error(`unsupported nonliteral runtime import: ${file}`);
      }
    }
    if (closingBrace) throw new Error(`unterminated runtime interpolation: ${file}`);
    return source.length;
  }
  code(0);
}

function discoverRuntime(entries: readonly string[], root: string): RuntimeGraph {
  const graph: RuntimeGraph = { sources: new Map(), evidenceImports: new Set() };
  const pending = [...entries], transpiler = new Bun.Transpiler({ loader: "ts" });
  while (pending.length) {
    const file = pending.pop()!;
    if (graph.sources.has(file)) continue;
    if (graph.sources.size >= 32) throw new Error("native fixture import-tree bound exceeded");
    const source = regularModule(root, file);
    graph.sources.set(file, source);
    const parsed = source.replace(/^#![^\n]*/, line => " ".repeat(line.length));
    assertLiteralRuntimeCalls(transpiler.transformSync(parsed), file);
    for (const imported of transpiler.scanImports(parsed)) {
      if (imported.path.startsWith("node:") || imported.path.startsWith("bun:")) continue;
      if (!imported.path.startsWith(".")) throw new Error(`unsupported runtime package: ${imported.path}`);
      const child = relative(root, resolve(dirname(resolve(root, file)), imported.path));
      relativeFile(root, child);
      if (child === JSON_RUNTIME_EVIDENCE) {
        graph.evidenceImports.add(child);
        continue;
      }
      if (!child.endsWith(".ts")) throw new Error(`unsupported bounded runtime import: ${child}`);
      pending.push(child);
    }
  }
  return graph;
}

/** Fixed roots only; cache the actual live graph lazily, never caller-owned map mutations. */
export function runtimeSources(entries: readonly string[] = ENTRIES, maybeRoot?: string,
  maybeEvidence?: ReadonlyMap<string, string>): Map<string, string> {
  if (!entries.length || new Set(entries).size !== entries.length || entries.some(file => !ENTRIES.includes(file))) {
    throw new Error("nonempty fixed runtime entry subset required");
  }
  const root = resolve(maybeRoot ?? REPO);
  const stat = lstatSync(root);
  if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("regular runtime repository root required");
  const key = [...entries].sort().join("\n");
  let maybeGraph = maybeRoot === undefined ? runtimeGraphs.get(key) : undefined;
  if (!maybeGraph) {
    maybeGraph = discoverRuntime(entries, root);
    if (maybeRoot === undefined) runtimeGraphs.set(key, maybeGraph);
  }
  for (const file of maybeGraph.evidenceImports) if (!maybeEvidence?.has(file)) throw new Error(`required fixed runtime evidence missing: ${file}`);
  return new Map(maybeGraph.sources);
}

/** Add only absent runtime modules; every supplied evidence key, value and order survives. */
export function copiedCliSources(sources: ReadonlyMap<string, string>, entries: readonly string[] = ENTRIES): Map<string, string> {
  const copied = new Map(sources);
  for (const [file, source] of runtimeSources(entries, undefined, sources)) if (!copied.has(file)) copied.set(file, source);
  return checkedFixtureSources(copied);
}

function checkedFixtureSources(sources: ReadonlyMap<string, string>): Map<string, string> {
  const copied = new Map(sources);
  if (copied.size > 400) throw new Error("native fixture evidence bound exceeded");
  for (const file of copied.keys()) relativeFile(REPO, file);
  return copied;
}

/** Copy only fixed evidence and the actual entrypoints' finite relative-import tree. */
function corpus(): Map<string, string> {
  const sources = loadPhase162OptionalSources(REPO);
  for (const file of [...new Set([...FILES158, ...FILES162])]) sources.set(file, readFileSync(resolve(REPO, file), "utf8"));
  for (const [file, source] of runtimeSources(ENTRIES, undefined, sources)) sources.set(file, source);
  if (sources.size > 400) throw new Error("native fixture evidence bound exceeded");
  return sources;
}
const base = corpus();
export const CORPUS_MANIFEST = [...base].map(([file, source]) => `${file}:${sha(source)}`).sort();
export const CORPUS_DIGEST = sha(CORPUS_MANIFEST.join("\n"));
export const CORPUS_BYTES = [...base.values()].reduce((total, source) => total + Buffer.byteLength(source), 0);

export function change(sources: Map<string, string>, file: string, transform: (source: string) => string): void {
  const maybeSource = sources.get(file);
  if (maybeSource === undefined) throw new Error(`mutation fixture source absent: ${file}`);
  const changed = transform(maybeSource);
  if (changed === maybeSource) throw new Error(`mutation did not change bytes: ${file}`);
  sources.set(file, changed);
}

type ParityOwner = { id: string; status: string; known_gaps: string[] };
function inventory(sources: Map<string, string>, transform: (owner: ParityOwner) => void): void {
  const index = JSON.parse(sources.get("docs/parity/index.json")!);
  transform(index.checklist.surfaces.find((row: ParityOwner) => row.id === "v2-5-integrated-retained-filter-proof"));
  sources.set("docs/parity/index.json", JSON.stringify(index));
}

function pending(): Map<string, string> {
  const sources = v25FixtureSources(base);
  sources.set(PHASE163_DOC, "# Phase 163 milestone provenance\n\nSynthetic native-fixture test data; not a live acceptance certificate.\n");
  change(sources, V25_REQUIREMENTS[0], source => source.replace(/^- \[[ xX]\] \*\*CFGR-01\*\*:/m, "- [ ] **CFGR-01**:")
    .replace(/(\|\s*CFGR-01\s*\|)\s*Phase 16[23]\s*\|\s*(?:Pending|Complete)\s*\|/, "$1 Phase 163 | Pending |") + "\n");
  sources.set(V25_AUDITS[0], sources.get(ORIGINAL_AUDITS[0])!);
  return sources;
}

function currentClaim(sources: Map<string, string>, claim: string): void {
  for (const file of CURRENT_CLAIM_FILES) sources.set(file, sources.get(file)! + "\n" + claim + "\n");
  inventory(sources, owner => { owner.known_gaps.push(claim); });
}

/** Synthetic report strings are test data, never live acceptance certificates. */
function closureReports(sources: Map<string, string>, final: boolean): void {
  const fields = `lifecycle_mode: yolo\nphase_lifecycle_id: 163-2026-10-12T21-09-41\ngenerated_at: 2026-10-12T23:00:00Z\nsource_digest: ${provenanceDigest(sources)}\n`;
  const report = (name: string, status: string, generator: string, extra: string) => sources.set(PHASE163 + `163-${name}.md`,
    `---\n${fields}status: ${status}\ngenerated_by: ${generator}\n${extra}---\nSynthetic Phase163 native-fixture test data; not a live acceptance certificate.\n`);
  report("04-SUMMARY", final ? "complete" : "in_progress", "gsd-execute-plan", `requirements-completed: [CFGR-01]\nexecution_status: ${final ? "complete" : "in_progress"}\n`);
  report("VERIFICATION", "passed", "gsd-verifier", `lifecycle_validated: true\nrequirements: [CFGR-01]\nsuccess_criteria: [SC1, SC2, SC3, SC4${final ? ", SC5" : ""}]\nreadiness_gate: enforced\nverification_scope: ${final ? "phase" : "functional-closure"}\naudit_status: ${final ? "passed" : "pending"}\nreadiness: ${final ? "established" : "not_asserted"}\nplan04_status: ${final ? "complete" : "in_progress"}\nphase_status: ${final ? "complete" : "in_progress"}\n`);
  report("NATIVE-EVIDENCE", "passed", "gsd-execute-phase", "exit_code: 0\n");
  sources.set(PHASE163 + "163-NATIVE-EVIDENCE.md", sources.get(PHASE163 + "163-NATIVE-EVIDENCE.md")! + "bash scripts/verify.sh\n");
  report("REVIEW", "clean", "gsd-code-reviewer", "");
  report("SECURITY", "verified", "gsd-security-auditor", "threats_open: 0\n");
}

function passedAudit(sources: Map<string, string>): void {
  sources.set(V25_AUDITS[0], `---\nmilestone: v2.5\ngenerated_by: gsd-audit-milestone\nstatus: passed\naudited: 2026-10-12T23:01:00Z\nphase_lifecycle_id: 163-2026-10-12T21-09-41\nsource_digest: ${provenanceDigest(sources)}\nscores:\n  requirements: 22/22\ngaps:\n  requirements: []\n  integration: []\n  flows: []\n---\nSynthetic Phase163 native-fixture test data; not a live acceptance certificate.\n`);
}

export function archiveOverlap(sources: Map<string, string>): void {
  const header = "# Requirements Archive: v2.5 Prune-Aware Compact-Filter Serving\n\n**Archived:** 2026-10-12\n**Status:** SHIPPED\n\nFor current requirements, see `.planning/REQUIREMENTS.md`.\n\n---\n\n";
  sources.set(V25_REQUIREMENTS[1], header + sources.get(V25_REQUIREMENTS[0])!);
  sources.set(V25_ROADMAPS[1], sources.get(V25_ROADMAPS[0])!);
  sources.set(V25_AUDITS[1], sources.get(V25_AUDITS[0])!);
  sources.delete(V25_AUDITS[0]);
}

export function archiveOnly(sources: Map<string, string>): void {
  sources.delete(V25_REQUIREMENTS[0]);
  // Standard closeout reorganizes active ROADMAP after preserving its v2.5 copy.
  sources.set(V25_ROADMAPS[0], "# Roadmap\n\nv2.5 controls: .planning/milestones/v2.5-ROADMAP.md\n");
  for (const [file, source] of sources) if ((file.endsWith(".md") && !file.startsWith(PHASE162) && file !== PHASE162_DOC) || file === "docs/parity/index.json") {
    sources.set(file, source.replaceAll(CLOSURE_CLAIM, ARCHIVED_CLAIM).replaceAll(READINESS_CLAIM, ARCHIVED_CLAIM));
  }
}

export function nextActive(sources: Map<string, string>): void {
  sources.set(V25_REQUIREMENTS[0], "# Requirements\n\n**Milestone:** v2.6 Future\n\n- [ ] **NEXT-01**: unrelated next milestone\n");
  sources.set(V25_ROADMAPS[0], "# Roadmap\n\n## Active Milestone: v2.6 Future\n\n### Phase 164: Future\n**Requirements**: NEXT-01\n");
}

export function fixtureSources(layout: Layout): Map<string, string> {
  const sources = pending();
  if (layout.startsWith("original-")) {
    sources.delete(PHASE163_DOC);
    const complete = layout === "original-formal";
    change(sources, V25_ROADMAPS[0], source => source.replace("**Requirements**: CFPR-02, CFGR-02", "**Requirements**: CFPR-02, CFGR-01, CFGR-02").replace("**Requirements**: CFGR-01", "**Requirements**: none"));
    change(sources, V25_REQUIREMENTS[0], source => source.replace(/^- \[[ xX]\] \*\*(CFPR-02|CFGR-01|CFGR-02)\*\*:/gm, `- [${complete ? "x" : " "}] **$1**:`)
      .replace(/(\|\s*(?:CFPR-02|CFGR-01|CFGR-02)\s*\|)\s*Phase 16[23]\s*\|\s*(?:Pending|Complete)\s*\|/g, `$1 Phase 162 | ${complete ? "Complete" : "Pending"} |`));
    if (!complete) {
      const claim = layout === "original-implementation" ? PHASE162_IMPLEMENTATION : "Phase162 scoped proof remains pending under phases159 and162; v2.5 remains unaudited and unshipped.";
      for (const [file, source] of sources) if (file.endsWith(".md") || file.endsWith(".json")) sources.set(file, source.replaceAll(PHASE162_COMPLETE, claim));
      inventory(sources, owner => { owner.status = "in_progress"; });
      for (const name of REPORT_NAMES.slice(0, 4)) sources.delete(PHASE162 + `162-${name}.md`);
      if (layout === "original-bootstrap") sources.delete(PHASE162_DOC);
    }
    return sources;
  }
  if (layout === "gap-pending") { currentClaim(sources, GAP_CLAIM); return sources; }
  change(sources, V25_REQUIREMENTS[0], source => source.replace("[ ] **CFGR-01**", "[x] **CFGR-01**")
    .replace(/(\|\s*CFGR-01\s*\|\s*Phase 163\s*\|)\s*Pending\s*\|/, "$1 Complete |"));
  const final = !["functional-closure", "audit-in-progress"].includes(layout);
  closureReports(sources, final);
  currentClaim(sources, CLOSURE_CLAIM);
  if (layout !== "functional-closure") { passedAudit(sources); currentClaim(sources, READINESS_CLAIM); }
  if (["overlap", "archive-only", "next-active"].includes(layout)) archiveOverlap(sources);
  if (["archive-only", "next-active"].includes(layout)) archiveOnly(sources);
  if (layout === "next-active") nextActive(sources);
  return sources;
}

export function writeSources(root: string, sources: ReadonlyMap<string, string>): void {
  if (!ownedRoots.has(root)) throw new Error("unowned fixture path");
  writeCopiedSources(root, sources);
}

function writeCopiedSources(root: string, sources: ReadonlyMap<string, string>): void {
  for (const file of sources.keys()) relativeFile(root, file);
  for (const [file, source] of sources) {
    const destination = boundedPath(root, file);
    mkdirSync(dirname(destination), { recursive: true });
    const descriptor = openSync(destination, constants.O_WRONLY | constants.O_CREAT | constants.O_TRUNC | constants.O_NOFOLLOW, 0o600);
    try {
      if (!fstatSync(descriptor).isFile()) throw new Error(`regular bounded fixture file required: ${file}`);
      writeFileSync(descriptor, source);
    } finally { closeSync(descriptor); }
  }
}

/** Register caller ownership before writes; the caller retains its own cleanup lifecycle. */
export function createCopiedFixture(sources: ReadonlyMap<string, string>, registerRoot: (root: string) => void,
  entries: readonly string[] = ENTRIES, mode: "executable" | "evidence-only" = "executable"): string {
  if (mode !== "executable" && mode !== "evidence-only") throw new Error("unsupported copied fixture mode");
  const copied = mode === "executable" ? copiedCliSources(sources, entries) : checkedFixtureSources(sources);
  const root = mkdtempSync(join(tmpdir(), "phase158-evidence-"));
  registerRoot(root);
  writeCopiedSources(root, copied);
  return root;
}

export function assertLiveUnchanged(): void {
  for (const [file, maybeHash] of liveHashes) if (maybeLiveHash(file) !== maybeHash) throw new Error(`live fixture boundary changed: ${file}`);
}

export function cleanupFixtures(): void {
  for (const root of ownedRoots) {
    rmSync(root, { recursive: true });
    if (existsSync(root)) throw new Error(`fixture cleanup failed: ${root}`);
    ownedRoots.delete(root);
    cleanedRoots.push(root);
  }
}

export function withFixture<T>(sources: ReadonlyMap<string, string>, action: (root: string) => T): T {
  const root = mkdtempSync(join(tmpdir(), "phase163-native-"));
  ownedRoots.add(root);
  try { writeSources(root, sources); return action(root); }
  finally { cleanupFixtures(); assertLiveUnchanged(); }
}

export type Boundary = { name: string; failures: string[] };
const captured = (action: () => string[]) => { try { return action(); } catch (error) { return [String(error)]; } };
export function boundaries(root: string, sources: ReadonlyMap<string, string>): Boundary[] {
  const results = [
    { name: "158-map", failures: checkPhase158ValidatedReorg(root, sources) },
    { name: "162-map", failures: checkPhase162IntegratedFilterProof(sources) },
    { name: "158-filesystem", failures: captured(() => checkPhase158ValidatedReorg(root)) },
    { name: "162-filesystem", failures: captured(() => checkPhase162IntegratedFilterProof(loadSources(root))) },
  ];
  for (const entry of ENTRIES) {
    const result = spawnSync(process.execPath, ["run", entry], { cwd: root, encoding: "utf8", timeout: 30_000, maxBuffer: 256 * 1024 });
    if (result.error) throw result.error;
    if (result.signal) throw new Error(`${entry}: unexpected signal ${result.signal}`);
    results.push({ name: entry, failures: result.status === 0 ? [] : [`exit ${result.status}: ${result.stderr.slice(0, 24_000)}`] });
  }
  return results;
}

export function checkAllBoundaries(root: string, sources: ReadonlyMap<string, string>): string[] {
  return boundaries(root, sources).flatMap(result => result.failures.map(failure => `${result.name}: ${failure}`));
}
