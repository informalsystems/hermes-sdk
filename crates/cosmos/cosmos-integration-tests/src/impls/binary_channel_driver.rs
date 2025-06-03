use cgp::core::field::Index;
use hermes_core::relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_core::relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_core::test_components::driver::traits::HasChainDriverTypeAt;
use hermes_core::test_components::setup::traits::{
    BinaryChannelDriverBuilder, BinaryChannelDriverBuilderComponent, HasTestDriverType,
};
use hermes_cosmos_core::chain_components::impls::CosmosRecoverClientPayload;
use hermes_cosmos_core::chain_components::types::CosmosCreateClientOptions;
use hermes_cosmos_relayer::contexts::{CosmosBiRelay, CosmosChain, CosmosRelay};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::{ChannelId, ClientId, ConnectionId, PortId};

use crate::contexts::{CosmosBinaryChannelTestDriver, CosmosChainDriver, CosmosRelayDriver};

#[cgp_new_provider(BinaryChannelDriverBuilderComponent)]
impl<Setup> BinaryChannelDriverBuilder<Setup> for BuildCosmosBinaryChannelDriver
where
    Setup: HasBiRelayTypeAt<Index<0>, Index<1>, BiRelay = CosmosBiRelay>
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = CosmosRelay>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = CosmosRelay>
        + HasChainTypeAt<Index<0>, Chain = CosmosChain>
        + HasChainTypeAt<Index<1>, Chain = CosmosChain>
        + HasChainDriverTypeAt<Index<0>, ChainDriver = CosmosChainDriver>
        + HasChainDriverTypeAt<Index<1>, ChainDriver = CosmosChainDriver>
        + HasTestDriverType<TestDriver = CosmosBinaryChannelTestDriver>
        + HasAsyncErrorType,
{
    async fn build_driver_with_binary_channel(
        _setup: &Setup,
        birelay: CosmosBiRelay,
        chain_driver_a: CosmosChainDriver,
        chain_driver_b: CosmosChainDriver,
        client_id_a: ClientId,
        client_id_b: ClientId,
        connection_id_a: ConnectionId,
        connection_id_b: ConnectionId,
        channel_id_a: ChannelId,
        channel_id_b: ChannelId,
        port_id_a: PortId,
        port_id_b: PortId,
        create_client_payload_options_a: &CosmosCreateClientOptions,
        create_client_payload_options_b: &CosmosCreateClientOptions,
    ) -> Result<CosmosBinaryChannelTestDriver, Setup::Error> {
        let relay_driver = CosmosRelayDriver { birelay };

        // TODO: These are hardcoded values for Osmosis v28.0.0
        let recover_client_payload = CosmosRecoverClientPayload {
            deposit_amount: 110000,
            deposit_denom: "stake".to_owned(),
        };

        let driver = CosmosBinaryChannelTestDriver {
            relay_driver,
            chain_driver_a,
            chain_driver_b,
            client_id_a,
            client_id_b,
            connection_id_a,
            connection_id_b,
            channel_id_a,
            channel_id_b,
            port_id_a,
            port_id_b,
            create_client_payload_options_a: create_client_payload_options_a.clone(),
            create_client_payload_options_b: create_client_payload_options_b.clone(),
            create_client_message_options_a: (),
            create_client_message_options_b: (),
            recover_client_payload_options_a: recover_client_payload.clone(),
            recover_client_payload_options_b: recover_client_payload,
        };

        Ok(driver)
    }
}
