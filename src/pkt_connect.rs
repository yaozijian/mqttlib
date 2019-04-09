use bincode;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::head::Head;
use crate::head::PacketType::PktTypeConnect;
use crate::utils::*;

#[derive(Default, Debug)]
pub struct PktConnect {
	pub head: Head,
	pub protocol_name: String,
	pub protocol_level: u8,
	pub client_id: String,
	pub clean_session: bool,
	//-------
	pub keep_alive: u16,
	pub will_topic: String,
	pub will_msg: Vec<u8>,
	pub user_name: String,
	pub pass_word: String,
}

impl SerdePacket for PktConnect {
	fn serialize_into<W>(&mut self, cfg: &bincode::Config, buf: &mut BufWriter<W>) -> bincode::Result<()>
		where
			W: Write,
	{
		let body = Vec::with_capacity(256);
		let mut body = BufWriter::new(body);

		cfg.serialize_into(&mut body, &(self.protocol_name.len() as u16))?;
		body.write(self.protocol_name.as_bytes())?;

		cfg.serialize_into(&mut body, &self.protocol_level)?;
		cfg.serialize_into(&mut body, &self.get_connect_flags())?;
		cfg.serialize_into(&mut body, &self.keep_alive)?;

		// payload
		cfg.serialize_into(&mut body, &(self.client_id.len() as u16))?;
		body.write(self.client_id.as_bytes())?;

		if self.will_topic.len() > 0 && self.will_msg.len() > 0 {
			cfg.serialize_into(&mut body, &(self.will_topic.len() as u16))?;
			body.write(self.will_topic.as_bytes())?;

			cfg.serialize_into(&mut body, &(self.will_msg.len() as u16))?;
			body.write(self.will_msg.as_slice())?;
		}

		if self.user_name.len() > 0 {
			cfg.serialize_into(&mut body, &(self.user_name.len() as u16))?;
			body.write(self.user_name.as_bytes())?;
			if self.pass_word.len() > 0 {
				cfg.serialize_into(&mut body, &(self.pass_word.len() as u16))?;
				body.write(self.pass_word.as_bytes())?;
			}
		}

		serialize_head_body(cfg, buf, &mut self.head, PktTypeConnect, 0, &mut body)
	}

	fn deserialize_from<R: Read>(&mut self, cfg: &bincode::Config, mut r: &mut BufReader<R>) -> bincode::Result<()> {
		self.head.deserialize_from(cfg, &mut r)?;

		deserialize_str(cfg, r, &mut self.protocol_name)?;

		self.protocol_level = cfg.deserialize_from(&mut r)?;
		let flags = cfg.deserialize_from::<_, u8>(&mut r)?;
		self.keep_alive = cfg.deserialize_from(&mut r)?;

		deserialize_str(cfg, r, &mut self.client_id)?;

		self.clean_session = flags & 0x02 != 0;

		if flags & 0x04 != 0 {
			deserialize_str(cfg, r, &mut self.will_topic)?;
			deserialize_vec(cfg, r, &mut self.will_msg)?;
		}

		if flags & 0x80 != 0 {
			deserialize_str(cfg, r, &mut self.user_name)?;
		}

		if flags & 0x70 != 0 {
			deserialize_str(cfg, r, &mut self.pass_word)?;
		}

		Ok(())
	}
}

impl PktConnect {
	fn get_connect_flags(&self) -> u8 {
		let mut flags = 0u8;

		if self.clean_session {
			flags |= 0x02;
		}

		if self.will_topic.len() > 0 && self.will_msg.len() > 0 {
			flags |= 0x04; // will flag
			flags |= 0x01 << 3; // will qos
			flags |= 0x01 << 5; // will retain
		}

		if self.user_name.len() > 0 {
			flags |= 0x01 << 7;
			if self.pass_word.len() > 0 {
				flags = 0x01 << 6;
			}
		}

		flags
	}
}
