use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_test_components::traits::chain::types::wallet::HasWalletType;

use crate::traits::genesis::add_genesis_account::CanAddGenesisAccount;
use crate::traits::genesis::add_genesis_validator::CanAddGenesisValidator;
use crate::traits::genesis::add_genesis_wallet::GenesisWalletAdder;
use crate::traits::initializers::init_wallet::CanInitWallet;
use crate::traits::types::io::file_path::HasFilePathType;
use crate::traits::types::wallet_config::HasWalletConfigFields;

pub struct AddCosmosWalletToGenesis;

#[async_trait]
impl<Bootstrap> GenesisWalletAdder<Bootstrap> for AddCosmosWalletToGenesis
where
    Bootstrap: HasFilePathType
        + HasChainIdType
        + HasWalletType
        + HasWalletConfigFields
        + HasErrorType
        + CanInitWallet
        + CanAddGenesisAccount
        + CanAddGenesisValidator,
{
    async fn add_wallet_to_genesis(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
        chain_id: &Bootstrap::ChainId,
        wallet_config: &Bootstrap::WalletConfig,
    ) -> Result<Bootstrap::Wallet, Bootstrap::Error> {
        let wallet_id = Bootstrap::wallet_config_wallet_id(wallet_config);

        let wallet = bootstrap
            .initialize_wallet(chain_home_dir, wallet_id)
            .await?;

        let address = Bootstrap::wallet_address(&wallet);

        let genesis_balance = Bootstrap::wallet_config_genesis_balance(wallet_config);

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
