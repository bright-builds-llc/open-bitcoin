# Open Bitcoin Conventions

Last updated: 2026-05-01

## Parity And Evidence

- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior
  for every in-scope surface.
- Record intentional behavior differences in `docs/parity/index.json` and the
  relevant companion catalog page under `docs/parity/`.
- Keep parity claims evidence-based. Prefer links to verification reports,
  catalog entries, tests, scripts, or checked-in docs over broad prose claims.

## Code Shape

- Keep pure domain behavior separate from adapters that perform I/O, runtime,
  storage, process, network, terminal, or service-manager effects.
- Make illegal states unrepresentable when Rust types can practically encode the
  invariant.
- Prefer typed errors and `?` propagation over panic-like production paths.
- Follow the repo's parity breadcrumb requirement when adding first-party Rust
  source or test files under `packages/open-bitcoin-*/src` or
  `packages/open-bitcoin-*/tests`.

## Operator Surface

- Keep operator output quiet, information-dense, and work-focused.
- Status, dashboard, service diagnostics, and support output should use the
  shared status snapshot and report unavailable live fields explicitly instead
  of inventing defaults.
- Preview or dry-run behavior must be explicit for migration and service
  actions that could affect local machine state.

## Tooling

- Use `bash scripts/verify.sh` as the source-of-truth local verification command.
- Use Bun only as the pinned runtime for repo-owned TypeScript automation unless
  the repository intentionally adds a package manifest later.
- Do not add `bun install` to setup instructions while the repo has no
  `package.json`.
- Keep Bash wrappers thin and put substantial repo-owned automation in
  TypeScript run by Bun.

## Planning Artifacts

- `.planning/PROJECT.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`, and
  `.planning/MILESTONES.md` are the current planning entrypoints.
- v1.0 raw phase history remains in `.planning/phases/`.
- v1.1 raw phase history is archived under
  `.planning/milestones/v1.1-phases/`.
- `.planning/research/` contains historical pre-v1.1 research unless a file
  explicitly says it has been refreshed.
