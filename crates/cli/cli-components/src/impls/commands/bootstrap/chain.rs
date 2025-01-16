use core::marker::PhantomData;
use std::path::PathBuf;

use cgp::prelude::*;
use hermes_error::traits::wrap::CanWrapError;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::CanWaitChildProcess;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::chain_process::CanTakeChainProcess;
use hermes_test_components::chain_driver::traits::config::ConfigUpdater;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;

use crate::traits::bootstrap::CanLoadBootstrap;
use crate::traits::command::CommandRunner;
use crate::traits::config::config_path::HasConfigPath;
use crate::traits::config::load_config::CanLoadConfig;
use crate::traits::config::write_config::CanWriteConfig;
use crate::traits::output::CanProduceOutput;

pub struct RunBootstrapChainCommand<UpdateConfig>(pub PhantomData<UpdateConfig>);

impl<App, Args, Bootstrap, ChainDriver, Chain, Runtime, UpdateConfig> CommandRunner<App, Args>
    for RunBootstrapChainCommand<UpdateConfig>
where
    App: HasLogger
        + CanProduceOutput<()>
        + CanLoadBootstrap<Args, Bootstrap = Bootstrap>
        + HasConfigPath
        + CanLoadConfig
        + CanWriteConfig
        + CanRaiseAsyncError<Bootstrap::Error>
        + CanRaiseAsyncError<Runtime::Error>
        + CanRaiseAsyncError<ChainDriver::Error>
        + CanWrapError<&'static str>,
    Bootstrap: CanBootstrapChain<ChainDriver = ChainDriver>,
    ChainDriver: HasChain<Chain = Chain>
        + HasRuntime<Runtime = Runtime>
        + CanTakeChainProcess
        + HasAsyncErrorType,
    UpdateConfig: ConfigUpdater<ChainDriver, App::Config>,
    Chain: HasChainId,
    Runtime: CanWaitChildProcess + HasFilePathType<FilePath = PathBuf>,
    Args: Async + HasField<symbol!("chain_id"), Value = String>,
    App::Logger: CanLog<LevelInfo>,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let logger = app.logger();
        let chain_id = args.get_field(PhantomData);

        let mut config = app.load_config().await?;
        let bootstrap = app.load_bootstrap(args).await?;

        let mut chain_driver = bootstrap
            .bootstrap_chain(chain_id)
            .await
            .map_err(App::raise_error)?;

        logger
            .log(
                &format!(
                    "Bootstrapped a new chain with chain ID: {}",
                    chain_driver.chain().chain_id()
                ),
                &LevelInfo,
            )
            .await;

        let chain_config =
            UpdateConfig::update_config(&chain_driver, &mut config).map_err(App::raise_error)?;

        app.write_config(&config).await?;

        logger
            .log(
                &format!(
                    "Added the following chain config to the main config file:\n{}",
                    chain_config
                ),
                &LevelInfo,
            )
            .await;

        let m_chain_process = chain_driver.take_chain_process();

        if let Some(chain_process) = m_chain_process {
            logger.log(&format!("running chain {} running in the background. Press Ctrl+C to stop then chain...", chain_driver.chain().chain_id()), &LevelInfo).await;

            Runtime::wait_child_process(chain_process)
                .await
                .map_err(|e| {
                    App::wrap_error("chain process exited with error", App::raise_error(e))
                })?;

            logger
                .log("chain process exited with no error", &LevelInfo)
                .await;
        }

        Ok(app.produce_output(()))
    }
}
