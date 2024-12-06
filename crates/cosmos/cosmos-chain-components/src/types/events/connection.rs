use core::str::FromStr;

use eyre::Report;
use ibc::core::client::types::events::CLIENT_ID_ATTRIBUTE_KEY;
use ibc::core::connection::types::events::{
    OpenInit, OpenTry, CONN_ID_ATTRIBUTE_KEY, COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
    COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
};
use ibc::core::host::types::identifiers::{ClientId, ConnectionId};
use tendermint::abci::Event;

pub struct CosmosConnectionOpenInitEvent {
    pub connection_id: ConnectionId,
}

pub struct CosmosConnectionOpenTryEvent {
    pub connection_id: ConnectionId,
}

pub fn try_conn_open_init_from_abci_event(event: &Event) -> Result<Option<OpenInit>, Report> {
    if event.kind.as_str() == "connection_open_init" {
        let conn_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some(CONN_ID_ATTRIBUTE_KEY))
            .ok_or_else(|| Report::msg("missing attribute `connection_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `connection_id` attribute value as str. Cause {e}"
                ))
            })?;
        let client_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some(CLIENT_ID_ATTRIBUTE_KEY))
            .ok_or_else(|| Report::msg("missing attribute `client_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let client_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| {
                attribute.key_str().ok() == Some(COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY)
            })
            .ok_or_else(|| Report::msg("missing attribute `counterparty_client_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `counterparty_client_id` attribute value as str. Cause {e}"
                ))
            })?;
        Ok(Some(OpenInit::new(
            ConnectionId::from_str(conn_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}"
                ))
            })?,
            ClientId::from_str(client_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{client_id_on_b_str}` to ClientId. Cause: {e}"
                ))
            })?,
            ClientId::from_str(client_id_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{client_id_on_a_str}` to Client Id. Cause: {e}"
                ))
            })?,
        )))
    } else {
        Ok(None)
    }
}

pub fn try_conn_open_try_from_abci_event(event: &Event) -> Result<Option<OpenTry>, Report> {
    if event.kind.as_str() == "connection_open_try" {
        let conn_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some(CONN_ID_ATTRIBUTE_KEY))
            .ok_or_else(|| Report::msg("missing attribute `connection_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `connection_id` attribute value as str. Cause {e}"
                ))
            })?;
        let client_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some(CLIENT_ID_ATTRIBUTE_KEY))
            .ok_or_else(|| Report::msg("missing attribute `client_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let conn_id_on_a_str = event.attributes.iter().find(|attribute| attribute.key_str().ok() == Some(COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY)).ok_or_else(|| Report::msg("missing attribute `counterparty_connection_id` in ABCI Event"))?.value_str().map_err(|e| Report::msg(format!("failed to retrieve `counterparty_connection_id` attribute value as str. Cause {e}")))?;
        let client_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| {
                attribute.key_str().ok() == Some(COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY)
            })
            .ok_or_else(|| Report::msg("missing attribute `counterparty_client_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `counterparty_client_id` attribute value as str. Cause {e}"
                ))
            })?;
        Ok(Some(OpenTry::new(
            ConnectionId::from_str(conn_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}"
                ))
            })?,
            ClientId::from_str(client_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{client_id_on_b_str}` to ClientId. Cause: {e}"
                ))
            })?,
            ConnectionId::from_str(conn_id_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}"
                ))
            })?,
            ClientId::from_str(client_id_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{client_id_on_a_str}` to ClientId. Cause: {e}"
                ))
            })?,
        )))
    } else {
        Ok(None)
    }
}
