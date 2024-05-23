pub type TendermintClientState =
    ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState;
pub type ProtoTendermintClientState = ibc_proto::ibc::lightclients::tendermint::v1::ClientState;

pub type TendermintConsensusState =
    ibc_relayer_types::clients::ics07_tendermint::consensus_state::ConsensusState;
pub type ProtoTendermintConsensusState =
    ibc_proto::ibc::lightclients::tendermint::v1::ConsensusState;

pub type TendermintHeader = ibc_relayer_types::clients::ics07_tendermint::header::Header;
