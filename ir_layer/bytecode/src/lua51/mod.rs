mod compile;
pub use compile::compile;
mod deserialize;
pub use deserialize::deserialize_bytecode;
pub mod instructions;
pub const SIGNATURE: &[u8] = b"\x1BLua";

pub type Header = (u8, u8, u8, u8); // int, size_t, instr, lua_number

#[derive(Debug)]
pub enum Constants {
	Nil,
	Boolean(bool),
	Number(f64),
	String(String),
}

#[derive(Debug)]
pub struct Local(String, u64, u64);

#[derive(Debug)]
pub struct Proto {
	pub source: String,
	pub line_defined: u32,
	pub last_line_defined: u32,
	pub nupvals: u8,
	pub nparams: u8,
	pub is_vararg_flag: u8,
	pub max_stack_size: u8,
	pub instructions: Vec<instructions::Instruction>,
	pub constants: Vec<Constants>,
	pub prototypes: Vec<Self>,
	pub source_lines: Option<Vec<u64>>,
	pub locals: Option<Vec<Local>>,
	pub upvals: Option<Vec<String>>,
}
