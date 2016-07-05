//! This module is strictly internal.
//!
//! These functions are used by `log_on()`, `frame()` and `log_off()`.
//! They are currently not private, because the need to be accessible,
//! but are not useful by themselves.

use helper::*;

use std::str;

use nom::{IResult, digit};
use super::{LogOn, Frame, LogOff};

// --------------------------------
/// parse a u64
fn number(i: &[u8]) -> IResult<&[u8], u64> {
    map_res!(i, digit, |d| str::FromStr::from_str(str::from_utf8(d).unwrap()))
}

named!(inumber<i64>, chain!(
        pre: opt!(tag!("-")) ~
        n: number
        ,
        || {
            match pre {
                Some(_) => -(n as i64),
                None    => n as i64,
            }
        }
        )
    );

fn integer(i: &[u8]) -> IResult<&[u8], i64> {
    let (i2, n) = try_parse!(i, delimited!(char!('i'), inumber, char!('e')));
    IResult::Done(i2, n as i64)
}

// --------------------------------

fn payload_length(i:&[u8]) -> u8 { i.to_vec().iter().fold(0, |sum, x| sum + x) }

named!(parse_frame_size, take!(2));

named!(pub parse_log_on<&[u8], LogOn>,
	chain!(
		f: take!(2) ~ s: take!(1) ~ u: take!(4) ~ p: take!(1) ~ o: take!(4) ~ m: take!(1) ~ c: take!(2),
		|| {
			LogOn {
				frame_size: 15, service_id: 10, unit_identifier: u.to_vec(),
				protocol_version: p.to_vec(), software_version: o.to_vec(),
				msg_number: m.to_vec(), crc: c.to_vec()
			}
		}
	)
);

named!(pub parse_frame<&[u8], Frame>,
	chain!(
		f: take!(2) ~ s: take!(1) ~ e: take!(4) ~ p: take!(4) ~ a: take!(payload_length(p)),
		|| {
			Frame {
				frame_size: f.to_vec(), service_id: s.to_vec(), session_key: e.to_vec(),
				payload_length: p.to_vec(), payload: a.to_vec()
			}
		}
	)
);

named!(pub parse_log_off<&[u8], LogOff>,
	chain!(
		f: take!(2) ~ s: take!(1) ~ e: take!(4),
		|| {
			LogOff {
				frame_size: f.to_vec(), service_id: s.to_vec(), session_key: e.to_vec()
			}
		}
	)
);

#[cfg(test)]
mod tests {

	use nom::IResult;
	use rustc_serialize::hex::FromHex;

	use ::{LogOn, Frame, LogOff};
	use super::{parse_log_on, parse_frame, parse_log_off, integer};

	#[test]
	fn test_log_on() {
		let hex = "0F001015A0D7130111433141020E1E".from_hex().unwrap();

		assert_eq!(parse_log_on(&hex), IResult::Done(&b""[..], LogOn {
				frame_size: 15, service_id: 10,
				unit_identifier: "15A0D713".from_hex().unwrap(), protocol_version: "01".from_hex().unwrap(),
				software_version: "11433141".from_hex().unwrap(), msg_number: "02".from_hex().unwrap(),
				crc: "0E1E".from_hex().unwrap()
		}));
	}

	#[test]
	fn frame() {
		let hex = "380020A0D713102D000000010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap();

		assert_eq!(parse_frame(&hex), IResult::Done(&b""[..], Frame {
				frame_size: "3800".from_hex().unwrap(), service_id: "20".from_hex().unwrap(),
				session_key: "A0D71310".from_hex().unwrap(), payload_length: "2D000000".from_hex().unwrap(),
				payload: "010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6".from_hex().unwrap()
		}));
	}

	#[test]
	fn test_log_off() {
		let hex = "0700F0A0D71310".from_hex().unwrap();

		assert_eq!(parse_log_off(&hex), IResult::Done(&b""[..], LogOff {
				frame_size: "0700".from_hex().unwrap(), service_id: "F0".from_hex().unwrap(),
				session_key: "A0D71310".from_hex().unwrap()
		}));
	}

	fn done<T>(x: T) -> ::nom::IResult<&'static [u8], T> {
        ::nom::IResult::Done(&[][..], x)
    }

    fn done_integer(x: i64) -> ::nom::IResult<&'static [u8], i64> {
        done(x)
    }

    #[test]
    fn integers() {
        let res = 123;
        let data = "i123e".as_bytes();
        assert_eq!(res, integer(data));
    }

}