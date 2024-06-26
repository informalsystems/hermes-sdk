use core::time::Duration;

use cgp_core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain_driver::traits::rpc_port::HasRpcPort;
use crate::chain_driver::traits::store_wasm_client::WasmClientCodeUploader;

pub struct UploadWasmClientCodeWithChainCommand;

impl<ChainDriver, Chain, Runtime> WasmClientCodeUploader<ChainDriver>
    for UploadWasmClientCodeWithChainCommand
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + HasChain<Chain = Chain>
        + CanRaiseError<Runtime::Error>
        + HasChainCommandPath
        + HasChainHomeDir
        + HasRpcPort,
    Runtime: CanExecCommand + CanWriteStringToFile + CanSleep,
    Chain: HasChainId,
{
    async fn store_wasm_client_code(
        chain_driver: &ChainDriver,
        wasm_client_code_path: &Runtime::FilePath,
        title: &str,
        summary: &str,
        sender: &str,
    ) -> Result<String, ChainDriver::Error> {
        let runtime = chain_driver.runtime();

        let output = runtime
            .exec_command(
                chain_driver.chain_command_path(),
                &[
                    "tx",
                    "ibc-wasm",
                    "store-code",
                    &Runtime::file_path_to_string(wasm_client_code_path),
                    "--title",
                    title,
                    "--summary",
                    summary,
                    "--chain-id",
                    &chain_driver.chain().chain_id().to_string(),
                    "--node",
                    &format!("tcp://localhost:{}", chain_driver.rpc_port()),
                    "--home",
                    &Runtime::file_path_to_string(chain_driver.chain_home_dir()),
                    "--from",
                    sender,
                    "--keyring-backend",
                    "test",
                    "--gas",
                    "auto",
                    "--fees",
                    "1000016stake",
                    "--deposit",
                    "200000stake",
                    "-y",
                ],
            )
            .await
            .map_err(ChainDriver::raise_error)?;

        // Wait for the governance proposal to be created
        runtime.sleep(Duration::from_secs(1)).await;

        Ok(output.stdout)
    }
}
