use std::io::Error as IoError;
use std::path::PathBuf;

use cgp::core::error::CanRaiseAsyncError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_runtime_components::traits::fs::copy_file::CanCopyFile;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use tokio::fs;

use crate::bootstrap::traits::import_bridge_key::BridgeKeyImporter;

pub struct CopyBridgeKey;

impl<Bootstrap, Chain, ChainDriver, Runtime> BridgeKeyImporter<Bootstrap> for CopyBridgeKey
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + CanRaiseAsyncError<Runtime::Error>
        + CanRaiseAsyncError<IoError>,
    Runtime: HasFilePathType<FilePath = PathBuf> + CanCopyFile,
    ChainDriver: HasChain<Chain = Chain> + HasRuntime<Runtime = Runtime> + HasChainHomeDir,
    Chain: HasChainId,
{
    async fn import_bridge_key(
        _bootstrap: &Bootstrap,
        bridge_home_dir: &Runtime::FilePath,
        chain_driver: &ChainDriver,
    ) -> Result<(), Bootstrap::Error> {
        let chain_home_dir = chain_driver.chain_home_dir();
        let chain_id = chain_driver.chain().chain_id();
        let chain_id_str = chain_id.to_string();

        let keyring_source_dir = chain_home_dir.join("keyring-test");
        let keyring_dest_dir =
            bridge_home_dir.join(format!(".celestia-bridge-{chain_id_str}/keys/keyring-test"));

        // We need to somehow share the bridge's private key from the chain keyring store to the
        // bridge keyring store. There are two files required: one is `bridge.info` and the other is
        // named after the hex of the raw address bytes. I haven't figured how to determine which is
        // the correct file to copy, and it is a bit tedious to copy all files, so we will just use a
        // symlink here as a quick hack.
        // TODO: properly copy over the bridge private key to the bridge keyring store.

        fs::remove_dir_all(&keyring_dest_dir)
            .await
            .map_err(Bootstrap::raise_error)?;
        fs::symlink(&keyring_source_dir, &keyring_dest_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
