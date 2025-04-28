use hermes_prelude::*;

use crate::impls::chain::{BuildStoreCodeMessage, SendStoreCodeProposalMessage};
use crate::traits::chain::{StoreCodeMessageBuilderComponent, WasmClientCodeUploaderComponent};

pub struct WasmChainComponents;

delegate_components! {
    WasmChainComponents {
        StoreCodeMessageBuilderComponent:
            BuildStoreCodeMessage,
        WasmClientCodeUploaderComponent:
            SendStoreCodeProposalMessage,
    }
}
