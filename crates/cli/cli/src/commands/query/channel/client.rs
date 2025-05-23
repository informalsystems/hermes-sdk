use hermes_cli_components::traits::{CanLoadBuilder, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos::chain_components::traits::HasGrpcAddress;
use hermes_cosmos::ibc::core::channel::types::proto::v1::query_client::QueryClient;
use hermes_cosmos::ibc::core::channel::types::proto::v1::QueryChannelClientStateRequest;
use hermes_cosmos::ibc::core::host::types::identifiers::{ChainId, ChannelId, PortId};
use hermes_prelude::*;
use http::Uri;

use crate::contexts::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryChannelClient {
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
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for QueryChannelClient {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let chain = builder.build_chain(&self.chain_id).await?;
        let channel_id = self.channel_id.clone();
        let port_id = self.port_id.clone();

        let mut client =
            QueryClient::connect(Uri::try_from(&chain.grpc_address().to_string())?).await?;

        let request = tonic::Request::new(QueryChannelClientStateRequest {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
        });

        let response = client.channel_client_state(request).await?.into_inner();

        let client_state = response.identified_client_state;

        Ok(Output::success(client_state))
    }
}
