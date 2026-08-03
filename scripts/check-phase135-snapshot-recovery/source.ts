function maskNonCode(source: string): string {
  const chars = [...source];
  let state: "code" | "line" | "block" | "string" | "char" | "raw" =
    "code";
  let blockDepth = 0;
  let rawHashCount = 0;
  let escaped = false;
  for (let index = 0; index < chars.length; index += 1) {
    const current = chars[index];
    const next = chars[index + 1] ?? "";
    if (state === "code") {
      if (current === "/" && next === "/") {
        state = "line";
        chars[index] = chars[index + 1] = " ";
        index += 1;
      } else if (current === "/" && next === "*") {
        state = "block";
        blockDepth = 1;
        chars[index] = chars[index + 1] = " ";
        index += 1;
      } else if (current === "r") {
        let delimiterEnd = index + 1;
        while (chars[delimiterEnd] === "#") delimiterEnd += 1;
        if (chars[delimiterEnd] === '"') {
          state = "raw";
          rawHashCount = delimiterEnd - index - 1;
          for (let offset = index; offset <= delimiterEnd; offset += 1) {
            chars[offset] = " ";
          }
          index = delimiterEnd;
        }
      } else if (current === '"') {
        state = "string";
        chars[index] = " ";
      } else if (
        current === "'" &&
        (chars[index + 2] === "'" ||
          (next === "\\" && chars[index + 3] === "'"))
      ) {
        state = "char";
        chars[index] = " ";
      }
      continue;
    }
    if (current === "\n" && state === "line") {
      state = "code";
      continue;
    }
    if (state === "block") {
      if (current === "/" && next === "*") {
        blockDepth += 1;
        chars[index + 1] = " ";
        index += 1;
      } else if (current === "*" && next === "/") {
        blockDepth -= 1;
        chars[index + 1] = " ";
        index += 1;
        if (blockDepth === 0) state = "code";
      }
      if (current !== "\n") chars[index] = " ";
      continue;
    }
    if (state === "line") {
      chars[index] = " ";
      continue;
    }
    if (state === "raw") {
      const closingHashes = chars.slice(
        index + 1,
        index + 1 + rawHashCount,
      );
      const closes =
        current === '"' &&
        closingHashes.length === rawHashCount &&
        closingHashes.every((character) => character === "#");
      if (current !== "\n") chars[index] = " ";
      if (closes) {
        for (let offset = 1; offset <= rawHashCount; offset += 1) {
          chars[index + offset] = " ";
        }
        index += rawHashCount;
        state = "code";
      }
      continue;
    }
    if (escaped) {
      escaped = false;
    } else if (current === "\\") {
      escaped = true;
    } else if (
      (state === "string" && current === '"') ||
      (state === "char" && current === "'")
    ) {
      state = "code";
    }
    if (current !== "\n") chars[index] = " ";
  }
  return chars.join("");
}

export function body(source: string, marker: string): string {
  const masked = maskNonCode(source);
  const start = masked.indexOf(marker);
  if (start < 0) return "";
  const brace = masked.indexOf("{", start);
  if (brace < 0) return "";
  let depth = 0;
  for (let index = brace; index < masked.length; index += 1) {
    if (masked[index] === "{") depth += 1;
    if (masked[index] !== "}") continue;
    depth -= 1;
    if (depth === 0) return masked.slice(brace + 1, index);
  }
  return "";
}

export function directStatementIndex(
  maskedBody: string,
  statement: string,
): number {
  if (statement.length === 0) return -1;
  let depth = 0;
  let firstIndex = -1;
  for (let index = 0; index < maskedBody.length; index += 1) {
    const current = maskedBody[index];
    if (current === "{") {
      depth += 1;
      continue;
    }
    if (current === "}") {
      if (depth === 0) return -1;
      depth -= 1;
      continue;
    }
    if (
      depth === 0 &&
      firstIndex < 0 &&
      maskedBody.startsWith(statement, index) &&
      beginsDirectStatement(maskedBody, index)
    ) {
      firstIndex = index;
    }
  }
  return depth === 0 ? firstIndex : -1;
}

function beginsDirectStatement(source: string, index: number): boolean {
  for (let cursor = index - 1; cursor >= 0; cursor -= 1) {
    if (source[cursor] === "\n") return true;
    if (!/\s/.test(source[cursor])) return false;
  }
  return true;
}

export function exactStructFields(source: string, marker: string): string[] {
  const fields = body(source, marker).matchAll(
    /^\s*(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:/gm,
  );
  return [...fields].map((match) => match[1]).sort();
}

export function sameFields(
  actual: string[],
  expected: readonly string[],
): boolean {
  return (
    actual.length === expected.length &&
    actual.every((field, index) => field === [...expected].sort()[index])
  );
}

export function hasAll(source: string, markers: readonly string[]): boolean {
  return markers.every((marker) => source.includes(marker));
}

export function addFailure(
  failures: string[],
  failed: boolean,
  diagnostic: string,
): void {
  if (failed && !failures.includes(diagnostic)) failures.push(diagnostic);
}

export function count(source: string, marker: string): number {
  return source.split(marker).length - 1;
}
