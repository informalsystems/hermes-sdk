use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::channel::{
    HasInitChannelOptionsType, InitChannelOptions,
};
use hermes_relayer_components::chain::traits::types::ibc::HasConnectionIdType;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
  name: InitChannelOptionsAtComponent,
  provider: ProvideInitChannelOptionsAt,
  context: Setup,
}]
pub trait HasInitChannelOptionsAt<Target: Async, Counterparty: Async>:
    HasChainTypeAt<
        Target,
        Chain: HasInitChannelOptionsType<ChainAt<Self, Counterparty>>
                   + HasConnectionIdType<ChainAt<Self, Counterparty>>,
    > + HasChainTypeAt<Counterparty, Chain: HasConnectionIdType<ChainAt<Self, Target>>>
{
    fn init_channel_options(
        &self,
        connection_id: &ConnectionIdOf<ChainAt<Self, Target>, ChainAt<Self, Counterparty>>,
        counterparty_connection_id: &ConnectionIdOf<
            ChainAt<Self, Counterparty>,
            ChainAt<Self, Target>,
        >,
    ) -> InitChannelOptions<ChainAt<Self, Target>, ChainAt<Self, Counterparty>>;
}
