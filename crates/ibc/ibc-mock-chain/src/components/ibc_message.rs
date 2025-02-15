use cgp::prelude::*;
use hermes_ibc_token_transfer_components::components::chain::IbcTokenTransferChainComponents;
use hermes_ibc_token_transfer_components::types::tags::IbcTransferApp;

pub struct MockIbcMessageTypes;

delegate_components! {
    MockIbcMessageTypes {
        IbcTransferApp: IbcTokenTransferChainComponents,
    }
}
