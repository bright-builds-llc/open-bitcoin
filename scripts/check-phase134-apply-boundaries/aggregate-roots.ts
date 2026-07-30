export const CONNECTED_BLOCK_ROOT_SYMBOL =
  "ManagedPeerNetwork::commit_connected_block_lifecycle_transaction" as const;

export const CONNECTED_BLOCK_SEAMS = [
  {
    symbol: "ManagedPeerNetwork::connect_local_block",
    chainstatePreparation: "prepare_connect_block",
  },
  {
    symbol: "ManagedPeerNetwork::connect_stored_block",
    chainstatePreparation: "prepare_connect_block_with_current_time",
  },
] as const;

export type StructuralFunction = {
  body: string;
};

type MethodCall = {
  receiver: string;
  name: string;
  index: number;
};

export type RustStructureTools = {
  maskCommentsAndStrings: (source: string) => string;
  matchingDelimiter: (
    source: string,
    open: number,
    openCharacter: string,
    closeCharacter: string,
  ) => number;
  methodCalls: (source: string) => MethodCall[];
  functionCallNames: (source: string, methods: MethodCall[]) => string[];
};

type MethodCallBounds = {
  call: number;
  open: number;
  close: number;
};

export function inspectOrdinaryAggregateRoot(
  root: StructuralFunction,
  tools: RustStructureTools,
  atomicCoreCommitAllowed: boolean,
): boolean {
  const maskedBody = tools.maskCommentsAndStrings(root.body);
  const atomicCalls = methodCallBounds(
    maskedBody,
    "commit_prepared_mempool_transition_with",
    tools,
  );
  if (!atomicCoreCommitAllowed || atomicCalls.length !== 1) {
    return false;
  }

  const atomicCall = atomicCalls[0];
  if (!atomicCall || atomicCall.call < 0 || atomicCall.close < 0) {
    return false;
  }
  const statementStart = maskedBody.lastIndexOf("let ", atomicCall.call);
  const statementEnd = maskedBody.indexOf(";", atomicCall.close);
  if (statementStart < 0 || statementEnd < atomicCall.call) {
    return false;
  }
  const beforeAtomicStatement = maskedBody.slice(0, statementStart);
  const earlierDependentMutation = [
    ...maskedBody.matchAll(/\.apply_prepared_[A-Za-z0-9_]+\s*\(/g),
  ].some((match) => (match.index ?? 0) < atomicCall.call);
  const dependentIndex = maskedBody.indexOf(
    ".apply_prepared_lifecycle(",
    atomicCall.call,
  );
  if (
    /\bself\s*\./.test(beforeAtomicStatement) ||
    earlierDependentMutation ||
    dependentIndex < statementEnd
  ) {
    return false;
  }

  const withoutAtomic =
    maskedBody.slice(0, statementStart) +
    " ".repeat(statementEnd + 1 - statementStart) +
    maskedBody.slice(statementEnd + 1);
  return !isFallibleOrEffectful(withoutAtomic);
}

export function inspectConnectedBlockRoot(
  root: StructuralFunction,
  tools: RustStructureTools,
): boolean {
  const maskedBody = tools.maskCommentsAndStrings(root.body);
  const atomicCalls = methodCallBounds(
    maskedBody,
    "commit_prepared_mempool_transition_with",
    tools,
  );
  if (atomicCalls.length !== 1) {
    return false;
  }
  const atomicCall = atomicCalls[0];
  if (!atomicCall || atomicCall.call < 0 || atomicCall.close < 0) {
    return false;
  }

  const statementStart = maskedBody.lastIndexOf("let ", atomicCall.call);
  const statementEnd = maskedBody.indexOf(";", atomicCall.close);
  if (statementStart < 0 || statementEnd < atomicCall.close) {
    return false;
  }
  const beforeTransaction = maskedBody.slice(0, statementStart);
  const beforeMethods = tools
    .methodCalls(beforeTransaction)
    .map(({ receiver, name }) => `${receiver}.${name}`);
  if (
    beforeMethods.length !== 1 ||
    beforeMethods[0] !== "sealed.into_parts" ||
    isFallibleOrEffectful(beforeTransaction)
  ) {
    return false;
  }

  const invocationPrefix = maskedBody.slice(statementStart, atomicCall.open);
  if (
    !/\bself\s*\.\s*mempool\s*\.\s*mempool_mut\s*\(\s*\)\s*\.\s*commit_prepared_mempool_transition_with\s*$/.test(
      invocationPrefix,
    )
  ) {
    return false;
  }

  const callbackMarker = /\|\|\s*\{/.exec(
    maskedBody.slice(atomicCall.open + 1, atomicCall.close),
  );
  if (!callbackMarker) {
    return false;
  }
  const callbackOpen =
    atomicCall.open +
    1 +
    callbackMarker.index +
    callbackMarker[0].lastIndexOf("{");
  const callbackClose = tools.matchingDelimiter(
    maskedBody,
    callbackOpen,
    "{",
    "}",
  );
  if (callbackClose > atomicCall.close) {
    return false;
  }
  const callback = maskedBody.slice(callbackOpen + 1, callbackClose);
  const callbackMethodCalls = tools.methodCalls(callback);
  const callbackMethods = callbackMethodCalls.map(
    ({ receiver, name }) => `${receiver}.${name}`,
  );
  const expectedCallbackMethods = [
    "chainstate.commit_prepared_connect",
    "peer_manager.on_active_tip_changed",
    "blocks_by_hash.insert",
    "block.clone",
    "peer_manager.note_local_position",
  ];
  const callbackFunctions = tools.functionCallNames(
    callback,
    callbackMethodCalls,
  );
  if (
    callbackMethods.length !== expectedCallbackMethods.length ||
    callbackMethods.some(
      (method, index) => method !== expectedCallbackMethods[index],
    ) ||
    callbackFunctions.length !== 1 ||
    callbackFunctions[0] !== "fresh_reject_evidence_tweak" ||
    isFallibleOrEffectful(callback)
  ) {
    return false;
  }

  const transactionTail = maskedBody.slice(atomicCall.close + 1, statementEnd);
  if (
    !/^\s*\.map_err\s*\(\s*LifecycleProjectionError::Mempool\s*\)\s*\?\s*$/.test(
      transactionTail,
    )
  ) {
    return false;
  }

  const afterTransaction = maskedBody.slice(statementEnd + 1);
  const afterMethods = tools
    .methodCalls(afterTransaction)
    .map(({ receiver, name }) => `${receiver}.${name}`);
  return (
    afterMethods.length === 1 &&
    afterMethods[0] === "self.apply_prepared_lifecycle" &&
    !isFallibleOrEffectful(afterTransaction)
  );
}

export function inspectConnectedBlockSeam(
  seam: StructuralFunction,
  chainstatePreparation: string,
  tools: RustStructureTools,
): boolean {
  const maskedBody = tools.maskCommentsAndStrings(seam.body);
  const chainstateCalls = methodCallBounds(
    maskedBody,
    chainstatePreparation,
    tools,
  );
  const lifecycleCalls = methodCallBounds(
    maskedBody,
    "prepare_connected_block_transition",
    tools,
  );
  const sealingCalls = methodCallBounds(
    maskedBody,
    "prepare_maintenance_step",
    tools,
  );
  const transactionCalls = methodCallBounds(
    maskedBody,
    "commit_connected_block_lifecycle_transaction",
    tools,
  );
  if (
    chainstateCalls.length !== 1 ||
    lifecycleCalls.length !== 1 ||
    sealingCalls.length !== 1 ||
    transactionCalls.length !== 1
  ) {
    return false;
  }
  const chainstate = chainstateCalls[0];
  const lifecycle = lifecycleCalls[0];
  const sealing = sealingCalls[0];
  const transaction = transactionCalls[0];
  if (!chainstate || !lifecycle || !sealing || !transaction) {
    return false;
  }
  if (
    !(
      chainstate.call < lifecycle.call &&
      lifecycle.call < sealing.call &&
      sealing.call < transaction.call
    )
  ) {
    return false;
  }
  const sealingStatementEnd = maskedBody.indexOf(";", sealing.close);
  if (sealingStatementEnd < sealing.close) {
    return false;
  }
  const sealedToTransaction = maskedBody.slice(
    sealingStatementEnd + 1,
    transaction.call,
  );
  const lastEarlyReturn = maskedBody.lastIndexOf("return ", chainstate.call);
  const connectedPathStart =
    lastEarlyReturn < 0
      ? 0
      : maskedBody.indexOf(";", lastEarlyReturn) + 1;
  const connectedPath = maskedBody.slice(
    connectedPathStart,
    transaction.call,
  );
  return (
    !isFallibleOrEffectful(sealedToTransaction) &&
    !/\.(?:commit_prepared_connect|commit_prepared_mempool_transition_with|commit_sealed_lifecycle|apply_prepared_lifecycle|on_active_tip_changed|note_local_position)\s*\(/.test(
      connectedPath,
    )
  );
}

function methodCallBounds(
  maskedBody: string,
  methodName: string,
  tools: RustStructureTools,
): MethodCallBounds[] {
  return [
    ...maskedBody.matchAll(new RegExp(`\\.${methodName}\\s*\\(`, "g")),
  ].map((match) => {
    const call = match.index ?? -1;
    const open = maskedBody.indexOf("(", call);
    return {
      call,
      open,
      close:
        open < 0
          ? -1
          : tools.matchingDelimiter(maskedBody, open, "(", ")"),
    };
  });
}

function isFallibleOrEffectful(source: string): boolean {
  return (
    /\?|\bawait\b/.test(source) ||
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
