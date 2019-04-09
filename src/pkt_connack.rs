use std::io::{BufReader, BufWriter, Read, Write};
use bincode;

use crate::head::Head;
use crate::head::PacketType::PktTypeConnAck;
use crate::utils::*;

#[derive(Default, Debug)]
pub struct PktConnAck {
	pub head: Head,
	pub present: bool,
	pub retcode: u8,
}

impl SerdePacket for PktConnAck {
	fn serialize_into<W>(&mut self, cfg: &bincode::Config, buf: &mut BufWriter<W>) -> bincode::Result<()>
		where
			W: Write,
	{
		let body = Vec::with_capacity(256);
		let mut body = BufWriter::new(body);

		cfg.serialize_into(&mut body, if self.present { &1u8 } else { &0u8 })?;
		cfg.serialize_into(&mut body, &self.retcode)?;

		serialize_head_body(cfg, buf, &mut self.head, PktTypeConnAck, 0, &mut body)
	}

	fn deserialize_from<R: Read>(&mut self, cfg: &bincode::Config, mut r: &mut BufReader<R>) -> bincode::Result<()> {
		self.head.deserialize_from(cfg, &mut r)?;
		let present: u8 = cfg.deserialize_from(&mut r)?;
		self.present = present == 1;
		self.retcode = cfg.deserialize_from(&mut r)?;
		Ok(())
	}
}
