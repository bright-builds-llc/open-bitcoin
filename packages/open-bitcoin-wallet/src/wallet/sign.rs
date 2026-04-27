// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/src/wallet/coinselection.cpp
// - packages/bitcoin-knots/src/wallet/transaction.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py
// - packages/bitcoin-knots/test/functional/feature_segwit.py

use secp256k1::{Message, Secp256k1};

use open_bitcoin_consensus::{
    ScriptExecutionData, ScriptVerifyFlags, SigHashType, SigVersion, legacy_sighash,
    segwit_v0_sighash, taproot_sighash,
};
use open_bitcoin_primitives::{ScriptBuf, ScriptWitness, Transaction};

use super::{BuildRequest, BuiltTransaction, Wallet};
use crate::WalletError;
use crate::address::{
    PrivateKey, public_key_bytes, push_data, taproot_output_key_from_private_key,
};
use crate::descriptor::SingleKeyDescriptor;

pub(super) fn sign_transaction(
    wallet: &Wallet,
    built: &BuiltTransaction,
) -> Result<Transaction, WalletError> {
    let mut transaction = built.transaction.clone();
    let input_contexts = wallet.input_contexts_for(built)?;
    let validation_context = wallet.validation_context(&input_contexts);
    let precomputed = validation_context.precompute(&transaction)?;

    for (input_index, utxo) in built.selected_inputs.iter().enumerate() {
        let descriptor = wallet
            .descriptor(utxo.descriptor_id)
            .ok_or(WalletError::UnknownDescriptor(utxo.descriptor_id))?;
        let descriptor_index = descriptor
            .descriptor
            .matching_index(&utxo.output.script_pubkey)?
            .unwrap_or(0);
        match &descriptor.descriptor {
            SingleKeyDescriptor::Pkh(key) => {
                let private_key = descriptor
                    .descriptor
                    .private_key_at(descriptor_index)?
                    .ok_or_else(|| {
                        WalletError::MissingSigningKey(descriptor.original_text.clone())
                    })?;
                let script_code = descriptor.descriptor.script_pubkey_at(descriptor_index)?;
                let sighash =
                    legacy_sighash(&script_code, &transaction, input_index, SigHashType::ALL);
                let signature = sign_ecdsa_low_s(&private_key, &sighash.to_byte_array())?;
                let public_key = key.public_key_at(descriptor_index)?;
                let public_key_bytes = public_key_bytes(&public_key, key.is_compressed());
                let script_sig = push_script(&[signature.as_slice(), public_key_bytes.as_slice()])?;
                transaction.inputs[input_index].script_sig = script_sig;
            }
            SingleKeyDescriptor::ShWpkh(key) | SingleKeyDescriptor::Wpkh(key) => {
                let private_key = descriptor
                    .descriptor
                    .private_key_at(descriptor_index)?
                    .ok_or_else(|| {
                        WalletError::MissingSigningKey(descriptor.original_text.clone())
                    })?;
                let public_key = key.public_key_at(descriptor_index)?;
                let script_code = p2pkh_script(&public_key)?;
                let input_context = input_contexts[input_index].clone();
                let sighash = segwit_v0_sighash(
                    &script_code,
                    &transaction,
                    input_index,
                    &input_context,
                    SigHashType::ALL,
                    &precomputed,
                );
                let signature = sign_ecdsa_low_s(&private_key, &sighash.to_byte_array())?;
                let public_key_bytes = public_key_bytes(&public_key, key.is_compressed());
                if let Some(redeem_script) =
                    descriptor.descriptor.redeem_script_at(descriptor_index)?
                {
                    transaction.inputs[input_index].script_sig =
                        push_script(&[redeem_script.as_bytes()])?;
                }
                transaction.inputs[input_index].witness =
                    ScriptWitness::new(vec![signature, public_key_bytes]);
            }
            SingleKeyDescriptor::Tr(_key) => {
                let private_key = descriptor
                    .descriptor
                    .private_key_at(descriptor_index)?
                    .ok_or_else(|| {
                        WalletError::MissingSigningKey(descriptor.original_text.clone())
                    })?;
                let secp = Secp256k1::new();
                let (keypair, _output_key) = taproot_output_key_from_private_key(&private_key)?;
                let sighash = taproot_sighash(
                    &ScriptExecutionData::default(),
                    &transaction,
                    input_index,
                    SigHashType::DEFAULT,
                    SigVersion::Taproot,
                    &validation_context,
                )
                .ok_or_else(taproot_sighash_unavailable_error)?;
                let message = Message::from_digest(sighash.to_byte_array());
                let signature = secp.sign_schnorr_no_aux_rand(message.as_ref(), &keypair);
                transaction.inputs[input_index].witness =
                    ScriptWitness::new(vec![signature.as_ref().to_vec()]);
            }
        }
    }

    Ok(transaction)
}

pub(super) fn taproot_sighash_unavailable_error() -> WalletError {
    WalletError::UnsupportedSigningDescriptor(
        "taproot key-path sighash unavailable for built transaction".to_string(),
    )
}

pub(super) fn build_and_sign(
    wallet: &Wallet,
    request: &BuildRequest,
    coinbase_maturity: u32,
) -> Result<BuiltTransaction, WalletError> {
    let built = wallet.build_transaction(request, coinbase_maturity)?;
    let signed = sign_transaction(wallet, &built)?;

    Ok(BuiltTransaction {
        transaction: signed,
        ..built
    })
}

fn sign_ecdsa_low_s(private_key: &PrivateKey, digest: &[u8; 32]) -> Result<Vec<u8>, WalletError> {
    let secp = Secp256k1::new();
    let message = Message::from_digest(*digest);
    let mut signature = secp.sign_ecdsa(message, private_key.secret_key());
    signature.normalize_s();
    let mut encoded = signature.serialize_der().as_ref().to_vec();
    encoded.push(SigHashType::ALL.raw() as u8);
    Ok(encoded)
}

pub(super) fn push_script(pushes: &[&[u8]]) -> Result<ScriptBuf, WalletError> {
    let mut bytes = Vec::new();
    for push in pushes {
        bytes.extend_from_slice(&push_data(push)?);
    }
    Ok(ScriptBuf::from_bytes(bytes)?)
}

fn p2pkh_script(public_key: &secp256k1::PublicKey) -> Result<ScriptBuf, WalletError> {
    crate::address::p2pkh_script(public_key)
}

pub(super) fn standard_wallet_verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::STRICTENC
        | ScriptVerifyFlags::DERSIG
        | ScriptVerifyFlags::LOW_S
        | ScriptVerifyFlags::NULLDUMMY
        | ScriptVerifyFlags::SIGPUSHONLY
        | ScriptVerifyFlags::MINIMALDATA
        | ScriptVerifyFlags::CLEANSTACK
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
        | ScriptVerifyFlags::WITNESS
        | ScriptVerifyFlags::MINIMALIF
        | ScriptVerifyFlags::NULLFAIL
        | ScriptVerifyFlags::WITNESS_PUBKEYTYPE
        | ScriptVerifyFlags::TAPROOT
}
