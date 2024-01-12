use cgp_core::prelude::*;
use hermes_test_components::chain::traits::types::denom::HasDenomType;

use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::impls::fields::denom::{DenomForStaking, DenomForTransfer, HasGenesisDenom};
use crate::bootstrap::traits::generator::generate_wallet_config::WalletConfigGenerator;
use crate::bootstrap::traits::types::genesis_config::HasGenesisConfigType;
use crate::bootstrap::traits::types::wallet_config::HasWalletConfigType;
use crate::bootstrap::types::wallet_config::CosmosWalletConfig;
use crate::chain::types::amount::Amount;
use crate::chain::types::denom::Denom;

/**
   Generator for standard sets of wallets for testing. Consists of one validator wallet,
   two user wallets, and one relayer wallet.

   If a bootstrap context requires custom generation of test wallets, one can implement
   a custom `WalletConfigGenerator` that works similar to this component.
*/
pub struct GenerateStandardWalletConfig;

#[async_trait]
impl<Bootstrap, ChainDriver> WalletConfigGenerator<Bootstrap> for GenerateStandardWalletConfig
where
    Bootstrap: HasWalletConfigType<WalletConfig = CosmosWalletConfig>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasGenesisConfigType
        + HasErrorType
        + HasGenesisDenom<DenomForStaking>
        + HasGenesisDenom<DenomForTransfer>,
    ChainDriver: HasDenomType<Denom = Denom>,
{
    async fn generate_wallet_configs(
        _bootstrap: &Bootstrap,
        genesis_config: &Bootstrap::GenesisConfig,
    ) -> Result<Vec<CosmosWalletConfig>, Bootstrap::Error> {
        // TODO: allow for randomization of denoms and amount

        let denom_for_staking =
            <Bootstrap as HasGenesisDenom<DenomForStaking>>::genesis_denom(genesis_config);

        let denom_for_transfer =
            <Bootstrap as HasGenesisDenom<DenomForTransfer>>::genesis_denom(genesis_config);

        let validator = CosmosWalletConfig {
            wallet_id: "validator".to_owned(),
            genesis_balances: vec![
                Amount::new(2_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: Some(Amount::new(
                1_000_000_000_000_000_000,
                denom_for_staking.clone(),
            )),
        };

        let user1 = CosmosWalletConfig {
            wallet_id: "user1".to_owned(),
            genesis_balances: vec![
                Amount::new(1_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        let user2 = CosmosWalletConfig {
            wallet_id: "user2".to_owned(),
            genesis_balances: vec![
                Amount::new(1_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        let relayer = CosmosWalletConfig {
            wallet_id: "relayer".to_owned(),
            genesis_balances: vec![
                Amount::new(1_000_000_000_000_000_000, denom_for_staking.clone()),
                Amount::new(1_000_000_000_000_000_000, denom_for_transfer.clone()),
            ],
            validator_staked_amount: None,
        };

        Ok(vec![validator, user1, user2, relayer])
    }
}
