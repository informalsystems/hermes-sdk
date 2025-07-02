use hermes_core::chain_components::traits::{
    CanSendSingleMessage, HasAddressType, HasDenomType, HasMessageType,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelDebug;
use hermes_core::test_components::chain::traits::{
    WasmContractInstantiator, WasmContractInstantiatorComponent,
};
use hermes_cosmos_chain_components::traits::{CosmosMessage, DynCosmosMessage, ToCosmosMessage};
use hermes_prelude::*;
use ibc::primitives::proto::Any;
use ibc::primitives::Signer;
use ibc_proto::cosmos::base::v1beta1::Coin;
use prost::Message;

use crate::protos::cosmwasm::MsgInstantiateContract;

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
    Chain: HasAddressType
        + HasDenomType
        + CanSendSingleMessage
        + HasMessageType<Message = CosmosMessage>
        + CanLog<LevelDebug>
        + CanRaiseAsyncError<String>,
{
    async fn instantiate_wasm_contract(
        chain: &Chain,
        sender: &Chain::Address,
        admin: &Chain::Address,
        code_id: u64,
        funds_denom: &Chain::Denom,
    ) -> Result<(), Chain::Error> {
        chain
            .log(
                &format!("Will instantiate Wasm contract: {code_id}"),
                &LevelDebug,
            )
            .await;

        let fund = Coin {
            denom: funds_denom.to_string(),
            amount: "1000000".to_string(),
        };

        let message = InstantiateMessage {
            sender: sender.to_string(),
            admin: admin.to_string(),
            code_id,
            label: "Instantiate Cosm Contract".to_string(),
            msg: vec![],
            funds: vec![fund],
        };

        let cosmos_message = message.to_cosmos_message();

        let response = chain.send_message(cosmos_message).await?;

        chain
            .log(
                &format!("Response from instantiating Wasm code: {response:?}"),
                &LevelDebug,
            )
            .await;

        Ok(())
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
