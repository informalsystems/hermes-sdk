use alloc::borrow::Cow;
use alloc::sync::Arc;

use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateFields;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetter;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use tendermint::abci::Event as AbciEvent;

use crate::chain::components::CosmosChainComponents;
use crate::contexts::chain::CosmosChain;

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

impl<Counterparty> CounterpartyMessageHeightGetter<CosmosChain, Counterparty>
    for CosmosChainComponents
where
    Counterparty: HasHeightType<Height = Height>,
{
    fn counterparty_message_height_for_update_client(message: &CosmosMessage) -> Option<Height> {
        message
            .message
            .counterparty_message_height_for_update_client()
    }
}

impl<Counterparty> HasConsensusStateFields<Counterparty> for CosmosChain
where
    Counterparty: HasTimestampType,
{
    fn consensus_state_timestamp(
        consensus_state: &Self::ConsensusState,
    ) -> Cow<'_, Counterparty::Timestamp> {
        // FIXME(romac): This is a temporary workaround until we have a proper conversion,
        // and can blow out if the timestamp is later than July 21st, 2554.
        let nanos = consensus_state.timestamp.unix_timestamp_nanos() as u64;
        Cow::Owned(Counterparty::timestamp_from_nanos(nanos))
    }
}
