use core::fmt;

use open_bitcoin_primitives::{Amount, OutPoint, Txid};

use crate::types::RbfPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitDirection {
    Ancestor,
    Descendant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitKind {
    Count,
    VirtualSize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MempoolError {
    DuplicateTransaction {
        txid: Txid,
    },
    MissingInput {
        outpoint: OutPoint,
    },
    Validation {
        reason: String,
    },
    NonStandard {
        reason: String,
    },
    RelayFeeTooLow {
        fee: Amount,
        required_fee_sats: i64,
        virtual_size: usize,
    },
    ConflictNotAllowed {
        conflicting: Vec<Txid>,
        policy: RbfPolicy,
    },
    ReplacementRejected {
        reason: String,
    },
    LimitExceeded {
        direction: LimitDirection,
        kind: LimitKind,
        txid: Option<Txid>,
        attempted: usize,
        max: usize,
    },
    CandidateEvicted {
        txid: Txid,
    },
}

impl fmt::Display for LimitDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ancestor => write!(f, "ancestor"),
            Self::Descendant => write!(f, "descendant"),
        }
    }
}

impl fmt::Display for LimitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::VirtualSize => write!(f, "virtual size"),
        }
    }
}

impl fmt::Display for MempoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateTransaction { txid } => {
                write!(f, "duplicate mempool transaction {:?}", txid)
            }
            Self::MissingInput { outpoint } => {
                write!(
                    f,
                    "missing input {:?}:{} for mempool admission",
                    outpoint.txid, outpoint.vout
                )
            }
            Self::Validation { reason } => write!(f, "{reason}"),
            Self::NonStandard { reason } => write!(f, "{reason}"),
            Self::RelayFeeTooLow {
                fee,
                required_fee_sats,
                virtual_size,
            } => write!(
                f,
                "transaction fee {} is below relay minimum {} for {} vbytes",
                fee.to_sats(),
                required_fee_sats,
                virtual_size
            ),
            Self::ConflictNotAllowed {
                conflicting,
                policy,
            } => write!(
                f,
                "transaction conflicts with {:?} but replacement policy {:?} disallows it",
                conflicting, policy
            ),
            Self::ReplacementRejected { reason } => write!(f, "{reason}"),
            Self::LimitExceeded {
                direction,
                kind,
                txid,
                attempted,
                max,
            } => {
                if let Some(txid) = txid {
                    write!(
                        f,
                        "{direction} {kind} limit exceeded for {:?}: attempted {}, max {}",
                        txid, attempted, max
                    )
                } else {
                    write!(
                        f,
                        "{direction} {kind} limit exceeded: attempted {}, max {}",
                        attempted, max
                    )
                }
            }
            Self::CandidateEvicted { txid } => {
                write!(
                    f,
                    "candidate {:?} was evicted during mempool trimming",
                    txid
                )
            }
        }
    }
}

impl std::error::Error for MempoolError {}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{Amount, OutPoint, Txid};

    use super::{LimitDirection, LimitKind, MempoolError};
    use crate::types::RbfPolicy;

    fn sample_outpoint() -> OutPoint {
        OutPoint {
            txid: Txid::from_byte_array([3_u8; 32]),
            vout: 1,
        }
    }

    #[test]
    fn display_covers_every_error_variant() {
        let txid = Txid::from_byte_array([9_u8; 32]);
        let outpoint = sample_outpoint();
        let fee = Amount::from_sats(100).expect("valid amount");
        let cases = [
            MempoolError::DuplicateTransaction { txid },
            MempoolError::MissingInput {
                outpoint: outpoint.clone(),
            },
            MempoolError::Validation {
                reason: "validation failed".to_string(),
            },
            MempoolError::NonStandard {
                reason: "non-standard".to_string(),
            },
            MempoolError::RelayFeeTooLow {
                fee,
                required_fee_sats: 200,
                virtual_size: 150,
            },
            MempoolError::ConflictNotAllowed {
                conflicting: vec![txid],
                policy: RbfPolicy::Never,
            },
            MempoolError::ReplacementRejected {
                reason: "replacement too cheap".to_string(),
            },
            MempoolError::LimitExceeded {
                direction: LimitDirection::Ancestor,
                kind: LimitKind::Count,
                txid: Some(txid),
                attempted: 3,
                max: 2,
            },
            MempoolError::CandidateEvicted { txid },
        ];

        for error in cases {
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn direction_and_kind_display_cover_remaining_variants() {
        assert_eq!(LimitDirection::Descendant.to_string(), "descendant");
        assert_eq!(LimitKind::VirtualSize.to_string(), "virtual size");

        let error = MempoolError::LimitExceeded {
            direction: LimitDirection::Descendant,
            kind: LimitKind::VirtualSize,
            txid: None,
            attempted: 5,
            max: 4,
        };

        assert!(
            error
                .to_string()
                .contains("descendant virtual size limit exceeded")
        );
    }
}
