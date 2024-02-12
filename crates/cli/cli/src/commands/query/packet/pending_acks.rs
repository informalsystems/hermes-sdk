use crate::Result;
use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::BaseError;
use ibc_relayer::chain::counterparty::{channel_connection_client, unreceived_acknowledgements};
use ibc_relayer::path::PathIdentifiers;
use ibc_relayer::util::collate::CollatedIterExt;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};
use oneline_eyre::eyre::eyre;

#[derive(Debug, clap::Parser)]
pub struct QueryPendingAcks {
    /// Identifier of the host chain
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    chain_id: ChainId,

    /// Identifier of the port
    #[clap(
        long = "port",
        required = true,
        value_name = "PORT_ID",
        help_heading = "REQUIRED"
    )]
    port_id: PortId,

    /// Identifier of the channel
    #[clap(
        long = "channel",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED"
    )]
    channel_id: ChannelId,
}

impl Runnable for QueryPendingAcks {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        let port_id = self.port_id.clone();
        let channel_id = self.channel_id.clone();
        let chain = builder.build_chain(&self.chain_id).await?;

        let chan_conn_cli = chain
            .with_blocking_chain_handle(move |handle| {
                let chan_conn_cli = channel_connection_client(&handle, &port_id, &channel_id)
                    .map_err(|e| {
                        BaseError::generic(eyre!("failed channel_connection_client: {}", e))
                    })?;
                Ok(chan_conn_cli)
            })
            .await
            .map_err(|e| {
                BaseError::generic(eyre!("failed pending_packet_summary on source: {}", e))
            })?;

        let counterparty_chain_id = chan_conn_cli.client.client_state.chain_id();
        let counterparty_chain = builder.build_chain(&counterparty_chain_id.clone()).await?;

        let channel = chan_conn_cli.channel.clone();
        let path_identifiers =
            PathIdentifiers::from_channel_end(channel.clone()).ok_or_else(|| {
                BaseError::generic(eyre!("failed to get the path identifiers from channel"))
            })?;

        let acks = unreceived_acknowledgements(
            &chain.handle,
            &counterparty_chain.handle,
            &path_identifiers,
        )
        .map_err(|e| BaseError::generic(eyre!("failed unreceived_acknowledgements: {}", e)))?;

        let seqs = acks.map_or(vec![], |(sns, _)| sns);
        let seqs_collated = seqs.into_iter().collated().collect::<Vec<_>>();
        Ok(Output::success(seqs_collated))
    }
}
