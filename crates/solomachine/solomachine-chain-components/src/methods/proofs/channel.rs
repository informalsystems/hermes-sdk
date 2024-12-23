use hermes_cosmos_chain_components::methods::encode::encode_protobuf;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::host::types::identifiers::ChannelId;
use ibc_proto::ibc::core::channel::v1::Channel as ProtoChannelEnd;

use crate::types::client_state::SolomachineClientState;
use crate::types::sign_data::SolomachineSignData;

// Create a sign data for the connection proof that the solomachine has
// the connection end of the counterparty Cosmos chain
pub fn channel_proof_data(
    client_state: &SolomachineClientState,
    commitment_prefix: &str,
    channel_id: &ChannelId,
    channel_end: ChannelEnd,
) -> SolomachineSignData {
    let proto_channel_end: ProtoChannelEnd = channel_end.into();

    let channel_end_bytes = encode_protobuf(&proto_channel_end);

    let path = format!("{commitment_prefix}channel/{channel_id}");

    let sign_data = SolomachineSignData {
        diversifier: client_state.consensus_state.diversifier.clone(),
        sequence: client_state.sequence,
        timestamp: client_state.consensus_state.timestamp,
        path: path.into(),
        data: channel_end_bytes,
    };

    sign_data
}
