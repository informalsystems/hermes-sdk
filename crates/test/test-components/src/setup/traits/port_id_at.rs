use core::marker::PhantomData;

use cgp::core::field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasPortIdType;
use hermes_relayer_components::chain::types::aliases::PortIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
    name: PortIdGetterAtComponent<A, B>,
    provider: PortIdGetterAt,
}]
pub trait HasPortIdAt<A, B>:
    HasChainTypeAt<A, Chain: HasPortIdType<ChainAt<Self, B>>> + HasChainTypeAt<B>
{
    fn port_id_at(
        &self,
        _index: PhantomData<(A, B)>,
    ) -> &PortIdOf<ChainAt<Self, A>, ChainAt<Self, B>>;
}

#[cgp_provider(PortIdGetterAtComponent<A, B>)]
impl<Context, A, B, Tag, Chain, Counterparty, PortId> PortIdGetterAt<Context, A, B>
    for UseField<Tag>
where
    Context: HasChainTypeAt<A, Chain = Chain>
        + HasChainTypeAt<B, Chain = Counterparty>
        + HasField<Tag, Value = PortId>,
    Chain: HasPortIdType<Counterparty, PortId = PortId>,
{
    fn port_id_at(context: &Context, _index: PhantomData<(A, B)>) -> &PortId {
        context.get_field(PhantomData)
    }
}
