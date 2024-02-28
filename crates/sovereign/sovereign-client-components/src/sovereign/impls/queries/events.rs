use core::str::Utf8Error;
use std::io::Error as IoError;

use borsh::BorshDeserialize;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::sovereign::traits::rollup::json_rpc_client::HasJsonRpcClient;
use crate::sovereign::traits::rollup::queries::events::EventsByEventIdsQuerier;
use crate::sovereign::traits::rollup::types::event_id::HasEventIdType;
use crate::sovereign::types::event::{RawEvent, SovereignEvent, SovereignEventDetail};

pub struct QuerySovereignEvents;

impl<Rollup> EventsByEventIdsQuerier<Rollup> for QuerySovereignEvents
where
    Rollup: HasEventType<Event = SovereignEvent>
        + HasEventIdType
        + HasJsonRpcClient
        + CanRaiseError<ClientError>
        + CanRaiseError<Utf8Error>
        + CanRaiseError<IoError>,
    Rollup::JsonRpcClient: ClientT,
    Rollup::EventId: Serialize,
{
    async fn query_events_by_event_ids(
        rollup: &Rollup,
        event_ids: &[Rollup::EventId],
    ) -> Result<Vec<SovereignEvent>, Rollup::Error> {
        let response: Vec<RawEvent> = rollup
            .json_rpc_client()
            .request("ledger_getEvents", (event_ids,))
            .await
            .map_err(Rollup::raise_error)?;

        let events = response
            .iter()
            .filter_map(|event| parse_event_response::<Rollup>(event).ok())
            .collect();

        Ok(events)
    }
}

pub fn parse_event_response<Rollup>(response: &RawEvent) -> Result<SovereignEvent, Rollup::Error>
where
    Rollup: CanRaiseError<Utf8Error> + CanRaiseError<IoError>,
{
    let key = core::str::from_utf8(&response.key)
        .map_err(Rollup::raise_error)?
        .to_string();

    let detail = SovereignEventDetail::deserialize(&mut response.value.as_slice())
        .map_err(Rollup::raise_error)?;

    Ok(SovereignEvent { key, detail })
}
