use cgp_core::error::HasErrorType;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerier;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::traits::solomachine::Solomachine;

pub struct QueryCosmosClientStateFromSolomachine;

impl<Chain, Counterparty> ClientStateQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromSolomachine
where
    Chain: Solomachine
        + HasIbcChainTypes<Counterparty, Height = Height, ClientId = ClientId>
        + HasErrorType,
    Counterparty: HasClientStateType<Chain>,
    Counterparty::ClientState: From<TendermintClientState>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &ClientId,
        _height: &Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let client_state = chain.query_client_state(client_id).await?;
        Ok(client_state.into())
    }
}
