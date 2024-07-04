use hermes_relayer_components::build::traits::birelay::ProvideBiRelayType;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::builder::CosmosBuilder;
use crate::impls::build::components::CosmosBuildComponents;

impl ProvideBiRelayType<CosmosBuilder> for CosmosBuildComponents {
    type BiRelay = CosmosBiRelay;
}
