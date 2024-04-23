#![allow(clippy::ptr_arg)]

use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::traits::encode_tx::TxEncoder;
use hermes_relayer_components::transaction::traits::types::fee::HasFeeType;
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;
use ibc_proto::cosmos::tx::v1beta1::mode_info::{Single, Sum};
use ibc_proto::cosmos::tx::v1beta1::{AuthInfo, Fee, ModeInfo, SignDoc, SignerInfo, TxBody};
use ibc_proto::google::protobuf::Any;
use ibc_relayer::chain::cosmos::types::account::{Account, AccountNumber, AccountSequence};
use ibc_relayer::chain::cosmos::types::tx::SignedTx;
use ibc_relayer::config::types::Memo;
use ibc_relayer::keyring::errors::Error as KeyringError;
use ibc_relayer::keyring::{Secp256k1KeyPair, SigningKeyPair};
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::signer::{Signer, SignerError};
use prost::{EncodeError, Message};

use crate::traits::message::CosmosMessage;
use crate::traits::tx_extension_options::HasTxExtensionOptions;

pub struct EncodeCosmosTx;

impl<Chain> TxEncoder<Chain> for EncodeCosmosTx
where
    Chain: HasSignerType<Signer = Secp256k1KeyPair>
        + HasNonceType<Nonce = Account>
        + HasFeeType<Fee = Fee>
        + HasMessageType<Message = CosmosMessage>
        + HasTransactionType<Transaction = SignedTx>
        + HasTxExtensionOptions
        + HasChainId<ChainId = ChainId>
        + CanRaiseError<EncodeError>
        + CanRaiseError<SignerError>
        + CanRaiseError<KeyringError>,
{
    async fn encode_tx(
        chain: &Chain,
        key_pair: &Secp256k1KeyPair,
        account: &Account,
        fee: &Fee,
        messages: &[CosmosMessage],
    ) -> Result<SignedTx, Chain::Error> {
        let signer: Signer = key_pair.account().parse().map_err(Chain::raise_error)?;

        let raw_messages = messages
            .iter()
            .map(|message| message.message.encode_protobuf(&signer))
            .collect::<Vec<_>>();

        let memo = Memo::default();
        let chain_id = chain.chain_id();
        let extension_options = chain.tx_extension_options();

        let signed_tx = sign_tx(
            chain_id,
            key_pair,
            account,
            &memo,
            &raw_messages,
            fee,
            extension_options,
        )
        .map_err(Chain::raise_error)?;

        Ok(signed_tx)
    }
}

pub fn sign_tx(
    chain_id: &ChainId,
    key_pair: &Secp256k1KeyPair,
    account: &Account,
    tx_memo: &Memo,
    messages: &[Any],
    fee: &Fee,
    extension_options: &Vec<Any>,
) -> Result<SignedTx, KeyringError> {
    let key_bytes = Message::encode_to_vec(&key_pair.public_key.serialize().to_vec());

    let signer = encode_signer_info(account.sequence, key_bytes);

    let (body, body_bytes) = tx_body_and_bytes(messages, tx_memo, extension_options);

    let (auth_info, auth_info_bytes) = auth_info_and_bytes(signer, fee.clone());

    let signed_doc = encode_sign_doc(
        chain_id,
        key_pair,
        account.number,
        auth_info_bytes.clone(),
        body_bytes.clone(),
    )?;

    Ok(SignedTx {
        body,
        body_bytes,
        auth_info,
        auth_info_bytes,
        signatures: vec![signed_doc],
    })
}

pub fn encode_sign_doc(
    chain_id: &ChainId,
    key_pair: &Secp256k1KeyPair,
    account_number: AccountNumber,
    auth_info_bytes: Vec<u8>,
    body_bytes: Vec<u8>,
) -> Result<Vec<u8>, KeyringError> {
    let sign_doc = SignDoc {
        body_bytes,
        auth_info_bytes,
        chain_id: chain_id.to_string(),
        account_number: account_number.to_u64(),
    };

    let signdoc_buf = Message::encode_to_vec(&sign_doc);

    let signed = key_pair.sign(&signdoc_buf)?;

    Ok(signed)
}

pub fn encode_signer_info(sequence: AccountSequence, key_bytes: Vec<u8>) -> SignerInfo {
    let pk_any = Any {
        type_url: "/cosmos.crypto.secp256k1.PubKey".to_string(),
        value: key_bytes,
    };

    let single = Single { mode: 1 };
    let sum_single = Some(Sum::Single(single));
    let mode = Some(ModeInfo { sum: sum_single });

    SignerInfo {
        public_key: Some(pk_any),
        mode_info: mode,
        sequence: sequence.to_u64(),
    }
}

pub fn auth_info_and_bytes(signer_info: SignerInfo, fee: Fee) -> (AuthInfo, Vec<u8>) {
    let auth_info = AuthInfo {
        signer_infos: vec![signer_info],
        fee: Some(fee),

        // Since Cosmos SDK v0.46.0
        tip: None,
    };

    let auth_buf = Message::encode_to_vec(&auth_info);

    (auth_info, auth_buf)
}

pub fn tx_body_and_bytes(
    proto_msgs: &[Any],
    memo: &Memo,
    extension_options: &Vec<Any>,
) -> (TxBody, Vec<u8>) {
    let body = TxBody {
        messages: proto_msgs.to_vec(),
        memo: memo.to_string(),
        timeout_height: 0_u64,
        extension_options: extension_options.clone(),
        non_critical_extension_options: Vec::<Any>::new(),
    };

    let body_buf = Message::encode_to_vec(&body);

    (body, body_buf)
}
