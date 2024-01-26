use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::address::HasAddressType;
use hermes_test_components::chain_driver::traits::types::amount::HasAmountType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::chain_driver::traits::types::wallet::HasWalletType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

use crate::bootstrap::traits::genesis::add_genesis_account::CanAddGenesisAccount;
use crate::bootstrap::traits::genesis::add_genesis_validator::CanAddGenesisValidator;
use crate::bootstrap::traits::genesis::add_genesis_wallet::GenesisWalletAdder;
use crate::bootstrap::traits::initializers::init_wallet::CanInitWallet;
use crate::bootstrap::traits::types::wallet_config::HasWalletConfigFields;

pub struct AddCosmosWalletToGenesis;

#[async_trait]
impl<Bootstrap, Runtime, Chain, ChainDriver> GenesisWalletAdder<Bootstrap>
    for AddCosmosWalletToGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasWalletConfigFields
        + HasErrorType
        + CanInitWallet
        + CanAddGenesisAccount
        + CanAddGenesisValidator,
    Runtime: HasFilePathType + HasErrorType,
    Chain: HasChainIdType,
    ChainDriver: HasWalletType + HasAmountType + HasAddressType,
{
    async fn add_wallet_to_genesis(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
        wallet_config: &Bootstrap::WalletConfig,
    ) -> Result<ChainDriver::Wallet, Bootstrap::Error> {
        let wallet_id = Bootstrap::wallet_config_wallet_id(wallet_config);

        let wallet = bootstrap
            .initialize_wallet(chain_home_dir, wallet_id)
            .await?;

        let address = ChainDriver::wallet_address(&wallet);

        let genesis_balance = Bootstrap::wallet_config_genesis_balances(wallet_config);

        bootstrap
            .add_genesis_account(chain_home_dir, address, genesis_balance)
            .await?;

        // If it is a validator wallet, add it as a validator given staked amount
        if let Some(stake_amount) = Bootstrap::wallet_config_validator_staked_amount(wallet_config)
        {
            bootstrap
                .add_genesis_validator(chain_home_dir, chain_id, wallet_id, stake_amount)
                .await?;
        }

        Ok(wallet)
    }
}
