use std::io::{BufReader, BufWriter};

use bincode;

use crate::head::Head;
use crate::mqtt_packet::MqttPacket;
use crate::pkt_connack::PktConnAck;
use crate::raw_packet::RawPacket;
use crate::utils::*;

#[test]
fn pkt_connack_read_write() -> bincode::Result<()> {
	let mut pkt1 = PktConnAck {
		present: false,
		retcode: 0,
		..Default::default()
	};

	let mut cfg = bincode::config();
	cfg.big_endian();

	let buf = Vec::with_capacity(128);
	let mut buf = BufWriter::new(buf);
	pkt1.serialize_into(&cfg, &mut buf)?;

	let mut buf = BufReader::new(buf.get_ref().as_slice());
	let mut pkt2: PktConnAck = Default::default();
	pkt2.deserialize_from(&cfg, &mut buf)?;

	test_head_equal(&pkt1.head, &pkt2.head);
	test_packet_equal(&pkt1, &pkt2);

	Ok(())
}

#[test]
fn pkt_connack_from_common_packet() -> bincode::Result<()> {
	let mut pkt1 = PktConnAck {
		present: false,
		retcode: 0,
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

	if let MqttPacket::MqttPktConnAck(pkt2) = pkt2 {
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

fn test_packet_equal(pkt1: &PktConnAck, pkt2: &PktConnAck) {
	test_head_equal(&pkt1.head, &pkt2.head);
	assert_eq!(pkt1.present, pkt2.present);
	assert_eq!(pkt1.retcode, pkt2.retcode);
}
