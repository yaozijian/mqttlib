use std::io::{BufReader, BufWriter};

use bincode;

use crate::head::Head;
use crate::mqtt_packet::MqttPacket;
use crate::pkt_connect::PktConnect;
use crate::raw_packet::RawPacket;
use crate::utils::*;

#[test]
fn pkt_connect_read_write() -> bincode::Result<()> {
	let mut pkt1 = PktConnect {
		protocol_name: "MQTT".to_string(),
		protocol_level: 4,
		client_id: "TestPktConnect".to_string(),
		clean_session: true,
		keep_alive: 300,
		..Default::default()
	};

	let mut cfg = bincode::config();
	cfg.big_endian();

	let buf = Vec::with_capacity(128);
	let mut buf = BufWriter::new(buf);
	pkt1.serialize_into(&cfg, &mut buf)?;

	let mut buf = BufReader::new(buf.get_ref().as_slice());
	let mut pkt2: PktConnect = Default::default();
	pkt2.deserialize_from(&cfg, &mut buf)?;

	test_head_equal(&pkt1.head, &pkt2.head);
	test_packet_equal(&pkt1, &pkt2);

	Ok(())
}

#[test]
fn pkt_connect_from_common_packet() -> bincode::Result<()> {
	let mut pkt1 = PktConnect {
		protocol_name: "MQTT".to_string(),
		protocol_level: 4,
		client_id: "TestPktConnect".to_string(),
		clean_session: true,
		keep_alive: 300,
		..Default::default()
	};

	let mut cfg = bincode::config();
	cfg.big_endian();

	let buf = Vec::with_capacity(128);
	let mut buf = BufWriter::new(buf);
	pkt1.serialize_into(&cfg, &mut buf)?;

	let mut buf = std::io::BufReader::new(buf.get_ref().as_slice());
	let mut pkt2: RawPacket = Default::default();
	pkt2.deserialize_from(&cfg, &mut buf)?;

	test_head_equal(&pkt1.head, &pkt2.head);

	let pkt2: MqttPacket = pkt2.into();

	if let MqttPacket::MqttPktConnect(pkt2) = pkt2 {
		test_packet_equal(&pkt1, &pkt2);
		Ok(())
	} else {
		invalid_data_error()
	}
}

fn test_head_equal(head1: &Head, head2: &Head) {
	assert_eq!(head1.pkt_type, head2.pkt_type);
	assert_eq!(head1.pkt_flags, head2.pkt_flags);
	assert_eq!(head1.body_len, head2.body_len);
}

fn test_packet_equal(pkt1: &PktConnect, pkt2: &PktConnect) {
	test_head_equal(&pkt1.head, &pkt2.head);
	assert_eq!(pkt1.protocol_name, pkt2.protocol_name);
	assert_eq!(pkt1.protocol_level, pkt2.protocol_level);
	assert_eq!(pkt1.client_id, pkt2.client_id);
	assert_eq!(pkt1.clean_session, pkt2.clean_session);
	assert_eq!(pkt1.keep_alive, pkt2.keep_alive);
}
