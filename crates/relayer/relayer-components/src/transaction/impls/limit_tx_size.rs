/*!
   TODO: the max transaction size may not be checked within the transaction
   encoder. Doing so may interfer with the nonce allocator, as it would
   invalidate subsequent nonces that are allocated, since the currently
   allocated nonce is not used.
*/

use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::types::message::HasMessageType;
use crate::transaction::traits::encode_tx::{TxEncoder, TxEncoderComponent};
use crate::transaction::traits::types::fee::HasFeeType;
use crate::transaction::traits::types::nonce::HasNonceType;
use crate::transaction::traits::types::signer::HasSignerType;
use crate::transaction::traits::types::transaction::HasTransactionType;

#[derive(Debug)]
pub struct MaxTxSizeExceededError {
    pub max_tx_size: usize,
    pub given_tx_size: usize,
}

pub trait HasMaxTxSizeExceededError: CanRaiseAsyncError<MaxTxSizeExceededError> {
    fn try_extract_max_tx_size_exceeded_error(e: Self::Error) -> Option<MaxTxSizeExceededError>;
}

pub trait HasMaxTxSize {
    fn max_tx_size(&self) -> usize;
}

pub struct LimitEncodedTxSize<InEncoder>(PhantomData<InEncoder>);

#[cgp_provider(TxEncoderComponent)]
impl<Chain, InEncoder> TxEncoder<Chain> for LimitEncodedTxSize<InEncoder>
where
    Chain: HasSignerType
        + HasNonceType
        + HasFeeType
        + HasMessageType
        + HasTransactionType
        + HasMaxTxSize
        + HasMaxTxSizeExceededError,
    InEncoder: TxEncoder<Chain>,
{
    async fn encode_tx(
        chain: &Chain,
        signer: &Chain::Signer,
        nonce: &Chain::Nonce,
        fee: &Chain::Fee,
        messages: &[Chain::Message],
    ) -> Result<Chain::Transaction, Chain::Error> {
        let tx = InEncoder::encode_tx(chain, signer, nonce, fee, messages).await?;

        let given_tx_size = Chain::tx_size(&tx);
        let max_tx_size = chain.max_tx_size();

        if given_tx_size > max_tx_size {
            Err(Chain::raise_error(MaxTxSizeExceededError {
                given_tx_size,
                max_tx_size,
            }))
        } else {
            Ok(tx)
        }
    }
}
