use core::marker::PhantomData;

use cgp::core::field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::connection::{
    HasInitConnectionOptionsType, InitConnectionOptionsOf,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
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

#[cgp_provider(InitConnectionOptionsGetterAtComponent<A, B>)]
impl<Setup, A, B, Tag, Chain, Counterparty, InitConnectionOptions>
    InitConnectionOptionsGetterAt<Setup, A, B> for UseField<Tag>
where
    Setup: HasChainTypeAt<A, Chain = Chain> + HasChainTypeAt<B, Chain = Counterparty>,
    Chain:
        HasInitConnectionOptionsType<Counterparty, InitConnectionOptions = InitConnectionOptions>,
    Setup: HasField<Tag, Value = InitConnectionOptions>,
{
    fn init_connection_options(
        setup: &Setup,
    ) -> &InitConnectionOptionsOf<ChainAt<Setup, A>, ChainAt<Setup, B>> {
        setup.get_field(PhantomData)
    }
}
