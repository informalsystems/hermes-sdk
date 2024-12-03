use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::channel::{
    HasInitChannelOptionsType, InitChannelOptions,
};
use hermes_relayer_components::chain::traits::types::ibc::HasConnectionIdType;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[derive_component(InitChannelOptionsAtComponent, ProvideInitChannelOptionsAt<Setup>)]
pub trait HasInitChannelOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<
        TARGET,
        Chain: HasInitChannelOptionsType<ChainAt<Self, COUNTERPARTY>>
                   + HasConnectionIdType<ChainAt<Self, COUNTERPARTY>>,
    > + HasChainTypeAt<COUNTERPARTY, Chain: HasConnectionIdType<ChainAt<Self, TARGET>>>
{
    fn init_channel_options(
        &self,
        connection_id: &ConnectionIdOf<ChainAt<Self, TARGET>, ChainAt<Self, COUNTERPARTY>>,
        counterparty_connection_id: &ConnectionIdOf<
            ChainAt<Self, COUNTERPARTY>,
            ChainAt<Self, TARGET>,
        >,
    ) -> InitChannelOptions<ChainAt<Self, TARGET>, ChainAt<Self, COUNTERPARTY>>;
}
