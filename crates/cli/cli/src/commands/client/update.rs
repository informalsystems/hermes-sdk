use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::BaseError;
use hermes_relayer_components::build::traits::components::relay_builder::CanBuildRelay;
use hermes_relayer_components::build::traits::target::relay::RelayAToBTarget;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::relay::traits::target::SourceTarget;
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendUpdateClientMessage;
use ibc_relayer::chain::handle::ChainHandle;
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
    //
    // #[clap(
    //     long = "trusted-height",
    //     value_name = "TRUSTED_HEIGHT",
    //     help = "The trusted height of the client update. Leave unspecified for latest height."
    // )]
    // trusted_height: Option<u64>,
    //
    // #[clap(
    //     long = "archive-address",
    //     value_name = "ARCHIVE_ADDRESS",
    //     group = "restart",
    //     requires = "restart_height",
    //     help_heading = "Update the client across a genesis restart",
    //     help = "The RPC address of the archive node to use to fetch headers from before the restart. Requires --restart-height if used."
    // )]
    // archive_address: Option<String>,
    //
    // #[clap(
    //     long = "restart-height",
    //     value_name = "RESTART_HEIGHT",
    //     group = "restart",
    //     requires = "archive_address",
    //     help_heading = "Update the client across a genesis restart",
    //     help = "The height that the chain underwent a genesis restart at. Requires --archive-address if used."
    // )]
    // restart_height: Option<u64>,
}

impl Runnable for ClientUpdate {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        let host_chain = builder.build_chain(&self.host_chain_id).await?;
        let client_state = host_chain.query_client_state(&self.client_id).await?;
        let reference_chain_id = client_state.chain_id;
        let reference_chain = builder.build_chain(&reference_chain_id).await?;

        let relayer = CanBuildRelay::build_relay(
            &builder,
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
                let height = reference_chain
                    .with_blocking_chain_handle(|handle| {
                        handle
                            .query_latest_height()
                            .map_err(|e| BaseError::relayer(e).into())
                    })
                    .await
                    .wrap_err("Failed to fetch latest height on reference chain")?;

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
