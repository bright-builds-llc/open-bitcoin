/** Classify the selected owning impl from comment/literal-masked Rust. */
export function hasOrdinaryRuntimeOwner(code: string): boolean {
  const maybeOwner = /\bimpl\s+DurableSyncRuntime\s*$/.exec(code);
  if (maybeOwner === null) return false;
  let cursor = maybeOwner.index;
  // Only contiguous outer attributes belong to this implementation.
  while (cursor > 0) {
    while (cursor > 0 && /\s/.test(code[cursor - 1])) cursor -= 1;
    if (code[cursor - 1] !== "]") break;
    const end = cursor;
    let depth = 1;
    cursor -= 1;
    while (cursor > 0 && depth > 0) {
      cursor -= 1;
      if (code[cursor] === "]") depth += 1;
      if (code[cursor] === "[") depth -= 1;
    }
    if (depth !== 0) return false;
    const open = cursor;
    while (cursor > 0 && /\s/.test(code[cursor - 1])) cursor -= 1;
    if (code[cursor - 1] !== "#") return false;
    cursor -= 1;
    if (code.slice(open + 1, end - 1).replace(/\s+/g, "") === "cfg(test)") return false;
  }
  return true;
}
