use std::io::*;

use bincode;

use PacketType::*;

use crate::utils::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PacketType {
	PktTypeReserved0 = 0,
	PktTypeConnect = 1,
	PktTypeConnAck = 2,
	PktTypePublish = 3,
	PktTypePubAck = 4,
	PktTypePubRec = 5,
	PktTypePubRel = 6,
	PktTypePubComp = 7,
	PktTypeSubscribe = 8,
	PktTypeSubAck = 9,
	PktTypeUnSubscribe = 10,
	PktTypeUnSubAck = 11,
	PktTypePingReq = 12,
	PktTypePingResp = 13,
	PktTypeDisConnect = 14,
	PktTypeReserved15 = 15,
}

impl Default for PacketType {
	fn default() -> Self {
		PktTypeReserved0
	}
}

impl From<u8> for PacketType {
	fn from(v: u8) -> PacketType {
		match v {
			0 => PktTypeReserved0,
			1 => PktTypeConnect,
			2 => PktTypeConnAck,
			3 => PktTypePublish,
			4 => PktTypePubAck,
			5 => PktTypePubRec,
			6 => PktTypePubRel,
			7 => PktTypePubComp,
			8 => PktTypeSubscribe,
			9 => PktTypeSubAck,
			10 => PktTypeUnSubscribe,
			11 => PktTypeUnSubAck,
			12 => PktTypePingReq,
			13 => PktTypePingResp,
			14 => PktTypeDisConnect,
			15 => PktTypeReserved15,
			_ => PktTypeReserved15,
		}
	}
}

const FLAGS_RESERVED_0: u8 = 0;
const FLAGS_QO_S_1: u8 = 2;

#[derive(Default, Debug)]
pub struct Head {
	pub pkt_flags: u8,
	pub pkt_type: PacketType,
	pub body_len: usize,
}

impl SerdePacket for Head {
	fn serialize_into<W>(&mut self, cfg: &bincode::Config, mut buf: &mut BufWriter<W>) -> bincode::Result<()>
		where W: Write
	{
		let mut byte = self.pkt_type as u8;
		byte = (byte & 0x0F) << 4;
		byte += self.pkt_flags & 0x0F;

		cfg.serialize_into(&mut buf, &byte)?;

		let mut remain = self.body_len;
		loop {
			byte = (remain % 128) as u8;
			remain = remain / 128;
			byte = if remain > 0 { byte | 128 } else { byte };
			cfg.serialize_into(&mut buf, &byte)?;
			if remain == 0 {
				break;
			}
		}

		buf.flush()?;

		Ok(())
	}

	fn deserialize_from<R: Read>(&mut self, cfg: &bincode::Config, mut r: &mut BufReader<R>) -> bincode::Result<()> {
		let mut idx = 0u8;
		let mut val: u8;

		loop {
			val = cfg.deserialize_from(&mut r)?;
			match idx {
				0 => {
					self.pkt_flags = val & 0x0F;
					self.pkt_type = PacketType::from((val & 0xF0) >> 4);
					self.body_len = 0;
					if let Err(e) = self.check_packet_flags() {
						return Err(e);
					}
					idx += 1;
				}
				_ => {
					if val < 128 || idx < 4 {
						self.body_len += ((val & 0x7F) as usize) << ((idx - 1) * 7);
						if val < 128 {
							break;
						}
						idx += 1;
					} else {
						return invalid_data_error();
					}
				}
			}
		}

		Ok(())
	}
}

impl Head {
	fn check_packet_flags(&self) -> bincode::Result<()> {
		let mut invalid = false;

		match self.pkt_type {
			PktTypeSubscribe | PktTypeUnSubscribe | PktTypePubRel => {
				invalid = self.pkt_flags != FLAGS_QO_S_1;
			}
			PktTypePublish => (),
			PktTypeReserved0 | PktTypeReserved15 => {
				invalid = true;
			}
			_ => invalid = self.pkt_flags != FLAGS_RESERVED_0,
		}

		if invalid {
			invalid_data_error()
		} else {
			Ok(())
		}
	}
}
