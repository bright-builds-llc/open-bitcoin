use core::fmt;

/// The amount of satoshis in one BTC.
pub const COIN: i64 = 100_000_000;

/// Maximum valid amount in satoshis.
pub const MAX_MONEY: i64 = 21_000_000 * COIN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmountError {
    OutOfRange(i64),
}

impl fmt::Display for AmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange(value) => write!(f, "amount out of range: {value}"),
        }
    }
}

impl std::error::Error for AmountError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount(i64);

impl Amount {
    pub const ZERO: Self = Self(0);

    pub fn from_sats(value: i64) -> Result<Self, AmountError> {
        if !(0..=MAX_MONEY).contains(&value) {
            return Err(AmountError::OutOfRange(value));
        }

        Ok(Self(value))
    }

    pub const fn to_sats(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Amount, AmountError, MAX_MONEY};

    #[test]
    fn from_sats_accepts_zero_and_max_money() {
        assert_eq!(Amount::from_sats(0), Ok(Amount::ZERO));
        assert_eq!(
            Amount::from_sats(MAX_MONEY),
            Ok(Amount::from_sats(MAX_MONEY).expect("MAX_MONEY should be valid")),
        );
    }

    #[test]
    fn from_sats_rejects_negative_and_overflow_values() {
        assert_eq!(Amount::from_sats(-1), Err(AmountError::OutOfRange(-1)));
        assert_eq!(
            Amount::from_sats(MAX_MONEY + 1),
            Err(AmountError::OutOfRange(MAX_MONEY + 1)),
        );
    }
}
