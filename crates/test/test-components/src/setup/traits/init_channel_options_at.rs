use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    HasConnectionIdType, HasInitChannelOptionsType, InitChannelOptions,
};
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
    name: InitChannelOptionsGetterAtComponent<A, B>,
    provider: InitChannelOptionsGetterAt,
}]
pub trait HasInitChannelOptionsAt<A, B>:
    HasChainTypeAt<
        A,
        Chain: HasInitChannelOptionsType<ChainAt<Self, B>> + HasConnectionIdType<ChainAt<Self, B>>,
    > + HasChainTypeAt<B, Chain: HasConnectionIdType<ChainAt<Self, A>>>
{
    fn init_channel_options(
        &self,
        connection_id: &ConnectionIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
        counterparty_connection_id: &ConnectionIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
    ) -> InitChannelOptions<ChainAt<Self, A>, ChainAt<Self, B>>;
}
