use cgp::prelude::*;
use hermes_ibc_components::types::tags::apps::any::AnyApp;
use hermes_ibc_token_transfer_components::components::chain::IbcTokenTransferChainComponents;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::impls::handlers::incoming::HandleIncomingAnyPayloadData;

define_components! {
    MockIncomingPayloadHandlers {
        AnyApp: HandleIncomingAnyPayloadData,
        [
            IbcTransferApp,
            IbcTransferMintApp,
            IbcTransferUnescrowApp,
        ]:
            IbcTokenTransferChainComponents,
    }
}
