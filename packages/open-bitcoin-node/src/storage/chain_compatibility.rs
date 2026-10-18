// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Non-opening observations. No observation currently authorizes ordinary storage access.

use std::{fmt, fs, io, path::Path};

use open_bitcoin_core::{
    consensus::{CanonicalChain, MissingValidationPrerequisite, ValidationScope},
    primitives::BlockHash,
};

impl super::FjallNodeStore {
    /// Refuse before any backend effect until canonical ordinary validation is supported.
    pub fn open_ordinary(
        path: impl AsRef<Path>,
        chain: CanonicalChain,
    ) -> Result<Self, CompatibilityRefusal> {
        Err(CompatibilityRefusal::from_observation(
            chain,
            inspect_datadir_without_backend(path.as_ref()),
        ))
    }
}

#[cfg(test)]
thread_local! {
    pub(super) static ORDINARY_BACKEND_OPEN_SPY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    pub(super) static BACKEND_BUILDER_CALLS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(super) fn record_backend_builder_call() {
    BACKEND_BUILDER_CALLS.with(|calls| calls.set(calls.get() + 1));
}

/// Coarse errors deliberately omit paths, backend messages and file contents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionIoKind {
    PermissionDenied,
    Other,
}

/// Filesystem shape only; even canonical-looking files carry no identity proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatadirObservation {
    Missing,
    EmptyDirectory,
    ExistingUnproved,
    UnsafePath,
    InspectionIoFailure(InspectionIoKind),
}

impl DatadirObservation {
    /// Stable bounded observation code for diagnostics.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::EmptyDirectory => "empty_directory",
            Self::ExistingUnproved => "existing_unproved",
            Self::UnsafePath => "unsafe_path",
            Self::InspectionIoFailure(InspectionIoKind::PermissionDenied) => "permission_denied",
            Self::InspectionIoFailure(InspectionIoKind::Other) => "inspection_io_other",
        }
    }
}

/// A shape observation cannot prove either stored network or stored root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservedChainIdentity {
    Unavailable,
}

/// Kept separate from root identity for the later exact-work compatibility gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkProfileObservation {
    Unavailable,
}

/// Refusal policy only; filesystem shape never identifies a stored work format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkCompatibilityDecision {
    BootstrapUnavailable,
    ExistingUnproved,
    UnsafePath,
    InspectionIoFailure,
}

impl WorkCompatibilityDecision {
    /// Fixed redacted policy code, separate from observed work evidence.
    pub const fn code(self) -> &'static str {
        match self {
            Self::BootstrapUnavailable => "bootstrap_unavailable",
            Self::ExistingUnproved => "existing_unproved",
            Self::UnsafePath => "unsafe_path",
            Self::InspectionIoFailure => "inspection_io_failure",
        }
    }
}

/// An unproved store is never described as a proved wrong-network store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityReason {
    ExistingStateUnproved,
    UnsafePath,
    InspectionIoFailure,
    UnsupportedValidationScope,
}

impl CompatibilityReason {
    /// Stable reason for adapter-facing refusals.
    pub const fn code(self) -> &'static str {
        match self {
            Self::ExistingStateUnproved => "existing_state_unproved",
            Self::UnsafePath => "unsafe_path",
            Self::InspectionIoFailure => "inspection_io_failure",
            Self::UnsupportedValidationScope => "unsupported_validation_scope",
        }
    }
}

/// Fixed facts only: caller-controlled paths or backend text cannot enter diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityRefusal {
    selected_chain: CanonicalChain,
    observation: DatadirObservation,
}

impl CompatibilityRefusal {
    /// Every filesystem observation currently maps to refusal, never an admission token.
    pub const fn from_observation(chain: CanonicalChain, observation: DatadirObservation) -> Self {
        Self {
            selected_chain: chain,
            observation,
        }
    }

    /// Selected canonical profile, not a claim about stored data.
    pub const fn selected_chain(&self) -> CanonicalChain {
        self.selected_chain
    }

    /// Immutable selected validation scope.
    pub const fn selected_scope(&self) -> ValidationScope {
        self.selected_chain.spec().validation_scope()
    }

    /// Expected root for the selected profile, not an observed root.
    pub fn selected_genesis_hash(&self) -> BlockHash {
        self.selected_chain.spec().genesis_hash()
    }

    /// Bounded filesystem observation that caused this refusal.
    pub const fn observation(&self) -> DatadirObservation {
        self.observation
    }

    /// Filesystem shape provides no stored identity evidence.
    pub const fn observed_identity(&self) -> ObservedChainIdentity {
        ObservedChainIdentity::Unavailable
    }

    /// Filesystem shape provides no stored work-profile evidence.
    pub const fn work_profile(&self) -> WorkProfileObservation {
        WorkProfileObservation::Unavailable
    }

    /// Existing bytes remain unproved, including legacy and exact-looking records.
    pub const fn work_decision(&self) -> WorkCompatibilityDecision {
        match self.observation {
            DatadirObservation::Missing | DatadirObservation::EmptyDirectory => {
                WorkCompatibilityDecision::BootstrapUnavailable
            }
            DatadirObservation::ExistingUnproved => WorkCompatibilityDecision::ExistingUnproved,
            DatadirObservation::UnsafePath => WorkCompatibilityDecision::UnsafePath,
            DatadirObservation::InspectionIoFailure(_) => {
                WorkCompatibilityDecision::InspectionIoFailure
            }
        }
    }

    /// Typed unavailable prerequisites for ordinary history.
    pub const fn missing_prerequisites(&self) -> &'static [MissingValidationPrerequisite] {
        self.selected_scope().missing_prerequisites()
    }

    /// Distinguishes unproved state, unsafe paths, I/O failures and absent support.
    pub const fn reason(&self) -> CompatibilityReason {
        match self.observation {
            DatadirObservation::Missing | DatadirObservation::EmptyDirectory => {
                CompatibilityReason::UnsupportedValidationScope
            }
            DatadirObservation::ExistingUnproved => CompatibilityReason::ExistingStateUnproved,
            DatadirObservation::UnsafePath => CompatibilityReason::UnsafePath,
            DatadirObservation::InspectionIoFailure(_) => CompatibilityReason::InspectionIoFailure,
        }
    }

    /// Safe fixed guidance; neither a repair instruction nor a claim of usable bootstrap.
    pub const fn guidance(&self) -> &'static str {
        "Review existing data separately. Use a separate datadir only after the missing validation prerequisites are implemented and verified; no repair or conversion was attempted."
    }
}

impl fmt::Display for CompatibilityRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: selected_network={}; selected_genesis=",
            self.reason().code(),
            self.selected_chain
        )?;
        for byte in self.selected_genesis_hash().as_bytes().iter().rev() {
            write!(f, "{byte:02x}")?;
        }
        write!(
            f,
            "; observation={}; observed_network=unavailable; observed_root=unavailable; observed_work=unavailable; scope=identity_only; ordinary_history=false; missing=",
            self.observation.code()
        )?;
        for (index, prerequisite) in self.missing_prerequisites().iter().enumerate() {
            if index != 0 {
                f.write_str(",")?;
            }
            f.write_str(prerequisite.code())?;
        }
        write!(
            f,
            "; {}; work_decision={}",
            self.guidance(),
            self.work_decision().code()
        )
    }
}

impl std::error::Error for CompatibilityRefusal {}

impl serde::Serialize for CompatibilityRefusal {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Inspect ancestor shape and at most one directory entry, without opening a backend,
/// reading source-file contents or creating anything. Concurrent path changes cannot
/// authorize access: every possible result is refused by the ordinary opener.
pub fn inspect_datadir_without_backend(path: &Path) -> DatadirObservation {
    inspect_with(
        path,
        |path| fs::symlink_metadata(path),
        |directory| fs::read_dir(directory).map(|mut entries| classify_first_entry(&mut entries)),
    )
}

fn inspect_with(
    path: &Path,
    metadata: impl Fn(&Path) -> io::Result<fs::Metadata>,
    first_entry: impl FnOnce(&Path) -> io::Result<DatadirObservation>,
) -> DatadirObservation {
    if path.as_os_str().is_empty() {
        return DatadirObservation::UnsafePath;
    }
    if path.is_relative() {
        let cwd = match std::env::current_dir() {
            Ok(cwd) => cwd,
            Err(error) => return io_observation(error),
        };
        if let Some(observation) = maybe_component_refusal(&cwd, &metadata) {
            return observation;
        }
    }
    if let Some(observation) = maybe_component_refusal(path, &metadata) {
        return observation;
    }
    match first_entry(path) {
        Ok(observation) => observation,
        Err(error) => io_observation(error),
    }
}

fn maybe_component_refusal(
    path: &Path,
    metadata: &impl Fn(&Path) -> io::Result<fs::Metadata>,
) -> Option<DatadirObservation> {
    // Revisit borrowed prefixes instead of allocating a depth-sized ancestor stack.
    // Root-to-leaf order rejects symlinks before inspecting anything beneath them.
    for ancestor in (0..path.ancestors().count())
        .rev()
        .filter_map(|depth| path.ancestors().nth(depth))
    {
        // A trailing slash or dot must not let lstat follow a final symlink.
        let ancestor = ancestor.components().as_path();
        if ancestor.as_os_str().is_empty() {
            continue;
        }
        match metadata(ancestor) {
            Ok(metadata) if metadata.file_type().is_dir() => {}
            Ok(_) => return Some(DatadirObservation::UnsafePath),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Some(DatadirObservation::Missing);
            }
            Err(error) => return Some(io_observation(error)),
        }
    }
    None
}

fn classify_first_entry<T>(
    entries: &mut impl Iterator<Item = io::Result<T>>,
) -> DatadirObservation {
    match entries.next() {
        None => DatadirObservation::EmptyDirectory,
        Some(Ok(_)) => DatadirObservation::ExistingUnproved,
        Some(Err(error)) => io_observation(error),
    }
}

fn io_observation(error: io::Error) -> DatadirObservation {
    DatadirObservation::InspectionIoFailure(match error.kind() {
        io::ErrorKind::PermissionDenied => InspectionIoKind::PermissionDenied,
        _ => InspectionIoKind::Other,
    })
}

#[cfg(test)]
#[path = "chain_compatibility/tests/phase165_work.rs"]
mod phase165_work;
#[cfg(test)]
#[path = "chain_compatibility/tests/preservation.rs"]
mod preservation;
#[cfg(test)]
mod tests;
