// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/clientversion.h
// - packages/bitcoin-knots/src/clientversion.cpp
// - packages/bitcoin-knots/src/pow.cpp

//! Shared pure header decisions. Passing these checks never establishes earned lineage.

use super::CandidateHistory;
use crate::block::{check_block_header_contextual, check_block_header_with_params};
use crate::validation::{BlockValidationError, BlockValidationResult, block_error};

/// Pinned2026 default: (2026-1970+2)*31558060+26784000.
pub const DEFAULT_SOFTWARE_EXPIRY: i64 = 1_857_151_480;

impl CandidateHistory {
    /// Check selected PoW and contextual rules against an explicitly shell-observed clock.
    /// Structural/independent seed classifications remain unchanged; only the canonical
    /// lineage owner can establish that every supplied ancestor was actually admitted.
    pub fn check_header_rules(&self, shell_observed_now: i64) -> Result<(), BlockValidationError> {
        check_block_header_with_params(self.candidate(), &self.rules().consensus_params())?;
        self.check_contextual_header_rules(shell_observed_now)
    }

    /// Check branch-dependent difficulty/time/version/expiry without asserting PoW.
    /// This is also the source-formula seam for independently seeded historical probes.
    /// Body owners must use actual retained header-admission clock evidence, never
    /// candidate time as a replacement for that evidence.
    pub fn check_contextual_header_rules(
        &self,
        shell_observed_now: i64,
    ) -> Result<(), BlockValidationError> {
        // Reuse next_work_required through the existing contextual validator, including
        // strictly-late min difficulty and the selected no-retarget boundary behavior.
        check_block_header_contextual(
            self.candidate(),
            &self.block_validation_context(shell_observed_now),
        )?;
        if !self.rules().accepts_block_version(self.candidate().version) {
            return Err(block_error(
                BlockValidationResult::InvalidHeader,
                "bad-version",
                Some(format!(
                    "rejected nVersion=0x{:08x} block",
                    self.candidate().version
                )),
            ));
        }
        if i64::from(self.candidate().time) > DEFAULT_SOFTWARE_EXPIRY
            && self.expiry_median_time_past() > DEFAULT_SOFTWARE_EXPIRY
        {
            return Err(block_error(
                BlockValidationResult::TimeFuture,
                "node-expired",
                Some("node software has expired".to_owned()),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CanonicalChain, HistoryHeader, HistoryOrigin, block_hash};
    use open_bitcoin_primitives::BlockHeader;

    // Synthetic links deliberately carry no PoW or admission evidence. The explicit
    // seeded constructor prevents these formula probes from claiming canonical origin.
    fn seeded(
        chain: CanonicalChain,
        height: u32,
        elapsed: u32,
        prior_bits: u32,
        candidate_bits: u32,
        delay: u32,
    ) -> CandidateHistory {
        let genesis = chain.spec().genesis_header();
        let mut rows = vec![HistoryHeader::new(chain, 0, genesis.clone())];
        for h in 1..height {
            let previous = &rows.last().expect("previous row").header;
            let time =
                genesis.time + (u64::from(elapsed) * u64::from(h) / u64::from(height - 1)) as u32;
            rows.push(HistoryHeader::new(
                chain,
                h,
                BlockHeader {
                    previous_block_hash: block_hash(previous),
                    time,
                    bits: prior_bits,
                    version: 4,
                    ..BlockHeader::default()
                },
            ));
        }
        let parent = rows.pop().expect("parent");
        let candidate = BlockHeader {
            previous_block_hash: parent.block_hash,
            time: parent.header.time + delay,
            bits: candidate_bits,
            version: 4,
            ..BlockHeader::default()
        };
        CandidateHistory::check_seeded(
            chain.spec().rules(),
            candidate,
            parent,
            rows.into_iter().rev().collect(),
            None,
        )
        .expect("complete seeded context")
    }

    #[test]
    fn phase167_header_rules_seeded_retarget_clamps_cap_and_compact_rounding() {
        for (elapsed, prior, expected) in [
            (1, 0x1c0f_fff0, 0x1c03_fffc),
            (302_400, 0x1c0f_fff0, 0x1c03_fffc),
            (1_209_600, 0x1c0f_fff0, 0x1c0f_fff0),
            (1_209_700, 0x1c0f_fff0, 0x1c10_0046),
            (4_838_400, 0x1c0f_fff0, 0x1c3f_ffc0),
            (10_000_000, 0x1c0f_fff0, 0x1c3f_ffc0),
            (4_838_400, 0x1d00_ffff, 0x1d00_ffff),
        ] {
            // Arrange
            let context = seeded(CanonicalChain::Mainnet, 2016, elapsed, prior, expected, 1);
            let wrong = seeded(
                CanonicalChain::Mainnet,
                2016,
                elapsed,
                prior,
                expected ^ 1,
                1,
            );
            // Act
            let accepted =
                context.check_contextual_header_rules(i64::from(context.candidate().time));
            let rejected = wrong.check_contextual_header_rules(i64::from(wrong.candidate().time));
            // Assert
            assert_eq!(context.origin(), HistoryOrigin::IndependentlySeeded);
            assert_eq!(accepted, Ok(()), "elapsed={elapsed}");
            assert_eq!(
                rejected.expect_err("wrong target").reject_reason,
                "bad-diffbits"
            );
        }
    }

    #[test]
    fn phase167_header_rules_seeded_testnet_minimum_is_strictly_late() {
        for (delay, expected) in [
            (1199, 0x1c0f_fff0),
            (1200, 0x1c0f_fff0),
            (1201, 0x1d00_ffff),
        ] {
            // Arrange
            let history = seeded(
                CanonicalChain::Testnet3,
                101,
                60_000,
                0x1c0f_fff0,
                expected,
                delay,
            );
            // Act
            let result = history.check_contextual_header_rules(i64::from(history.candidate().time));
            // Assert
            assert_eq!(result, Ok(()));
            assert_eq!(history.origin(), HistoryOrigin::IndependentlySeeded);
        }
    }

    #[test]
    fn phase167_header_rules_seeded_testnet_recovers_across_special_blocks() {
        // Arrange
        let original = seeded(
            CanonicalChain::Testnet3,
            101,
            60_000,
            0x1c0f_fff0,
            0x1c0f_fff0,
            1200,
        );
        let mut rows = original.branch().to_vec();
        rows.reverse();
        for index in 91..rows.len() {
            rows[index].header.bits = 0x1d00_ffff;
            rows[index].header.previous_block_hash = rows[index - 1].block_hash;
            rows[index].block_hash = block_hash(&rows[index].header);
        }
        let parent = rows.pop().expect("parent");
        let mut candidate = original.candidate().clone();
        candidate.previous_block_hash = parent.block_hash;
        let history = CandidateHistory::check_seeded(
            CanonicalChain::Testnet3.spec().rules(),
            candidate,
            parent,
            rows.into_iter().rev().collect(),
            None,
        )
        .expect("recovery links");
        // Act
        let result = history.check_contextual_header_rules(i64::from(history.candidate().time));
        // Assert
        assert_eq!(
            history
                .maybe_min_difficulty_recovery()
                .expect("recovery")
                .height,
            90
        );
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn phase167_header_rules_seeded_regtest_144_and_288_keep_boundary_target() {
        for height in [143, 144, 145, 287, 288, 289] {
            for delay in [1200, 1201] {
                // Arrange
                let boundary = height % 144 == 0;
                let expected = if delay > 1200 && !boundary {
                    0x207f_ffff
                } else {
                    0x206f_fff0
                };
                let history = seeded(
                    CanonicalChain::Regtest,
                    height,
                    90_000,
                    0x206f_fff0,
                    expected,
                    delay,
                );
                // Act
                let result =
                    history.check_contextual_header_rules(i64::from(history.candidate().time));
                // Assert
                assert_eq!(result, Ok(()), "height={height},delay={delay}");
                assert_eq!(history.origin(), HistoryOrigin::IndependentlySeeded);
            }
        }
    }

    #[test]
    fn phase167_header_rules_expiry_strict_equality_and_full_lookback() {
        let base = CanonicalChain::Regtest.spec().genesis_header().time;
        for (candidate_time, lookback_time, expired) in [
            (
                DEFAULT_SOFTWARE_EXPIRY as u32,
                DEFAULT_SOFTWARE_EXPIRY as u32 - 1,
                false,
            ),
            (
                DEFAULT_SOFTWARE_EXPIRY as u32 + 1,
                DEFAULT_SOFTWARE_EXPIRY as u32,
                false,
            ),
            (
                DEFAULT_SOFTWARE_EXPIRY as u32 + 2,
                DEFAULT_SOFTWARE_EXPIRY as u32 + 1,
                true,
            ),
        ] {
            // Arrange
            let history = seeded(
                CanonicalChain::Regtest,
                301,
                lookback_time - base,
                0x207f_ffff,
                0x207f_ffff,
                candidate_time - lookback_time,
            );
            // Place the complete expiry MTP window at the exact tested edge, while
            // retaining structurally linked but intentionally synthetic timestamps.
            let mut rows = history.branch().to_vec();
            rows.reverse();
            for index in 1..rows.len() {
                rows[index].header.time = lookback_time;
                rows[index].header.previous_block_hash = rows[index - 1].block_hash;
                rows[index].block_hash = block_hash(&rows[index].header);
            }
            let parent = rows.pop().expect("parent");
            let mut candidate = history.candidate().clone();
            candidate.previous_block_hash = parent.block_hash;
            let checked = CandidateHistory::check_seeded(
                CanonicalChain::Regtest.spec().rules(),
                candidate,
                parent,
                rows.into_iter().rev().collect(),
                None,
            )
            .expect("expiry context");
            // Act
            let result = checked.check_contextual_header_rules(i64::from(candidate_time));
            // Assert
            assert_eq!(checked.expiry_median_time_past(), i64::from(lookback_time));
            assert_eq!(
                result.as_ref().err().map(|error| error.reject_reason),
                if expired { Some("node-expired") } else { None }
            );
        }
    }
}
