use alloc::collections::BTreeMap;
use cgp_core::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::HasAccountPrefix;
use hermes_test_components::chain_driver::traits::types::wallet::HasWalletType;

use crate::bootstrap::traits::generate_rollup_wallets::RollupWalletGenerator;
use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use crate::types::wallet::SovereignWallet;

pub struct GenerateSovereignRollupWallets;

impl<Bootstrap, RollupDriver> RollupWalletGenerator<Bootstrap> for GenerateSovereignRollupWallets
where
    Bootstrap: HasRollupDriverType<RollupDriver = RollupDriver>
        + HasAccountPrefix
        + CanRaiseError<bech32::Error>,
    RollupDriver: HasWalletType<Wallet = SovereignWallet>,
{
    async fn generate_rollup_wallets(
        bootstrap: &Bootstrap,
    ) -> Result<BTreeMap<String, SovereignWallet>, Bootstrap::Error> {
        let account_prefix = bootstrap.account_prefix();
        let wallet_ids = ["sequencer", "relayer", "user-a", "user-b"];

        let wallets = wallet_ids
            .iter()
            .map(|wallet_id| {
                let wallet = SovereignWallet::generate(wallet_id, account_prefix)?;
                Ok((wallet_id.to_string(), wallet))
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(Bootstrap::raise_error)?;

        Ok(BTreeMap::from_iter(wallets))
    }
}
