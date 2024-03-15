use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use basecoin::app::{BaseCoinApp, Builder};
use basecoin::modules::auth::Auth;
use basecoin::modules::bank::Bank;
use basecoin::modules::context::{prefix, Identifiable};
use basecoin::modules::ibc::Ibc;
use basecoin::store::context::ProvableStore;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use tendermint::{AppHash, Time};
use tendermint_testgen::light_block::TmLightBlock;
use tendermint_testgen::{Generator, Header, LightBlock, Validator};

use crate::traits::runner::BasecoinRunner;
use crate::types::status::ChainStatus;
use crate::util::mutex::MutexUtil;

/// A mock ABCI application that includes simplified store, application,
/// consensus layers.
///
/// The store consists of an in-memory AVL implementation that facilitates
/// proof verification.
///
/// The application layer includes Authentication, Bank, and IBC modules,
/// resulting in a fully operational ibc-rs implementation that runs in a
/// lightweight manner.
///
/// The consensus layer consists of a simple block production engine that
/// forgoes voting, validation, and transaction phases for the sake of
/// simplicity.
#[derive(Clone)]
pub struct MockBasecoin<S>
where
    S: ProvableStore + Debug,
{
    /// Chain runtime
    pub runtime: HermesRuntime,
    /// Chain identifier
    pub chain_id: ChainId,
    /// Chain validators
    pub validators: Arc<Mutex<Vec<Validator>>>,
    /// Chain blocks
    pub blocks: Arc<Mutex<Vec<TmLightBlock>>>,
    /// Chain application
    pub app: BaseCoinApp<S>,
    /// Current chain status
    pub current_status: Arc<Mutex<ChainStatus>>,
}

impl<S: ProvableStore + Default + Debug> MockBasecoin<S> {
    /// Constructs a new mock cosmos chain instance.
    pub fn new(
        runtime: HermesRuntime,
        chain_id: ChainId,
        validators: Vec<Validator>,
        store: S,
    ) -> Self {
        let app_builder = Builder::new(store);

        let auth = Auth::new(app_builder.module_store(&prefix::Auth {}.identifier()));
        let bank = Bank::new(
            app_builder.module_store(&prefix::Bank {}.identifier()),
            auth.account_reader().clone(),
            auth.account_keeper().clone(),
        );
        let ibc = Ibc::new(
            app_builder.module_store(&prefix::Ibc {}.identifier()),
            bank.bank_keeper().clone(),
        );

        // register modules with the app
        let app = app_builder
            .add_module(prefix::Auth {}.identifier(), auth.clone())
            .add_module(prefix::Bank {}.identifier(), bank.clone())
            .add_module(prefix::Ibc {}.identifier(), ibc)
            .build();

        let genesis_height = Height::new(chain_id.revision_number(), 1).expect("never fails");

        let genesis_time = Time::now();

        let genesis_block = Self::generate_block(
            &chain_id,
            genesis_height.revision_height(),
            genesis_time,
            &validators,
            AppHash::default(),
        );

        let genesis_status = Arc::new(Mutex::new(ChainStatus::new(
            genesis_height,
            genesis_time.into(),
        )));

        Self {
            runtime,
            chain_id,
            validators: Arc::new(Mutex::new(validators)),
            blocks: Arc::new(Mutex::new(vec![genesis_block])),
            app,
            current_status: genesis_status,
        }
    }

    pub fn runtime(&self) -> &HermesRuntime {
        &self.runtime
    }

    pub fn get_blocks(&self) -> Vec<TmLightBlock> {
        self.blocks.acquire_mutex().clone()
    }

    pub fn get_current_status(&self) -> ChainStatus {
        self.current_status.acquire_mutex().clone()
    }

    pub fn update_status(&self) {
        let blocks = self.blocks.acquire_mutex();

        let last_block = blocks.last().expect("never fails");

        let current_revision_height = last_block.signed_header.header.height.value();

        let current_time = last_block.signed_header.header.time;

        let current_height = Height::new(self.chain_id.revision_number(), current_revision_height)
            .expect("never fails");

        let mut last_status = self.current_status.acquire_mutex();

        *last_status = ChainStatus::new(current_height, current_time.into());
    }

    pub fn generate_block(
        chain_id: &ChainId,
        height: u64,
        time: Time,
        validators: &[Validator],
        app_hash: AppHash,
    ) -> TmLightBlock {
        let header = Header::new(validators)
            .chain_id(&chain_id.to_string())
            .height(height)
            .time(time)
            .next_validators(validators)
            .app_hash(app_hash);

        LightBlock::new_default_with_header(header)
            .generate()
            .expect("failed to generate light block")
    }

    pub fn grow_blocks(&self) {
        let root_hash = self.app.store.root_hash();

        let app_hash = AppHash::try_from(root_hash).expect("invalid app hash");

        let mut blocks = self.blocks.acquire_mutex();

        let validators = self.validators.acquire_mutex();

        let new_tm_light_block = Self::generate_block(
            &self.chain_id,
            blocks.len() as u64 + 1,
            Time::now(),
            &validators,
            app_hash,
        );

        blocks.push(new_tm_light_block);
    }

    pub fn run(&self) {
        let chain = self.clone();

        self.runtime().runtime.spawn(async move {
            chain.init().await;

            loop {
                chain.begin_block().await;

                tokio::time::sleep(Duration::from_millis(200)).await;

                chain.commit().await;
            }
        });
    }
}
