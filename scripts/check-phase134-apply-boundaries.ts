#!/usr/bin/env bun

import path from "node:path";

import { readSourceRoot } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");

export const PHASE134_APPLY_TARGET_FILES = [
  "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs",
  "packages/open-bitcoin-node/src/network/inventory.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-node/src/network/lifecycle_projection.rs",
  "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs",
] as const;

const REQUIRED_TARGETS = [
  "apply_prepared_compact",
  "apply_prepared_serving",
  "apply_prepared_fanout",
  "apply_prepared_peer_lifecycle",
  "apply_prepared_unbroadcast",
  "apply_prepared_persistence",
  "apply_prepared_evidence",
  "apply_prepared_lifecycle",
] as const;

const DISCOVERY_EXCLUSIONS = new Set(["validate_prepared_lifecycle"]);

type ExtractedFunction = {
  name: string;
  source: string;
};

function maskCommentsAndStrings(source: string): string {
  let result = "";
  let state: "code" | "line" | "block" | "string" | "char" = "code";
  let escaped = false;
  for (let index = 0; index < source.length; index += 1) {
    const current = source[index] ?? "";
    const next = source[index + 1] ?? "";
    if (state === "line") {
      if (current === "\n") {
        state = "code";
        result += "\n";
      } else {
        result += " ";
      }
      continue;
    }
    if (state === "block") {
      if (current === "*" && next === "/") {
        result += "  ";
        index += 1;
        state = "code";
      } else {
        result += current === "\n" ? "\n" : " ";
      }
      continue;
    }
    if (state === "string" || state === "char") {
      result += current === "\n" ? "\n" : " ";
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
      continue;
    }
    if (current === "/" && next === "/") {
      result += "  ";
      index += 1;
      state = "line";
    } else if (current === "/" && next === "*") {
      result += "  ";
      index += 1;
      state = "block";
    } else if (current === '"') {
      result += " ";
      state = "string";
    } else if (current === "'") {
      result += " ";
      state = "char";
    } else {
      result += current;
    }
  }
  return result;
}

function extractFunction(source: string, name: string): ExtractedFunction | null {
  const masked = maskCommentsAndStrings(source);
  const maybeMatch = new RegExp(`\\bfn\\s+${name}\\b`).exec(masked);
  if (!maybeMatch) {
    return null;
  }
  const brace = masked.indexOf("{", maybeMatch.index);
  if (brace < 0) {
    throw new Error(`${name}: missing function body`);
  }
  let depth = 0;
  for (let index = brace; index < masked.length; index += 1) {
    if (masked[index] === "{") {
      depth += 1;
    } else if (masked[index] === "}") {
      depth -= 1;
      if (depth === 0) {
        return {
          name,
          source: source.slice(maybeMatch.index, index + 1),
        };
      }
    }
  }
  throw new Error(`${name}: unbalanced function body`);
}

function checkFunction(target: ExtractedFunction): string[] {
  const source = maskCommentsAndStrings(target.source);
  const forbidden: Array<[RegExp, string]> = [
    [/->\s*Result\b/, "Result return"],
    [/\?/, "? propagation"],
    [/\b(?:transaction_|compute_)?(?:txid|wtxid)\s*\(/, "identifier derivation"],
    [/\b(?:encode|decode)[A-Za-z0-9_]*\s*\(/, "encode/decode"],
    [
      /\b(?:std::fs|File::|OpenOptions::|TcpStream|UdpSocket|tokio::fs)\b/,
      "I/O type",
    ],
    [
      /\.(?:read|read_to_end|read_to_string|write|write_all|flush)\s*\(/,
      "I/O call",
    ],
    [/\bawait\b/, "async I/O await"],
  ];
  return forbidden
    .filter(([pattern]) => pattern.test(source))
    .map(
      ([, label]) =>
        `${target.name}: forbidden ${label} inside exact target apply`,
    );
}

export function checkPhase134ApplyBoundaries(
  maybeRepoRoot: string = DEFAULT_REPO_ROOT,
): string[] {
  const sources = new Map(
    PHASE134_APPLY_TARGET_FILES.map((file) => [
      file,
      readSourceRoot(maybeRepoRoot, file),
    ]),
  );
  const extracted: ExtractedFunction[] = [];
  const failures: string[] = [];

  for (const name of REQUIRED_TARGETS) {
    const matches = [...sources.values()]
      .map((source) => extractFunction(source, name))
      .filter((target): target is ExtractedFunction => target !== null);
    if (matches.length === 0) {
      failures.push(`${name}: required Phase 134 target apply not found`);
      continue;
    }
    if (matches.length > 1) {
      failures.push(`${name}: duplicate exact target apply`);
      continue;
    }
    extracted.push(matches[0]);
  }

  for (const [file, source] of sources) {
    const discovered = [
      ...source.matchAll(/\bfn\s+(apply_prepared_[A-Za-z0-9_]+)\b/g),
    ]
      .map((match) => match[1] ?? "")
      .filter((name) => !DISCOVERY_EXCLUSIONS.has(name));
    for (const name of discovered) {
      if (!REQUIRED_TARGETS.includes(name as (typeof REQUIRED_TARGETS)[number])) {
        failures.push(`${file}: unexpected target-like apply ${name}`);
      }
    }
  }

  for (const target of extracted) {
    failures.push(...checkFunction(target));
  }
  return failures;
}

if (import.meta.main) {
  const failures = checkPhase134ApplyBoundaries();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }
  console.log(
    `Phase 134 target apply discovery: ${REQUIRED_TARGETS.join(", ")}`,
  );
  console.log("Phase 134 target apply boundaries are structurally infallible.");
}
