use crate::traits::{IROperand, Operand};
use bytecode::lua51::instructions::{Instruction, Opcode, Value};

impl IROperand<Value> {
	pub fn get_kst(&self) -> Option<usize> {
		match &self {
			Self::Operand(op, val) => match val {
				Value::Kst(kst) => Some(*kst as usize),
				Value::RK(_) => match val.get_rk() {
					Value::Kst(kst) => Some(kst as usize),
					_ => None,
				},
				_ => None,
			},
			Self::None => None,
		}
	}
}

pub struct IRInstruction {
	opcode: usize,
	val: Opcode,
}

impl IRInstruction {
	pub fn from_instruction((opcode, inst): Instruction) -> Self {
		Self {
			opcode: opcode as usize,
			val: inst,
		}
	}

	#[inline]
	pub fn is(&self, opcode: usize) -> bool {
		self.opcode == opcode
	}

	#[inline]
	pub fn opcode(&self) -> usize {
		self.opcode
	}

	/* messy af */

	#[inline]
	pub fn get_a(&self) -> IROperand<Value> {
		if let Some(a) = self.val.get_a() {
			IROperand::Operand(Operand::A, *a)
		} else {
			IROperand::None
		}
	}

	#[inline]
	pub fn get_b(&self) -> IROperand<Value> {
		if let Some(b) = self.val.get_b() {
			IROperand::Operand(Operand::B, *b)
		} else {
			IROperand::None
		}
	}

	#[inline]
	pub fn get_bx(&self) -> IROperand<Value> {
		if let Some(bx) = self.val.get_bx() {
			IROperand::Operand(Operand::A, *bx)
		} else {
			IROperand::None
		}
	}

	#[inline]
	pub fn get_sbx(&self) -> IROperand<Value> {
		if let Some(sbx) = self.val.get_sbx() {
			IROperand::Operand(Operand::A, *sbx)
		} else {
			IROperand::None
		}
	}

	#[inline]
	pub fn get_c(&self) -> IROperand<Value> {
		if let Some(c) = self.val.get_c() {
			IROperand::Operand(Operand::A, *c)
		} else {
			IROperand::None
		}
	}

	/** even more messier */

	pub fn set_a(&mut self, value: Value) {
		match &mut self.val {
			Opcode::iABC(a, ..)
			| Opcode::iABx(a, ..)
			| Opcode::iAC(a, ..)
			| Opcode::iAsBx(a, ..)
				if a.is_some() =>
			{
				if let Some(a) = a {
					*a = value;
				}
			}
			_ => {}
		}
	}

	pub fn set_b(&mut self, value: Value) {
		match &mut self.val {
			Opcode::iABC(_, b, ..) if b.is_some() => {
				if let Some(b) = b {
					*b = value;
				}
			}
			_ => {}
		}
	}

	pub fn set_bx(&mut self, value: Value) {
		match &mut self.val {
			Opcode::iABx(_, bx) if bx.is_some() => {
				if let Some(bx) = bx {
					*bx = value;
				}
			}
			_ => {}
		}
	}

	pub fn set_sbx(&mut self, value: Value) {
		match &mut self.val {
			Opcode::iAsBx(_, sbx) | Opcode::isBx(sbx) if sbx.is_some() => {
				if let Some(sbx) = sbx {
					*sbx = value;
				}
			}
			_ => {}
		}
	}

	pub fn set_c(&mut self, value: Value) {
		match &mut self.val {
			Opcode::iABC(_, _, c) | Opcode::iAC(_, c) if c.is_some() => {
				if let Some(c) = c {
					*c = value;
				}
			}
			_ => {}
		}
	}

	/* back to cleanliness */
	pub fn modify(&mut self, operand: IROperand<Value>) {
		match operand {
			IROperand::Operand(operand, value) => match operand {
				Operand::A => self.set_a(value),
				Operand::B => self.set_b(value),
				Operand::Bx => self.set_bx(value),
				Operand::sBx => self.set_sbx(value),
				Operand::C => self.set_c(value),
			},
			_ => {}
		}
	}
}

pub struct IRInstructions {
	instructions: Vec<IRInstruction>,
}

impl IRInstructions {
	pub fn from_instructions(insts: Vec<Instruction>) -> Self {
		Self {
			instructions: insts
				.into_iter()
				.map(|inst| IRInstruction::from_instruction(inst))
				.collect(),
		}
	}

	pub fn get(&self, idx: usize) -> Option<&IRInstruction> {
		self.instructions.get(idx)
	}

	pub fn get_mut(&mut self, idx: usize) -> Option<&mut IRInstruction> {
		self.instructions.get_mut(idx)
	}

	pub fn find_all(&self, opcode: usize) -> Vec<usize> {
		let mut all = vec![];

		for (pc, inst) in self.instructions.iter().enumerate() {
			if inst.is(opcode) {
				all.push(pc);
			}
		}

		all
	}

	pub fn iter(&self) -> IRInstructionIterator {
		IRInstructionIterator {
			instructions: self,
			current: 0,
		}
	}
}

pub struct IRInstructionIterator<'a> {
	instructions: &'a IRInstructions,
	current: usize,
}

impl<'a> Iterator for IRInstructionIterator<'a> {
	type Item = &'a IRInstruction;

	fn next(&mut self) -> Option<Self::Item> {
		self.current += 1;
		self.instructions.get(self.current - 1)
	}
}
