use bincode;
use std::io::BufWriter;

use crate::head::Head;
use crate::head::PacketType::*;
use crate::raw_packet::RawPacket;
use crate::utils::SerdePacket;

#[test]
fn raw_packet_read_write() -> bincode::Result<()> {
	let body: Vec<u8> = vec![90, 91, 92, 93, 94, 95];

	let head1 = Head {
		pkt_type: PktTypePingReq,
		body_len: body.len(),
		..Default::default()
	};

	let mut pkt1 = RawPacket {
		head: head1,
		body,
	};

	let mut cfg = bincode::config();
	cfg.big_endian();

	let buf = Vec::with_capacity(128);
	let mut buf = BufWriter::new(buf);
	pkt1.serialize_into(&cfg, &mut buf)?;

	let mut buf = std::io::BufReader::new(buf.get_ref().as_slice());
	let mut pkt2: RawPacket = Default::default();

	pkt2.deserialize_from(&cfg, &mut buf)?;

	// Is packet equal ?
	let head1 = &pkt1.head;
	let head2 = &pkt2.head;
	assert_eq!(head1.pkt_type, head2.pkt_type);
	assert_eq!(head2.pkt_flags, head2.pkt_flags);
	assert_eq!(head2.body_len, head2.body_len);
	assert_eq!(pkt1.body, pkt2.body);

	Ok(())
}
