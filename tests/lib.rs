extern crate tpms;
extern crate nom;
extern crate rustc_serialize;

use rustc_serialize::hex::FromHex;
use tpms::*;

#[test]
fn test_parser() {
	// Parse LogOn
	let log_on_hex = "0F001015A0D7130111433141020E1E";
	assert_eq!(log_on(log_on_hex).unwrap(), LogOn {
			frame_size: 15, service_id: 16, unit_identifier: 332898325,
			protocol_version: 1, software_version: 1093747473, msg_number: 2, crc: 7694
	});

	// Parse LogOnAck
	let log_on_ack_hex = "08000AA0D7131000";
	assert_eq!(log_on_ack(log_on_ack_hex).unwrap(), LogOnAck {
		frame_size: 8, service_id: 10, session_key: 269735840, error_code: 0
	});

	// Parse Frame
	let frame_hex = "380020A0D713102D000000010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6";
	assert_eq!(frame(frame_hex).unwrap(), Frame {
			frame_size: 56, service_id: 32,
			session_key: 269735840, payload_length: 45,
			payload: "010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap()
	});

	// Parse FrameAck
	let frame_ack_hex = "04002000";
	assert_eq!(frame_ack(frame_ack_hex).unwrap(), FrameAck {
		frame_size: 4, service_id: 32, error_code: 0
	});

	// Parse LogOff
	let log_off_hex = "0700F0A0D71310";
	assert_eq!(log_off(log_off_hex).unwrap(), LogOff {
			frame_size: 7, service_id: 240, session_key: 269735840
	});

	// Parse LogOffAck
	let log_off_ack_hex = "0400F000";
	assert_eq!(log_off_ack(log_off_ack_hex).unwrap(), LogOffAck {
			frame_size: 4, service_id: 240, error_code: 0
	});
}