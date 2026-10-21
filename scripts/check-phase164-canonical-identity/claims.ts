const DOC = "docs/parity/v2-6-canonical-chain-identity.md";
export const CLAIM_FILES = [DOC, "README.md", "docs/parity/support-matrix.md", "docs/parity/index.json"];
export const QUALIFICATION_STATUS = /\b(?:qualified|proved|proven|passed|delivered|complete|established|supported|accepted|deterministic|ready|achieved|available)\b/gi;
const IDENTITY_ONLY_STATUS = /\b(?:qualified|supported|ready|proven|complete|accepted)\b/gi;
const WORK_SUBJECTS = /\b(?:Phase\s*16[678]|canonical(?=[^.!?;]*\b(?:bootstrap|reopen)\b)|ordinary[- ]history|bootstrap|reopen|(?:deterministic\s+)?(?:restart|reopen)(?=[^.!?;]*\b(?:tie|ordering|order)\b)|(?:actual\s+)?(?:pinned[- ]?)?Knots(?=[^.!?;]*\b(?:daemon|differential|qualification)\b))\b/gi;
const LEGACY_IDENTITY_SUBJECTS = /\b(?:bootstrap|reopen|whole.chain|production|Knots qualification)\b/gi;
const IDENTITY_CONTEXT_SUBJECTS = new RegExp(WORK_SUBJECTS.source + "|\\b(?:whole.chain|production)\\b", "gi");
const SUBJECT_QUALIFIERS = new RegExp("(?:" + QUALIFICATION_STATUS.source + "\\s*)+$", "i");

/** Current identity/refusal copy only; historical release sections retain their original scope. */
export function checkCurrentClaims(text: (file: string) => string, fail: (file: string, rule: string, detail: string) => void): void {
  const doc = text(DOC);
  for (const fact of ["identity-only", "165", "166", "167", "168", "unavailable", "signet_challenge_validation",
    "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
    "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943",
    "00000008819873e925422c1ff0f99f7cc9bbb232af63a077a480a3633bee1ef6",
    "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206"]) {
    if (!doc.includes(fact)) fail(DOC, "evidence", `required identity/refusal fact ${fact}`);
  }
  rejectClaims(doc, DOC, fail, identityDocumentSentences);
  try {
    const index = JSON.parse(text("docs/parity/index.json"));
    const owners = index.checklist.surfaces.filter((row: { id: string }) => row.id === "v2-6-canonical-chain-identity");
    if (owners.length !== 1 || !["in_progress", "done"].includes(owners[0].status)) fail("docs/parity/index.json", "evidence", "unique current identity surface required");
    if (owners.length === 1) {
      if (JSON.stringify(owners[0].requirements) !== '["CBOT-01","CBOT-04","CVLD-05"]') fail("docs/parity/index.json", "evidence", "complete current requirement scope required");
      rejectClaims(JSON.stringify(owners[0]), "docs/parity/index.json", fail);
    }
  } catch { fail("docs/parity/index.json", "input", "invalid current parity catalog"); }
  for (const [file, heading] of [["README.md", "## Current source: canonical identity and ordinary refusal"],
    ["docs/parity/support-matrix.md", "## Current Phase 164 source boundary"]]) {
    const section = text(file).split(heading)[1]?.split(/\n## /)[0] ?? "";
    if (!section.includes("identity-only") || !section.replaceAll("\n", " ").includes("currently refused for all four selectors")) fail(file, "evidence", "current identity-only refusal section required");
    rejectClaims(section, file, fail);
    for (const current of text(file).split("## Current Phase 166 canonical genesis boundary").slice(1)) {
      rejectClaims(current.split(/\n## /)[0], file, fail);
    }
  }
}

function rejectClaims(source: string, file: string, fail: (file: string, rule: string, detail: string) => void,
  sentences: (source: string) => readonly string[] = identitySectionSentences): void {
  for (const _claim of identityClaims(source, sentences)) fail(file, "claim", "unsupported current canonical qualification claim");
}

/** A single explicit present-genesis clause cannot qualify another subject or phase. */
export function boundedGenesisClaim(source: string): boolean {
  const plain = source.replace(/[`*_#>]/g, "").trim().replace(/\s+/g, " ");
  return /^(?:Phase\s*166\s+)?canonical\s+genesis(?:-only)?\s+(?:bootstrap(?:\s+and\s+reopen)?|reopen)\s+(?:is|are)\s+(?:supported|available|proved|proven|accepted)(?:\s+for\s+all\s+four\s+selectors)?$/i.test(plain);
}

/** Conditional readiness concerns its immediately following subject, never another clause. */
export function conditionalSubjectPrefix(source: string): boolean {
  return /\b(?:once|when|if)\s+(?:(?:a|an|the|built|configured|pinned|actual)\s+)*$/i.test(source);
}

/** Every known subject owns only its following predicates, never unrelated context. */
export function affirmativeScopeClaims(source: string): { phase: boolean }[] {
  return workScopeClaims(source);
}

/** Current identity sections retain their original generic production/whole-chain subjects. */
export function currentQualificationClaims(source: string): { phase: boolean }[] {
  return identityClaims(source, identitySectionSentences);
}

/** Only an exact new heading immediately owning a bounded genesis clause starts a new sentence. */
function withoutGenesisHeading(source: string): string {
  const lines = source.split("\n");
  return lines.map((line, index) => {
    if (line.trimEnd() !== "## Current Phase 166 canonical genesis boundary") return line;
    let next = index + 1;
    while (next < lines.length && !lines[next].trim()) next++;
    const paragraph: string[] = [];
    while (next < lines.length && lines[next].trim() && !lines[next].startsWith("## ")) paragraph.push(lines[next++]);
    const firstClause = paragraph.join(" ").split(/[.!?;](?:\s|$)/)[0];
    return boundedGenesisClaim(firstClause) ? "." : line;
  }).join("\n");
}

function identityDocumentSentences(source: string): readonly string[] {
  return withoutGenesisHeading(source).split(/\n\s*\n/).flatMap(paragraph => paragraph.replaceAll("\n", " ").split(/[.!?](?:\s|$)/));
}

function identitySectionSentences(source: string): readonly string[] {
  return withoutGenesisHeading(source).split(/\n\s*\n/).flatMap(paragraph => paragraph.replaceAll("\n", " ").split(/[.!?;](?:\s|$)/));
}

function hasAttachedNo(source: string): boolean {
  const prefix = source.replace(SUBJECT_QUALIFIERS, "");
  return /\b(?:no|never|without|(?:do|does)\s+not\s+(?:claim|grant|provide|confer|establish|qualify))\s+(?:(?:actual|positive|deterministic|whole-chain|production-service|production-funds|public-default|networking|and|or)[,\s-]*)*$/i.test(prefix);
}

function qualificationDenied(before: string, attachedNo: boolean, conditional: boolean): boolean {
  const localPrefix = before.split(/\band\b|,/i).at(-1) ?? "";
  return conditional || /\b(?:not|never)\s*(?:yet\s*)?$/i.test(localPrefix)
    || /\bnot\s+(?:an?\s+)?(?:evidence|proof|claim)\s+for\s+(?:a\s+)?$/i.test(localPrefix)
    || /\b(?:only after|until)\b/i.test(localPrefix)
    || /\b(?:remains?|is|are)\s+deferred\b/i.test(localPrefix)
    || (attachedNo && !/\band\b|,/i.test(before));
}

type SubjectOwner = { start: number; end: number; phase: boolean; attachedNo: boolean; conditional: boolean };

function subjectOwners(plain: string, subjects: RegExp): readonly SubjectOwner[] {
  const targets = [...plain.matchAll(subjects)];
  const owners: SubjectOwner[] = [];
  let phaseScope = false;
  let inheritedNo = false;
  for (let index = 0; index < targets.length; index++) {
    const target = targets[index];
    const maybePrevious = targets[index - 1];
    const connector = maybePrevious ? plain.slice(maybePrevious.index + maybePrevious[0].length, target.index) : "";
    const sharesPredicate = !/\b(?:is|are|was|were|has|have|will|can|does|do)\b/i.test(connector);
    phaseScope = /^Phase\b/i.test(target[0]) || (phaseScope && sharesPredicate);
    const prefix = plain.slice(0, target.index);
    const attachedNo = (inheritedNo && sharesPredicate) || hasAttachedNo(prefix);
    inheritedNo = attachedNo;
    owners.push({ start: target.index, end: target.index + target[0].length, phase: phaseScope,
      attachedNo, conditional: conditionalSubjectPrefix(prefix.replace(SUBJECT_QUALIFIERS, "")) });
  }
  return owners;
}

/** Original165 flattens all newlines, then selects each subject's following predicates. */
function workScopeClaims(source: string): { phase: boolean }[] {
  const claims: { phase: boolean }[] = [];
  for (const sentence of withoutGenesisHeading(source).replaceAll("\n", " ").split(/[.!?;](?:\s|$)|\b(?:but|however)\b/i)) {
    if (boundedGenesisClaim(sentence)) continue;
    const plain = sentence.replace(/[`*_#>]/g, "");
    const owners = subjectOwners(plain, WORK_SUBJECTS);
    for (const [index, owner] of owners.entries()) {
      const predicate = plain.slice(owner.end, owners[index + 1]?.start ?? plain.length);
      for (const status of predicate.matchAll(QUALIFICATION_STATUS)) {
        const before = predicate.slice(0, status.index);
        if (!qualificationDenied(before, owner.attachedNo, owner.conditional)) claims.push({ phase: owner.phase });
      }
    }
  }
  return claims;
}

function qualificationClause(sentence: string, position: number): { prefix: string; raw: string } {
  let start = 0;
  let end = sentence.length;
  for (const boundary of sentence.matchAll(/,|\b(?:and|but|however|while)\b/gi)) {
    if (boundary.index < position) start = boundary.index + boundary[0].length;
    else { end = boundary.index; break; }
  }
  return { prefix: sentence.slice(start, position), raw: sentence.slice(start, end) };
}

/** Original164's five subjects and six statuses co-occur in either order. */
function legacyIdentityPositive(sentence: string): boolean {
  const plain = sentence.replace(/[`*_#>]/g, "");
  const subjects = [...plain.matchAll(LEGACY_IDENTITY_SUBJECTS)];
  const owners = subjectOwners(plain, IDENTITY_CONTEXT_SUBJECTS);
  for (const subject of subjects) {
    const maybeOwner = owners.find(owner => subject.index >= owner.start && subject.index < owner.end);
    for (const status of plain.matchAll(IDENTITY_ONLY_STATUS)) {
      const left = Math.min(subject.index, status.index);
      const right = Math.max(subject.index, status.index);
      if (subjects.some(other => other !== subject && other.index > left && other.index < right)) continue;
      const clause = qualificationClause(plain, status.index);
      const denied = qualificationDenied(clause.prefix, (maybeOwner?.attachedNo ?? false) || hasAttachedNo(clause.prefix),
        (maybeOwner?.conditional ?? false) || conditionalSubjectPrefix(clause.prefix));
      if (!denied && !/\b(?:missing|until|future|deferred|only after)\b/i.test(clause.raw)) return true;
    }
  }
  return false;
}

function identityClaims(source: string, sentences: (source: string) => readonly string[]): { phase: boolean }[] {
  const claims: { phase: boolean }[] = [];
  for (const sentence of sentences(source)) {
    if (boundedGenesisClaim(sentence)) continue;
    const following = workScopeClaims(sentence);
    if (following.length) claims.push(...following);
    else if (legacyIdentityPositive(sentence)) claims.push({ phase: false });
  }
  return claims;
}
