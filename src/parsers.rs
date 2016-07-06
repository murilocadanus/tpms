//! This module is strictly internal.
//!
//! These functions are used by `log_on()`, `log_on_ack()`, `frame()`,
//! `frame_ack()`, `log_off()` and `log_off_ack()`.
//! They are currently not private, because the need to be accessible,
//! but are not useful by themselves.

use nom::{le_u8, le_u16, le_u32};
use super::{LogOn, LogOnAck, Frame, FrameAck, LogOff, LogOffAck};

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

named!(pub parse_log_on_ack<LogOnAck>,
	chain!(
		f: le_u16 ~ s: le_u8 ~ e: le_u32 ~ r: le_u8,
		|| {
			LogOnAck {
				frame_size: f, service_id: s, session_key: e, error_code: r
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

named!(pub parse_frame_ack<FrameAck>,
	chain!(
		f: le_u16 ~ s: le_u8 ~ e: le_u8,
		|| {
			FrameAck {
				frame_size: f, service_id: s, error_code: e
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

named!(pub parse_log_off_ack<LogOffAck>,
	chain!(
		f: le_u16 ~ s: le_u8 ~ e: le_u8,
		|| {
			LogOffAck {
				frame_size: f, service_id: s, error_code: e
			}
		}
	)
);

#[cfg(test)]
mod tests {

	use nom::IResult;
	use rustc_serialize::hex::FromHex;

	use ::{LogOn, LogOnAck, Frame, FrameAck, LogOff, LogOffAck};
	use super::{parse_log_on, parse_log_on_ack, parse_frame, parse_frame_ack, parse_log_off, parse_log_off_ack};

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
	fn test_log_on_ack() {
		let hex = "08000AA0D7131000".from_hex().unwrap();

		assert_eq!(parse_log_on_ack(&hex), IResult::Done(&b""[..], LogOnAck {
				frame_size: 8, service_id: 10, session_key: 269735840, error_code: 0
		}));

		assert!(parse_log_on_ack(&"0800".from_hex().unwrap()).is_incomplete());
	}

	#[test]
	fn frame() {
		let hex = "380020A0D713102D000000010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap();

		assert_eq!(parse_frame(&hex), IResult::Done(&b""[..], Frame {
				frame_size: 56, service_id: 32, session_key: 269735840, payload_length: 45,
				payload: "010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap()
		}));
		assert!(parse_frame(&"3800".from_hex().unwrap()).is_incomplete());
	}

	#[test]
	fn frame_ack() {
		let hex = "04002000".from_hex().unwrap();

		assert_eq!(parse_frame_ack(&hex), IResult::Done(&b""[..], FrameAck {
				frame_size: 4, service_id: 32, error_code: 0
		}));
		assert!(parse_frame(&"0400".from_hex().unwrap()).is_incomplete());
	}

	#[test]
	fn test_log_off() {
		let hex = "0700F0A0D71310".from_hex().unwrap();

		assert_eq!(parse_log_off(&hex), IResult::Done(&b""[..], LogOff {
				frame_size: 7, service_id: 240, session_key: 269735840
		}));
		assert!(parse_log_off(&"0700".from_hex().unwrap()).is_incomplete());
	}

	#[test]
	fn test_log_off_ack() {
		let hex = "0400F000".from_hex().unwrap();

		assert_eq!(parse_log_off_ack(&hex), IResult::Done(&b""[..], LogOffAck {
				frame_size: 4, service_id: 240, error_code: 0
		}));
		assert!(parse_log_off_ack(&"0400".from_hex().unwrap()).is_incomplete());
	}
}