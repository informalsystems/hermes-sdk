use std::fmt::Debug;

use basecoin_app::modules::ibc::Ibc;
use basecoin_app::modules::types::IdentifiedModule;
use basecoin_store::context::{ProvableStore, Store};
use basecoin_store::impls::RevertibleStore;
use basecoin_store::utils::SharedRwExt;
use cgp_core::prelude::*;
use ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc::core::ics24_host::identifier::ChainId;
use ibc::core::ics24_host::path::Path;
use ibc::core::timestamp::Timestamp;
use ibc::hosts::tendermint::IBC_QUERY_PATH;
use ibc::Height;
use tendermint::block::Height as TmHeight;
use tendermint::time::Time;
use tendermint::v0_37::abci::request::{InitChain, Query};
use tendermint::v0_37::abci::{Request as AbciRequest, Response as AbciResponse};
use tendermint_testgen::consensus::default_consensus_params;
use tendermint_testgen::light_block::TmLightBlock;
use tower::Service;

use crate::contexts::basecoin::MockBasecoin;
use crate::traits::endpoint::BasecoinEndpoint;
use crate::traits::runner::BasecoinRunner;
use crate::types::error::Error;
use crate::util::conversion::convert_tm_to_ics_merkle_proof;
use crate::util::dummy::genesis_app_state;
use crate::util::mutex::MutexUtil;

#[async_trait]
impl<S> BasecoinRunner for MockBasecoin<S>
where
    S: ProvableStore + Default + Debug,
{
    type Store = S;

    /// Initialize the chain with the app state.
    async fn init(&self) {
        let app_state = serde_json::to_vec(&genesis_app_state()).expect("infallible serialization");

        let request = InitChain {
            time: Time::now(),
            chain_id: self.chain_id.to_string(),
            consensus_params: default_consensus_params(),
            validators: vec![],
            app_state_bytes: app_state.into(),
            initial_height: TmHeight::from(1_u8),
        };

        let mut app = self.app.clone();

        app.call(AbciRequest::InitChain(request))
            .await
            .expect("failed to initialize chain");
    }

    async fn begin_block(&self) {
        self.grow_blocks();

        self.update_status();

        let last_block = self.blocks.acquire_mutex().last().unwrap().clone();

        let mut events = Vec::new();

        let mut modules = self.app.modules.write_access();

        for IdentifiedModule { id: _, module } in modules.iter_mut() {
            let event = module.begin_block(&last_block.signed_header.header);
            events.extend(event);
        }
    }

    /// Commits the chain state to the store.
    async fn commit(&self) {
        let mut modules = self.app.modules.write_access();

        let mut state = self.app.store.write_access();

        for IdentifiedModule { id, module } in modules.iter_mut() {
            module
                .store_mut()
                .commit()
                .expect("failed to commit to state");

            state
                .set(id.clone().into(), module.store().root_hash())
                .expect("failed to update sub-store commitment");
        }

        state.commit().expect("failed to commit to state");
    }
}

#[async_trait]
impl<S: ProvableStore + Default + Debug> BasecoinEndpoint for MockBasecoin<S> {
    type Store = S;
    /// Queries the mock chain for the given path and height.
    async fn query(
        &self,
        path: impl Into<Path> + Send,
        height: &Height,
    ) -> Result<(Vec<u8>, CommitmentProofBytes), Error> {
        let request = Query {
            path: IBC_QUERY_PATH.to_string(),
            data: path.into().to_string().into_bytes().into(),
            height: TmHeight::try_from(height.revision_height()).unwrap(),
            prove: true,
        };

        let mut app = self.app.clone();

        let response = match app
            .call(AbciRequest::Query(request))
            .await
            .map_err(Error::source)?
        {
            AbciResponse::Query(res) => res,
            _ => panic!("unexpected response from query"),
        };

        let proof = match response.proof {
            Some(proof) => proof,
            None => return Err(Error::empty("proof")),
        };

        let merkle_proof = convert_tm_to_ics_merkle_proof(&proof)?;

        let commitment_proof = merkle_proof.try_into().map_err(Error::source)?;

        Ok((response.value.into(), commitment_proof))
    }

    fn ibc(&self) -> Ibc<RevertibleStore<S>> {
        self.app.ibc()
    }

    fn get_chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    fn get_current_height(&self) -> Height {
        self.get_current_status().height
    }

    fn get_current_timestamp(&self) -> Timestamp {
        self.get_current_status().timestamp
    }

    fn get_light_block(&self, height: &Height) -> Result<TmLightBlock, Error> {
        let blocks = self.get_blocks();

        let revision_height = height.revision_height() as usize;

        if revision_height > blocks.len() {
            return Err(Error::invalid("block index out of bounds"));
        }

        Ok(blocks[revision_height - 1].clone())
    }
}
