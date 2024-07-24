use core::marker::PhantomData;

use cgp_core::prelude::*;
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
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::SourceTarget;

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
        + CanParseCreateClientOptions<Args, 0, 1>
        + CanParseArg<Args, symbol!("target_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("counterparty_chain_id"), Parsed = Counterparty::ChainId>
        + CanRaiseError<Relay::Error>
        + CanRaiseError<Builder::Error>,
    Builder: CanBuildChain<0, Chain = Chain>
        + CanBuildChain<1, Chain = Counterparty>
        + CanBuildRelay<0, 1, Relay = Relay>,
    Chain: HasIbcChainTypes<Counterparty>
        + HasCreateClientMessageOptionsType<Counterparty>
        + HasErrorType,
    Counterparty: HasChainIdType + HasCreateClientPayloadOptionsType<Chain> + HasErrorType,
    Relay:
        HasRelayChains<SrcChain = Chain, DstChain = Counterparty> + CanCreateClient<SourceTarget>,
    Args: Async,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let target_chain_id = app.parse_arg(args, PhantomData::<symbol!("target_chain_id")>)?;
        let counterparty_chain_id =
            app.parse_arg(args, PhantomData::<symbol!("counterparty_chain_id")>)?;

        let builder = app.load_builder().await?;

        let target_chain = builder
            .build_chain(Index::<0>, &target_chain_id)
            .await
            .map_err(App::raise_error)?;

        let counterparty_chain = builder
            .build_chain(Index::<1>, &counterparty_chain_id)
            .await
            .map_err(App::raise_error)?;

        let (message_options, payload_options) = app
            .parse_create_client_options(args, &target_chain, &counterparty_chain)
            .await?;

        let client_id = Relay::create_client(
            SourceTarget,
            &target_chain,
            &counterparty_chain,
            &payload_options,
            &message_options,
        )
        .await
        .map_err(App::raise_error)?;

        Ok(app.produce_output(client_id))
    }
}

#[derive_component(CreateClientOptionsParserComponent, CreateClientOptionsParser<App>)]
#[async_trait]
pub trait CanParseCreateClientOptions<Args: Async, const TARGET: usize, const COUNTERPARTY: usize>:
    HasBuilderType<
        Builder: HasChainTypeAt<
            TARGET,
            Chain: HasCreateClientMessageOptionsType<ChainAt<BuilderOf<Self>, COUNTERPARTY>>,
        > + HasChainTypeAt<
            COUNTERPARTY,
            Chain: HasCreateClientPayloadOptionsType<ChainAt<BuilderOf<Self>, TARGET>>,
        >,
    > + HasErrorType
{
    async fn parse_create_client_options(
        &self,
        args: &Args,
        target_chain: &ChainAt<BuilderOf<Self>, TARGET>,
        counterparty_chain: &ChainAt<BuilderOf<Self>, COUNTERPARTY>,
    ) -> Result<
        (
            CreateClientMessageOptionsOf<
                ChainAt<BuilderOf<Self>, TARGET>,
                ChainAt<BuilderOf<Self>, COUNTERPARTY>,
            >,
            CreateClientPayloadOptionsOf<
                ChainAt<BuilderOf<Self>, COUNTERPARTY>,
                ChainAt<BuilderOf<Self>, TARGET>,
            >,
        ),
        Self::Error,
    >;
}
