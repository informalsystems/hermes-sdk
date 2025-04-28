use core::marker::PhantomData;

use cgp::core::field::UseField;
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasPortIdType;
use hermes_relayer_components::chain::types::aliases::PortIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_getter {
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
