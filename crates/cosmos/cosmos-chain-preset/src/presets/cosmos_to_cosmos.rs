#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_core::relayer_components::chain::impls::{
        QueryAndConvertRawClientState, QueryAndConvertRawConsensusState,
    };
    use hermes_core::relayer_components::chain::traits::{
        AckPacketMessageBuilderComponent, AllClientStatesQuerierComponent,
        ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent,
        ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent,
        ClientStateFieldsComponent, ClientStateQuerierComponent, ClientStateTypeComponent,
        ClientStateWithProofsQuerierComponent, ConnectionOpenAckMessageBuilderComponent,
        ConnectionOpenConfirmMessageBuilderComponent, ConnectionOpenInitMessageBuilderComponent,
        ConnectionOpenTryMessageBuilderComponent, ConsensusStateFieldComponent,
        ConsensusStateHeightsQuerierComponent, ConsensusStateQuerierComponent,
        ConsensusStateTypeComponent, ConsensusStateWithProofsQuerierComponent,
        CounterpartyMessageHeightGetterComponent, CreateClientMessageBuilderComponent,
        CreateClientMessageOptionsTypeComponent, CreateClientPayloadBuilderComponent,
        CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
        PacketDstChannelIdGetterComponent, PacketDstPortIdGetterComponent,
        PacketSequenceGetterComponent, PacketSrcChannelIdGetterComponent,
        PacketSrcPortIdGetterComponent, PacketTimeoutHeightGetterComponent,
        PacketTimeoutTimestampGetterComponent, ReceivePacketMessageBuilderComponent,
        TimeoutUnorderedPacketMessageBuilderComponent, UpdateClientMessageBuilderComponent,
        UpdateClientPayloadBuilderComponent, UpdateClientPayloadTypeComponent,
    };
    use hermes_core::test_components::chain::traits::IbcTransferredAmountConverterComponent;
    use hermes_cosmos_chain_components::impls::{
        BuildAnyCreateClientMessage, BuildCosmosChannelHandshakeMessage,
        BuildCosmosConnectionHandshakeMessage, BuildCosmosCreateClientPayload,
        BuildCosmosPacketMessages, BuildCosmosUpdateClientMessage,
        BuildTendermintUpdateClientPayload, CosmosPacketFieldReader,
        GetCosmosCounterpartyMessageHeight, ProvideCosmosCreateClientSettings,
        ProvideCosmosPayloadTypes, ProvideNoCreateClientMessageOptionsType,
        ProvideTendermintClientState, ProvideTendermintConsensusState,
        QueryConsensusStateHeightsFromGrpc,
    };
    use hermes_cosmos_test_components::chain::impls::ConvertCosmosIbcAmount;

    cgp_preset! {
        CosmosToCosmosComponents {
            [
                ClientStateTypeComponent,
                ClientStateFieldsComponent,
            ]:
                ProvideTendermintClientState,
            [
                ConsensusStateTypeComponent,
                ConsensusStateFieldComponent,
            ]:
                ProvideTendermintConsensusState,
            [
                CreateClientPayloadTypeComponent,
                UpdateClientPayloadTypeComponent,
            ]:
                ProvideCosmosPayloadTypes,
            CreateClientPayloadOptionsTypeComponent:
                ProvideCosmosCreateClientSettings,
            [
                ClientStateQuerierComponent,
                ClientStateWithProofsQuerierComponent,
                AllClientStatesQuerierComponent,
            ]:
                QueryAndConvertRawClientState,
            [
                ConsensusStateQuerierComponent,
                ConsensusStateWithProofsQuerierComponent,
            ]:
                QueryAndConvertRawConsensusState,
            CreateClientMessageOptionsTypeComponent:
                ProvideNoCreateClientMessageOptionsType,
            CreateClientMessageBuilderComponent:
                BuildAnyCreateClientMessage,
            UpdateClientMessageBuilderComponent:
                BuildCosmosUpdateClientMessage,
            CreateClientPayloadBuilderComponent:
                BuildCosmosCreateClientPayload,
            UpdateClientPayloadBuilderComponent:
                BuildTendermintUpdateClientPayload,
            [
                ConnectionOpenInitMessageBuilderComponent,
                ConnectionOpenTryMessageBuilderComponent,
                ConnectionOpenAckMessageBuilderComponent,
                ConnectionOpenConfirmMessageBuilderComponent,
            ]:
                BuildCosmosConnectionHandshakeMessage,
            [
                ChannelOpenInitMessageBuilderComponent,
                ChannelOpenTryMessageBuilderComponent,
                ChannelOpenAckMessageBuilderComponent,
                ChannelOpenConfirmMessageBuilderComponent,
            ]:
                BuildCosmosChannelHandshakeMessage,
            ConsensusStateHeightsQuerierComponent:
                QueryConsensusStateHeightsFromGrpc,
            CounterpartyMessageHeightGetterComponent:
                GetCosmosCounterpartyMessageHeight,
            [
                PacketSrcChannelIdGetterComponent,
                PacketSrcPortIdGetterComponent,
                PacketDstChannelIdGetterComponent,
                PacketDstPortIdGetterComponent,
                PacketSequenceGetterComponent,
                PacketTimeoutHeightGetterComponent,
                PacketTimeoutTimestampGetterComponent,
            ]:
                CosmosPacketFieldReader,
            [
                ReceivePacketMessageBuilderComponent,
                AckPacketMessageBuilderComponent,
                TimeoutUnorderedPacketMessageBuilderComponent,
            ]:
                BuildCosmosPacketMessages,

            IbcTransferredAmountConverterComponent:
                ConvertCosmosIbcAmount,
        }
    }
}
