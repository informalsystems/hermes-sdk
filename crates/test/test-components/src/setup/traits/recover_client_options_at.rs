use core::marker::PhantomData;

use cgp::core::field::UseField;
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasRecoverClientPayloadType;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_getter {
    name: RecoverClientPayloadOptionsGetterAtComponent<A>,
    provider: RecoverClientPayloadOptionsGetterAt,
}]
pub trait HasRecoverClientPayloadOptionsAt<A>: HasChainTypeAt<A>
where
    ChainAt<Self, A>: HasRecoverClientPayloadType,
{
    fn recover_client_payload_options(
        &self,
        _index: PhantomData<A>,
    ) -> &<ChainAt<Self, A> as HasRecoverClientPayloadType>::RecoverClientPayload;
}
