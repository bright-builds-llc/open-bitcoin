// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Saved active indexing follows the existing full live-owner startup path.

#[cfg(test)]
use crate::storage::fjall_store::filters::FilterPublicationFault;
use crate::{FjallNodeStore, StorageError, chainstate::BasicFilterStartupMode};

/// The caller has consumed and checked the sealed canonical store capability.
/// Generic PreserveSaved recovery may remain metadata-only; canonical Active
/// startup needs Enabled's retained-input checks and genuine append capability.
pub(super) fn resolve_startup_mode(
    store: &FjallNodeStore,
    requested: BasicFilterStartupMode,
) -> Result<BasicFilterStartupMode, StorageError> {
    if matches!(requested, BasicFilterStartupMode::PreserveSaved)
        && store.resolved_basic_filter_indexing_after_startup()? == Some(true)
    {
        return Ok(BasicFilterStartupMode::Enabled);
    }
    Ok(requested)
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Stage {
    Configure,
    Append,
}

#[cfg(test)]
thread_local! {
    pub(super) static MAYBE_FAULT: std::cell::Cell<Option<(Stage, FilterPublicationFault)>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(super) fn inject(store: &FjallNodeStore, stage: Stage) {
    if let Some((selected, fault)) = MAYBE_FAULT.get()
        && selected == stage
    {
        MAYBE_FAULT.set(None);
        store.set_basic_filter_fault(fault);
    }
}
