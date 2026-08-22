import type { ClaimFile } from "./constants.ts";

export type TextCorpus = Map<string, string>;

export function checkClaims(_texts: TextCorpus, _failures: string[]): void {}

export function claimFiles(): readonly ClaimFile[] {
  return [];
}
