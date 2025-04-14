use core::marker::PhantomData;
use std::path::PathBuf;

use cgp::prelude::*;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::chain::traits::HasChainId;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::CanWaitChildProcess;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::chain_process::CanTakeChainProcess;
use hermes_test_components::chain_driver::traits::config::ConfigUpdater;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;

use crate::traits::bootstrap::CanLoadBootstrap;
use crate::traits::command::{CommandRunner, CommandRunnerComponent};
use crate::traits::config::config_path::HasConfigPath;
use crate::traits::config::load_config::CanLoadConfig;
use crate::traits::config::write_config::CanWriteConfig;
use crate::traits::output::CanProduceOutput;

pub struct RunBootstrapChainCommand<Tag, UpdateConfig>(pub PhantomData<(Tag, UpdateConfig)>);

#[cgp_provider(CommandRunnerComponent)]
impl<App, Tag, Args, Bootstrap, ChainDriver, Chain, Runtime, UpdateConfig> CommandRunner<App, Args>
    for RunBootstrapChainCommand<Tag, UpdateConfig>
where
    App: CanProduceOutput<()>
        + CanLoadBootstrap<Tag, Args, Bootstrap = Bootstrap>
        + HasConfigPath
        + CanLoadConfig
        + CanWriteConfig
        + CanLog<LevelInfo>
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
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = args.get_field(PhantomData);

        let mut config = app.load_config().await?;
        let bootstrap = app.load_bootstrap(args).await?;

        let mut chain_driver = bootstrap
            .bootstrap_chain(chain_id)
            .await
            .map_err(App::raise_error)?;

        app.log(
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

        app.log(
            &format!(
                "Added the following chain config to the main config file:\n{}",
                chain_config
            ),
            &LevelInfo,
        )
        .await;

        let chain_processes = chain_driver.take_chain_process();

        if !chain_processes.is_empty() {
            app.log(
                &format!(
                    "running chain {} in the background. Press Ctrl+C to stop then chain...",
                    chain_driver.chain().chain_id()
                ),
                &LevelInfo,
            )
            .await;

            for chain_process in chain_processes {
                Runtime::wait_child_process(chain_process)
                    .await
                    .map_err(|e| {
                        App::wrap_error(App::raise_error(e), "chain process exited with error")
                    })?;
            }

            app.log("chain processes exited with no error", &LevelInfo)
                .await;
        }

        Ok(app.produce_output(()))
    }
}
