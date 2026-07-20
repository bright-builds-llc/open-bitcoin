/**
 * Returns every initializer assigned to one exact Rust `let` binding.
 *
 * The scanner ignores comments and literals, then respects nested delimiters so
 * callers can validate the live expression instead of accepting a stray anchor.
 */
export function rustLetInitializers(text: string, binding: string): string[] {
  const code = stripRustNonCode(text);
  const escapedBinding = binding.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const declaration = new RegExp(
    String.raw`\blet\s+(?:mut\s+)?${escapedBinding}\s*=`,
    "g",
  );
  const initializers: string[] = [];
  for (const match of code.matchAll(declaration)) {
    const start = (match.index ?? 0) + match[0].length;
    const maybeInitializer = rustExpressionBefore(code, start, ";");
    if (maybeInitializer !== null) initializers.push(maybeInitializer.trim());
  }
  return initializers;
}

/**
 * Returns every initializer assigned to one exact Rust named field.
 */
export function rustFieldInitializers(text: string, field: string): string[] {
  const code = stripRustNonCode(text);
  const escapedField = field.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const fieldStart = new RegExp(String.raw`\b${escapedField}\s*:`, "g");
  const initializers: string[] = [];
  for (const match of code.matchAll(fieldStart)) {
    const start = (match.index ?? 0) + match[0].length;
    const maybeInitializer = rustExpressionBefore(code, start, ",");
    if (maybeInitializer !== null) initializers.push(maybeInitializer.trim());
  }
  return initializers;
}

/**
 * Returns the top-level arguments for every call to one exact Rust callee.
 */
export function rustCallArguments(text: string, callee: string): string[][] {
  const code = stripRustNonCode(text);
  const calls: string[][] = [];
  let searchStart = 0;
  while (searchStart < code.length) {
    const calleeStart = code.indexOf(callee, searchStart);
    if (calleeStart === -1) break;
    let open = calleeStart + callee.length;
    while (/\s/.test(code[open] ?? "")) open += 1;
    if (code[open] !== "(") {
      searchStart = open;
      continue;
    }
    const maybeClose = rustClosingDelimiter(code, open);
    if (maybeClose === null) break;
    calls.push(splitTopLevelRustArguments(code.slice(open + 1, maybeClose)));
    searchStart = maybeClose + 1;
  }
  return calls;
}

/**
 * Returns the methods called on a Rust type or any local alias of that type.
 */
export function rustAssociatedCallMethods(
  text: string,
  targetType: string,
): string[] {
  const code = stripRustNonCode(text);
  const typeNames = rustTypeAliases(code, targetType);
  const methods: string[] = [];
  for (const typeName of typeNames) {
    const escapedType = escapeRegExp(typeName);
    const associatedCall = new RegExp(
      String.raw`\b${escapedType}\s*(?:::<[^;{}()]*>)?::\s*([A-Za-z_]\w*)\s*\(`,
      "g",
    );
    for (const match of code.matchAll(associatedCall)) {
      methods.push(match[1] ?? "");
    }
  }
  return methods;
}

/**
 * Returns every top-level `match` discriminant in a Rust expression.
 */
export function rustMatchDiscriminants(text: string): string[] {
  const code = stripRustNonCode(text);
  const discriminants: string[] = [];
  for (const match of code.matchAll(/\bmatch\b/g)) {
    const start = (match.index ?? 0) + match[0].length;
    const delimiters: string[] = [];
    for (let cursor = start; cursor < code.length; cursor += 1) {
      const character = code[cursor] ?? "";
      if (character === "(" || character === "[") {
        delimiters.push(character);
        continue;
      }
      if (character === ")" || character === "]") {
        if (delimiters.length === 0) break;
        delimiters.pop();
        continue;
      }
      if (character === "{" && delimiters.length === 0) {
        discriminants.push(code.slice(start, cursor).trim());
        break;
      }
    }
  }
  return discriminants;
}

/**
 * Returns one field's top-level initializers from an exact Rust struct literal.
 */
export function rustStructLiteralFieldInitializers(
  expression: string,
  structType: string,
  field: string,
): string[] {
  const code = stripRustNonCode(expression).trim();
  const literalStart = new RegExp(`^${escapeRegExp(structType)}\\s*\\{`).exec(
    code,
  );
  if (literalStart === null) return [];
  const open = code.indexOf("{", literalStart.index);
  const maybeClose = rustClosingDelimiter(code, open);
  if (maybeClose === null || code.slice(maybeClose + 1).trim() !== "") return [];
  const fieldStart = new RegExp(`^${escapeRegExp(field)}\\s*:`);
  return splitTopLevelRustArguments(code.slice(open + 1, maybeClose))
    .filter((entry) => fieldStart.test(entry))
    .map((entry) => entry.slice(entry.indexOf(":") + 1).trim());
}

/**
 * Extracts one Rust function from its signature through its balanced body.
 */
export function rustFunction(text: string, signatureNeedle: string): string {
  const code = stripRustNonCode(text);
  const start = code.indexOf(signatureNeedle);
  if (start === -1) return "";
  const bodyStart = code.indexOf("{", start + signatureNeedle.length);
  if (bodyStart === -1) return "";
  let depth = 0;
  for (let cursor = bodyStart; cursor < code.length; cursor += 1) {
    if (code[cursor] === "{") depth += 1;
    if (code[cursor] !== "}") continue;
    depth -= 1;
    if (depth === 0) return code.slice(start, cursor + 1);
  }
  return "";
}

/**
 * Blanks Rust comments and literals while retaining byte offsets and newlines.
 */
export function stripRustNonCode(text: string): string {
  const stripped = text.split("");
  let cursor = 0;
  while (cursor < text.length) {
    if (text.startsWith("//", cursor)) {
      cursor = blankThrough(text, stripped, cursor, "\n");
      continue;
    }
    if (text.startsWith("/*", cursor)) {
      cursor = blankNestedBlockComment(text, stripped, cursor);
      continue;
    }
    const maybeRawStringEnd = rawStringEnd(text, cursor);
    if (maybeRawStringEnd !== null) {
      blankRange(text, stripped, cursor, maybeRawStringEnd);
      cursor = maybeRawStringEnd;
      continue;
    }
    if (
      text[cursor] === '"' ||
      (text[cursor] === "b" && text[cursor + 1] === '"')
    ) {
      const quote = text[cursor] === '"' ? cursor : cursor + 1;
      const end = quotedLiteralEnd(text, quote, '"');
      blankRange(text, stripped, cursor, end);
      cursor = end;
      continue;
    }
    if (text[cursor] === "'" && looksLikeCharLiteral(text, cursor)) {
      const end = quotedLiteralEnd(text, cursor, "'");
      blankRange(text, stripped, cursor, end);
      cursor = end;
      continue;
    }
    cursor += 1;
  }
  return stripped.join("");
}

/**
 * Removes insignificant whitespace for exact Rust expression comparisons.
 */
export function normalizeRust(text: string): string {
  return text.replace(/\s+/g, "");
}

function rustTypeAliases(code: string, targetType: string): Set<string> {
  const typeNames = new Set([targetType]);
  const aliases = [
    ...code.matchAll(
      /\btype\s+([A-Za-z_]\w*)(?:\s*<[^;=]+>)?\s*=\s*([^;]+);/g,
    ),
    ...code.matchAll(/\buse\s+([^;]+)\s+as\s+([A-Za-z_]\w*)\s*;/g),
  ].map((match) =>
    match[0].trimStart().startsWith("type ")
      ? { alias: match[1] ?? "", target: match[2] ?? "" }
      : { alias: match[2] ?? "", target: match[1] ?? "" },
  );
  let changed = true;
  while (changed) {
    changed = false;
    for (const alias of aliases) {
      if (
        ![...typeNames].some((typeName) =>
          new RegExp(`(?:^|::)${escapeRegExp(typeName)}(?:<|$)`).test(
            normalizeRust(alias.target),
          ),
        ) ||
        typeNames.has(alias.alias)
      ) {
        continue;
      }
      typeNames.add(alias.alias);
      changed = true;
    }
  }
  return typeNames;
}

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function splitTopLevelRustArguments(argumentsText: string): string[] {
  const argumentsList: string[] = [];
  const delimiters: string[] = [];
  let argumentStart = 0;
  for (let cursor = 0; cursor < argumentsText.length; cursor += 1) {
    const character = argumentsText[cursor] ?? "";
    if (character === "(" || character === "[" || character === "{") {
      delimiters.push(character);
      continue;
    }
    if (character === ")" || character === "]" || character === "}") {
      delimiters.pop();
      continue;
    }
    if (character === "," && delimiters.length === 0) {
      argumentsList.push(argumentsText.slice(argumentStart, cursor).trim());
      argumentStart = cursor + 1;
    }
  }
  const trailing = argumentsText.slice(argumentStart).trim();
  if (trailing.length > 0) argumentsList.push(trailing);
  return argumentsList;
}

function rustExpressionBefore(
  code: string,
  start: number,
  terminator: ";" | ",",
): string | null {
  const delimiters: string[] = [];
  for (let cursor = start; cursor < code.length; cursor += 1) {
    const character = code[cursor] ?? "";
    if (character === "(" || character === "[" || character === "{") {
      delimiters.push(character);
      continue;
    }
    if (character === ")" || character === "]" || character === "}") {
      if (delimiters.length === 0) return null;
      delimiters.pop();
      continue;
    }
    if (character === terminator && delimiters.length === 0) {
      return code.slice(start, cursor);
    }
  }
  return null;
}

function rustClosingDelimiter(code: string, open: number): number | null {
  const delimiters: string[] = [];
  const matchingOpen: Record<string, string> = {
    ")": "(",
    "]": "[",
    "}": "{",
  };
  for (let cursor = open; cursor < code.length; cursor += 1) {
    const character = code[cursor] ?? "";
    if (character === "(" || character === "[" || character === "{") {
      delimiters.push(character);
      continue;
    }
    if (character !== ")" && character !== "]" && character !== "}") continue;
    if (delimiters.pop() !== matchingOpen[character]) return null;
    if (delimiters.length === 0) return cursor;
  }
  return null;
}

function blankThrough(
  text: string,
  stripped: string[],
  start: number,
  terminator: string,
): number {
  const maybeEnd = text.indexOf(terminator, start);
  const end = maybeEnd === -1 ? text.length : maybeEnd;
  blankRange(text, stripped, start, end);
  return end;
}

function blankNestedBlockComment(
  text: string,
  stripped: string[],
  start: number,
): number {
  let cursor = start;
  let depth = 0;
  while (cursor < text.length) {
    if (text.startsWith("/*", cursor)) {
      depth += 1;
      cursor += 2;
      continue;
    }
    if (text.startsWith("*/", cursor)) {
      depth -= 1;
      cursor += 2;
      if (depth === 0) {
        blankRange(text, stripped, start, cursor);
        return cursor;
      }
      continue;
    }
    cursor += 1;
  }
  blankRange(text, stripped, start, text.length);
  return text.length;
}

function rawStringEnd(text: string, start: number): number | null {
  let cursor = start;
  if (text[cursor] === "b") cursor += 1;
  if (text[cursor] !== "r") return null;
  cursor += 1;
  const hashesStart = cursor;
  while (text[cursor] === "#") cursor += 1;
  if (cursor - hashesStart > 255 || text[cursor] !== '"') return null;
  const hashes = text.slice(hashesStart, cursor);
  const terminator = `"${hashes}`;
  const bodyStart = cursor + 1;
  const maybeEnd = text.indexOf(terminator, bodyStart);
  return maybeEnd === -1 ? text.length : maybeEnd + terminator.length;
}

function quotedLiteralEnd(
  text: string,
  quote: number,
  delimiter: '"' | "'",
): number {
  let cursor = quote + 1;
  while (cursor < text.length) {
    if (text[cursor] === "\\") {
      cursor += 2;
      continue;
    }
    if (text[cursor] === delimiter) return cursor + 1;
    cursor += 1;
  }
  return text.length;
}

function looksLikeCharLiteral(text: string, start: number): boolean {
  const maybeEnd = quotedLiteralEnd(text, start, "'");
  return maybeEnd - start <= 12 && text[maybeEnd - 1] === "'";
}

function blankRange(
  text: string,
  stripped: string[],
  start: number,
  end: number,
): void {
  for (let cursor = start; cursor < end; cursor += 1) {
    if (text[cursor] !== "\n" && text[cursor] !== "\r") {
      stripped[cursor] = " ";
    }
  }
}
