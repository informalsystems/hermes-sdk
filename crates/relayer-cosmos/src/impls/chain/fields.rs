use alloc::sync::Arc;

use cgp_core::Async;
use ibc_cosmos_client_components::traits::message::CosmosMessage;
use ibc_cosmos_client_components::types::tendermint::TendermintClientState;
use ibc_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use ibc_relayer_components::chain::traits::types::height::{CanIncrementHeight, HasHeightType};
use ibc_relayer_components::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use ibc_relayer_components::chain::traits::types::message::CanEstimateMessageSize;
use ibc_relayer_subscription::traits::subscription::Subscription;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height;
use prost::Message;
use tendermint::abci::Event as AbciEvent;

use crate::contexts::chain::CosmosChain;
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

impl<Chain> HasEventSubscription for CosmosChain<Chain>
where
    Chain: Async,
{
    fn event_subscription(&self) -> &Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>> {
        &self.subscription
    }
}

impl<Chain, Counterparty> HasCounterpartyMessageHeight<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
    Counterparty: HasHeightType<Height = Height>,
{
    fn counterparty_message_height_for_update_client(
        message: &Arc<dyn CosmosMessage>,
    ) -> Option<Height> {
        message.counterparty_message_height_for_update_client()
    }
}

impl<Chain, Counterparty> HasClientStateFields<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    fn client_state_latest_height(client_state: &TendermintClientState) -> &Height {
        &client_state.latest_height
    }
}
