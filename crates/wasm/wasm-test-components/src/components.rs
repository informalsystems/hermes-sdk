use hermes_prelude::*;
use hermes_test_components::chain::traits::{
    WasmContractInstantiatorComponent, WasmContractUploaderComponent,
};
use hermes_wasm_chain_components::impls::{InstantiateWasmContracts, UploadCosmosWasmContract};

use crate::impls::chain::{BuildStoreCodeMessage, SendStoreCodeProposalMessage};
use crate::traits::chain::{StoreCodeMessageBuilderComponent, WasmClientCodeUploaderComponent};

pub struct WasmChainComponents;

delegate_components! {
    WasmChainComponents {
        StoreCodeMessageBuilderComponent:
            BuildStoreCodeMessage,
        WasmClientCodeUploaderComponent:
            SendStoreCodeProposalMessage,
        WasmContractUploaderComponent:
            UploadCosmosWasmContract,
        WasmContractInstantiatorComponent:
            InstantiateWasmContracts,
    }
}
