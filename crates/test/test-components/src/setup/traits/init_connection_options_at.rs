use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::connection::{
    HasInitConnectionOptionsType, InitConnectionOptions,
};

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};

#[derive_component(InitConnectionOptionsAtComponent, ProvideInitConnectionOptionsAt<Setup>)]
pub trait HasInitConnectionOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainTypeAt<Self, TARGET>: HasInitConnectionOptionsType<ChainTypeAt<Self, COUNTERPARTY>>,
{
    fn init_connection_options(
        &self,
    ) -> &InitConnectionOptions<ChainTypeAt<Self, TARGET>, ChainTypeAt<Self, COUNTERPARTY>>;
}
