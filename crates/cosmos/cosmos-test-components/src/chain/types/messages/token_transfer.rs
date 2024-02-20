use hermes_cosmos_client_components::methods::encode::encode_to_any;
use hermes_cosmos_client_components::traits::message::DynCosmosMessage;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::apps::transfer::v1::MsgTransfer;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::timestamp::Timestamp;
use ibc_relayer_types::Height;
use prost::EncodeError;

use crate::chain::types::amount::Amount;

#[derive(Debug)]
pub struct TokenTransferMessage {
    pub channel_id: ChannelId,
    pub port_id: PortId,
    pub recipient_address: String,
    pub amount: Amount,
    pub memo: Option<String>,
    pub timeout_height: Option<Height>,
    pub timeout_time: Option<Timestamp>,
}

impl DynCosmosMessage for TokenTransferMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Result<Any, EncodeError> {
        let timeout_timestamp = self.timeout_time.unwrap_or_default().nanoseconds();

        let message = MsgTransfer {
            source_port: self.port_id.to_string(),
            source_channel: self.channel_id.to_string(),
            token: Some(Coin {
                denom: self.amount.denom.to_string(),
                amount: self.amount.quantity.to_string(),
            }),
            sender: signer.to_string(),
            receiver: self.recipient_address.clone(),
            timeout_height: self.timeout_height.map(Into::into),
            timeout_timestamp,
            memo: self.memo.clone().unwrap_or_default(),
        };

        encode_to_any("/ibc.applications.transfer.v1.MsgTransfer", &message)
    }
}
