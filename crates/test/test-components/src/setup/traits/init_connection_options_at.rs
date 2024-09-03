use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::connection::{
    HasInitConnectionOptionsType, InitConnectionOptionsOf,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[derive_component(InitConnectionOptionsAtComponent, ProvideInitConnectionOptionsAt<Setup>)]
pub trait HasInitConnectionOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainAt<Self, TARGET>: HasInitConnectionOptionsType<ChainAt<Self, COUNTERPARTY>>,
{
    fn init_connection_options(
        &self,
    ) -> InitConnectionOptionsOf<ChainAt<Self, TARGET>, ChainAt<Self, COUNTERPARTY>>;
}
