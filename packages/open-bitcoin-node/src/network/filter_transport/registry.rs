// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Finite shared admission and settlement, independent of chain and socket locks.

use super::*;

#[derive(Debug, Default)]
pub(super) struct Registry {
    pub(super) next: u64,
    pub(super) active: BTreeMap<u64, Entry>,
    pub(super) terminals: VecDeque<u64>,
    pub(super) failed: bool,
    pub(super) generic: BTreeMap<u64, super::pending::PendingEntry>,
}
impl Registry {
    pub(super) fn pending_bytes(&self) -> Result<u64, FilterTransportError> {
        self.active
            .values()
            .filter_map(|entry| entry.maybe_frame)
            .chain(self.generic.values().map(|entry| entry.bytes))
            .try_fold(0u64, |total, bytes| total.checked_add(bytes))
            .ok_or(FilterTransportError::Exhausted)
    }
    pub(super) fn retire(&mut self, id: u64, disposition: FilterTransportDisposition) {
        if let Some(entry) = self.active.get_mut(&id) {
            match entry.frames.lock() {
                Ok(mut frames) => *frames = VecDeque::new(),
                Err(poison) => {
                    self.failed = true;
                    *poison.into_inner() = VecDeque::new();
                }
            }
            match entry.proof.lock() {
                Ok(mut proof) => proof.maybe_terminal = Some(disposition),
                Err(poison) => {
                    self.failed = true;
                    poison.into_inner().maybe_terminal = Some(disposition);
                }
            }
            // Handed-out bytes remain physically owned by transport. Keep their
            // reservation until that affine capability completes or is dropped.
            entry.suffix = entry.maybe_frame.unwrap_or(0);
            if entry.maybe_frame.is_some() {
                return;
            }
            self.active.remove(&id);
            self.terminals.push_back(id);
            if self.terminals.len() > 64 {
                self.terminals.pop_front();
            }
        }
    }
    pub(super) fn snapshot(&self) -> FilterTransportSnapshot {
        FilterTransportSnapshot {
            requests: self.active.len() as u64,
            suffix_bytes: self.active.values().map(|e| e.suffix).sum(),
            active_frame_bytes: self.active.values().filter_map(|e| e.maybe_frame).sum(),
            generic_pending_bytes: self.generic.values().map(|entry| entry.bytes).sum(),
            generic_descriptors: self.generic.len() as u64,
        }
    }
}
#[derive(Debug, Default)]
pub(in crate::network) struct FilterTransportOwner {
    pub(super) registry: Arc<Mutex<Registry>>,
}
impl FilterTransportOwner {
    pub(in crate::network) fn owns(&self, lease: &FilterRequestLease) -> bool {
        Arc::ptr_eq(&self.registry, &lease.inner.registry)
    }
    #[cfg(test)]
    pub(in crate::network) fn raw_snapshot(&self) -> FilterTransportSnapshot {
        self.registry
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .snapshot()
    }
    pub(in crate::network) fn mark_failed(&self) {
        let mut registry = self
            .registry
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        registry.failed = true;
        let ids: Vec<_> = registry.active.keys().copied().collect();
        for id in ids {
            registry.retire(id, FilterTransportDisposition::PhysicalPrefixInvalidated);
        }
    }
    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Registry>, FilterTransportError> {
        let registry = self
            .registry
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        if registry.failed {
            return Err(FilterTransportError::Poisoned);
        }
        Ok(registry)
    }
    pub(in crate::network) fn snapshot(
        &self,
    ) -> Result<FilterTransportSnapshot, FilterTransportError> {
        Ok(self.lock()?.snapshot())
    }
    /// Reserve a peer/request slot before gathering output; resize before materializing bytes.
    pub(in crate::network) fn reserve(
        &self,
        peer: PeerId,
        suffix: u64,
    ) -> Result<FilterRequestLease, FilterTransportError> {
        let mut registry = self.lock()?;
        let snapshot = registry.snapshot();
        let maybe_peer = registry.active.values().find(|e| e.peer == peer);
        admit_response_reservation(
            u64::from(maybe_peer.is_some()),
            snapshot.requests,
            maybe_peer.map_or(0, |e| e.suffix),
            snapshot.suffix_bytes,
            suffix,
        )
        .map_err(FilterTransportError::Capacity)?;
        let id = registry
            .next
            .checked_add(1)
            .ok_or(FilterTransportError::Exhausted)?;
        registry.next = id;
        let proof = Arc::new(Mutex::new(Proof::default()));
        let frames = Arc::new(Mutex::new(VecDeque::new()));
        registry.active.insert(
            id,
            Entry {
                peer,
                suffix,
                maybe_identity: None,
                maybe_expected_frames: None,
                next: 0,
                maybe_frame: None,
                proof: Arc::clone(&proof),
                frames: Arc::clone(&frames),
            },
        );
        Ok(FilterRequestLease {
            inner: Arc::new(Lease {
                id,
                peer,
                registry: Arc::clone(&self.registry),
                proof,
                frames,
            }),
        })
    }
    pub(in crate::network) fn invalidate(
        &self,
        mut current: impl FnMut(
            PeerId,
            Option<FilterTransportIdentity>,
        ) -> Result<bool, FilterTransportError>,
    ) -> Result<(), FilterTransportError> {
        let identities: Vec<_> = self
            .lock()?
            .active
            .iter()
            .map(|(id, e)| (*id, e.peer, e.maybe_identity.clone()))
            .collect();
        let mut ids = Vec::new();
        for (id, peer, identity) in identities {
            match current(peer, identity) {
                Ok(true) => {}
                Ok(false) => ids.push(id),
                Err(error) => {
                    self.mark_failed();
                    return Err(error);
                }
            }
        }
        let mut registry = self.lock()?;
        for id in ids {
            registry.retire(id, FilterTransportDisposition::PhysicalPrefixInvalidated);
        }
        Ok(())
    }
    pub(in crate::network) fn complete(
        &self,
        mut receipt: FilterFrameReceipt,
        current: bool,
    ) -> Result<FilterTransportCompletion, FilterTransportError> {
        if !Arc::ptr_eq(&self.registry, &receipt.lease.registry) {
            return Ok(FilterTransportCompletion {
                disposition: FilterTransportDisposition::DuplicateOrForeign,
                progress: receipt.lease.progress()?,
            });
        }
        let mut registry = match self.registry.lock() {
            Ok(registry) => registry,
            Err(poison) => {
                let mut registry = poison.into_inner();
                registry.failed = true;
                registry
            }
        };
        let registry_failed = registry.failed;
        let id = receipt.lease.id;
        let mut proof = receipt
            .lease
            .proof
            .lock()
            .map_err(|_| FilterTransportError::Poisoned)?;
        if proof.progress.messages != receipt.sequence {
            return Err(FilterTransportError::Sequence);
        }
        if proof.maybe_terminal.is_some() {
            if registry.active.get(&id).is_none_or(|entry| {
                entry.next != receipt.sequence || entry.maybe_frame != Some(receipt.bytes)
            }) {
                receipt.consumed = true;
                return Ok(FilterTransportCompletion {
                    disposition: FilterTransportDisposition::DuplicateOrForeign,
                    progress: proof.progress,
                });
            }
            // A successful in-flight frame is factual even after removal of its reservation.
            if matches!(
                proof.maybe_terminal,
                Some(
                    FilterTransportDisposition::PhysicalPrefixInvalidated
                        | FilterTransportDisposition::Aborted
                )
            ) {
                proof.progress.messages += 1;
                proof.progress.bytes = proof
                    .progress
                    .bytes
                    .checked_add(receipt.bytes)
                    .ok_or(FilterTransportError::Exhausted)?;
                proof.maybe_terminal = Some(FilterTransportDisposition::PhysicalPrefixInvalidated);
                receipt.consumed = true;
            }
            let completion = FilterTransportCompletion {
                disposition: proof
                    .maybe_terminal
                    .ok_or(FilterTransportError::Invalidated)?,
                progress: proof.progress,
            };
            drop(proof);
            if let Some(entry) = registry.active.get_mut(&id) {
                entry.maybe_frame = None;
            }
            registry.retire(id, completion.disposition);
            return if registry_failed {
                Err(FilterTransportError::Poisoned)
            } else {
                Ok(completion)
            };
        }
        let Some(entry) = registry.active.get_mut(&id) else {
            return Err(FilterTransportError::Foreign);
        };
        if entry.next != receipt.sequence || entry.maybe_frame != Some(receipt.bytes) {
            return Err(FilterTransportError::Sequence);
        }
        proof.progress.messages += 1;
        proof.progress.bytes = proof
            .progress
            .bytes
            .checked_add(receipt.bytes)
            .ok_or(FilterTransportError::Exhausted)?;
        entry.next += 1;
        entry.suffix = entry
            .suffix
            .checked_sub(receipt.bytes)
            .ok_or(FilterTransportError::Length)?;
        entry.maybe_frame = None;
        let disposition = if !current || registry_failed {
            FilterTransportDisposition::PhysicalPrefixInvalidated
        } else if entry.maybe_expected_frames == Some(entry.next) && entry.suffix == 0 {
            FilterTransportDisposition::CurrentRangeComplete
        } else {
            FilterTransportDisposition::WrittenPrefix
        };
        let progress = proof.progress;
        drop(proof);
        if disposition != FilterTransportDisposition::WrittenPrefix {
            registry.retire(id, disposition);
        }
        receipt.consumed = true;
        let completion = FilterTransportCompletion {
            disposition,
            progress,
        };
        if registry_failed {
            Err(FilterTransportError::Poisoned)
        } else {
            Ok(completion)
        }
    }
}
impl Drop for FilterTransportOwner {
    fn drop(&mut self) {
        let mut registry = self
            .registry
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let ids: Vec<_> = registry.active.keys().copied().collect();
        for id in ids {
            registry.retire(id, FilterTransportDisposition::PhysicalPrefixInvalidated);
        }
    }
}
