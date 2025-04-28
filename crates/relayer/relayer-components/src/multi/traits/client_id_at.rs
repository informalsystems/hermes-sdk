use core::marker::PhantomData;

use cgp::core::field::UseField;
use hermes_chain_components::traits::HasClientIdType;
use hermes_prelude::*;

use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::multi::traits::relay_at::ClientIdAt;

#[cgp_component {
  name: ClientIdAtGetterComponent<Chain, Counterparty>,
  provider: ClientIdAtGetter,
}]
pub trait HasClientIdAt<Chain, Counterparty>:
    HasChainTypeAt<Chain, Chain: HasClientIdType<ChainAt<Self, Counterparty>>>
    + HasChainTypeAt<Counterparty>
{
    fn client_id_at(
        &self,
        _tag: PhantomData<(Chain, Counterparty)>,
    ) -> &ClientIdAt<Self, Chain, Counterparty>;
}

#[cgp_provider(ClientIdAtGetterComponent<ChainTag, CounterpartyTag>)]
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
