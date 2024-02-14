use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerier;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoClientState;
use ibc_proto::Protobuf;

use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;
use tendermint_proto::Error as ProtoError;

use crate::traits::abci_query::CanQueryAbci;
use crate::types::tendermint::TendermintClientState;

pub struct QueryCosmosClientStateFromChainHandle;

pub const IBC_QUERY_PATH: &str = "store/ibc/key";

impl<Chain, Counterparty> ClientStateQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromChainHandle
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + CanQueryAbci
        + CanRaiseError<ProtoError>,
    Counterparty: HasClientStateType<Chain, ClientState = TendermintClientState>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<TendermintClientState, Chain::Error> {
        let client_state_path = format!("clients/{client_id}/clientState");

        let client_state_bytes = chain
            .query_abci(IBC_QUERY_PATH, client_state_path.as_bytes(), height)
            .await?;

        let client_state = Protobuf::<ProtoClientState>::decode_vec(&client_state_bytes)
            .map_err(Chain::raise_error)?;

        Ok(client_state)
    }
}
