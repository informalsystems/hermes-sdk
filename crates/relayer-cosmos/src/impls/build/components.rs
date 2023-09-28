use cgp_core::traits::HasComponents;
use ibc_relayer_all_in_one::all_for_one::builder::CanBuildAfoBiRelay;
use ibc_relayer_components_extra::components::extra::build::ExtraBuildComponents;

use crate::contexts::builder::CosmosBuilder;

pub struct CosmosBuildComponents;

impl HasComponents for CosmosBuilder {
    type Components = ExtraBuildComponents<CosmosBuildComponents>;
}

pub trait IsAfoBuilder: CanBuildAfoBiRelay {}

impl IsAfoBuilder for CosmosBuilder {}
