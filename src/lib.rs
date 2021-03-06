//! TPMS is a parser library for Ldl equipment

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
	pub frame_size: 					u16,
	pub service_id: 					u8,
	pub unit_identifier:				u32,
	pub protocol_version: 				u8,
	pub software_version: 				u32,
	pub msg_number: 					u8,
	pub crc: 							u16
}

/// A LogOnAck data spec
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct LogOnAck {
	pub frame_size: 					u16,
	pub service_id: 					u8,
	pub session_key: 					u32,
	pub error_code:						u8
}

/// A Frame data spec
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct Frame {
	pub frame_size: 					u16,
	pub service_id: 					u8,
	pub session_key: 					u32,
	pub payload_length:					u32,
	pub payload: 						Vec<u8>
}

/// A FrameAck data spec
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct FrameAck {
	pub frame_size: 					u16,
	pub service_id: 					u8,
	pub error_code:						u8
}

/// A LogOff data spec
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct LogOff {
	pub frame_size: 					u16,
	pub service_id: 					u8,
	pub session_key:					u32
}

/// A LogOffAck data spec
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct LogOffAck {
	pub frame_size: 					u16,
	pub service_id: 					u8,
	pub error_code:						u8
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

/// Parses a LogOnAck data string.
///
/// ## Example
///
/// ```rust
/// let log_on_ack = tpms::log_on_ack("080010A0D7131000").unwrap();
/// ```
pub fn log_on_ack(string: &str) -> Result<LogOnAck, String> {
	if let Done(_, parsed) = parsers::parse_log_on_ack(&string.from_hex().unwrap()) {
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

/// Parses a FrameAck data string.
///
/// ## Example
///
/// ```rust
/// let frame_ack = tpms::frame("04002000").unwrap();
/// ```
pub fn frame_ack(string: &str) -> Result<FrameAck, String> {
	if let Done(_, parsed) = parsers::parse_frame_ack(&string.from_hex().unwrap()) {
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
/// let log_off = tpms::log_off("0700F0A0D71310").unwrap();
/// ```
pub fn log_off(string: &str) -> Result<LogOff, String> {
	if let Done(_, parsed) = parsers::parse_log_off(&string.from_hex().unwrap()) {
		Ok(parsed)
	} else {
		Err(format!("Parser Error: {}", string))
	}
}

/// Parses a LogOffAck data string.
///
/// ## Example
///
/// ```rust
/// let log_off_ack = tpms::log_off_ack("0400F000").unwrap();
/// ```
pub fn log_off_ack(string: &str) -> Result<LogOffAck, String> {
	if let Done(_, parsed) = parsers::parse_log_off_ack(&string.from_hex().unwrap()) {
		Ok(parsed)
	} else {
		Err(format!("Parser Error: {}", string))
	}
}