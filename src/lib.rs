//! TPMS is a parser library for Ldl equipment
//!
//! # Example
//!
//! ```rust
//! let tpms = tpms::log_on("0F001015A0D7130111433141020E1E").unwrap();
//! ```

#[macro_use]
extern crate nom;
extern crate rustc_serialize;
use nom::IResult::*;
use rustc_serialize::hex::FromHex;

#[macro_use]
mod helper;
pub mod parsers;

/// A LogOn data spec
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct LogOn {
	pub frame_size: 					u8,
	pub service_id: 					u8,
	pub unit_identifier:			Vec<u8>,
	pub protocol_version: 			Vec<u8>,
	pub software_version: 			Vec<u8>,
	pub msg_number: 				Vec<u8>,
	pub crc: 						Vec<u8>
}

/// A Frame data spec
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct Frame {
	pub frame_size: 				Vec<u8>,
	pub service_id: 				Vec<u8>,
	pub session_key: 				Vec<u8>,
	pub payload_length:				Vec<u8>,
	pub payload: 					Vec<u8>
}

/// A LogOff data spec
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct LogOff {
	pub frame_size: 			Vec<u8>,
	pub service_id: 			Vec<u8>,
	pub session_key:			Vec<u8>
}

/// Parses a LogOn data string.
///
/// ## Example
///
/// ```rust
/// let log_on = tpms::log_on("0F001015A0D7130111433141020E1E").unwrap();
/// ```
pub fn log_on(string: &str) -> Result<LogOn, String> {
    if let Done(_, parsed) = parsers::parse_log_on(&string.from_hex().unwrap()) {
        Ok(parsed)
    } else {
        Err(format!("Parser Error: {}", string))
    }
}

/// Parses a Frame data string.
///
/// ## Example
///
/// ```rust
/// let frame = tpms::frame("380020A0D713102D000000010101F202B1274A56B098D703A0DACB077B07B0274A565900D80000000D13AE0000074E01010F0A640201ABA6").unwrap();
/// ```
pub fn frame(string: &str) -> Result<Frame, String> {
    if let Done(_, parsed) = parsers::parse_frame(&string.from_hex().unwrap()) {
        Ok(parsed)
    } else {
        Err(format!("Parser Error: {}", string))
    }
}

/// Parses a LogOff data string.
///
/// ## Example
///
/// ```rust
/// let frame = tpms::log_off("0700F0A0D71310").unwrap();
/// ```
pub fn log_off(string: &str) -> Result<LogOff, String> {
    if let Done(_, parsed) = parsers::parse_log_off(&string.from_hex().unwrap()) {
        Ok(parsed)
    } else {
        Err(format!("Parser Error: {}", string))
    }
}