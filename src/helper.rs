/*
use std::str::{FromStr, from_utf8_unchecked};

pub fn to_string(s: &[u8]) -> &str {
	unsafe { from_utf8_unchecked(s) }
}

pub fn to_u8(s: &str) -> u8 {
	FromStr::from_str(s).unwrap()
	//s.parse::<u8>().unwrap()
}

pub fn buf_to_u8(s: &[u8]) -> u8 {
	to_u8(to_string(s))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_buf_to_u8() {
		assert_eq!(15, buf_to_u8(&b"15"[..]));
	}
}
*/