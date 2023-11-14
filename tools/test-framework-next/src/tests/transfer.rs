use async_trait::async_trait;
use cgp_core::{HasErrorType, Runner};
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::traits::chain::assert::CanAssertEventualAmount;
use crate::traits::chain::fields::amount::{
    CanGenerateRandomAmount, HasAmountMethods, HasIbcTransferredAmount,
};
use crate::traits::chain::fields::channel::HasChannel;
use crate::traits::chain::fields::denom::HasDenom;
use crate::traits::chain::fields::wallet::{HasOneUserWallet, HasTwoUserWallets};
use crate::traits::chain::queries::balance::CanQueryBalance;
use crate::traits::chain::queries::ibc_transfer::CanIbcTransferToken;
use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::chain::{HasChain, HasOneChain, HasTwoChains};

pub struct TestIbcTransfer;

#[async_trait]
impl<Test, ChainA, ChainB> Runner<Test> for TestIbcTransfer
where
    Test: HasErrorType + HasChain<0, Chain = ChainA> + HasChain<1, Chain = ChainB>,
    ChainA: HasIbcChainTypes<ChainB>
        + CanGenerateRandomAmount
        + HasAmountMethods
        + CanQueryBalance
        + CanIbcTransferToken<ChainB>
        + HasChannel<ChainB, 0>
        + HasTwoUserWallets
        + HasDenom<0>,
    ChainB: HasErrorType
        + HasIbcChainTypes<ChainA>
        + HasAddressType
        + CanAssertEventualAmount
        + HasIbcTransferredAmount<ChainA>
        + HasOneUserWallet
        + HasChannel<ChainA, 0>,
    Test::Error: From<ChainA::Error> + From<ChainB::Error>,
{
    async fn run(test: &Test) -> Result<(), Test::Error> {
        let chain_a = test.first_chain();

        let chain_b = test.second_chain();

        let wallet_a1 = chain_a.first_user_wallet();

        let address_a1 = ChainA::wallet_address(wallet_a1);

        let wallet_a2 = chain_a.second_user_wallet();

        let wallet_b = chain_b.first_user_wallet();

        let address_b = ChainB::wallet_address(wallet_b);

        let denom_a = chain_a.denom();

        let balance_a1 = chain_a.query_balance(address_a1, denom_a).await?;

        let a_to_b_amount = ChainA::random_amount(1000, &balance_a1);

        let channel_id_a = chain_a.channel_id();

        let port_id_a = chain_a.port_id();

        let channel_id_b = chain_b.channel_id();

        let port_id_b = chain_b.port_id();

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

        let balance_b1 = ChainB::ibc_transfer_amount_from(&a_to_b_amount, channel_id_b, port_id_b);

        chain_b
            .assert_eventual_amount(address_b, &balance_b1)
            .await?;

        let _address_a2 = ChainA::wallet_address(wallet_a2);

        Ok(())
    }
}
