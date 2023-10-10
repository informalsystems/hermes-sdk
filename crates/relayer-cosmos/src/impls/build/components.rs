use cgp_core::HasComponents;
use ibc_relayer_components_extra::components::extra::build::ExtraBuildComponents;

use crate::contexts::builder::CosmosBuilder;

pub struct CosmosBuildComponents;

impl HasComponents for CosmosBuilder {
    type Components = ExtraBuildComponents<CosmosBuildComponents>;
}
