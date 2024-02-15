use oneline_eyre::eyre::eyre;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::BaseError;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{
    IncludeProof, QueryClientStateRequest, QueryConsensusStateHeightsRequest,
    QueryConsensusStateRequest, QueryHeight,
};
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use ibc_relayer_types::Height;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryClientConsensus {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "client",
        required = true,
        value_name = "CLIENT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the client to query"
    )]
    client_id: ClientId,

    #[clap(
        long = "consensus-height",
        value_name = "CONSENSUS_HEIGHT",
        help = "Height of the client's consensus state to query"
    )]
    consensus_height: Option<u64>,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "The chain height context to be used, applicable only to a specific height"
    )]
    height: Option<u64>,
}

impl CommandRunner<CosmosBuilder> for QueryClientConsensus {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let chain_id = self.chain_id.clone();
        let client_id = self.client_id.clone();
        let chain = builder.build_chain(&chain_id).await?;

        let counterparty_chain_id = chain
            .with_blocking_chain_handle(move |chain_handle| {
                match chain_handle.query_client_state(
                    QueryClientStateRequest {
                        client_id: client_id.clone(),
                        height: QueryHeight::Latest,
                    },
                    IncludeProof::No,
                ) {
                    Ok((client_state, _)) => Ok(client_state.chain_id()),
                    Err(e) => Err(BaseError::generic(eyre!(
                        "failed while querying client {client_id} for chain {chain_id}; {e}"
                    ))
                    .into()),
                }
            })
            .await?;

        if let Some(cs_height) = self.consensus_height {
            let client_id = self.client_id.clone();
            let height = self.height;

            let consensus_height = Height::new(counterparty_chain_id.version(), cs_height)
                .map_err(|e| BaseError::generic(eyre!(
                    "failed to create Height with revision number `{}` and revision height `{cs_height}`; {e}",
                    counterparty_chain_id.version()
                )))?;

            let consensus_state = chain
                .with_blocking_chain_handle(move |chain_handle| {
                    let chain_id = chain_handle.id();
                    let query_height = if let Some(height) = height {
                        let specified_height = Height::new(chain_id.version(), height)
                            .map_err(|e| BaseError::generic(eyre!("Failed to create Height with revision number `{}` and revision height `{height}`. Error: {e}", chain_id.version())))?;

                        QueryHeight::Specific(specified_height)
                    } else {
                        QueryHeight::Latest
                    };

                    match chain_handle.query_consensus_state(
                        QueryConsensusStateRequest {
                            client_id,
                            consensus_height,
                            query_height,
                        },
                        IncludeProof::No,
                    ) {
                        Ok((consensus_state, _)) => Ok(consensus_state),
                        Err(e) => Err(BaseError::relayer(e).into()),
                    }
                })
                .await?;

            Ok(Output::success(consensus_state))
        } else {
            let client_id = self.client_id.clone();

            let consensus_state_heights = chain
                .with_blocking_chain_handle(move |chain_handle| {
                    match chain_handle.query_consensus_state_heights(
                        QueryConsensusStateHeightsRequest {
                            client_id: client_id.clone(),
                            pagination: Some(ibc_relayer::chain::requests::PageRequest::all()),
                        },
                    ) {
                        Ok(csh) => Ok(csh),
                        Err(e) => Err(BaseError::relayer(e).into()),
                    }
                })
                .await?;

            Ok(Output::success(consensus_state_heights))
        }
    }
}
