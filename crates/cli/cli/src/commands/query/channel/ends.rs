use std::str::FromStr;

use cgp::prelude::*;
use eyre::eyre;
use hermes_cli_components::traits::{CanLoadBuilder, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_core::chain_components::traits::{CanQueryChainHeight, CanQueryConnectionEnd};
use hermes_core::encoding_components::traits::{CanConvert, HasDefaultEncoding};
use hermes_cosmos_chain_components::traits::CanQueryAbci;
use hermes_cosmos_chain_components::types::TendermintClientState;
use hermes_cosmos_relayer::contexts::CosmosChain;
use hermes_protobuf_encoding_components::types::any::Any;
use ibc::clients::tendermint::types::TENDERMINT_CLIENT_STATE_TYPE_URL;
use ibc::core::channel::types::channel::{ChannelEnd, State};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChainId, ChannelId, ClientId, ConnectionId, PortId};
use ibc::cosmos_host::IBC_QUERY_PATH;
use ibc::primitives::proto::Protobuf;
use serde::{Deserialize, Serialize};

use crate::contexts::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryChannelEnds {
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
        help = "Height of the state to query. Leave unspecified for latest height."
    )]
    height: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChannelEndsSummary {
    chain_id: ChainId,
    client_id: ClientId,
    connection_id: ConnectionId,
    channel_id: ChannelId,
    port_id: PortId,
    counterparty_chain_id: ChainId,
    counterparty_client_id: ClientId,
    counterparty_connection_id: ConnectionId,
    counterparty_channel_id: ChannelId,
    counterparty_port_id: PortId,
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for QueryChannelEnds {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let chain_id = self.chain_id.clone();
        let channel_id = self.channel_id.clone();
        let port_id = self.port_id.clone();
        let height = self.height;

        let chain = builder.build_chain(&chain_id).await?;

        let query_height = if let Some(height) = height {
            Height::new(chain.chain_id.revision_number(), height)?
        } else {
            chain.query_chain_height().await?
        };

        // channel end query path
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let channel_end_bytes: Vec<u8> = chain
            .query_abci(IBC_QUERY_PATH, channel_end_path.as_bytes(), &query_height)
            .await?
            .ok_or_else(|| {
                HermesApp::raise_error(format!("channel not found: {channel_id}/{port_id}"))
            })?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes)?;

        if channel_end
            .verify_state_matches(&State::Uninitialized)
            .is_ok()
        {
            return Err(eyre!(
                "{port_id}/{channel_id} on chain {chain_id} @ {query_height:?} is uninitialized",
            )
            .into());
        }

        let Some(connection_id) = channel_end.connection_hops.first() else {
            return Err(eyre!(
                        "missing connection hops for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?}",
                    ).into());
        };

        let connection_end =
            <CosmosChain as CanQueryConnectionEnd<CosmosChain>>::query_connection_end(
                &chain,
                connection_id,
                &query_height,
            )
            .await?;

        let client_id = ClientId::from_str(connection_end.client_id().as_str())?;

        // client state query path
        let client_state_path = format!("clients/{client_id}/clientState");

        let client_state_bytes = chain
            .query_abci(IBC_QUERY_PATH, client_state_path.as_bytes(), &query_height)
            .await?
            .ok_or_else(|| {
                HermesApp::raise_error(format!("client state not found: {client_id}"))
            })?;

        let any_client_state = Any {
            type_url: TENDERMINT_CLIENT_STATE_TYPE_URL.to_owned(),
            value: client_state_bytes,
        };

        let client_state: TendermintClientState =
            CosmosChain::default_encoding().convert(&any_client_state)?;

        let channel_counterparty = channel_end.counterparty().clone();
        let connection_counterparty = connection_end.counterparty().clone();
        let counterparty_client_id =
            ClientId::from_str(connection_counterparty.client_id().as_str())?;

        let Some(counterparty_connection_id) = connection_counterparty.connection_id else {
            return Err(eyre!(
                        "connection end for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?} does not have counterparty connection id {connection_end:?}",
                    ).into());
        };

        let counterparty_port_id = channel_counterparty.port_id().clone();

        let Some(counterparty_channel_id) = channel_counterparty.channel_id else {
            return Err(eyre!(
                        "channel end for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?} does not have counterparty channel id {channel_end:?}",
                    ).into());
        };

        let counterparty_chain_id = client_state.chain_id();

        let channel_ends_summary = ChannelEndsSummary {
            chain_id,
            client_id,
            connection_id: connection_id.clone(),
            channel_id,
            port_id,
            counterparty_chain_id: counterparty_chain_id.clone(),
            counterparty_client_id,
            counterparty_connection_id: ConnectionId::from_str(
                counterparty_connection_id.as_str(),
            )?,
            counterparty_channel_id,
            counterparty_port_id,
        };

        Ok(Output::success(channel_ends_summary))
    }
}
