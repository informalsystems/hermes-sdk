use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[derive_component(CreateClientMessageOptionsAtComponent, ProvideCreateClientMessageOptionsAt<Setup>)]
pub trait HasCreateClientMessageOptionsAt<Target: Async, Counterparty: Async>:
    HasChainTypeAt<Target> + HasChainTypeAt<Counterparty>
where
    ChainAt<Self, Target>: HasCreateClientMessageOptionsType<ChainAt<Self, Counterparty>>,
{
    fn create_client_message_options(
        &self,
        _index: PhantomData<(Target, Counterparty)>,
    ) -> &CreateClientMessageOptionsOf<ChainAt<Self, Target>, ChainAt<Self, Counterparty>>;
}

#[derive_component(CreateClientPayloadOptionsAtComponent, ProvideCreateClientPayloadOptionsAt<Setup>)]
pub trait HasCreateClientPayloadOptionsAt<Target: Async, Counterparty: Async>:
    HasChainTypeAt<Target> + HasChainTypeAt<Counterparty>
where
    ChainAt<Self, Target>: HasCreateClientPayloadOptionsType<ChainAt<Self, Counterparty>>,
{
    fn create_client_payload_options(
        &self,
        _index: PhantomData<(Target, Counterparty)>,
    ) -> &CreateClientPayloadOptionsOf<ChainAt<Self, Target>, ChainAt<Self, Counterparty>>;
}

impl<Setup, Tag, Target: Async, Counterparty: Async, ChainA, ChainB>
    ProvideCreateClientMessageOptionsAt<Setup, Target, Counterparty> for UseField<Tag>
where
    Setup: HasChainTypeAt<Target, Chain = ChainA>
        + HasChainTypeAt<Counterparty, Chain = ChainB>
        + HasField<Tag, Field = ChainA::CreateClientMessageOptions>,
    ChainA: HasCreateClientPayloadOptionsType<ChainB> + HasCreateClientMessageOptionsType<ChainB>,
{
    fn create_client_message_options(
        setup: &Setup,
        _index: PhantomData<(Target, Counterparty)>,
    ) -> &ChainA::CreateClientMessageOptions {
        setup.get_field(PhantomData)
    }
}

impl<Setup, Tag, Target: Async, Counterparty: Async, ChainA, ChainB>
    ProvideCreateClientPayloadOptionsAt<Setup, Target, Counterparty> for UseField<Tag>
where
    Setup: HasChainTypeAt<Target, Chain = ChainA>
        + HasChainTypeAt<Counterparty, Chain = ChainB>
        + HasField<Tag, Field = ChainA::CreateClientPayloadOptions>,
    ChainA: HasCreateClientPayloadOptionsType<ChainB> + HasCreateClientMessageOptionsType<ChainB>,
{
    fn create_client_payload_options(
        setup: &Setup,
        _index: PhantomData<(Target, Counterparty)>,
    ) -> &ChainA::CreateClientPayloadOptions {
        setup.get_field(PhantomData)
    }
}
