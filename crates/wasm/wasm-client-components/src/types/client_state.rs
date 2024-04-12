use eyre::{eyre, Error};
use ibc::core::client::types::Height;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height as ProtoHeight;
use prost::EncodeError;

use crate::utils::encode::encode_to_any;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoWasmClientState {
    /// bytes encoding the client state of the underlying light client
    /// implemented as a Wasm contract.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub latest_height: ::core::option::Option<ProtoHeight>,
}

const TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientState";

#[derive(Clone, Debug)]
pub struct WasmClientState {
    pub data: Vec<u8>,
    pub checksum: Vec<u8>,
    pub latest_height: Height,
}

impl WasmClientState {
    pub fn encode_protobuf(&self) -> Result<Any, EncodeError> {
        let latest_height = ProtoHeight {
            revision_number: self.latest_height.revision_number(),
            revision_height: self.latest_height.revision_height(),
        };
        let proto_message = ProtoWasmClientState {
            data: self.data.clone(),
            checksum: self.checksum.clone(),
            latest_height: Some(latest_height),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}

impl TryFrom<ProtoWasmClientState> for WasmClientState {
    type Error = Error;

    fn try_from(value: ProtoWasmClientState) -> Result<Self, Self::Error> {
        let maybe_height = value
            .latest_height
            .ok_or_else(|| eyre!("Empty 'latest_height' in proto Wasm client state"))?;
        let height = Height::try_from(maybe_height)?;
        Ok(Self {
            data: value.data,
            checksum: value.checksum,
            latest_height: height,
        })
    }
}

impl From<WasmClientState> for ProtoWasmClientState {
    fn from(value: WasmClientState) -> Self {
        let height = ProtoHeight::from(value.latest_height);
        Self {
            data: value.data,
            checksum: value.checksum,
            latest_height: Some(height),
        }
    }
}
