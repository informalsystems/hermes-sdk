use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_chain_components::traits::types::ibc::HasClientIdType;

use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::multi::traits::relay_at::ClientIdAt;

#[derive_component(ClientIdAtGetterComponent<Chain, Counterparty>, ClientIdAtGetter<Context>)]
pub trait HasClientIdAt<Chain, Counterparty>:
    HasChainTypeAt<Chain, Chain: HasClientIdType<ChainAt<Self, Counterparty>>>
    + HasChainTypeAt<Counterparty>
{
    fn client_id_at(
        &self,
        _tag: PhantomData<(Chain, Counterparty)>,
    ) -> &ClientIdAt<Self, Chain, Counterparty>;
}

impl<Context, ChainTag, CounterpartyTag, FieldTag, Chain, Counterparty, ClientId>
    ClientIdAtGetter<Context, ChainTag, CounterpartyTag> for UseField<FieldTag>
where
    Context: HasChainTypeAt<ChainTag, Chain = Chain>
        + HasChainTypeAt<CounterpartyTag, Chain = Counterparty>
        + HasField<FieldTag, Value = ClientId>,
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>,
{
    fn client_id_at(
        context: &Context,
        _tag: PhantomData<(ChainTag, CounterpartyTag)>,
    ) -> &ClientId {
        context.get_field(PhantomData)
    }
}
