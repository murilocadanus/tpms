extern crate tpms;
extern crate nom;
extern crate rustc_serialize;

use rustc_serialize::hex::FromHex;
use tpms::*;

#[test]
fn test_parser() {
	// Parse LogOn
	let hex = "0F001015A0D7130111433141020E1E";
	assert_eq!(log_on(hex).unwrap(), LogOn {
			frame_size: 15, service_id: 16, unit_identifier: 332898325,
			protocol_version: 1, software_version: 1093747473, msg_number: 2, crc: 7694
	});

	// Parse Frame
	let hex = "380020A0D713102D000000010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6";
	assert_eq!(frame(hex).unwrap(), Frame {
			frame_size: 56, service_id: 32,
			session_key: 269735840, payload_length: 45,
			payload: "010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap()
	});

	// Parse LogOff
	let hex = "0700F0A0D71310";
	assert_eq!(log_off(hex).unwrap(), LogOff {
			frame_size: 7, service_id: 240, session_key: 269735840
	});
}