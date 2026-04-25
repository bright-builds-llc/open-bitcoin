// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/fuzz/deserialize.cpp
// - packages/bitcoin-knots/src/test/fuzz/protocol.cpp
// - packages/bitcoin-knots/src/test/fuzz/primitives_transaction.cpp

use open_bitcoin_codec::{
    TransactionEncoding, encode_message_header, encode_transaction, parse_message_header,
    parse_transaction,
};
use open_bitcoin_primitives::{
    Amount, MessageCommand, MessageHeader, NetworkMagic, OutPoint, ScriptBuf, ScriptWitness,
    Transaction, TransactionInput, TransactionOutput, Txid,
};

#[derive(Debug, Clone)]
struct DeterministicGenerator {
    state: u64,
}

impl DeterministicGenerator {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u8(&mut self) -> u8 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        (self.state >> 32) as u8
    }

    fn next_u32(&mut self) -> u32 {
        let bytes = [
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
        ];
        u32::from_le_bytes(bytes)
    }

    fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    fn bytes(&mut self, max_len: usize) -> Vec<u8> {
        let len = usize::from(self.next_u8()) % (max_len + 1);
        (0..len).map(|_| self.next_u8()).collect()
    }

    fn array_32(&mut self) -> [u8; 32] {
        let mut out = [0_u8; 32];
        for byte in &mut out {
            *byte = self.next_u8();
        }
        out
    }
}

#[test]
fn generated_transactions_round_trip_through_codec() {
    // Arrange
    let mut generator = DeterministicGenerator::new(0x0b17_c01d);

    for _case in 0..64 {
        let transaction = generated_transaction(&mut generator);
        let encoding = if transaction.has_witness() {
            TransactionEncoding::WithWitness
        } else {
            TransactionEncoding::WithoutWitness
        };

        // Act
        let encoded = encode_transaction(&transaction, encoding).expect("transaction encodes");
        let decoded = parse_transaction(&encoded).expect("transaction decodes");

        // Assert
        assert_eq!(decoded, transaction);
    }
}

#[test]
fn generated_message_headers_round_trip_through_codec() {
    // Arrange
    let mut generator = DeterministicGenerator::new(0x5eed_cafe);

    for case in 0..64 {
        let command = if case % 2 == 0 { "version" } else { "ping" };
        let header = MessageHeader {
            magic: NetworkMagic::from_bytes([
                generator.next_u8(),
                generator.next_u8(),
                generator.next_u8(),
                generator.next_u8(),
            ]),
            command: MessageCommand::new(command).expect("valid command"),
            payload_size: generator.next_u32(),
            checksum: [
                generator.next_u8(),
                generator.next_u8(),
                generator.next_u8(),
                generator.next_u8(),
            ],
        };

        // Act
        let encoded = encode_message_header(&header);
        let decoded = parse_message_header(&encoded).expect("header decodes");

        // Assert
        assert_eq!(decoded, header);
    }
}

#[test]
fn generated_truncated_message_headers_never_panic() {
    // Arrange
    let mut generator = DeterministicGenerator::new(0xbaad_f00d);

    for len in 0..24 {
        let bytes = generator.bytes(len);

        // Act
        let result = parse_message_header(&bytes);

        // Assert
        assert!(result.is_err());
    }
}

fn generated_transaction(generator: &mut DeterministicGenerator) -> Transaction {
    let input_count = usize::from(generator.next_u8() % 3) + 1;
    let output_count = usize::from(generator.next_u8() % 3) + 1;
    Transaction {
        version: generator.next_i32(),
        inputs: (0..input_count)
            .map(|_| generated_input(generator))
            .collect(),
        outputs: (0..output_count)
            .map(|_| generated_output(generator))
            .collect(),
        lock_time: generator.next_u32(),
    }
}

fn generated_input(generator: &mut DeterministicGenerator) -> TransactionInput {
    let maybe_witness = if generator.next_u8().is_multiple_of(2) {
        ScriptWitness::default()
    } else {
        ScriptWitness::new(vec![generator.bytes(8), generator.bytes(8)])
    };
    TransactionInput {
        previous_output: OutPoint {
            txid: Txid::from_byte_array(generator.array_32()),
            vout: generator.next_u32(),
        },
        script_sig: ScriptBuf::from_bytes(generator.bytes(12)).expect("bounded script"),
        sequence: generator.next_u32(),
        witness: maybe_witness,
    }
}

fn generated_output(generator: &mut DeterministicGenerator) -> TransactionOutput {
    TransactionOutput {
        value: Amount::from_sats(i64::from(generator.next_u32() % 21_000_000))
            .expect("bounded amount"),
        script_pubkey: ScriptBuf::from_bytes(generator.bytes(12)).expect("bounded script"),
    }
}
