use alloc::sync::Arc;

use hermes_cosmos_client_components::traits::message::CosmosMessage;
use hermes_cosmos_relayer::types::error::Error;
use tendermint::abci::Event as AbciEvent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::Sender as SenderOnce;

use crate::types::event::SolomachineEvent;
use crate::types::message::SolomachineMessage;

pub type CosmosBatchPayload = (
    Vec<CosmosMessage>,
    SenderOnce<Result<Vec<Vec<Arc<AbciEvent>>>, Error>>,
);

pub type CosmosBatchSender = UnboundedSender<CosmosBatchPayload>;

pub type CosmosBatchReceiver = UnboundedReceiver<CosmosBatchPayload>;

pub type SolomachineBatchPayload = (
    Vec<SolomachineMessage>,
    SenderOnce<Result<Vec<Vec<SolomachineEvent>>, Error>>,
);

pub type SolomachineBatchSender = UnboundedSender<SolomachineBatchPayload>;

pub type SolomachineBatchReceiver = UnboundedReceiver<SolomachineBatchPayload>;
