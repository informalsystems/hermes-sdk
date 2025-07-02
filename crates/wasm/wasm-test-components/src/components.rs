use hermes_prelude::*;
use hermes_test_components::chain::traits::WasmContractUploaderComponent;
use hermes_wasm_chain_components::impls::upload_contract::UploadWasmContractsFromEnv;

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
            UploadWasmContractsFromEnv,
    }
}
