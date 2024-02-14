use cgp_core::prelude::*;
use cgp_core::{DelegateComponent, HasErrorType};
use hermes_cosmos_client_components::traits::rpc_client::HasRpcClient;
use hermes_cosmos_relayer::chain::impls::query_client_state::DelegateCosmosClientStateQuerier;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerier;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::cosmos::query::abci_query;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::core::ics24_host::path::ClientStatePath;
use ibc_relayer_types::core::ics24_host::IBC_QUERY_PATH;
use ibc_relayer_types::Height;

use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::client_state::{decode_client_state, SolomachineClientState};

pub struct QuerySolomachineClientStateFromCosmos;

impl<Counterparty> DelegateComponent<SolomachineChain<Counterparty>>
    for DelegateCosmosClientStateQuerier
where
    Counterparty: Solomachine,
{
    type Delegate = QuerySolomachineClientStateFromCosmos;
}

#[async_trait]
impl<Chain, Counterparty> ClientStateQuerier<Chain, Counterparty>
    for QuerySolomachineClientStateFromCosmos
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasErrorType
        + HasRpcClient
        + CanQueryChainHeight,
    Counterparty: HasClientStateType<Chain, ClientState = SolomachineClientState>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &ClientId,
        _height: &Height,
    ) -> Result<SolomachineClientState, Chain::Error> {
        let data = ClientStatePath(client_id.clone());

        let query_height = chain.query_chain_height().await?;

        let response = abci_query(
            chain.rpc_client(),
            chain.rpc_address(),
            IBC_QUERY_PATH.to_string(),
            data.to_string(),
            query_height.into(),
            true,
        )
        .await
        .unwrap();

        let client_state = decode_client_state(response.value.as_slice());

        Ok(client_state)
    }
}
