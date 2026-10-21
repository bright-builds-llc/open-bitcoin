# Standards Overrides

Use this file to record deliberate deviations from the canonical coding and architecture standards.

## Active overrides

| Standard | Local decision | Rationale | Owner | Review date |
| --- | --- | --- | --- | --- |
| `REPLACE_WITH_STANDARD` | `REPLACE_WITH_LOCAL_DECISION` | `REPLACE_WITH_REASON` | `REPLACE_WITH_OWNER` | `REPLACE_WITH_DATE` |
| Rust panic avoidance | Retain only the existing `ManagedRpcContext::from_fixture_runtime_config` expect, identified by exact path and literal in `scripts/panic-sites.allowlist`. | This explicit synthetic fixture API intentionally panics on invalid configuration or failed storage, network configuration or fixture recovery. It is used by tests and benchmark fixtures, never ordinary or sealed canonical startup. Its public Panics documentation names the fallible `from_fixture_runtime_config_with_store(config, None)` alternative. This is a tooling compatibility boundary, not an infallibility claim or permission for new panic sites. | Phase166 code/security review | 2026-10-21 |

## Notes

- Prefer narrow, explicit exceptions over broad "this repo is different" statements.
- If local verification is intentionally hook-owned or leaves heavy suites to CI, record that explicitly here.
- Revisit overrides periodically instead of letting them become permanent by accident.
- If an override becomes common across many repos, move it back upstream into the canonical standards repo.
