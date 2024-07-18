use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::PortIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainTypeAt, HasChainTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(PortIdAtComponent, ProvidePortIdAt<Context>)]
pub trait HasPortIdAt<const CHAIN: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<CHAIN> + HasChainTypeAt<COUNTERPARTY>
where
    ChainTypeAt<Self, CHAIN>: HasIbcChainTypes<ChainTypeAt<Self, COUNTERPARTY>>,
{
    fn port_id_at(
        &self,
        index: Twindex<CHAIN, COUNTERPARTY>,
    ) -> &PortIdOf<ChainTypeAt<Self, CHAIN>, ChainTypeAt<Self, COUNTERPARTY>>;
}
