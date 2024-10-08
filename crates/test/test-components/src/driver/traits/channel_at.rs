use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(ChannelGetterAtComponent, ChannelGetterAt<ChainDriver>)]
pub trait HasChannelAt<const CHAIN: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<CHAIN> + HasChainTypeAt<COUNTERPARTY>
where
    ChainAt<Self, CHAIN>: HasIbcChainTypes<ChainAt<Self, COUNTERPARTY>>,
{
    fn channel_id_at(
        &self,
        index: Twindex<CHAIN, COUNTERPARTY>,
    ) -> &ChannelIdOf<ChainAt<Self, CHAIN>, ChainAt<Self, COUNTERPARTY>>;

    fn port_id_at(
        &self,
        index: Twindex<CHAIN, COUNTERPARTY>,
    ) -> &PortIdOf<ChainAt<Self, CHAIN>, ChainAt<Self, COUNTERPARTY>>;
}
