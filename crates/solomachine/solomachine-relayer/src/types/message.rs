use hermes_cosmos_chain_components::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use hermes_cosmos_chain_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use hermes_cosmos_chain_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};

#[derive(Debug)]
pub enum SolomachineMessage {
    CosmosCreateClient(Box<CosmosCreateClientPayload>),
    CosmosUpdateClient(Box<CosmosUpdateClientPayload>),
    CosmosChannelOpenTry(Box<CosmosChannelOpenTryPayload>),
    CosmosChannelOpenAck(Box<CosmosChannelOpenAckPayload>),
    CosmosChannelOpenConfirm(Box<CosmosChannelOpenConfirmPayload>),
    CosmosConnectionOpenInit(Box<CosmosConnectionOpenInitPayload>),
    CosmosConnectionOpenTry(Box<CosmosConnectionOpenTryPayload>),
    CosmosConnectionOpenAck(Box<CosmosConnectionOpenAckPayload>),
    CosmosConnectionOpenConfirm(Box<CosmosConnectionOpenConfirmPayload>),
}
