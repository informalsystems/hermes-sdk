use core::marker::PhantomData;

use cgp::core::field::Index;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelInfo;
use hermes_core::relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_core::relayer_components::build::traits::builders::relay_builder::CanBuildRelay;
use hermes_core::relayer_components::chain::traits::{
    CanQueryChainHeight, CanQueryClientStateWithLatestHeight, HasChainIdType, HasClientIdType,
    HasClientStateFields, HasClientStateType, HasHeightType,
};
use hermes_core::relayer_components::relay::traits::{
    CanSendTargetUpdateClientMessage, HasRelayChains, HasRelayClientIds, HasSourceTargetChainTypes,
    SourceTarget,
};
use hermes_prelude::*;

use crate::traits::{
    CanLoadBuilder, CanParseArg, CanProduceOutput, CommandRunner, CommandRunnerComponent,
};

#[derive(Debug, clap::Parser, HasField)]
pub struct UpdateClientArgs {
    #[clap(
        long = "host-chain",
        required = true,
        value_name = "HOST_CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain that hosts the client"
    )]
    host_chain_id: String,

    #[clap(
        long = "client",
        required = true,
        value_name = "CLIENT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the client to update"
    )]
    client_id: String,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "The target height of the client update. Leave unspecified for latest height."
    )]
    target_height: Option<String>,

    // TODO: Remove once it is not necessary to specify the counterparty client ID
    // when building the relayer
    #[clap(
        long = "counterparty-client",
        required = true,
        value_name = "COUNTERPARTY_CLIENT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the counterparty client to update"
    )]
    counterparty_client_id: String,
}

#[cgp_new_provider(CommandRunnerComponent)]
impl<App, Args, Builder, Chain, Counterparty, Relay> CommandRunner<App, Args>
    for RunUpdateClientCommand
where
    App: CanLoadBuilder<Builder = Builder>
        + CanProduceOutput<&'static str>
        + CanLog<LevelInfo>
        + CanParseArg<Args, symbol!("host_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("client_id"), Parsed = Chain::ClientId>
        + CanParseArg<Args, symbol!("counterparty_client_id"), Parsed = Counterparty::ClientId>
        + CanParseArg<Args, symbol!("target_height"), Parsed = Option<Counterparty::Height>>
        + CanRaiseAsyncError<Builder::Error>
        + CanRaiseAsyncError<Chain::Error>
        + CanRaiseAsyncError<Counterparty::Error>
        + CanRaiseAsyncError<Relay::Error>,
    Builder: CanBuildChain<Index<0>, Chain = Chain>
        + CanBuildChain<Index<1>, Chain = Counterparty>
        + CanBuildRelay<Index<0>, Index<1>, Relay = Relay>,
    Chain: HasChainIdType + CanQueryClientStateWithLatestHeight<Counterparty> + HasAsyncErrorType,
    Counterparty: HasChainIdType
        + HasClientStateType<Chain>
        + HasClientIdType<Chain>
        + HasHeightType
        + HasClientStateFields<Chain>
        + CanQueryChainHeight
        + HasAsyncErrorType,
    Relay: HasRelayChains<SrcChain = Chain, DstChain = Counterparty>
        + HasSourceTargetChainTypes
        + HasRelayClientIds
        + CanSendTargetUpdateClientMessage<SourceTarget>,
    Args: Async,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let host_chain_id = app.parse_arg(args, PhantomData::<symbol!("host_chain_id")>)?;
        let client_id = app.parse_arg(args, PhantomData::<symbol!("client_id")>)?;
        let counterparty_client_id =
            app.parse_arg(args, PhantomData::<symbol!("counterparty_client_id")>)?;
        let target_height = app.parse_arg(args, PhantomData::<symbol!("target_height")>)?;
        let builder = app.load_builder().await?;

        let host_chain = builder
            .build_chain(PhantomData::<Index<0>>, &host_chain_id)
            .await
            .map_err(App::raise_error)?;

        let client_state = host_chain
            .query_client_state_with_latest_height(PhantomData, &client_id)
            .await
            .map_err(App::raise_error)?;

        let reference_chain_id = Counterparty::client_state_chain_id(&client_state);
        let reference_chain = builder
            .build_chain(PhantomData::<Index<1>>, &reference_chain_id)
            .await
            .map_err(App::raise_error)?;

        let relayer = builder
            .build_relay(
                PhantomData::<(Index<0>, Index<1>)>,
                &host_chain_id,
                &reference_chain_id,
                &client_id,
                &counterparty_client_id,
                None, // client refresh not needed for this CLI
                None, // client refresh not needed for this CLI
            )
            .await
            .map_err(App::raise_error)?;

        let target_height = match target_height {
            Some(height) => {
                app.log(
                    &format!("Updating client using specified target height: {height}"),
                    &LevelInfo,
                )
                .await;
                height
            }
            None => {
                let height = reference_chain
                    .query_chain_height()
                    .await
                    .map_err(App::raise_error)?;

                app.log(
                    &format!("Updating client using specified target height: {height}"),
                    &LevelInfo,
                )
                .await;
                height
            }
        };

        relayer
            .send_target_update_client_messages(SourceTarget, &target_height)
            .await
            .map_err(App::raise_error)?;

        Ok(app.produce_output("Client successfully updated!"))
    }
}
