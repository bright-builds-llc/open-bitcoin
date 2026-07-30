import path from "node:path";

export type ExtractedFunction = {
  file: string;
  symbol: string;
  name: string;
  owner: string | null;
  signature: string;
  body: string;
};

export type MethodCall = {
  receiver: string;
  name: string;
  index: number;
};

type FunctionCall = {
  path: string;
  name: string;
  index: number;
};

type CallGraphTools = {
  maskCommentsAndStrings: (source: string) => string;
  matchingDelimiter: (
    source: string,
    open: number,
    openCharacter: string,
    closeCharacter: string,
  ) => number;
};

type ReachabilityRoots = {
  ordinary: string;
  connectedBlock: string;
  connectedBlockSeams: readonly string[];
  dependent: string;
};

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

export function methodCalls(source: string): MethodCall[] {
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

export function functionCallNames(
  source: string,
  methods: MethodCall[],
): string[] {
  return functionCalls(source, methods).map(({ name }) => name);
}

export function inspectCriticalReachability(
  functions: Map<string, ExtractedFunction>,
  sources: Map<string, string>,
  roots: ReachabilityRoots,
  classifiedRepoMethods: ReadonlySet<string>,
  tools: CallGraphTools,
): string[] {
  const candidates = candidatesByName(functions);
  const aliases = aliasesByFile(sources, tools.maskCommentsAndStrings);
  const visited = new Set<string>();
  const violations = new Set<string>();

  const visitFunctionCalls = (
    current: ExtractedFunction,
    source: string,
    strictMutationBoundary: boolean,
  ): void => {
    const masked = tools.maskCommentsAndStrings(source);
    for (const call of functionCalls(masked, methodCalls(masked))) {
      const resolved = resolveFunctionCall(current, call, candidates, aliases);
      if (resolved === "unresolved") {
        violations.add(
          `${current.symbol}: unresolved function call ${call.path}`,
        );
        continue;
      }
      if (!resolved) {
        violations.add(`${current.symbol}: unclassified call ${call.path}`);
        continue;
      }
      visit(resolved, strictMutationBoundary);
    }
  };

  const visit = (symbol: string, strictMutationBoundary: boolean): void => {
    const visitKey = `${strictMutationBoundary ? "strict" : "dependent"}:${symbol}`;
    if (visited.has(visitKey)) {
      return;
    }
    visited.add(visitKey);
    const target = functions.get(symbol);
    if (!target) {
      violations.add(`${symbol}: classified function not found`);
      return;
    }
    const maskedSignatureAndBody = tools.maskCommentsAndStrings(
      `${target.signature}{${target.body}}`,
    );
    if (isFallibleOrEffectful(maskedSignatureAndBody)) {
      violations.add(`${symbol}: fallible or effectful body`);
    }
    if (
      strictMutationBoundary &&
      hasDirectAssignmentMutation(maskedSignatureAndBody)
    ) {
      violations.add(`${symbol}: direct mutation outside aggregate transaction`);
    }

    const maskedBody = tools.maskCommentsAndStrings(target.body);
    const methods = methodCalls(maskedBody);
    for (const call of methods) {
      const resolved = resolveMethodCall(target, call, candidates);
      if (resolved === "unresolved") {
        violations.add(`${symbol}: unresolved method overload ${call.name}`);
        continue;
      }
      if (!resolved) {
        violations.add(
          `${symbol}: unclassified call ${call.receiver}.${call.name}`,
        );
        continue;
      }
      if (functions.has(resolved)) {
        if (
          !classifiedRepoMethods.has(resolved) &&
          resolved !== roots.dependent &&
          !PURE_CALL_ALLOWLIST.has(resolved)
        ) {
          violations.add(`${symbol}: unclassified method ${resolved}`);
        }
        visit(resolved, strictMutationBoundary);
      } else if (!PURE_CALL_ALLOWLIST.has(resolved)) {
        violations.add(`${symbol}: unclassified method ${resolved}`);
      }
    }
    visitFunctionCalls(target, target.body, strictMutationBoundary);
  };

  const seedOutsideTransaction = (symbol: string): void => {
    const target = maybeFunction(functions, symbol, violations);
    if (!target) {
      return;
    }
    const masked = tools.maskCommentsAndStrings(target.body);
    const maybeBounds = singleMethodCallBounds(
      masked,
      "commit_prepared_mempool_transition_with",
      tools,
    );
    if (!maybeBounds) {
      violations.add(`${symbol}: aggregate transaction is not unique`);
      return;
    }
    const statementStart = masked.lastIndexOf("let ", maybeBounds.call);
    const statementEnd = masked.indexOf(";", maybeBounds.close);
    if (statementStart < 0 || statementEnd < maybeBounds.close) {
      violations.add(`${symbol}: aggregate transaction statement is malformed`);
      return;
    }
    visitFunctionCalls(target, target.body.slice(0, statementStart), true);
    visitFunctionCalls(target, target.body.slice(statementEnd + 1), true);
  };

  const seedBetweenSealAndTransaction = (symbol: string): void => {
    const target = maybeFunction(functions, symbol, violations);
    if (!target) {
      return;
    }
    const masked = tools.maskCommentsAndStrings(target.body);
    const maybeSealing = singleMethodCallBounds(
      masked,
      "prepare_maintenance_step",
      tools,
    );
    const maybeTransaction = singleMethodCallBounds(
      masked,
      "commit_connected_block_lifecycle_transaction",
      tools,
    );
    if (!maybeSealing || !maybeTransaction) {
      violations.add(`${symbol}: sealed transaction seam is not unique`);
      return;
    }
    const sealingStatementEnd = masked.indexOf(";", maybeSealing.close);
    if (
      sealingStatementEnd < maybeSealing.close ||
      maybeTransaction.call < sealingStatementEnd
    ) {
      violations.add(`${symbol}: sealed transaction seam is malformed`);
      return;
    }
    visitFunctionCalls(
      target,
      target.body.slice(sealingStatementEnd + 1, maybeTransaction.call),
      true,
    );
  };

  seedOutsideTransaction(roots.ordinary);
  seedOutsideTransaction(roots.connectedBlock);
  for (const seam of roots.connectedBlockSeams) {
    seedBetweenSealAndTransaction(seam);
  }
  visit(roots.dependent, false);
  return [...violations];
}

function functionCalls(
  source: string,
  methods: MethodCall[],
): FunctionCall[] {
  const methodNameIndexes = new Set(
    methods.map(
      (call) => call.index + source.slice(call.index).indexOf(call.name),
    ),
  );
  const calls: FunctionCall[] = [];
  for (const match of source.matchAll(
    /\b([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)\s*\(/g,
  )) {
    const callPath = match[1];
    const index = match.index ?? 0;
    const name = callPath?.split("::").at(-1);
    if (
      !callPath ||
      !name ||
      methodNameIndexes.has(index) ||
      source[index - 1] === "." ||
      /^[A-Z]/.test(name) ||
      ["if", "let", "while", "for", "match", "return"].includes(name)
    ) {
      continue;
    }
    calls.push({ path: callPath, name, index });
  }
  return calls;
}

function candidatesByName(
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

function aliasesByFile(
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

function resolveFunctionCall(
  current: ExtractedFunction,
  call: FunctionCall,
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
    const sameFile = matches.filter((candidate) => candidate.file === current.file);
    if (sameFile.length > 0) {
      matches = sameFile;
    }
  } else {
    const qualifier = [...resolvedSegments]
      .slice(0, -1)
      .filter((segment) => !["crate", "self", "super"].includes(segment))
      .at(-1);
    if (qualifier) {
      const qualified = matches.filter(
        (candidate) =>
          candidate.owner === qualifier ||
          path.basename(candidate.file, path.extname(candidate.file)) ===
            qualifier,
      );
      if (qualified.length > 0) {
        matches = qualified;
      }
    }
  }
  if (matches.length === 1) {
    return matches[0]?.symbol ?? null;
  }
  return matches.length > 1 ? "unresolved" : null;
}

function resolveMethodCall(
  current: ExtractedFunction,
  call: MethodCall,
  candidates: Map<string, ExtractedFunction[]>,
): string | null | "unresolved" {
  if (call.receiver === "self" && current.owner) {
    const selfSymbol = `${current.owner}::${call.name}`;
    if ((candidates.get(call.name) ?? []).some(({ symbol }) => symbol === selfSymbol)) {
      return selfSymbol;
    }
  }
  const maybePure = PURE_RECEIVER_CALLS.get(`${call.receiver}.${call.name}`);
  if (maybePure) {
    return maybePure;
  }
  const matches = candidates.get(call.name) ?? [];
  if (matches.length === 1) {
    return matches[0]?.symbol ?? null;
  }
  return matches.length > 1 ? "unresolved" : null;
}

function maybeFunction(
  functions: Map<string, ExtractedFunction>,
  symbol: string,
  violations: Set<string>,
): ExtractedFunction | null {
  const maybeTarget = functions.get(symbol);
  if (!maybeTarget) {
    violations.add(`${symbol}: root function not found`);
    return null;
  }
  return maybeTarget;
}

function singleMethodCallBounds(
  source: string,
  methodName: string,
  tools: CallGraphTools,
): { call: number; close: number } | null {
  const matches = [
    ...source.matchAll(new RegExp(`\\.${methodName}\\s*\\(`, "g")),
  ];
  if (matches.length !== 1) {
    return null;
  }
  const call = matches[0]?.index ?? -1;
  const open = source.indexOf("(", call);
  if (call < 0 || open < 0) {
    return null;
  }
  return {
    call,
    close: tools.matchingDelimiter(source, open, "(", ")"),
  };
}

function isFallibleOrEffectful(source: string): boolean {
  return (
    /->\s*Result\b|\?|\bawait\b/.test(source) ||
    /\b(?:std::fs|File::|OpenOptions::|TcpStream|UdpSocket|tokio::fs)\b/.test(
      source,
    ) ||
    /\.(?:read|read_to_end|read_to_string|write|write_all|flush)\s*\(/.test(
      source,
    ) ||
    /\b(?:encode|decode)[A-Za-z0-9_]*\s*\(/.test(source) ||
    /\b(?:transaction_|compute_)?(?:txid|wtxid)\s*\(/.test(source)
  );
}

function hasDirectAssignmentMutation(source: string): boolean {
  return (
    /\b(?:self|[a-z_][A-Za-z0-9_]*)\s*(?:\.\s*[A-Za-z_][A-Za-z0-9_]*)+\s*(?:=|\+=|-=|\*=|\/=)/.test(
      source,
    ) || /\*\s*[a-z_][A-Za-z0-9_]*\s*(?:=|\+=|-=|\*=|\/=)/.test(source)
  );
}
