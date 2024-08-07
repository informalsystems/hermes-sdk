use alloc::sync::Arc;

use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::channel::oneshot::Sender as SenderOnce;
use futures::lock::Mutex;
use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use hermes_error::types::Error;
use tendermint::abci::Event as AbciEvent;

pub type CosmosBatchPayload = (
    Vec<CosmosMessage>,
    SenderOnce<Result<Vec<Vec<Arc<AbciEvent>>>, Error>>,
);

pub type CosmosBatchSender = Arc<Mutex<UnboundedSender<CosmosBatchPayload>>>;

pub type CosmosBatchReceiver = UnboundedReceiver<CosmosBatchPayload>;
