use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::connection::{
    HasInitConnectionOptionsType, InitConnectionOptionsOf,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[derive_component(InitConnectionOptionsAtComponent, ProvideInitConnectionOptionsAt<Setup>)]
pub trait HasInitConnectionOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET, Chain: HasInitConnectionOptionsType<ChainAt<Self, COUNTERPARTY>>>
    + HasChainTypeAt<COUNTERPARTY>
{
    fn init_connection_options(
        &self,
    ) -> &InitConnectionOptionsOf<ChainAt<Self, TARGET>, ChainAt<Self, COUNTERPARTY>>;
}

impl<
        Setup,
        const TARGET: usize,
        const COUNTERPARTY: usize,
        Tag,
        Chain,
        Counterparty,
        InitConnectionOptions,
    > ProvideInitConnectionOptionsAt<Setup, TARGET, COUNTERPARTY> for UseField<Tag>
where
    Setup:
        HasChainTypeAt<TARGET, Chain = Chain> + HasChainTypeAt<COUNTERPARTY, Chain = Counterparty>,
    Chain:
        HasInitConnectionOptionsType<Counterparty, InitConnectionOptions = InitConnectionOptions>,
    Setup: HasField<Tag, Field = InitConnectionOptions>,
{
    fn init_connection_options(
        setup: &Setup,
    ) -> &InitConnectionOptionsOf<ChainAt<Setup, TARGET>, ChainAt<Setup, COUNTERPARTY>> {
        setup.get_field(PhantomData)
    }
}
