use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::address::HasAddressType;
use ibc_test_components::chain::traits::types::amount::HasAmountType;
use ibc_test_components::chain::traits::types::wallet::HasWalletType;

use crate::traits::genesis::add_genesis_account::CanAddGenesisAccount;
use crate::traits::genesis::add_genesis_validator::CanAddGenesisValidator;
use crate::traits::genesis::add_genesis_wallet::GenesisWalletAdder;
use crate::traits::initializers::init_wallet::CanInitWallet;
use crate::traits::types::wallet_config::HasWalletConfigFields;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;

pub struct AddCosmosWalletToGenesis;

#[async_trait]
impl<Bootstrap, Runtime, Chain> GenesisWalletAdder<Bootstrap> for AddCosmosWalletToGenesis
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasWalletConfigFields
        + HasErrorType
        + CanInitWallet
        + CanAddGenesisAccount
        + CanAddGenesisValidator,
    Runtime: HasFilePathType,
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
