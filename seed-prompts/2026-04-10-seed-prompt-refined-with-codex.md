# Open Bitcoin Seed Prompt

We are building a fully working Bitcoin node in Rust.

This document defines the project's seed prompt and states its direction, constraints, and expectations in a clear, decision-ready form.

## Goal

- Deliver a headless Bitcoin node and wallet implementation in Rust that preserves externally observable behavior for the in-scope surfaces defined below.
- Use more idiomatic, type-safe Rust internals where that improves clarity and safety, as long as external contracts and behavior are upheld.
- Favor simple, modular, strongly typed designs that make illegal states unrepresentable and keep business logic isolated from I/O.

## MUST

### Reference Baseline and Scope

- Use Bitcoin Knots `29.3.knots20260210` as the pinned behavioral reference baseline for in-scope behavior.
- Keep the reference implementation vendored under `packages/` as a pinned external source, such as a git submodule or an equivalent pinned source snapshot.
- Track intentional deviations explicitly rather than allowing silent drift from the reference baseline.
- Target behavior parity for these in-scope surfaces:
  - consensus-critical rules
  - transaction and block validation
  - chainstate behavior
  - mempool and node policy behavior
  - P2P networking behavior
  - wallet behavior
  - RPC behavior
  - CLI and configuration behavior
- Treat Qt GUI parity and faithful porting of the reference GUI code as out of scope for the initial implementation.
- Do not require line-by-line code parity with the C++ implementation. The requirement is behavioral parity for the in-scope surfaces.

### Architecture

- Follow functional core / imperative shell architecture.
- Keep pure-core logic free from direct I/O and runtime side effects.
- In pure-core crates or modules, do not depend directly on:
  - filesystem access
  - network sockets
  - clocks or wall time
  - environment variables
  - process execution
  - threads
  - async runtimes
  - randomness sources
- Pass all effects through explicit boundaries, ports, or adapters owned by the imperative shell.
- Follow the "parse, don't validate" approach where it materially improves correctness and clarity.
- Prefer domain types that encode invariants over repeated primitive validation at call sites.
- Keep the codebase modular. First-party source code should live in well-bounded packages with clear responsibilities.

### Testing and Verification

- Maintain 100% unit test coverage for pure-core code as measured by the project's chosen coverage tooling.
- Mirror the intent of the reference implementation's tests without requiring a literal one-to-one port of every C++ test.
- Use a layered test strategy:
  - unit tests for pure logic and domain behavior
  - black-box functional or integration tests for externally observable node, wallet, RPC, CLI, and P2P behavior
  - fuzzing or property-style tests for parser, serialization, and protocol surfaces where they meaningfully reduce risk
- Prefer platform-independent black-box tests that can be run against both the reference implementation and the Rust implementation to lock down parity.
- Make integration tests parallel-safe and hermetic where practical by isolating ports, processes, data directories, and temporary state per test run.
- Enforce strong pre-commit and CI verification, including formatting, linting, build, test, and architecture-policy checks appropriate to changed paths.
- Add explicit checks that catch accidental I/O leakage into the pure core.

### Dependency Policy

- Do not use existing Rust Bitcoin libraries in the production implementation path.
- Keep dependencies minimal, justified, and security-conscious.
- Favor small, well-maintained foundational dependencies over large, opaque stacks.
- It is acceptable to use foundational infrastructure crates such as async runtimes, TLS libraries, and similar building blocks when they are justified and kept out of the pure core.
- Export our own Rust Bitcoin libraries from this repository where appropriate, and keep those libraries aligned with the same purity and modularity rules.

### Tooling and Repository Structure

- Use Bazelisk and Bazel with Bzlmod for first-party multi-language workspace builds unless a later project decision explicitly replaces that choice.
- Do not require the vendored reference implementation to adopt Bazel; it may keep its native upstream build system.
- Ensure first-party packages are invokable from the top level of the repository through the chosen build tooling.
- Keep correctness, architecture, and contributor workflow requirements codified in `AGENTS.md` and related repo policy files.

## SHOULD

- Maintain a living catalog of the reference implementation's major features, subsystems, quirks, known bugs, and suspected unknowns.
- Maintain a companion parity checklist that maps each reference surface to our implementation status, such as planned, in progress, done, deferred, or explicitly out of scope.
- Maintain benchmarks that measure important performance characteristics and, where meaningful, compare the Rust implementation against the reference implementation.
- Prefer designs that make future verification, benchmarking, and parity auditing easier rather than harder.
- Keep room for a future progress-tracking package or website that explains milestones and implementation status.

## DEFERRED / OUT OF SCOPE

- A graphical user interface is a separate future milestone.
- If a GUI is built later, it should be designed on its own terms rather than as a line-by-line port of the reference Qt GUI.
- Marketing sites, progress dashboards, and other product-adjacent packages are deferred until they support a clearer project milestone.
- Any framework choice for a future GUI should be made at that milestone based on the project's needs at that time.

## Guiding References

- Bitcoin Knots release baseline: <https://github.com/bitcoinknots/bitcoin/releases/tag/v29.3.knots20260210>
- Parse, don't validate: <https://www.harudagondi.space/blog/parse-dont-validate-and-type-driven-design-in-rust>
- Functional core / imperative shell: <https://www.youtube.com/watch?v=P1vES9AgfC4>
