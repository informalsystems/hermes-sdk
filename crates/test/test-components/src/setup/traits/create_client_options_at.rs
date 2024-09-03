use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(CreateClientOptionsAtComponent, ProvideCreateClientOptionsAt<Setup>)]
pub trait HasCreateClientOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainAt<Self, TARGET>: HasCreateClientPayloadOptionsType<ChainAt<Self, COUNTERPARTY>>
        + HasCreateClientMessageOptionsType<ChainAt<Self, COUNTERPARTY>>,
{
    fn create_client_payload_options(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &CreateClientPayloadOptionsOf<ChainAt<Self, TARGET>, ChainAt<Self, COUNTERPARTY>>;

    fn create_client_message_options(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &CreateClientMessageOptionsOf<ChainAt<Self, TARGET>, ChainAt<Self, COUNTERPARTY>>;
}
