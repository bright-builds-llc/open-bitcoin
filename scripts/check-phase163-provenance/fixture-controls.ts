import { PHASE163, PHASE163_DOC, ORIGINAL_AUDITS, PROVENANCE_CLAIMS,
  V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS } from "../check-phase163-provenance.ts";

const PHASE162 = ".planning/phases/162-real-prune-retention-and-integrated-parity-proof/";
const PHASE162_DOC = "docs/parity/v2-5-integrated-filter-proof.md";
const REPORTS = ["VERIFICATION", "NATIVE-EVIDENCE", "REVIEW", "SECURITY", "CLOSURE-VERIFICATION", "04-SUMMARY"];

function control(sources: ReadonlyMap<string, string>, paths: string[], identity: RegExp): string {
  for (const file of paths) {
    const maybeSource = sources.get(file);
    if (maybeSource !== undefined && identity.test(maybeSource)) return maybeSource;
  }
  throw new Error(`historical fixture requires explicit v2.5 control: ${paths.join(" or ")}`);
}

/** Materialize a synthetic historical active layout; never normalize raw checker inputs. */
export function v25FixtureSources(sources: ReadonlyMap<string, string>): Map<string, string> {
  const requirements = control(sources, V25_REQUIREMENTS, /^\*\*Milestone:\*\*[ \t]*v2\.5(?:[ \t]|$)/m);
  const roadmap = control(sources, V25_ROADMAPS, /^(?:##[ \t]+Active Milestone:[ \t]*v2\.5(?:[ \t]|$)|#[ \t]+Milestone[ \t]+v2\.5:)/m);
  const historical = new Map(sources);
  historical.set(V25_REQUIREMENTS[0], requirements);
  historical.set(V25_ROADMAPS[0], roadmap);
  for (const paths of [V25_REQUIREMENTS, V25_ROADMAPS, V25_AUDITS]) historical.delete(paths[1]);
  historical.delete(V25_AUDITS[0]);
  historical.delete(PHASE163_DOC);
  for (const name of REPORTS) historical.delete(PHASE163 + `163-${name}.md`);
  for (const [file, source] of historical) {
    if ((!file.endsWith(".md") && !file.endsWith(".json")) || file.startsWith(PHASE162)
      || file === PHASE162_DOC || ORIGINAL_AUDITS.includes(file)
      || /-(?:CONTEXT|GAP-SCOPE)\.md$/.test(file)) continue;
    historical.set(file, PROVENANCE_CLAIMS.reduce((view, claim) => view.replaceAll(claim, ""), source));
  }
  return historical;
}
