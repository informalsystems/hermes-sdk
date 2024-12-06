use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_chain_components::traits::abci_query::CanQueryAbci;
use ibc::core::channel::types::channel::{ChannelEnd, State};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChainId, ChannelId, PortId};
use ibc::cosmos_host::IBC_QUERY_PATH;
use ibc::primitives::proto::Protobuf;
use oneline_eyre::eyre::eyre;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryChannelEnd {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "port",
        required = true,
        value_name = "PORT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the port to query"
    )]
    port_id: PortId,

    #[clap(
        long = "channel",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the channel to query"
    )]
    channel_id: ChannelId,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "Height of the state to query. Leave unspecified for the latest height."
    )]
    height: Option<u64>,
}

impl CommandRunner<HermesApp> for QueryChannelEnd {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let chain = builder.build_chain(&self.chain_id).await?;
        let channel_id = self.channel_id.clone();
        let port_id = self.port_id.clone();
        let height = self.height;

        let query_height = if let Some(height) = height {
            Height::new(chain.chain_id.revision_number(), height)?
        } else {
            chain.query_chain_height().await?
        };

        // channel end query path
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let channel_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, channel_end_path.as_bytes(), &query_height)
            .await?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes);

        match channel_end {
            Ok(channel_end) => {
                if channel_end
                    .verify_state_matches(&State::Uninitialized)
                    .is_ok()
                {
                    Err(eyre!(
                        "port '{}' & channel '{}' do not exist",
                        self.port_id,
                        self.channel_id,
                    )
                    .into())
                } else {
                    Ok(Output::success(channel_end))
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
