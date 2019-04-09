use std::io::{BufReader, BufWriter};

use bincode;

use crate::head::PacketType::*;
use crate::pkt_connack::PktConnAck;
use crate::pkt_connect::PktConnect;
use crate::raw_packet::RawPacket;
use crate::utils::*;

#[derive(Debug)]
pub enum MqttPacket {
	MqttPktInvalid,
	MqttPktConnect(PktConnect),
	MqttPktConnAck(PktConnAck),
}

impl From<RawPacket> for MqttPacket {
	fn from(pkt: RawPacket) -> MqttPacket {
		match pkt.head.pkt_type {
			PktTypeConnect => PktConnect::from_raw_packet(pkt, MqttPacket::MqttPktConnect),
			PktTypeConnAck => PktConnAck::from_raw_packet(pkt, MqttPacket::MqttPktConnAck),
			_ => MqttPacket::MqttPktInvalid,
		}
	}
}

trait FromRawPacket {
	fn from_raw_packet<T: SerdePacket + Default>(mut packet: RawPacket, tomqtt: fn(T) -> MqttPacket) -> MqttPacket {
		let buf = Vec::with_capacity(256);
		let mut buf = BufWriter::new(buf);

		let mut cfg = bincode::config();
		cfg.big_endian();

		if packet.serialize_into(&cfg, &mut buf).is_ok() {
			let mut dstpkt: T = Default::default();
			let mut buf = BufReader::new(buf.get_ref().as_slice());
			if dstpkt.deserialize_from(&cfg, &mut buf).is_ok() {
				return tomqtt(dstpkt);
			}
		}

		MqttPacket::MqttPktInvalid
	}
}

impl<T> FromRawPacket for T where T: SerdePacket + Default {}
