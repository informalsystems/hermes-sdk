use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptions, CreateClientPayloadOptions, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::types::index::Twindex;

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
    ) -> &CreateClientPayloadOptions<ChainTypeAt<Self, TARGET>, ChainTypeAt<Self, COUNTERPARTY>>;

    fn create_client_message_options(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
    ) -> &CreateClientMessageOptions<ChainTypeAt<Self, TARGET>, ChainTypeAt<Self, COUNTERPARTY>>;
}
