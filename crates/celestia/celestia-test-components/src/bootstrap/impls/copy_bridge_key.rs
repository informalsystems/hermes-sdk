use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::copy_file::CanCopyFile;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

use crate::bootstrap::traits::import_bridge_key::BridgeKeyImporter;

pub struct CopyBridgeKey;

impl<Bootstrap, Chain, ChainDriver, Runtime> BridgeKeyImporter<Bootstrap> for CopyBridgeKey
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + CanRaiseError<Runtime::Error>,
    Runtime: HasFilePathType + CanCopyFile,
    ChainDriver: HasChain<Chain = Chain> + HasRuntime<Runtime = Runtime> + HasChainHomeDir,
    Chain: HasChainId,
{
    async fn import_bridge_key(
        bootstrap: &Bootstrap,
        bridge_home_dir: &Runtime::FilePath,
        chain_driver: &ChainDriver,
    ) -> Result<(), Bootstrap::Error> {
        let chain_home_dir = chain_driver.chain_home_dir();
        let chain_id = chain_driver.chain().chain_id();
        let chain_id_str = chain_id.to_string();

        let bridge_key_source_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("keyring-test/bridge.info"),
        );

        let bridge_key_destination_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string(&format!(
                ".celestia-bridge-{chain_id_str}/keys/keyring-test/bridge.info"
            )),
        );

        bootstrap
            .runtime()
            .copy_file(&bridge_key_source_path, &bridge_key_destination_path)
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
