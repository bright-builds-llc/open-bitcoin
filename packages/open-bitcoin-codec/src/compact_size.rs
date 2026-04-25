// Parity breadcrumbs:
// - packages/bitcoin-knots/src/serialize.h
// - packages/bitcoin-knots/src/streams.h

use crate::error::CodecError;
use crate::primitives::{Reader, write_u16_le, write_u32_le};

pub const MAX_SIZE: u64 = 0x0200_0000;

pub fn read_compact_size(reader: &mut Reader<'_>) -> Result<u64, CodecError> {
    let value = match reader.read_u8()? {
        value @ 0..=252 => u64::from(value),
        0xfd => {
            let value = u64::from(reader.read_u16_le()?);
            if value < 253 {
                return Err(CodecError::NonCanonicalCompactSize { value });
            }
            value
        }
        0xfe => {
            let value = u64::from(reader.read_u32_le()?);
            if value <= u64::from(u16::MAX) {
                return Err(CodecError::NonCanonicalCompactSize { value });
            }
            value
        }
        0xff => {
            let value = reader.read_u64_le()?;
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

pub fn write_compact_size(out: &mut Vec<u8>, value: u64) -> Result<(), CodecError> {
    if value > MAX_SIZE {
        return Err(CodecError::CompactSizeTooLarge(value));
    }

    match value {
        0..=252 => out.push(value as u8),
        253..=0xffff => {
            out.push(0xfd);
            write_u16_le(out, value as u16);
        }
        _ => {
            out.push(0xfe);
            write_u32_le(out, value as u32);
        }
    }

    Ok(())
}

pub fn compact_size_to_usize(value: u64, _field: &'static str) -> usize {
    debug_assert!(value <= MAX_SIZE, "compact size must already be bounded");
    value as usize
}

#[cfg(test)]
mod tests {
    use crate::primitives::Reader;

    use super::{CodecError, compact_size_to_usize, read_compact_size, write_compact_size};

    #[test]
    fn compact_size_round_trips_canonical_values() {
        let cases = [0_u64, 252, 253, 65_535, 65_536, super::MAX_SIZE];

        for value in cases {
            let mut encoded = Vec::new();
            write_compact_size(&mut encoded, value).expect("compact size should encode");
            let mut reader = Reader::new(&encoded);
            assert_eq!(read_compact_size(&mut reader), Ok(value));
            assert!(reader.finish().is_ok());
        }
    }

    #[test]
    fn compact_size_rejects_non_canonical_encodings() {
        let bytes = [0xfd, 1, 0];
        let mut reader = Reader::new(&bytes);

        assert_eq!(
            read_compact_size(&mut reader),
            Err(CodecError::NonCanonicalCompactSize { value: 1 }),
        );

        let bytes = [0xfe, 1, 0, 0, 0];
        let mut reader = Reader::new(&bytes);
        assert_eq!(
            read_compact_size(&mut reader),
            Err(CodecError::NonCanonicalCompactSize { value: 1 }),
        );

        let bytes = [0xff, 1, 0, 0, 0, 0, 0, 0, 0];
        let mut reader = Reader::new(&bytes);
        assert_eq!(
            read_compact_size(&mut reader),
            Err(CodecError::NonCanonicalCompactSize { value: 1 }),
        );
    }

    #[test]
    fn compact_size_rejects_values_above_max_size() {
        let mut encoded = Vec::new();
        assert_eq!(
            write_compact_size(&mut encoded, super::MAX_SIZE + 1),
            Err(CodecError::CompactSizeTooLarge(super::MAX_SIZE + 1)),
        );

        let bytes = [0xff, 0, 0, 0, 0, 1, 0, 0, 0];
        let mut reader = Reader::new(&bytes);
        assert_eq!(
            read_compact_size(&mut reader),
            Err(CodecError::CompactSizeTooLarge(4_294_967_296)),
        );
    }

    #[test]
    fn compact_size_to_usize_keeps_small_values() {
        assert_eq!(compact_size_to_usize(7, "test"), 7);
        assert_eq!(
            compact_size_to_usize(super::MAX_SIZE, "test"),
            super::MAX_SIZE as usize
        );
    }
}
