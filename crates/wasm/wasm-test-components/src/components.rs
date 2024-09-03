use cgp::prelude::*;

use crate::impls::chain::messages::store_code::BuildStoreCodeMessage;
use crate::impls::chain::upload_client_code::SendStoreCodeProposalMessage;
use crate::traits::chain::messages::store_code::StoreCodeMessageBuilderComponent;
use crate::traits::chain::upload_client_code::WasmClientCodeUploaderComponent;

pub struct WasmChainComponents;

delegate_components! {
    WasmChainComponents {
        StoreCodeMessageBuilderComponent:
            BuildStoreCodeMessage,
        WasmClientCodeUploaderComponent:
            SendStoreCodeProposalMessage,
    }
}
