import type { ScannedMethodCall } from "./rust-calls";
import { type RustToken, tokenizeRust } from "./rust-lexer";

export function provenPureReceiverSymbol(
  symbol: string,
  signature: string,
  body: string,
  call: ScannedMethodCall,
): string | null {
  const signatureTokens = tokenizeRust(signature);
  const bodyTokens = tokenizeRust(body).filter(({ start }) => start < call.index);

  if (
    call.name === "is_empty" &&
    parameterOuterTypeIs(signatureTokens, call.receiver, "BTreeSet")
  ) {
    return "BTreeSet::is_empty";
  }
  const initializer = bindingInitializer(bodyTokens, call.receiver);
  if (call.name === "iter" && initializer?.[0]?.value === "Some") {
    return "Option::iter";
  }
  if (
    call.name === "saturating_sub" &&
    initializer?.some(
      ({ kind, value }) => kind === "number" && value.endsWith("usize"),
    )
  ) {
    return "usize::saturating_sub";
  }
  if (
    symbol === "TxOrphanage::remove_orphan_without_candidate_scan" &&
    call.receiver === "maybe_entry" &&
    call.name === "iter" &&
    containsSequence(bodyTokens, [
      "let",
      "maybe_entry",
      "=",
      "self",
      ".",
      "orphans",
      ".",
      "remove",
    ])
  ) {
    return "Option::iter";
  }
  if (
    symbol === "TxOrphanage::remove_child_index" &&
    call.receiver === "children" &&
    call.name === "is_empty" &&
    containsSequence(bodyTokens, [
      "Some",
      "(",
      "children",
      ")",
      "=",
      "self",
      ".",
      "children_by_parent",
      ".",
      "get_mut",
    ])
  ) {
    return "BTreeSet::is_empty";
  }
  if (
    symbol === "TxOrphanage::decrement_peer_count" &&
    call.receiver === "count" &&
    call.name === "saturating_sub" &&
    containsSequence(bodyTokens, [
      "Some",
      "(",
      "count",
      ")",
      "=",
      "self",
      ".",
      "orphan_count_by_peer",
      ".",
      "get_mut",
    ])
  ) {
    return "usize::saturating_sub";
  }
  if (
    (symbol.endsWith("::connect_local_block") ||
      symbol.endsWith("::connect_stored_block")) &&
    call.receiver === "position.height" &&
    call.name === "saturating_add" &&
    containsSequence(bodyTokens, [
      "let",
      "position",
      "=",
      "prepared_chainstate",
      ".",
      "position",
      "(",
      ")",
      ".",
      "clone",
      "(",
      ")",
    ])
  ) {
    return "usize::saturating_add";
  }
  return null;
}

export function provenCollectionMutationSymbol(
  symbol: string,
  body: string,
  call: ScannedMethodCall,
): string | null {
  if (
    symbol !== "TxOrphanage::remove_child_index" ||
    call.receiver !== "children" ||
    call.name !== "retain"
  ) {
    return null;
  }
  const bodyTokens = tokenizeRust(body).filter(({ start }) => start < call.index);
  return containsSequence(bodyTokens, [
    "Some",
    "(",
    "children",
    ")",
    "=",
    "self",
    ".",
    "children_by_parent",
    ".",
    "get_mut",
  ])
    ? "BTreeSet::retain"
    : null;
}

function parameterOuterTypeIs(
  tokens: RustToken[],
  receiver: string,
  typeName: string,
): boolean {
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(receiver)) {
    return false;
  }
  for (let index = 0; index < tokens.length - 2; index += 1) {
    if (
      tokens[index]?.value !== receiver ||
      tokens[index + 1]?.value !== ":"
    ) {
      continue;
    }
    let cursor = index + 2;
    if (tokens[cursor]?.value === "&") {
      cursor += 1;
    }
    if (tokens[cursor]?.value === "'") {
      cursor += 2;
    }
    if (tokens[cursor]?.value === "mut") {
      cursor += 1;
    }
    if (tokens[cursor]?.value === "::") {
      cursor += 1;
    }
    let maybeOuterType = tokens[cursor];
    if (maybeOuterType?.kind !== "identifier") {
      continue;
    }
    cursor += 1;
    while (
      tokens[cursor]?.value === "::" &&
      tokens[cursor + 1]?.kind === "identifier"
    ) {
      maybeOuterType = tokens[cursor + 1];
      cursor += 2;
    }
    if (maybeOuterType.value === typeName) {
      return true;
    }
  }
  return false;
}

function bindingInitializer(
  tokens: RustToken[],
  receiver: string,
): RustToken[] | null {
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(receiver)) {
    return null;
  }
  let result: RustToken[] | null = null;
  for (let index = 0; index < tokens.length - 3; index += 1) {
    if (tokens[index]?.value !== "let") {
      continue;
    }
    let name = index + 1;
    if (tokens[name]?.value === "mut") {
      name += 1;
    }
    if (tokens[name]?.value !== receiver) {
      continue;
    }
    const equals = tokens.findIndex(
      ({ value }, cursor) => cursor > name && value === "=",
    );
    if (equals < 0) {
      continue;
    }
    const semicolon = tokens.findIndex(
      ({ value }, cursor) => cursor > equals && value === ";",
    );
    result = tokens.slice(equals + 1, semicolon < 0 ? undefined : semicolon);
  }
  return result;
}

function containsSequence(tokens: RustToken[], values: string[]): boolean {
  return tokens.some((_, start) =>
    values.every((value, offset) => tokens[start + offset]?.value === value),
  );
}
