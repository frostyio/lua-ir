#[allow(non_snake_case, non_camel_case_types)]
pub enum Operand {
	A,
	B,
	Bx,
	sBx,
	C,
}

pub enum IROperand<T> {
	Operand(Operand, T),
	None,
}

impl<T> IROperand<T> {
	pub fn modify(&mut self, value: T) {
		match self {
			Self::Operand(_, val) => *val = value,
			_ => {}
		}
	}
}

pub trait Context {
	/// Get all instructions by opcode
	fn get_instructions(&self, opcode: usize) -> Vec<usize>;

	/// Only returns instructions that reference a constant
	fn get_constant_instructions(&self) -> Vec<usize>;

	/// Remaps all references of kst1 to kst2
	fn remap_constant(&mut self, kst1: usize, kst2: usize);
}
