use cgp::prelude::*;
use hermes_ibc_components::types::tags::apps::any::AnyApp;
use hermes_ibc_token_transfer_components::components::chain::IbcTokenTransferChainComponents;
use hermes_ibc_token_transfer_components::types::tags::{
    IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp,
};

use crate::types::packet_data::UseMockAnyPayloadData;

pub struct MockPayloadDataTypes;

delegate_components! {
    MockPayloadDataTypes {
        AnyApp: UseMockAnyPayloadData,
        [
            IbcTransferApp,
            IbcTransferMintApp,
            IbcTransferUnescrowApp,
        ]:
            IbcTokenTransferChainComponents,
    }
}
