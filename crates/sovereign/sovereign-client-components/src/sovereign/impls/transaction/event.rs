use core::str::Utf8Error;
use std::io::Error as IoError;

use borsh::BorshDeserialize;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::transaction::traits::event::TxResponseAsEventsParser;
use hermes_relayer_components::transaction::traits::types::HasTxResponseType;

use crate::sovereign::types::event::{RawEvent, SovereignEvent, SovereignEventDetail};
use crate::sovereign::types::rpc::tx_response::TxResponse;

pub struct ParseSovTxResponseAsEvents;

impl<Chain> TxResponseAsEventsParser<Chain> for ParseSovTxResponseAsEvents
where
    Chain: HasTxResponseType<TxResponse = TxResponse>
        + HasEventType<Event = SovereignEvent>
        + CanRaiseError<Utf8Error>
        + CanRaiseError<IoError>,
{
    fn parse_tx_response_as_events(
        response: TxResponse,
    ) -> Result<Vec<Vec<SovereignEvent>>, Chain::Error> {
        let events = response
            .events
            .iter()
            .filter_map(|event| {
                // By default, we discard events that we fail to parse,
                // which may not be of interest by the relayer.
                // In case if any expected event is missing,
                // try changing this to return error
                parse_event_response::<Chain>(event).ok()
            })
            .collect();

        Ok(vec![events])
    }
}

pub fn parse_event_response<Chain>(response: &RawEvent) -> Result<SovereignEvent, Chain::Error>
where
    Chain: CanRaiseError<Utf8Error> + CanRaiseError<IoError>,
{
    let key = core::str::from_utf8(&response.key)
        .map_err(Chain::raise_error)?
        .to_string();

    let detail = SovereignEventDetail::deserialize(&mut response.value.as_slice())
        .map_err(Chain::raise_error)?;

    Ok(SovereignEvent { key, detail })
}
