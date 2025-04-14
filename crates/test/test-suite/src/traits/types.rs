use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::field::Index;
use cgp::core::macros::blanket_trait;
use cgp::prelude::{CanRaiseError, HasAsyncErrorType, HasErrorType};
use hermes_chain_type_components::traits::{DenomOf, HasAmountDenom};
use hermes_logging_components::traits::logger::CanLogMessage;
use hermes_relayer_components::birelay::traits::CanAutoBiRelay;
use hermes_relayer_components::chain::traits::{
    CanQueryChainStatus, CanQueryPacketIsCleared, CanQueryPacketIsReceived, CanReadPacketFields,
    HasChainId, HasIbcChainTypes,
};
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayAt;
use hermes_relayer_components::relay::traits::{
    CanAutoRelayWithHeights, CanRelayReceivePacket, DestinationTarget, HasChainTargets,
    HasDstChain, HasSrcChain, SourceTarget,
};
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::amount::CanConvertIbcTransferredAmount;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain::traits::types::amount::HasAmountMethods;
use hermes_test_components::chain::traits::types::memo::HasDefaultMemo;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::fields::amount::CanGenerateRandomAmount;
use hermes_test_components::chain_driver::traits::fields::denom::{HasDenom, TransferDenom};
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWallet, UserWallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::driver::traits::channel_at::HasChannelIdAt;
use hermes_test_components::driver::traits::types::chain_driver_at::HasChainDriverAt;
use hermes_test_components::driver::traits::types::relay_driver_at::HasRelayDriverAt;
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::setup::traits::port_id_at::HasPortIdAt;

#[blanket_trait]
pub trait HasBinaryTestDriverFields<A, B>:
    HasAsyncErrorType
    + HasChainTypeAt<A, Chain = Self::ChainA>
    + HasChainTypeAt<B, Chain = Self::ChainB>
    + HasChainDriverAt<A, ChainDriver = Self::ChainDriverA>
    + HasChainDriverAt<B, ChainDriver = Self::ChainDriverB>
    + HasRelayDriverAt<
        A,
        B,
        RelayDriver: HasErrorType + HasBiRelayAt<Index<0>, Index<1>, BiRelay = Self::BiRelay>,
    > + CanLogMessage
    + CanRaiseError<ErrorOf<Self::ChainA>>
    + CanRaiseError<ErrorOf<Self::ChainB>>
    + CanRaiseError<ErrorOf<Self::RelayAToB>>
    + CanRaiseError<ErrorOf<Self::RelayBToA>>
    + CanRaiseError<ErrorOf<Self::BiRelay>>
    + CanRaiseError<ErrorOf<Self::RelayDriver>>
    + CanRaiseError<ErrorOf<Self::ChainDriverA>>
    + CanRaiseError<ErrorOf<Self::ChainDriverB>>
{
    type ChainDriverA: HasErrorType + HasChain<Chain = Self::ChainA>;

    type ChainDriverB: HasErrorType + HasChain<Chain = Self::ChainB>;

    type ChainA: HasErrorType;

    type ChainB: HasErrorType;

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

    fn relay_driver(&self) -> &Self::RelayDriver {
        self.relay_driver_at(PhantomData)
    }

    fn birelay(&self) -> &Self::BiRelay {
        self.relay_driver().birelay_at(PhantomData)
    }

    fn relay_a_to_b(&self) -> &Self::RelayAToB {
        self.birelay().relay_at(PhantomData::<(Index<0>, Index<1>)>)
    }

    fn relay_b_to_a(&self) -> &Self::RelayBToA {
        self.birelay().relay_at(PhantomData::<(Index<1>, Index<0>)>)
    }
}

#[blanket_trait]
pub trait CanUseBinaryTestDriverMethods<A, B>:
    HasBinaryTestDriverFields<
        A,
        B,
        RelayDriver: CanRunRelayerInBackground,
        ChainDriverA: HasDenom<TransferDenom>
                          + HasWallet<UserWallet<0>>
                          + HasWallet<UserWallet<1>>
                          + CanGenerateRandomAmount,
        ChainDriverB: HasDenom<TransferDenom>
                          + HasWallet<UserWallet<0>>
                          + HasWallet<UserWallet<1>>
                          + CanGenerateRandomAmount,
        ChainA: HasChainId
                    + HasWalletType
                    + CanQueryBalance
                    + CanQueryChainStatus
                    + HasAmountMethods
                    + HasAmountDenom
                    + CanAssertEventualAmount
                    + HasDefaultMemo
                    + HasIbcChainTypes<Self::ChainB>
                    + CanReadPacketFields<Self::ChainB>
                    + CanQueryPacketIsCleared<Self::ChainB>
                    + CanQueryPacketIsReceived<Self::ChainB>
                    + CanIbcTransferToken<Self::ChainB>
                    + CanConvertIbcTransferredAmount<Self::ChainB>,
        ChainB: HasChainId
                    + HasWalletType
                    + CanQueryBalance
                    + CanQueryChainStatus
                    + HasAmountMethods
                    + HasAmountDenom
                    + CanAssertEventualAmount
                    + HasDefaultMemo
                    + HasIbcChainTypes<Self::ChainA>
                    + CanReadPacketFields<Self::ChainA>
                    + CanQueryPacketIsCleared<Self::ChainA>
                    + CanQueryPacketIsReceived<Self::ChainA>
                    + CanIbcTransferToken<Self::ChainA>
                    + CanConvertIbcTransferredAmount<Self::ChainA>,
        RelayAToB: HasChainTargets
                       + CanRelayReceivePacket
                       + CanAutoRelayWithHeights<SourceTarget>
                       + CanAutoRelayWithHeights<DestinationTarget>,
        RelayBToA: HasChainTargets
                       + CanRelayReceivePacket
                       + CanAutoRelayWithHeights<SourceTarget>
                       + CanAutoRelayWithHeights<DestinationTarget>,
        BiRelay: CanAutoBiRelay,
    > + HasChannelIdAt<A, B>
    + HasChannelIdAt<B, A>
    + HasPortIdAt<A, B>
    + HasPortIdAt<B, A>
{
    fn channel_id_a(&self) -> &ChannelIdOf<Self::ChainA, Self::ChainB> {
        self.channel_id_at(PhantomData::<(A, B)>)
    }

    fn channel_id_b(&self) -> &ChannelIdOf<Self::ChainB, Self::ChainA> {
        self.channel_id_at(PhantomData::<(B, A)>)
    }

    fn port_id_a(&self) -> &PortIdOf<Self::ChainA, Self::ChainB> {
        self.port_id_at(PhantomData::<(A, B)>)
    }

    fn port_id_b(&self) -> &PortIdOf<Self::ChainB, Self::ChainA> {
        self.port_id_at(PhantomData::<(B, A)>)
    }

    fn transfer_denom_a(&self) -> &DenomOf<Self::ChainA> {
        self.chain_driver_a().denom(PhantomData::<TransferDenom>)
    }

    fn transfer_denom_b(&self) -> &DenomOf<Self::ChainB> {
        self.chain_driver_b().denom(PhantomData::<TransferDenom>)
    }

    fn user_wallet_a1(&self) -> &WalletOf<Self::ChainA> {
        self.chain_driver_a().wallet(PhantomData::<UserWallet<0>>)
    }

    fn user_wallet_a2(&self) -> &WalletOf<Self::ChainA> {
        self.chain_driver_a().wallet(PhantomData::<UserWallet<1>>)
    }

    fn user_wallet_b1(&self) -> &WalletOf<Self::ChainB> {
        self.chain_driver_b().wallet(PhantomData::<UserWallet<0>>)
    }

    fn user_wallet_b2(&self) -> &WalletOf<Self::ChainB> {
        self.chain_driver_b().wallet(PhantomData::<UserWallet<1>>)
    }
}
