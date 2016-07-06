//! This module is strictly internal.
//!
//! These functions are used by `log_on()`, `frame()` and `log_off()`.
//! They are currently not private, because the need to be accessible,
//! but are not useful by themselves.

use helper::*;

use std::str;

use nom::{IResult, le_u8, le_u16, le_u32};
use super::{LogOn, Frame, LogOff};

named!(pub parse_log_on<LogOn>,
	chain!(
		f: le_u16 ~ s: le_u8 ~ u: le_u32 ~ p: le_u8 ~ o: le_u32 ~ m: le_u8 ~ c: le_u16,
		|| {
			LogOn {
				frame_size: f, service_id: s, unit_identifier: u, protocol_version: p,
				software_version: o, msg_number: m, crc: c
			}
		}
	)
);

named!(pub parse_frame<Frame>,
	chain!(
		f: le_u16 ~ s: le_u8 ~ e: le_u32 ~ p: le_u32 ~ a: take!(p),
		|| {
			Frame {
				frame_size: f, service_id: s, session_key: e,
				payload_length: p, payload: a.to_vec()
			}
		}
	)
);

named!(pub parse_log_off<LogOff>,
	chain!(
		f: le_u16 ~ s: le_u8 ~ e: le_u32,
		|| {
			LogOff {
				frame_size: f, service_id: s, session_key: e
			}
		}
	)
);

#[cfg(test)]
mod tests {

	use nom::IResult;
	use rustc_serialize::hex::FromHex;

	use ::{LogOn, Frame, LogOff};
	use super::{parse_log_on, parse_frame, parse_log_off};

	#[test]
	fn test_log_on() {
		let hex = "0F001015A0D7130111433141020E1E".from_hex().unwrap();

		assert_eq!(parse_log_on(&hex), IResult::Done(&b""[..], LogOn {
				frame_size: 15, service_id: 16, unit_identifier: 332898325,
				protocol_version: 1, software_version: 1093747473, msg_number: 2, crc: 7694
		}));

		assert!(parse_log_on(&"0F00".from_hex().unwrap()).is_incomplete());
	}

	#[test]
	fn frame() {
		let hex = "380020A0D713102D000000010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap();

		assert_eq!(parse_frame(&hex), IResult::Done(&b""[..], Frame {
				frame_size: 56, service_id: 32,
				session_key: 269735840, payload_length: 45,
				payload: "010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap()
		}));
	}

	#[test]
	fn test_log_off() {
		let hex = "0700F0A0D71310".from_hex().unwrap();

		assert_eq!(parse_log_off(&hex), IResult::Done(&b""[..], LogOff {
				frame_size: 7, service_id: 240, session_key: 269735840
		}));
	}
}