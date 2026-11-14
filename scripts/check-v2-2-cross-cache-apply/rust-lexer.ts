export type RustToken = {
  kind: "identifier" | "number" | "punct";
  value: string;
  start: number;
  end: number;
};

type LexedRust = {
  tokens: RustToken[];
  ignoredRanges: Array<{ start: number; end: number }>;
};

const MULTI_PUNCTUATION = [
  "<<=",
  ">>=",
  "..=",
  "::",
  "->",
  "=>",
  "+=",
  "-=",
  "*=",
  "/=",
  "%=",
  "&=",
  "|=",
  "^=",
  "==",
  "!=",
  "<=",
  ">=",
  "&&",
  "||",
  "..",
] as const;

export function tokenizeRust(source: string): RustToken[] {
  return lexRust(source).tokens;
}

export function maskRustCommentsAndLiterals(source: string): string {
  const { ignoredRanges } = lexRust(source);
  const characters = [...source];
  for (const { start, end } of ignoredRanges) {
    for (let index = start; index < end; index += 1) {
      if (characters[index] !== "\n" && characters[index] !== "\r") {
        characters[index] = " ";
      }
    }
  }
  return characters.join("");
}

function lexRust(source: string): LexedRust {
  const tokens: RustToken[] = [];
  const ignoredRanges: Array<{ start: number; end: number }> = [];
  let index = 0;
  while (index < source.length) {
    const current = source[index] ?? "";
    const next = source[index + 1] ?? "";
    if (/\s/.test(current)) {
      index += 1;
      continue;
    }
    if (current === "/" && next === "/") {
      const end = lineCommentEnd(source, index);
      ignoredRanges.push({ start: index, end });
      index = end;
      continue;
    }
    if (current === "/" && next === "*") {
      const end = blockCommentEnd(source, index);
      ignoredRanges.push({ start: index, end });
      index = end;
      continue;
    }
    const maybeRawEnd = rawLiteralEnd(source, index);
    if (maybeRawEnd !== null) {
      ignoredRanges.push({ start: index, end: maybeRawEnd });
      index = maybeRawEnd;
      continue;
    }
    const maybeQuotedEnd = quotedLiteralEnd(source, index);
    if (maybeQuotedEnd !== null) {
      ignoredRanges.push({ start: index, end: maybeQuotedEnd });
      index = maybeQuotedEnd;
      continue;
    }
    if (/[A-Za-z_]/.test(current)) {
      const end = consumeWhile(source, index + 1, /[A-Za-z0-9_]/);
      tokens.push({
        kind: "identifier",
        value: source.slice(index, end),
        start: index,
        end,
      });
      index = end;
      continue;
    }
    if (/[0-9]/.test(current)) {
      const end = numberEnd(source, index);
      tokens.push({
        kind: "number",
        value: source.slice(index, end),
        start: index,
        end,
      });
      index = end;
      continue;
    }
    const punctuation = MULTI_PUNCTUATION.find((candidate) =>
      source.startsWith(candidate, index),
    );
    const value = punctuation ?? current;
    tokens.push({
      kind: "punct",
      value,
      start: index,
      end: index + value.length,
    });
    index += value.length;
  }
  return { tokens, ignoredRanges };
}

function numberEnd(source: string, start: number): number {
  let index = consumeWhile(source, start + 1, /[A-Za-z0-9_]/);
  while (
    source[index] === "." &&
    /[0-9]/.test(source[index + 1] ?? "")
  ) {
    index = consumeWhile(source, index + 2, /[A-Za-z0-9_]/);
  }
  return index;
}

function lineCommentEnd(source: string, start: number): number {
  const newline = source.indexOf("\n", start + 2);
  return newline < 0 ? source.length : newline;
}

function blockCommentEnd(source: string, start: number): number {
  let depth = 1;
  let index = start + 2;
  while (index < source.length && depth > 0) {
    if (source.startsWith("/*", index)) {
      depth += 1;
      index += 2;
    } else if (source.startsWith("*/", index)) {
      depth -= 1;
      index += 2;
    } else {
      index += 1;
    }
  }
  return index;
}

function rawLiteralEnd(source: string, start: number): number | null {
  for (const prefix of ["br", "cr", "r"] as const) {
    if (!source.startsWith(prefix, start)) {
      continue;
    }
    let cursor = start + prefix.length;
    let hashes = 0;
    while (source[cursor] === "#") {
      hashes += 1;
      cursor += 1;
    }
    if (source[cursor] !== '"') {
      continue;
    }
    const close = `"${"#".repeat(hashes)}`;
    const end = source.indexOf(close, cursor + 1);
    return end < 0 ? source.length : end + close.length;
  }
  return null;
}

function quotedLiteralEnd(source: string, start: number): number | null {
  let quoteIndex = start;
  if (
    (source[start] === "b" || source[start] === "c") &&
    (source[start + 1] === '"' || source[start + 1] === "'")
  ) {
    quoteIndex += 1;
  }
  const quote = source[quoteIndex];
  if (quote !== '"' && quote !== "'") {
    return null;
  }
  if (quote === "'" && isLifetime(source, quoteIndex)) {
    return null;
  }
  let escaped = false;
  for (let index = quoteIndex + 1; index < source.length; index += 1) {
    const current = source[index];
    if (escaped) {
      escaped = false;
    } else if (current === "\\") {
      escaped = true;
    } else if (current === quote) {
      return index + 1;
    }
  }
  return source.length;
}

function isLifetime(source: string, quote: number): boolean {
  if (!/[A-Za-z_]/.test(source[quote + 1] ?? "")) {
    return false;
  }
  const end = consumeWhile(source, quote + 2, /[A-Za-z0-9_]/);
  return source[end] !== "'";
}

function consumeWhile(source: string, start: number, pattern: RegExp): number {
  let index = start;
  while (index < source.length && pattern.test(source[index] ?? "")) {
    index += 1;
  }
  return index;
}
