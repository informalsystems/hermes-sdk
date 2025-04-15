use cgp::prelude::*;
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_core::relayer_components::chain::traits::HasMessageType;

#[cgp_component {
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
