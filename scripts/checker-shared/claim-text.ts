/** Wording that asserts a capability, matched against lower-cased clauses. */
export const POSITIVE_PATTERNS = [
  /\bsupports\b/,
  /\bprovides?\b/,
  /\benables?\b/,
  /\badds?\b/,
  /\bimplements?\b/,
  /\bships?\b/,
  /\bproves?\b/,
  /\bis supported\b/,
  /\bis enabled\b/,
  /\bis ready\b/,
] as const;

/** Wording that marks a clause as a denial or deferral rather than a claim. */
export const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "is not",
  "are not",
  "not a ",
  "must not",
  "not required",
  "not supported",
  "not run",
  "without claiming",
  "without making",
  "without turning",
  "without broadening",
  "outside",
  "out of scope",
  "deferred",
  "remain deferred",
  "remains deferred",
  "future",
  "future-gated",
  "no claim",
  "optional uat",
] as const;

const NO_CLAIM_TABLE_STATUSES = [
  "deferred",
  "unsupported",
  "not allowed",
  "not allowed yet",
  "not run",
];

/** Splits Markdown into prose paragraphs and single table rows, with 1-based start lines. */
export function markdownParagraphs(text: string): Array<{ startLine: number; text: string }> {
  const paragraphs: Array<{ startLine: number; text: string }> = [];
  let current: string[] = [];
  let startLine = 1;
  for (const [index, line] of text.split("\n").entries()) {
    if (line.trim().startsWith("|") && line.trim().endsWith("|")) {
      if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
      current = [];
      paragraphs.push({ startLine: index + 1, text: line.trim() });
      startLine = index + 2;
      continue;
    }
    if (line.trim() === "") {
      if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
      current = [];
      startLine = index + 2;
      continue;
    }
    if (current.length === 0) startLine = index + 1;
    current.push(line);
  }
  if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
  return paragraphs;
}

/** Splits a paragraph into sentences, or a table row into its non-empty cells. */
export function claimClauses(paragraph: string): string[] {
  if (paragraph.startsWith("|") && paragraph.endsWith("|")) {
    return paragraph
      .slice(1, -1)
      .split("|")
      .map((value) => value.trim())
      .filter((value) => value !== "");
  }
  return paragraph.split(/(?<=[.!?])\s+|\s+\|\s+/).filter((value) => value.trim() !== "");
}

/** True when a table row carries a status cell that denies or defers the capability. */
export function tableRowHasNoClaimStatus(paragraph: string): boolean {
  if (!paragraph.startsWith("|") || !paragraph.endsWith("|")) return false;
  return paragraph
    .slice(1, -1)
    .split("|")
    .map((value) => value.trim().toLowerCase().replaceAll("`", ""))
    .some((value) => NO_CLAIM_TABLE_STATUSES.includes(value));
}

/** Expects lower-cased text. */
export function hasPositiveClaim(text: string): boolean {
  return POSITIVE_PATTERNS.some((pattern) => pattern.test(text));
}

/** Expects lower-cased text. */
export function hasNoClaimMarker(text: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => text.includes(marker));
}
