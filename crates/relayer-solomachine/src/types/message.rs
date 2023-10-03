use ibc_relayer_cosmos::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use ibc_relayer_cosmos::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use ibc_relayer_cosmos::types::payloads::connection::{
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
