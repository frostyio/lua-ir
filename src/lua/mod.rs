use std::{error::Error, fmt};

pub mod lua51;

#[derive(Debug)]
pub enum LuaVersion {
	Lua51,
}

#[derive(Debug)]
pub struct Bytecode {
	pub version: LuaVersion,
	pub buff: Vec<u8>,
}

#[derive(Debug)]
pub struct LuaError(String);

impl LuaError {
	pub fn new(message: &str) -> Self {
		Self(message.to_string())
	}
}

impl Error for LuaError {
	fn description(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for LuaError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}