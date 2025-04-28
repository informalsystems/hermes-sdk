use core::marker::PhantomData;

use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasChannelIdType;
use hermes_relayer_components::chain::types::aliases::ChannelIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_getter {
    name: ChannelIdGetterAtComponent<A, B>,
    provider: ChannelIdGetterAt,
}]
pub trait HasChannelIdAt<A, B>:
    HasChainTypeAt<A, Chain: HasChannelIdType<ChainAt<Self, B>>> + HasChainTypeAt<B>
{
    fn channel_id_at(
        &self,
        _index: PhantomData<(A, B)>,
    ) -> &ChannelIdOf<ChainAt<Self, A>, ChainAt<Self, B>>;
}
