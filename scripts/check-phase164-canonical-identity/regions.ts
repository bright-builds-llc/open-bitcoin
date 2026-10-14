import { ordinaryRust, rustCode } from "../check-phase157-index-catch-up/rust-evidence.ts";

export type Region = { symbol: string; ordinal: number; start: number; end: number; code: string; raw: string; test: boolean };

/** Offset-preserving, bounded Rust function classification; unsupported braces fail closed. */
export function regions(source: string): readonly Region[] {
  const code = rustCode(source);
  const ordinary = ordinaryRust(source);
  const counts = new Map<string, number>();
  return [...code.matchAll(/\bfn\s+(\w+)\b/g)].flatMap(match => {
    const start = match.index;
    const open = code.indexOf("{", start);
    if (open < 0 || code.slice(start, open).includes(";")) return [];
    let depth = 1;
    let end = open + 1;
    while (end < code.length && depth) {
      if (code[end] === "{") depth++;
      if (code[end] === "}") depth--;
      end++;
    }
    if (depth) throw new Error("unterminated function region");
    const symbol = match[1];
    const ordinal = counts.get(symbol) ?? 0;
    counts.set(symbol, ordinal + 1);
    return [{ symbol, ordinal, start, end, code: code.slice(start, end), raw: source.slice(start, end), test: ordinary[start] === " " }];
  });
}

/** Executable opener/constructor references include aliases; masked text earns no edge. */
export const SENSITIVE_CALLS = /\b(?:\w+\s*::\s*(?:open(?!_bitcoin)\w*|builder|from_(?:fixture_)?runtime_config\w*|for_(?:fixture_)?local_operator|from_network\w*|from_canonical_network)|AdmissionScope\s*::\s*Fixture)\b|\b(?:build_fixture_wallet_state_with_store|recover_fixture_mempool_snapshot_from_store_handle|fixture_open_\w+)\s*\(|\.\s*(?:open|open_unrecovered|builder)\s*\(|\badmission_scope\s*=(?!=)/g;
