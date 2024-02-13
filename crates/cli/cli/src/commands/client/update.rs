use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_components::build::traits::components::relay_builder::CanBuildRelay;
use hermes_relayer_components::build::traits::target::relay::RelayAToBTarget;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::relay::traits::target::SourceTarget;
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendUpdateClientMessage;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use oneline_eyre::eyre::Context;
use tracing::info;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct ClientUpdate {
    #[clap(
        long = "host-chain",
        required = true,
        value_name = "HOST_CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain that hosts the client"
    )]
    host_chain_id: ChainId,

    #[clap(
        long = "client",
        required = true,
        value_name = "CLIENT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the client to update"
    )]
    client_id: ClientId,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "The target height of the client update. Leave unspecified for latest height."
    )]
    target_height: Option<u64>,
}

impl CommandRunner<CosmosBuilder> for ClientUpdate {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let host_chain = builder.build_chain(&self.host_chain_id).await?;
        let client_state = host_chain.query_client_state(&self.client_id).await?;
        let reference_chain_id = client_state.chain_id;
        let reference_chain = builder.build_chain(&reference_chain_id).await?;

        let relayer = builder
            .build_relay(
                RelayAToBTarget,
                &self.host_chain_id,
                &reference_chain_id,
                &self.client_id,
                &self.client_id, // nothing to pass here
            )
            .await?;

        let target_height = match self.target_height {
            Some(height) => {
                let height = Height::new(reference_chain_id.version(), height)
                    .wrap_err("Invalid value for --target-height")?;

                info!("Updating client using specified target height: {height}");
                height
            }
            None => {
                let height = reference_chain.query_chain_height().await?;

                info!("Updating client using specified target height: {height}");
                height
            }
        };

        relayer
            .send_update_client_messages(SourceTarget, &target_height)
            .await
            .wrap_err("Failed to send update client message")?;

        Ok(Output::success_msg("Client successfully updated!"))
    }
}
