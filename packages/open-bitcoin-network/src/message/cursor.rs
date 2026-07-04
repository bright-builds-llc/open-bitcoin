// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netbase.h

use open_bitcoin_codec::{CodecError, MAX_SIZE};

#[derive(Debug, Clone)]
pub(super) struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub(super) fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }

    pub(super) fn finish(self) -> Result<(), CodecError> {
        if self.remaining() == 0 {
            return Ok(());
        }

        Err(CodecError::TrailingData {
            remaining: self.remaining(),
        })
    }

    pub(super) fn read_u8(&mut self) -> Result<u8, CodecError> {
        Ok(self.read_array::<1>()?[0])
    }

    pub(super) fn read_u64_le(&mut self) -> Result<u64, CodecError> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    pub(super) fn read_u32_le(&mut self) -> Result<u32, CodecError> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    pub(super) fn read_i32_le(&mut self) -> Result<i32, CodecError> {
        Ok(i32::from_le_bytes(self.read_array()?))
    }

    pub(super) fn read_i64_le(&mut self) -> Result<i64, CodecError> {
        Ok(i64::from_le_bytes(self.read_array()?))
    }

    pub(super) fn read_array<const N: usize>(&mut self) -> Result<[u8; N], CodecError> {
        let slice = self.read_slice(N)?;
        let mut array = [0_u8; N];
        array.copy_from_slice(slice);
        Ok(array)
    }

    pub(super) fn read_slice(&mut self, len: usize) -> Result<&'a [u8], CodecError> {
        let remaining = self.remaining();
        if remaining < len {
            return Err(CodecError::UnexpectedEof {
                needed: len,
                remaining,
            });
        }

        let start = self.offset;
        self.offset += len;
        Ok(&self.bytes[start..self.offset])
    }

    pub(super) fn read_compact_size(&mut self) -> Result<u64, CodecError> {
        let first = self.read_u8()?;
        let value = match first {
            value @ 0..=252 => u64::from(value),
            0xfd => {
                let value = u64::from(u16::from_le_bytes(self.read_array()?));
                if value < 253 {
                    return Err(CodecError::NonCanonicalCompactSize { value });
                }
                value
            }
            0xfe => {
                let value = u64::from(u32::from_le_bytes(self.read_array()?));
                if value <= u64::from(u16::MAX) {
                    return Err(CodecError::NonCanonicalCompactSize { value });
                }
                value
            }
            0xff => {
                let value = u64::from_le_bytes(self.read_array()?);
                if value <= u64::from(u32::MAX) {
                    return Err(CodecError::NonCanonicalCompactSize { value });
                }
                value
            }
        };

        if value > MAX_SIZE {
            return Err(CodecError::CompactSizeTooLarge(value));
        }
        Ok(value)
    }
}

pub(super) fn compact_size_to_usize(value: u64, field: &'static str) -> usize {
    debug_assert!(
        value <= usize::MAX as u64,
        "{field} does not fit into usize"
    );
    value as usize
}
