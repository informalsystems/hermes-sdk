use hermes_core::chain_components::traits::{HasAddressType, HasAmountType};
use hermes_prelude::*;

#[cgp_component {
  provider: WasmContractInstantiator,
  context: Chain,
}]
#[async_trait]
pub trait CanInstantiateWasmContract: HasAddressType + HasAmountType + HasAsyncErrorType {
    async fn instantiate_wasm_contract(
        &self,
        sender: &Self::Address,
        admin: &Self::Address,
        msg: &[u8],
        code_id: u64,
        funds: &Self::Amount,
    ) -> Result<Self::Address, Self::Error>;
}
