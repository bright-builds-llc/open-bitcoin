/** Mask comments and literals while preserving offsets and structural Rust tokens. */
export function rustCode(source: string): string {
  const chars = source.split("");
  const blank = (start: number, end: number) => {
    for (let index = start; index < end; index++) if (chars[index] !== "\n") chars[index] = " ";
  };
  let index = 0;
  while (index < source.length) {
    if (source.startsWith("//", index)) {
      const next = source.indexOf("\n", index);
      const end = next < 0 ? source.length : next;
      blank(index, end); index = end; continue;
    }
    if (source.startsWith("/*", index)) {
      const start = index;
      let depth = 1;
      index += 2;
      while (index < source.length && depth) {
        if (source.startsWith("/*", index)) { depth++; index += 2; }
        else if (source.startsWith("*/", index)) { depth--; index += 2; }
        else index++;
      }
      blank(start, index); continue;
    }
    const maybeRaw = /^(?:b)?r(#+)?"/.exec(source.slice(index));
    if (maybeRaw && (index === 0 || !/\w/.test(source[index - 1]))) {
      const endMarker = '"' + (maybeRaw[1] ?? "");
      const next = source.indexOf(endMarker, index + maybeRaw[0].length);
      const end = next < 0 ? source.length : next + endMarker.length;
      blank(index, end); index = end; continue;
    }
    if (source[index] === '"') {
      const start = index++;
      while (index < source.length) {
        if (source[index] === "\\") { index += 2; continue; }
        if (source[index++] === '"') break;
      }
      blank(start, index); continue;
    }
    const maybeChar = /^'(?:\\[^\n]|[^'\\\n])'/.exec(source.slice(index));
    if (maybeChar) { blank(index, index + maybeChar[0].length); index += maybeChar[0].length; continue; }
    index++;
  }
  return chars.join("");
}

export type RustFunction = { body: string; attributes: string };
/** Narrow source guard, not a Rust parser or proof of runtime safety. */
export function maybeRustFunction(source: string, symbol: string, production = false): RustFunction | undefined {
  const code = production ? withoutTestModules(rustCode(source)) : rustCode(source);
  const matches = [...code.matchAll(new RegExp(`\\bfn\\s+${symbol}\\b`, "g"))];
  if (matches.length !== 1) return undefined;
  const start = matches[0].index;
  const open = code.indexOf("{", start);
  if (open < 0 || code.slice(start, open).includes(";")) return undefined;
  let depth = 1;
  let end = open + 1;
  while (end < code.length && depth) {
    if (code[end] === "{") depth++;
    if (code[end] === "}") depth--;
    end++;
  }
  if (depth) return undefined;
  const previousItem = Math.max(code.lastIndexOf("}", start), code.lastIndexOf(";", start));
  return { body: code.slice(open + 1, end - 1), attributes: code.slice(previousItem + 1, start) };
}

function withoutTestModules(code: string): string {
  const chars = code.split("");
  const modules = /#\[\s*cfg\(\s*test\s*\)\s*\]\s*(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+\w+\s*\{/g;
  for (const match of code.matchAll(modules)) {
    let depth = 1;
    let end = match.index + match[0].length;
    while (end < code.length && depth) {
      if (code[end] === "{") depth++;
      if (code[end] === "}") depth--;
      end++;
    }
    for (let index = match.index; index < end; index++) if (chars[index] !== "\n") chars[index] = " ";
  }
  return chars.join("");
}

export function compact(code: string): string { return code.replace(/\s+/g, ""); }

/** Ordered named statements must exist inside the selected function body. */
export function hasOrderedCode(body: string, anchors: readonly string[]): boolean {
  const source = compact(body);
  let cursor = 0;
  for (const anchor of anchors) {
    const next = source.indexOf(compact(anchor), cursor);
    if (next < 0) return false;
    cursor = next + compact(anchor).length;
  }
  return true;
}
