use std::sync::Arc;

use basecoin_modules::ibc::IbcContext;
use basecoin_store::impls::RevertibleStore;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc::core::handler::types::events::IbcEvent;
use ibc::core::host::ValidationContext;
use ibc::primitives::proto::Any;

use crate::traits::endpoint::BasecoinEndpoint;
use crate::types::error::Error;

/// Holds the necessary fields for querying a mock Cosmos
/// chain endpoint.
#[derive(Clone)]
pub struct MockCosmosContext<Endpoint: BasecoinEndpoint> {
    /// Chain runtime
    pub runtime: HermesRuntime,
    /// Chain handle
    pub querier: Arc<Endpoint>,
}

impl<Endpoint: BasecoinEndpoint> MockCosmosContext<Endpoint> {
    /// Constructs a new mock cosmos chain instance.
    pub fn new(runtime: HermesRuntime, querier: Arc<Endpoint>) -> Self {
        Self { runtime, querier }
    }

    pub fn runtime(&self) -> &HermesRuntime {
        &self.runtime
    }

    pub fn ibc_context(
        &self,
    ) -> IbcContext<RevertibleStore<<Endpoint as BasecoinEndpoint>::Store>> {
        self.ibc().ctx()
    }

    pub fn submit_messages(&self, msgs: Vec<Any>) -> Result<Vec<Vec<IbcEvent>>, Error> {
        let mut events = Vec::new();

        self.ibc_context().host_height()?;

        for msg in msgs {
            let ibc_events = self.ibc().process_message(msg)?;

            events.push(ibc_events);
        }

        Ok(events)
    }
}
