#!/usr/bin/env bun

import path from "node:path";
import { readFileSync } from "node:fs";

const repoRoot = path.resolve(import.meta.dir, "..");

const targetFiles = [
  "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs",
  "packages/open-bitcoin-node/src/network/inventory.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-node/src/network/lifecycle_projection.rs",
  "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs",
] as const;

const plan04Targets = [
  "apply_prepared_compact",
  "apply_prepared_serving",
  "apply_prepared_fanout",
  "apply_prepared_peer_lifecycle",
] as const;

const plan05Targets = [
  "apply_prepared_unbroadcast",
  "apply_prepared_persistence",
  "apply_prepared_evidence",
] as const;

const aggregateExclusions = new Set([
  "apply_prepared_lifecycle",
  "validate_prepared_lifecycle",
]);

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
  const match = new RegExp(`\\bfn\\s+${name}\\b`).exec(masked);
  if (!match) {
    return null;
  }
  const brace = masked.indexOf("{", match.index);
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
        return { name, source: source.slice(match.index, index + 1) };
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
    [/\b(?:std::fs|File::|OpenOptions::|TcpStream|UdpSocket|tokio::fs)\b/, "I/O type"],
    [/\.(?:read|read_to_end|read_to_string|write|write_all|flush)\s*\(/, "I/O call"],
    [/\bawait\b/, "async I/O await"],
  ];
  return forbidden
    .filter(([pattern]) => pattern.test(source))
    .map(([, label]) => `${target.name}: forbidden ${label} inside exact target apply`);
}

const sources = new Map(
  targetFiles.map((file) => [
    file,
    readFileSync(path.join(repoRoot, file), "utf8"),
  ]),
);
const allowlist = [...plan04Targets, ...plan05Targets];
const extracted: ExtractedFunction[] = [];
const failures: string[] = [];

for (const name of allowlist) {
  let maybeTarget: ExtractedFunction | null = null;
  for (const source of sources.values()) {
    const candidate = extractFunction(source, name);
    if (!candidate) {
      continue;
    }
    if (maybeTarget) {
      failures.push(`${name}: duplicate exact target apply`);
      break;
    }
    maybeTarget = candidate;
  }
  if (maybeTarget) {
    extracted.push(maybeTarget);
  } else if ((plan04Targets as readonly string[]).includes(name)) {
    failures.push(`${name}: required Plan 04 target apply not found`);
  }
}

for (const [file, source] of sources) {
  const discovered = [...source.matchAll(/\bfn\s+(apply_prepared_[A-Za-z0-9_]+)\b/g)]
    .map((match) => match[1] ?? "")
    .filter((name) => !aggregateExclusions.has(name));
  for (const name of discovered) {
    if (!allowlist.includes(name as (typeof allowlist)[number])) {
      failures.push(`${file}: unexpected target-like apply ${name}`);
    }
  }
}

for (const target of extracted) {
  failures.push(...checkFunction(target));
}

const names = extracted.map((target) => target.name);
console.log(`Phase 134 target apply discovery: ${names.join(", ")}`);
if (failures.length > 0) {
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}
console.log("Phase 134 target apply boundaries are structurally infallible.");
