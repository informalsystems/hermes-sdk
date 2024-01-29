use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::channel::{
    HasInitChannelOptionsType, InitChannelOptions,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};

#[derive_component(InitChannelOptionsAtComponent, ProvideInitChannelOptionsAt<Setup>)]
pub trait HasInitChannelOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainTypeAt<Self, TARGET>: HasInitChannelOptionsType<ChainTypeAt<Self, COUNTERPARTY>>
        + HasIbcChainTypes<ChainTypeAt<Self, COUNTERPARTY>>,
    ChainTypeAt<Self, COUNTERPARTY>: HasIbcChainTypes<ChainTypeAt<Self, TARGET>>,
{
    fn init_channel_options(
        &self,
        connection_id: &ConnectionIdOf<ChainTypeAt<Self, TARGET>, ChainTypeAt<Self, COUNTERPARTY>>,
        counterparty_connection_id: &ConnectionIdOf<
            ChainTypeAt<Self, COUNTERPARTY>,
            ChainTypeAt<Self, TARGET>,
        >,
    ) -> InitChannelOptions<ChainTypeAt<Self, TARGET>, ChainTypeAt<Self, COUNTERPARTY>>;
}
