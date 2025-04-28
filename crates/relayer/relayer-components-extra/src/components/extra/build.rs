#[cgp::re_export_imports]
mod preset {
    use hermes_prelude::*;
    use hermes_relayer_components::build::traits::builders::birelay_builder::BiRelayBuilderComponent;
    use hermes_relayer_components::build::traits::builders::chain_builder::ChainBuilderComponent;
    use hermes_relayer_components::build::traits::builders::relay_builder::RelayBuilderComponent;
    use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::RelayFromChainsBuilderComponent;
    use hermes_relayer_components::components::default::DefaultBuildComponents;

    use crate::build::impls::relay::batch::BuildRelayWithBatchWorker;

    cgp_preset! {
        ExtraBuildComponents<BaseComponents: Async> {
            RelayFromChainsBuilderComponent: BuildRelayWithBatchWorker,
            [
                ChainBuilderComponent,
                RelayBuilderComponent,
                BiRelayBuilderComponent,
            ]:
                DefaultBuildComponents::Provider<BaseComponents>,
        }
    }
}
