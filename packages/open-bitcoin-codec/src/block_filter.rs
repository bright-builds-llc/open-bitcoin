// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/util/golombrice.h
// - packages/bitcoin-knots/src/streams.h
// - packages/bitcoin-knots/src/serialize.h

//! Bounded, hash-free encoding of BASIC/type 0 mapped values.

use core::fmt;

use crate::{CodecError, MAX_SIZE, write_compact_size};

pub mod validation;

/// BASIC Golomb-Rice remainder width.
pub const BASIC_FILTER_P: u32 = 19;
/// BASIC range multiplier for each distinct raw script.
pub const BASIC_FILTER_M: u64 = 784_931;

/// A malformed or resource-exceeding BASIC mapped-value input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicFilterEncodingError {
    CountTooLarge(u64),
    Unsorted,
    OutOfRange { value: u64, range: u64 },
    OutputTooLarge,
    AllocationFailed,
    Codec(CodecError),
}

impl fmt::Display for BasicFilterEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountTooLarge(count) => {
                write!(f, "BASIC element count exceeds MAX_SIZE: {count}")
            }
            Self::Unsorted => f.write_str("BASIC mapped values must be sorted"),
            Self::OutOfRange { value, range } => {
                write!(f, "BASIC mapped value {value} is outside range {range}")
            }
            Self::OutputTooLarge => f.write_str("BASIC encoded output exceeds MAX_SIZE"),
            Self::AllocationFailed => f.write_str("BASIC output allocation failed"),
            Self::Codec(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for BasicFilterEncodingError {}

impl From<CodecError> for BasicFilterEncodingError {
    fn from(error: CodecError) -> Self {
        Self::Codec(error)
    }
}

/// Return N*M for BASIC, refusing counts above the codec's block-bound MAX_SIZE.
pub fn basic_filter_range(count: u64) -> Result<u64, BasicFilterEncodingError> {
    if count > MAX_SIZE {
        return Err(BasicFilterEncodingError::CountTooLarge(count));
    }
    count
        .checked_mul(BASIC_FILTER_M)
        .ok_or(BasicFilterEncodingError::CountTooLarge(count))
}

/// Encode sorted mapped values, preserving duplicates as zero deltas.
///
/// Both N and total encoded bytes are capped at MAX_SIZE (32 MiB). Values must
/// be below N*M. These checks bound total unary work before allocation/writing.
pub fn encode_basic_filter_values(values: &[u64]) -> Result<Vec<u8>, BasicFilterEncodingError> {
    let count = values.len() as u64;
    let range = basic_filter_range(count)?;
    let mut last = 0;
    let mut bit_count = 0;
    for &value in values {
        if value >= range {
            return Err(BasicFilterEncodingError::OutOfRange { value, range });
        }
        if value < last {
            return Err(BasicFilterEncodingError::Unsorted);
        }
        bit_count += ((value - last) >> BASIC_FILTER_P) + 1 + u64::from(BASIC_FILTER_P);
        last = value;
    }
    let output_size = checked_output_size(count, bit_count)?;
    let mut encoded = Vec::new();
    reserve_output(&mut encoded, output_size)?;
    write_compact_size(&mut encoded, count)?;
    let prefix_size = encoded.len();
    encoded.resize(output_size, 0);
    let mut bit_position = prefix_size * 8;
    last = 0;
    for &value in values {
        let delta = value - last;
        for _ in 0..(delta >> BASIC_FILTER_P) {
            write_bit(&mut encoded, &mut bit_position, true);
        }
        write_bit(&mut encoded, &mut bit_position, false);
        for shift in (0..BASIC_FILTER_P).rev() {
            write_bit(&mut encoded, &mut bit_position, (delta >> shift) & 1 != 0);
        }
        last = value;
    }
    Ok(encoded)
}

fn checked_output_size(count: u64, bits: u64) -> Result<usize, BasicFilterEncodingError> {
    let prefix = if count < 253 {
        1
    } else if count <= 65_535 {
        3
    } else {
        5
    };
    let bytes = bits
        .checked_add(7)
        .and_then(|rounded| (rounded / 8).checked_add(prefix))
        .filter(|size| *size <= MAX_SIZE)
        .ok_or(BasicFilterEncodingError::OutputTooLarge)?;
    Ok(bytes as usize)
}

fn reserve_output(output: &mut Vec<u8>, size: usize) -> Result<(), BasicFilterEncodingError> {
    output
        .try_reserve_exact(size)
        .map_err(|_| BasicFilterEncodingError::AllocationFailed)
}

fn write_bit(output: &mut [u8], position: &mut usize, bit: bool) {
    if bit {
        output[*position / 8] |= 1 << (7 - *position % 8);
    }
    *position += 1;
}

#[cfg(test)]
mod tests;
