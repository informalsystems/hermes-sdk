use hermes_chain_type_components::traits::{HasAddressType, HasDenomType};
use hermes_prelude::*;

#[cgp_component {
  provider: WasmContractInstantiator,
  context: Chain,
}]
#[async_trait]
pub trait CanInstantiateWasmContract: HasAddressType + HasDenomType + HasAsyncErrorType {
    async fn instantiate_wasm_contract(
        &self,
        sender: &Self::Address,
        admin: &Self::Address,
        msg: &[u8],
        code_id: u64,
        funds: &Self::Denom,
    ) -> Result<Self::Address, Self::Error>;
}
