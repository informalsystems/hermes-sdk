use cgp_core::prelude::*;

use crate::impls::upload_client_code::UploadWasmClientCodeWithChainCommand;
use crate::traits::chain_driver::upload_client_code::WasmClientCodeUploaderComponent;

pub struct WasmChainDriverComponents;

delegate_components! {
    WasmChainDriverComponents {
        WasmClientCodeUploaderComponent:
            UploadWasmClientCodeWithChainCommand,
    }
}
