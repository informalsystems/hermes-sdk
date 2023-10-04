use alloc::sync::Arc;
use cgp_core::Async;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::chain::traits::types::height::CanIncrementHeight;
use ibc_relayer_components::chain::traits::types::message::CanEstimateMessageSize;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height;
use prost::Message;

use crate::contexts::chain::CosmosChain;
use crate::traits::message::CosmosMessage;
use crate::types::error::{BaseError, Error};

impl<Chain> CanIncrementHeight for CosmosChain<Chain>
where
    Chain: Async,
{
    fn increment_height(height: &Height) -> Result<Height, Error> {
        Ok(height.increment())
    }
}

impl<Chain> CanEstimateMessageSize for CosmosChain<Chain>
where
    Chain: Async,
{
    fn estimate_message_size(message: &Arc<dyn CosmosMessage>) -> Result<usize, Error> {
        let raw = message
            .encode_protobuf(&Signer::dummy())
            .map_err(BaseError::encode)?;

        Ok(raw.encoded_len())
    }
}

impl<Chain> HasChainId for CosmosChain<Chain>
where
    Chain: Async,
{
    fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }
}
