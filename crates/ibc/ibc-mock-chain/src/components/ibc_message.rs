use cgp::prelude::*;
use hermes_ibc_token_transfer_components::components::chain::IbcTokenTransferChainComponents;
use hermes_ibc_token_transfer_components::types::tags::IbcTransferApp;

cgp_preset! {
    MockIbcMessageTypes {
        IbcTransferApp: IbcTokenTransferChainComponents,
    }
}
