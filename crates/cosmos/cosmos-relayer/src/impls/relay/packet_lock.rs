use hermes_relayer_components::relay::impls::packet_lock::PacketMutexGetter;

use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;

impl PacketMutexGetter<CosmosRelay> for CosmosRelayComponents {
    fn packet_mutex(
        relay: &CosmosRelay,
    ) -> &hermes_relayer_components::relay::impls::packet_lock::PacketMutex<CosmosRelay> {
        &relay.packet_lock_mutex
    }
}
