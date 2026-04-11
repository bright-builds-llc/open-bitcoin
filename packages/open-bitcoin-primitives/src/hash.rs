use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashLengthError {
    pub expected: usize,
    pub actual: usize,
}

impl fmt::Display for HashLengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid hash length: expected {}, got {}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for HashLengthError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Hash32([u8; 32]);

impl Hash32 {
    pub const LEN: usize = 32;

    pub const fn from_byte_array(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, HashLengthError> {
        let Ok(array) = <[u8; 32]>::try_from(bytes) else {
            return Err(HashLengthError {
                expected: Self::LEN,
                actual: bytes.len(),
            });
        };

        Ok(Self(array))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub const fn to_byte_array(self) -> [u8; 32] {
        self.0
    }
}

macro_rules! define_hash_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name(Hash32);

        impl $name {
            pub const fn from_byte_array(bytes: [u8; 32]) -> Self {
                Self(Hash32::from_byte_array(bytes))
            }

            pub fn from_slice(bytes: &[u8]) -> Result<Self, HashLengthError> {
                Ok(Self(Hash32::from_slice(bytes)?))
            }

            pub fn as_bytes(&self) -> &[u8; 32] {
                self.0.as_bytes()
            }

            pub const fn to_byte_array(self) -> [u8; 32] {
                self.0.to_byte_array()
            }
        }

        impl From<Hash32> for $name {
            fn from(value: Hash32) -> Self {
                Self(value)
            }
        }

        impl From<$name> for Hash32 {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

define_hash_wrapper!(Txid);
define_hash_wrapper!(Wtxid);
define_hash_wrapper!(BlockHash);
define_hash_wrapper!(MerkleRoot);

#[cfg(test)]
mod tests {
    use super::{Hash32, HashLengthError, Txid};

    #[test]
    fn hash32_from_slice_requires_exact_length() {
        let valid = [7_u8; 32];
        assert_eq!(Hash32::from_slice(&valid), Ok(Hash32::from_byte_array(valid)));
        assert_eq!(
            Hash32::from_slice(&valid[..31]),
            Err(HashLengthError {
                expected: 32,
                actual: 31,
            }),
        );
    }

    #[test]
    fn wrappers_preserve_underlying_bytes() {
        let bytes = [9_u8; 32];
        let txid = Txid::from_byte_array(bytes);

        assert_eq!(txid.as_bytes(), &bytes);
        assert_eq!(txid.to_byte_array(), bytes);
    }
}
