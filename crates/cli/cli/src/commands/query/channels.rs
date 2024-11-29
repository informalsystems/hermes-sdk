use std::marker::PhantomData;
use std::str::FromStr;
use tracing::{info, warn};

use hermes_chain_components::traits::queries::client_state::CanQueryClientState;
use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_chain_components::traits::grpc_address::HasGrpcAddress;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;

use ibc::core::channel::types::proto::v1::query_client::QueryClient;
use ibc_relayer::chain::requests::QueryChannelsRequest;
use ibc_relayer_types::core::ics04_channel::channel::{IdentifiedChannelEnd, State};
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, ClientId, PortId};

use crate::contexts::app::HermesApp;
use crate::impls::error_wrapper::ErrorWrapper;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryChannels {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "counterparty-chain",
        value_name = "COUNTERPARTY_CHAIN_ID",
        help = "Filter the query response by the counterparty chain"
    )]
    counterparty_chain_id: Option<ChainId>,

    #[clap(
        long = "show-counterparty",
        help = "Show the counterparty's chain, port, and channel"
    )]
    show_counterparty: bool,
}

impl CommandRunner<HermesApp> for QueryChannels {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let chain = builder.build_chain(&self.chain_id).await?;
        let chain_id = self.chain_id.clone();
        let dst_chain_id = self.counterparty_chain_id.clone();
        let show_counterparty = self.show_counterparty;

        let mut client = QueryClient::connect(chain.grpc_address().clone())
            .await
            .unwrap();

        let request = tonic::Request::new(QueryChannelsRequest { pagination: None }.into());

        let response = client.channels(request).await.unwrap().into_inner();

        let all_channels = response
            .channels
            .into_iter()
            .filter_map(|ch| IdentifiedChannelEnd::try_from(ch.clone()).ok())
            .collect::<Vec<IdentifiedChannelEnd>>();

        let chain_height = chain
            .query_chain_height()
            .await
            .wrap_error("Failed to query latest chain height")?;

        let mut channels = Vec::new();

        for channel in all_channels {
            let port_id = &channel.port_id;
            let channel_id = &channel.channel_id;
            let chain_id = chain_id.clone();
            let channel_end = &channel.channel_end;

            if channel_end.state_matches(&State::Uninitialized) {
                warn!(
                    "channel `{port_id}/{channel_id}` on chain `{chain_id}` at {chain_height} is uninitialized"
                );

                continue;
            }

            let Some(connection_id) = channel.channel_end.connection_hops.first() else {
                warn!(
                    "missing connection hops for `{port_id}/{channel_id}` on chain `{chain_id}` at `{chain_height}`"
                );

                continue;
            };

            let counterparty = if show_counterparty || dst_chain_id.is_some() {
                let connection_id = connection_id.clone();
                let connection_end = <hermes_cosmos_relayer::contexts::chain::CosmosChain as hermes_chain_components::traits::queries::connection_end::CanQueryConnectionEnd<Counterparty>>::query_connection_end(&chain, &connection_id, &chain_height)
                    .await;

                let Ok(connection_end) = connection_end else {
                    warn!(
                        "missing connection end for `{port_id}/{channel_id}` on chain `{chain_id}` at {chain_height}"
                    );

                    continue;
                };

                let client_id = ClientId::from_str(connection_end.client_id().as_str())?;
                let client_state = chain
                    .query_client_state(PhantomData::<CosmosChain>, &client_id, &chain_height)
                    .await;

                let Ok(client_state) = client_state else {
                    warn!("missing client state for {port_id}/{channel_id} on chain {chain_id} at {chain_height}");

                    continue;
                };

                let client_state_chain_id_matches_dst_chain_id = dst_chain_id
                    .as_ref()
                    .map(|dst_chain_id| dst_chain_id == &client_state.chain_id)
                    .unwrap_or(true);

                if !client_state_chain_id_matches_dst_chain_id {
                    continue;
                }

                let counterparty = channel_end.counterparty();

                Some(Counterparty {
                    chain_id: client_state.chain_id.clone(),
                    port_id: counterparty.port_id.clone(),
                    channel_id: counterparty.channel_id.clone(),
                })
            } else {
                None
            };

            channels.push((channel, counterparty));
        }

        info!("Found {} channels on chain `{chain_id}`", channels.len());

        if json() {
            let channels = channels
                .into_iter()
                .map(|(channel, counterparty)| {
                    let (port_id, channel_id) = (channel.port_id, channel.channel_id);

                    let mut result = serde_json::json!({
                        "port_id": port_id,
                        "channel_id": channel_id,
                    });

                    if let Some(counterparty) = counterparty {
                        result["counterparty"] = serde_json::to_value(counterparty).unwrap();
                    }

                    result
                })
                .collect::<Vec<_>>();

            return Ok(Output::success(channels));
        }

        channels.iter().for_each(|(channel, counterparty)| {
            info!("- {}/{}", channel.port_id, channel.channel_id);

            if let Some(counterparty) = counterparty {
                info!(
                    "  - counterparty: {}/{} on chain {}",
                    counterparty.port_id,
                    counterparty
                        .channel_id
                        .as_ref()
                        .map_or("unknown".to_string(), |c| c.to_string()),
                    counterparty.chain_id
                );
            }
        });

        Ok(Output::success_msg(format!(
            "Total: {} channels",
            channels.len()
        )))
    }
}

#[derive(Debug, serde::Serialize)]
struct Counterparty {
    chain_id: ChainId,
    port_id: PortId,
    channel_id: Option<ChannelId>,
}
