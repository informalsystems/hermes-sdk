use async_trait::async_trait;
use cgp_core::{HasErrorType, Runner};
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use tracing::info;

use crate::traits::chain::assert::eventual_amount::CanAssertEventualAmount;
use crate::traits::chain::fields::amount::{
    CanConvertIbcTransferredAmount, CanGenerateRandomAmount, HasAmountMethods,
};
use crate::traits::chain::fields::channel::HasChannel;
use crate::traits::chain::fields::denom::HasDenom;
use crate::traits::chain::fields::wallet::{HasOneUserWallet, HasTwoUserWallets};
use crate::traits::chain::queries::balance::CanQueryBalance;
use crate::traits::chain::queries::ibc_transfer::CanIbcTransferToken;
use crate::traits::chain::types::chain::{HasChain, HasOneChain, HasTwoChains};

pub struct TestIbcTransfer;

#[async_trait]
impl<Test, ChainA, ChainB> Runner<Test> for TestIbcTransfer
where
    Test: HasErrorType + HasChain<0, Chain = ChainA> + HasChain<1, Chain = ChainB>,
    ChainA: HasIbcChainTypes<ChainB>
        + HasChannel<ChainB, 0>
        + HasDenom<0>
        + HasChainId
        + HasTwoUserWallets
        + HasAmountMethods
        + CanQueryBalance
        + CanGenerateRandomAmount
        + CanAssertEventualAmount
        + CanIbcTransferToken<ChainB>
        + CanConvertIbcTransferredAmount<ChainB>,
    ChainB: HasIbcChainTypes<ChainA>
        + HasChannel<ChainA, 0>
        + HasChainId
        + HasOneUserWallet
        + HasAmountMethods
        + CanQueryBalance
        + CanGenerateRandomAmount
        + CanAssertEventualAmount
        + CanIbcTransferToken<ChainA>
        + CanConvertIbcTransferredAmount<ChainA>,
    Test::Error: From<ChainA::Error> + From<ChainB::Error>,
{
    async fn run(test: &Test) -> Result<(), Test::Error> {
        let chain_a = test.first_chain();

        let chain_id_a = chain_a.chain_id();

        let chain_b = test.second_chain();

        let chain_id_b = chain_b.chain_id();

        let wallet_a1 = chain_a.first_user_wallet();

        let address_a1 = ChainA::wallet_address(wallet_a1);

        let wallet_b = chain_b.first_user_wallet();

        let address_b = ChainB::wallet_address(wallet_b);

        let denom_a = chain_a.denom();

        let balance_a1 = chain_a.query_balance(address_a1, denom_a).await?;

        let a_to_b_amount = ChainA::random_amount(1000, &balance_a1);

        let channel_id_a = chain_a.channel_id();

        let port_id_a = chain_a.port_id();

        let channel_id_b = chain_b.channel_id();

        let port_id_b = chain_b.port_id();

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chain_id_a, chain_id_b, a_to_b_amount, denom_a
        );

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

        info!(
            "Waiting for user on chain B to receive IBC transferred amount of {}",
            balance_b1
        );

        chain_b
            .assert_eventual_amount(address_b, &balance_b1)
            .await?;

        let wallet_a2 = chain_a.second_user_wallet();

        let address_a2 = ChainA::wallet_address(wallet_a2);

        let b_to_a_amount = ChainB::random_amount(500, &balance_b1);

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {}",
            chain_id_b, chain_id_a, b_to_a_amount,
        );

        chain_b
            .ibc_transfer_token(
                channel_id_b,
                port_id_b,
                wallet_b,
                address_a2,
                &b_to_a_amount,
            )
            .await?;

        let balance_b2 = ChainB::subtract_amount(&balance_b1, &b_to_a_amount)?;

        let denom_b = ChainB::amount_denom(&balance_b1);

        let balance_b3 = chain_b.query_balance(&address_b, denom_b).await?;

        assert_eq!(balance_b2, balance_b3);

        let balance_a4 = chain_a.query_balance(address_a2, denom_a).await?;

        let balance_a5 = ChainA::add_amount(
            &balance_a4,
            &ChainA::transmute_counterparty_amount(&b_to_a_amount, denom_a),
        )?;

        chain_a.assert_eventual_amount(address_a2, &balance_a5);

        info!(
            "successfully performed reverse IBC transfer from chain {} back to chain {}",
            chain_id_b, chain_id_a,
        );

        Ok(())
    }
}
