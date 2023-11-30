use cgp_core::prelude::*;
use ibc_test_components::traits::bootstrap::chain::ChainBootstrapper;

use crate::traits::commands::init_chain::CanRunInitChainCommand;
use crate::traits::generator::generate_chain_id::CanGenerateChainId;
use crate::traits::initializers::init_chain_home_dir::CanInitChainHomeDir;
use crate::traits::initializers::init_genesis_config::CanInitGenesisConfig;
use crate::traits::io::reserve_port::CanReserveTcpPort;

pub struct BoostrapCosmosChain;

#[async_trait]
impl<Bootstrap, Chain> ChainBootstrapper<Bootstrap, Chain> for BoostrapCosmosChain
where
    Bootstrap: HasErrorType
        + CanGenerateChainId
        + CanInitChainHomeDir
        + CanReserveTcpPort
        + CanRunInitChainCommand
        + CanInitGenesisConfig,
{
    async fn bootstrap_chain(
        bootstrap: &Bootstrap,
        chain_id_prefix: &str,
    ) -> Result<Chain, Bootstrap::Error> {
        let chain_id = bootstrap.generate_chain_id(chain_id_prefix).await;

        let chain_home_dir = bootstrap.init_chain_home_dir(chain_id).await?;

        // Run the `init` chain CLI subcommand to initialize the chain data files on the
        // given chain home directory.
        bootstrap
            .run_init_chain_command(chain_id, &chain_home_dir)
            .await?;

        bootstrap.init_genesis_config(&chain_home_dir).await?;

        let _rpc_port = bootstrap.reserve_tcp_port().await?;
        let _grpc_port = bootstrap.reserve_tcp_port().await?;
        let _grpc_web_port = bootstrap.reserve_tcp_port().await?;
        let _p2p_port = bootstrap.reserve_tcp_port().await?;
        let _pprof_port = bootstrap.reserve_tcp_port().await?;

        todo!()
    }
}
