use crate::contexts::wasm_counterparty::WasmCounterparty;
use crate::types::client_state::ProtoClientState;
use crate::types::client_state::WasmClientState;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoder, HasClientStateType,
};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::sovereign::tendermint::v1::ClientState as ProtoSovereignClientState;
use ibc_proto::Protobuf;
use prost::{DecodeError, Message};
use sov_celestia_client::types::client_state::SovTmClientState;
use tendermint_proto::Error as ProtoError;

pub struct DecodeSovereignClientStateFromAnyProto;

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty>
    for DecodeSovereignClientStateFromAnyProto
where
    Chain: HasClientStateType<Counterparty, ClientState = SovTmClientState>,
    Counterparty: CanRaiseError<DecodeError>,
    DecodeWasmClientStateFromProto: ClientStateDecoder<WasmCounterparty, Counterparty>,
    DecodeSovereignClientStateFromProto: ClientStateDecoder<Chain, Counterparty>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<SovTmClientState, Counterparty::Error> {
        let any = Any::decode(client_state_bytes).map_err(Counterparty::raise_error)?;

        let wasm_client_state: WasmClientState =
            DecodeWasmClientStateFromProto::decode_client_state_bytes(&any.value)?;

        let raw_sovereign = wasm_client_state.data;

        let any_sovereign =
            Any::decode(raw_sovereign.as_slice()).map_err(Counterparty::raise_error)?;

        let sovereign_client_state =
            DecodeSovereignClientStateFromProto::decode_client_state_bytes(&any_sovereign.value)?;

        Ok(sovereign_client_state)
    }
}

pub struct DecodeSovereignClientStateFromProto;

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty>
    for DecodeSovereignClientStateFromProto
where
    Chain: HasClientStateType<Counterparty, ClientState = SovTmClientState>,
    Counterparty: CanRaiseError<ProtoError>,
{
    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<SovTmClientState, Counterparty::Error> {
        let client_state = Protobuf::<ProtoSovereignClientState>::decode_vec(client_state_bytes)
            .map_err(Counterparty::raise_error)?;

        Ok(client_state)
    }
}

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
