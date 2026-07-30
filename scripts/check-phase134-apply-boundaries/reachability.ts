import { strictSyntaxViolations } from "./strict-syntax";

export type ExtractedFunction = {
  file: string;
  symbol: string;
  name: string;
  owner: string | null;
  modulePath: string;
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

type ClassifiedMethod = {
  symbol: string;
  effect: "pure" | "mutation";
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

const RECEIVER_CALLS: ReadonlyMap<string, ClassifiedMethod> = new Map([
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
    "children.retain",
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
  ["children.is_empty", { symbol: "BTreeSet::is_empty", effect: "pure" }],
  ["maybe_entry.iter", { symbol: "Option::iter", effect: "pure" }],
  ["count.saturating_sub", { symbol: "usize::saturating_sub", effect: "pure" }],
  [
    "position.height.saturating_add",
    { symbol: "usize::saturating_add", effect: "pure" },
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
    inspectSource(target, target.body, strictMutationBoundary);
  };

  const inspectSource = (
    current: ExtractedFunction,
    source: string,
    strictMutationBoundary: boolean,
    allowedDirectMethods: ReadonlySet<string> = new Set(),
    allowedMutableBorrowTargets: ReadonlySet<string> = new Set(),
  ): void => {
    const masked = tools.maskCommentsAndStrings(source);
    if (strictMutationBoundary && isFallibleOrEffectful(masked)) {
      violations.add(`${current.symbol}: fallible or effectful critical slice`);
    }
    if (strictMutationBoundary) {
      for (const violation of strictSyntaxViolations(
        masked,
        allowedMutableBorrowTargets,
      )) {
        violations.add(`${current.symbol}: ${violation}`);
      }
    }

    const methods = methodCalls(masked);
    for (const call of methods) {
      const callPath = `${call.receiver}.${call.name}`;
      if (allowedDirectMethods.has(callPath)) {
        continue;
      }
      const resolved = resolveMethodCall(current, call, candidates);
      if (resolved === "unresolved") {
        violations.add(
          `${current.symbol}: unresolved method overload ${call.name}`,
        );
        continue;
      }
      if (!resolved) {
        violations.add(`${current.symbol}: unclassified call ${callPath}`);
        continue;
      }
      const repoMutation =
        classifiedRepoMethods.has(resolved.symbol) ||
        resolved.symbol === roots.dependent;
      if (
        strictMutationBoundary &&
        (resolved.effect === "mutation" || repoMutation)
      ) {
        violations.add(
          `${current.symbol}: mutating call ${callPath} outside aggregate transaction`,
        );
        continue;
      }
      if (!functions.has(resolved.symbol)) {
        if (
          resolved.effect === "mutation" &&
          resolved.symbol.startsWith("collection::")
        ) {
          violations.add(
            `${current.symbol}: unclassified method ${resolved.symbol}`,
          );
        }
        continue;
      }
      if (
        !strictMutationBoundary &&
        !classifiedRepoMethods.has(resolved.symbol) &&
        resolved.symbol !== roots.dependent &&
        !PURE_CALL_ALLOWLIST.has(resolved.symbol)
      ) {
        violations.add(
          `${current.symbol}: unclassified method ${resolved.symbol}`,
        );
      }
      visit(resolved.symbol, strictMutationBoundary);
    }

    for (const call of functionCalls(masked, methods)) {
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
    const allowedMutableBorrowTargets =
      symbol === roots.connectedBlock
        ? new Set([
            "self.chainstate",
            "self.peer_manager",
            "self.blocks_by_hash",
          ])
        : new Set<string>();
    inspectSource(
      target,
      target.body.slice(0, statementStart),
      true,
      new Set(),
      allowedMutableBorrowTargets,
    );
    inspectSource(
      target,
      target.body.slice(statementEnd + 1),
      true,
      new Set(["self.apply_prepared_lifecycle"]),
    );
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
    inspectSource(
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

function resolveMethodCall(
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
  const maybeClassified = RECEIVER_CALLS.get(
    `${call.receiver}.${call.name}`,
  );
  if (maybeClassified) {
    return maybeClassified;
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
