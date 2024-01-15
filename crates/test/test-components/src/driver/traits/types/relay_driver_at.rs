use hermes_relayer_components::build::traits::birelay::HasBiRelayType;

use crate::driver::traits::types::birelay_at::HasBiRelayTypeAt;

pub trait HasRelayDriverAt<const A: usize, const B: usize>: HasBiRelayTypeAt<A, B> {
    type RelayDriver: HasBiRelayType<BiRelay = Self::BiRelay>;
}
