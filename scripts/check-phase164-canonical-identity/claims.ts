const DOC = "docs/parity/v2-6-canonical-chain-identity.md";
export const CLAIM_FILES = [DOC, "README.md", "docs/parity/support-matrix.md", "docs/parity/index.json"];

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
  for (const sentence of doc.split(/\n\s*\n/).flatMap(paragraph => paragraph.replaceAll("\n", " ").split(/[.!?](?:\s|$)/))) {
    if (/\b(?:bootstrap|reopen|whole.chain|production|Knots qualification)\b/i.test(sentence)
      && /\b(?:qualified|supported|ready|proven|complete|accepted)\b/i.test(sentence)
      && !/\b(?:no|not|never|without|missing|until|future|deferred|only after|does not)\b/i.test(sentence)) fail(DOC, "claim", "unsupported current canonical qualification claim");
  }
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
  }
}

function rejectClaims(source: string, file: string, fail: (file: string, rule: string, detail: string) => void): void {
  for (const sentence of source.split(/\n\s*\n/).flatMap(paragraph => paragraph.replaceAll("\n", " ").split(/[.!?;](?:\s|$)/))) {
    if (/\b(?:bootstrap|reopen|whole.chain|production|Knots qualification)\b/i.test(sentence)
      && /\b(?:qualified|supported|ready|proven|complete|accepted)\b/i.test(sentence)
      && !/\b(?:no|not|never|without|missing|until|future|deferred|only after|does not)\b/i.test(sentence)) fail(file, "claim", "unsupported current canonical qualification claim");
  }
}
