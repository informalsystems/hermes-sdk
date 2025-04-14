use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::genesis::add_genesis_account::CanAddGenesisAccount;
use crate::bootstrap::traits::genesis::add_genesis_validator::CanAddGenesisValidator;
use crate::bootstrap::traits::genesis::add_genesis_wallet::{
    GenesisWalletAdder, GenesisWalletAdderComponent,
};
use crate::bootstrap::traits::initializers::init_wallet::CanInitWallet;
use crate::bootstrap::traits::types::wallet_config::HasWalletConfigFields;

pub struct AddCosmosWalletToGenesis;

#[cgp_provider(GenesisWalletAdderComponent)]
impl<Bootstrap, Runtime, Chain, ChainDriver> GenesisWalletAdder<Bootstrap>
    for AddCosmosWalletToGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasWalletConfigFields
        + HasAsyncErrorType
        + CanInitWallet
        + CanAddGenesisAccount
        + CanAddGenesisValidator,
    Runtime: HasFilePathType + HasAsyncErrorType,
    Chain: HasChainIdType + HasWalletType + HasAmountType + HasAddressType,
{
    async fn add_wallet_to_genesis(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
        wallet_config: &Bootstrap::WalletConfig,
    ) -> Result<Chain::Wallet, Bootstrap::Error> {
        let wallet_id = Bootstrap::wallet_config_wallet_id(wallet_config);

        let wallet = bootstrap
            .initialize_wallet(chain_home_dir, wallet_id)
            .await?;

        let address = Chain::wallet_address(&wallet);

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
