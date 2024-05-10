use cgp_core::{CanRaiseError, HasErrorType};
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_sovereign_rollup_components::traits::json_rpc_client::HasJsonRpcClient;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_sovereign_rollup_components::types::rpc::height::HeightParam;
use ibc::core::client::types::error::ClientError;
use ibc_proto::ibc::core::connection::v1::Version as ProtoVersion;
use ibc_query::core::client::{QueryClientStateResponse, QueryConsensusStateResponse};
use ibc_query::core::connection::QueryConnectionResponse;
use ibc_relayer_types::core::ics02_client::error::Error as RelayerClientError;
use ibc_relayer_types::core::ics03_connection::error::Error as ConnectionError;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::proofs::ProofError;
use ibc_relayer_types::Height;
use jsonrpsee::core::client::ClientT;
use serde::Serialize;
use sov_celestia_client::types::client_state::SovTmClientState as SovereignClientState;

use crate::sovereign::traits::chain::rollup::HasRollup;
use crate::sovereign::types::payloads::connection::{
    SovereignConnectionOpenAckPayload, SovereignConnectionOpenConfirmPayload,
    SovereignConnectionOpenInitPayload, SovereignConnectionOpenTryPayload,
};

pub struct BuildSovereignConnectionHandshakePayload;

impl<Chain, Counterparty, Rollup> ConnectionHandshakePayloadBuilder<Chain, Counterparty>
    for BuildSovereignConnectionHandshakePayload
where
    Chain: HasConnectionHandshakePayloadTypes<
            Counterparty,
            ConnectionOpenInitPayload = SovereignConnectionOpenInitPayload,
            ConnectionOpenTryPayload = SovereignConnectionOpenTryPayload,
            ConnectionOpenAckPayload = SovereignConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = SovereignConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<Counterparty, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasHeightType<Height = RollupHeight>
        + HasRollup<Rollup = Rollup>
        + HasClientStateType<Counterparty, ClientState = SovereignClientState>
        + HasErrorType
        + CanRaiseError<ClientError>
        + CanRaiseError<RelayerClientError>
        + CanRaiseError<ProofError>
        + CanRaiseError<ConnectionError>
        + CanRaiseError<Rollup::Error>,
    Rollup: CanQueryChainHeight<Height = RollupHeight> + HasJsonRpcClient,
    Rollup::JsonRpcClient: ClientT,
{
    async fn build_connection_open_init_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
    ) -> Result<Chain::ConnectionOpenInitPayload, Chain::Error> {
        // TODO: retrieve commimtment prefix
        let commitment_prefix = "ibc".into();
        Ok(SovereignConnectionOpenInitPayload { commitment_prefix })
    }

    async fn build_connection_open_try_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _client_id: &Chain::ClientId,
        _connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenTryPayload, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_ack_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &ConnectionId,
    ) -> Result<Chain::ConnectionOpenAckPayload, Chain::Error> {
        let rollup_height = chain
            .rollup()
            .query_chain_height()
            .await
            .map_err(Chain::raise_error)?;

        let rollup_connection_end =
            query_connection_end(chain.rollup(), connection_id, &rollup_height).await;

        let rollup_client_state =
            query_client_state(chain.rollup(), client_id, &rollup_height).await;

        let client_state = SovereignClientState::try_from(rollup_client_state.client_state)
            .map_err(Chain::raise_error)?;

        let consensus_height = Height::new(
            client_state
                .sovereign_params
                .genesis_da_height
                .revision_number(),
            height.slot_number
                + client_state
                    .sovereign_params
                    .genesis_da_height
                    .revision_height(),
        )
        .map_err(Chain::raise_error)?;

        let rollup_consensus_state =
            query_consensus_state(chain.rollup(), client_id, &consensus_height, &rollup_height)
                .await;

        let consensus_proof_height = Height::new(
            rollup_consensus_state.proof_height.revision_number(),
            rollup_consensus_state.proof_height.revision_height(),
        )
        .map_err(Chain::raise_error)?;

        let ibc_version = rollup_connection_end
            .conn_end
            .versions()
            .iter()
            .next()
            .cloned()
            .unwrap();

        let proto_version = ProtoVersion::from(ibc_version);

        let version = Version::try_from(proto_version).map_err(Chain::raise_error)?;

        Ok(SovereignConnectionOpenAckPayload {
            client_state,
            version,
            update_height: consensus_proof_height,
            proof_try: rollup_connection_end.proof,
            proof_client: rollup_client_state.proof,
            proof_consensus: rollup_consensus_state.proof,
            proof_consensus_height: consensus_proof_height,
        })
    }

    async fn build_connection_open_confirm_payload(
        _chain: &Chain,
        _client_state: &Chain::ClientState,
        _height: &Chain::Height,
        _client_id: &Chain::ClientId,
        _connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenConfirmPayload, Chain::Error> {
        todo!()
    }
}

pub async fn query_connection_end<'a, Rollup>(
    rollup: &Rollup,
    connection_id: &'a ConnectionId,
    rollup_height: &'a RollupHeight,
) -> QueryConnectionResponse
where
    Rollup: HasJsonRpcClient,
    Rollup::JsonRpcClient: ClientT,
{
    #[derive(Serialize)]
    struct Request<'a> {
        pub connection_id: &'a str,
        pub query_height: &'a HeightParam,
    }

    let request = Request {
        connection_id: connection_id.as_str(),
        query_height: &rollup_height.into(),
    };

    rollup
        .json_rpc_client()
        .request("ibc_connection", (request,))
        .await
        .unwrap()
}

pub async fn query_client_state<'a, Rollup>(
    rollup: &Rollup,
    client_id: &'a ClientId,
    rollup_height: &'a RollupHeight,
) -> QueryClientStateResponse
where
    Rollup: HasJsonRpcClient,
    Rollup::JsonRpcClient: ClientT,
{
    #[derive(Serialize)]
    pub struct Request<'a> {
        pub client_id: &'a str,
        pub query_height: &'a HeightParam,
    }

    let request = Request {
        client_id: client_id.as_str(),
        query_height: &rollup_height.into(),
    };

    rollup
        .json_rpc_client()
        .request("ibc_clientState", (request,))
        .await
        .unwrap()
}

pub async fn query_consensus_state<'a, Rollup>(
    rollup: &Rollup,
    client_id: &'a ClientId,
    consensus_height: &'a Height,
    rollup_height: &'a RollupHeight,
) -> QueryConsensusStateResponse
where
    Rollup: HasJsonRpcClient,
    Rollup::JsonRpcClient: ClientT,
{
    #[derive(Serialize)]
    pub struct Request<'a> {
        pub client_id: &'a str,
        pub consensus_height: &'a HeightParam,
        pub query_height: &'a HeightParam,
    }

    let request = Request {
        client_id: client_id.as_str(),
        consensus_height: &HeightParam {
            revision_number: consensus_height.revision_number(),
            revision_height: consensus_height.revision_height(),
        },
        query_height: &rollup_height.into(),
    };

    rollup
        .json_rpc_client()
        .request("ibc_consensusState", (request,))
        .await
        .unwrap()
}
