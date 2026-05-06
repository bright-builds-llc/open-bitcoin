<!-- bright-builds-rules-managed:begin -->

# Bright Builds Rules

`AGENTS.md` is the entrypoint for repo-local instructions, not the complete Bright Builds Rules specification.

This managed block is owned upstream by `bright-builds-rules`. If this block needs a fix, open an upstream PR or issue instead of editing the managed text in a downstream repo. Keep downstream-specific instructions outside this managed block.

Before plan, review, implementation, or audit work:

1. Read the repo-local instructions in `AGENTS.md`, including any `## Repo-Local Guidance` section and any instructions outside this managed block.
1. Read `AGENTS.bright-builds.md`.
1. Read `standards-overrides.md` when present.
1. Read the pinned canonical standards pages relevant to the task.
1. If you have not done that yet, stop and load those sources before continuing.

Use this routing map when deciding what to load next:

- For repo-specific commands, prerequisites, generated-file ownership, CI-only suites, or recurring workflow facts, use the local `AGENTS.md`, especially `## Repo-Local Guidance`.
- For the Bright Builds default workflow and high-signal cross-cutting rules used in most tasks, use `AGENTS.bright-builds.md`.
- For deliberate repo-specific exceptions to the Bright Builds defaults, use `standards-overrides.md`.
- To choose the right pinned canonical standards page, start with the Bright Builds entrypoint `standards/index.md`.
- For business-logic structure, domain modeling, and functional-core versus imperative-shell decisions, use the canonical page `standards/core/architecture.md`.
- For control flow, naming, function/file size, and readability rules, use the canonical page `standards/core/code-shape.md`.
- For sync, bootstrap, and pre-commit verification rules, use the canonical page `standards/core/verification.md`.
- For unit-test expectations, use the canonical page `standards/core/testing.md`.
- For Rust or TypeScript/JavaScript-specific rules, use the matching canonical page under `standards/languages/`.
- Keep recurring repo-specific workflow facts, commands, and links in a `## Repo-Local Guidance` section elsewhere in this file.
- Record deliberate repo-specific exceptions and override decisions in `standards-overrides.md`.
- If instructions elsewhere in `AGENTS.md` conflict with `AGENTS.bright-builds.md`, follow the repo-local instructions and treat them as an explicit local exception.

<!-- bright-builds-rules-managed:end -->

## Repo-Local Guidance

- Use `git submodule update --init --recursive` to materialize the pinned Knots baseline under `packages/bitcoin-knots`.
- Use `rust-toolchain.toml` as the Rust source of truth for local Cargo, CI, and Bazel. The current pinned version is `1.94.1`.
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build.
- During UAT, provide copy-pasteable repo-local Cargo and Bazel commands for operator workflows instead of only naming the installed `open-bitcoin` alias. Prefer `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts; prefer TypeScript for substantial script logic, and keep Bash for thin orchestration wrappers and simple shell checks.
- Use `bash scripts/install-git-hooks.sh` to install the repo-managed Git hooks under `.githooks`; rerunning it is safe and idempotent, and `bash scripts/verify.sh` will self-heal missing local hook installation outside CI.
- Treat `docs/metrics/lines-of-code.md` as an intentionally tracked generated artifact. Expect it to change when hooks or verification regenerate it, and treat stale LOC updates as required freshness changes rather than incidental noise.
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`.
- When adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, add the required parity breadcrumb block through `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`; use the explicit `none` breadcrumb only when no defensible Bitcoin Knots source anchor exists.
- After substantial feature, parity, operator-surface, or workflow changes, check whether the relevant README files need updates so contributor-facing status stays current.

<!-- GSD:project-start source:PROJECT.md -->

## Project

**Open Bitcoin**

Open Bitcoin is a headless Bitcoin node and wallet implementation in Rust, built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` across the in-scope consensus, validation, chainstate, mempool, networking, wallet, RPC, CLI, and configuration surfaces. It is for contributors who want a reference-grade node with a cleaner, more type-safe internal architecture, auditable parity, and a strict separation between pure domain logic and effectful adapters.

**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

### Constraints

- **Behavioral baseline**: Match Bitcoin Knots `29.3.knots20260210` for all in-scope surfaces — parity claims must be auditable.
- **Architecture**: Follow functional core / imperative shell boundaries — pure business logic stays free of direct I/O and runtime side effects.
- **Dependency policy**: Keep dependencies minimal and security-conscious, and do not use existing Rust Bitcoin libraries in the production path — the project owns its own domain model and implementation surface.
- **Build tooling**: Use Bazelisk and Bazel with Bzlmod for first-party workspace builds — multi-package growth should remain manageable from the repo root.
- **Verification**: Enforce formatting, linting, build, testing, coverage, and architecture-policy checks in pre-commit and CI — regressions should fail early.
- **Scope**: Ship a headless node and wallet first — GUI work is explicitly deferred.

<!-- GSD:project-end -->

<!-- GSD:stack-start source:STACK.md -->

## Technology Stack

- Rust `1.94.1` and Rust 2024 edition are pinned by `rust-toolchain.toml` and `packages/Cargo.toml`.
- Cargo workspace crates live under `packages/open-bitcoin-*`; Bazel/Bzlmod with `rules_rust` provides the top-level smoke build.
- Bun is pinned by `.bun-version` and is used as a runtime for repo-owned TypeScript automation in `scripts/`; this repo has no `package.json`, so there is no `bun install` bootstrap step.
- Current runtime dependencies include Fjall for durable storage, Tokio/Axum for the JSON-RPC server, clap for the operator CLI, Ratatui/Crossterm for the terminal dashboard, jsonc-parser for Open Bitcoin JSONC config, and serde/serde_json for stable data shapes.
- Full stack notes live in `.planning/STACK.md`.

<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->

## Conventions

- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and keep parity evidence auditable through `docs/parity/`.
- Keep pure Bitcoin domain behavior in functional-core crates and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters.
- Use `bash scripts/verify.sh` as the repo-native verification contract before marking work complete.
- Treat migration as dry-run-first and backup-aware; do not imply source datadir, service, config, or wallet mutation without an explicit future plan.
- Full conventions live in `.planning/CONVENTIONS.md`.

<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->

## Architecture

- Pure-core crates cover primitives, codec, consensus, chainstate, mempool, networking, and wallet behavior.
- `open-bitcoin-node` owns adapter-facing runtime orchestration, durable storage, status, metrics, logs, sync foundations, and wallet rescans.
- `open-bitcoin-rpc` owns JSON-RPC dispatch and the current `open-bitcoind` local RPC server binary.
- `open-bitcoin-cli` owns `open-bitcoin-cli` for baseline-compatible RPC calls and `open-bitcoin` for Open Bitcoin-specific operator workflows.
- `DurableSyncRuntime` is wired into `open-bitcoind` only as an explicit mainnet sync activation/preflight path; it is not yet an unattended public-mainnet full-sync daemon loop.
- Full architecture notes live in `.planning/ARCHITECTURE.md`.

<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->

## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, or `.github/skills/` with a `SKILL.md` index file.

<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->

## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:

- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.

<!-- GSD:workflow-end -->

<!-- GSD:profile-start -->

## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.

<!-- GSD:profile-end -->
