use std::io::Error as IoError;

use borsh::BorshSerialize;
use cgp_core::CanRaiseError;
use ed25519_dalek::SigningKey;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::traits::encode_tx::TxEncoder;
use hermes_relayer_components::transaction::traits::types::fee::HasFeeType;
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;

use crate::impls::errors::multi_message_unsupported::MultiMessageUnsupportedError;
use crate::types::message::SovereignMessage;
use crate::types::rollup_id::RollupId;
use crate::utils::encode_tx::encode_and_sign_sovereign_tx;

pub struct EncodeSovereignTx;

impl<Rollup> TxEncoder<Rollup> for EncodeSovereignTx
where
    Rollup: HasSignerType<Signer = SigningKey>
        + HasNonceType<Nonce = u64>
        + HasFeeType<Fee = u64>
        + HasMessageType<Message = SovereignMessage>
        + HasTransactionType<Transaction = Vec<u8>>
        + HasChainId<ChainId = RollupId>
        + CanRaiseError<IoError>
        + for<'a> CanRaiseError<MultiMessageUnsupportedError<'a, Rollup>>,
{
    async fn encode_tx(
        rollup: &Rollup,
        signer: &SigningKey,
        nonce: &u64,
        fee: &u64,
        messages: &[SovereignMessage],
    ) -> Result<Rollup::Transaction, Rollup::Error> {
        let messages_vec: Vec<&SovereignMessage> = messages.iter().collect();

        let [message]: [&SovereignMessage; 1] = messages_vec
            .try_into()
            .map_err(|_| Rollup::raise_error(MultiMessageUnsupportedError { messages }))?;

        let message_bytes = message.try_to_vec().map_err(Rollup::raise_error)?;

        let rollup_id = rollup.chain_id();

        let transaction = encode_and_sign_sovereign_tx(
            signer,
            message_bytes.clone(),
            rollup_id.0,
            0,
            *fee,
            *nonce,
        )
        .map_err(Rollup::raise_error)?;

        Ok(transaction)
    }
}
