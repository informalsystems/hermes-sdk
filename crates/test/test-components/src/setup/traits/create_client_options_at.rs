use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientOptions, HasCreateClientOptionsType,
};

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::types::index::Twindex;

#[derive_component(CreateClientOptionsAtComponent, ProvideCreateClientOptionsAt<Setup>)]
pub trait HasCreateClientOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainTypeAt<Self, TARGET>: HasCreateClientOptionsType<ChainTypeAt<Self, COUNTERPARTY>>,
{
    fn create_client_options(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &CreateClientOptions<ChainTypeAt<Self, TARGET>, ChainTypeAt<Self, COUNTERPARTY>>;
}
