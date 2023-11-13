use async_trait::async_trait;
use cgp_core::{HasErrorType, Runner};
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::traits::binary::chain::HasTwoChains;
use crate::traits::binary::channel::HasTwoChannels;
use crate::traits::chain::assert::CanAssertEventualAmount;
use crate::traits::chain::fields::amount::{
    CanGenerateRandomAmount, HasAmountMethods, HasIbcTransferredAmount,
};
use crate::traits::chain::fields::denom::HasDenom;
use crate::traits::chain::fields::wallet::{HasUserWallet, HasWalletFields};
use crate::traits::chain::queries::balance::CanQueryBalance;
use crate::traits::chain::queries::ibc_transfer::CanIbcTransferToken;
use crate::traits::chain::types::address::HasAddressType;

pub struct TestIbcTransfer;

#[async_trait]
impl<Test, ChainA, ChainB> Runner<Test> for TestIbcTransfer
where
    Test: HasErrorType + HasTwoChains<ChainA = ChainA, ChainB = ChainB> + HasTwoChannels,
    ChainA: HasIbcChainTypes<ChainB>
        + CanGenerateRandomAmount
        + HasAmountMethods
        + CanQueryBalance
        + CanIbcTransferToken<ChainB>
        + HasWalletFields
        + HasUserWallet<0>
        + HasUserWallet<1>
        + HasDenom<0>,
    ChainB: HasErrorType
        + HasIbcChainTypes<ChainA>
        + HasAddressType
        + HasWalletFields
        + CanAssertEventualAmount
        + HasIbcTransferredAmount<ChainA>
        + HasUserWallet<0>,
    Test::Error: From<ChainA::Error> + From<ChainB::Error>,
{
    async fn run(test: &Test) -> Result<(), Test::Error> {
        let chain_a = test.chain_a();

        let chain_b = test.chain_b();

        let wallet_a1 = <ChainA as HasUserWallet<0>>::user_wallet(chain_a);

        let address_a1 = ChainA::wallet_address(wallet_a1);

        let wallet_a2 = <ChainA as HasUserWallet<1>>::user_wallet(chain_a);

        let wallet_b = chain_b.user_wallet();

        let address_b = ChainB::wallet_address(wallet_b);

        let denom_a = chain_a.denom();

        let balance_a1 = chain_a.query_balance(address_a1, denom_a).await?;

        let a_to_b_amount = ChainA::random_amount(denom_a, 1000, 5000);

        let channel_id_a = test.channel_id_a();

        let port_id_a = test.port_id_a();

        let channel_id_b = test.channel_id_b();

        let port_id_b = test.port_id_b();

        chain_a
            .ibc_transfer_token(
                channel_id_a,
                port_id_a,
                wallet_a1,
                address_b,
                &a_to_b_amount,
            )
            .await?;

        let balance_a2 = ChainA::subtract_amount(&balance_a1, &a_to_b_amount)?;

        let balance_a3 = chain_a.query_balance(address_a1, denom_a).await?;

        assert_eq!(balance_a2, balance_a3);

        let balance_b1 =
            ChainB::ibc_transfer_amount_from(&a_to_b_amount, &channel_id_b, &port_id_b);

        chain_b
            .assert_eventual_amount(address_b, &balance_b1)
            .await?;

        let _address_a2 = ChainA::wallet_address(wallet_a2);

        Ok(())
    }
}
