## deferred-20-02-loc-report | 2026-04-27 11:00 | verify.sh blocked by stale LOC report

- Scope: out-of-scope pre-existing/generated verification blocker
- Detail: `bash scripts/verify.sh` fails on `docs/metrics/lines-of-code.md` with `stale LOC report`
- Reason deferred: `docs/metrics/lines-of-code.md` is outside Plan 20-02's owned write set
- Verification impact: repo-native verify remains blocked until the LOC report is refreshed in a plan that owns that file
