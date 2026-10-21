// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{StorageError, copy, inspection_error};
use fjall::{Database, KeyspaceCreateOptions};
use std::collections::BTreeMap;

/// Complete untrusted logical bytes, with missing namespaces and rows preserved.
pub(in crate::storage) struct InspectedRecords {
    pub(super) namespaces: BTreeMap<String, BTreeMap<Vec<u8>, Vec<u8>>>,
}

impl InspectedRecords {
    pub(in crate::storage) fn namespaces(&self) -> impl Iterator<Item = &str> {
        self.namespaces.keys().map(String::as_str)
    }

    pub(in crate::storage) fn maybe_records(
        &self,
        name: &str,
    ) -> Option<&BTreeMap<Vec<u8>, Vec<u8>>> {
        self.namespaces.get(name)
    }

    /// Uses only enumerated, already existing keyspaces and returns no engine owners.
    pub(in crate::storage) fn read_existing(database: &Database) -> Result<Self, StorageError> {
        if database.keyspace_count() > copy::MAX_ENTRIES {
            return Err(inspection_error(
                "inspection logical namespace limit exceeded",
            ));
        }
        let mut budget = copy::Budget::default();
        let mut namespaces = BTreeMap::new();
        for name in database.list_keyspace_names() {
            budget.add(name.len() as u64, 0)?;
            if !database.keyspace_exists(&name) {
                return Err(inspection_error("inspection existing namespace changed"));
            }
            // Fjall exposes existing handles through this same API. No other
            // owner can create/delete namespaces during this exclusive read.
            let keyspace = database
                .keyspace(&name, KeyspaceCreateOptions::default)
                .map_err(|_| inspection_error("inspection existing namespace unreadable"))?;
            let mut records = BTreeMap::new();
            for guard in keyspace.iter() {
                let (key, value) = guard
                    .into_inner()
                    .map_err(|_| inspection_error("inspection existing record unreadable"))?;
                let bytes = key
                    .len()
                    .checked_add(value.len())
                    .and_then(|length| u64::try_from(length).ok())
                    .ok_or_else(|| inspection_error("inspection logical arithmetic overflow"))?;
                budget.add(bytes, 0)?;
                records.insert(key.to_vec(), value.to_vec());
            }
            namespaces.insert(name.to_string(), records);
        }
        Ok(Self { namespaces })
    }
}
