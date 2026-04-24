# Deviations And Unknowns

This page collects current deviations, known gaps, suspected unknowns, deferred
surfaces, and folded todo history for audit review. It summarizes existing
parity artifacts and completed planning follow-ups without adding new
implementation scope.

## Intentional Deviations

`docs/parity/index.json` currently records no intentional in-scope deviations
from Bitcoin Knots `29.3.knots20260210`.

Intentional differences must still be added to `docs/parity/index.json` when an
in-scope external behavior deliberately diverges from the pinned baseline.

## Deferred Surfaces

The Phase 8 operator catalog records these deferred RPC, CLI, and config
surfaces:

- Richer send RPC ergonomics beyond the current deterministic build and sign
  extensions.
- Peer-info views and the human net-info dashboard that depends on them.
- Multi-endpoint wallet selection semantics.
- Remote-operator auth and ACL features such as rpcauth and whitelist handling.
- Wait-for-daemon, daemon supervision, and broader process-control helpers.

The Phase 9 verification catalog records these deferred harness and fuzzing
surfaces:

- Building or spawning the vendored Knots daemon from the shared harness.
- Translating the full upstream Python functional suite.
- Adding a dedicated cargo-fuzz or libFuzzer runtime beyond deterministic
  generated property tests.

These are deferred by scope decision, not silent omissions.

## Suspected Unknowns

Current catalog entries preserve these review targets:

- Deprecated or ambiguous hex acceptance may still matter at some future
  user-facing boundaries.
- Additional serializer parameter contexts may need explicit typed Rust
  boundaries as disk and networking adapters grow.
- Address-codec, protocol, and witness edge cases may need more repo-owned
  fixtures as new surfaces become public.
- Cache-flush, multi-manager, and long-lived runtime policy behavior may need
  clearer ownership between pure core and shell adapters.
- Future peer governance, discovery, address relay, HD descriptors, multisig,
  PSBT, send, peer-info, and multi-endpoint semantics need scoped parity
  decisions before they are claimed complete.
- Future Knots-backed harness work must decide which upstream functional cases
  are translated into Rust and which are wrapped around a managed baseline
  process.

## Folded Todo Audit

### AI-Agent-Friendly CLI Surface

The pending CLI todo is folded into audit evidence only. Phase 8 already proves
non-interactive CLI execution, stable JSON output for get-info, explicit exit
code failures, duplicate named-parameter rejection, and no hidden stdin wait on
normal invocations.

Evidence to inspect:

- [Phase 8 verification](../../.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md)
- [RPC, CLI, and config catalog](catalog/rpc-cli-config.md)
- [Original pending todo](../../.planning/todos/pending/2026-04-18-ai-agent-friendly-cli-surface.md)

Residual risk: broader command discovery, schema introspection, and dedicated
agent affordances remain future design work, not Phase 10 implementation scope.

### Panic And Illegal-State Exposure

The former panic and illegal-state sweep was completed as Phase 11. The phase
inventoried first-party production panic-like sites, replaced reachable crash
paths with typed failures, and added a repo-owned guard for future changes.

Evidence to inspect:

- [Phase 11 context](../../.planning/phases/11-panic-and-illegal-state-hardening/11-CONTEXT.md)
- [Phase 11 inventory](../../.planning/phases/11-panic-and-illegal-state-hardening/11-INVENTORY.md)
- [Original completed todo](../../.planning/todos/completed/2026-04-18-sweep-panics-and-illegal-states.md)
- [Repo verification contract](../../scripts/verify.sh)
- [Production panic-site guard](../../scripts/check-panic-sites.sh)

Residual risk: future production `expect`, `unwrap`, `panic!`, `unreachable!`,
`todo!`, or `unimplemented!` sites must either be fixed or added to the narrow
allowlist with a local invariant rationale. The allowlist is empty at Phase 11
close.

## Follow-Up Triggers

Update this page when:

- `docs/parity/index.json` gains a new intentional deviation.
- A deferred RPC, CLI, config, harness, fuzzing, or benchmark surface becomes
  in scope.
- A suspected unknown is resolved by evidence and can move into a catalog entry,
  deviation record, or release-readiness note.
- A folded todo becomes active implementation work in a dedicated phase.
- Phase 11's panic-site allowlist gains an entry that needs audit review.
