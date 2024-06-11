use cgp_core::CanRaiseError;
use hermes_cosmos_chain_components::methods::encode::encode_to_any;
use hermes_cosmos_chain_components::types::messages::client::create::{
    ProtoMsgCreateClient, TYPE_URL,
};
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientPayload;
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_protobuf_encoding_components::types::Any;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_relayer_types::signer::Signer;

use crate::types::message::SovereignMessage;
use crate::types::messages::ibc::IbcMessageWithHeight;

/**
   Build a message to create a Cosmos client on a Sovereign rollup
*/
pub struct BuildCreateCosmosClientMessageOnSovereign;

impl<Chain, Counterparty, Encoding> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateCosmosClientMessageOnSovereign
where
    Chain: HasMessageType<Message = SovereignMessage>
        + HasCreateClientMessageOptionsType<Counterparty>
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = CosmosCreateClientPayload>,
    Encoding: CanConvert<TendermintClientState, Any> + CanConvert<TendermintConsensusState, Any>,
{
    async fn build_create_client_message(
        chain: &Chain,
        _options: &Chain::CreateClientMessageOptions,
        payload: CosmosCreateClientPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        let encoding = chain.encoding();

        let client_state = encoding
            .convert(&payload.client_state)
            .map_err(Chain::raise_error)?;

        let consensus_state = encoding
            .convert(&payload.consensus_state)
            .map_err(Chain::raise_error)?;

        let proto_message = ProtoMsgCreateClient {
            client_state: Some(client_state),
            consensus_state: Some(consensus_state),
            signer: Signer::dummy().to_string(),
        };

        let any_message = encode_to_any(TYPE_URL, &proto_message);

        let message = IbcMessageWithHeight::new(any_message, None).into();

        Ok(message)
    }
}
