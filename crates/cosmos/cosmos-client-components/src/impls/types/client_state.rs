use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoder, HasClientStateType, ProvideClientStateType,
};
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoClientState;
use ibc_proto::Protobuf;
use tendermint_proto::Error as ProtoError;

use crate::types::tendermint::TendermintClientState;

pub struct ProvideTendermintClientState;

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideTendermintClientState
where
    Chain: Async,
{
    type ClientState = TendermintClientState;
}

impl<Chain, Counterparty> ClientStateDecoder<Chain, Counterparty> for ProvideTendermintClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = TendermintClientState>,
{
    type DecodeClientStateError = ProtoError;

    fn decode_client_state_bytes(
        client_state_bytes: &[u8],
    ) -> Result<TendermintClientState, ProtoError> {
        Protobuf::<ProtoClientState>::decode_vec(&client_state_bytes)
    }
}
