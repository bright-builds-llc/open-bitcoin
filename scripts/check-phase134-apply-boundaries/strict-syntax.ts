import { scanRust } from "./rust-calls";
import type { RustToken } from "./rust-lexer";

export {
  type ScannedFunctionCall,
  type ScannedMethodCall,
  type RustScan,
  scanRust,
} from "./rust-calls";
export {
  maskRustCommentsAndLiterals,
  type RustToken,
  tokenizeRust,
} from "./rust-lexer";
export {
  provenCollectionMutationSymbol,
  provenPureReceiverSymbol,
} from "./receiver-evidence";

const PURE_MACRO_ALLOWLIST = new Set<string>();
const ASSIGNMENT_OPERATORS = new Set([
  "=",
  "+=",
  "-=",
  "*=",
  "/=",
  "%=",
  "&=",
  "|=",
  "^=",
  "<<=",
  ">>=",
]);
const CONTROL_FLOW_KEYWORDS = new Set([
  "break",
  "continue",
  "for",
  "if",
  "loop",
  "match",
  "return",
  "while",
  "yield",
]);

export function strictSyntaxViolations(
  source: string,
  allowedMutableBorrowTargets: ReadonlySet<string> = new Set(),
): string[] {
  const scan = scanRust(source);
  const violations: string[] = [];
  for (const macro of macroInvocations(scan.tokens)) {
    if (!PURE_MACRO_ALLOWLIST.has(macro)) {
      violations.push(`unresolved macro invocation ${macro}!`);
    }
  }
  if (hasDirectAssignmentMutation(scan.tokens)) {
    violations.push("direct mutation outside aggregate transaction");
  }
  for (const target of mutableBorrowTargets(scan.tokens)) {
    if (!allowedMutableBorrowTargets.has(target)) {
      violations.push(`mutable borrow ${target} outside aggregate transaction`);
    }
  }
  if (hasAsyncOrUnsafeBlock(scan.tokens)) {
    violations.push("unresolved async or unsafe block");
  }
  if (hasClosure(scan.tokens)) {
    violations.push("unresolved closure");
  }
  if (scan.tokens.some(({ value }) => CONTROL_FLOW_KEYWORDS.has(value))) {
    violations.push("unresolved control flow");
  }
  violations.push(...scan.unknownCallLikes);
  return violations;
}

function macroInvocations(tokens: RustToken[]): string[] {
  const macros = new Set<string>();
  for (let index = 0; index < tokens.length; index += 1) {
    if (
      tokens[index]?.value !== "!" ||
      !["(", "[", "{"].includes(tokens[index + 1]?.value ?? "")
    ) {
      continue;
    }
    const path = pathEndingAt(tokens, index - 1);
    if (path) {
      macros.add(path);
    }
  }
  return [...macros];
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

function hasDirectAssignmentMutation(tokens: RustToken[]): boolean {
  const depths = delimiterDepths(tokens);
  const functionItemClosures = functionItemBodyClosures(tokens);
  return tokens.some(
    ({ value }, index) =>
      ASSIGNMENT_OPERATORS.has(value) &&
      !(
        value === "=" &&
        isBindingAssignment(tokens, depths, functionItemClosures, index)
      ),
  );
}

function isBindingAssignment(
  tokens: RustToken[],
  depths: number[],
  functionItemClosures: ReadonlySet<number>,
  assignment: number,
): boolean {
  const depth = depths[assignment];
  let start = assignment - 1;
  while (start >= 0 && (depths[start] ?? 0) >= (depth ?? 0)) {
    if (
      depths[start] === depth &&
      (tokens[start]?.value === ";" || functionItemClosures.has(start))
    ) {
      break;
    }
    start -= 1;
  }
  const statement = tokens.slice(start + 1, assignment);
  const first = statement[0]?.value;
  if (first === "type" || first === "fn") {
    return true;
  }
  if (!["let", "const", "static"].includes(first ?? "")) {
    return false;
  }
  if (first === "static" && statement[1]?.value === "mut") {
    return false;
  }
  return !statement.some(({ value }) => ASSIGNMENT_OPERATORS.has(value));
}

function functionItemBodyClosures(tokens: RustToken[]): Set<number> {
  const closures = new Set<number>();
  for (let index = 0; index < tokens.length; index += 1) {
    if (
      tokens[index]?.value !== "fn" ||
      tokens[index + 1]?.kind !== "identifier"
    ) {
      continue;
    }
    const maybeOpen = functionItemBodyOpen(tokens, index);
    if (maybeOpen === null) {
      continue;
    }
    const maybeClose = matchingClosingBrace(tokens, maybeOpen);
    if (maybeClose !== null) {
      closures.add(maybeClose);
    }
  }
  return closures;
}

function functionItemBodyOpen(
  tokens: RustToken[],
  functionToken: number,
): number | null {
  let parentheses = 0;
  let brackets = 0;
  let angles = 0;
  let signatureBraces = 0;
  for (let index = functionToken + 1; index < tokens.length; index += 1) {
    const value = tokens[index]?.value;
    if (signatureBraces > 0) {
      if (value === "{") {
        signatureBraces += 1;
      } else if (value === "}") {
        signatureBraces -= 1;
      }
      continue;
    }
    if (value === "(") {
      parentheses += 1;
    } else if (value === ")") {
      parentheses -= 1;
    } else if (value === "[") {
      brackets += 1;
    } else if (value === "]") {
      brackets -= 1;
    } else if (value === "<") {
      angles += 1;
    } else if (value === ">" && angles > 0) {
      angles -= 1;
    } else if (value === "{") {
      if (parentheses === 0 && brackets === 0 && angles === 0) {
        return index;
      }
      signatureBraces = 1;
    } else if (
      value === ";" &&
      parentheses === 0 &&
      brackets === 0 &&
      angles === 0
    ) {
      return null;
    }
  }
  return null;
}

function matchingClosingBrace(
  tokens: RustToken[],
  open: number,
): number | null {
  let depth = 0;
  for (let index = open; index < tokens.length; index += 1) {
    if (tokens[index]?.value === "{") {
      depth += 1;
    } else if (tokens[index]?.value === "}") {
      depth -= 1;
      if (depth === 0) {
        return index;
      }
    }
  }
  return null;
}

function delimiterDepths(tokens: RustToken[]): number[] {
  let parentheses = 0;
  let brackets = 0;
  let braces = 0;
  return tokens.map(({ value }) => {
    if (value === ")") {
      parentheses -= 1;
    } else if (value === "]") {
      brackets -= 1;
    } else if (value === "}") {
      braces -= 1;
    }
    const depth = parentheses + brackets + braces;
    if (value === "(") {
      parentheses += 1;
    } else if (value === "[") {
      brackets += 1;
    } else if (value === "{") {
      braces += 1;
    }
    return depth;
  });
}

function mutableBorrowTargets(tokens: RustToken[]): string[] {
  const targets = new Set<string>();
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]?.value !== "&") {
      continue;
    }
    let target = index + 1;
    if (tokens[target]?.value === "raw") {
      target += 1;
    }
    if (tokens[target]?.value !== "mut") {
      continue;
    }
    const receiver = receiverFromStart(tokens, target + 1);
    if (receiver) {
      targets.add(receiver);
    }
  }
  return [...targets];
}

function receiverFromStart(tokens: RustToken[], start: number): string | null {
  const receiver: RustToken[] = [];
  let index = tokens[start]?.value === "(" ? start + 1 : start;
  while (index < tokens.length) {
    const token = tokens[index];
    if (
      (receiver.length % 2 === 0 && token?.kind === "identifier") ||
      (receiver.length % 2 === 1 && token?.value === ".")
    ) {
      receiver.push(token);
      index += 1;
    } else {
      break;
    }
  }
  return receiver.length > 0
    ? receiver.map(({ value }) => value).join("")
    : null;
}

function hasAsyncOrUnsafeBlock(tokens: RustToken[]): boolean {
  return tokens.some(({ value }, index) => {
    if (value === "unsafe") {
      return tokens[index + 1]?.value === "{";
    }
    if (value !== "async") {
      return false;
    }
    const next = tokens[index + 1]?.value;
    return next === "{" || (next === "move" && tokens[index + 2]?.value === "{");
  });
}

function hasClosure(tokens: RustToken[]): boolean {
  return tokens.some(
    ({ value }, index) =>
      (value === "|" || value === "||") &&
      isExpressionStart(tokens[index - 1]?.value),
  );
}

function isExpressionStart(previous: string | undefined): boolean {
  return (
    previous === undefined ||
    ["=", "(", "[", "{", ",", ";", "=>", "move", "async"].includes(
      previous,
    )
  );
}
