# Config Ownership and Precedence

For the practical operator workflow around these rules, see
[`docs/operator/runtime-guide.md`](../operator/runtime-guide.md).

## Open Bitcoin JSONC

`open-bitcoin.jsonc` is the Open Bitcoin-owned config file for wizard and onboarding answers plus dashboard, service, migration, metrics, logging, storage, and sync settings. It is user-editable JSONC so operators can keep comments near local operational choices.

Environment is the source for `OPEN_BITCOIN_CONFIG`, `OPEN_BITCOIN_DATADIR`, and `OPEN_BITCOIN_NETWORK`.

For `open-bitcoind`, Phase 35 mainnet sync activation reads
`sync.network_enabled` plus `sync.mode` from the selected Open Bitcoin JSONC
file. The daemon also accepts Open Bitcoin-only CLI overrides such as
`-openbitcoinconf=<path>` and `-openbitcoinsync=mainnet-ibd`; these keys are not
valid `bitcoin.conf` settings.

Phase 36 extends that JSONC-owned sync surface with peer-lifecycle config:

- `sync.manual_peers` for explicit outbound peers (`host` or `host:port`)
- `sync.dns_seeds` for overriding the default mainnet DNS seed list
- `sync.target_outbound_peers` for the bounded outbound target per sync round

Those settings remain Open Bitcoin-only knobs owned by `open-bitcoin.jsonc`, not
baseline `bitcoin.conf`.

The operator resolver reports the selected Open Bitcoin JSONC path, baseline-compatible `bitcoin.conf` path, datadir, structured log directory, metrics store directory, network, and credential source. Credential reporting is metadata-only: cookie files are reported by path/source and presence, never by cookie contents.

## Precedence

Configuration precedence is:

`CLI flags > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`

Cookie files are an auth fallback, not an application-settings layer.

## bitcoin.conf compatibility boundary

Baseline Bitcoin/Knots keys remain in `bitcoin.conf`. Open Bitcoin-only keys must not be written to `bitcoin.conf`; that includes wizard, onboarding, dashboard, service, migration, metrics, logging, storage, and sync settings.

The existing `bitcoin.conf` loader should continue to reject unknown Open Bitcoin-only keys such as `dashboard`, `service`, or `openbitcoinsync` instead of silently accepting them.
