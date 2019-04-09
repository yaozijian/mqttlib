use std::io::{BufReader, BufWriter, Read, Write};

use bincode;

use crate::head::Head;
use crate::utils::SerdePacket;

#[derive(Default, Debug)]
pub struct RawPacket {
	pub head: Head,
	pub body: Vec<u8>,
}

impl SerdePacket for RawPacket {
	fn serialize_into<W>(&mut self, cfg: &bincode::Config, mut buf: &mut BufWriter<W>) -> bincode::Result<()>
		where W: Write
	{
		self.head.serialize_into(cfg, &mut buf)?;
		buf.write_all(self.body.as_slice())?;
		buf.flush()?;
		Ok(())
	}

	fn deserialize_from<R: Read>(&mut self, cfg: &bincode::Config, mut r: &mut BufReader<R>) -> bincode::Result<()>
	{
		self.head.deserialize_from(cfg, &mut r)?;

		let mut tmp = Vec::with_capacity(self.head.body_len as usize);
		tmp.resize(self.head.body_len as usize, 0);
		r.read_exact(tmp.as_mut_slice())?;

		self.body = tmp;

		Ok(())
	}
}

