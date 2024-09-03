use std::fmt::Debug;
use std::sync::Arc;

use basecoin::modules::ibc::Ibc;
use basecoin::store::context::ProvableStore;
use basecoin::store::impls::RevertibleStore;
use cgp::core::Async;
use cgp::prelude::*;
use ibc::core::client::types::Height;
use ibc::core::commitment_types::commitment::CommitmentProofBytes;
use ibc::core::host::types::identifiers::ChainId;
use ibc::core::host::types::path::Path;
use ibc::primitives::Timestamp;
use tendermint_testgen::light_block::TmLightBlock;

use crate::types::error::Error;

/// Defines the interface that empowers a chain context with the ability to
/// query different states of a chain.
pub trait QueryService: Async {
    type Endpoint: BasecoinEndpoint;

    fn service(&self) -> &Arc<Self::Endpoint>;
}

/// Defines the interface that enables a mock Cosmos chain to provide query
/// endpoints.
#[async_trait]
pub trait BasecoinEndpoint: Async + Clone {
    type Store: ProvableStore + Debug + Default;

    async fn query(
        &self,
        path: impl Into<Path> + Send,
        height: &Height,
    ) -> Result<(Vec<u8>, CommitmentProofBytes), Error>;

    fn ibc(&self) -> Ibc<RevertibleStore<Self::Store>>;

    fn get_chain_id(&self) -> &ChainId;

    fn get_current_height(&self) -> Height;

    fn get_current_timestamp(&self) -> Timestamp;

    fn get_light_block(&self, height: &Height) -> Result<TmLightBlock, Error>;
}

impl<Ctx> BasecoinEndpoint for Ctx
where
    Ctx: QueryService + Async + Clone,
{
    type Store = <<Ctx as QueryService>::Endpoint as BasecoinEndpoint>::Store;

    async fn query(
        &self,
        path: impl Into<Path> + Send,
        height: &Height,
    ) -> Result<(Vec<u8>, CommitmentProofBytes), Error> {
        Ctx::service(self).query(path, height).await
    }

    fn ibc(&self) -> Ibc<RevertibleStore<Self::Store>> {
        Ctx::service(self).ibc()
    }

    fn get_chain_id(&self) -> &ChainId {
        Ctx::service(self).get_chain_id()
    }

    fn get_current_height(&self) -> Height {
        Ctx::service(self).get_current_height()
    }

    fn get_current_timestamp(&self) -> Timestamp {
        Ctx::service(self).get_current_timestamp()
    }

    fn get_light_block(&self, height: &Height) -> Result<TmLightBlock, Error> {
        Ctx::service(self).get_light_block(height)
    }
}
