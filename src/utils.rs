use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use bincode;
use serde;
use serde::de::Error;

use crate::head::Head;
use crate::head::PacketType;

pub trait SerdePacket {
	fn serialize_into<W>(&mut self, cfg: &bincode::Config, w: &mut BufWriter<W>) -> bincode::Result<()> where W: Write;
	fn deserialize_from<R>(&mut self, cfg: &bincode::Config, r: &mut BufReader<R>) -> bincode::Result<()> where R: Read;
}

pub fn serialize_head_body<W: Write>(
	cfg: &bincode::Config, mut buf: &mut BufWriter<W>,
	head: &mut Head, pkt: PacketType, flags: u8,
	body: &mut BufWriter<Vec<u8>>) -> bincode::Result<()> {
	body.flush()?;

	head.pkt_type = pkt;
	head.pkt_flags = flags;
	head.body_len = body.get_ref().len();

	head.serialize_into(cfg, &mut buf)?;

	buf.write(body.get_ref().as_slice())?;
	buf.flush()?;

	Ok(())
}

pub fn deserialize_vec<R: Read>(
	cfg: &bincode::Config,
	mut r: &mut BufReader<R>,
	item: &mut Vec<u8>,
) -> bincode::Result<()> {
	let len = cfg.deserialize_from::<_, u16>(&mut r)?;

	let mut tmp = Vec::with_capacity(len as usize);

	tmp.resize(len as usize, 0);

	r.read_exact(tmp.as_mut_slice())?;

	*item = tmp;

	Ok(())
}

pub fn deserialize_str<R: Read>(
	cfg: &bincode::Config,
	r: &mut BufReader<R>,
	item: &mut String,
) -> bincode::Result<()> {
	let mut tmp = vec![];

	deserialize_vec(cfg, r, &mut tmp)?;

	*item = String::from_utf8(tmp).map_err(|_| bincode::Error::custom("Invalid UTF-8"))?;

	Ok(())
}

pub fn invalid_data_error() -> bincode::Result<()> {
	Err(Box::new(bincode::ErrorKind::Custom("InvalidData".to_string())))
}
