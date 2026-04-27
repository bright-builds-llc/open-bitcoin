// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use open_bitcoin_node::core::primitives::ScriptBuf;
use open_bitcoin_node::core::wallet::AddressNetwork;

use crate::error::RpcFailure;

const BASE58_ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BECH32_ALPHABET: &[u8; 32] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

pub(super) fn script_pubkey_from_address(
    network: AddressNetwork,
    address: &str,
) -> Result<ScriptBuf, RpcFailure> {
    if address.starts_with(network.hrp()) {
        return decode_segwit_script(network, address);
    }

    decode_base58_script(network, address)
}

fn decode_base58_script(network: AddressNetwork, address: &str) -> Result<ScriptBuf, RpcFailure> {
    let decoded = base58_decode(address)?;
    if decoded.len() < 5 {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    let payload_len = decoded.len().saturating_sub(4);
    let (payload, checksum) = decoded.split_at(payload_len);
    let expected_checksum = open_bitcoin_node::core::consensus::crypto::double_sha256(payload);
    if checksum != &expected_checksum[..4] {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }
    let Some((prefix, body)) = payload.split_first() else {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    };
    match (*prefix, body.len()) {
        (prefix, 20) if prefix == network.p2pkh_prefix() => {
            let mut script = vec![0x76, 0xa9, 0x14];
            script.extend_from_slice(body);
            script.extend_from_slice(&[0x88, 0xac]);
            ScriptBuf::from_bytes(script)
                .map_err(|error| RpcFailure::invalid_params(error.to_string()))
        }
        (prefix, 20) if prefix == network.p2sh_prefix() => {
            let mut script = vec![0xa9, 0x14];
            script.extend_from_slice(body);
            script.push(0x87);
            ScriptBuf::from_bytes(script)
                .map_err(|error| RpcFailure::invalid_params(error.to_string()))
        }
        _ => Err(RpcFailure::invalid_params("invalid destination address")),
    }
}

fn decode_segwit_script(network: AddressNetwork, address: &str) -> Result<ScriptBuf, RpcFailure> {
    let (hrp, data, bech32m) = bech32_decode(address)?;
    if hrp != network.hrp() || data.is_empty() {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }
    let version = data[0];
    let program = convert_bits(&data[1..], 5, 8, false)?;
    if version == 0 && bech32m {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }
    if version != 0 && !bech32m {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    let opcode = if version == 0 { 0x00 } else { 0x50 + version };
    let mut script = vec![opcode, program.len() as u8];
    script.extend_from_slice(&program);
    ScriptBuf::from_bytes(script).map_err(|error| RpcFailure::invalid_params(error.to_string()))
}

fn base58_decode(input: &str) -> Result<Vec<u8>, RpcFailure> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    let mut bytes = vec![0_u8];
    for ch in trimmed.bytes() {
        let Some(mut carry) = BASE58_ALPHABET
            .iter()
            .position(|candidate| *candidate == ch)
        else {
            return Err(RpcFailure::invalid_params("invalid destination address"));
        };
        for byte in bytes.iter_mut().rev() {
            let value = usize::from(*byte) * 58 + carry;
            *byte = (value & 0xff) as u8;
            carry = value >> 8;
        }
        while carry > 0 {
            bytes.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
        }
    }

    let leading_zeros = trimmed.bytes().take_while(|byte| *byte == b'1').count();
    let mut decoded = vec![0_u8; leading_zeros];
    decoded.extend(bytes.into_iter().skip_while(|byte| *byte == 0));
    Ok(decoded)
}

fn bech32_decode(input: &str) -> Result<(String, Vec<u8>, bool), RpcFailure> {
    let trimmed = input.trim();
    let Some(separator) = trimmed.rfind('1') else {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    };
    let hrp = trimmed[..separator].to_ascii_lowercase();
    let payload = &trimmed[separator + 1..];
    if hrp.is_empty() || payload.len() < 6 {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    let values = payload
        .bytes()
        .map(|byte| {
            BECH32_ALPHABET
                .iter()
                .position(|candidate| *candidate == byte.to_ascii_lowercase())
                .map(|index| index as u8)
                .ok_or_else(|| RpcFailure::invalid_params("invalid destination address"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let polymod = bech32_polymod(&[expand_hrp(&hrp), values.clone()].concat());
    let bech32m = match polymod {
        1 => false,
        0x2bc8_30a3 => true,
        _ => return Err(RpcFailure::invalid_params("invalid destination address")),
    };
    Ok((hrp, values[..values.len() - 6].to_vec(), bech32m))
}

fn expand_hrp(hrp: &str) -> Vec<u8> {
    let mut expanded = Vec::with_capacity(hrp.len() * 2 + 1);
    expanded.extend(hrp.bytes().map(|byte| byte >> 5));
    expanded.push(0);
    expanded.extend(hrp.bytes().map(|byte| byte & 0x1f));
    expanded
}

fn bech32_polymod(values: &[u8]) -> u32 {
    let mut checksum = 1_u32;
    for value in values {
        let top = checksum >> 25;
        checksum = ((checksum & 0x01ff_ffff) << 5) ^ u32::from(*value);
        for (index, generator) in [
            0x3b6a_57b2_u32,
            0x2650_8e6d,
            0x1ea1_19fa,
            0x3d42_33dd,
            0x2a14_62b3,
        ]
        .iter()
        .enumerate()
        {
            if ((top >> index) & 1) == 1 {
                checksum ^= generator;
            }
        }
    }
    checksum
}

fn convert_bits(data: &[u8], from: u32, to: u32, pad: bool) -> Result<Vec<u8>, RpcFailure> {
    let max_value = (1_u32 << to) - 1;
    let max_accumulator = (1_u32 << (from + to - 1)) - 1;
    let mut accumulator = 0_u32;
    let mut bits = 0_u32;
    let mut output = Vec::new();

    for value in data {
        if (u32::from(*value) >> from) != 0 {
            return Err(RpcFailure::invalid_params("invalid destination address"));
        }
        accumulator = ((accumulator << from) | u32::from(*value)) & max_accumulator;
        bits += from;
        while bits >= to {
            bits -= to;
            output.push(((accumulator >> bits) & max_value) as u8);
        }
    }

    if pad {
        if bits > 0 {
            output.push(((accumulator << (to - bits)) & max_value) as u8);
        }
    } else if bits >= from || ((accumulator << (to - bits)) & max_value) != 0 {
        return Err(RpcFailure::invalid_params("invalid destination address"));
    }

    Ok(output)
}

pub(super) fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

pub(super) fn decode_hex(text: &str) -> Result<Vec<u8>, &'static str> {
    let trimmed = text.trim();
    if !trimmed.len().is_multiple_of(2) {
        return Err("hex strings must have even length");
    }

    let mut bytes = Vec::with_capacity(trimmed.len() / 2);
    for pair in trimmed.as_bytes().chunks_exact(2) {
        let high = decode_nibble(pair[0])?;
        let low = decode_nibble(pair[1])?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn decode_nibble(byte: u8) -> Result<u8, &'static str> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err("hex strings may only contain ASCII hex digits"),
    }
}
