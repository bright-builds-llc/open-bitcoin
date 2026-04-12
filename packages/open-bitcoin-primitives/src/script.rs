use core::fmt;

/// Maximum number of bytes pushable to the stack.
pub const MAX_SCRIPT_ELEMENT_SIZE: usize = 520;

/// Maximum number of non-push operations per script.
pub const MAX_OPS_PER_SCRIPT: usize = 201;

/// Maximum number of public keys per multisig.
pub const MAX_PUBKEYS_PER_MULTISIG: usize = 20;

/// Maximum script length in bytes.
pub const MAX_SCRIPT_SIZE: usize = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptError {
    TooLarge(usize),
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge(size) => write!(f, "script too large: {size}"),
        }
    }
}

impl std::error::Error for ScriptError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ScriptBuf(Vec<u8>);

impl ScriptBuf {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, ScriptError> {
        if bytes.len() > MAX_SCRIPT_SIZE {
            return Err(ScriptError::TooLarge(bytes.len()));
        }

        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScriptWitness {
    stack: Vec<Vec<u8>>,
}

impl ScriptWitness {
    pub fn new(stack: Vec<Vec<u8>>) -> Self {
        Self { stack }
    }

    pub fn stack(&self) -> &[Vec<u8>] {
        &self.stack
    }

    pub fn into_stack(self) -> Vec<Vec<u8>> {
        self.stack
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_SCRIPT_SIZE, ScriptBuf, ScriptError, ScriptWitness};

    #[test]
    fn script_buf_rejects_oversized_scripts() {
        let oversized = vec![0_u8; MAX_SCRIPT_SIZE + 1];

        assert_eq!(
            ScriptBuf::from_bytes(oversized),
            Err(ScriptError::TooLarge(MAX_SCRIPT_SIZE + 1)),
        );
    }

    #[test]
    fn script_witness_empty_detects_empty_stack_only() {
        let witness = ScriptWitness::default();

        assert!(witness.is_empty());
        assert!(!ScriptWitness::new(vec![vec![]]).is_empty());
    }

    #[test]
    fn script_buf_round_trips_bytes_and_empty_state() {
        let script = ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script");

        assert_eq!(script.as_bytes(), &[0x51, 0xac]);
        assert!(!script.is_empty());
        assert_eq!(script.into_bytes(), vec![0x51, 0xac]);
    }

    #[test]
    fn script_witness_exposes_stack_contents() {
        let witness = ScriptWitness::new(vec![vec![0x01], vec![0x02, 0x03]]);

        assert_eq!(witness.stack(), &[vec![0x01], vec![0x02, 0x03]]);
        assert_eq!(witness.into_stack(), vec![vec![0x01], vec![0x02, 0x03]]);
    }

    #[test]
    fn script_error_display_is_descriptive() {
        assert_eq!(
            ScriptError::TooLarge(MAX_SCRIPT_SIZE + 1).to_string(),
            format!("script too large: {}", MAX_SCRIPT_SIZE + 1),
        );
    }
}
