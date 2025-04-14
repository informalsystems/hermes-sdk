use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cosmos_chain_components::types::CosmosInitChannelOptions;
use hermes_relayer_components::chain::traits::{HasConnectionIdType, HasInitChannelOptionsType};
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_test_components::setup::traits::{
    InitChannelOptionsGetterAt, InitChannelOptionsGetterAtComponent,
};
use ibc::core::host::types::identifiers::ConnectionId;

#[cgp_new_provider(InitChannelOptionsGetterAtComponent<A, B>)]
impl<Context, Chain, Counterparty, A, B, Tag> InitChannelOptionsGetterAt<Context, A, B>
    for UseCosmosInitChannelOptions<Tag>
where
    Context: HasChainTypeAt<A, Chain = Chain>
        + HasChainTypeAt<B, Chain = Counterparty>
        + HasField<Tag, Value = CosmosInitChannelOptions>,
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
