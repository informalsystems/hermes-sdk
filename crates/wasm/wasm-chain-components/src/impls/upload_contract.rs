use std::num::ParseIntError;
use std::sync::Arc;

use hermes_core::chain_components::traits::{CanSendSingleMessage, HasAddressType, HasMessageType};
use hermes_cosmos_chain_components::impls::{MsgStoreCode, WasmAccessConfig};
use hermes_cosmos_chain_components::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::{AbciEvent, HasWasmAccessType};
use hermes_prelude::*;
use ibc::primitives::proto::Any;
use ibc::primitives::Signer;

use crate::traits::{WasmContractUploader, WasmContractUploaderComponent};

#[derive(Debug)]
pub struct StoreCodeMessage {
    pub wasm_byte_code: Vec<u8>,
    pub sender: String,
    pub instantiate_permission: WasmAccessConfig,
}

pub struct UploadCosmosWasmContract;

#[cgp_provider(WasmContractUploaderComponent)]
impl<Chain> WasmContractUploader<Chain> for UploadCosmosWasmContract
where
    Chain: HasAddressType
        + HasWasmAccessType<WasmAccess = WasmAccessConfig>
        + CanSendSingleMessage<MessageResponse = Vec<Arc<AbciEvent>>>
        + HasMessageType<Message = CosmosMessage>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<ParseIntError>,
{
    async fn upload_wasm_contract(
        chain: &Chain,
        wasm_byte_code: &[u8],
        sender: &Chain::Address,
        access_type: &WasmAccessConfig,
    ) -> Result<u64, Chain::Error> {
        let message = StoreCodeMessage {
            wasm_byte_code: wasm_byte_code.to_vec(),
            sender: sender.to_string(),
            instantiate_permission: access_type.clone(),
        };

        let cosmos_message = message.to_cosmos_message();

        let responses = chain.send_message(cosmos_message).await?;

        let store_code_event = responses
            .iter()
            .find(|event| event.kind == "store_code")
            .ok_or(Chain::raise_error(format!(
                "failed to find `store_code` event in responses `{responses:?}`"
            )))?;

        store_code_event
            .attributes
            .iter()
            .find_map(|attr| {
                let key = attr.key_str().ok()?;
                let value = attr.value_str().ok()?;
                if key == "code_id" {
                    Some(value)
                } else {
                    None
                }
            })
            .ok_or(Chain::raise_error(format!(
                "failed to find `code_id` attribute in event `{store_code_event:?}`"
            )))?
            .parse::<u64>()
            .map_err(Chain::raise_error)
    }
}

impl DynCosmosMessage for StoreCodeMessage {
    fn encode_protobuf(&self, _signer: &Signer) -> Any {
        let store_code_message = MsgStoreCode {
            sender: self.sender.clone(),
            wasm_byte_code: self.wasm_byte_code.clone(),
            instantiate_permission: Some(self.instantiate_permission.clone().into()),
        };

        Any::from_msg(&store_code_message).expect("Failed to convert `MsgStoreCode` to `Any`")
    }
}
