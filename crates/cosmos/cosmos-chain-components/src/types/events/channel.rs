use core::str::FromStr;

use ibc::core::channel::types::channel::Order;
use ibc::core::channel::types::events::{OpenInit, OpenTry, SendPacket, WriteAcknowledgement};
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::channel::types::Version;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, ConnectionId, PortId, Sequence};
use ibc::primitives::Timestamp;
use subtle_encoding::hex;
use tendermint::abci::{Event, EventAttribute};

pub struct CosmosChannelOpenInitEvent {
    pub channel_id: ChannelId,
}
pub struct CosmosChannelOpenTryEvent {
    pub channel_id: ChannelId,
}

pub fn try_chan_open_init_from_abci_event(event: &Event) -> Result<Option<OpenInit>, String> {
    if event.kind.as_str() == "channel_open_init" {
        let port_id_on_a_str = find_attribute_as_string(&event.attributes, "port_id")?;
        let chan_id_on_a_str = find_attribute_as_string(&event.attributes, "channel_id")?;
        let port_id_on_b_str = find_attribute_as_string(&event.attributes, "counterparty_port_id")?;
        let conn_id_on_a_str = find_attribute_as_string(&event.attributes, "connection_id")?;
        let version_on_a_str = find_attribute_as_string(&event.attributes, "version")?;

        Ok(Some(OpenInit::new(
            PortId::from_str(&port_id_on_a_str).map_err(|e| {
                format!("failed to convert `{port_id_on_a_str}` to PortId. Cause: {e}")
            })?,
            ChannelId::from_str(&chan_id_on_a_str).map_err(|e| {
                format!("failed to convert `{chan_id_on_a_str}` to ChannelId. Cause: {e}")
            })?,
            PortId::from_str(&port_id_on_b_str).map_err(|e| {
                format!("failed to convert `{port_id_on_b_str}` to PortId. Cause: {e}")
            })?,
            ConnectionId::from_str(&conn_id_on_a_str).map_err(|e| {
                format!("failed to convert `{conn_id_on_a_str}` to ConnectionId. Cause: {e}")
            })?,
            Version::from_str(&version_on_a_str).map_err(|e| {
                format!("failed to convert `{version_on_a_str}` to Version. Cause: {e}")
            })?,
        )))
    } else {
        Ok(None)
    }
}

pub fn try_chan_open_try_from_abci_event(event: &Event) -> Result<Option<OpenTry>, String> {
    if event.kind.as_str() == "channel_open_try" {
        let port_id_on_b_str = find_attribute_as_string(&event.attributes, "port_id")?;
        let chan_id_on_b_str = find_attribute_as_string(&event.attributes, "channel_id")?;
        let port_id_on_a_str = find_attribute_as_string(&event.attributes, "counterparty_port_id")?;
        let chan_id_on_a_str =
            find_attribute_as_string(&event.attributes, "counterparty_channel_id")?;
        let conn_id_on_b_str = find_attribute_as_string(&event.attributes, "connection_id")?;
        let version_on_b_str = find_attribute_as_string(&event.attributes, "version")?;

        Ok(Some(OpenTry::new(
            PortId::from_str(&port_id_on_b_str).map_err(|e| {
                format!("failed to convert `{port_id_on_b_str}` to PortId. Cause: {e}")
            })?,
            ChannelId::from_str(&chan_id_on_b_str).map_err(|e| {
                format!("failed to convert `{chan_id_on_b_str}` to ChannelId. Cause: {e}")
            })?,
            PortId::from_str(&port_id_on_a_str).map_err(|e| {
                format!("failed to convert `{port_id_on_a_str}` to PortId. Cause: {e}")
            })?,
            ChannelId::from_str(&chan_id_on_a_str).map_err(|e| {
                format!("failed to convert `{chan_id_on_a_str}` to ChannelId. Cause: {e}")
            })?,
            ConnectionId::from_str(&conn_id_on_b_str).map_err(|e| {
                format!("failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}")
            })?,
            Version::from_str(&version_on_b_str).map_err(|e| {
                format!("failed to convert `{version_on_b_str}` to Version. Cause: {e}")
            })?,
        )))
    } else {
        Ok(None)
    }
}

pub fn try_write_acknowledgment_from_abci_event(
    event: &Event,
) -> Result<Option<WriteAcknowledgement>, String> {
    if event.kind.as_str() == "write_acknowledgement" {
        let seq_on_a = find_attribute_as_string(&event.attributes, "packet_sequence")?;
        let port_id_on_a_str = find_attribute_as_string(&event.attributes, "packet_src_port")?;
        let chan_id_on_a_str = find_attribute_as_string(&event.attributes, "packet_src_channel")?;
        let port_id_on_b_str = find_attribute_as_string(&event.attributes, "packet_dst_port")?;
        let chan_id_on_b_str = find_attribute_as_string(&event.attributes, "packet_dst_channel")?;
        let timeout_height_on_b_str =
            find_attribute_as_string(&event.attributes, "packet_timeout_height")?;
        let timeout_timestamp_on_b_str =
            find_attribute_as_string(&event.attributes, "packet_timeout_timestamp")?;
        let conn_id_on_b_str = find_attribute_as_string(&event.attributes, "packet_connection")?;

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
            let timeout_height = Height::from_str(&timeout_height_on_b_str)
                .map_err(|e| format!("failed to convert string to height. Cause: {e}"))?;
            TimeoutHeight::from(timeout_height)
        };

        let timeout_timestamp_on_b = if timeout_timestamp_on_b_str == "0" {
            TimeoutTimestamp::no_timeout()
        } else {
            let timeout_timestamp =
                Timestamp::from_str(&timeout_timestamp_on_b_str).map_err(|e| {
                    format!("failed to convert string to timeout timestamp. Cause: {e}")
                })?;
            TimeoutTimestamp::from(timeout_timestamp)
        };

        let packet_data = if let Some(event_attribute) = maybe_packet_data_str {
            event_attribute
                .value_str()
                .map_err(|e| {
                    format!("failed to retrieve `packet_data` attribute value as str. Cause {e}")
                })?
                .as_bytes()
                .to_vec()
        } else if let Some(event_attribute) = maybe_packet_data_hex_str {
            hex::decode(event_attribute.value_str().map_err(|e| {
                format!("failed to retrieve `packet_data` attribute value as str. Cause {e}")
            })?)
            .map_err(|e| format!("failed to decode packet data from hex string. Cause: {e}"))?
        } else {
            return Err("missing `packet_data` and `packet_data_hex` in ABCI Event".to_owned());
        };

        let acknowledgment = if let Some(event_attribute) = maybe_packet_ack_str {
            event_attribute
                .value_str()
                .map_err(|e| {
                    format!("failed to retrieve `packet_ack` attribute value as str. Cause {e}")
                })?
                .as_bytes()
                .to_vec()
        } else if let Some(event_attribute) = maybe_packet_ack_hex_str {
            hex::decode(event_attribute.value_str().map_err(|e| {
                format!("failed to retrieve `packet_ack_hex` attribute value as str. Cause {e}")
            })?)
            .map_err(|e| format!("failed to decode acknowledgment from hex string. Cause: {e}"))?
        } else {
            return Err("missing `packet_ack` and `packet_ack_hex` in ABCI Event".to_owned());
        };

        let packet = Packet {
            seq_on_a: Sequence::from_str(&seq_on_a)
                .map_err(|e| format!("failed to convert string to sequence. Cause: {e}"))?,
            port_id_on_a: PortId::from_str(&port_id_on_a_str)
                .map_err(|e| format!("failed to convert string to port ID. Cause: {e}"))?,
            chan_id_on_a: ChannelId::from_str(&chan_id_on_a_str)
                .map_err(|e| format!("failed to convert string to channel ID. Cause: {e}"))?,
            port_id_on_b: PortId::from_str(&port_id_on_b_str)
                .map_err(|e| format!("failed to convert string to port ID. Cause: {e}"))?,
            chan_id_on_b: ChannelId::from_str(&chan_id_on_b_str)
                .map_err(|e| format!("failed to convert string to channel ID. Cause: {e}"))?,
            data: packet_data,
            timeout_height_on_b,
            timeout_timestamp_on_b,
        };

        Ok(Some(WriteAcknowledgement::new(
            packet,
            acknowledgment
                .try_into()
                .map_err(|e| format!("failed to convert bytes to acknowledgment. Cause: {e}"))?,
            ConnectionId::from_str(&conn_id_on_b_str).map_err(|e| {
                format!("failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}")
            })?,
        )))
    } else {
        Ok(None)
    }
}

pub fn try_send_packet_from_abci_event(event: &Event) -> Result<Option<SendPacket>, String> {
    if event.kind.as_str() == "send_packet" {
        let seq_on_a = find_attribute_as_string(&event.attributes, "packet_sequence")?;
        let port_id_on_a_str = find_attribute_as_string(&event.attributes, "packet_src_port")?;
        let chan_id_on_a_str = find_attribute_as_string(&event.attributes, "packet_src_channel")?;
        let port_id_on_b_str = find_attribute_as_string(&event.attributes, "packet_dst_port")?;
        let chan_id_on_b_str = find_attribute_as_string(&event.attributes, "packet_dst_channel")?;
        let timeout_height_on_b_str =
            find_attribute_as_string(&event.attributes, "packet_timeout_height")?;
        let timeout_timestamp_on_b_str =
            find_attribute_as_string(&event.attributes, "packet_timeout_timestamp")?;
        let conn_id_on_b_str = find_attribute_as_string(&event.attributes, "packet_connection")?;
        let channel_ordering_str =
            find_attribute_as_string(&event.attributes, "packet_channel_ordering")?;

        let maybe_packet_data_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_data"));

        let maybe_packet_data_hex_str = event
            .attributes
            .iter()
            .find(|attribute| attribute.key_str().ok() == Some("packet_data_hex"));

        let timeout_height_on_b = if timeout_height_on_b_str == "0-0" {
            TimeoutHeight::no_timeout()
        } else {
            let timeout_height = Height::from_str(&timeout_height_on_b_str)
                .map_err(|e| format!("failed to convert string to height. Cause: {e}"))?;
            TimeoutHeight::from(timeout_height)
        };

        let timeout_timestamp_on_b = if timeout_timestamp_on_b_str == "0" {
            TimeoutTimestamp::no_timeout()
        } else {
            let timeout_timestamp =
                Timestamp::from_str(&timeout_timestamp_on_b_str).map_err(|e| {
                    format!("failed to convert string to timeout timestamp. Cause: {e}")
                })?;
            TimeoutTimestamp::from(timeout_timestamp)
        };

        let packet_data = if let Some(event_attribute) = maybe_packet_data_str {
            event_attribute
                .value_str()
                .map_err(|e| {
                    format!("failed to retrieve `packet_data` attribute value as str. Cause {e}")
                })?
                .as_bytes()
                .to_vec()
        } else if let Some(event_attribute) = maybe_packet_data_hex_str {
            hex::decode(event_attribute.value_str().map_err(|e| {
                format!("failed to retrieve `packet_data_hex` attribute value as str. Cause {e}")
            })?)
            .map_err(|e| format!("failed to decode packet data from hex string. Cause: {e}"))?
        } else {
            return Err("missing `packet_data` and `packet_data_hex` in ABCI Event".to_owned());
        };

        let packet = Packet {
            seq_on_a: Sequence::from_str(&seq_on_a)
                .map_err(|e| format!("failed to convert string to sequence. Cause: {e}"))?,
            port_id_on_a: PortId::from_str(&port_id_on_a_str)
                .map_err(|e| format!("failed to convert string to port ID. Cause: {e}"))?,
            chan_id_on_a: ChannelId::from_str(&chan_id_on_a_str)
                .map_err(|e| format!("failed to convert string to channel ID. Cause: {e}"))?,
            port_id_on_b: PortId::from_str(&port_id_on_b_str)
                .map_err(|e| format!("failed to convert string to port ID. Cause: {e}"))?,
            chan_id_on_b: ChannelId::from_str(&chan_id_on_b_str)
                .map_err(|e| format!("failed to convert string to channel ID. Cause: {e}"))?,
            data: packet_data,
            timeout_height_on_b,
            timeout_timestamp_on_b,
        };

        Ok(Some(SendPacket::new(
            packet,
            Order::from_str(&channel_ordering_str).map_err(|e| {
                format!("failed to convert `{channel_ordering_str}` to Order. Cause: {e}")
            })?,
            ConnectionId::from_str(&conn_id_on_b_str).map_err(|e| {
                format!("failed to convert `{conn_id_on_b_str}` to ConnectionId. Cause: {e}")
            })?,
        )))
    } else {
        Ok(None)
    }
}

fn find_attribute_as_string(
    event_attributes: &[EventAttribute],
    attribute_key: &str,
) -> Result<String, String> {
    event_attributes
        .iter()
        .find(|attribute| attribute.key_str().ok() == Some(attribute_key))
        .ok_or_else(|| format!("missing attribute `{attribute_key}` in ABCI Event"))?
        .value_str()
        .map(|value| value.to_string())
        .map_err(|e| {
            format!("failed to retrieve `{attribute_key}` attribute value as str. Cause {e}")
        })
}
