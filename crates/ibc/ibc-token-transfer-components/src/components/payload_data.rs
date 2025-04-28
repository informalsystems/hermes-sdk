use hermes_prelude::*;

use crate::types::packet_data::mint::UseIbcTransferMintPayloadData;
use crate::types::packet_data::transfer::UseIbcTransferPayloadData;
use crate::types::packet_data::unescrow::UseIbcTransferUnescrowPayloadData;
use crate::types::tags::{IbcTransferApp, IbcTransferMintApp, IbcTransferUnescrowApp};

pub struct IbcTokenTransferPayloadDataTypes;

delegate_components! {
    IbcTokenTransferPayloadDataTypes {
        IbcTransferApp: UseIbcTransferPayloadData,
        IbcTransferMintApp: UseIbcTransferMintPayloadData,
        IbcTransferUnescrowApp: UseIbcTransferUnescrowPayloadData,
    }
}
