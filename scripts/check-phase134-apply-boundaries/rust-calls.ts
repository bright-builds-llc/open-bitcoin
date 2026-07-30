import { type RustToken, tokenizeRust } from "./rust-lexer";

export type ScannedMethodCall = {
  receiver: string;
  name: string;
  index: number;
};

export type ScannedFunctionCall = {
  path: string;
  name: string;
  index: number;
};

export type RustScan = {
  tokens: RustToken[];
  methodCalls: ScannedMethodCall[];
  functionCalls: ScannedFunctionCall[];
  unknownCallLikes: string[];
};

const CALL_SYNTAX_KEYWORDS = new Set([
  "break",
  "continue",
  "for",
  "if",
  "let",
  "loop",
  "match",
  "move",
  "return",
  "unsafe",
  "while",
  "yield",
]);

export function scanRust(source: string): RustScan {
  const tokens = tokenizeRust(source);
  const { openByClose } = delimiterPairMaps(tokens);
  const methodCalls: ScannedMethodCall[] = [];
  const functionCalls: ScannedFunctionCall[] = [];
  const unknownCallLikes: string[] = [];

  for (let open = 0; open < tokens.length; open += 1) {
    if (tokens[open]?.value !== "(") {
      continue;
    }
    if (isFunctionParameterList(tokens, open)) {
      continue;
    }
    if (tokens[open - 1]?.value === ")") {
      const maybeCall = parenthesizedFunctionCall(
        tokens,
        open - 1,
        openByClose,
      );
      if (!maybeCall) {
        unknownCallLikes.push("unparsed parenthesized call");
      } else if (
        !CALL_SYNTAX_KEYWORDS.has(maybeCall.name) &&
        !/^[A-Z]/.test(maybeCall.name)
      ) {
        functionCalls.push(maybeCall);
      }
      continue;
    }
    const maybeCalleeEnd = calleeEndBeforeTurbofish(tokens, open);
    if (maybeCalleeEnd === null) {
      if (tokens[open - 1]?.value === ">") {
        unknownCallLikes.push("unparsed generic call");
      }
      continue;
    }
    const nameToken = tokens[maybeCalleeEnd];
    if (nameToken?.kind !== "identifier") {
      continue;
    }
    if (tokens[maybeCalleeEnd - 1]?.value === ".") {
      const receiver = receiverBeforeDot(
        tokens,
        maybeCalleeEnd - 2,
        openByClose,
      );
      if (!receiver) {
        unknownCallLikes.push(`unparsed receiver for ${nameToken.value}`);
        continue;
      }
      methodCalls.push({
        receiver,
        name: nameToken.value,
        index: nameToken.start,
      });
      continue;
    }
    if (CALL_SYNTAX_KEYWORDS.has(nameToken.value)) {
      continue;
    }
    const path = pathEndingAt(tokens, maybeCalleeEnd);
    if (!path) {
      unknownCallLikes.push(`unparsed call ${nameToken.value}`);
      continue;
    }
    if (/^[A-Z]/.test(nameToken.value)) {
      continue;
    }
    functionCalls.push({
      path,
      name: nameToken.value,
      index: nameToken.start,
    });
  }
  return { tokens, methodCalls, functionCalls, unknownCallLikes };
}

function isFunctionParameterList(tokens: RustToken[], open: number): boolean {
  const end = open - 1;
  if (tokens[end]?.kind === "identifier") {
    return tokens[end - 1]?.value === "fn";
  }
  if (tokens[end]?.value !== ">") {
    return false;
  }
  let depth = 0;
  for (let index = end; index >= 0; index -= 1) {
    if (tokens[index]?.value === ">") {
      depth += 1;
    } else if (tokens[index]?.value === "<") {
      depth -= 1;
      if (depth === 0) {
        return (
          tokens[index - 1]?.kind === "identifier" &&
          tokens[index - 2]?.value === "fn"
        );
      }
    }
  }
  return false;
}

function parenthesizedFunctionCall(
  tokens: RustToken[],
  close: number,
  openByClose: Map<number, number>,
): ScannedFunctionCall | null {
  const maybeOpen = openByClose.get(close);
  if (maybeOpen === undefined || isCallDelimiter(tokens, maybeOpen)) {
    return null;
  }
  const inner = tokens.slice(maybeOpen + 1, close);
  const nameToken = inner.at(-1);
  if (nameToken?.kind !== "identifier") {
    return null;
  }
  const path = pathEndingAt(inner, inner.length - 1);
  if (!path || path.split("::").length * 2 - 1 !== inner.length) {
    return null;
  }
  return {
    path,
    name: nameToken.value,
    index: nameToken.start,
  };
}

function delimiterPairMaps(tokens: RustToken[]): {
  openByClose: Map<number, number>;
} {
  const stack: Array<{ value: string; index: number }> = [];
  const openByClose = new Map<number, number>();
  const matchingOpen = new Map([
    [")", "("],
    ["]", "["],
    ["}", "{"],
  ]);
  for (let index = 0; index < tokens.length; index += 1) {
    const value = tokens[index]?.value ?? "";
    if (["(", "[", "{"].includes(value)) {
      stack.push({ value, index });
      continue;
    }
    const expected = matchingOpen.get(value);
    if (!expected) {
      continue;
    }
    const maybeOpen = stack.pop();
    if (maybeOpen?.value === expected) {
      openByClose.set(index, maybeOpen.index);
    }
  }
  return { openByClose };
}

function calleeEndBeforeTurbofish(
  tokens: RustToken[],
  open: number,
): number | null {
  const end = open - 1;
  if (tokens[end]?.value !== ">") {
    return end;
  }
  let depth = 0;
  for (let index = end; index >= 0; index -= 1) {
    if (tokens[index]?.value === ">") {
      depth += 1;
    } else if (tokens[index]?.value === "<") {
      depth -= 1;
      if (depth === 0) {
        return tokens[index - 1]?.value === "::" ? index - 2 : null;
      }
    }
  }
  return null;
}

function receiverBeforeDot(
  tokens: RustToken[],
  end: number,
  openByClose: Map<number, number>,
): string | null {
  if (end < 0) {
    return null;
  }
  if (tokens[end]?.value === ")") {
    const maybeOpen = openByClose.get(end);
    if (maybeOpen === undefined || isCallDelimiter(tokens, maybeOpen)) {
      return null;
    }
    return normalizeReceiver(tokens.slice(maybeOpen + 1, end));
  }
  if (!isReceiverSegment(tokens[end])) {
    return null;
  }
  let start = end;
  while (
    start >= 2 &&
    tokens[start - 1]?.value === "." &&
    isReceiverSegment(tokens[start - 2])
  ) {
    start -= 2;
  }
  return normalizeReceiver(tokens.slice(start, end + 1));
}

function isCallDelimiter(tokens: RustToken[], open: number): boolean {
  const before = tokens[open - 1];
  return (
    before?.kind === "identifier" ||
    before?.value === ")" ||
    before?.value === "]" ||
    before?.value === ">"
  );
}

function normalizeReceiver(tokens: RustToken[]): string | null {
  let start = 0;
  while (["&", "*", "raw", "mut"].includes(tokens[start]?.value ?? "")) {
    start += 1;
  }
  const remaining = tokens.slice(start);
  if (
    remaining.length === 0 ||
    remaining[0]?.kind !== "identifier" ||
    remaining.some(
      (token, index) =>
        (index % 2 === 0 && !isReceiverSegment(token)) ||
        (index % 2 === 1 && token.value !== "."),
    )
  ) {
    return null;
  }
  return remaining.map(({ value }) => value).join("");
}

function isReceiverSegment(token: RustToken | undefined): boolean {
  return token?.kind === "identifier" || token?.kind === "number";
}

function pathEndingAt(tokens: RustToken[], end: number): string | null {
  if (tokens[end]?.kind !== "identifier") {
    return null;
  }
  let start = end;
  while (
    start >= 2 &&
    tokens[start - 1]?.value === "::" &&
    tokens[start - 2]?.kind === "identifier"
  ) {
    start -= 2;
  }
  return tokens
    .slice(start, end + 1)
    .map(({ value }) => value)
    .join("");
}
