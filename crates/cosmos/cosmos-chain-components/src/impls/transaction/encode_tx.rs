#![allow(clippy::ptr_arg)]

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{HasChainId, HasMessageType};
use hermes_relayer_components::transaction::traits::{
    HasFeeType, HasNonceType, HasSignerType, HasTransactionType, TxEncoder, TxEncoderComponent,
};
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::Signer;
use ibc_proto::cosmos::tx::v1beta1::mode_info::{Single, Sum};
use ibc_proto::cosmos::tx::v1beta1::{AuthInfo, Fee, ModeInfo, SignDoc, SignerInfo, TxBody};
use ibc_proto::google::protobuf::Any;
use prost::{EncodeError, Message};

use crate::traits::message::CosmosMessage;
use crate::traits::tx_extension_options::HasTxExtensionOptions;
use crate::types::key_types::secp256k1::Secp256k1KeyPair;
use crate::types::transaction::account::Account;
use crate::types::transaction::memo::Memo;
use crate::types::transaction::signed_tx::SignedTx;

pub struct EncodeCosmosTx;

#[cgp_provider(TxEncoderComponent)]
impl<Chain> TxEncoder<Chain> for EncodeCosmosTx
where
    Chain: HasSignerType<Signer = Secp256k1KeyPair>
        + HasNonceType<Nonce = Account>
        + HasFeeType<Fee = Fee>
        + HasMessageType<Message = CosmosMessage>
        + HasTransactionType<Transaction = SignedTx>
        + HasTxExtensionOptions
        + HasChainId<ChainId = ChainId>
        + CanRaiseAsyncError<EncodeError>
        + CanRaiseAsyncError<String>,
{
    async fn encode_tx(
        chain: &Chain,
        key_pair: &Secp256k1KeyPair,
        account: &Account,
        fee: &Fee,
        messages: &[CosmosMessage],
    ) -> Result<SignedTx, Chain::Error> {
        let signer: Signer = key_pair.account().into();

        let raw_messages = messages
            .iter()
            .map(|message| message.message.encode_protobuf(&signer))
            .collect::<Vec<_>>();

        let memo = Memo::default();
        let chain_id = chain.chain_id();
        let extension_options = chain.tx_extension_options();

        let signed_tx = sign_tx::<Chain>(
            chain_id,
            key_pair,
            account,
            &memo,
            &raw_messages,
            fee,
            extension_options,
        )?;

        Ok(signed_tx)
    }
}

pub fn sign_tx<Chain>(
    chain_id: &ChainId,
    key_pair: &Secp256k1KeyPair,
    account: &Account,
    tx_memo: &Memo,
    messages: &[Any],
    fee: &Fee,
    extension_options: &Vec<Any>,
) -> Result<SignedTx, Chain::Error>
where
    Chain: CanRaiseAsyncError<String>,
{
    let key_bytes = Message::encode_to_vec(&key_pair.public_key.serialize().to_vec());

    let signer = encode_signer_info(account.sequence, key_bytes);

    let (body, body_bytes) = tx_body_and_bytes(messages, tx_memo, extension_options);

    let (auth_info, auth_info_bytes) = auth_info_and_bytes(signer, fee.clone());

    let signed_doc = encode_sign_doc::<Chain>(
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

pub fn encode_sign_doc<Chain>(
    chain_id: &ChainId,
    key_pair: &Secp256k1KeyPair,
    account_number: u64,
    auth_info_bytes: Vec<u8>,
    body_bytes: Vec<u8>,
) -> Result<Vec<u8>, Chain::Error>
where
    Chain: CanRaiseAsyncError<String>,
{
    let sign_doc = SignDoc {
        body_bytes,
        auth_info_bytes,
        chain_id: chain_id.to_string(),
        account_number,
    };

    let signdoc_buf = Message::encode_to_vec(&sign_doc);

    let signed = key_pair.sign(&signdoc_buf).map_err(Chain::raise_error)?;

    Ok(signed)
}

pub fn encode_signer_info(sequence: u64, key_bytes: Vec<u8>) -> SignerInfo {
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
        sequence,
    }
}

#[allow(deprecated)]
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
