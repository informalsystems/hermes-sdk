use oneline_eyre::eyre::Context;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithLatestHeight,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeights;
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
        help = "Height of the client's consensus state to query, if not specified all consensus heights are returned"
    )]
    consensus_height: Option<u64>,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "The height at which to query the chain"
    )]
    height: Option<u64>,
}

impl CommandRunner<CosmosBuilder> for QueryClientConsensus {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let chain = builder.build_chain(&self.chain_id).await?;

        let counterparty_chain_id = <CosmosChain as CanQueryClientStateWithLatestHeight<
            CosmosChain,
        >>::query_client_state_with_latest_height(
            &chain, &self.client_id
        )
        .await
        .map(|cs| cs.chain_id)
        .wrap_err("failed to query counterparty chain from client state")?;

        if let Some(consensus_height) = self.consensus_height {
            let consensus_height = Height::new(counterparty_chain_id.version(), consensus_height)
                .wrap_err_with(|| format!(
                    "failed to create Height with revision number `{}` and revision height `{consensus_height}`",
                    counterparty_chain_id.version()
                ))?;

            let query_height = self.height.map(|height| {
                Height::new(self.chain_id.version(), height)
                    .wrap_err_with(|| format!(
                        "Failed to create Height with revision number `{}` and revision height `{height}`", 
                        self.chain_id.version()
                    ))
            }).transpose()?;

            let consensus_state = if let Some(query_height) = query_height {
                chain
                    .query_consensus_state(&self.client_id, &consensus_height, &query_height)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to query consensus state at height `{}` for client `{}`",
                            consensus_height, self.client_id
                        )
                    })?
            } else {
                chain
                    .query_consensus_state_with_latest_height(&self.client_id, &consensus_height)
                    .await
                    .wrap_err_with(|| {
                        format!(
                            "failed to query latest consensus state at height `{}` for client `{}`",
                            consensus_height, self.client_id
                        )
                    })?
            };

            Ok(Output::success(consensus_state))
        } else {
            let heights = <CosmosChain as CanQueryConsensusStateHeights<CosmosChain>>::query_consensus_state_heights(&chain, &self.client_id).await
                .wrap_err_with(|| format!("failed to query consensus state heights for client `{}`", self.client_id))?;

            Ok(Output::success(heights))
        }
    }
}
