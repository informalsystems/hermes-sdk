use alloc::sync::Arc;

use cgp_core::Async;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_cosmos_client_components::traits::message::CosmosMessage;
use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use hermes_relayer_components::chain::traits::types::height::{CanIncrementHeight, HasHeightType};
use hermes_relayer_components::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use hermes_relayer_components::chain::traits::types::message::CanEstimateMessageSize;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height;
use prost::Message;
use tendermint::abci::Event as AbciEvent;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::component::CosmosChainComponents;
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

impl<Chain> ChainIdGetter<CosmosChain<Chain>> for CosmosChainComponents
where
    Chain: Async,
{
    fn chain_id(chain: &CosmosChain<Chain>) -> &ChainId {
        &chain.chain_id
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
