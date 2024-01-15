use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelId, PortId};

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::types::index::Twindex;

#[derive_component(ChannelGetterAtComponent, ChannelGetterAt<ChainDriver>)]
pub trait HasChannelAt<const CHAIN: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<CHAIN> + HasChainTypeAt<COUNTERPARTY>
where
    ChainTypeAt<Self, CHAIN>: HasIbcChainTypes<ChainTypeAt<Self, COUNTERPARTY>>,
{
    fn channel_id_at(
        &self,
        index: Twindex<CHAIN, COUNTERPARTY>,
    ) -> &ChannelId<ChainTypeAt<Self, CHAIN>, ChainTypeAt<Self, COUNTERPARTY>>;

    fn port_id_at(
        &self,
        index: Twindex<CHAIN, COUNTERPARTY>,
    ) -> &PortId<ChainTypeAt<Self, CHAIN>, ChainTypeAt<Self, COUNTERPARTY>>;
}
