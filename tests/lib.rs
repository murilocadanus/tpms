extern crate tpms;
extern crate nom;
extern crate rustc_serialize;

use nom::IResult;
use rustc_serialize::hex::FromHex;

use tpms::*;
use tpms::parsers::*;

#[test]
fn test_parser() {

		// Parse LogOn
		let hex = "0F001015A0D7130111433141020E1E".from_hex().unwrap();
		assert_eq!(parse_log_on(&hex), IResult::Done(&b""[..], LogOn {
				frame_size: 15, service_id: 10,
				unit_identifier: "15A0D713".from_hex().unwrap(), protocol_version: "01".from_hex().unwrap(),
				software_version: "11433141".from_hex().unwrap(), msg_number: "02".from_hex().unwrap(),
				crc: "0E1E".from_hex().unwrap()
		}));
		//assert!(parse_log_on(b"0F00").is_incomplete());
		//assert!(parse_log_on(b"0F00").is_err());
		//assert!(parse_date(b"pppp").is_err());

		// Parse Frame
		let hex = "380020A0D713102D000000010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap();
		assert_eq!(parse_frame(&hex), IResult::Done(&b""[..], Frame {
				frame_size: "3800".from_hex().unwrap(), service_id: "20".from_hex().unwrap(),
				session_key: "A0D71310".from_hex().unwrap(), payload_length: "2D000000".from_hex().unwrap(),
				payload: "010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap()
		}));

		// Parse LogOff
		let hex = "0700F0A0D71310".from_hex().unwrap();
		assert_eq!(parse_log_off(&hex), IResult::Done(&b""[..], LogOff {
				frame_size: "0700".from_hex().unwrap(), service_id: "F0".from_hex().unwrap(),
				session_key: "A0D71310".from_hex().unwrap()
		}));
}