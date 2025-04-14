#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::build::components::{
        BuildBiRelayFromRelays, BuildChainWithCache, BuildRelayFromChains, BuildRelayWithCache,
    };
    use crate::build::traits::builders::birelay_builder::BiRelayBuilderComponent;
    use crate::build::traits::builders::chain_builder::ChainBuilderComponent;
    use crate::build::traits::builders::relay_builder::RelayBuilderComponent;

    cgp_preset! {
        DefaultBuildComponents<BaseComponents: Async> {
            ChainBuilderComponent: BuildChainWithCache<BaseComponents>,
            RelayBuilderComponent: BuildRelayWithCache<BuildRelayFromChains>,
            BiRelayBuilderComponent: BuildBiRelayFromRelays,
        }
    }
}
