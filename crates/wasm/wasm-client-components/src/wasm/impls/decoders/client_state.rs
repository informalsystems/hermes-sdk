use crate::wasm::types::messages::client::state::ProtoClientState;
use crate::wasm::types::messages::client::state::WasmClientState;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoder, HasClientStateType,
};
use ibc_proto::google::protobuf::Any;
use ibc_proto::Protobuf;
use prost::{DecodeError, Message};
use tendermint_proto::Error as ProtoError;

pub struct DecodeWasmClientStateFromAnyProto;

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty>
    for DecodeWasmClientStateFromAnyProto
where
    Chain: HasClientStateType<Counterparty, ClientState = WasmClientState>,
    Counterparty: CanRaiseError<DecodeError>,
    DecodeWasmClientStateFromProto: ClientStateDecoder<Chain, Counterparty>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<WasmClientState, Counterparty::Error> {
        let any = Any::decode(client_state_bytes).map_err(Counterparty::raise_error)?;

        let client_state = DecodeWasmClientStateFromProto::decode_client_state_bytes(&any.value)?;

        Ok(client_state)
    }
}

pub struct DecodeWasmClientStateFromProto;

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty> for DecodeWasmClientStateFromProto
where
    Chain: HasClientStateType<Counterparty, ClientState = WasmClientState>,
    Counterparty: CanRaiseError<ProtoError>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<WasmClientState, Counterparty::Error> {
        let client_state = Protobuf::<ProtoClientState>::decode_vec(client_state_bytes)
            .map_err(Counterparty::raise_error)?;

        Ok(client_state)
    }
}
