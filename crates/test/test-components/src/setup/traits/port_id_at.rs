use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasPortIdType;
use hermes_relayer_components::chain::types::aliases::PortIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(PortIdAtComponent, ProvidePortIdAt<Context>)]
pub trait HasPortIdAt<TargetTag: Async, CounterpartyTag: Async>:
    HasChainTypeAt<TargetTag, Chain: HasPortIdType<ChainAt<Self, CounterpartyTag>>>
    + HasChainTypeAt<CounterpartyTag>
{
    fn port_id_at(
        &self,
        _index: PhantomData<(TargetTag, CounterpartyTag)>,
    ) -> &PortIdOf<ChainAt<Self, TargetTag>, ChainAt<Self, CounterpartyTag>>;
}

impl<Context, TargetTag: Async, CounterpartyTag: Async, Tag, Chain, Counterparty, PortId>
    ProvidePortIdAt<Context, TargetTag, CounterpartyTag> for UseField<Tag>
where
    Context: HasChainTypeAt<TargetTag, Chain = Chain>
        + HasChainTypeAt<CounterpartyTag, Chain = Counterparty>
        + HasField<Tag, Field = PortId>,
    Chain: HasPortIdType<Counterparty, PortId = PortId>,
{
    fn port_id_at(context: &Context, _index: PhantomData<(TargetTag, CounterpartyTag)>) -> &PortId {
        context.get_field(PhantomData)
    }
}
