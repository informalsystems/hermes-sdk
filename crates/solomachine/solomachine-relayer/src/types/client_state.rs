use eyre::eyre;
use hermes_cosmos_relayer::types::error::Error;
use ibc_proto::google::protobuf::Any;
use ibc_proto::Protobuf;
use ibc_relayer_types::keys::ROUTER_KEY;
use ibc_relayer_types::tx_msg::Msg;
use prost::Message;

use crate::types::consensus_state::SolomachineConsensusState;

pub use ibc_proto::ibc::lightclients::solomachine::v3::ClientState as ProtoSolomachineClientState;

const TYPE_URL: &str = "/ibc.lightclients.solomachine.v3.ClientState";

#[derive(Clone, Debug)]
pub struct SolomachineClientState {
    pub sequence: u64,
    pub is_frozen: bool,
    pub consensus_state: SolomachineConsensusState,
}

impl TryFrom<Any> for SolomachineClientState {
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Error> {
        use core::ops::Deref;

        use bytes::Buf;

        fn decode_client_state<B: Buf>(buf: B) -> Result<SolomachineClientState, Error> {
            ProtoSolomachineClientState::decode(buf)
                .map_err(|e| eyre!("error decoding client state: {e}"))?
                .try_into()
        }

        match raw.type_url.as_str() {
            TYPE_URL => decode_client_state(raw.value.deref()).map_err(Into::into),
            _ => Err(eyre!("unknown client state: {}", raw.type_url).into()),
        }
    }
}

impl Msg for SolomachineClientState {
    type ValidationError = Error;
    type Raw = ProtoSolomachineClientState;

    fn route(&self) -> String {
        ROUTER_KEY.to_string()
    }

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<ProtoSolomachineClientState> for SolomachineClientState {}

impl TryFrom<ProtoSolomachineClientState> for SolomachineClientState {
    type Error = Error;

    fn try_from(value: ProtoSolomachineClientState) -> Result<Self, Self::Error> {
        let consensus_state = value.consensus_state.unwrap().try_into().unwrap();

        Ok(SolomachineClientState {
            sequence: value.sequence,
            is_frozen: value.is_frozen,
            consensus_state,
        })
    }
}

impl From<SolomachineClientState> for ProtoSolomachineClientState {
    fn from(value: SolomachineClientState) -> Self {
        ProtoSolomachineClientState {
            sequence: value.sequence,
            is_frozen: value.is_frozen,
            consensus_state: Some(value.consensus_state.into()),
            //allow_update_after_proposal: true,
        }
    }
}
