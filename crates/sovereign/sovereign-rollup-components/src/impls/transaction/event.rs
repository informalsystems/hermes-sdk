use core::str::Utf8Error;
use std::io::Error as IoError;

use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParser;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;

use crate::types::event::SovereignEvent;
use crate::types::tx::tx_response::TxResponse;

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
        Ok(vec![response.events])
    }
}
