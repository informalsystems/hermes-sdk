use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;

#[cgp_component {
  name: StoreCodeMessageBuilderComponent,
  provider: StoreCodeMessageBuilder,
  context: Chain,
}]
pub trait CanBuildStoreCodeMessage: HasAmountType + HasAddressType + HasMessageType {
    fn build_store_code_message(
        &self,
        wasm_client_bytes: &Vec<u8>,
        title: &str,
        summary: &str,
        authority: &Self::Address,
        deposit_amount: &Self::Amount,
    ) -> Self::Message;
}
