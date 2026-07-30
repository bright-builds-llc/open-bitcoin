#!/usr/bin/env bun

import path from "node:path";

import {
  CONNECTED_BLOCK_ROOT_SYMBOL,
  CONNECTED_BLOCK_SEAMS,
  inspectConnectedBlockRoot,
  inspectConnectedBlockSeam,
  inspectOrdinaryAggregateRoot,
} from "./check-phase134-apply-boundaries/aggregate-roots";
import { readSourceRoot } from "./source-corpus";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");

export const PHASE134_APPLY_BOUNDARY_DIAGNOSTIC =
  "P134 apply boundary: reachable apply helpers must remain classified and mutation-safe";

export const PHASE134_APPLY_TARGET_FILES = [
  "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs",
  "packages/open-bitcoin-node/src/network/inventory.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-node/src/network/lifecycle_projection.rs",
  "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs",
] as const;

export const PHASE134_APPLY_SOURCE_FILES = [
  ...PHASE134_APPLY_TARGET_FILES,
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  "packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs",
  "packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
] as const;

const REQUIRED_TARGETS = [
  "apply_prepared_compact",
  "apply_prepared_serving",
  "apply_prepared_fanout",
  "apply_prepared_peer_lifecycle",
  "apply_prepared_unbroadcast",
  "apply_prepared_persistence",
  "apply_prepared_evidence",
  "apply_prepared_lifecycle",
] as const;

const REQUIRED_ROOT_SYMBOL =
  "ManagedPeerNetwork::commit_sealed_lifecycle" as const;
// The public transaction operation validates instance and revision before its
// exclusive callback, then immediately performs the infallible core apply.
const DEPENDENT_ROOT_SYMBOL =
  "ManagedPeerNetwork::apply_prepared_lifecycle" as const;
const DISCOVERY_EXCLUSIONS = new Set(["validate_prepared_lifecycle"]);
const REMOVED_VALIDATED_API = [
  "ValidatedMempoolTransition",
  "validate_prepared_mempool_transition",
  "apply_validated_mempool_transition",
  "SealedMempoolTransition",
  "seal_prepared_mempool_transition",
  "commit_sealed_mempool_transition",
] as const;

export const ATOMIC_CORE_COMMIT = new Set([
  "Mempool::commit_prepared_mempool_transition_with",
]);

export const INFALLIBLE_APPLY_CALLEES = new Set([
  "ManagedPeerNetwork::apply_prepared_compact",
  "ManagedPeerNetwork::apply_prepared_serving",
  "ManagedPeerNetwork::apply_prepared_fanout",
  "ManagedPeerNetwork::apply_prepared_peer_lifecycle",
  "ManagedPeerNetwork::apply_prepared_unbroadcast",
  "ManagedPeerNetwork::apply_prepared_persistence",
  "ManagedPeerNetwork::apply_prepared_evidence",
  "PeerManager::apply_prepared_transaction_lifecycle",
  "transaction_lifecycle::apply_prepared_orphan_lifecycle",
  "TxDownloadScheduler::forget_lifecycle_identity",
  "TxDownloadScheduler::mark_already_have",
  "TxOrphanage::remove_candidate_cursor",
  "TxOrphanage::remove_orphan_without_candidate_scan",
  "TxOrphanage::record_accepted_package_fingerprint",
  "TxOrphanage::retire_accepted_package_fingerprint",
  "TxOrphanage::remove_child_index",
  "TxOrphanage::decrement_peer_count",
]);

export const PURE_CALL_ALLOWLIST = new Set([
  "PeerTransactionIdentity::relay_ids",
  "BTreeMap::get",
  "BTreeMap::get_mut",
  "BTreeMap::insert",
  "BTreeMap::remove",
  "BTreeSet::insert",
  "BTreeSet::is_empty",
  "BTreeSet::remove",
  "BTreeSet::retain",
  "Option::iter",
  "usize::saturating_sub",
]);

const PURE_RECEIVER_CALLS: ReadonlyMap<string, string> = new Map([
  ["self.known_txids.remove", "BTreeSet::remove"],
  ["self.known_txids.insert", "BTreeSet::insert"],
  ["self.known_wtxids.remove", "BTreeSet::remove"],
  ["self.known_wtxids.insert", "BTreeSet::insert"],
  ["self.mempool_known.remove", "BTreeSet::remove"],
  ["self.mempool_known.insert", "BTreeSet::insert"],
  ["self.already_have.remove", "BTreeSet::remove"],
  ["self.already_have.insert", "BTreeSet::insert"],
  ["self.announcements.remove", "BTreeMap::remove"],
  ["self.in_flight.remove", "BTreeMap::remove"],
  ["self.known_wtxids_by_txid.get", "BTreeMap::get"],
  ["self.known_wtxids_by_txid.remove", "BTreeMap::remove"],
  ["self.known_wtxids_by_txid.insert", "BTreeMap::insert"],
  ["self.compact_download_states.insert", "BTreeMap::insert"],
  ["self.pending_reconsideration.remove", "BTreeSet::remove"],
  ["self.orphans.remove", "BTreeMap::remove"],
  ["self.candidate_cursors.remove", "BTreeMap::remove"],
  ["self.accepted_package_fingerprints.insert", "BTreeMap::insert"],
  ["self.accepted_package_fingerprints.remove", "BTreeMap::remove"],
  ["self.children_by_parent.get_mut", "BTreeMap::get_mut"],
  ["self.children_by_parent.remove", "BTreeMap::remove"],
  ["self.orphan_count_by_peer.get_mut", "BTreeMap::get_mut"],
  ["self.orphan_count_by_peer.remove", "BTreeMap::remove"],
  ["children.retain", "BTreeSet::retain"],
  ["children.is_empty", "BTreeSet::is_empty"],
  ["maybe_entry.iter", "Option::iter"],
  ["count.saturating_sub", "usize::saturating_sub"],
] as const);

type ExtractedFunction = {
  symbol: string;
  name: string;
  owner: string | null;
  signature: string;
  body: string;
};

type ImplRange = {
  owner: string;
  open: number;
  close: number;
};

type MethodCall = {
  receiver: string;
  name: string;
  index: number;
};

function maskCommentsAndStrings(source: string): string {
  let result = "";
  let state: "code" | "line" | "block" | "string" | "char" = "code";
  let escaped = false;
  for (let index = 0; index < source.length; index += 1) {
    const current = source[index] ?? "";
    const next = source[index + 1] ?? "";
    if (state === "line") {
      if (current === "\n") {
        state = "code";
        result += "\n";
      } else {
        result += " ";
      }
      continue;
    }
    if (state === "block") {
      if (current === "*" && next === "/") {
        result += "  ";
        index += 1;
        state = "code";
      } else {
        result += current === "\n" ? "\n" : " ";
      }
      continue;
    }
    if (state === "string" || state === "char") {
      result += current === "\n" ? "\n" : " ";
      if (escaped) {
        escaped = false;
      } else if (current === "\\") {
        escaped = true;
      } else if (
        (state === "string" && current === '"') ||
        (state === "char" && current === "'")
      ) {
        state = "code";
      }
      continue;
    }
    if (current === "/" && next === "/") {
      result += "  ";
      index += 1;
      state = "line";
    } else if (current === "/" && next === "*") {
      result += "  ";
      index += 1;
      state = "block";
    } else if (current === '"') {
      result += " ";
      state = "string";
    } else if (
      current === "'" &&
      !(/[A-Za-z_]/.test(next) && source[index + 2] !== "'")
    ) {
      result += " ";
      state = "char";
    } else {
      result += current;
    }
  }
  return result;
}

function matchingBrace(masked: string, open: number): number {
  return matchingDelimiter(masked, open, "{", "}");
}

function matchingDelimiter(
  masked: string,
  open: number,
  openCharacter: string,
  closeCharacter: string,
): number {
  let depth = 0;
  for (let index = open; index < masked.length; index += 1) {
    if (masked[index] === openCharacter) {
      depth += 1;
    } else if (masked[index] === closeCharacter) {
      depth -= 1;
      if (depth === 0) {
        return index;
      }
    }
  }
  throw new Error(
    `unbalanced Rust source delimiter near ${masked.slice(Math.max(0, open - 80), open + 40)}`,
  );
}

function implRanges(masked: string): ImplRange[] {
  const ranges: ImplRange[] = [];
  const pattern =
    /(?:^|\n)\s*impl(?:\s*<[^{}]*>)?\s+([A-Za-z_][A-Za-z0-9_:]*)(?:\s*<[^{}]*>)?\s*\{/g;
  for (const match of masked.matchAll(pattern)) {
    const owner = match[1];
    const open = (match.index ?? 0) + match[0].lastIndexOf("{");
    if (!owner) {
      continue;
    }
    ranges.push({ owner, open, close: matchingBrace(masked, open) });
  }
  return ranges;
}

function moduleName(relativePath: string): string {
  return path.basename(relativePath, path.extname(relativePath));
}

function extractFunctions(
  relativePath: string,
  source: string,
): ExtractedFunction[] {
  const masked = maskCommentsAndStrings(source);
  const ranges = implRanges(masked);
  const functions: ExtractedFunction[] = [];
  for (const match of masked.matchAll(/\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\b/g)) {
    const name = match[1];
    const start = match.index ?? 0;
    const open = masked.indexOf("{", start);
    if (!name || open < 0) {
      continue;
    }
    const close = matchingBrace(masked, open);
    const maybeImpl = ranges
      .filter((range) => range.open < start && start < range.close)
      .sort((left, right) => left.close - left.open - (right.close - right.open))[0];
    const owner = maybeImpl?.owner ?? null;
    functions.push({
      symbol: owner
        ? `${owner}::${name}`
        : `${moduleName(relativePath)}::${name}`,
      name,
      owner,
      signature: source.slice(start, open),
      body: source.slice(open + 1, close),
    });
  }
  return functions;
}

function directTargetFailures(target: ExtractedFunction): string[] {
  const source = maskCommentsAndStrings(`${target.signature}{${target.body}}`);
  const forbidden: Array<[RegExp, string]> = [
    [/->\s*Result\b/, "Result return"],
    [/\?/, "? propagation"],
    [/\b(?:transaction_|compute_)?(?:txid|wtxid)\s*\(/, "identifier derivation"],
    [/\b(?:encode|decode)[A-Za-z0-9_]*\s*\(/, "encode/decode"],
    [
      /\b(?:std::fs|File::|OpenOptions::|TcpStream|UdpSocket|tokio::fs)\b/,
      "I/O type",
    ],
    [
      /\.(?:read|read_to_end|read_to_string|write|write_all|flush)\s*\(/,
      "I/O call",
    ],
    [/\bawait\b/, "async I/O await"],
  ];
  return forbidden
    .filter(([pattern]) => pattern.test(source))
    .map(
      ([, label]) =>
        `${target.name}: forbidden ${label} inside exact target apply`,
    );
}

function methodCalls(source: string): MethodCall[] {
  const calls: MethodCall[] = [];
  const pattern =
    /\b((?:self|[a-z_][A-Za-z0-9_]*)(?:\s*\.\s*[A-Za-z_][A-Za-z0-9_]*)*)\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*\(/g;
  for (const match of source.matchAll(pattern)) {
    const receiver = (match[1] ?? "").replace(/\s+/g, "");
    const name = match[2];
    if (receiver && name) {
      calls.push({ receiver, name, index: match.index ?? 0 });
    }
  }
  return calls;
}

function functionCallNames(source: string, methods: MethodCall[]): string[] {
  const methodNameIndexes = new Set(
    methods.map((call) => call.index + source.slice(call.index).indexOf(call.name)),
  );
  const names: string[] = [];
  for (const match of source.matchAll(/\b([A-Za-z_][A-Za-z0-9_:]*)\s*\(/g)) {
    const name = match[1];
    const index = match.index ?? 0;
    if (
      !name ||
      methodNameIndexes.has(index) ||
      source[index - 1] === "." ||
      /^[A-Z]/.test(name) ||
      ["if", "while", "for", "match", "return"].includes(name)
    ) {
      continue;
    }
    names.push(name.split("::").at(-1) ?? name);
  }
  return names;
}

function repoCandidatesByName(
  functions: Map<string, ExtractedFunction>,
): Map<string, string[]> {
  const candidates = new Map<string, string[]>();
  for (const target of functions.values()) {
    const symbols = candidates.get(target.name) ?? [];
    symbols.push(target.symbol);
    candidates.set(target.name, symbols);
  }
  return candidates;
}

function resolveMethodCall(
  current: ExtractedFunction,
  call: MethodCall,
  candidates: Map<string, string[]>,
): string | null | "unresolved" {
  if (call.receiver === "self" && current.owner) {
    const selfSymbol = `${current.owner}::${call.name}`;
    if ((candidates.get(call.name) ?? []).includes(selfSymbol)) {
      return selfSymbol;
    }
  }
  const maybePure = PURE_RECEIVER_CALLS.get(`${call.receiver}.${call.name}`);
  if (maybePure) {
    return maybePure;
  }
  const repoSymbols = candidates.get(call.name) ?? [];
  if (repoSymbols.length === 1) {
    return repoSymbols[0] ?? null;
  }
  if (repoSymbols.length > 1) {
    return "unresolved";
  }
  return null;
}

function inspectReachableFunctions(
  functions: Map<string, ExtractedFunction>,
): string[] {
  const candidates = repoCandidatesByName(functions);
  const visited = new Set<string>();
  const violations: string[] = [];

  const visit = (symbol: string): void => {
    if (visited.has(symbol)) {
      return;
    }
    visited.add(symbol);
    const target = functions.get(symbol);
    if (!target) {
      violations.push(`${symbol}: classified function not found`);
      return;
    }
    const masked = maskCommentsAndStrings(`${target.signature}{${target.body}}`);
    if (
      /->\s*Result\b|\?|\bawait\b/.test(masked) ||
      /\b(?:std::fs|File::|OpenOptions::|TcpStream|UdpSocket|tokio::fs)\b/.test(
        masked,
      ) ||
      /\.(?:read|read_to_end|read_to_string|write|write_all|flush)\s*\(/.test(
        masked,
      ) ||
      /\b(?:encode|decode)[A-Za-z0-9_]*\s*\(/.test(masked) ||
      /\b(?:transaction_|compute_)?(?:txid|wtxid)\s*\(/.test(masked)
    ) {
      violations.push(`${symbol}: fallible or effectful body`);
    }

    const methods = methodCalls(maskCommentsAndStrings(target.body));
    for (const call of methods) {
      const resolved = resolveMethodCall(target, call, candidates);
      if (resolved === "unresolved") {
        violations.push(`${symbol}: unresolved method overload ${call.name}`);
        continue;
      }
      if (!resolved) {
        violations.push(
          `${symbol}: unclassified call ${call.receiver}.${call.name}`,
        );
        continue;
      }
      if (PURE_CALL_ALLOWLIST.has(resolved)) {
        if (functions.has(resolved)) {
          visit(resolved);
        }
        continue;
      }
      if (
        !INFALLIBLE_APPLY_CALLEES.has(resolved) &&
        resolved !== DEPENDENT_ROOT_SYMBOL
      ) {
        violations.push(`${symbol}: unclassified method ${resolved}`);
      }
      if (functions.has(resolved)) {
        visit(resolved);
      }
    }

    for (const name of functionCallNames(maskCommentsAndStrings(target.body), methods)) {
      const repoSymbols = candidates.get(name) ?? [];
      if (repoSymbols.length === 0) {
        violations.push(`${symbol}: unclassified call ${name}`);
        continue;
      }
      if (repoSymbols.length !== 1) {
        violations.push(`${symbol}: unresolved function overload ${name}`);
        continue;
      }
      const resolved = repoSymbols[0] ?? "";
      if (
        !INFALLIBLE_APPLY_CALLEES.has(resolved) &&
        !PURE_CALL_ALLOWLIST.has(resolved)
      ) {
        violations.push(`${symbol}: unclassified function ${resolved}`);
      }
      visit(resolved);
    }
  };

  visit(DEPENDENT_ROOT_SYMBOL);
  return violations;
}

export function checkPhase134ApplyBoundaries(
  maybeRepoRoot: string = DEFAULT_REPO_ROOT,
): string[] {
  const sources = new Map(
    PHASE134_APPLY_SOURCE_FILES.map((file) => [
      file,
      readSourceRoot(maybeRepoRoot, file),
    ]),
  );
  const functions = new Map<string, ExtractedFunction>();
  const duplicateSymbols: string[] = [];
  for (const [file, source] of sources) {
    for (const target of extractFunctions(file, source)) {
      if (
        functions.has(target.symbol) &&
        (target.symbol === REQUIRED_ROOT_SYMBOL ||
          target.symbol === CONNECTED_BLOCK_ROOT_SYMBOL ||
          CONNECTED_BLOCK_SEAMS.some(({ symbol }) => symbol === target.symbol) ||
          target.symbol === DEPENDENT_ROOT_SYMBOL ||
          INFALLIBLE_APPLY_CALLEES.has(target.symbol) ||
          PURE_CALL_ALLOWLIST.has(target.symbol))
      ) {
        duplicateSymbols.push(target.symbol);
      }
      functions.set(target.symbol, target);
    }
  }

  const directFailures: string[] = [];
  for (const name of REQUIRED_TARGETS) {
    const matches = [...functions.values()].filter(
      (target) => target.name === name && target.owner === "ManagedPeerNetwork",
    );
    if (matches.length === 0) {
      directFailures.push(`${name}: required Phase 134 target apply not found`);
      continue;
    }
    if (matches.length > 1) {
      directFailures.push(`${name}: duplicate exact target apply`);
      continue;
    }
    directFailures.push(...directTargetFailures(matches[0]));
  }

  for (const file of PHASE134_APPLY_TARGET_FILES) {
    const source = sources.get(file) ?? "";
    const discovered = [
      ...source.matchAll(/\bfn\s+(apply_prepared_[A-Za-z0-9_]+)\b/g),
    ]
      .map((match) => match[1] ?? "")
      .filter((name) => !DISCOVERY_EXCLUSIONS.has(name));
    for (const name of discovered) {
      if (!REQUIRED_TARGETS.includes(name as (typeof REQUIRED_TARGETS)[number])) {
        directFailures.push(`${file}: unexpected target-like apply ${name}`);
      }
    }
  }
  if (directFailures.length > 0) {
    return directFailures;
  }

  const aggregate = functions.get(REQUIRED_ROOT_SYMBOL);
  const structuralTools = {
    maskCommentsAndStrings,
    matchingDelimiter,
    methodCalls,
    functionCallNames,
  };
  const aggregateValid = aggregate
    ? inspectOrdinaryAggregateRoot(
        aggregate,
        structuralTools,
        ATOMIC_CORE_COMMIT.has(
          "Mempool::commit_prepared_mempool_transition_with",
        ),
      )
    : false;
  const connectedBlockRoot = functions.get(CONNECTED_BLOCK_ROOT_SYMBOL);
  const connectedBlockRootValid = connectedBlockRoot
    ? inspectConnectedBlockRoot(connectedBlockRoot, structuralTools)
    : false;
  const connectedBlockSeamsValid = CONNECTED_BLOCK_SEAMS.every(
    ({ symbol, chainstatePreparation }) => {
      const seam = functions.get(symbol);
      return (
        seam !== undefined &&
        inspectConnectedBlockSeam(
          seam,
          chainstatePreparation,
          structuralTools,
        )
      );
    },
  );
  const reachableViolations = inspectReachableFunctions(functions);
  const removedApiPresent = REMOVED_VALIDATED_API.some((name) =>
    [...sources.values()].some((source) =>
      maskCommentsAndStrings(source).includes(name),
    ),
  );
  if (
    duplicateSymbols.length > 0 ||
    !aggregate ||
    !connectedBlockRoot ||
    removedApiPresent ||
    !aggregateValid ||
    !connectedBlockRootValid ||
    !connectedBlockSeamsValid ||
    reachableViolations.length > 0
  ) {
    if (process.env.PHASE134_APPLY_DEBUG === "1") {
      console.error(
        JSON.stringify(
          {
            duplicateSymbols,
            aggregateFound: aggregate !== undefined,
            aggregateValid,
            connectedBlockRootFound: connectedBlockRoot !== undefined,
            connectedBlockRootValid,
            connectedBlockSeamsValid,
            removedApiPresent,
            reachableViolations,
          },
          null,
          2,
        ),
      );
    }
    return [PHASE134_APPLY_BOUNDARY_DIAGNOSTIC];
  }
  return [];
}

if (import.meta.main) {
  const failures = checkPhase134ApplyBoundaries();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }
  console.log(
    `Phase 134 target apply discovery: ${REQUIRED_TARGETS.join(", ")}`,
  );
  console.log(
    `Phase 134 transitive apply boundary: atomic=${[...ATOMIC_CORE_COMMIT].join(", ")}`,
  );
  console.log(
    `Phase 134 classified infallible apply callees: ${[...INFALLIBLE_APPLY_CALLEES].join(", ")}`,
  );
  console.log("Phase 134 reachable apply helpers are classified and mutation-safe.");
}
