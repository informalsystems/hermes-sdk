use core::str::FromStr;

use eyre::Report;
use ibc::core::channel::types::channel::Order;
use ibc::core::channel::types::events::{OpenInit, OpenTry, SendPacket, WriteAcknowledgement};
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::channel::types::Version;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, ConnectionId, PortId, Sequence};
use ibc::primitives::Timestamp;
use subtle_encoding::hex;
use tendermint::abci::Event;

pub struct CosmosChannelOpenInitEvent {
    pub channel_id: ChannelId,
}
pub struct CosmosChannelOpenTryEvent {
    pub channel_id: ChannelId,
}

pub fn try_chan_open_init_from_abci_event(event: &Event) -> Result<Option<OpenInit>, Report> {
    if event.kind.as_str() == "channel_open_init" {
        let port_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("port_id"))
            .ok_or_else(|| Report::msg("missing attribute `connection_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `connection_id` attribute value as str. Cause {e}"
                ))
            })?;
        let chan_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("channel_id"))
            .ok_or_else(|| Report::msg("missing attribute `client_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let port_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("counterparty_port_id"))
            .ok_or_else(|| Report::msg("missing attribute `counterparty_client_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `counterparty_client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let conn_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("connection_id"))
            .ok_or_else(|| Report::msg("missing attribute `counterparty_client_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `counterparty_client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let version_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("version"))
            .ok_or_else(|| Report::msg("missing attribute `counterparty_client_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `counterparty_client_id` attribute value as str. Cause {e}"
                ))
            })?;
        Ok(Some(OpenInit::new(
            PortId::from_str(port_id_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{port_id_on_a_str}` to PortId. Cause: {e}"
                ))
            })?,
            ChannelId::from_str(chan_id_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{chan_id_on_a_str}` to ChannelId. Cause: {e}"
                ))
            })?,
            PortId::from_str(port_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{port_id_on_b_str}` to PortId. Cause: {e}"
                ))
            })?,
            ConnectionId::from_str(conn_id_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{conn_id_on_a_str}` to ConnectionId. Cause: {e}"
                ))
            })?,
            Version::from_str(version_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{version_on_a_str}` to Version. Cause: {e}"
                ))
            })?,
        )))
    } else {
        Ok(None)
    }
}

pub fn try_chan_open_try_from_abci_event(event: &Event) -> Result<Option<OpenTry>, Report> {
    if event.kind.as_str() == "channel_open_try" {
        let port_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("port_id"))
            .ok_or_else(|| Report::msg("missing attribute `port_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `connection_id` attribute value as str. Cause {e}"
                ))
            })?;
        let chan_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("channel_id"))
            .ok_or_else(|| Report::msg("missing attribute `channel_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let port_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("counterparty_port_id"))
            .ok_or_else(|| Report::msg("missing attribute `counterparty_port_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `counterparty_client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let chan_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("counterparty_channel_id"))
            .ok_or_else(|| {
                Report::msg("missing attribute `counterparty_channel_id` in ABCI Event")
            })?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let conn_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("connection_id"))
            .ok_or_else(|| Report::msg("missing attribute `connection_id` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `counterparty_client_id` attribute value as str. Cause {e}"
                ))
            })?;
        let version_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("version"))
            .ok_or_else(|| Report::msg("missing attribute `version` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `counterparty_client_id` attribute value as str. Cause {e}"
                ))
            })?;
        Ok(Some(OpenTry::new(
            PortId::from_str(port_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{port_id_on_b_str}` to PortId. Cause: {e}"
                ))
            })?,
            ChannelId::from_str(chan_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{chan_id_on_b_str}` to ChannelId. Cause: {e}"
                ))
            })?,
            PortId::from_str(port_id_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{port_id_on_a_str}` to PortId. Cause: {e}"
                ))
            })?,
            ChannelId::from_str(chan_id_on_a_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{chan_id_on_a_str}` to ChannelId. Cause: {e}"
                ))
            })?,
            ConnectionId::from_str(conn_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}"
                ))
            })?,
            Version::from_str(version_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{version_on_b_str}` to Version. Cause: {e}"
                ))
            })?,
        )))
    } else {
        Ok(None)
    }
}

pub fn try_write_acknowledgment_from_abci_event(
    event: &Event,
) -> Result<Option<WriteAcknowledgement>, Report> {
    if event.kind.as_str() == "write_acknowledgement" {
        let seq_on_a = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_sequence"))
            .ok_or_else(|| Report::msg("missing attribute `packet_sequence` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `connection_id` attribute value as str. Cause {e}"
                ))
            })?;
        let port_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_src_port"))
            .ok_or_else(|| Report::msg("missing attribute `packet_src_port` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_src_port` attribute value as str. Cause {e}"
                ))
            })?;
        let chan_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_src_channel"))
            .ok_or_else(|| Report::msg("missing attribute `packet_src_channel` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_src_channel` attribute value as str. Cause {e}"
                ))
            })?;
        let port_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_dst_port"))
            .ok_or_else(|| Report::msg("missing attribute `packet_dst_port` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_dst_port` attribute value as str. Cause {e}"
                ))
            })?;
        let chan_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_dst_channel"))
            .ok_or_else(|| Report::msg("missing attribute `packet_dst_channel` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_dst_channel` attribute value as str. Cause {e}"
                ))
            })?;
        let timeout_height_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_timeout_height"))
            .ok_or_else(|| Report::msg("missing attribute `packet_timeout_height` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_timeout_height` attribute value as str. Cause {e}"
                ))
            })?;
        let timeout_timestamp_on_b_str = event.attributes.iter().find(|attribute| attribute.key_str().ok() == Some("packet_timeout_timestamp")).ok_or_else(|| Report::msg("missing attribute `packet_timeout_timestamp` in ABCI Event"))?.value_str().map_err(|e| Report::msg(format!("failed to retrieve `packet_timeout_timestamp` attribute value as str. Cause {e}")))?;
        let conn_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_connection"))
            .ok_or_else(|| Report::msg("missing attribute `packet_connection` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_connection` attribute value as str. Cause {e}"
                ))
            })?;

        let maybe_packet_data_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_data"));
        let maybe_packet_data_hex_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_data_hex"));

        let maybe_packet_ack_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_ack"));
        let maybe_packet_ack_hex_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_ack_hex"));

        let timeout_height_on_b = if timeout_height_on_b_str == "0-0" {
            TimeoutHeight::no_timeout()
        } else {
            let timeout_height = Height::from_str(timeout_height_on_b_str)?;
            TimeoutHeight::from(timeout_height)
        };

        let timeout_timestamp_on_b = if timeout_timestamp_on_b_str == "0" {
            TimeoutTimestamp::no_timeout()
        } else {
            let timeout_timestamp = Timestamp::from_str(timeout_height_on_b_str)?;
            TimeoutTimestamp::from(timeout_timestamp)
        };

        let packet_data = if let Some(event_attribute) = maybe_packet_data_str {
            event_attribute
                .value_str()
                .map_err(|e| {
                    Report::msg(format!(
                        "failed to retrieve `packet_data` attribute value as str. Cause {e}"
                    ))
                })?
                .as_bytes()
                .to_vec()
        } else if let Some(event_attribute) = maybe_packet_data_hex_str {
            hex::decode(event_attribute.value_str().map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_data` attribute value as str. Cause {e}"
                ))
            })?)?
        } else {
            return Err(Report::msg(
                "missing `packet_data` and `packet_data_hex` in ABCI Event",
            ));
        };

        let acknowledgment = if let Some(event_attribute) = maybe_packet_ack_str {
            event_attribute
                .value_str()
                .map_err(|e| {
                    Report::msg(format!(
                        "failed to retrieve `packet_ack` attribute value as str. Cause {e}"
                    ))
                })?
                .as_bytes()
                .to_vec()
        } else if let Some(event_attribute) = maybe_packet_ack_hex_str {
            hex::decode(event_attribute.value_str().map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_ack_hex` attribute value as str. Cause {e}"
                ))
            })?)?
        } else {
            return Err(Report::msg(
                "missing `packet_ack` and `packet_ack_hex` in ABCI Event",
            ));
        };

        let packet = Packet {
            seq_on_a: Sequence::from_str(seq_on_a)?,
            port_id_on_a: PortId::from_str(port_id_on_a_str)?,
            chan_id_on_a: ChannelId::from_str(chan_id_on_a_str)?,
            port_id_on_b: PortId::from_str(port_id_on_b_str)?,
            chan_id_on_b: ChannelId::from_str(chan_id_on_b_str)?,
            data: packet_data,
            timeout_height_on_b,
            timeout_timestamp_on_b,
        };

        Ok(Some(WriteAcknowledgement::new(
            packet,
            acknowledgment.try_into()?,
            ConnectionId::from_str(conn_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}"
                ))
            })?,
        )))
    } else {
        Ok(None)
    }
}

pub fn try_send_packet_from_abci_event(event: &Event) -> Result<Option<SendPacket>, Report> {
    if event.kind.as_str() == "send_packet" {
        let seq_on_a = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_sequence"))
            .ok_or_else(|| Report::msg("missing attribute `packet_sequence` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `connection_id` attribute value as str. Cause {e}"
                ))
            })?;
        let port_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_src_port"))
            .ok_or_else(|| Report::msg("missing attribute `packet_src_port` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_src_port` attribute value as str. Cause {e}"
                ))
            })?;
        let chan_id_on_a_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_src_channel"))
            .ok_or_else(|| Report::msg("missing attribute `packet_src_channel` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_src_channel` attribute value as str. Cause {e}"
                ))
            })?;
        let port_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_dst_port"))
            .ok_or_else(|| Report::msg("missing attribute `packet_dst_port` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_dst_port` attribute value as str. Cause {e}"
                ))
            })?;
        let chan_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_dst_channel"))
            .ok_or_else(|| Report::msg("missing attribute `packet_dst_channel` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_dst_channel` attribute value as str. Cause {e}"
                ))
            })?;
        let timeout_height_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_timeout_height"))
            .ok_or_else(|| Report::msg("missing attribute `packet_timeout_height` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_timeout_height` attribute value as str. Cause {e}"
                ))
            })?;
        let timeout_timestamp_on_b_str = event.attributes.iter().find(|attribute| attribute.key_str().ok() == Some("packet_timeout_timestamp")).ok_or_else(|| Report::msg("missing attribute `packet_timeout_timestamp` in ABCI Event"))?.value_str().map_err(|e| Report::msg(format!("failed to retrieve `packet_timeout_timestamp` attribute value as str. Cause {e}")))?;
        let conn_id_on_b_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_connection"))
            .ok_or_else(|| Report::msg("missing attribute `packet_connection` in ABCI Event"))?
            .value_str()
            .map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_connection` attribute value as str. Cause {e}"
                ))
            })?;
        let channel_ordering_str = event.attributes.iter().find(|attribute| attribute.key_str().ok() == Some("packet_channel_ordering")).ok_or_else(|| Report::msg("missing attribute `packet_channel_ordering` in ABCI Event"))?.value_str().map_err(|e| Report::msg(format!("failed to retrieve `packet_channel_ordering` attribute value as str. Cause {e}")))?;

        let maybe_packet_data_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_data")); //.ok_or_else(|| Report::msg("missing attribute `packet_data` in ABCI Event"))?.value_str().map_err(|e| Report::msg(format!("failed to retrieve `packet_data` attribute value as str. Cause {e}")))?;
        let maybe_packet_data_hex_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_data_hex")); //.ok_or_else(|| Report::msg("missing attribute `packet_data_hex` in ABCI Event"))?.value_str().map_err(|e| Report::msg(format!("failed to retrieve `packet_data_hex` attribute value as str. Cause {e}")))?;

        let timeout_height_on_b = if timeout_height_on_b_str == "0-0" {
            TimeoutHeight::no_timeout()
        } else {
            let timeout_height = Height::from_str(timeout_height_on_b_str)?;
            TimeoutHeight::from(timeout_height)
        };

        let timeout_timestamp_on_b = if timeout_timestamp_on_b_str == "0" {
            TimeoutTimestamp::no_timeout()
        } else {
            let timeout_timestamp = Timestamp::from_str(timeout_height_on_b_str)?;
            TimeoutTimestamp::from(timeout_timestamp)
        };

        let packet_data = if let Some(event_attribute) = maybe_packet_data_str {
            event_attribute
                .value_str()
                .map_err(|e| {
                    Report::msg(format!(
                        "failed to retrieve `packet_data` attribute value as str. Cause {e}"
                    ))
                })?
                .as_bytes()
                .to_vec()
        } else if let Some(event_attribute) = maybe_packet_data_hex_str {
            hex::decode(event_attribute.value_str().map_err(|e| {
                Report::msg(format!(
                    "failed to retrieve `packet_data` attribute value as str. Cause {e}"
                ))
            })?)?
        } else {
            return Err(Report::msg(
                "missing `packet_data` and `packet_data_hex` in ABCI Event",
            ));
        };

        let packet = Packet {
            seq_on_a: Sequence::from_str(seq_on_a)?,
            port_id_on_a: PortId::from_str(port_id_on_a_str)?,
            chan_id_on_a: ChannelId::from_str(chan_id_on_a_str)?,
            port_id_on_b: PortId::from_str(port_id_on_b_str)?,
            chan_id_on_b: ChannelId::from_str(chan_id_on_b_str)?,
            data: packet_data,
            timeout_height_on_b,
            timeout_timestamp_on_b,
        };

        Ok(Some(SendPacket::new(
            packet,
            Order::from_str(channel_ordering_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{channel_ordering_str}` to Order. Cause: {e}"
                ))
            })?,
            ConnectionId::from_str(conn_id_on_b_str).map_err(|e| {
                Report::msg(format!(
                    "failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}"
                ))
            })?,
        )))
    } else {
        Ok(None)
    }
}
