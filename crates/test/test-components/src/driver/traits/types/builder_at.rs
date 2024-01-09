use hermes_relayer_components::build::traits::birelay::HasBiRelayType;

use crate::driver::traits::types::birelay_at::HasBiRelayTypeAt;

pub trait HasBuilderTypeAt<const A: usize, const B: usize>: HasBiRelayTypeAt<A, B> {
    type Builder: HasBiRelayType<BiRelay = Self::BiRelay>;
}
