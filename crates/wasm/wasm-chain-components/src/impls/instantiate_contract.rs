use std::sync::Arc;

use hermes_core::chain_components::traits::{
    CanSendSingleMessage, HasAddressType, HasDenomType, HasMessageType,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelDebug;
use hermes_core::test_components::chain::traits::{
    WasmContractInstantiator, WasmContractInstantiatorComponent,
};
use hermes_cosmos_chain_components::impls::MsgInstantiateContract;
use hermes_cosmos_chain_components::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::AbciEvent;
use hermes_prelude::*;
use ibc::primitives::proto::Any;
use ibc::primitives::Signer;
use ibc_proto::cosmos::base::v1beta1::Coin;
use prost::Message;

#[derive(Debug)]
pub struct InstantiateMessage {
    pub sender: String,
    pub admin: String,
    pub code_id: u64,
    pub label: String,
    pub msg: Vec<u8>,
    pub funds: Vec<Coin>,
}

pub struct InstantiateWasmContracts;

#[cgp_provider(WasmContractInstantiatorComponent)]
impl<Chain> WasmContractInstantiator<Chain> for InstantiateWasmContracts
where
    Chain: HasAddressType<Address = String>
        + HasDenomType
        + CanSendSingleMessage<MessageResponse = Vec<Arc<AbciEvent>>>
        + HasMessageType<Message = CosmosMessage>
        + CanLog<LevelDebug>
        + CanRaiseAsyncError<String>,
{
    async fn instantiate_wasm_contract(
        chain: &Chain,
        sender: &Chain::Address,
        admin: &Chain::Address,
        msg: &[u8],
        code_id: u64,
        funds_denom: &Chain::Denom,
    ) -> Result<Chain::Address, Chain::Error> {
        // TODO: Use meaningful value instead of hardcoded value
        let fund = Coin {
            denom: funds_denom.to_string(),
            amount: "1000000".to_string(),
        };

        let message = InstantiateMessage {
            sender: sender.to_string(),
            admin: admin.to_string(),
            code_id,
            label: format!("Instantiate Cosm Contract with code `{code_id}`"),
            msg: msg.to_vec(),
            funds: vec![fund],
        };

        let cosmos_message = message.to_cosmos_message();

        let responses = chain.send_message(cosmos_message).await?;

        let instantiate_event = responses
            .iter()
            .find(|event| event.kind == "instantiate")
            .ok_or(Chain::raise_error(format!(
                "failed to find `instantiate` event in responses `{responses:?}`"
            )))?;

        let contract_address = instantiate_event
            .attributes
            .iter()
            .find_map(|attr| {
                let key = attr.key_str().ok()?;
                let value = attr.value_str().ok()?;
                if key == "_contract_address" {
                    Some(value)
                } else {
                    None
                }
            })
            .ok_or(Chain::raise_error(format!(
                "failed to find `_contract_address` attribute in event `{instantiate_event:?}`"
            )))?;

        Ok(contract_address.to_string())
    }
}

impl DynCosmosMessage for InstantiateMessage {
    fn encode_protobuf(&self, _signer: &Signer) -> Any {
        let instantiate_message = MsgInstantiateContract {
            sender: self.sender.clone(),
            admin: self.admin.clone(),
            code_id: self.code_id,
            label: self.label.clone(),
            msg: self.msg.clone(),
            funds: self.funds.clone(),
        };

        Any {
            type_url: "/cosmwasm.wasm.v1.MsgInstantiateContract".into(),
            value: instantiate_message.encode_to_vec(),
        }
    }
}
