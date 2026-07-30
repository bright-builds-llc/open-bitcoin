import type { ScannedMethodCall } from "./rust-calls";
import { type RustToken, tokenizeRust } from "./rust-lexer";

export function provenPureReceiverSymbol(
  symbol: string,
  body: string,
  call: ScannedMethodCall,
): string | null {
  const bodyTokens = tokenizeRust(body).filter(({ start }) => start < call.index);

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
    symbol === "PeerManager::apply_prepared_transaction_lifecycle" &&
    call.receiver === "self.known_wtxids_by_txid" &&
    call.name === "get"
  ) {
    return "BTreeMap::get";
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

function containsSequence(tokens: RustToken[], values: string[]): boolean {
  return tokens.some((_, start) =>
    values.every((value, offset) => tokens[start + offset]?.value === value),
  );
}
