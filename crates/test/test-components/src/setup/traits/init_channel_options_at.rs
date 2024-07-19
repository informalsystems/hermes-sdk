use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::channel::{
    HasInitChannelOptionsType, InitChannelOptions,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[derive_component(InitChannelOptionsAtComponent, ProvideInitChannelOptionsAt<Setup>)]
pub trait HasInitChannelOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainAt<Self, TARGET>: HasInitChannelOptionsType<ChainAt<Self, COUNTERPARTY>>
        + HasIbcChainTypes<ChainAt<Self, COUNTERPARTY>>,
    ChainAt<Self, COUNTERPARTY>: HasIbcChainTypes<ChainAt<Self, TARGET>>,
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
