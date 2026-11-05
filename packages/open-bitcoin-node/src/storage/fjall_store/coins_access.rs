// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    FjallNodeStore,
    tracked::{Database, Keyspace},
};

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
