use std::path::PathBuf;

use hermes_core::chain_components::traits::{CanSendSingleMessage, HasAddressType, HasMessageType};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelDebug;
use hermes_core::test_components::chain::traits::{
    WasmContractUploader, WasmContractUploaderComponent,
};
use hermes_cosmos_chain_components::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};
use hermes_prelude::*;
use ibc::primitives::proto::Any;
use ibc::primitives::Signer;
use prost::Message;
use sha2::{Digest, Sha256};

use crate::protos::cosmwasm::MsgStoreCode;

#[derive(Debug)]
pub enum AccessType {
    Unspecified = 0,
    Nobody = 1,
    Everybody = 3,
    AnyOfAddresses = 4,
}

#[derive(Debug)]
pub struct StoreCodeMessage {
    pub wasm_byte_code: Vec<u8>,
    pub sender: String,
    pub instantiate_permission: AccessType,
}

pub struct UploadWasmContractsFromEnv;

#[cgp_provider(WasmContractUploaderComponent)]
impl<Chain> WasmContractUploader<Chain> for UploadWasmContractsFromEnv
where
    Chain: HasAddressType
        + CanSendSingleMessage
        + HasMessageType<Message = CosmosMessage>
        + CanLog<LevelDebug>
        + CanRaiseAsyncError<String>,
{
    async fn upload_wasm_contract(
        chain: &Chain,
        _wasm_byte_code: &[u8],
        sender: &Chain::Address,
    ) -> Result<(), Chain::Error> {
        let env_wasm = std::env::var("WASM_FILES").expect("Wasm file is required");

        let raw_wasm_code_paths: Vec<&str> = env_wasm.split(',').collect();

        for raw_wasm_code_path in raw_wasm_code_paths.iter() {
            chain
                .log(
                    &format!("Will upload Wasm contract: {raw_wasm_code_path}"),
                    &LevelDebug,
                )
                .await;

            let wasm_code_path = PathBuf::from(raw_wasm_code_path);
            let wasm_byte_code = tokio::fs::read(&wasm_code_path).await.map_err(|e| {
                Chain::raise_error(format!("failed to read `{wasm_code_path:?}`: {e}"))
            })?;

            let wasm_code_hash: [u8; 32] = {
                let mut hasher = Sha256::new();
                hasher.update(&wasm_byte_code);
                hasher.finalize().into()
            };

            let message = StoreCodeMessage {
                wasm_byte_code: wasm_code_hash.to_vec(),
                sender: sender.to_string(),
                instantiate_permission: AccessType::Everybody,
            };

            let cosmos_message = message.to_cosmos_message();

            let response = chain.send_message(cosmos_message).await?;

            chain
                .log(
                    &format!("Response from uploading Wasm code: {response:?}"),
                    &LevelDebug,
                )
                .await;
        }

        Ok(())
    }
}
pub struct UploadCosmosWasmContract;

#[cgp_provider(WasmContractUploaderComponent)]
impl<Chain> WasmContractUploader<Chain> for UploadCosmosWasmContract
where
    Chain: HasAddressType
        + CanSendSingleMessage
        + HasMessageType<Message = CosmosMessage>
        + CanLog<LevelDebug>
        + CanRaiseAsyncError<String>,
{
    async fn upload_wasm_contract(
        chain: &Chain,
        wasm_byte_code: &[u8],
        sender: &Chain::Address,
    ) -> Result<(), Chain::Error> {
        let message = StoreCodeMessage {
            wasm_byte_code: wasm_byte_code.to_vec(),
            sender: sender.to_string(),
            instantiate_permission: AccessType::Everybody,
        };

        let cosmos_message = message.to_cosmos_message();

        let response = chain.send_message(cosmos_message).await?;

        chain
            .log(
                &format!("Response from uploading Wasm code: {response:?}"),
                &LevelDebug,
            )
            .await;

        Ok(())
    }
}

impl DynCosmosMessage for StoreCodeMessage {
    fn encode_protobuf(&self, _signer: &Signer) -> Any {
        let store_code_message = MsgStoreCode {
            sender: self.sender.clone(),
            wasm_byte_code: self.wasm_byte_code.clone(),
            instantiate_permission: None,
        };

        Any {
            type_url: "/cosmwasm.wasm.v1.MsgStoreCode".into(),
            value: store_code_message.encode_to_vec(),
        }
    }
}
