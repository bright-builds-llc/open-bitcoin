<!-- bright-builds-rules-managed:begin -->

# Bright Builds Rules

`AGENTS.md` is the entrypoint for repo-local instructions, not the complete Bright Builds Rules specification.

This managed block is owned upstream by `bright-builds-rules`. If this block needs a fix, open an upstream PR or issue instead of editing the managed text in a downstream repo. Keep downstream-specific instructions outside this managed block.

Before plan, review, implementation, or audit work:

1. Read the repo-local instructions in `AGENTS.md`, including any `## Repo-Local Guidance` section and any instructions outside this managed block.
1. Read `AGENTS.bright-builds.md`.
1. Read `standards-overrides.md` when present.
1. Read the local managed standards pages under `standards/` relevant to the task.
1. If you have not done that yet, stop and load those sources before continuing.

Use this routing map when deciding what to load next:

- For repo-specific commands, prerequisites, generated-file ownership, CI-only suites, or recurring workflow facts, use the local `AGENTS.md`, especially `## Repo-Local Guidance`.
- For the Bright Builds default workflow and high-signal cross-cutting rules used in most tasks, use `AGENTS.bright-builds.md`.
- For deliberate repo-specific exceptions to the Bright Builds defaults, use `standards-overrides.md`.
- To choose the right managed standards page, start with the local Bright Builds entrypoint `standards/index.md`.
- For business-logic structure, domain modeling, and functional-core versus imperative-shell decisions, use the managed standards page `standards/core/architecture.md`.
- For control flow, naming, function/file size, and readability rules, use the managed standards page `standards/core/code-shape.md`.
- For frontend visual defaults, theme defaults, and dark-mode decisions, use the managed standards page `standards/core/frontend-ui.md`.
- For sync, bootstrap, and pre-commit verification rules, use the managed standards page `standards/core/verification.md`.
- For the managed starter checks, run `bun scripts/bright-builds-check.ts all`; use its `--help` output for check-specific commands, exact-file exceptions, and trailing-slash `file-lengths` directory exceptions.
- For unit-test expectations, use the managed standards page `standards/core/testing.md`.
- For Rust or TypeScript/JavaScript-specific rules, use the matching managed standards page under `standards/languages/`.
- For TypeScript/JavaScript frontend framework and UI-library defaults, use `standards/languages/typescript-javascript.md`.
- Keep recurring repo-specific workflow facts, commands, and links in a `## Repo-Local Guidance` section elsewhere in this file.
- Record deliberate repo-specific exceptions and override decisions in `standards-overrides.md`.
- If instructions elsewhere in `AGENTS.md` conflict with `AGENTS.bright-builds.md`, follow the repo-local instructions and treat them as an explicit local exception.

<!-- bright-builds-rules-managed:end -->

## Repo-Local Guidance

- Use `git submodule update --init --recursive` to materialize the pinned Knots baseline under `packages/bitcoin-knots`.
- Use `rust-toolchain.toml` as the Rust source of truth for local Cargo, CI, and Bazel. The current pinned version is `1.94.1`.
- Use `bash scripts/verify.sh --quick` for the repo-managed pre-commit gate and the default `bash scripts/verify.sh` for full parity verification before affected pushes and in CI. `--profile` adds full-run timing; the existing `--fast` remains a legacy iteration mode and does not replace the full gate.
- CI runs the full contract as one parallel job per shard (`bash scripts/verify.sh --list-shards`, `--shard <name>`, `--plan`). Shard assignment lives in `scripts/verify/helpers.sh`; `scripts/verify-shards.test.ts` keeps the CI matrix equal to the shard list and requires every `scripts/check-*.ts` checker to run as exactly one gate step. The local pre-push gate runs the same contract as `bash scripts/verify.sh --parallel`: checker-test shards concurrently, Cargo and Bazel shards serially because they share one target directory. Pushes that only add or edit Markdown under `docs/` (outside `docs/parity/`), `.codex/tasks/` or `.agents/skills/` take the quick gate.
- On macOS outside CI, full verification mounts a RAM disk at `/Volumes/open-bitcoin-verify-<pid>`, points `TMPDIR` at it for the run and detaches it on exit, because the storage engine's syncs are about 80 times slower on the internal disk there. Set `OPEN_BITCOIN_VERIFY_RAMDISK=0` to keep the normal temporary directory. For ad-hoc store-backed test runs, a RAM-backed `TMPDIR` gives the same speed-up.
- The verifier runs `scripts/check-phase*` test suites through the `scripts/verify/sample-checker-tests.ts` preload, which keeps a spread of rows per looped call site (the first five, then Fibonacci-spaced) because every row exercises the same checker path. Set `OPEN_BITCOIN_EXHAUSTIVE_CHECKER_TESTS=1` to run every case, for example after editing a checker.
- During UAT, provide copy-pasteable repo-local Cargo and Bazel commands for operator workflows instead of only naming the installed `open-bitcoin` alias. Prefer `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts; prefer TypeScript for substantial script logic, and keep Bash for thin orchestration wrappers and simple shell checks.
- For direct Bun test-file runs, use an explicit path such as `bun test ./scripts/check-phase154-basic-filters.test.ts`; the `./` selects the file without checkout-wide discovery on the pinned Bun runtime.
- The test-only BASIC oracle in `scripts/basic-filter-oracle.py` imports pinned Knots Python SipHash, block and peer-message helpers and implements source-derived proof formulas for independent protocol evidence; this narrow compatibility exception keeps all filesystem orchestration, pin checks, bounds, corpus validation and fixture rendering in Bun.
- Use `bash scripts/install-git-hooks.sh` to install the repo-managed commit and push hooks under `.githooks`; rerunning it is safe and idempotent, and the verifier self-heals missing local hook installation outside CI.
- Treat `docs/metrics/lines-of-code.md` as an intentionally tracked generated artifact. Expect it to change when hooks or verification regenerate it, and treat stale LOC updates as required freshness changes rather than incidental noise.
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`.
- When adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, add the required parity breadcrumb block through `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`; use the explicit `none` breadcrumb only when no defensible Bitcoin Knots source anchor exists.
- After substantial feature, parity, operator-surface, or workflow changes, check whether the relevant README files need updates so contributor-facing status stays current.
- Run ad-hoc Cargo and Bazel work through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>` so local duration history and the checkout-scoped cooperative build lock stay effective. Do not overlap Cargo jobs against the same target directory.
- Treat a quiet command, tool yield, or client polling interval as different from a command timeout. Poll resumable verification sessions at least every 60 seconds, do not interrupt solely because an estimate was exceeded, and capture liveness evidence before terminating an apparently stalled build or test.

## Project and Parity Boundaries

Open Bitcoin is a headless Rust node and wallet. The behavioral reference is Bitcoin Knots `29.3.knots20260210`, pinned by `packages/bitcoin-knots`; keep externally observable in-scope behavior auditable through `docs/parity/`. Pure Bitcoin domain rules belong in functional-core crates; filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects belong in shell adapters. Keep dependencies minimal and avoid existing Rust Bitcoin libraries in the production path. GUI, public-default operation, production service, and production-funds claims require separate evidence.

- Read `docs/parity/index.json` for current capability status, evidence classes, intentional differences, and limits. Read `docs/parity/requirements.md` for approved v2.6 requirement wording. `docs/parity/README.md` defines the contributor workflow and evidence standard.
- `open-bitcoind` consumes proved canonical genesis for all four selectors into a local `DurableSyncRuntime`/RPC context with explicit BASIC support. The [bounded local route](docs/parity/v2-6-candidate-history-validation.md) admits 301 default-Regtest descendants under selected network rules and validated ancestry, then reopens complete retained history through offline `sync status`. Seeded Mainnet/Testnet3 script and Mainnet BIP30 cases remain non-authoritative; Mainnet, testnet3 and default Signet ordinary canonical product routes refuse. Public networking stays default off, the canonical-history runtime wallet remains unavailable, and actual pinned-Knots daemon differential qualification remains future parity work.
- Treat `.planning/` as read-only historical evidence. Preserve its referenced records and create no new GSD plans, summaries, verification reports, or state updates. A later reference-audited cleanup may prune it.

## Direct-Main Parity Workflow

1. Sync local `main` with `origin/main` before implementation. Work on local `main` and push it directly after verification; this project does not use PRs for parity work.
2. For multi-session work, add one checkable append-only block to `.codex/tasks/todo.md`. Keep a short plan in the task or chat when the change spans components or carries non-obvious regression risk.
3. Identify the ledger surface and its pinned Knots anchors, expected external behavior, evidence class, and claim limits. Update tests, corpus/oracle evidence, source breadcrumbs, and the ledger with the code change. Seeded fixtures never grant ordinary product or actual daemon-differential authority.
4. Run focused tests for every changed behavior and inspect the diff. The quick commit hook checks formatting and current ledger/claim metadata. For this repo, the full Rust/build/coverage/Bazel contract is enforced by the automatic pre-push hook for parity-affecting changes; this is the documented local exception to the global Rust pre-commit sequence in `standards-overrides.md`.
5. Commit coherent changes on local `main`, then push `main`. The push hook checks all outgoing commits, refuses a dirty or divergent checkout, and runs the full default verifier for code, fixtures, parity evidence, and other non-allowlisted paths. Watch CI after the push and fix any failure before considering the change complete.
