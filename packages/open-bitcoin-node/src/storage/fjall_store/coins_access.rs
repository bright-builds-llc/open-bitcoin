// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use fjall::{Database, Keyspace};

use super::FjallNodeStore;

impl FjallNodeStore {
    #[allow(dead_code)]
    pub(crate) fn database(&self) -> &Database {
        &self.db
    }

    #[allow(dead_code)]
    pub(crate) fn coins_keyspace(&self) -> &Keyspace {
        &self.coins
    }
}
