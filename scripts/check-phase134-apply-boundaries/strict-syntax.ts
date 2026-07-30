const PURE_MACRO_ALLOWLIST = new Set<string>();

const ASSIGNMENT_OPERATOR =
  /(?:<<=|>>=|\+=|-=|\*=|\/=|%=|&=|\|=|\^=|(?<![=!<>])=(?!=|>))/;

export function strictSyntaxViolations(
  source: string,
  allowedMutableBorrowTargets: ReadonlySet<string> = new Set(),
): string[] {
  const violations: string[] = [];
  for (const macro of macroInvocations(source)) {
    if (!PURE_MACRO_ALLOWLIST.has(macro)) {
      violations.push(`unresolved macro invocation ${macro}!`);
    }
  }
  if (hasDirectAssignmentMutation(source)) {
    violations.push("direct mutation outside aggregate transaction");
  }
  for (const target of mutableBorrowTargets(source)) {
    if (!allowedMutableBorrowTargets.has(target)) {
      violations.push(`mutable borrow ${target} outside aggregate transaction`);
    }
  }
  if (/\b(?:async|unsafe)\s*(?:move\s*)?\{/.test(source)) {
    violations.push("unresolved async or unsafe block");
  }
  if (/\bmove\s*\|[^|\n]*\||\|[^|\n]*\|/.test(source)) {
    violations.push("unresolved closure");
  }
  if (/\b(?:if|match|for|while|loop|return|break|continue)\b/.test(source)) {
    violations.push("unresolved control flow");
  }
  return violations;
}

export function hasDirectAssignmentMutation(source: string): boolean {
  return source
    .split(";")
    .map((statement) => statement.replace(/^[\s{}]+/, "").trim())
    .filter(Boolean)
    .some((statement) => {
      if (!/^(?:let|if\s+let|while\s+let)\b/.test(statement)) {
        return ASSIGNMENT_OPERATOR.test(statement);
      }
      const binding = ASSIGNMENT_OPERATOR.exec(statement);
      return binding
        ? ASSIGNMENT_OPERATOR.test(
            statement.slice(binding.index + binding[0].length),
          )
        : false;
    });
}

function macroInvocations(source: string): string[] {
  const macros = new Set<string>();
  for (const match of source.matchAll(
    /\b([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)\s*!\s*[\(\[{]/g,
  )) {
    const macro = match[1];
    if (macro) {
      macros.add(macro);
    }
  }
  return [...macros];
}

function mutableBorrowTargets(source: string): string[] {
  const targets = new Set<string>();
  for (const match of source.matchAll(
    /&\s*(?:raw\s+)?mut\s+((?:self|[a-z_][A-Za-z0-9_]*)(?:\s*\.\s*[A-Za-z_][A-Za-z0-9_]*)*)/g,
  )) {
    const target = match[1]?.replace(/\s+/g, "");
    if (target) {
      targets.add(target);
    }
  }
  return [...targets];
}
