use cgp::core::field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    HasInitConnectionOptionsType, InitConnectionOptionsOf,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_getter {
    name: InitConnectionOptionsGetterAtComponent<A, B>,
    provider: InitConnectionOptionsGetterAt,
}]
pub trait HasInitConnectionOptionsAt<A, B>:
    HasChainTypeAt<A, Chain: HasInitConnectionOptionsType<ChainAt<Self, B>>> + HasChainTypeAt<B>
{
    fn init_connection_options(
        &self,
    ) -> &InitConnectionOptionsOf<ChainAt<Self, A>, ChainAt<Self, B>>;
}
