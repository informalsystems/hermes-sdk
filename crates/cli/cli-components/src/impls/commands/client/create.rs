use core::fmt::{Debug, Display};
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_error::traits::wrap::CanWrapError;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::build::traits::builders::relay_builder::CanBuildRelay;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::relay::traits::chains::{HasRelayChains, HasRelayClientIds};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{HasSourceTargetChainTypes, SourceTarget};

use crate::traits::build::{BuilderOf, CanLoadBuilder, HasBuilderType};
use crate::traits::command::CommandRunner;
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

pub struct RunCreateClientCommand;

impl<App, Args, Builder, Chain, Counterparty, Relay> CommandRunner<App, Args>
    for RunCreateClientCommand
where
    App: CanLoadBuilder<Builder = Builder>
        + CanProduceOutput<Chain::ClientId>
        + HasLogger
        + CanParseCreateClientOptions<Args, Index<0>, Index<1>>
        + CanParseArg<Args, symbol!("target_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("counterparty_chain_id"), Parsed = Counterparty::ChainId>
        + CanRaiseError<Relay::Error>
        + CanRaiseError<Builder::Error>
        + CanWrapError<String>,
    Builder: CanBuildChain<Index<0>, Chain = Chain>
        + CanBuildChain<Index<1>, Chain = Counterparty>
        + CanBuildRelay<Index<0>, Index<1>, Relay = Relay>,
    Chain: HasIbcChainTypes<Counterparty>
        + HasCreateClientMessageOptionsType<Counterparty>
        + HasErrorType,
    Counterparty: HasChainIdType + HasCreateClientPayloadOptionsType<Chain> + HasErrorType,
    Relay: HasRelayChains<SrcChain = Chain, DstChain = Counterparty>
        + HasSourceTargetChainTypes
        + HasRelayClientIds
        + CanCreateClient<SourceTarget>,
    Args: Async,
    Chain::CreateClientMessageOptions: Debug,
    Counterparty::CreateClientPayloadOptions: Debug,
    App::Logger: CanLog<LevelInfo>,
    Chain::ClientId: Display,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let target_chain_id = app.parse_arg(args, PhantomData::<symbol!("target_chain_id")>)?;
        let counterparty_chain_id =
            app.parse_arg(args, PhantomData::<symbol!("counterparty_chain_id")>)?;

        let logger = app.logger();
        let builder = app.load_builder().await?;

        let target_chain = builder
            .build_chain(PhantomData::<Index<0>>, &target_chain_id)
            .await
            .map_err(App::raise_error)?;

        let counterparty_chain = builder
            .build_chain(PhantomData::<Index<1>>, &counterparty_chain_id)
            .await
            .map_err(App::raise_error)?;

        let (message_options, payload_options) = app
            .parse_create_client_options(args, &target_chain, &counterparty_chain)
            .await?;

        logger.log(
            &format!(
                "Creating client on target chain `{}` with counterparty chain `{}`. Create options: {:?}, {:?}",
                target_chain_id,
                counterparty_chain_id,
                message_options,
                payload_options,
            ),
            &LevelInfo).await;

        let client_id = Relay::create_client(
            SourceTarget,
            &target_chain,
            &counterparty_chain,
            &payload_options,
            &message_options,
        )
        .await
        .map_err(|e| {
            App::wrap_error(
                format!(
                    "Failed to create client on target chain {}",
                    target_chain_id
                ),
                App::raise_error(e),
            )
        })?;

        logger
            .log(
                &format!(
                    "Successfully created client {} on target chain `{}`",
                    client_id, target_chain_id
                ),
                &LevelInfo,
            )
            .await;

        Ok(app.produce_output(client_id))
    }
}

#[cgp_component {
  provider: CreateClientOptionsParser,
  context: App,
}]
#[async_trait]
pub trait CanParseCreateClientOptions<Args: Async, Target: Async, Counterparty: Async>:
    HasBuilderType<
        Builder: HasChainTypeAt<
            Target,
            Chain: HasCreateClientMessageOptionsType<ChainAt<BuilderOf<Self>, Counterparty>>,
        > + HasChainTypeAt<
            Counterparty,
            Chain: HasCreateClientPayloadOptionsType<ChainAt<BuilderOf<Self>, Target>>,
        >,
    > + HasErrorType
{
    async fn parse_create_client_options(
        &self,
        args: &Args,
        target_chain: &ChainAt<BuilderOf<Self>, Target>,
        counterparty_chain: &ChainAt<BuilderOf<Self>, Counterparty>,
    ) -> Result<
        (
            CreateClientMessageOptionsOf<
                ChainAt<BuilderOf<Self>, Target>,
                ChainAt<BuilderOf<Self>, Counterparty>,
            >,
            CreateClientPayloadOptionsOf<
                ChainAt<BuilderOf<Self>, Counterparty>,
                ChainAt<BuilderOf<Self>, Target>,
            >,
        ),
        Self::Error,
    >;
}
