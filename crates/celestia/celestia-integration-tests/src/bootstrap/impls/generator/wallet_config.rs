use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::types::denom::HasDenomType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_cosmos_test_components::bootstrap::impls::fields::denom::{DenomForStaking, DenomForTransfer, HasGenesisDenom};
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGenerator;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasGenesisConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::wallet_config::HasWalletConfigType;
use hermes_cosmos_test_components::bootstrap::types::wallet_config::CosmosWalletConfig;
use hermes_cosmos_test_components::chain_driver::types::amount::Amount;
use hermes_cosmos_test_components::chain_driver::types::denom::Denom;

pub struct GenerateCelestiaWalletConfig;

#[async_trait]
impl<Bootstrap, ChainDriver> WalletConfigGenerator<Bootstrap> for GenerateCelestiaWalletConfig
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
        bootstrap: &Bootstrap,
        genesis_config: &Bootstrap::GenesisConfig,
    ) -> Result<Vec<CosmosWalletConfig>, Bootstrap::Error> {
        let denom_for_staking = bootstrap.genesis_denom(DenomForStaking, genesis_config);

        let denom_for_transfer = bootstrap.genesis_denom(DenomForTransfer, genesis_config);

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

        let bridge = CosmosWalletConfig {
            wallet_id: "bridge".to_owned(),
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

        Ok(vec![validator, bridge, user1, user2, relayer])
    }
}
