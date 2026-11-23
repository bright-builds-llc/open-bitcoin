#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { lstatSync, readFileSync } from "node:fs";
import { posix, resolve } from "node:path";
import { checkCurrentParityLedger } from "./check-parity-ledger.ts";

const ROOT = resolve(import.meta.dir, "..");
const INDEX = "docs/parity/index.json";
const REQUIREMENTS = "docs/parity/requirements.md";
const KNOTS_ROOT = "packages/bitcoin-knots/";
const KNOTS_COMMIT = "a9aee730466ac67d35a3c03ee24676be5e045878";
const ARCHIVE_COMMIT = "5b0b2acfacc8ad47ceeb926c469da05a7712aa57";
const PHASE162 = ".planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-";

// SHA-256 of files at the v2.5 archive commit. Validation also works in shallow checkouts.
const V25_ARCHIVE = {
  ".planning/milestones/v2.5-REQUIREMENTS.md": "5d6c6e8b8ae21eb75a71f7667f9f9f730f85c0ddad0fcdf3232e244e303d879b",
  ".planning/milestones/v2.5-ROADMAP.md": "8882c4390683725de94d46aabff95d56b86b4e48e8934886d787d03c246ff567",
  ".planning/milestones/v2.5-MILESTONE-AUDIT.md": "42d3178670cd9298456e6a640b0324e4f84f528ef316aa9104ccb8110b875f78",
  ".planning/milestones/v2.5-INTEGRATION-AUDIT.md": "e2ceb009abea04309a1894954518d86374767f0fedfadbd3d40da4fe80be2155",
  [`${PHASE162}NATIVE-EVIDENCE.md`]: "d0f469ef09859499a518c32c7f89ec55c6e64a34e7596d5278a6d2086b93072b",
  [`${PHASE162}UAT.md`]: "1b5a6cff6a896e48796695a6f1de4b2053d056ed34f295bb8528c9352e303233",
  [`${PHASE162}VERIFICATION.md`]: "1ddde66d94dfe4bd47151a2c7a152980d26cfc121774f730ff418fd93a6af56e",
  [`${PHASE162}SECURITY.md`]: "c492e0707fa05246ff5cfae47aa70f6107cae205c63ac0ee67692dae0f4b3afa",
  ".planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-HANDOFF.md": "01e46674450f76015e3a83687864b2308e8965a32ae733079cad2aa2fcb812c3",
  "docs/parity/v2-5-integrated-filter-proof.md": "91205d4109f6c9d5db3ec079b14d22412675811049b6e087708b08404131272a",
};

const REQUIRED_DIFFERENCES = new Set([
  "canonical-ordinary-history-refused-pending-prerequisites", "v2-6-exact-work-invalid-target-errors",
  "v2-6-bip30-lossless-overwrite-undo", "v2-6-canonical-paired-prune-storage",
  "v2-6-manual-prune-keep-window-refusal", "v2-6-archive-backed-pruned-reopen",
]);

type Surface = {
  id: string; status: string; requirements: string[]; evidence: string[]; evidence_classes: string[];
  upstream?: { sources: string[] }; known_gaps?: string[];
  differential_receipt?: string; knots_binary_sha256?: string; recovery_receipt?: string;
};
type Deviation = {
  id: string; scope?: string; surfaces: string[]; summary: string; evidence: string[];
  upstream?: { sources: string[] };
};
export type TraceIndex = {
  baseline: string; checklist: { surfaces: Surface[] }; deviations: Deviation[];
};
type ReadBytes = (file: string) => Buffer;
const digest = (bytes: Buffer) => createHash("sha256").update(bytes).digest("hex");

function requireTrace(condition: unknown, message: string): asserts condition {
  if (!condition) throw new Error(message);
}

function safePath(file: string): boolean {
  return file.length > 0 && !file.includes("\\") && !posix.isAbsolute(file)
    && !file.split("/").some(part => part === "." || part === ".." || part === "");
}

function checkoutReader(root: string): ReadBytes {
  return file => {
    requireTrace(safePath(file), `Unsafe evidence path: ${file}`);
    const path = resolve(root, file);
    const info = lstatSync(path);
    requireTrace(info.isFile() && info.size <= 4 * 1024 * 1024, `Expected bounded regular file: ${file}`);
    const bytes = readFileSync(path);
    requireTrace(bytes.length === info.size, `Evidence changed while reading: ${file}`);
    return bytes;
  };
}

function archiveTrace(read: ReadBytes) {
  const files = Object.entries(V25_ARCHIVE).map(([file, sha256]) => {
    requireTrace(digest(read(file)) === sha256, `v2.5 archive bytes changed: ${file}`);
    return { file, sha256 };
  });
  return { commit: ARCHIVE_COMMIT, files };
}

function receiptTrace(surfaces: Surface[], read: ReadBytes) {
  const records = surfaces.filter(surface => surface.evidence_classes.includes("knots_daemon_differential")).map(surface => {
    const file = surface.differential_receipt;
    requireTrace(surface.status === "done" && file && surface.evidence.includes(file), `Unattributed differential: ${surface.id}`);
    const bytes = read(file);
    const receipt: { source_commit?: string; binary_sha256?: string; status?: string } = JSON.parse(bytes.toString());
    requireTrace(receipt.status === "passed" && receipt.source_commit === KNOTS_COMMIT
      && receipt.binary_sha256 === surface.knots_binary_sha256 && /^[0-9a-f]{64}$/.test(receipt.binary_sha256 ?? ""),
    `Invalid observed result: ${surface.id}`);
    return { owner: surface.id, file, sha256: digest(bytes), binary_sha256: receipt.binary_sha256 };
  });
  for (const owner of ["cdif-01", "cdif-02", "cdif-03", "cdif-04", "cdif-05", "crec-01", "crec-02"]) {
    requireTrace(records.some(record => record.owner === `v2-6-${owner}`), `Missing actual Knots result: ${owner}`);
  }
  return records;
}

function deviationTrace(index: TraceIndex, owners: Map<string, Surface>, read: ReadBytes) {
  const remaining = new Set(REQUIRED_DIFFERENCES);
  const seen = new Set<string>();
  const differences = index.deviations.filter(row => row.scope === "v2.6").map(row => {
    requireTrace(!seen.has(row.id), `Duplicate v2.6 difference: ${row.id}`);
    seen.add(row.id);
    remaining.delete(row.id);
    const related = row.surfaces.filter(id => id.startsWith("v2-6-"));
    requireTrace(related.length > 0 && related.every(id => owners.get(id)?.status === "done"), `Difference needs completed owners: ${row.id}`);
    const sources = row.upstream?.sources;
    requireTrace(sources?.length && sources.every(source => safePath(source) && source.startsWith(KNOTS_ROOT)), `Difference needs pinned Knots sources: ${row.id}`);
    requireTrace(row.evidence.length > 0 && row.summary.trim().length > 0, `Difference needs evidence and a description: ${row.id}`);
    requireTrace(related.every(id => owners.get(id)?.evidence.some(file => row.evidence.includes(file))), `Difference evidence is disconnected from its owners: ${row.id}`);
    for (const file of row.evidence) read(file);
    return { id: row.id, owners: related, summary: row.summary, sources, evidence: row.evidence };
  });
  requireTrace(remaining.size === 0, `Missing v2.6 differences: ${[...remaining].join(", ")}`);
  return differences;
}

/** Project a structurally verified ledger, keeping observed results and archived provenance distinct. */
export function buildTrace(index: TraceIndex, requirementsText: string, read: ReadBytes) {
  const surfaces = index.checklist.surfaces.filter(row => row.id.startsWith("v2-6-"));
  const owners = new Map(surfaces.map(surface => [surface.id, surface]));
  const claims = [...requirementsText.matchAll(/^- \*\*([A-Z]+-\d{2})\*\*: (.+)$/gm)].map(([, id, wording]) => {
    const matched = surfaces.filter(surface => surface.requirements.includes(id));
    requireTrace(matched.length === 1, `Claim must have one owner: ${id}`);
    const owner = matched[0];
    requireTrace(owner.status === "done" || id === "CPAR-02" && owner.status === "planned", `Claim owner is unfinished: ${id}`);
    return { id, wording, owner: owner.id, status: owner.status };
  });
  requireTrace(claims.length === 30 && new Set(claims.map(claim => claim.id)).size === 30, "Expected 30 approved v2.6 claims");
  const differential = receiptTrace(surfaces, read);
  const differences = deviationTrace(index, owners, read).map(row => ({ ...row,
    actual_results: differential.filter(receipt => row.evidence.includes(receipt.file)).map(receipt => receipt.file),
  }));
  return {
    schema: 1, baseline: { tag: index.baseline, commit: KNOTS_COMMIT }, claims, surfaces, differences, differential,
    reference_ambiguities: [{
      owner: "v2-6-crec-01", case: "Knots equal-work and equal-sequence restart fallback",
      receipt: differential.find(receipt => receipt.owner === "v2-6-crec-01")?.file,
    }],
    v2_5_archive: archiveTrace(read),
  };
}

export function checkCurrentTrace(root = ROOT) {
  const ledgerFailures = checkCurrentParityLedger(root);
  requireTrace(ledgerFailures.length === 0, ledgerFailures.join("\n"));
  const read = checkoutReader(root);
  const index: TraceIndex = JSON.parse(read(INDEX).toString());
  const trace = buildTrace(index, read(REQUIREMENTS).toString(), read);
  const anchors = new Set([
    ...trace.surfaces.flatMap(surface => surface.upstream?.sources ?? []),
    ...trace.differences.flatMap(row => row.sources),
  ]);
  const sources = [...anchors].sort().map(file => {
    requireTrace(safePath(file) && file.startsWith(KNOTS_ROOT), `Invalid source anchor: ${file}`);
    const upstream = execFileSync("git", ["-C", resolve(root, KNOTS_ROOT), "show", `${KNOTS_COMMIT}:${file.slice(KNOTS_ROOT.length)}`], { maxBuffer: 4 * 1024 * 1024 });
    const current = read(file);
    requireTrace(upstream.equals(current), `Source anchor differs from the pin: ${file}`);
    return { file, sha256: digest(upstream) };
  });
  return { ...trace, sources };
}

if (import.meta.main) {
  try {
    const argument = Bun.argv[2];
    if (Bun.argv.length !== 3 || (argument !== "--check" && argument !== "--print")) {
      throw new Error("Usage: bun run scripts/check-v2-6-traceability.ts --check|--print");
    }
    const trace = checkCurrentTrace();
    if (argument === "--print") console.log(JSON.stringify(trace, null, 2));
    else console.log(`CPAR-01 verified ${trace.claims.length} claims, ${trace.differences.length} differences, ${trace.differential.length} actual Knots results, and ${trace.v2_5_archive.files.length} archived v2.5 files`);
  } catch (error) {
    console.error(String(error));
    process.exitCode = 1;
  }
}
