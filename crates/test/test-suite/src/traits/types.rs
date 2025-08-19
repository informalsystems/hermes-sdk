use alloc::string::String;
use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::field::Index;
use cgp::core::macros::blanket_trait;
use hermes_chain_components::traits::{
    CanBuildCreateClientMessage, CanBuildCreateClientPayload, CanBuildUpdateClientMessage,
    CanBuildUpdateClientPayload, CanExtractFromMessageResponse,
    CanOverrideCreateClientPayloadOptions, CanQueryClientStateWithLatestHeight,
    CanQueryClientStatus, CanRecoverClient, CanSendMessages, CanSendSingleMessage,
    HasClientStateFields, HasClientStateType, HasClientStatusMethods, HasClientStatusType,
    HasCreateClientEvent, HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
    HasRecoverClientPayloadType,
};
use hermes_chain_components::types::aliases::ClientIdOf;
use hermes_chain_type_components::traits::{DenomOf, HasAmountDenom};
use hermes_logging_components::traits::CanLogMessage;
use hermes_prelude::{CanRaiseError, HasAsyncErrorType, HasErrorType};
use hermes_relayer_components::birelay::traits::CanAutoBiRelay;
use hermes_relayer_components::chain::traits::{
    CanQueryChainStatus, CanQueryPacketIsCleared, CanQueryPacketIsReceived, CanReadPacketFields,
    HasChainId, HasIbcChainTypes,
};
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::client_id_at::HasClientIdAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayAt;
use hermes_relayer_components::relay::traits::{
    CanAutoRelayWithHeights, CanRelayReceivePacket, DestinationTarget, HasChainTargets,
    HasDstChain, HasSrcChain, SourceTarget,
};
use hermes_test_components::chain::traits::{
    CanAssertEventualAmount, CanConvertIbcTransferredAmount, CanIbcTransferToken, CanQueryBalance,
    HasAmountMethods, HasDefaultMemo, HasWalletSigner, HasWalletType, WalletOf,
};
use hermes_test_components::chain_driver::traits::{
    CanGenerateRandomAmount, HasChain, HasDenom, HasSetupUpgradeClientTestResultType, HasWallet,
    StakingDenom, TransferDenom, UserWallet,
};
use hermes_test_components::driver::traits::{HasChainDriverAt, HasChannelIdAt, HasRelayDriverAt};
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::setup::traits::{
    HasCreateClientMessageOptionsAt, HasCreateClientPayloadOptionsAt, HasPortIdAt,
    HasRecoverClientPayloadOptionsAt,
};
use hermes_test_components::test_case::traits::node::CanHaltFullNode;

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
    + CanRaiseError<String>
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
                          + HasDenom<StakingDenom>
                          + HasWallet<UserWallet<0>>
                          + HasWallet<UserWallet<1>>
                          + CanGenerateRandomAmount
                          + HasSetupUpgradeClientTestResultType
                          + CanHaltFullNode,
        ChainDriverB: HasDenom<TransferDenom>
                          + HasDenom<StakingDenom>
                          + HasWallet<UserWallet<0>>
                          + HasWallet<UserWallet<1>>
                          + CanGenerateRandomAmount
                          + HasSetupUpgradeClientTestResultType
                          + CanHaltFullNode,
        ChainA: HasChainId
                    + HasWalletType
                    + HasWalletSigner
                    + CanQueryBalance
                    + CanQueryChainStatus
                    + HasAmountMethods
                    + HasAmountDenom
                    + CanAssertEventualAmount
                    + HasDefaultMemo
                    + CanSendSingleMessage
                    + CanSendMessages
                    + HasCreateClientPayloadOptionsType<Self::ChainB>
                    + CanBuildCreateClientPayload<Self::ChainB>
                    + CanBuildCreateClientMessage<Self::ChainB>
                    + CanRecoverClient<Self::ChainB>
                    + HasCreateClientEvent<Self::ChainB>
                    + CanOverrideCreateClientPayloadOptions<Self::ChainB>
                    + HasClientStateType<Self::ChainB>
                    + HasClientStatusType<Self::ChainB>
                    + HasClientStatusMethods<Self::ChainB>
                    + CanQueryClientStateWithLatestHeight<Self::ChainB>
                    + CanQueryClientStatus<Self::ChainB>
                    + HasClientStateFields<Self::ChainB>
                    + CanBuildUpdateClientPayload<Self::ChainB>
                    + CanBuildUpdateClientMessage<Self::ChainB>
                    + CanExtractFromMessageResponse<
            <Self::ChainA as HasCreateClientEvent<Self::ChainB>>::CreateClientEvent,
        > + HasIbcChainTypes<Self::ChainB>
                    + CanReadPacketFields<Self::ChainB>
                    + CanQueryPacketIsCleared<Self::ChainB>
                    + CanQueryPacketIsReceived<Self::ChainB>
                    + CanIbcTransferToken<Self::ChainB>
                    + CanConvertIbcTransferredAmount<Self::ChainB>,
        ChainB: HasChainId
                    + HasWalletType
                    + HasWalletSigner
                    + CanQueryBalance
                    + CanQueryChainStatus
                    + HasAmountMethods
                    + HasAmountDenom
                    + CanAssertEventualAmount
                    + HasDefaultMemo
                    + CanSendSingleMessage
                    + CanSendMessages
                    + HasCreateClientPayloadOptionsType<Self::ChainA>
                    + CanBuildCreateClientPayload<Self::ChainA>
                    + CanBuildCreateClientMessage<Self::ChainA>
                    + CanRecoverClient<Self::ChainA>
                    + HasCreateClientEvent<Self::ChainA>
                    + CanOverrideCreateClientPayloadOptions<Self::ChainA>
                    + HasClientStateType<Self::ChainA>
                    + HasClientStatusType<Self::ChainA>
                    + HasClientStatusMethods<Self::ChainA>
                    + CanQueryClientStateWithLatestHeight<Self::ChainA>
                    + CanQueryClientStatus<Self::ChainA>
                    + HasClientStateFields<Self::ChainA>
                    + CanBuildUpdateClientPayload<Self::ChainA>
                    + CanBuildUpdateClientMessage<Self::ChainA>
                    + CanExtractFromMessageResponse<
            <Self::ChainB as HasCreateClientEvent<Self::ChainA>>::CreateClientEvent,
        > + HasIbcChainTypes<Self::ChainA>
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
    + HasClientIdAt<A, B>
    + HasClientIdAt<B, A>
    + HasCreateClientPayloadOptionsAt<A, B>
    + HasCreateClientPayloadOptionsAt<B, A>
    + HasCreateClientMessageOptionsAt<A, B>
    + HasCreateClientMessageOptionsAt<B, A>
    + HasRecoverClientPayloadOptionsAt<A>
    + HasRecoverClientPayloadOptionsAt<B>
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

    fn client_id_a(&self) -> &ClientIdOf<Self::ChainA, Self::ChainB> {
        self.client_id_at(PhantomData::<(A, B)>)
    }

    fn client_id_b(&self) -> &ClientIdOf<Self::ChainB, Self::ChainA> {
        self.client_id_at(PhantomData::<(B, A)>)
    }

    fn transfer_denom_a(&self) -> &DenomOf<Self::ChainA> {
        self.chain_driver_a().denom(PhantomData::<TransferDenom>)
    }

    fn transfer_denom_b(&self) -> &DenomOf<Self::ChainB> {
        self.chain_driver_b().denom(PhantomData::<TransferDenom>)
    }

    fn staking_denom_a(&self) -> &DenomOf<Self::ChainA> {
        self.chain_driver_a().denom(PhantomData::<StakingDenom>)
    }

    fn staking_denom_b(&self) -> &DenomOf<Self::ChainB> {
        self.chain_driver_b().denom(PhantomData::<StakingDenom>)
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

    fn create_client_payload_options_a_to_b(&self) -> &<Self::ChainA as HasCreateClientPayloadOptionsType<Self::ChainB>>::CreateClientPayloadOptions{
        self.create_client_payload_options(PhantomData::<(A, B)>)
    }

    fn create_client_payload_options_b_to_a(&self) -> &<Self::ChainB as HasCreateClientPayloadOptionsType<Self::ChainA>>::CreateClientPayloadOptions{
        self.create_client_payload_options(PhantomData::<(B, A)>)
    }

    fn create_client_message_options_a_to_b(&self) -> &<Self::ChainA as HasCreateClientMessageOptionsType<Self::ChainB>>::CreateClientMessageOptions{
        self.create_client_message_options(PhantomData::<(A, B)>)
    }

    fn recover_client_payload_options_a(
        &self,
    ) -> &<Self::ChainA as HasRecoverClientPayloadType>::RecoverClientPayload {
        self.recover_client_payload_options(PhantomData::<A>)
    }

    fn recover_client_payload_options_b(
        &self,
    ) -> &<Self::ChainB as HasRecoverClientPayloadType>::RecoverClientPayload {
        self.recover_client_payload_options(PhantomData::<B>)
    }
}
