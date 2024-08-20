use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

#[derive_component(StoreCodeMessageBuilderComponent, StoreCodeMessageBuilder<Chain>)]
pub trait CanBuildStoreCodeMessage: HasMessageType {
    fn build_store_code_message(
        &self,
        wasm_client_bytes: &Vec<u8>,
        title: &str,
        summary: &str,
    ) -> Self::Message;
}
