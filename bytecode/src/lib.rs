pub mod lua51;
pub mod shared;

#[derive(Debug)]
pub enum LuaVersion {
	Lua51,
}

#[derive(Debug)]
pub struct Bytecode {
	pub version: LuaVersion,
	pub buff: Vec<u8>,
}
