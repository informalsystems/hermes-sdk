use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;

#[derive_component(StoreCodeMessageBuilderComponent, StoreCodeMessageBuilder<Chain>)]
pub trait CanBuildStoreCodeMessage: HasAmountType + HasMessageType {
    fn build_store_code_message(
        &self,
        wasm_client_bytes: &Vec<u8>,
        title: &str,
        summary: &str,
        deposit_amount: &Self::Amount,
    ) -> Self::Message;
}
