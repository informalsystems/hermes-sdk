use core::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::types::index::Index;

use crate::traits::any_counterparty::HasAnyCounterparty;
use crate::traits::build::{CanLoadBuilder, HasBuilderType};
use crate::traits::command::CommandRunner;
use crate::traits::output::CanShowOutput;

pub struct RunQueryClientState;

#[derive(HasField)]
pub struct QueryClientStateArgs<Chain, Counterparty>
where
    Chain: HasChainIdType + HasHeightType + HasIbcChainTypes<Counterparty>,
{
    pub chain_id: Chain::ChainId,
    pub client_id: Chain::ClientId,
    pub height: Option<Chain::Height>,
}

impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args> for RunQueryClientState
where
    App: HasBuilderType<Builder = Build>
        + CanLoadBuilder
        + HasLogger
        + HasAnyCounterparty<AnyCounterparty = Counterparty>
        + CanShowOutput<Counterparty::ClientState>
        + CanRaiseError<Build::Error>
        + CanRaiseError<Chain::Error>,
    Args: Async
        + HasField<symbol!("chain_id"), Field = Chain::ChainId>
        + HasField<symbol!("client_id"), Field = Chain::ClientId>
        + HasField<symbol!("height"), Field = Option<Chain::Height>>,
    Build: CanBuildChain<0, Chain = Chain>,
    Chain: HasChainIdType + CanQueryChainHeight + CanQueryClientState<Counterparty>,
    Counterparty: HasClientStateType<Chain>,
    App::Logger: CanLog<LevelInfo>,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let logger = app.logger();
        let builder = app.load_builder().await?;

        let chain_id = args.get_field(PhantomData::<symbol!("chain_id")>);
        let client_id = args.get_field(PhantomData::<symbol!("client_id")>);
        let m_height = args.get_field(PhantomData::<symbol!("height")>);

        let chain = builder
            .build_chain(Index::<0>, chain_id)
            .await
            .map_err(App::raise_error)?;

        let query_height = match m_height {
            Some(height) => height,
            None => &chain.query_chain_height().await.map_err(App::raise_error)?,
        };

        let client_state = chain
            .query_client_state(client_id, query_height)
            .await
            .map_err(App::raise_error)?;

        logger
            .log(
                &format!("Found client state for client `{client_id}` on chain `{chain_id}`!"),
                &LevelInfo,
            )
            .await;

        Ok(app.show_output(client_state))
    }
}
