use async_trait::async_trait;
use cgp_core::traits::delegate_component::DelegateComponent;
use cgp_core::traits::error::HasErrorType;
use cgp_core::traits::has_components::HasComponents;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::types::aliases::{ChainIdA, ChainIdB, ClientIdA, ClientIdB};
use crate::std_prelude::*;

pub struct BiRelayBuilderComponent;

#[async_trait]
pub trait BiRelayBuilder<Build>
where
    Build: HasBiRelayType + HasErrorType,
{
    async fn build_birelay(
        build: &Build,
        chain_id_a: &ChainIdA<Build>,
        chain_id_b: &ChainIdB<Build>,
        client_id_a: &ClientIdA<Build>,
        client_id_b: &ClientIdB<Build>,
    ) -> Result<Build::BiRelay, Build::Error>;
}

#[async_trait]
impl<Build, Component> BiRelayBuilder<Build> for Component
where
    Build: HasBiRelayType + HasErrorType,
    Component: DelegateComponent<BiRelayBuilderComponent>,
    Component::Delegate: BiRelayBuilder<Build>,
{
    async fn build_birelay(
        build: &Build,
        chain_id_a: &ChainIdA<Build>,
        chain_id_b: &ChainIdB<Build>,
        client_id_a: &ClientIdA<Build>,
        client_id_b: &ClientIdB<Build>,
    ) -> Result<Build::BiRelay, Build::Error> {
        Component::Delegate::build_birelay(build, chain_id_a, chain_id_b, client_id_a, client_id_b)
            .await
    }
}

#[async_trait]
pub trait CanBuildBiRelay: HasBiRelayType + HasErrorType {
    async fn build_birelay(
        &self,
        chain_id_a: &ChainIdA<Self>,
        chain_id_b: &ChainIdB<Self>,
        client_id_a: &ClientIdA<Self>,
        client_id_b: &ClientIdB<Self>,
    ) -> Result<Self::BiRelay, Self::Error>;
}

#[async_trait]
impl<Build> CanBuildBiRelay for Build
where
    Build: HasBiRelayType + HasErrorType + HasComponents,
    Build::Components: BiRelayBuilder<Build>,
{
    async fn build_birelay(
        &self,
        chain_id_a: &ChainIdA<Self>,
        chain_id_b: &ChainIdB<Self>,
        client_id_a: &ClientIdA<Self>,
        client_id_b: &ClientIdB<Self>,
    ) -> Result<Self::BiRelay, Self::Error> {
        Build::Components::build_birelay(self, chain_id_a, chain_id_b, client_id_a, client_id_b)
            .await
    }
}
