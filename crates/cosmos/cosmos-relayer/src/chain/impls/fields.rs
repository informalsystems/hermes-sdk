use alloc::sync::Arc;

use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_cosmos_client_components::traits::message::CosmosMessage;
use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use hermes_relayer_components::chain::traits::types::message::CanEstimateMessageSize;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height;
use prost::Message;
use tendermint::abci::Event as AbciEvent;

use crate::chain::components::CosmosChainComponents;
use crate::contexts::chain::CosmosChain;
use crate::types::error::{BaseError, Error};

impl CanEstimateMessageSize for CosmosChain {
    fn estimate_message_size(message: &CosmosMessage) -> Result<usize, Error> {
        let raw = message
            .message
            .encode_protobuf(&Signer::dummy())
            .map_err(BaseError::encode)?;

        Ok(raw.encoded_len())
    }
}

impl ChainIdGetter<CosmosChain> for CosmosChainComponents {
    fn chain_id(chain: &CosmosChain) -> &ChainId {
        &chain.chain_id
    }
}

impl HasEventSubscription for CosmosChain {
    fn event_subscription(&self) -> &Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>> {
        &self.subscription
    }
}

impl<Counterparty> HasCounterpartyMessageHeight<Counterparty> for CosmosChain
where
    Counterparty: HasHeightType<Height = Height>,
{
    fn counterparty_message_height_for_update_client(message: &CosmosMessage) -> Option<Height> {
        message
            .message
            .counterparty_message_height_for_update_client()
    }
}

impl<Counterparty> HasClientStateFields<Counterparty> for CosmosChain {
    fn client_state_chain_id(client_state: &Self::ClientState) -> &Self::ChainId {
        &client_state.chain_id
    }

    fn client_state_latest_height(client_state: &TendermintClientState) -> &Height {
        &client_state.latest_height
    }
}
