#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const ACTIVE_TEST = "bun test scripts/check-active-milestone-verification-traceability.test.ts";
const ACTIVE_CHECK = "bun run scripts/check-active-milestone-verification-traceability.ts";
const PHASE126_TEST = "bun test scripts/check-phase126-compact-relay-residual-hardening.test.ts";
const PHASE126_CHECK = "bun run scripts/check-phase126-compact-relay-residual-hardening.ts";
const PHASE117_TEST = "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const PHASE117_CHECK = "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
const ARCHIVED_V21_ROADMAP = ".planning/milestones/v2.1-ROADMAP.md";

const TARGET_FILES = [
  "packages/open-bitcoin-network/src/peer/message_dispatch.rs",
  "packages/open-bitcoin-network/src/peer/compact_download_state.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/announcement_transport.rs",
  "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
  "packages/open-bitcoin-node/src/network/runtime_authority/effects.rs",
  "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs",
  "packages/open-bitcoin-node/Cargo.toml",
  "packages/open-bitcoin-node/BUILD.bazel",
  ".planning/ROADMAP.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-phase126-compact-relay-residual-hardening.ts",
  "scripts/verify.sh",
] as const;

const COMPACT_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/blockencodings.h",
  "packages/bitcoin-knots/src/blockencodings.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/net_processing.h",
  "packages/bitcoin-knots/test/functional/p2p_compactblocks.py",
] as const;

const COMPLETED_PHASE126_CATALOG_LIFECYCLE =
  "Phase 126 remains locally complete at 4/4 plans. The canonical v2.1 integration audit reports 29/39 requirements complete and routes gap closure through Phases 127–129 before any fresh archive decision.";

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type ParitySurface = {
  evidence?: unknown;
  id?: unknown;
  rationale?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type ParityIndex = { checklist?: { surfaces?: unknown } };
type BreadcrumbGroup = { breadcrumbs?: unknown; files?: unknown; label?: unknown };

export function checkPhase126CompactRelayResidualHardening(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE126_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = loadCorpus(repoRoot, failures);
  checkFactInjection(texts, failures);
  checkNonceAndEvidence(texts, failures);
  checkDependencies(texts, failures);
  checkParityAnchors(texts, failures);
  checkCompletedPhase126CatalogLifecycle(texts, failures);
  checkVerifier(texts, failures);
  return failures;
}

function loadCorpus(repoRoot: string, failures: string[]): TextCorpus {
  const texts = new Map<TargetFile, string>();
  for (const file of TARGET_FILES) {
    const sourceFile =
      file === ".planning/ROADMAP.md" &&
      existsSync(path.join(repoRoot, ARCHIVED_V21_ROADMAP))
        ? ARCHIVED_V21_ROADMAP
        : file;
    const absolutePath = path.join(repoRoot, sourceFile);
    if (!existsSync(absolutePath)) {
      failures.push(`P126 missing target: ${file}`);
      texts.set(file, "");
      continue;
    }
    texts.set(file, readFileSync(absolutePath, "utf8"));
  }
  return texts;
}

function checkFactInjection(texts: TextCorpus, failures: string[]): void {
  const dispatch =
    texts.get("packages/open-bitcoin-network/src/peer/message_dispatch.rs") ?? "";
  if (
    !dispatch.includes("WireNetworkMessage::CompactBlock(_) => {") ||
    !dispatch.includes("Err(NetworkError::CompactBlockReceiveFactsRequired)")
  ) {
    failures.push(
      "P126 factless compact dispatch: generic dispatch must return CompactBlockReceiveFactsRequired",
    );
  }

  const downloadState =
    texts.get("packages/open-bitcoin-network/src/peer/compact_download_state.rs") ?? "";
  const maybeDerive = downloadState.match(
    /#\[derive\(([^)]*)\)\]\s*pub struct CompactBlockReceiveFacts/,
  )?.[1];
  if (maybeDerive?.split(",").some((item) => item.trim() === "Default")) {
    failures.push("P126 receive facts default: CompactBlockReceiveFacts must not implement Default");
  }
  if (downloadState.includes("impl Default for CompactBlockReceiveFacts")) {
    failures.push("P126 receive facts default: CompactBlockReceiveFacts must not implement Default");
  }

  const network = texts.get("packages/open-bitcoin-node/src/network.rs") ?? "";
  const managedCall = "self.handle_compact_block_receive(peer_id, payload, timestamp)?";
  if (countOccurrences(network, managedCall) !== 2) {
    failures.push(
      "P126 managed receive paths: receive and sync entrypoints must inject live facts",
    );
  }

  const candidates =
    texts.get("packages/open-bitcoin-node/src/network/compact_receive_candidates.rs") ?? "";
  const snapshotAnchors = [
    "mempool_compact_candidate_owned(self.mempool.mempool())",
    "compact_extra_owned(&self.compact_extra_txn)",
    "candidates: &candidate_refs,",
    "extra: &extra_refs,",
    ".handle_compact_block_download(peer_id, payload, facts, timestamp)",
  ];
  if (!snapshotAnchors.every((anchor) => candidates.includes(anchor))) {
    failures.push(
      "P126 live receive snapshots: compact facts must carry mempool and bounded extras",
    );
  }
}

function checkNonceAndEvidence(texts: TextCorpus, failures: string[]): void {
  const network = texts.get("packages/open-bitcoin-node/src/network.rs") ?? "";
  const announceStart = network.indexOf("pub fn announce_block(");
  const helperEnd = network.indexOf("pub fn announce_transaction(", announceStart);
  const announceSection =
    announceStart === -1 || helperEnd === -1 ? "" : network.slice(announceStart, helperEnd);
  if (
    !orderedFragments(announceSection, [
      "let announcement = self.peer_manager.decide_compact_announcement_for_peer(",
      "self.announce_block_with_nonce(peer_id, block, announcement, || {",
      "getrandom::fill(&mut nonce_bytes)?;",
      "Ok::<u64, getrandom::Error>(u64::from_le_bytes(nonce_bytes))",
    ])
  ) {
    failures.push(
      "P126 lazy fallible nonce: compact selection must acquire getrandom entropy in the node shell",
    );
  }
  if (
    /nonce_bytes\.(?:copy_from_slice|clone_from_slice)[\s\S]{0,160}block_hash/.test(
      announceSection,
    ) ||
    /block_hash[\s\S]{0,160}nonce_bytes\.(?:copy_from_slice|clone_from_slice)/.test(
      announceSection,
    )
  ) {
    failures.push("P126 nonce provenance: compact nonce must not derive from block-hash bytes");
  }
  if (!announceSection.includes("Err(_) => self.peer_manager.announce_block(peer_id, block)?,")) {
    failures.push(
      "P126 entropy failure emission: entropy failure must fall back without cmpctblock",
    );
  }
  const transport =
    texts.get("packages/open-bitcoin-node/src/network/announcement_transport.rs") ?? "";
  if (
    !orderedFragments(transport, [
      "let Ok(Some(message)) = maybe_message else {",
      "self.prepare_peer_emission(peer_id, message, block_hash)",
      "AnnouncementPreparationOutcome::Ready(Box::new(emission))",
    ]) ||
    network.includes(".record_compact_block_announcement(") ||
    transport.includes(".record_compact_block_announcement(")
  ) {
    failures.push(
      "P126 compact provenance: bound emission must require an actual wire message",
    );
  }

  const evidence =
    texts.get("packages/open-bitcoin-node/src/network/block_relay_evidence.rs") ?? "";
  const completion =
    texts.get("packages/open-bitcoin-node/src/network/runtime_authority/effects.rs") ?? "";
  if (
    !orderedFragments(evidence, [
      "fn record_peer_emission(",
      ".record_compact_block_announcement(peer_id, evidence.block_hash())?;",
      ".record_announcement(evidence.evidence_reason());",
    ]) ||
    !orderedFragments(completion, [
      "pub fn complete_peer_emission(",
      ".apply_lifecycle_command(LifecycleCommand::CompletePeerEmission(receipt))",
    ]) ||
    !completion.includes(
      "LifecycleCommandResult::PeerEffectCompleted(completion) => Ok(completion),",
    ) ||
    completion.includes("try_mutate")
  ) {
    failures.push(
      "P126 achieved effect evidence: consuming completion must derive from the written emission",
    );
  }
}

function checkDependencies(texts: TextCorpus, failures: string[]): void {
  const cargo = texts.get("packages/open-bitcoin-node/Cargo.toml") ?? "";
  if (!/^getrandom\s*=\s*"[^"]+"/m.test(cargo)) {
    failures.push("P126 Cargo entropy dependency: open-bitcoin-node must declare getrandom");
  }
  const bazel = texts.get("packages/open-bitcoin-node/BUILD.bazel") ?? "";
  if (!bazel.includes('"@crate_index//:getrandom",')) {
    failures.push("P126 Bazel entropy dependency: open-bitcoin-node must depend on getrandom");
  }
}

function checkParityAnchors(texts: TextCorpus, failures: string[]): void {
  const maybeIndex = parseJson<ParityIndex>(
    texts.get("docs/parity/index.json") ?? "",
    "P126 parity anchors: compact relay surfaces must retain exact Knots anchors",
    failures,
  );
  const surfaces = asObjectArray<ParitySurface>(maybeIndex?.checklist?.surfaces);
  const requiredSurfaces = [
    "v2-1-compact-relay-negotiation-announcement-policy",
    "v2-1-compact-block-reconstruction",
  ];
  for (const surfaceId of requiredSurfaces) {
    const maybeSurface = surfaces.find((surface) => surface.id === surfaceId);
    const anchors = new Set([
      ...asStringArray(maybeSurface?.upstream?.sources),
      ...asStringArray(maybeSurface?.upstream?.tests),
    ]);
    if (!maybeSurface || !COMPACT_KNOTS_ANCHORS.every((anchor) => anchors.has(anchor))) {
      pushUnique(
        failures,
        "P126 parity anchors: compact relay surfaces must retain exact Knots anchors",
      );
    }
  }

  const maybeBreadcrumbs = parseJson<{ groups?: unknown }>(
    texts.get("docs/parity/source-breadcrumbs.json") ?? "",
    "P126 breadcrumb anchors: compact download group must retain exact Knots anchors",
    failures,
  );
  const groups = asObjectArray<BreadcrumbGroup>(maybeBreadcrumbs?.groups);
  const maybeGroup = groups.find((group) => group.label === "network-compact-block-download");
  const breadcrumbAnchors = new Set(asStringArray(maybeGroup?.breadcrumbs));
  const sourceFiles = asStringArray(maybeGroup?.files);
  if (
    !maybeGroup ||
    !COMPACT_KNOTS_ANCHORS.every((anchor) => breadcrumbAnchors.has(anchor)) ||
    !sourceFiles.includes("packages/open-bitcoin-network/src/peer/message_dispatch.rs") ||
    !sourceFiles.includes("packages/open-bitcoin-network/src/peer/compact_download_state.rs")
  ) {
    pushUnique(
      failures,
      "P126 breadcrumb anchors: compact download group must retain exact Knots anchors",
    );
  }
}

function checkCompletedPhase126CatalogLifecycle(
  texts: TextCorpus,
  failures: string[],
): void {
  const roadmap = texts.get(".planning/ROADMAP.md") ?? "";
  if (
    !roadmap.includes("- [x] **Phase 126: Compact Relay Residual Hardening**") ||
    !phaseSection(roadmap, 126).includes("**Plans:** 4/4 plans complete")
  ) {
    return;
  }

  checkCompletedPhase126Catalog(
    texts.get("docs/parity/catalog/mempool-policy.md") ?? "",
    [
      "The Phase 126 runtime candidate",
      "All Phase 126 requirements remain pending",
    ],
    "P126 completed Phase 126 mempool catalog lifecycle must retain the current post-audit projection",
    failures,
  );
  checkCompletedPhase126Catalog(
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    [
      "The Phase 126 runtime candidate",
      "This is candidate evidence only",
      "all six Phase 126 requirements remain pending",
    ],
    "P126 completed Phase 126 P2P catalog lifecycle must retain the current post-audit projection",
    failures,
  );
}

function checkCompletedPhase126Catalog(
  catalog: string,
  forbiddenLifecycleClaims: readonly string[],
  failure: string,
  failures: string[],
): void {
  const normalizedCatalog = catalog.replace(/\s+/g, " ").trim();
  const hasCompletedLifecycle =
    countOccurrences(normalizedCatalog, COMPLETED_PHASE126_CATALOG_LIFECYCLE) === 1;
  const hasStaleLifecycle = forbiddenLifecycleClaims.some((claim) =>
    normalizedCatalog.includes(claim),
  );
  if (!hasCompletedLifecycle || hasStaleLifecycle) {
    failures.push(failure);
  }
}

function phaseSection(roadmap: string, phase: number): string {
  const marker = `#### Phase ${phase}:`;
  const start = roadmap.indexOf(marker);
  if (start === -1) return "";
  const end = roadmap.indexOf("\n#### Phase ", start + marker.length);
  return roadmap.slice(start, end === -1 ? roadmap.length : end);
}

function checkVerifier(texts: TextCorpus, failures: string[]): void {
  const verifyText = texts.get("scripts/verify.sh") ?? "";
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = verifyText.indexOf(marker);
  const bodyStart = start + marker.length;
  const end = verifyText.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  const visible = start === -1 || end === -1 ? "" : verifyText.slice(bodyStart, end);
  if (
    !orderedLines(visible, [
      ACTIVE_TEST,
      ACTIVE_CHECK,
      PHASE126_TEST,
      PHASE126_CHECK,
      PHASE117_TEST,
      PHASE117_CHECK,
    ])
  ) {
    failures.push(
      "P126 verifier visible order: Phase 126 must follow active traceability and precede Phase 117",
    );
  }
  const runSteps = [
    `run_step "test active milestone verification traceability checker" ${ACTIVE_TEST}`,
    `run_step "check active milestone verification traceability" ${ACTIVE_CHECK}`,
    `run_step "test Phase 126 compact relay residual hardening checker" ${PHASE126_TEST}`,
    `run_step "check Phase 126 compact relay residual hardening" ${PHASE126_CHECK}`,
    `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`,
    `run_step "check Phase 117 parity UAT release boundary" ${PHASE117_CHECK}`,
  ];
  if (!orderedLines(verifyText, runSteps)) {
    failures.push(
      "P126 verifier executable order: Phase 126 must follow active traceability and precede Phase 117",
    );
  }

  const checker = texts.get("scripts/check-phase126-compact-relay-residual-hardening.ts") ?? "";
  const forbiddenTokens = [
    "fetch" + "(",
    "Bun." + "spawn",
    "node:" + "child_process",
    "http" + "://",
    "https" + "://",
  ];
  if (forbiddenTokens.some((token) => checker.includes(token))) {
    failures.push("P126 deterministic scope: checker must remain local and deterministic");
  }
}

function parseJson<T>(raw: string, failure: string, failures: string[]): T | null {
  try {
    return JSON.parse(raw) as T;
  } catch {
    pushUnique(failures, failure);
    return null;
  }
}

function asObjectArray<T>(value: unknown): T[] {
  return Array.isArray(value)
    ? value.filter((item): item is T => typeof item === "object" && item !== null)
    : [];
}

function asStringArray(value: unknown): string[] {
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
}

function countOccurrences(text: string, needle: string): number {
  return needle.length === 0 ? 0 : text.split(needle).length - 1;
}

function orderedFragments(text: string, fragments: readonly string[]): boolean {
  let cursor = -1;
  for (const fragment of fragments) {
    const index = text.indexOf(fragment, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

function orderedLines(text: string, requiredLines: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const required of requiredLines) {
    const index = lines.indexOf(required, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

function pushUnique(failures: string[], failure: string): void {
  if (!failures.includes(failure)) failures.push(failure);
}

if (import.meta.main) {
  const failures = checkPhase126CompactRelayResidualHardening();
  if (failures.length > 0) {
    console.error("Phase 126 compact relay residual hardening check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 126 compact relay residual hardening validated.");
}
