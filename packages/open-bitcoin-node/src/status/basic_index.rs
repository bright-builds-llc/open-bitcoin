// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

//! Identifier-free BASIC observations; collecting these facts grants no authority.

use serde::{Deserialize, Serialize};

use super::FieldAvailability;

/// Fixed reasons permitted on the BASIC wire and support surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicIndexUnavailableReason {
    NotReported,
    LiveUnavailable,
    RetentionIncomplete,
    RetentionFailed,
    RetentionBudgetExceeded,
    Backend,
    Integrity,
    Invalidated,
}

impl BasicIndexUnavailableReason {
    /// Normalize direct Rust and wire reasons to the same exact safe vocabulary.
    pub fn normalize(reason: &str) -> &'static str {
        use BasicIndexUnavailableReason::*;
        [
            NotReported,
            LiveUnavailable,
            RetentionIncomplete,
            RetentionFailed,
            RetentionBudgetExceeded,
            Backend,
            Integrity,
            Invalidated,
        ]
        .into_iter()
        .map(Self::as_str)
        .find(|label| *label == reason)
        .unwrap_or(NotReported.as_str())
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotReported => "BASIC evidence not reported",
            Self::LiveUnavailable => "live BASIC evidence unavailable",
            Self::RetentionIncomplete => "retention measurement incomplete",
            Self::RetentionFailed => "retention measurement failed",
            Self::RetentionBudgetExceeded => "retention measurement budget exceeded",
            Self::Backend => "BASIC backend evidence unavailable",
            Self::Integrity => "BASIC integrity evidence unavailable",
            Self::Invalidated => "BASIC lifecycle evidence invalidated",
        }
    }
}

/// Independently available configuration, owner progress, growth and protection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct BasicIndexOperatorStatus {
    #[serde(with = "safe_availability")]
    pub configured_indexing: FieldAvailability<bool>,
    #[serde(with = "safe_availability")]
    pub configured_peer_serving: FieldAvailability<bool>,
    #[serde(with = "safe_availability")]
    pub peer_scope: FieldAvailability<BasicPeerScopeStatus>,
    #[serde(with = "safe_availability")]
    pub operational_state: FieldAvailability<BasicIndexOperationalState>,
    #[serde(with = "safe_availability")]
    pub initial_sync_complete: FieldAvailability<bool>,
    #[serde(rename = "accepted_height", with = "safe_availability")]
    pub maybe_accepted_height: FieldAvailability<Option<u32>>,
    #[serde(rename = "processed_height", with = "safe_availability")]
    pub maybe_processed_height: FieldAvailability<Option<u32>>,
    #[serde(rename = "safe_durable_height", with = "safe_availability")]
    pub maybe_safe_durable_height: FieldAvailability<Option<u32>>,
    #[serde(rename = "lag_blocks", with = "safe_availability")]
    pub maybe_lag_blocks: FieldAvailability<Option<u64>>,
    #[serde(rename = "failure", with = "safe_availability")]
    pub maybe_failure: FieldAvailability<Option<BasicIndexFailureCategory>>,
    #[serde(with = "safe_availability")]
    pub retained_availability: FieldAvailability<BasicRetainedAvailability>,
    #[serde(with = "safe_availability")]
    pub retention: FieldAvailability<BasicRetentionMeasurement>,
    #[serde(with = "safe_availability")]
    pub protection: FieldAvailability<BasicIndexProtection>,
    #[serde(with = "safe_availability")]
    pub next_action: FieldAvailability<BasicIndexNextAction>,
}

impl BasicIndexOperatorStatus {
    /// No configuration or live facts are inferred from absent collection.
    pub fn unavailable(reason: BasicIndexUnavailableReason) -> Self {
        let reason = reason.as_str();
        Self {
            configured_indexing: FieldAvailability::unavailable(reason),
            configured_peer_serving: FieldAvailability::unavailable(reason),
            peer_scope: FieldAvailability::unavailable(reason),
            operational_state: FieldAvailability::unavailable(reason),
            initial_sync_complete: FieldAvailability::unavailable(reason),
            maybe_accepted_height: FieldAvailability::unavailable(reason),
            maybe_processed_height: FieldAvailability::unavailable(reason),
            maybe_safe_durable_height: FieldAvailability::unavailable(reason),
            maybe_lag_blocks: FieldAvailability::unavailable(reason),
            maybe_failure: FieldAvailability::unavailable(reason),
            retained_availability: FieldAvailability::unavailable(reason),
            retention: FieldAvailability::unavailable(reason),
            protection: FieldAvailability::unavailable(reason),
            next_action: FieldAvailability::unavailable(reason),
        }
    }
}

impl Default for BasicIndexOperatorStatus {
    fn default() -> Self {
        Self::unavailable(BasicIndexUnavailableReason::NotReported)
    }
}

/// Aggregate effective capability provenance; categories may overlap per session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BasicPeerScopeStatus {
    pub general_sessions: u64,
    pub explicit_sessions: u64,
    pub all_sessions: u64,
    pub implicit_sessions: u64,
    pub offered_sessions: u64,
}

/// Owner-projected state, independent of configuration and retained evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicIndexOperationalState {
    Disabled,
    CatchingUp,
    Current,
    Lagged,
    Paused,
    Failed,
    Invalidated,
}

/// Bounded classification; backend messages and identifiers are never payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicIndexFailureCategory {
    RequiredBody,
    RequiredUndo,
    Backend,
    IntegrityOrInvalidFacts,
    ResourceBound,
    Invalidated,
}

/// An observation of retained records does not promise arbitrary lookup success.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicRetainedAvailability {
    Observed,
    NoneSeen,
}

/// Complete unique immutable-row accounting, including retained stale branches.
/// Payload bytes are a subset of key/value bytes, never an additional footprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BasicRetentionMeasurement {
    pub retained_record_count: u64,
    pub encoded_filter_payload_bytes: u64,
    pub retained_record_key_value_bytes: u64,
    pub basis: BasicRetentionBasis,
}

/// Excludes active projections, lifecycle/checkpoints/locks and Fjall overhead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicRetentionBasis {
    ImmutableRecordKeysAndValuesIncludingStaleBranches,
}

/// Reserved BASIC-owned inclusive bounds, without a raw lock name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum BasicIndexProtection {
    None,
    InclusiveRange(BasicIndexProtectionRange),
    HeightSpaceExhausted,
}

/// Validated inclusive bounds. Deserialization also rejects reversed ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ProtectionRangeWire")]
pub struct BasicIndexProtectionRange {
    height_first: u32,
    height_last: u32,
}

impl BasicIndexProtectionRange {
    pub const fn new(height_first: u32, height_last: u32) -> Result<Self, &'static str> {
        if height_first > height_last {
            return Err("BASIC protection bounds reversed");
        }
        Ok(Self {
            height_first,
            height_last,
        })
    }

    pub const fn height_first(self) -> u32 {
        self.height_first
    }

    pub const fn height_last(self) -> u32 {
        self.height_last
    }
}

#[derive(Deserialize)]
struct ProtectionRangeWire {
    height_first: u32,
    height_last: u32,
}

impl TryFrom<ProtectionRangeWire> for BasicIndexProtectionRange {
    type Error = &'static str;

    fn try_from(value: ProtectionRangeWire) -> Result<Self, Self::Error> {
        Self::new(value.height_first, value.height_last)
    }
}

/// Fixed guidance selected by the authority; no repair or mutation instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicIndexNextAction {
    Disabled,
    EmptyPrefix,
    InitialCatchUp,
    Current,
    Lagged,
    ProcessedAheadOfDurable,
    RequiredBody,
    RequiredUndo,
    Backend,
    IntegrityOrInvalidFacts,
    ResourceBound,
    Invalidated,
    CollectLiveEvidence,
    InspectSupportEvidence,
}

// FieldAvailability is shared with older contracts that accept free text. Narrow
// this section at its serde boundary even when a caller constructs it directly.
mod safe_availability {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{BasicIndexUnavailableReason, FieldAvailability};

    pub(super) fn serialize<T: Serialize, S: Serializer>(
        value: &FieldAvailability<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            FieldAvailability::Available(_) => value.serialize(serializer),
            FieldAvailability::Unavailable { reason } => {
                FieldAvailability::<&T>::unavailable(BasicIndexUnavailableReason::normalize(reason))
                    .serialize(serializer)
            }
        }
    }

    pub(super) fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<FieldAvailability<T>, D::Error> {
        Ok(match FieldAvailability::deserialize(deserializer)? {
            FieldAvailability::Available(value) => FieldAvailability::available(value),
            FieldAvailability::Unavailable { reason } => {
                FieldAvailability::unavailable(BasicIndexUnavailableReason::normalize(&reason))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn phase161_contract_reason_normalization_matches_exact_vocabulary_only() {
        // Arrange
        use BasicIndexUnavailableReason::*;
        let reasons = [
            NotReported,
            LiveUnavailable,
            RetentionIncomplete,
            RetentionFailed,
            RetentionBudgetExceeded,
            Backend,
            Integrity,
            Invalidated,
        ];
        // Act / Assert
        for reason in reasons {
            assert_eq!(
                BasicIndexUnavailableReason::normalize(reason.as_str()),
                reason.as_str()
            );
            for hostile in [
                format!("{} secret", reason.as_str()),
                format!("\u{1b}[31m{}", reason.as_str()),
                format!("{}\n", reason.as_str()),
            ] {
                assert_eq!(
                    BasicIndexUnavailableReason::normalize(&hostile),
                    NotReported.as_str()
                );
            }
        }
        assert_eq!(
            BasicIndexUnavailableReason::normalize("arbitrary private detail"),
            NotReported.as_str()
        );
    }

    #[test]
    fn phase161_contract_partial_section_defaults_only_unreported_facts() {
        // Arrange
        let json = json!({"configured_indexing": {"state": "available", "value": false}});

        // Act
        let status: BasicIndexOperatorStatus =
            serde_json::from_value(json).expect("partial status");

        // Assert
        assert_eq!(
            status.configured_indexing,
            FieldAvailability::available(false)
        );
        assert!(matches!(
            status.initial_sync_complete,
            FieldAvailability::Unavailable { .. }
        ));
        assert!(matches!(
            status.maybe_lag_blocks,
            FieldAvailability::Unavailable { .. }
        ));
        assert!(matches!(
            status.retention,
            FieldAvailability::Unavailable { .. }
        ));
    }

    #[test]
    fn phase161_contract_empty_endpoint_is_distinct_from_genesis_and_unknown() {
        // Arrange
        let unknown = BasicIndexOperatorStatus::default();
        let empty = BasicIndexOperatorStatus {
            maybe_processed_height: FieldAvailability::available(None),
            maybe_safe_durable_height: FieldAvailability::available(None),
            maybe_accepted_height: FieldAvailability::available(None),
            maybe_lag_blocks: FieldAvailability::available(None),
            ..unknown.clone()
        };
        let genesis = BasicIndexOperatorStatus {
            maybe_processed_height: FieldAvailability::available(Some(0)),
            maybe_safe_durable_height: FieldAvailability::available(Some(0)),
            maybe_accepted_height: FieldAvailability::available(Some(0)),
            maybe_lag_blocks: FieldAvailability::available(Some(0)),
            ..unknown.clone()
        };

        // Act
        let encoded = [unknown, empty, genesis]
            .map(|status| serde_json::to_value(status).expect("status json"));

        // Assert
        for field in [
            "processed_height",
            "safe_durable_height",
            "accepted_height",
            "lag_blocks",
        ] {
            assert_eq!(encoded[0][field]["state"], "unavailable");
            assert_eq!(
                encoded[1][field],
                json!({"state": "available", "value": null})
            );
            assert_eq!(encoded[2][field], json!({"state": "available", "value": 0}));
        }
    }

    #[test]
    fn phase161_contract_earned_initial_sync_coexists_with_lag_and_distinct_durability() {
        // Arrange
        let status = BasicIndexOperatorStatus {
            operational_state: FieldAvailability::available(BasicIndexOperationalState::Lagged),
            initial_sync_complete: FieldAvailability::available(true),
            maybe_accepted_height: FieldAvailability::available(Some(23)),
            maybe_processed_height: FieldAvailability::available(Some(21)),
            maybe_safe_durable_height: FieldAvailability::available(Some(18)),
            maybe_lag_blocks: FieldAvailability::available(Some(2)),
            ..BasicIndexOperatorStatus::default()
        };

        // Act
        let encoded = serde_json::to_value(status).expect("status json");

        // Assert
        assert_eq!(encoded["initial_sync_complete"]["value"], true);
        assert_eq!(encoded["operational_state"]["value"], "lagged");
        assert_eq!(encoded["lag_blocks"]["value"], 2);
        assert_eq!(encoded["processed_height"]["value"], 21);
        assert_eq!(encoded["safe_durable_height"]["value"], 18);
        assert!(encoded.get("maybe_processed_height").is_none());
    }

    #[test]
    fn phase161_contract_fixed_failures_preserve_observed_no_failure() {
        // Arrange
        let categories = [
            (BasicIndexFailureCategory::RequiredBody, "required_body"),
            (BasicIndexFailureCategory::RequiredUndo, "required_undo"),
            (BasicIndexFailureCategory::Backend, "backend"),
            (
                BasicIndexFailureCategory::IntegrityOrInvalidFacts,
                "integrity_or_invalid_facts",
            ),
            (BasicIndexFailureCategory::ResourceBound, "resource_bound"),
            (BasicIndexFailureCategory::Invalidated, "invalidated"),
        ];
        let no_failure = BasicIndexOperatorStatus {
            maybe_failure: FieldAvailability::available(None),
            ..BasicIndexOperatorStatus::default()
        };

        // Act
        let encoded = serde_json::to_value(no_failure).expect("no failure json");
        let failures = categories.map(|(category, expected)| {
            let status = BasicIndexOperatorStatus {
                maybe_failure: FieldAvailability::available(Some(category)),
                ..BasicIndexOperatorStatus::default()
            };
            (
                serde_json::to_value(status).expect("failure json"),
                expected,
            )
        });

        // Assert
        assert_eq!(
            encoded["failure"],
            json!({"state": "available", "value": null})
        );
        for (failure, expected) in failures {
            assert_eq!(failure["failure"]["value"], expected);
            assert_eq!(failure["retained_availability"]["state"], "unavailable");
        }
    }

    #[test]
    fn phase161_contract_serialization_replaces_untrusted_unavailable_text() {
        // Arrange
        let status = BasicIndexOperatorStatus {
            configured_indexing: FieldAvailability::unavailable(
                "/private/wallet secret peer=192.0.2.1 \u{1b}[31m",
            ),
            retention: FieldAvailability::unavailable(
                BasicIndexUnavailableReason::RetentionBudgetExceeded.as_str(),
            ),
            ..BasicIndexOperatorStatus::default()
        };

        // Act
        let encoded = serde_json::to_value(status).expect("safe status json");

        // Assert
        assert_eq!(
            encoded["configured_indexing"]["value"]["reason"],
            "BASIC evidence not reported"
        );
        assert_eq!(
            encoded["retention"]["value"]["reason"],
            "retention measurement budget exceeded"
        );
        assert!(!encoded.to_string().contains("/private"));
        assert!(!encoded.to_string().contains("192.0.2.1"));
    }

    #[test]
    fn phase161_contract_deserialization_drops_untrusted_unavailable_text() {
        // Arrange
        let json = json!({"configured_indexing": {"state": "unavailable", "value": {"reason": "secret backend key"}}});

        // Act
        let status: BasicIndexOperatorStatus =
            serde_json::from_value(json).expect("safe unavailable status");

        // Assert
        assert_eq!(
            status.configured_indexing,
            FieldAvailability::unavailable("BASIC evidence not reported")
        );
    }

    #[test]
    fn phase161_contract_protection_rejects_reversed_bounds() {
        // Arrange
        let invalid =
            json!({"state": "inclusive_range", "value": {"height_first": 9, "height_last": 3}});

        // Act
        let parsed = serde_json::from_value::<BasicIndexProtection>(invalid);
        let constructed = BasicIndexProtectionRange::new(9, 3);

        // Assert
        assert!(parsed.is_err());
        assert!(constructed.is_err());
    }

    #[test]
    fn phase161_contract_protection_distinguishes_absence_range_and_exhaustion() {
        // Arrange
        let range = BasicIndexProtectionRange::new(0, u32::MAX).expect("valid full range");
        let protections = [
            BasicIndexProtection::None,
            BasicIndexProtection::InclusiveRange(range),
            BasicIndexProtection::HeightSpaceExhausted,
        ];

        // Act
        let encoded = protections
            .map(|protection| serde_json::to_value(protection).expect("protection json"));

        // Assert
        assert_eq!(range.height_first(), 0);
        assert_eq!(range.height_last(), u32::MAX);
        assert_eq!(encoded[0]["state"], "none");
        assert_eq!(encoded[1]["value"]["height_first"], 0);
        assert_eq!(encoded[1]["value"]["height_last"], u32::MAX);
        assert_eq!(encoded[2]["state"], "height_space_exhausted");
    }

    #[test]
    fn phase161_contract_failed_owner_retains_independent_logical_growth() {
        // Arrange
        let status = BasicIndexOperatorStatus {
            configured_indexing: FieldAvailability::available(true),
            configured_peer_serving: FieldAvailability::available(false),
            operational_state: FieldAvailability::available(BasicIndexOperationalState::Failed),
            maybe_failure: FieldAvailability::available(Some(
                BasicIndexFailureCategory::RequiredUndo,
            )),
            retained_availability: FieldAvailability::available(
                BasicRetainedAvailability::Observed,
            ),
            retention: FieldAvailability::available(BasicRetentionMeasurement {
                retained_record_count: 3,
                encoded_filter_payload_bytes: 12,
                retained_record_key_value_bytes: 315,
                basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
            }),
            ..BasicIndexOperatorStatus::default()
        };

        // Act
        let encoded = serde_json::to_value(status).expect("independent facts json");

        // Assert
        assert_eq!(encoded["operational_state"]["value"], "failed");
        assert_eq!(encoded["retained_availability"]["value"], "observed");
        assert_eq!(encoded["retention"]["value"]["retained_record_count"], 3);
        assert_eq!(
            encoded["retention"]["value"]["encoded_filter_payload_bytes"],
            12
        );
        assert_eq!(
            encoded["retention"]["value"]["retained_record_key_value_bytes"],
            315
        );
        assert_eq!(
            encoded["retention"]["value"]["basis"],
            "immutable_record_keys_and_values_including_stale_branches"
        );
        assert_eq!(encoded["configured_peer_serving"]["value"], false);
    }

    #[test]
    fn phase161_contract_peer_scope_preserves_distinct_aggregate_provenance() {
        // Arrange
        let scope = BasicPeerScopeStatus {
            general_sessions: 0,
            explicit_sessions: 1,
            all_sessions: 2,
            implicit_sessions: 4,
            offered_sessions: 3,
        };

        // Act
        let encoded = serde_json::to_value(scope).expect("scope json");

        // Assert
        assert_eq!(
            encoded,
            json!({"general_sessions": 0, "explicit_sessions": 1,
            "all_sessions": 2, "implicit_sessions": 4, "offered_sessions": 3})
        );
    }

    #[test]
    fn phase161_contract_guidance_rejects_arbitrary_instructions() {
        // Arrange
        let malicious =
            json!({"next_action": {"state": "available", "value": "download /private/wallet"}});

        // Act
        let parsed = serde_json::from_value::<BasicIndexOperatorStatus>(malicious);

        // Assert
        assert!(parsed.is_err());
    }
}
