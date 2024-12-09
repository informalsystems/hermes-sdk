use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
  provider: ChannelGetterAt,
  context: ChainDriver,
}]
pub trait HasChannelAt<Chain: Async, Counterparty: Async>:
    HasChainTypeAt<Chain> + HasChainTypeAt<Counterparty>
where
    ChainAt<Self, Chain>: HasIbcChainTypes<ChainAt<Self, Counterparty>>,
{
    fn channel_id_at(
        &self,
        _index: PhantomData<(Chain, Counterparty)>,
    ) -> &ChannelIdOf<ChainAt<Self, Chain>, ChainAt<Self, Counterparty>>;

    fn port_id_at(
        &self,
        _index: PhantomData<(Chain, Counterparty)>,
    ) -> &PortIdOf<ChainAt<Self, Chain>, ChainAt<Self, Counterparty>>;
}
