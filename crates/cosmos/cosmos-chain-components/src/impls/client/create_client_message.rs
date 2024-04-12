use core::fmt::Display;
use core::marker::PhantomData;

use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::schema::HasSchema;
use hermes_protobuf_encoding_components::types::Protobuf;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_proto::google::protobuf::Any;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::client::create::CosmosCreateClientMessage;
use crate::types::payloads::client::CosmosCreateClientPayload;
use crate::types::tendermint::TendermintClientState;

pub struct BuildCosmosCreateClientMessage;

impl<Chain, Counterparty, Encoding> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCosmosCreateClientMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = CosmosCreateClientPayload>,
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<Protobuf, TendermintClientState>
        + HasSchema<TendermintClientState>,
    Encoding::Schema: Display,
{
    async fn build_create_client_message(
        chain: &Chain,
        payload: CosmosCreateClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let encoding = chain.encoding();

        let client_state_bytes = encoding
            .encode(&payload.client_state)
            .map_err(Chain::raise_error)?;

        let client_state_any = Any {
            type_url: encoding.schema(PhantomData).to_string(),
            value: client_state_bytes,
        };

        let message = CosmosCreateClientMessage {
            client_state: client_state_any,
            consensus_state: payload.consensus_state.into(),
        };

        Ok(message.to_cosmos_message())
    }
}
