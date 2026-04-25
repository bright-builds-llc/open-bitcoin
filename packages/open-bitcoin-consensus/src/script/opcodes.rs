// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

pub(super) const OP_PUSHDATA1: u8 = 0x4c;
pub(super) const OP_PUSHDATA2: u8 = 0x4d;
pub(super) const OP_PUSHDATA4: u8 = 0x4e;
pub(super) const OP_1NEGATE: u8 = 0x4f;
pub(super) const OP_RESERVED: u8 = 0x50;
pub(super) const OP_1: u8 = 0x51;
pub(super) const OP_16: u8 = 0x60;
pub(super) const OP_NOP: u8 = 0x61;
pub(super) const OP_VER: u8 = 0x62;
pub(super) const OP_IF: u8 = 0x63;
pub(super) const OP_NOTIF: u8 = 0x64;
pub(super) const OP_ELSE: u8 = 0x67;
pub(super) const OP_ENDIF: u8 = 0x68;
pub(super) const OP_VERIFY: u8 = 0x69;
pub(super) const OP_RETURN: u8 = 0x6a;
pub(super) const OP_DROP: u8 = 0x75;
pub(super) const OP_DUP: u8 = 0x76;
pub(super) const OP_OVER: u8 = 0x78;
pub(super) const OP_SWAP: u8 = 0x7c;
pub(super) const OP_SIZE: u8 = 0x82;
pub(super) const OP_EQUAL: u8 = 0x87;
pub(super) const OP_EQUALVERIFY: u8 = 0x88;
pub(super) const OP_1ADD: u8 = 0x8b;
pub(super) const OP_1SUB: u8 = 0x8c;
pub(super) const OP_NEGATE: u8 = 0x8f;
pub(super) const OP_NOT: u8 = 0x91;
pub(super) const OP_0NOTEQUAL: u8 = 0x92;
pub(super) const OP_ADD: u8 = 0x93;
pub(super) const OP_SUB: u8 = 0x94;
pub(super) const OP_BOOLAND: u8 = 0x9a;
pub(super) const OP_BOOLOR: u8 = 0x9b;
pub(super) const OP_NUMEQUAL: u8 = 0x9c;
pub(super) const OP_NUMEQUALVERIFY: u8 = 0x9d;
pub(super) const OP_NUMNOTEQUAL: u8 = 0x9e;
pub(super) const OP_LESSTHAN: u8 = 0x9f;
pub(super) const OP_GREATERTHAN: u8 = 0xa0;
pub(super) const OP_MIN: u8 = 0xa3;
pub(super) const OP_MAX: u8 = 0xa4;
pub(super) const OP_WITHIN: u8 = 0xa5;
pub(super) const OP_RIPEMD160: u8 = 0xa6;
pub(super) const OP_SHA256: u8 = 0xa8;
pub(super) const OP_HASH160: u8 = 0xa9;
pub(super) const OP_HASH256: u8 = 0xaa;
pub(super) const OP_CODESEPARATOR: u8 = 0xab;
pub(super) const OP_CHECKSIG: u8 = 0xac;
pub(super) const OP_CHECKSIGVERIFY: u8 = 0xad;
pub(super) const OP_CHECKMULTISIG: u8 = 0xae;
pub(super) const OP_CHECKMULTISIGVERIFY: u8 = 0xaf;
pub(super) const OP_CHECKSIGADD: u8 = 0xba;
pub(super) const MAX_PUBKEYS_PER_MULTISIG: usize = 20;

pub(super) fn decode_small_int_opcode(opcode: u8) -> Option<usize> {
    match opcode {
        0x51..=0x60 => Some(usize::from(opcode - 0x50)),
        _ => None,
    }
}

pub(super) fn is_disabled_opcode(opcode: u8) -> bool {
    matches!(
        opcode,
        0x7e | 0x7f
            | 0x80
            | 0x81
            | 0x83
            | 0x84
            | 0x85
            | 0x86
            | 0x8d
            | 0x8e
            | 0x95
            | 0x96
            | 0x97
            | 0x98
            | 0x99
    )
}

pub(super) fn is_op_success(opcode: u8) -> bool {
    opcode == OP_RESERVED
        || opcode == OP_VER
        || (0x7e..=0x81).contains(&opcode)
        || (0x83..=0x86).contains(&opcode)
        || (0x89..=0x8a).contains(&opcode)
        || (0x8d..=0x8e).contains(&opcode)
        || (0x95..=0x99).contains(&opcode)
        || (0xbb..=0xfe).contains(&opcode)
}
