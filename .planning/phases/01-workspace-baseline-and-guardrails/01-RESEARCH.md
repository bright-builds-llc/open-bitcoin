# Phase 1: Workspace, Baseline, and Guardrails - Research

**Researched:** 2026-04-11
**Domain:** Repository bootstrap, Bazel/Rust workspace setup, verification tooling, and architectural policy enforcement
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Vendor Bitcoin Knots as a pinned git submodule at `packages/bitcoin-knots`.
- Keep the reference baseline read-only from the first-party workspace perspective; compare against it and document intentional deviations instead of blending first-party code into it.
- Start with a bounded multi-crate layout under `packages/` that separates pure-core Rust libraries from shell and adapter crates.
- Avoid both a single monolithic first-party crate and microcrates for every primitive in Phase 1; split only at clear domain boundaries that later phases will reuse.
- Enforce functional core / imperative shell boundaries structurally with dedicated pure-core crates plus automated dependency-policy checks.
- Treat filesystem, sockets, clocks, environment variables, process execution, threads, async runtimes, and randomness as forbidden direct dependencies in pure-core crates.
- Expose one top-level repo verification contract for contributors, while delegating first-party package execution to Bazel targets and explicit Rust tooling where needed.
- Make format, lint, build, tests, coverage, and architecture-policy checks all part of the default pre-commit and CI verification path for changed areas.
- Seed a central parity/deviation ledger in Phase 1 with one machine-readable index and human-readable subsystem notes that can expand in later phases.
- Track intentional deviations explicitly from the start instead of relying on undocumented differences or commit history.

### the agent's Discretion
- Exact naming of first-party crates and Bazel targets, as long as the package layout keeps pure-core crates separate from adapters.
- Exact implementation of the architecture-policy checker, as long as it produces hard failures for forbidden pure-core dependencies.
- Exact choice of wrapper script or Bazel target name for the repo-native verification entrypoint.

### Deferred Ideas (OUT OF SCOPE)
- A GUI package or app surface for Open Bitcoin.
- A public progress dashboard or marketing site.
- Rich published benchmark or parity dashboards beyond repo-local audit artifacts.

</user_constraints>

<research_summary>
## Summary

Phase 1 should establish a dual-tooling workflow rather than trying to force Bazel to replace Cargo for every local Rust developer task on day one. The cleanest bootstrap path is: use a pinned Knots git submodule for the behavioral baseline, create a first-party Cargo workspace under `packages/` for Rust-native lint/test/coverage tooling, and layer Bazelisk + Bazel/Bzlmod + `rules_rust` on top so the repository already has its long-term top-level build entrypoint.

The official `rules_rust` docs support Bzlmod through `MODULE.bazel` and a `rust.toolchain(...)` extension, and the Cargo ecosystem already has an official-quality coverage path via `cargo-llvm-cov`. For Phase 1, the lowest-risk architecture enforcement approach is not a heavyweight lint framework but a repo-owned script that combines `cargo metadata` dependency checks with a forbidden-API source scan for pure-core crates. That keeps dependencies minimal while giving early, hard failures.

**Primary recommendation:** Build Phase 1 around a pinned Knots submodule, a bounded Cargo workspace (`open-bitcoin-core` + `open-bitcoin-node`), Bzlmod-based Bazel bootstrap with `rules_rust`, and a repo-owned `scripts/verify.sh` plus `scripts/check-pure-core-deps.sh` contract that CI can run unchanged.
</research_summary>

<standard_stack>
## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Bazelisk | latest stable installer flow | Ensures contributors and CI use the repo-pinned Bazel version from `.bazelversion` | Bazelisk is the normal entrypoint for pinning Bazel per repo rather than asking developers to manage versions manually |
| Bazel | `8.6.0` pin in `.bazelversion` | Top-level build system for first-party packages | Aligns with the project decision to use Bazel/Bzlmod from the repo root |
| `rules_rust` | `0.69.0` | Rust toolchain and build rules for Bazel | Official `rules_rust` docs show Bzlmod setup through `bazel_dep` and `rust.toolchain(...)` |
| Cargo workspace | local manifests under `packages/Cargo.toml` | Rust-native fmt, clippy, test, metadata, and coverage workflows | Cargo remains the clearest way to run Rust-native verification tools even when Bazel is the workspace orchestrator |
| Git submodule | Git built-in | Vendor the Knots baseline under `packages/bitcoin-knots` | Official Git submodule docs provide explicit add/update/set-branch flows that fit a pinned reference baseline |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `cargo-llvm-cov` | current release at implementation time | LLVM source-based coverage with `--fail-under-*` gates | Use for the pure-core 100% coverage contract and CI reporting |
| `cargo-nextest` | optional `0.9` series in CI if needed | Faster test execution and JUnit output | Use once the test suite becomes large enough that `cargo test` slows CI materially |
| GitHub Actions | repo-hosted workflow runner | Mirrors the repo-native verification contract in CI | Use for branch protection and repeatable verification on pull requests |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Git submodule baseline | Source snapshot committed into the repo | Simpler clone experience, but weaker upstream provenance and rougher update diffs |
| Cargo workspace + Bazel together | Bazel-only Rust workflows from day one | Fewer tool surfaces, but worse ergonomics for fmt/clippy/coverage bootstrap |
| `cargo-llvm-cov` | ad-hoc custom coverage scripts | Fewer tool installs, but weaker coverage enforcement and more maintenance burden |

**Installation:**
```bash
# Repository-level tools chosen for Phase 1
bazelisk version
git submodule update --init --recursive
cargo +stable llvm-cov --help
```
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Recommended Project Structure
```text
packages/
├── bitcoin-knots/          # pinned upstream reference baseline (git submodule)
├── Cargo.toml             # first-party Rust workspace
├── open-bitcoin-core/     # pure-core Rust library crate
└── open-bitcoin-node/     # shell/runtime Rust crate

scripts/
├── verify.sh              # contributor verification contract
└── check-pure-core-deps.sh

docs/
└── parity/
    ├── README.md
    └── index.json
```

### Pattern 1: Dual Build Surface
**What:** Keep Cargo manifests for Rust-native developer workflows while exposing first-party packages through Bazel targets at the repo root.
**When to use:** Early workspace bootstrap where Rust lint/test/coverage ergonomics matter as much as long-term Bazel integration.
**Example:**
```starlark
module(name = "open_bitcoin")

bazel_dep(name = "rules_rust", version = "0.69.0")

rust = use_extension("@rules_rust//rust:extensions.bzl", "rust")
rust.toolchain(
    edition = "2024",
    versions = ["1.85.0"],
)
```

### Pattern 2: Structural Purity Enforcement
**What:** Separate pure-core crates from shell crates, then check both dependency graphs and source imports for forbidden APIs.
**When to use:** Functional-core projects where comments and code review are not strong enough to stop architectural drift.
**Example:**
```bash
# Dependency graph gate
cargo metadata --manifest-path packages/Cargo.toml --format-version 1

# Source-level gate for forbidden imports
rg 'std::(fs|net|env|process|thread)|tokio|reqwest|rustls|rand' packages/open-bitcoin-core/src
```

### Anti-Patterns to Avoid
- **Bazel-only bootstrap for Rust verification:** It hides standard Rust feedback loops behind extra build-system work before the project has enough code to justify it.
- **Pure-core by convention only:** It makes later violations look accidental and hard to detect, especially once contributors multiply.
- **Vendored baseline without provenance:** A copied upstream tree with no pinned git relationship makes parity auditing and updates harder to trust.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Bazel version management | A custom shell wrapper that downloads Bazel | Bazelisk + `.bazelversion` | Per-repo Bazel pinning already exists and is the expected workflow |
| Coverage instrumentation | A custom rustc/llvm-profdata wrapper | `cargo-llvm-cov` | Official wrapper supports coverage thresholds, HTML/LCOV output, and `cargo nextest` integration |
| Vendored baseline update flow | A bespoke git-fetch script for the reference tree | `git submodule add/update/set-branch` | Git already supports the exact pin/update lifecycle needed for a reference baseline |

**Key insight:** Phase 1 should spend complexity budget on the project's actual architecture rules, not on replacing mature repo-management and verification tooling with one-off scripts.
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Cargo and Bazel drift
**What goes wrong:** Cargo manifests, Bazel targets, and CI commands stop describing the same first-party packages.
**Why it happens:** Teams bootstrap one tool first and treat the other as an afterthought.
**How to avoid:** Make Plan 02 own both Cargo workspace membership and Bazel target exposure for the same crate set.
**Warning signs:** `cargo metadata` sees crates that `bazel query` does not, or CI verifies only one surface.

### Pitfall 2: Purity policy that only checks dependencies
**What goes wrong:** Pure-core crates pass dependency checks but still import `std::fs`, `std::env`, or wall-clock APIs directly.
**Why it happens:** Cargo metadata cannot see direct standard-library API use.
**How to avoid:** Pair dependency allowlists with a forbidden-import source scan scoped to pure-core crates.
**Warning signs:** Pure-core code compiles with no external I/O crates but still reaches files, sockets, env vars, or threads.

### Pitfall 3: Baseline provenance hidden in prose only
**What goes wrong:** The repo claims a pinned reference baseline, but the actual gitlink/tag/update procedure is unclear or inconsistent.
**Why it happens:** The baseline is vendored quickly before documentation and parity tracking exist.
**How to avoid:** Commit the submodule pin, document the update flow, and seed a parity/deviation ledger in the same milestone.
**Warning signs:** Reviewers cannot tell whether a Knots change came from an intentional update or from ad-hoc directory edits.
</common_pitfalls>

<code_examples>
## Code Examples

Verified patterns from official sources:

### Bzlmod rules_rust bootstrap
```starlark
# Source: https://bazelbuild.github.io/rules_rust/
module(name = "open_bitcoin")
bazel_dep(name = "rules_rust", version = "0.69.0")

rust = use_extension("@rules_rust//rust:extensions.bzl", "rust")
rust.toolchain(
    edition = "2024",
    versions = ["1.85.0"],
)
```

### Git submodule baseline pin
```bash
# Source: https://git-scm.com/docs/git-submodule
git submodule add https://github.com/bitcoinknots/bitcoin packages/bitcoin-knots
git -C packages/bitcoin-knots checkout v29.3.knots20260210
```

### Coverage gate
```bash
# Source: https://github.com/taiki-e/cargo-llvm-cov
cargo llvm-cov --package open-bitcoin-core --fail-under-lines 100 --summary-only
```
</code_examples>

<sota_updates>
## State of the Art (2024-2026)

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| WORKSPACE-first Bazel bootstraps | Bzlmod-first Bazel bootstraps for new projects | Bazel 7/8 era | New repos should start with `MODULE.bazel` instead of building new workflow around WORKSPACE |
| Manual coverage plumbing | `cargo-llvm-cov` as the standard Rust coverage wrapper | Mature 2024-2026 Rust tooling | Coverage gates are easier to adopt early and easier to keep consistent in CI |
| `cargo test` as the only CI runner | `cargo test` plus optional `cargo-nextest` when suites grow | Current Rust CI practice | Start simple, but leave room for faster parallel test execution without redesigning the workspace |

**New tools/patterns to consider:**
- `rules_rust` Bzlmod toolchain registration through `rust.toolchain(...)` instead of bespoke toolchain bootstrapping.
- Pre-built `cargo-nextest` binaries in CI when the suite grows large enough to justify them.

**Deprecated/outdated:**
- Treating WORKSPACE as the primary new-project Bazel bootstrap path when Bzlmod is already the project's declared direction.
</sota_updates>

<open_questions>
## Open Questions

1. **Which exact Bazel release should the repo pin?**
   - What we know: the Phase 1 plan should use Bazelisk plus a repo-pinned Bazel version.
   - What's unclear: whether the team wants the latest Bazel 8 stable patch or a more conservative pin.
   - Recommendation: choose one Bazel 8 patch during implementation and document the reason in README/CONTRIBUTING; do not leave `.bazelversion` floating.

2. **Should `cargo-nextest` be mandatory in the first CI pass?**
   - What we know: official docs make installation straightforward, but the current repo has no meaningful test volume yet.
   - What's unclear: whether the initial suite will be large enough to justify the extra install/tool surface immediately.
   - Recommendation: make nextest optional in Phase 1 and revisit once Phase 2 adds real Rust tests.
</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)
- [Bazel external dependency overview](https://bazel.build/external/overview) — checked the Bzlmod-first dependency model for new Bazel workspaces.
- [rules_rust docs](https://bazelbuild.github.io/rules_rust/) — checked Bzlmod setup, `bazel_dep`, and `rust.toolchain(...)` usage.
- [cargo-llvm-cov README](https://github.com/taiki-e/cargo-llvm-cov/blob/main/README.md) — checked coverage capabilities, fail-under flags, and CI integration.
- [cargo-nextest installation docs](https://nexte.st/docs/installation/pre-built-binaries/) — checked current install and GitHub Actions usage.
- [git-submodule docs](https://git-scm.com/docs/git-submodule) — checked add/update/set-branch lifecycle for a pinned reference baseline.

### Secondary (MEDIUM confidence)
- None.

### Tertiary (LOW confidence - needs validation)
- None.
</sources>

<metadata>
## Metadata

**Research scope:**
- Core technology: Bazelisk, Bazel/Bzlmod, `rules_rust`, Cargo workspace bootstrap
- Ecosystem: `cargo-llvm-cov`, `cargo-nextest`, Git submodule lifecycle
- Patterns: dual build surface, structural purity enforcement, parity ledger seeding
- Pitfalls: Bazel/Cargo drift, incomplete purity checks, undocumented baseline provenance

**Confidence breakdown:**
- Standard stack: HIGH - based on official tool docs
- Architecture: HIGH - directly aligned with project constraints plus official tool capabilities
- Pitfalls: HIGH - derived from the constraints and tool surfaces in scope for this phase
- Code examples: HIGH - adapted from official docs with repo-specific naming

**Research date:** 2026-04-11
**Valid until:** 2026-05-11
</metadata>

---

*Phase: 01-workspace-baseline-and-guardrails*
*Research completed: 2026-04-11*
*Ready for planning: yes*
