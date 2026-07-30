import {
  aliasesByFile,
  candidatesByName,
  PURE_CALL_ALLOWLIST,
  resolveFunctionCall,
  resolveMethodCall,
} from "./call-resolution";
import {
  type ScannedMethodCall,
  scanRust,
  strictSyntaxViolations,
} from "./strict-syntax";

export type ExtractedFunction = {
  file: string;
  symbol: string;
  name: string;
  owner: string | null;
  modulePath: string;
  signature: string;
  body: string;
};

export type MethodCall = ScannedMethodCall;

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

export function methodCalls(source: string): MethodCall[] {
  return scanRust(source).methodCalls;
}

export function functionCallNames(
  source: string,
  _methods: MethodCall[],
): string[] {
  return scanRust(source).functionCalls.map(({ name }) => name);
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
    sourceOffset = 0,
  ): void => {
    const masked = tools.maskCommentsAndStrings(source);
    if (strictMutationBoundary && isFallibleOrEffectful(masked)) {
      violations.add(`${current.symbol}: fallible or effectful critical slice`);
    }
    if (strictMutationBoundary) {
      for (const violation of strictSyntaxViolations(
        source,
        allowedMutableBorrowTargets,
      )) {
        violations.add(`${current.symbol}: ${violation}`);
      }
    }

    const scan = scanRust(source);
    const methods = scan.methodCalls;
    for (const scannedCall of methods) {
      const call = {
        ...scannedCall,
        index: scannedCall.index + sourceOffset,
      };
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

    for (const call of scan.functionCalls) {
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
      new Set(),
      statementEnd + 1,
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
      new Set(),
      new Set(),
      sealingStatementEnd + 1,
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
