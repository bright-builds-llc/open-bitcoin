import type { ExtractedFunction, MethodCall } from "./reachability";
import type { ScannedFunctionCall } from "./rust-calls";
import {
  provenCollectionMutationSymbol,
  provenPureReceiverSymbol,
} from "./strict-syntax";

type ClassifiedMethod = {
  symbol: string;
  effect: "pure" | "mutation";
};

const SELF_RECEIVER_CALLS: ReadonlyMap<string, ClassifiedMethod> = new Map([
  ...[
    "self.known_txids.remove",
    "self.known_txids.insert",
    "self.known_wtxids.remove",
    "self.known_wtxids.insert",
    "self.mempool_known.remove",
    "self.mempool_known.insert",
    "self.already_have.remove",
    "self.already_have.insert",
    "self.announcements.remove",
    "self.in_flight.remove",
    "self.known_wtxids_by_txid.remove",
    "self.known_wtxids_by_txid.insert",
    "self.compact_download_states.insert",
    "self.pending_reconsideration.remove",
    "self.orphans.remove",
    "self.candidate_cursors.remove",
    "self.accepted_package_fingerprints.insert",
    "self.accepted_package_fingerprints.remove",
    "self.children_by_parent.get_mut",
    "self.children_by_parent.remove",
    "self.orphan_count_by_peer.get_mut",
    "self.orphan_count_by_peer.remove",
  ].map((receiver) => [
    receiver,
    {
      symbol: collectionSymbol(receiver.split(".").at(-1) ?? ""),
      effect: "mutation" as const,
    },
  ]),
  [
    "self.known_wtxids_by_txid.get",
    { symbol: "BTreeMap::get", effect: "pure" },
  ],
]);

export const PURE_CALL_ALLOWLIST = new Set([
  "PeerTransactionIdentity::relay_ids",
  "SealedLifecycleProjection::into_parts",
  "BTreeMap::get",
  "BTreeSet::is_empty",
  "Option::iter",
  "usize::saturating_add",
  "usize::saturating_sub",
]);

const MUTATING_METHOD_NAMES = new Set([
  "append",
  "as_mut",
  "clear",
  "dedup",
  "drain",
  "entry",
  "extend",
  "extend_from_slice",
  "get_mut",
  "insert",
  "iter_mut",
  "pop",
  "pop_back",
  "pop_front",
  "push",
  "push_back",
  "push_front",
  "remove",
  "replace",
  "retain",
  "sort",
  "sort_by",
  "split_off",
  "swap_remove",
  "take",
  "truncate",
  "values_mut",
]);

export function candidatesByName(
  functions: Map<string, ExtractedFunction>,
): Map<string, ExtractedFunction[]> {
  const candidates = new Map<string, ExtractedFunction[]>();
  for (const target of functions.values()) {
    const sameName = candidates.get(target.name) ?? [];
    sameName.push(target);
    candidates.set(target.name, sameName);
  }
  return candidates;
}

export function aliasesByFile(
  sources: Map<string, string>,
  maskCommentsAndStrings: (source: string) => string,
): Map<string, Map<string, string>> {
  const aliases = new Map<string, Map<string, string>>();
  for (const [file, source] of sources) {
    const fileAliases = new Map<string, string>();
    const masked = maskCommentsAndStrings(source);
    for (const match of masked.matchAll(
      /\buse\s+([A-Za-z_][A-Za-z0-9_:]*)\s+as\s+([A-Za-z_][A-Za-z0-9_]*)\s*;/g,
    )) {
      const target = match[1];
      const alias = match[2];
      if (target && alias) {
        fileAliases.set(alias, target);
      }
    }
    aliases.set(file, fileAliases);
  }
  return aliases;
}

export function resolveFunctionCall(
  current: ExtractedFunction,
  call: ScannedFunctionCall,
  candidates: Map<string, ExtractedFunction[]>,
  aliases: Map<string, Map<string, string>>,
): string | null | "unresolved" {
  const segments = call.path.split("::");
  const maybeAliasTarget = aliases.get(current.file)?.get(segments[0] ?? "");
  const resolvedPath = maybeAliasTarget
    ? [...maybeAliasTarget.split("::"), ...segments.slice(1)].join("::")
    : call.path;
  const resolvedSegments = resolvedPath.split("::");
  const name = resolvedSegments.at(-1) ?? call.name;
  let matches = candidates.get(name) ?? [];
  if (resolvedSegments.length === 1) {
    const sameModule = matches.filter(
      (candidate) => candidate.modulePath === current.modulePath,
    );
    if (sameModule.length > 0) {
      matches = sameModule;
    }
  } else {
    const canonicalPath = canonicalFunctionPath(
      current.modulePath,
      resolvedSegments,
    );
    matches = matches.filter(
      (candidate) =>
        `${candidate.modulePath}::${candidate.name}` === canonicalPath,
    );
  }
  if (matches.length === 1) {
    return matches[0]?.symbol ?? null;
  }
  return matches.length > 1 || resolvedSegments.length > 1
    ? "unresolved"
    : null;
}

export function resolveMethodCall(
  current: ExtractedFunction,
  call: MethodCall,
  candidates: Map<string, ExtractedFunction[]>,
): ClassifiedMethod | null | "unresolved" {
  if (call.receiver === "self" && current.owner) {
    const selfSymbol = `${current.owner}::${call.name}`;
    if (
      (candidates.get(call.name) ?? []).some(
        ({ symbol }) => symbol === selfSymbol,
      )
    ) {
      return { symbol: selfSymbol, effect: "pure" };
    }
  }
  const maybeClassified = SELF_RECEIVER_CALLS.get(
    `${call.receiver}.${call.name}`,
  );
  if (maybeClassified) {
    return maybeClassified;
  }
  const maybePureSymbol = provenPureReceiverSymbol(
    current.symbol,
    current.signature,
    current.body,
    call,
  );
  if (maybePureSymbol) {
    return { symbol: maybePureSymbol, effect: "pure" };
  }
  const maybeCollectionMutation = provenCollectionMutationSymbol(
    current.symbol,
    current.body,
    call,
  );
  if (maybeCollectionMutation) {
    return { symbol: maybeCollectionMutation, effect: "mutation" };
  }
  const matches = candidates.get(call.name) ?? [];
  if (matches.length === 1) {
    const symbol = matches[0]?.symbol;
    return symbol ? { symbol, effect: "pure" } : null;
  }
  if (matches.length > 1) {
    return "unresolved";
  }
  if (MUTATING_METHOD_NAMES.has(call.name)) {
    return { symbol: `collection::${call.name}`, effect: "mutation" };
  }
  return null;
}

function canonicalFunctionPath(
  currentModulePath: string,
  resolvedSegments: string[],
): string {
  const segments = [...resolvedSegments];
  const current = currentModulePath.split("::").filter(Boolean);
  if (segments[0] === "crate") {
    segments.shift();
    return segments.join("::");
  }
  if (segments[0] === "self") {
    segments.shift();
    return [...current, ...segments].join("::");
  }
  while (segments[0] === "super") {
    segments.shift();
    current.pop();
  }
  return [...current, ...segments].join("::");
}

function collectionSymbol(methodName: string): string {
  if (methodName === "retain") {
    return "BTreeSet::retain";
  }
  if (methodName === "get_mut") {
    return "BTreeMap::get_mut";
  }
  return `Collection::${methodName}`;
}
