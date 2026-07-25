// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp

use std::collections::{HashMap, HashSet};
use std::fmt;

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{OutPoint, Transaction, Txid, Wtxid};

use super::{
    MAX_PACKAGE_COUNT, MAX_PACKAGE_WEIGHT, PackageFingerprint, PackageMember, SubmissionPackage,
    SubmissionPackageKind, WellFormedPackage,
};
use crate::{MempoolMemberIdentity, transaction_weight_and_virtual_size};

/// Failure to establish a package shape or submission refinement invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageShapeError {
    Empty,
    TooManyTransactions {
        count: usize,
        maximum: usize,
    },
    TransactionEncoding {
        index: usize,
        reason: String,
    },
    TotalWeightOverflow,
    TooHeavy {
        weight: usize,
        maximum: usize,
    },
    DuplicateTxid {
        txid: Txid,
    },
    DuplicateWtxid {
        wtxid: Wtxid,
    },
    ZeroInputMember {
        index: usize,
    },
    ChildBeforeParent {
        child_index: usize,
        parent_txid: Txid,
    },
    CrossMemberInputConflict {
        outpoint: OutPoint,
        first_index: usize,
        second_index: usize,
    },
    NotChildWithUnconfirmedParents,
    MissingUnconfirmedParent {
        outpoint: OutPoint,
    },
}

pub(super) fn transaction_encoding_error(index: usize, reason: impl ToString) -> PackageShapeError {
    PackageShapeError::TransactionEncoding {
        index,
        reason: reason.to_string(),
    }
}

impl fmt::Display for PackageShapeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "package must contain at least one transaction"),
            Self::TooManyTransactions { count, maximum } => {
                write!(
                    formatter,
                    "package contains {count} transactions; maximum is {maximum}"
                )
            }
            Self::TransactionEncoding { index, reason } => {
                write!(
                    formatter,
                    "package member {index} could not be encoded: {reason}"
                )
            }
            Self::TotalWeightOverflow => write!(formatter, "package total weight overflowed"),
            Self::TooHeavy { weight, maximum } => {
                write!(
                    formatter,
                    "package weight {weight} exceeds maximum {maximum}"
                )
            }
            Self::DuplicateTxid { txid } => {
                write!(formatter, "package repeats txid {txid:?}")
            }
            Self::DuplicateWtxid { wtxid } => {
                write!(formatter, "package repeats wtxid {wtxid:?}")
            }
            Self::ZeroInputMember { index } => {
                write!(formatter, "package member {index} has no inputs")
            }
            Self::ChildBeforeParent {
                child_index,
                parent_txid,
            } => write!(
                formatter,
                "package member {child_index} appears before parent {parent_txid:?}"
            ),
            Self::CrossMemberInputConflict {
                outpoint,
                first_index,
                second_index,
            } => write!(
                formatter,
                "package members {first_index} and {second_index} both spend {outpoint:?}"
            ),
            Self::NotChildWithUnconfirmedParents => {
                write!(
                    formatter,
                    "package is not one child with its unconfirmed parents"
                )
            }
            Self::MissingUnconfirmedParent { outpoint } => write!(
                formatter,
                "child input {outpoint:?} is neither supplied nor present in chainstate"
            ),
        }
    }
}

impl std::error::Error for PackageShapeError {}

impl TryFrom<Vec<Transaction>> for WellFormedPackage {
    type Error = PackageShapeError;

    fn try_from(transactions: Vec<Transaction>) -> Result<Self, Self::Error> {
        if transactions.is_empty() {
            return Err(PackageShapeError::Empty);
        }
        if transactions.len() > MAX_PACKAGE_COUNT {
            return Err(PackageShapeError::TooManyTransactions {
                count: transactions.len(),
                maximum: MAX_PACKAGE_COUNT,
            });
        }

        let mut members = Vec::with_capacity(transactions.len());
        let mut txids = HashSet::with_capacity(transactions.len());
        let mut wtxids = HashSet::with_capacity(transactions.len());
        let mut total_weight = 0_usize;
        for (input_index, transaction) in transactions.into_iter().enumerate() {
            let txid = transaction_txid(&transaction)
                .map_err(|source| transaction_encoding_error(input_index, source))?;
            let wtxid = transaction_wtxid(&transaction)
                .map_err(|source| transaction_encoding_error(input_index, source))?;
            if !wtxids.insert(wtxid) {
                return Err(PackageShapeError::DuplicateWtxid { wtxid });
            }
            if !txids.insert(txid) {
                return Err(PackageShapeError::DuplicateTxid { txid });
            }

            let (weight, _) = transaction_weight_and_virtual_size(&transaction)
                .map_err(|source| transaction_encoding_error(input_index, source))?;
            total_weight = total_weight
                .checked_add(weight)
                .ok_or(PackageShapeError::TotalWeightOverflow)?;
            if total_weight > MAX_PACKAGE_WEIGHT {
                return Err(PackageShapeError::TooHeavy {
                    weight: total_weight,
                    maximum: MAX_PACKAGE_WEIGHT,
                });
            }

            members.push(PackageMember {
                transaction,
                identity: MempoolMemberIdentity { txid, wtxid },
                weight,
                input_index,
            });
        }

        validate_topology_and_conflicts(&members, &txids)?;
        let fingerprint = PackageFingerprint::from_members(&members);
        Ok(Self {
            members,
            fingerprint,
        })
    }
}

impl SubmissionPackage {
    /// Refines a context-free package into a checked submission capability.
    pub fn try_from_package(
        package: WellFormedPackage,
        chainstate: &ChainstateSnapshot,
    ) -> Result<Self, PackageShapeError> {
        if package.len() == 1 {
            return Ok(Self {
                package,
                kind: SubmissionPackageKind::Single,
            });
        }

        let Some(child) = package.members.last() else {
            return Err(PackageShapeError::Empty);
        };
        let child_parent_txids: HashSet<Txid> = child
            .transaction
            .inputs
            .iter()
            .map(|input| input.previous_output.txid)
            .collect();
        let direct_parent_txids: HashSet<Txid> = package.members[..package.members.len() - 1]
            .iter()
            .map(|member| member.identity.txid)
            .collect();
        if !direct_parent_txids
            .iter()
            .all(|txid| child_parent_txids.contains(txid))
        {
            return Err(PackageShapeError::NotChildWithUnconfirmedParents);
        }

        for input in &child.transaction.inputs {
            if direct_parent_txids.contains(&input.previous_output.txid) {
                continue;
            }
            if !chainstate.utxos.contains_key(&input.previous_output) {
                return Err(PackageShapeError::MissingUnconfirmedParent {
                    outpoint: input.previous_output.clone(),
                });
            }
        }

        Ok(Self {
            package,
            kind: SubmissionPackageKind::ChildWithUnconfirmedParents,
        })
    }
}

fn validate_topology_and_conflicts(
    members: &[PackageMember],
    all_txids: &HashSet<Txid>,
) -> Result<(), PackageShapeError> {
    let mut earlier_txids = HashSet::with_capacity(members.len());
    let mut prior_spenders: HashMap<OutPoint, usize> = HashMap::new();
    for member in members {
        if member.transaction.inputs.is_empty() {
            return Err(PackageShapeError::ZeroInputMember {
                index: member.input_index,
            });
        }

        let mut member_inputs = HashSet::with_capacity(member.transaction.inputs.len());
        for input in &member.transaction.inputs {
            let parent_txid = input.previous_output.txid;
            if all_txids.contains(&parent_txid) && !earlier_txids.contains(&parent_txid) {
                return Err(PackageShapeError::ChildBeforeParent {
                    child_index: member.input_index,
                    parent_txid,
                });
            }
            member_inputs.insert(input.previous_output.clone());
        }
        for outpoint in member_inputs {
            if let Some(first_index) = prior_spenders.get(&outpoint).copied() {
                return Err(PackageShapeError::CrossMemberInputConflict {
                    outpoint,
                    first_index,
                    second_index: member.input_index,
                });
            }
            prior_spenders.insert(outpoint, member.input_index);
        }
        earlier_txids.insert(member.identity.txid);
    }

    Ok(())
}
