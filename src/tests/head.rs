use std::io::BufWriter;

use bincode;

use crate::head::*;
use crate::head::PacketType::PktTypePingReq;
use crate::utils::SerdePacket;

#[test]
fn head_read_write() -> bincode::Result<()> {
	let mut head1 = Head {
		pkt_type: PktTypePingReq,
		body_len: 129,
		..Default::default()
	};

	let mut cfg = bincode::config();
	cfg.big_endian();

	let buf = Vec::with_capacity(128);
	let mut w = BufWriter::new(buf);
	head1.serialize_into(&cfg, &mut w)?;

	let mut buf = std::io::BufReader::new(w.get_ref().as_slice());
	let mut head2: Head = Default::default();

	head2.deserialize_from(&cfg, &mut buf)?;

	assert_eq!(head1.pkt_type, head2.pkt_type);
	assert_eq!(head1.pkt_flags, head2.pkt_flags);
	assert_eq!(head1.body_len, head2.body_len);

	Ok(())
}
