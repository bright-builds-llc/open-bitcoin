---
phase: 01
slug: workspace-baseline-and-guardrails
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-25
updated: 2026-04-25T12:17:47Z
---

# Phase 01 - Security

Per-phase security contract for workspace baseline and guardrails. Source artifacts:

- `.planning/phases/01-workspace-baseline-and-guardrails/01-01-PLAN.md`
- `.planning/phases/01-workspace-baseline-and-guardrails/01-02-PLAN.md`
- `.planning/phases/01-workspace-baseline-and-guardrails/01-03-PLAN.md`
- `.planning/phases/01-workspace-baseline-and-guardrails/01-04-PLAN.md`
- `.planning/phases/01-workspace-baseline-and-guardrails/01-01-SUMMARY.md`
- `.planning/phases/01-workspace-baseline-and-guardrails/01-02-SUMMARY.md`
- `.planning/phases/01-workspace-baseline-and-guardrails/01-03-SUMMARY.md`
- `.planning/phases/01-workspace-baseline-and-guardrails/01-04-SUMMARY.md`

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| TB-01 | Upstream internet repositories to vendored baseline state inside this repo | Git submodule URL, gitlink, tag, and upstream source tree |
| TB-02 | Contributor edits to first-party workspace layout | Cargo workspace membership, crate names, crate dependency declarations |
| TB-03 | Bazel module resolution and toolchain downloads to repo-pinned build inputs | Bazel version, Bzlmod module graph, rules_rust version, Rust toolchain pin |
| TB-04 | Repo-root build entrypoints to first-party package targets | Root Bazel aliases and package BUILD targets |
| TB-05 | Contributor code changes to pure-core architectural boundary | Pure-core allowlist, Cargo metadata, source imports, runtime dependencies |
| TB-06 | Local verification scripts to CI enforcement and merge gating | `scripts/verify.sh` and `.github/workflows/ci.yml` |
| TB-07 | Contributor and reviewer understanding to parity claims made by the repository | `docs/parity/index.json`, parity docs, README, CONTRIBUTING, AGENTS guidance |
| TB-08 | Repo policy docs to future execution and review workflows | Contributor-facing documentation and local workflow instructions |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-01-01 | Tampering | `packages/bitcoin-knots` gitlink and `.gitmodules` | mitigate | `.gitmodules` registers `packages/bitcoin-knots`; `git submodule status packages/bitcoin-knots` reports gitlink `a9aee730466ac67d35a3c03ee24676be5e045878`; `git -C packages/bitcoin-knots describe --tags --exact-match` returns `v29.3.knots20260210`. | closed |
| T-01-02 | Spoofing | first-party crate layout under `packages/` | mitigate | `packages/Cargo.toml` declares explicit first-party workspace members, including `open-bitcoin-core` and `open-bitcoin-node`; their crate manifests use matching package names and path dependencies. | closed |
| T-01-03 | Tampering | `.bazelversion` and `MODULE.bazel` | mitigate | `.bazelversion` pins Bazel `8.6.0`; `MODULE.bazel` declares `rules_rust` `0.69.0` and Rust `1.94.1`; `rust-toolchain.toml` also pins Rust `1.94.1`. | closed |
| T-01-04 | Denial of service | root build entrypoints and crate BUILD files | mitigate | `BUILD.bazel` exposes `//:core` and `//:node` aliases to package BUILD targets; `bazel query 'set(//:core //:node)'` resolved both labels; `bazel build //:core //:node` completed successfully. | closed |
| T-01-05 | Elevation of privilege | pure-core crates | mitigate | `scripts/pure-core-crates.txt` source-controls the pure-core crate allowlist; `scripts/check-pure-core-deps.sh` enforces forbidden dependency and import checks; the checker completed with `Pure-core dependency and import checks passed.` | closed |
| T-01-06 | Repudiation | verification contract between local and CI environments | mitigate | `.github/workflows/ci.yml` checks out submodules, installs the pinned Rust toolchain and Bazelisk, then runs `bash scripts/verify.sh`; the same script is the local repo-native verification contract. | closed |
| T-01-07 | Repudiation | parity/deviation history | mitigate | `docs/parity/index.json` records baseline `29.3.knots20260210`, an explicit `deviations` array, and catalog entries; `docs/parity/README.md` documents the parity ledger workflow. | closed |
| T-01-08 | Information disclosure | contributor workflow docs | mitigate | `README.md`, `CONTRIBUTING.md`, and `AGENTS.md` document baseline sync, verification, and parity update workflow. Targeted scan for secret, token, credential, local path, and private marker patterns found no sensitive repo-facing values in those docs or `docs/parity/`. | closed |

## Summary Threat Flags

No `## Threat Flags` sections were present in the four phase summaries. No additional summary-derived threats were added.

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-25 | 8 | 8 | 0 | Codex |

## Verification Evidence

- `git fetch origin`
- `git rebase origin/main`
- `git submodule status packages/bitcoin-knots`
- `git -C packages/bitcoin-knots describe --tags --exact-match`
- `bash scripts/check-pure-core-deps.sh`
- `bazel query 'set(//:core //:node)'`
- `bazel build //:core //:node`
- `jq '{baseline, deviations, catalog_documents: (.catalog.documents | length)}' docs/parity/index.json`
- `rg -n "git submodule|verify\\.sh|parity|deviation|baseline|secret|token|password|credential|/Users/|localhost|AWS_|GITHUB_TOKEN|PRIVATE" README.md CONTRIBUTING.md AGENTS.md docs/parity/README.md docs/parity/index.json`

## Sign-Off

- [x] All threats have a disposition: mitigate, accept, or transfer
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-04-25
