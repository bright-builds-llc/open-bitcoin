// Parity breadcrumbs:
// - packages/bitcoin-knots/src/serialize.h
// - packages/bitcoin-knots/src/streams.h

use crate::error::CodecError;

#[derive(Debug, Clone)]
pub struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }

    pub fn finish(self) -> Result<(), CodecError> {
        if self.remaining() == 0 {
            return Ok(());
        }

        Err(CodecError::TrailingData {
            remaining: self.remaining(),
        })
    }

    pub fn read_u8(&mut self) -> Result<u8, CodecError> {
        Ok(self.read_array::<1>()?[0])
    }

    pub fn read_u16_le(&mut self) -> Result<u16, CodecError> {
        Ok(u16::from_le_bytes(self.read_array()?))
    }

    pub fn read_u16_be(&mut self) -> Result<u16, CodecError> {
        Ok(u16::from_be_bytes(self.read_array()?))
    }

    pub fn read_u32_le(&mut self) -> Result<u32, CodecError> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    pub fn read_u64_le(&mut self) -> Result<u64, CodecError> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    pub fn read_i32_le(&mut self) -> Result<i32, CodecError> {
        Ok(i32::from_le_bytes(self.read_array()?))
    }

    pub fn read_i64_le(&mut self) -> Result<i64, CodecError> {
        Ok(i64::from_le_bytes(self.read_array()?))
    }

    pub fn read_array<const N: usize>(&mut self) -> Result<[u8; N], CodecError> {
        let slice = self.take(N)?;
        let mut array = [0_u8; N];
        array.copy_from_slice(slice);
        Ok(array)
    }

    pub fn read_vec(&mut self, len: usize) -> Result<Vec<u8>, CodecError> {
        Ok(self.take(len)?.to_vec())
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], CodecError> {
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
}

pub fn write_u16_be(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_be_bytes());
}

pub fn write_u16_le(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn write_u32_le(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn write_u64_le(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn write_i32_le(out: &mut Vec<u8>, value: i32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn write_i64_le(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::{
        Reader, write_i32_le, write_i64_le, write_u16_be, write_u16_le, write_u32_le, write_u64_le,
    };

    #[test]
    fn reader_reads_all_supported_integer_widths() {
        let bytes = [
            0xaa, 0x34, 0x12, 0xbe, 0xef, 0x78, 0x56, 0x34, 0x12, 0x08, 0x07, 0x06, 0x05, 0x04,
            0x03, 0x02, 0x01, 0xfe, 0xff, 0xff, 0xff, 0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc,
            0xfe,
        ];
        let mut reader = Reader::new(&bytes);

        assert_eq!(reader.read_u8(), Ok(0xaa));
        assert_eq!(reader.read_u16_le(), Ok(0x1234));
        assert_eq!(reader.read_u16_be(), Ok(0xbeef));
        assert_eq!(reader.read_u32_le(), Ok(0x1234_5678));
        assert_eq!(reader.read_u64_le(), Ok(0x0102_0304_0506_0708));
        assert_eq!(reader.read_i32_le(), Ok(-2));
        assert_eq!(reader.read_i64_le(), Ok(-81_985_529_216_486_896));
        assert!(reader.finish().is_ok());
    }

    #[test]
    fn reader_and_writers_round_trip_bytes() {
        let mut bytes = Vec::new();
        write_u16_be(&mut bytes, 0xabcd);
        write_u16_le(&mut bytes, 0x1234);
        write_u32_le(&mut bytes, 0x89ab_cdef);
        write_u64_le(&mut bytes, 0x0102_0304_0506_0708);
        write_i32_le(&mut bytes, -4);
        write_i64_le(&mut bytes, -5);

        let mut reader = Reader::new(&bytes);
        assert_eq!(reader.read_u16_be(), Ok(0xabcd));
        assert_eq!(reader.read_u16_le(), Ok(0x1234));
        assert_eq!(reader.read_u32_le(), Ok(0x89ab_cdef));
        assert_eq!(reader.read_u64_le(), Ok(0x0102_0304_0506_0708));
        assert_eq!(reader.read_i32_le(), Ok(-4));
        assert_eq!(reader.read_i64_le(), Ok(-5));
        assert!(reader.finish().is_ok());
    }

    #[test]
    fn reader_reports_unexpected_eof_and_trailing_data() {
        let mut reader = Reader::new(&[0x01]);
        let error = reader
            .read_array::<2>()
            .expect_err("two-byte read should fail with one byte");
        assert_eq!(
            error.to_string(),
            "unexpected EOF: needed 2 bytes, remaining 1",
        );

        let reader = Reader::new(&[0x01]);
        assert_eq!(
            reader
                .finish()
                .expect_err("remaining byte must be reported")
                .to_string(),
            "trailing data: 1 bytes",
        );
    }
}
