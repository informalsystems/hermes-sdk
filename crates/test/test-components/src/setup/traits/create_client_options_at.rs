use core::marker::PhantomData;

use cgp::core::field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
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

#[cgp_component {
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

#[cgp_provider(CreateClientMessageOptionsGetterAtComponent<A, B>)]
impl<Setup, Tag, A, B, ChainA, ChainB> CreateClientMessageOptionsGetterAt<Setup, A, B>
    for UseField<Tag>
where
    Setup: HasChainTypeAt<A, Chain = ChainA>
        + HasChainTypeAt<B, Chain = ChainB>
        + HasField<Tag, Value = ChainA::CreateClientMessageOptions>,
    ChainA: HasCreateClientPayloadOptionsType<ChainB> + HasCreateClientMessageOptionsType<ChainB>,
{
    fn create_client_message_options(
        setup: &Setup,
        _index: PhantomData<(A, B)>,
    ) -> &ChainA::CreateClientMessageOptions {
        setup.get_field(PhantomData)
    }
}

#[cgp_provider(CreateClientPayloadOptionsGetterAtComponent<A, B>)]
impl<Setup, Tag, A, B, ChainA, ChainB> CreateClientPayloadOptionsGetterAt<Setup, A, B>
    for UseField<Tag>
where
    Setup: HasChainTypeAt<A, Chain = ChainA>
        + HasChainTypeAt<B, Chain = ChainB>
        + HasField<Tag, Value = ChainA::CreateClientPayloadOptions>,
    ChainA: HasCreateClientPayloadOptionsType<ChainB> + HasCreateClientMessageOptionsType<ChainB>,
{
    fn create_client_payload_options(
        setup: &Setup,
        _index: PhantomData<(A, B)>,
    ) -> &ChainA::CreateClientPayloadOptions {
        setup.get_field(PhantomData)
    }
}
