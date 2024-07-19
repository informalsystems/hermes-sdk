use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainTypeAt, HasChainTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(CreateClientOptionsAtComponent, ProvideCreateClientOptionsAt<Setup>)]
pub trait HasCreateClientOptionsAt<const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET> + HasChainTypeAt<COUNTERPARTY>
where
    ChainTypeAt<Self, TARGET>: HasCreateClientPayloadOptionsType<ChainTypeAt<Self, COUNTERPARTY>>
        + HasCreateClientMessageOptionsType<ChainTypeAt<Self, COUNTERPARTY>>,
{
    fn create_client_payload_options(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &CreateClientPayloadOptionsOf<ChainTypeAt<Self, TARGET>, ChainTypeAt<Self, COUNTERPARTY>>;

    fn create_client_message_options(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &CreateClientMessageOptionsOf<ChainTypeAt<Self, TARGET>, ChainTypeAt<Self, COUNTERPARTY>>;
}
