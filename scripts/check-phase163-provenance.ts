import { createHash } from "node:crypto";
import { historicalToolingView, HISTORICAL_TOOLING_FILES } from "./check-phase164-canonical-identity/historical-tooling.ts";

export const PHASE163 = ".planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/";
export const PHASE163_DOC = "docs/parity/v2-5-archive-safe-provenance.md";
const PHASE162 = ".planning/phases/162-real-prune-retention-and-integrated-parity-proof/";
const PROOF = "docs/parity/v2-5-integrated-filter-proof.md";
export const V25_REQUIREMENTS = [".planning/REQUIREMENTS.md", ".planning/milestones/v2.5-REQUIREMENTS.md"];
export const V25_ROADMAPS = [".planning/ROADMAP.md", ".planning/milestones/v2.5-ROADMAP.md"];
export const V25_AUDITS = [".planning/v2.5-MILESTONE-AUDIT.md", ".planning/milestones/v2.5-MILESTONE-AUDIT.md"];
export const ORIGINAL_AUDITS = [PHASE163 + "163-ORIGINAL-MILESTONE-AUDIT.md", PHASE163 + "163-ORIGINAL-INTEGRATION-AUDIT.md"];
export const ORIGINAL_HASHES = ["f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8", "f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709"];
export const PROVENANCE_CODE = ["scripts/check-phase163-provenance.ts"];
export const PROVENANCE_DIGEST_FILES = [...PROVENANCE_CODE, "scripts/check-phase163-provenance.test.ts",
  "scripts/check-phase158-validated-reorg.ts", "scripts/check-phase158-validated-reorg.test.ts",
  "scripts/check-phase162-integrated-filter-proof.ts", "scripts/check-phase162-integrated-filter-proof.test.ts",
  "scripts/check-phase159-filter-rpcs.ts", "scripts/check-phase159-filter-rpcs.test.ts",
  "scripts/check-phase160-peer-filter-serving.ts", "scripts/check-phase160-peer-filter-serving.test.ts",
  "scripts/check-phase161-operator-index-evidence.ts", "scripts/check-phase161-operator-index-evidence.test.ts",
  "scripts/check-phase163-provenance/native.test.ts", "scripts/check-phase163-provenance/fixtures.ts", "scripts/check-phase163-provenance/fixture-controls.ts",
  "scripts/check-phase162-integrated-filter-proof/contracts.ts", "scripts/verify.sh",
  PHASE163 + "163-CONTEXT.md", PHASE163 + "163-GAP-SCOPE.md", ...ORIGINAL_AUDITS];
export const PROVENANCE_FILES = [...new Set([...V25_REQUIREMENTS, ...V25_ROADMAPS, ...V25_AUDITS, ...HISTORICAL_TOOLING_FILES,
  ...PROVENANCE_DIGEST_FILES, PROOF, PHASE163_DOC, PHASE162 + "162-CONTEXT.md",
  ...["VERIFICATION", "NATIVE-EVIDENCE", "REVIEW", "SECURITY"].map(name => PHASE162 + `162-${name}.md`),
  ...["VERIFICATION", "NATIVE-EVIDENCE", "REVIEW", "SECURITY", "CLOSURE-VERIFICATION", "04-SUMMARY"].map(name => PHASE163 + `163-${name}.md`)])];

export const GAP_CLAIM = "Phase 162 scoped integrated proof remains historical delivery evidence; CFGR-01 is Pending under Phase 163 for AG01/AF01; v2.5 is audited with gaps, unarchived and unshipped.";
export const CLOSURE_CLAIM = "CFGR-01 archive-safe requirement closure is complete under Phase 163; v2.5 remains unarchived and unshipped.";
export const READINESS_CLAIM = "v2.5 archive readiness is established by the renewed passing milestone audit; v2.5 remains unarchived and unshipped.";
export const ARCHIVED_CLAIM = "v2.5 controls are archived under .planning/milestones/; this provenance does not establish new runtime capability or production/funds safety.";
export const PROVENANCE_CLAIMS = [GAP_CLAIM, CLOSURE_CLAIM, READINESS_CLAIM, ARCHIVED_CLAIM];
export type V25Provenance = {
  historical: "pending" | "implementation" | "complete";
  closure: "original" | "gap_pending" | "gap_complete";
  publication: "unaudited" | "gaps_found" | "audit_passed" | "archived";
  requirementsPath: string;
  roadmapPath: string;
  maybeAuditPath?: string;
};
type RecordValue = Record<string, unknown>;
type Requirement = { id: string; description: string; owner: number; status: string };
type SourceDisposition = "validate" | "use-archive" | "invalid";
const IDS = ["CFIL-01", "CFIL-02", "CFAC-01", "CFAC-02", "CFIX-01", "CFIX-02", "CFIX-03", "CFIX-04",
  "CFPR-01", "CFPR-02", "CFPR-03", "CFNET-01", "CFNET-02", "CFNET-03", "CFNET-04", "CFNET-05", "CFNET-06",
  "CFRP-01", "CFRP-02", "CFOP-01", "CFGR-01", "CFGR-02"];
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const isRecord = (value: unknown): value is RecordValue => typeof value === "object" && value !== null && !Array.isArray(value);
function timestamp(value: unknown): number {
  if (typeof value !== "string" || !/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$/.test(value)
    || !Number.isFinite(Date.parse(value)) || new Date(value).toISOString().replace(".000Z", "Z") !== value) throw new Error("missing/invalid canonical timestamp");
  return Date.parse(value);
}
function canonical(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (isRecord(value)) return `{${Object.keys(value).sort().map(key => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  return JSON.stringify(value);
}

/** Bind closure receipts to fixed implementation inputs, never mutable reports or audits. */
export function provenanceDigest(sources: ReadonlyMap<string, string>): string {
  const historical = historicalToolingView(sources);
  return hash(PROVENANCE_DIGEST_FILES.map(file => `${file}:${hash(historical.get(file) ?? "")}`).join("\n"));
}

/** Parse only the first frontmatter; reject ambiguous keys before YAML can overwrite them. */
export function provenanceMetadata(source: string): RecordValue {
  const maybeHeader = source.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/)?.[1];
  if (!maybeHeader) throw new Error("missing first frontmatter");
  const levels = new Map<number, Set<string>>();
  for (const line of maybeHeader.split(/\r?\n/)) {
    const structure = line.replace(/"(?:\\.|[^"\\])*"|'(?:''|[^'])*'/g, '""');
    if (/^\s*(?:-\s+)?["'].*["']\s*:/.test(line) || /^\s*(?:-\s+)?(?:[!&*?\[]|<<\s*:|\{)/.test(line)
      || /^\s*(?:-\s+)?[A-Za-z_][\w-]*\s*:\s*(?:\{|\[[^\n]*\{)/.test(structure)) throw new Error("unsupported noncanonical metadata key/mapping");
    const maybeKey = line.match(/^( *)(-\s+)?([A-Za-z_][\w-]*)\s*:/);
    if (!maybeKey) continue;
    const indent = maybeKey[1].length + (maybeKey[2] ? 2 : 0);
    for (const depth of levels.keys()) if (depth > indent || (maybeKey[2] && depth >= indent)) levels.delete(depth);
    const keys = levels.get(indent) ?? new Set<string>();
    if (keys.has(maybeKey[3])) throw new Error(`duplicate metadata ${maybeKey[3]}`);
    keys.add(maybeKey[3]);
    levels.set(indent, keys);
  }
  const parsed: unknown = Bun.YAML.parse(maybeHeader);
  if (!isRecord(parsed)) throw new Error("frontmatter must be a mapping");
  return parsed;
}

function activeControlDisposition(source: string, kind: "requirements" | "roadmap"): SourceDisposition {
  const lines = source.split(/\r?\n/);
  const primary = lines.filter(line => kind === "requirements"
    ? /^\*\*Milestone\b/.test(line) : /^##[ \t]+Active Milestone\b/.test(line));
  const secondary = lines.filter(line => kind === "requirements"
    ? /^##[ \t]+v[^ \t]*[ \t]+Requirements\b/.test(line) : /^#[ \t]+Milestone\b/.test(line));
  if (kind === "roadmap" && !primary.length && !secondary.length
    && source === `# Roadmap\n\nv2.5 controls: ${V25_ROADMAPS[1]}\n`) return "use-archive";
  if (primary.length > 1 || secondary.length > 1
    || (kind === "requirements" ? primary.length !== 1 : primary.length + secondary.length === 0)) return "invalid";
  const primaryPattern = kind === "requirements"
    ? /^\*\*Milestone:\*\*[ \t]*(v[^ \t]+)(?:[ \t].*)?$/
    : /^##[ \t]+Active Milestone:[ \t]*(v[^ \t]+)(?:[ \t].*)?$/;
  const secondaryPattern = kind === "requirements"
    ? /^##[ \t]+(v[^ \t]+)[ \t]+Requirements(?:[ \t].*)?$/
    : /^#[ \t]+Milestone[ \t]+(v[^ \t:]+):(?:[ \t].*)?$/;
  const maybeVersions = [...primary.map(line => line.match(primaryPattern)?.[1]),
    ...secondary.map(line => line.match(secondaryPattern)?.[1])];
  const maybeVersion = maybeVersions[0];
  if (!maybeVersion || maybeVersions.some(maybeValue => maybeValue !== maybeVersion)) return "invalid";
  const maybeParts = maybeVersion.match(/^v(0|[1-9]\d*)\.(0|[1-9]\d*)$/);
  if (!maybeParts) return "invalid";
  const major = Number(maybeParts[1]);
  const minor = Number(maybeParts[2]);
  if (!Number.isSafeInteger(major) || !Number.isSafeInteger(minor)) return "invalid";
  if (major === 2 && minor === 5) return "validate";
  return major > 2 || major === 2 && minor > 5 ? "use-archive" : "invalid";
}

function requirements(source: string): Requirement[] {
  const baselines = [...source.matchAll(/\bKnots\s+`?(\d[\w.-]*)`?/g)];
  if (activeControlDisposition(source, "requirements") !== "validate" || !baselines.length
    || baselines.some(match => match[1] !== "29.3.knots20260210")) throw new Error("unique v2.5 identity/baseline");
  const boxes = [...source.matchAll(/^- \[([ xX])\] \*\*(CF[A-Z]+-\d+)\*\*:\s*(.+)$/gm)];
  const rows = [...source.matchAll(/^\|\s*(CF[A-Z]+-\d+)\s*\|\s*Phase (\d+)\s*\|\s*(Pending|Complete)\s*\|\s*$/gm)];
  if (boxes.length !== 22 || rows.length !== 22 || new Set(boxes.map(row => row[2])).size !== 22 || new Set(rows.map(row => row[1])).size !== 22) throw new Error("requirement checkbox/traceability consistency: 22 unique IDs/owners");
  return IDS.map(id => {
    const maybeBox = boxes.find(row => row[2] === id);
    const maybeRow = rows.find(row => row[1] === id);
    if (!maybeBox || !maybeRow || (maybeBox[1] === " ") !== (maybeRow[3] === "Pending")) throw new Error(`${id} requirement checkbox/traceability consistency`);
    const owner = Number(maybeRow[2]);
    const expected = id.startsWith("CFIL") ? 154 : ["CFIX-02", "CFIX-04", "CFPR-03"].includes(id) ? 155
      : id === "CFPR-01" ? 156 : ["CFAC-01", "CFAC-02", "CFIX-01"].includes(id) ? 157 : id === "CFIX-03" ? 158
      : id.startsWith("CFRP") ? 159 : id.startsWith("CFNET") ? 160 : id === "CFOP-01" ? 161 : 162;
    if (owner !== expected && !(id === "CFGR-01" && owner === 163)) throw new Error(`${id} wrong owning phase`);
    if (!["CFPR-02", "CFGR-01", "CFGR-02"].includes(id) && maybeRow[3] !== "Complete") throw new Error(`${id} incomplete historical requirement`);
    return { id, description: maybeBox[3].trim(), owner, status: maybeRow[3] };
  });
}

function roadmap(source: string): string[] {
  if (activeControlDisposition(source, "roadmap") !== "validate") throw new Error("unique v2.5 roadmap identity");
  return source.split(/\r?\n/).filter(line => /^### Phase \d+:|^\*\*(?:Requirements|Depends on|Gap Closure)(?::\*\*|\*\*:)|^- \[[ xX]\] \*\*Phase \d+:/.test(line)).map(line => line.trim());
}

function validateRoadmapOwners(source: string, rows: Requirement[]): void {
  const owners = new Map<string, number[]>();
  for (const section of source.split(/(?=^### Phase |^## )/m)) {
    const maybePhase = section.match(/^### Phase (\d+):/);
    if (!maybePhase) continue;
    for (const line of section.matchAll(/^\*\*Requirements(?::\*\*|\*\*:)[ \t]*([^\n]*)/gm)) {
      for (const id of line[1].match(/CF[A-Z]+-\d+/g) ?? []) owners.set(id, [...(owners.get(id) ?? []), Number(maybePhase[1])]);
    }
  }
  for (const row of rows) if (JSON.stringify(owners.get(row.id)) !== JSON.stringify([row.owner])) throw new Error(`${row.id} unique roadmap/requirements owner`);
}

function select<T>(sources: ReadonlyMap<string, string>, paths: string[], parse: (source: string) => T, identity: (source: string) => SourceDisposition): { file: string; value: T } {
  const candidates = paths.filter(file => sources.has(file));
  const relevant = candidates.filter(file => {
    if (file === paths[1]) return true;
    const disposition = identity(sources.get(file)!);
    if (disposition === "invalid") throw new Error(`${file}: invalid active milestone identity/baseline`);
    return disposition === "validate";
  });
  if (!relevant.length) throw new Error(`cannot read required evidence: missing v2.5 source ${paths.join(" or ")}`);
  const values = relevant.map(file => ({ file, value: parse(sources.get(file)!) }));
  if (values.some(item => canonical(item.value) !== canonical(values[0].value))) throw new Error(`conflicting v2.5 candidates ${relevant.join(" / ")}`);
  return values[0];
}

function selectAudit(sources: ReadonlyMap<string, string>): { file: string; value: RecordValue } | undefined {
  if (!V25_AUDITS.some(file => sources.has(file))) return undefined;
  return select(sources, V25_AUDITS, source => {
    const value = provenanceMetadata(source);
    if (value.milestone !== "v2.5" || value.generated_by !== "gsd-audit-milestone" || !["passed", "gaps_found"].includes(String(value.status))) throw new Error("invalid v2.5 milestone audit");
    return value;
  }, () => "validate");
}

function validateGap(sources: ReadonlyMap<string, string>, roadmapSource: string, currentAudit: RecordValue): RecordValue {
  for (const [index, file] of ORIGINAL_AUDITS.entries()) if (hash(sources.get(file) ?? "") !== ORIGINAL_HASHES[index]) throw new Error(`original audit lineage ${file}`);
  const audit = provenanceMetadata(sources.get(ORIGINAL_AUDITS[0])!);
  const gaps = audit.gaps;
  if (audit.status !== "gaps_found" || audit.milestone !== "v2.5" || !isRecord(gaps)
    || !Array.isArray(gaps.requirements) || gaps.requirements.length !== 1 || !isRecord(gaps.requirements[0])
    || gaps.requirements[0].id !== "CFGR-01" || gaps.requirements[0].phase !== 162 || gaps.requirements[0].status !== "partial"
    || !Array.isArray(gaps.integration) || !gaps.integration.some(gap => isRecord(gap) && gap.id === "AG01" && Array.isArray(gap.requirements) && gap.requirements.includes("CFGR-01"))
    || !Array.isArray(gaps.flows) || !gaps.flows.some(gap => isRecord(gap) && gap.id === "AF01" && Array.isArray(gap.requirements) && gap.requirements.includes("CFGR-01"))) throw new Error("original CFGR-01/AG01/AF01 gap lineage");
  if (currentAudit.status === "gaps_found" && canonical(currentAudit.gaps) !== canonical(gaps)) throw new Error("current audit CFGR-01/Phase162/AG01/AF01 gap linkage");
  const scope = provenanceMetadata(sources.get(PHASE163 + "163-GAP-SCOPE.md") ?? "");
  const context = provenanceMetadata(sources.get(PHASE163 + "163-CONTEXT.md") ?? "");
  const phase = "163-archive-safe-parity-provenance-and-milestone-handoff";
  if (scope.phase !== phase || context.phase !== phase || scope.lifecycle_mode !== "interactive"
    || scope.generated_by !== "gsd-plan-milestone-gaps" || scope.phase_lifecycle_id !== "163-2026-10-12T21-03-37"
    || scope.status !== "ready-for-planning" || JSON.stringify(scope.requirements) !== '["CFGR-01"]'
    || JSON.stringify(scope.audit_gaps) !== '["AG01","AF01"]' || JSON.stringify(scope.depends_on) !== "[162]"
    || scope.audit_path !== V25_AUDITS[0] || scope.source_commit !== audit.source_commit
    || context.generated_by !== "gsd-discuss-phase" || context.status !== "approved-scope" || context.lifecycle_mode !== "yolo"
    || context.phase_lifecycle_id !== "163-2026-10-12T21-09-41" || context.scope_path !== PHASE163 + "163-GAP-SCOPE.md"
    || JSON.stringify(context.requirements) !== '["CFGR-01"]') throw new Error("approved Phase163 scope/current lifecycle");
  if (timestamp(scope.generated_at) !== Date.parse("2026-10-12T21:03:37Z")
    || timestamp(context.generated_at) < Date.parse("2026-10-12T21:09:41Z")) throw new Error("scope/context timestamp precedes approved lifecycle");
  const sections = roadmapSource.split(/(?=^### Phase |^## )/m);
  const matching = sections.filter(section => /^### Phase 163: /m.test(section));
  if (matching.length !== 1 || !/^\*\*Requirements\*\*: CFGR-01\s*$/m.test(matching[0])
    || !/^\*\*Depends on\*\*: Phase 162\s*$/m.test(matching[0]) || !/^\*\*Gap Closure\*\*:.*AG01.*AF01/m.test(matching[0])) throw new Error("unique Phase163 roadmap scope/dependency");
  return context;
}

function freshReport(sources: ReadonlyMap<string, string>, name: string, status: string, generator: string, context: RecordValue): RecordValue {
  const file = PHASE163 + `163-${name}.md`;
  const report = provenanceMetadata(sources.get(file) ?? "");
  const generated = timestamp(report.generated_at);
  if (report.status !== status || report.generated_by !== generator || report.lifecycle_mode !== context.lifecycle_mode
    || report.phase_lifecycle_id !== context.phase_lifecycle_id || report.source_digest !== provenanceDigest(sources)
    || PROVENANCE_DIGEST_FILES.some(file => !sources.get(file))
    || generated < timestamp(context.generated_at)) throw new Error(`fresh Phase163 ${name} gate/lifecycle/digest`);
  return report;
}

function validateClosure(sources: ReadonlyMap<string, string>, context: RecordValue): RecordValue {
  const verification = freshReport(sources, "VERIFICATION", "passed", "gsd-verifier", context);
  const functional = verification.verification_scope === "functional-closure";
  const stage = functional ? "in_progress" : "complete";
  const summary = freshReport(sources, "04-SUMMARY", stage, "gsd-execute-plan", context);
  if (summary.execution_status !== stage || JSON.stringify(summary["requirements-completed"]) !== '["CFGR-01"]') throw new Error("Phase163 SUMMARY activation/execution stage");
  if (verification.lifecycle_validated !== true || JSON.stringify(verification.requirements) !== '["CFGR-01"]'
    || JSON.stringify(verification.success_criteria) !== (functional ? '["SC1","SC2","SC3","SC4"]' : '["SC1","SC2","SC3","SC4","SC5"]') || verification.readiness_gate !== "enforced"
    || !["functional-closure", "phase"].includes(String(verification.verification_scope))) throw new Error("canonical Phase163 CFGR-01 verification scope/coverage");
  if (functional && (verification.audit_status !== "pending" || verification.readiness !== "not_asserted"
    || verification.plan04_status !== "in_progress" || verification.phase_status !== "in_progress")) throw new Error("functional-closure must leave audit/readiness/Phase163 pending");
  if (!functional && (verification.audit_status !== "passed" || verification.readiness !== "established"
    || verification.plan04_status !== "complete" || verification.phase_status !== "complete")) throw new Error("final phase scope requires audit/readiness/completion");
  freshReport(sources, "REVIEW", "clean", "gsd-code-reviewer", context);
  const security = freshReport(sources, "SECURITY", "verified", "gsd-security-auditor", context);
  const native = freshReport(sources, "NATIVE-EVIDENCE", "passed", "gsd-execute-phase", context);
  if (security.threats_open !== 0 || native.exit_code !== 0 || !/^bash scripts\/verify\.sh[ \t]*$/m.test(sources.get(PHASE163 + "163-NATIVE-EVIDENCE.md")!)) throw new Error("Phase163 actual full native/security gate");
  return verification;
}

function auditPassed(audit: RecordValue, verification: RecordValue, sources: ReadonlyMap<string, string>): boolean {
  const gaps = audit.gaps;
  const scores = audit.scores;
  if (audit.status !== "passed" || !isRecord(scores) || scores.requirements !== "22/22" || !isRecord(gaps)
    || !["requirements", "integration", "flows"].every(key => Array.isArray(gaps[key]) && gaps[key].length === 0)
    || audit.source_digest !== provenanceDigest(sources) || audit.phase_lifecycle_id !== verification.phase_lifecycle_id
    || !Number.isFinite(Date.parse(String(audit.audited)))
    || Date.parse(String(audit.audited)) < timestamp(provenanceMetadata(sources.get(PHASE163 + "163-CONTEXT.md")!).generated_at)
    || Date.parse(String(audit.audited)) < timestamp(verification.generated_at) && verification.verification_scope !== "phase") throw new Error("renewed passing v2.5 audit/22 satisfied/zero blocking gaps");
  return true;
}

/** Resolve a finite milestone universe using supplied strings only; fail closed on ambiguity. */
export function resolveV25Provenance(sources: ReadonlyMap<string, string>, failures: string[]): V25Provenance | undefined {
  try {
    const selected = select(sources, V25_REQUIREMENTS, requirements, source => activeControlDisposition(source, "requirements"));
    const control = select(sources, V25_ROADMAPS, roadmap, source => activeControlDisposition(source, "roadmap"));
    validateRoadmapOwners(sources.get(control.file)!, selected.value);
    const scoped = ["CFPR-02", "CFGR-01", "CFGR-02"].map(id => selected.value.find(row => row.id === id)!);
    const gap = scoped[1].owner === 163;
    if (scoped.some(row => row.owner !== (gap && row.id === "CFGR-01" ? 163 : 162))) throw new Error("scoped unique Phase162/Phase163 ownership");
    const complete = scoped.every(row => row.status === "Complete");
    if (!gap && new Set(scoped.map(row => row.status)).size !== 1 || gap && [scoped[0], scoped[2]].some(row => row.status !== "Complete")) throw new Error("historical requirement checkbox/traceability consistency");
    const proof = sources.get(PROOF) ?? "";
    const historical = gap || complete ? "complete" : proof ? "implementation" : "pending";
    const maybeAudit = selectAudit(sources);
    let publication: V25Provenance["publication"] = maybeAudit?.value.status === "gaps_found" ? "gaps_found" : "unaudited";
    let closure: V25Provenance["closure"] = "original";
    if (gap) {
      closure = complete ? "gap_complete" : "gap_pending";
      if (!maybeAudit) throw new Error("missing current v2.5 gap audit");
      const context = validateGap(sources, sources.get(control.file)!, maybeAudit.value);
      if (!complete && maybeAudit.value.status !== "gaps_found") throw new Error("Pending gap requires gaps_found audit");
      if (complete) {
        const verification = validateClosure(sources, context);
        if (maybeAudit.value.status === "passed" && auditPassed(maybeAudit.value, verification, sources)) {
          publication = "audit_passed";
        } else if (verification.verification_scope === "phase") throw new Error("final phase publication requires renewed passing audit");
      }
    } else if (maybeAudit?.value.status === "passed") throw new Error("archive readiness requires new-owner closure");
    const archived = [selected.file, control.file, maybeAudit?.file].every(file => file?.startsWith(".planning/milestones/"));
    if (archived) {
      if (publication !== "audit_passed" || provenanceMetadata(sources.get(PHASE163 + "163-VERIFICATION.md") ?? "").verification_scope !== "phase") throw new Error("archived controls require final phase closure/passing audit");
      publication = "archived";
    }
    return { historical, closure, publication, requirementsPath: selected.file, roadmapPath: control.file, maybeAuditPath: maybeAudit?.file };
  } catch (error) {
    failures.push(`T-162-10: v2.5 provenance: ${(error instanceof Error ? error.message : String(error)).slice(0, 300)}`);
    return undefined;
  }
}

/** Exempt only exact sentences authorized by the independently resolved state. */
export function provenanceClaimView(source: string, maybeState: V25Provenance | undefined): string {
  if (!maybeState) return source;
  let view = source;
  if (maybeState.closure === "gap_pending" && maybeState.publication === "gaps_found") view = view.replaceAll(GAP_CLAIM, "");
  if (maybeState.closure === "gap_complete" && maybeState.publication !== "archived") view = view.replaceAll(CLOSURE_CLAIM, "");
  if (maybeState.publication === "audit_passed") view = view.replaceAll(READINESS_CLAIM, "");
  if (maybeState.publication === "archived") view = view.replaceAll(ARCHIVED_CLAIM, "");
  return view;
}
