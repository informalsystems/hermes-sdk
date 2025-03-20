use core::marker::PhantomData;

use cgp::core::field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_getter {
    name: CreateClientMessageOptionsGetterAtComponent<A, B>,
    provider: CreateClientMessageOptionsGetterAt,
}]
pub trait HasCreateClientMessageOptionsAt<A, B>: HasChainTypeAt<A> + HasChainTypeAt<B>
where
    ChainAt<Self, A>: HasCreateClientMessageOptionsType<ChainAt<Self, B>>,
{
    fn create_client_message_options(
        &self,
        _index: PhantomData<(A, B)>,
    ) -> &CreateClientMessageOptionsOf<ChainAt<Self, A>, ChainAt<Self, B>>;
}

#[cgp_getter {
    name: CreateClientPayloadOptionsGetterAtComponent<A, B>,
    provider: CreateClientPayloadOptionsGetterAt,
}]
pub trait HasCreateClientPayloadOptionsAt<A, B>: HasChainTypeAt<A> + HasChainTypeAt<B>
where
    ChainAt<Self, A>: HasCreateClientPayloadOptionsType<ChainAt<Self, B>>,
{
    fn create_client_payload_options(
        &self,
        _index: PhantomData<(A, B)>,
    ) -> &CreateClientPayloadOptionsOf<ChainAt<Self, A>, ChainAt<Self, B>>;
}
