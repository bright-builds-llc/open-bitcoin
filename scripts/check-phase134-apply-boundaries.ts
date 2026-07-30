#!/usr/bin/env bun

import path from "node:path";

import {
  CONNECTED_BLOCK_ROOT_SYMBOL,
  CONNECTED_BLOCK_SEAMS,
  inspectConnectedBlockRoot,
  inspectConnectedBlockSeam,
  inspectOrdinaryAggregateRoot,
} from "./check-phase134-apply-boundaries/aggregate-roots";
import {
  type ExtractedFunction,
  functionCallNames,
  inspectCriticalReachability,
  methodCalls,
} from "./check-phase134-apply-boundaries/reachability";
import { PURE_CALL_ALLOWLIST } from "./check-phase134-apply-boundaries/call-resolution";
import { maskRustCommentsAndLiterals } from "./check-phase134-apply-boundaries/strict-syntax";
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
  "peer::transaction_lifecycle::apply_prepared_orphan_lifecycle",
  "TxDownloadScheduler::forget_lifecycle_identity",
  "TxDownloadScheduler::mark_already_have",
  "TxOrphanage::remove_candidate_cursor",
  "TxOrphanage::remove_orphan_without_candidate_scan",
  "TxOrphanage::record_accepted_package_fingerprint",
  "TxOrphanage::retire_accepted_package_fingerprint",
  "TxOrphanage::remove_child_index",
  "TxOrphanage::decrement_peer_count",
]);

type ImplRange = {
  owner: string;
  open: number;
  close: number;
};

type ModuleRange = {
  name: string;
  open: number;
  close: number;
};

const maskCommentsAndStrings = maskRustCommentsAndLiterals;

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

function modulePath(relativePath: string): string {
  const normalized = relativePath.replaceAll("\\", "/");
  const sourceSegments = normalized.split("/src/").at(-1)?.split("/") ?? [];
  const fileName = sourceSegments.pop()?.replace(/\.rs$/, "") ?? "";
  if (fileName && !["lib", "main", "mod"].includes(fileName)) {
    sourceSegments.push(fileName);
  }
  return sourceSegments.join("::");
}

function moduleRanges(masked: string): ModuleRange[] {
  const ranges: ModuleRange[] = [];
  for (const match of masked.matchAll(
    /\bmod\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/g,
  )) {
    const name = match[1];
    const open = (match.index ?? 0) + match[0].lastIndexOf("{");
    if (name) {
      ranges.push({ name, open, close: matchingBrace(masked, open) });
    }
  }
  return ranges;
}

function extractFunctions(
  relativePath: string,
  source: string,
): ExtractedFunction[] {
  const masked = maskCommentsAndStrings(source);
  const ranges = implRanges(masked);
  const modules = moduleRanges(masked);
  const fileModulePath = modulePath(relativePath);
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
    const inlineModules = modules
      .filter((range) => range.open < start && start < range.close)
      .sort((left, right) => left.open - right.open)
      .map(({ name: module }) => module);
    const targetModulePath = [fileModulePath, ...inlineModules]
      .filter(Boolean)
      .join("::");
    functions.push({
      file: relativePath,
      symbol: owner
        ? `${owner}::${name}`
        : `${targetModulePath}::${name}`,
      name,
      owner,
      modulePath: targetModulePath,
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
  const reachableViolations = inspectCriticalReachability(
    functions,
    sources,
    {
      ordinary: REQUIRED_ROOT_SYMBOL,
      connectedBlock: CONNECTED_BLOCK_ROOT_SYMBOL,
      connectedBlockSeams: CONNECTED_BLOCK_SEAMS.map(({ symbol }) => symbol),
      dependent: DEPENDENT_ROOT_SYMBOL,
    },
    INFALLIBLE_APPLY_CALLEES,
    structuralTools,
  );
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
