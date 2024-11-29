use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(CreateClientMessageOptionsAtComponent, ProvideCreateClientMessageOptionsAt<Setup>)]
pub trait HasCreateClientMessageOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainAt<Self, TARGET>: HasCreateClientMessageOptionsType<ChainAt<Self, COUNTERPARTY>>,
{
    fn create_client_message_options(
        &self,
        _index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &CreateClientMessageOptionsOf<ChainAt<Self, TARGET>, ChainAt<Self, COUNTERPARTY>>;
}

#[derive_component(CreateClientPayloadOptionsAtComponent, ProvideCreateClientPayloadOptionsAt<Setup>)]
pub trait HasCreateClientPayloadOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainAt<Self, TARGET>: HasCreateClientPayloadOptionsType<ChainAt<Self, COUNTERPARTY>>,
{
    fn create_client_payload_options(
        &self,
        _index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &CreateClientPayloadOptionsOf<ChainAt<Self, TARGET>, ChainAt<Self, COUNTERPARTY>>;
}

impl<Setup, Tag, const TARGET: usize, const COUNTERPARTY: usize, ChainA, ChainB>
    ProvideCreateClientMessageOptionsAt<Setup, TARGET, COUNTERPARTY> for UseField<Tag>
where
    Setup: HasChainTypeAt<TARGET, Chain = ChainA>
        + HasChainTypeAt<COUNTERPARTY, Chain = ChainB>
        + HasField<Tag, Field = ChainA::CreateClientMessageOptions>,
    ChainA: HasCreateClientPayloadOptionsType<ChainB> + HasCreateClientMessageOptionsType<ChainB>,
{
    fn create_client_message_options(
        setup: &Setup,
        _index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &ChainA::CreateClientMessageOptions {
        setup.get_field(PhantomData)
    }
}

impl<Setup, Tag, const TARGET: usize, const COUNTERPARTY: usize, ChainA, ChainB>
    ProvideCreateClientPayloadOptionsAt<Setup, TARGET, COUNTERPARTY> for UseField<Tag>
where
    Setup: HasChainTypeAt<TARGET, Chain = ChainA>
        + HasChainTypeAt<COUNTERPARTY, Chain = ChainB>
        + HasField<Tag, Field = ChainA::CreateClientPayloadOptions>,
    ChainA: HasCreateClientPayloadOptionsType<ChainB> + HasCreateClientMessageOptionsType<ChainB>,
{
    fn create_client_payload_options(
        setup: &Setup,
        _index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &ChainA::CreateClientPayloadOptions {
        setup.get_field(PhantomData)
    }
}
