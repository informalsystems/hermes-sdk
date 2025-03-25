use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::field::Index;
use cgp::core::macros::trait_alias;
use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLogMessage;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayAt;
use hermes_relayer_components::relay::traits::chains::{HasDstChain, HasSrcChain};
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::amount::CanConvertIbcTransferredAmount;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain::traits::types::amount::HasAmountMethods;
use hermes_test_components::chain::traits::types::memo::HasDefaultMemo;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::amount::CanGenerateRandomAmount;
use hermes_test_components::chain_driver::traits::fields::denom::{HasDenom, TransferDenom};
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWallet, UserWallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::driver::traits::channel_at::HasChannelIdAt;
use hermes_test_components::driver::traits::types::chain_driver_at::HasChainDriverAt;
use hermes_test_components::driver::traits::types::relay_driver_at::HasRelayDriverAt;
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::setup::traits::port_id_at::HasPortIdAt;

#[trait_alias]
pub trait HasTestDriverTypes<A, B>:
    HasChainDriverAt<A, ChainDriver = Self::ChainDriverA>
    + HasChainDriverAt<B, ChainDriver = Self::ChainDriverB>
    + HasRelayDriverAt<
        A,
        B,
        RelayDriver: CanRunRelayerInBackground
                         + HasBiRelayAt<Index<0>, Index<1>, BiRelay = Self::BiRelay>,
    > + HasChannelIdAt<A, B>
    + HasChannelIdAt<B, A>
    + HasPortIdAt<A, B>
    + HasPortIdAt<B, A>
    + HasLogger<Logger: CanLogMessage>
    + CanRaiseError<ErrorOf<Self::ChainA>>
    + CanRaiseError<ErrorOf<Self::ChainB>>
    + CanRaiseError<ErrorOf<Self::RelayAToB>>
    + CanRaiseError<ErrorOf<Self::RelayBToA>>
    + CanRaiseError<ErrorOf<Self::BiRelay>>
    + CanRaiseError<ErrorOf<Self::RelayDriver>>
    + CanRaiseError<ErrorOf<Self::ChainDriverA>>
    + CanRaiseError<ErrorOf<Self::ChainDriverB>>
{
    type ChainDriverA: HasErrorType
        + HasChain<Chain = Self::ChainA>
        + HasDenom<TransferDenom>
        + HasWallet<UserWallet<0>>
        + HasWallet<UserWallet<1>>
        + CanGenerateRandomAmount;

    type ChainDriverB: HasErrorType
        + HasChain<Chain = Self::ChainB>
        + HasDenom<TransferDenom>
        + HasWallet<UserWallet<0>>
        + HasWallet<UserWallet<1>>
        + CanGenerateRandomAmount;

    type ChainA: HasIbcChainTypes<Self::ChainB>
        + HasWalletType
        + CanQueryBalance
        + CanQueryChainStatus
        + HasAmountMethods
        + CanAssertEventualAmount
        + HasDefaultMemo
        + CanIbcTransferToken<Self::ChainB>
        + CanConvertIbcTransferredAmount<Self::ChainB>;

    type ChainB: HasIbcChainTypes<Self::ChainA>
        + HasWalletType
        + CanQueryBalance
        + CanQueryChainStatus
        + HasAmountMethods
        + CanAssertEventualAmount
        + HasDefaultMemo
        + CanIbcTransferToken<Self::ChainA>
        + CanConvertIbcTransferredAmount<Self::ChainA>;

    type RelayAToB: HasErrorType
        + HasSrcChain<SrcChain = Self::ChainA>
        + HasDstChain<DstChain = Self::ChainB>;

    type RelayBToA: HasErrorType
        + HasSrcChain<SrcChain = Self::ChainB>
        + HasDstChain<DstChain = Self::ChainA>;

    type BiRelay: HasErrorType
        + HasRelayAt<Index<0>, Index<1>, Relay = Self::RelayAToB>
        + HasRelayAt<Index<1>, Index<0>, Relay = Self::RelayBToA>;

    fn chain_driver_a(&self) -> &Self::ChainDriverA {
        self.chain_driver_at(PhantomData::<A>)
    }

    fn chain_a(&self) -> &Self::ChainA {
        self.chain_driver_a().chain()
    }

    fn chain_driver_b(&self) -> &Self::ChainDriverB {
        self.chain_driver_at(PhantomData::<B>)
    }

    fn chain_b(&self) -> &Self::ChainB {
        self.chain_driver_b().chain()
    }

    fn birelay(&self) -> &Self::BiRelay {
        self.relay_driver_at(PhantomData::<(A, B)>)
            .birelay_at(PhantomData)
    }

    fn relay_a_to_b(&self) -> &Self::RelayAToB {
        self.birelay().relay_at(PhantomData::<(Index<0>, Index<1>)>)
    }

    fn relay_b_to_a(&self) -> &Self::RelayBToA {
        self.birelay().relay_at(PhantomData::<(Index<1>, Index<0>)>)
    }
}
