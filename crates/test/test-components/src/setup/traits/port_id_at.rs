use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasPortIdType;
use hermes_relayer_components::chain::types::aliases::PortIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(PortIdAtComponent, ProvidePortIdAt<Context>)]
pub trait HasPortIdAt<const CHAIN: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<CHAIN, Chain: HasPortIdType<ChainAt<Self, COUNTERPARTY>>>
    + HasChainTypeAt<COUNTERPARTY>
{
    fn port_id_at(
        &self,
        _index: Twindex<CHAIN, COUNTERPARTY>,
    ) -> &PortIdOf<ChainAt<Self, CHAIN>, ChainAt<Self, COUNTERPARTY>>;
}

impl<Context, const CHAIN: usize, const COUNTERPARTY: usize, Tag, Chain, Counterparty, PortId>
    ProvidePortIdAt<Context, CHAIN, COUNTERPARTY> for UseField<Tag>
where
    Context: HasChainTypeAt<CHAIN, Chain = Chain>
        + HasChainTypeAt<COUNTERPARTY, Chain = Counterparty>
        + HasField<Tag, Field = PortId>,
    Chain: HasPortIdType<Counterparty, PortId = PortId>,
{
    fn port_id_at(context: &Context, _index: Twindex<CHAIN, COUNTERPARTY>) -> &PortId {
        context.get_field(PhantomData)
    }
}
