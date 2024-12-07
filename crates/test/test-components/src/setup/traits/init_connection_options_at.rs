use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::connection::{
    HasInitConnectionOptionsType, InitConnectionOptionsOf,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[derive_component(InitConnectionOptionsAtComponent, ProvideInitConnectionOptionsAt<Setup>)]
pub trait HasInitConnectionOptionsAt<Target: Async, Counterparty: Async>:
    HasChainTypeAt<Target, Chain: HasInitConnectionOptionsType<ChainAt<Self, Counterparty>>>
    + HasChainTypeAt<Counterparty>
{
    fn init_connection_options(
        &self,
    ) -> &InitConnectionOptionsOf<ChainAt<Self, Target>, ChainAt<Self, Counterparty>>;
}

impl<
        Setup,
        Target: Async,
        CounterpartyTag: Async,
        Tag,
        Chain,
        Counterparty,
        InitConnectionOptions,
    > ProvideInitConnectionOptionsAt<Setup, Target, CounterpartyTag> for UseField<Tag>
where
    Setup: HasChainTypeAt<Target, Chain = Chain>
        + HasChainTypeAt<CounterpartyTag, Chain = Counterparty>,
    Chain:
        HasInitConnectionOptionsType<Counterparty, InitConnectionOptions = InitConnectionOptions>,
    Setup: HasField<Tag, Value = InitConnectionOptions>,
{
    fn init_connection_options(
        setup: &Setup,
    ) -> &InitConnectionOptionsOf<ChainAt<Setup, Target>, ChainAt<Setup, CounterpartyTag>> {
        setup.get_field(PhantomData)
    }
}
