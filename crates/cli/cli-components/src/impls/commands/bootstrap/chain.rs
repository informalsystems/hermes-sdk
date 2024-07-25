use core::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;

use crate::traits::bootstrap::CanLoadBootstrap;
use crate::traits::command::CommandRunner;
use crate::traits::output::CanProduceOutput;

pub struct RunBootstrapChainCommand;

impl<App, Args, Bootstrap> CommandRunner<App, Args> for RunBootstrapChainCommand
where
    App: HasLogger
        + CanProduceOutput<()>
        + CanLoadBootstrap<Args, Bootstrap = Bootstrap>
        + CanRaiseError<Bootstrap::Error>,
    Bootstrap: CanBootstrapChain<ChainDriver: HasChain<Chain: HasChainId>>,
    Args: Async + HasField<symbol!("chain_id"), Field = String>,
    App::Logger: CanLog<LevelInfo>,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let logger = app.logger();
        let chain_id = args.get_field(PhantomData);

        let bootstrap = app.load_bootstrap(args).await?;

        let chain_driver = bootstrap
            .bootstrap_chain(&chain_id)
            .await
            .map_err(App::raise_error)?;

        let chain = chain_driver.chain();

        logger.log(&format!("bootstrapped chain {} running in the background. Press Ctrl+C to stop then chain...", chain.chain_id()), &LevelInfo).await;

        Ok(app.produce_output(()))
    }
}
