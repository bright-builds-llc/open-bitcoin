export { compact, hasOrderedCode, rustCode } from "../check-phase156-prune-coordination/rust-evidence.ts";
import { rustCode } from "../check-phase156-prune-coordination/rust-evidence.ts";

/** Offset-preserving ordinary-source masking, extending the established Phase 156 guard. */
export function ordinaryRust(source: string): string {
  const code = rustCode(source);
  const chars = code.split("");
  const tests = /#\[\s*cfg\s*\(\s*(?:(?:\w+|::)\s*::\s*)?test\s*\)\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?(?:mod|(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?(?:extern\s+)?fn)\s+\w+[^;{]*\{/g;
  for (const match of code.matchAll(tests)) {
    const end = closingBrace(code, match.index + match[0].length - 1);
    for (let index = match.index; index <= end; index++) if (chars[index] !== "\n") chars[index] = " ";
  }
  return chars.join("");
}

function closingBrace(code: string, open: number): number {
  let depth = 1;
  let end = open + 1;
  while (end < code.length && depth) {
    if (code[end] === "{") depth++;
    if (code[end] === "}") depth--;
    end++;
  }
  return end - 1;
}

/** A narrow source contract, not a Rust parser, runtime test or performance proof. */
export function maybeFunction(source: string, symbol: string, production = false):
  { body: string; attributes: string; start: number; end: number } | undefined {
  const code = production ? ordinaryRust(source) : rustCode(source);
  const matches = [...code.matchAll(new RegExp(`\\bfn\\s+${symbol}\\b`, "g"))];
  if (matches.length !== 1) return undefined;
  const start = matches[0].index;
  const open = code.indexOf("{", start);
  if (open < 0 || code.slice(start, open).includes(";")) return undefined;
  const end = closingBrace(code, open);
  const previous = Math.max(code.lastIndexOf("}", start), code.lastIndexOf(";", start));
  return { body: code.slice(open + 1, end), attributes: code.slice(previous + 1, start), start, end: end + 1 };
}

export function hasAssertions(source: string, body: string, depth = 0): boolean {
  if (/\bassert(?:_eq|_ne)?\s*!/.test(body)) return true;
  if (depth >= 3) return false;
  return [...body.matchAll(/\b(\w+)\s*\(/g)].some(match => {
    const maybeHelper = maybeFunction(source, match[1]);
    return maybeHelper !== undefined && hasAssertions(source, maybeHelper.body, depth + 1);
  });
}
