use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxValidationResult {
    Consensus,
    InputsNotStandard,
    NotStandard,
    MissingInputs,
    PrematureSpend,
    WitnessMutated,
    WitnessStripped,
    Conflict,
    MempoolPolicy,
    NoMempool,
    Reconsiderable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockValidationResult {
    Consensus,
    CachedInvalid,
    InvalidHeader,
    Mutated,
    MissingPrev,
    InvalidPrev,
    TimeFuture,
    Checkpoint,
    HeaderLowWork,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError<R> {
    pub result: R,
    pub reject_reason: &'static str,
    pub debug_message: Option<String>,
}

impl<R> ValidationError<R> {
    pub fn new(result: R, reject_reason: &'static str, debug_message: Option<String>) -> Self {
        Self {
            result,
            reject_reason,
            debug_message,
        }
    }
}

impl<R: fmt::Debug> fmt::Display for ValidationError<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(debug_message) = &self.debug_message {
            write!(f, "{} ({debug_message})", self.reject_reason)
        } else {
            write!(f, "{}", self.reject_reason)
        }
    }
}

impl<R: fmt::Debug> std::error::Error for ValidationError<R> {}

pub type TxValidationError = ValidationError<TxValidationResult>;
pub type BlockValidationError = ValidationError<BlockValidationResult>;

pub fn tx_error(
    result: TxValidationResult,
    reject_reason: &'static str,
    debug_message: impl Into<Option<String>>,
) -> TxValidationError {
    ValidationError::new(result, reject_reason, debug_message.into())
}

pub fn block_error(
    result: BlockValidationResult,
    reject_reason: &'static str,
    debug_message: impl Into<Option<String>>,
) -> BlockValidationError {
    ValidationError::new(result, reject_reason, debug_message.into())
}

#[cfg(test)]
mod tests {
    use super::{BlockValidationResult, ValidationError};

    #[test]
    fn display_includes_debug_message_when_present() {
        let error = ValidationError::new(
            BlockValidationResult::Consensus,
            "bad-block",
            Some("details".to_string()),
        );

        assert_eq!(error.to_string(), "bad-block (details)");
    }

    #[test]
    fn display_omits_debug_message_when_absent() {
        let error = ValidationError::new(BlockValidationResult::Consensus, "bad-block", None);

        assert_eq!(error.to_string(), "bad-block");
    }
}
