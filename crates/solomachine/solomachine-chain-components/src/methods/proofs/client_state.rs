use hermes_cosmos_chain_components::methods::encode_protobuf;
use hermes_cosmos_chain_components::types::TendermintClientState;
use ibc::core::host::types::identifiers::ClientId;
use ibc_proto::cosmos::tx::signing::v1beta1::signature_descriptor::data::{Single, Sum};
use ibc_proto::cosmos::tx::signing::v1beta1::signature_descriptor::Data;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoClientState;
use secp256k1::SecretKey;

use crate::methods::encode::public_key::PublicKey;
use crate::methods::encode::sign_data::sign_with_data;
use crate::types::client_state::SolomachineClientState;
use crate::types::sign_data::{SolomachineSignData, SolomachineTimestampedSignData};

// Create a sign data for the client state proof that the solomachine has
// the Tendermint client state of the counterparty Cosmos chain
pub fn client_state_proof_data(
    _public_key: &PublicKey,
    secret_key: &SecretKey,
    solo_client_state: &SolomachineClientState,
    commitment_prefix: &str,
    client_id: &ClientId,
    cosmos_client_state: &TendermintClientState,
) -> SolomachineTimestampedSignData {
    let proto_client_state: ProtoClientState = cosmos_client_state.clone().into();

    let client_state_bytes = encode_protobuf(&proto_client_state);

    let path = format!("{commitment_prefix}clients/{client_id}/clientState");

    // Create SignData
    let sign_data = SolomachineSignData {
        diversifier: solo_client_state.consensus_state.diversifier.clone(),
        sequence: solo_client_state.sequence,
        timestamp: solo_client_state.consensus_state.timestamp,
        path: path.into(),
        data: client_state_bytes,
    };

    // Sign data using Secret Key
    let signed_data = sign_with_data(secret_key, &sign_data);

    let data = Data {
        sum: Some(Sum::Single(Single {
            mode: 0,
            signature: signed_data.serialize_compact().to_vec(),
        })),
    };

    let bytes_data = encode_protobuf(&data);

    // Create Timestamped signed data
    let timestamped_signed_data = SolomachineTimestampedSignData {
        signature_data: bytes_data,
        timestamp: solo_client_state.consensus_state.timestamp,
    };

    timestamped_signed_data
}
