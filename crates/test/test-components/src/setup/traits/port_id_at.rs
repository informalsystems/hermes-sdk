use core::marker::PhantomData;

use cgp::core::field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasPortIdType;
use hermes_relayer_components::chain::types::aliases::PortIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
  name: PortIdAtComponent,
  provider: ProvidePortIdAt,
}]
pub trait HasPortIdAt<TargetTag: Async, CounterpartyTag: Async>:
    HasChainTypeAt<TargetTag, Chain: HasPortIdType<ChainAt<Self, CounterpartyTag>>>
    + HasChainTypeAt<CounterpartyTag>
{
    fn port_id_at(
        &self,
        _index: PhantomData<(TargetTag, CounterpartyTag)>,
    ) -> &PortIdOf<ChainAt<Self, TargetTag>, ChainAt<Self, CounterpartyTag>>;
}

#[cgp_provider(PortIdAtComponent)]
impl<Context, TargetTag: Async, CounterpartyTag: Async, Tag, Chain, Counterparty, PortId>
    ProvidePortIdAt<Context, TargetTag, CounterpartyTag> for UseField<Tag>
where
    Context: HasChainTypeAt<TargetTag, Chain = Chain>
        + HasChainTypeAt<CounterpartyTag, Chain = Counterparty>
        + HasField<Tag, Value = PortId>,
    Chain: HasPortIdType<Counterparty, PortId = PortId>,
{
    fn port_id_at(context: &Context, _index: PhantomData<(TargetTag, CounterpartyTag)>) -> &PortId {
        context.get_field(PhantomData)
    }
}
