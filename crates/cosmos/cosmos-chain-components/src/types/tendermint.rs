pub type TendermintClientState = ibc::clients::tendermint::types::ClientState;
pub type ProtoTendermintClientState = ibc_proto::ibc::lightclients::tendermint::v1::ClientState;

pub type TendermintConsensusState = ibc::clients::tendermint::types::ConsensusState;
pub type ProtoTendermintConsensusState =
    ibc_proto::ibc::lightclients::tendermint::v1::ConsensusState;

pub type TendermintHeader = ibc::clients::tendermint::types::Header;
pub type ProtoTendermintHeader = ibc_proto::ibc::lightclients::tendermint::v1::Header;
