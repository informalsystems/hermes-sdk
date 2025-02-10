use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasConnectionIdType;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_test_components::setup::traits::init_channel_options_at::{
    InitChannelOptionsAtComponent, ProvideInitChannelOptionsAt,
};
use ibc::core::host::types::identifiers::ConnectionId;

pub struct UseCosmosInitChannelOptions;

#[cgp_provider(InitChannelOptionsAtComponent)]
impl<Context, Chain, Counterparty, TargetTag: Async, CounterpartyTag: Async>
    ProvideInitChannelOptionsAt<Context, TargetTag, CounterpartyTag> for UseCosmosInitChannelOptions
where
    Context: HasChainTypeAt<TargetTag, Chain = Chain>
        + HasChainTypeAt<CounterpartyTag, Chain = Counterparty>
        + HasField<symbol!("init_channel_options"), Value = CosmosInitChannelOptions>,
    Chain: HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>,
    Counterparty: HasConnectionIdType<Chain>,
{
    fn init_channel_options(
        context: &Context,
        connection_id: &ConnectionId,
        _counterparty_connection_id: &Counterparty::ConnectionId,
    ) -> CosmosInitChannelOptions {
        let mut options = context.get_field(PhantomData).clone();

        // Use an init channel options that is provided by the setup.
        // Insert the connection ID to the front (or to the back?) to allow
        // testing multihop connections in the future.
        options.connection_hops.insert(0, connection_id.clone());

        options
    }
}
