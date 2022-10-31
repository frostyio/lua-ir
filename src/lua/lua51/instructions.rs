#![allow(non_snake_case, non_camel_case_types)]

/* definitions */

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
	Reg(u8),
	Kst(u32),
	RK(u32),
	sBx(i32),
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
	iABC(Option<Value>, Option<Value>, Option<Value>),
	iAC(Option<Value>, Option<Value>),
	iABx(Option<Value>, Option<Value>),
	iAsBx(Option<Value>, Option<Value>),
	isBx(Option<Value>),
}

#[derive(Debug, Clone, Copy)]
pub enum Opmode {
	iABC,
	iAC,
	iABx,
	iAsBx,
	isBx,
}

macro_rules! opcode {
	($type:expr,$a:ident,$b:ident,$c:ident) => {{
		match $type {
			Opmode::iABC => {
				Opcode::iABC(Some(Value::$a(0)), Some(Value::$b(0)), Some(Value::$c(0)))
			}
			_ => panic!(),
		}
	}};
	($type:expr,$a:ident,$b:ident) => {{
		match $type {
			Opmode::iABC => Opcode::iABC(Some(Value::$a(0)), Some(Value::$b(0)), None),
			Opmode::iABx => Opcode::iABx(Some(Value::$a(0)), Some(Value::$b(0))),
			Opmode::iAsBx => Opcode::iAsBx(Some(Value::$a(0)), Some(Value::$b(0))),
			Opmode::iAC => Opcode::iAC(Some(Value::$a(0)), Some(Value::$b(0))),
			_ => todo!(),
		}
	}};
	($type:expr,$a:ident) => {{
		match $type {
			Opmode::iABC => Opcode::iABC(Some(Value::$a(0)), None, None),
			Opmode::iABx => Opcode::iABx(Some(Value::$a(0)), None),
			Opmode::iAsBx => Opcode::iAsBx(Some(Value::$a(0)), None),
			Opmode::isBx => Opcode::isBx(Some(Value::$a(0))),
			_ => todo!(),
		}
	}};
}

#[rustfmt::skip]
pub const OPCODES: [Opcode; 38] = [
	opcode!(Opmode::iABC, Reg, Reg),		// MOVE
	opcode!(Opmode::iABx, Reg, Kst),		// LOADK
	opcode!(Opmode::iABC, Reg, Reg, Reg),	// LOADBOOL
	opcode!(Opmode::iABC, Reg, Reg),		// LOADNIL
	opcode!(Opmode::iABC, Reg, Reg),		// GETUPVAL

	opcode!(Opmode::iABx, Reg, Kst),		// GETGLOBAL
	opcode!(Opmode::iABC, Reg, Reg, RK),	// GETTABLE

	opcode!(Opmode::iABx, Reg, Kst),		// SETGLOBAL
	opcode!(Opmode::iABC, Reg, Reg),		// SETUPVAL
	opcode!(Opmode::iABC, Reg, RK, RK),		// SETTABLE

	opcode!(Opmode::iABC, Reg, Reg, Reg),	// NEWTABLE

	opcode!(Opmode::iABC, Reg, Reg, RK),	// SELF

	opcode!(Opmode::iABC, Reg, RK, RK),		// ADD
	opcode!(Opmode::iABC, Reg, RK, RK),		// SUB
	opcode!(Opmode::iABC, Reg, RK, RK),		// MUL
	opcode!(Opmode::iABC, Reg, RK, RK),		// DIV
	opcode!(Opmode::iABC, Reg, RK, RK),		// MOD
	opcode!(Opmode::iABC, Reg, RK, RK),		// POW
	opcode!(Opmode::iABC, Reg, Reg),		// UNM
	opcode!(Opmode::iABC, Reg, Reg),		// NOT
	opcode!(Opmode::iABC, Reg, Reg),		// LEN

	opcode!(Opmode::iABC, Reg, Reg, Reg),	// CONCAT

	opcode!(Opmode::isBx, sBx),				// JMP : TODO

	opcode!(Opmode::iABC, Reg, RK, RK),		// EQ
	opcode!(Opmode::iABC, Reg, RK, RK),		// LT
	opcode!(Opmode::iABC, Reg, RK, RK),		// LE

	opcode!(Opmode::iABC, Reg, Reg),		// TEST
	opcode!(Opmode::iABC, Reg, Reg, Reg),	// TESTSET

	opcode!(Opmode::iABC, Reg, Reg, Reg),	// CALL
	opcode!(Opmode::iABC, Reg, Reg, Reg),	// TAILCALL
	opcode!(Opmode::iABC, Reg, Reg),		// RETURN

	opcode!(Opmode::iAsBx, Reg, sBx),		// FORLOOP

	opcode!(Opmode::iAsBx, Reg, sBx),		// FORPREP

	opcode!(Opmode::iAC, Reg, Reg),			// TFORLOOP : TODO
	
	opcode!(Opmode::iABC, Reg, Reg, Reg),	// SETLIST

	opcode!(Opmode::iABC, Reg),				// CLOSE
	opcode!(Opmode::iABx, Reg, Reg),		// CLOSURE

	opcode!(Opmode::iABC, Reg, Reg),		// VARARG
];

pub type Instruction = (u8, Opcode);

/* functionality */

impl Value {
	pub fn reg(&self) -> u8 {
		match self {
			Value::Reg(r) => *r,
			_ => panic!(),
		}
	}

	pub fn kst(&self) -> u32 {
		match self {
			Value::Kst(v) => *v,
			_ => panic!(),
		}
	}

	pub fn rk(&self) -> u32 {
		match self {
			Value::RK(v) => *v,
			_ => panic!(),
		}
	}

	pub fn sbx(&self) -> i32 {
		match self {
			Value::sBx(v) => *v,
			_ => panic!(),
		}
	}

	fn set(&mut self, data: u32) {
		match self {
			Value::Reg(v) => *v = data as u8,
			Value::Kst(v) => *v = data,
			Value::RK(v) => *v = data,
			Value::sBx(v) => unimplemented!(),
		}
	}

	fn set_i32(&mut self, data: i32) {
		match self {
			Value::sBx(v) => *v = data,
			_ => unimplemented!(),
		}
	}
}

impl Opcode {
	fn ABC(serialized: u32) -> (u8, u16, u16) {
		(
			((serialized >> (6)) & 0xff) as u8,
			((serialized >> (6 + 8 + 9)) & 0x1ff) as u16,
			((serialized >> (6 + 8)) & 0x1ff) as u16,
		)
	}

	fn ABx(serialized: u32) -> (u8, u32) {
		(
			((serialized >> 6) & 0xff) as u8,
			((serialized >> (6 + 8)) & 0x3ffff) as u32,
		)
	}

	fn AsBx(serialized: u32) -> (u8, i32) {
		(
			((serialized >> 6) & 0xff) as u8,
			((serialized >> (6 + 8)) & 0x3ffff) as i32 - 0x1ffff,
		)
	}

	pub fn from_serialized(serialized: u32) -> Instruction {
		let opcode = (serialized & 0x3f) as u8;
		let mut instruction = OPCODES[opcode as usize].clone();
		match &mut instruction {
			Opcode::iABC(a, b, c) => {
				let (a_val, b_val, c_val) = Self::ABC(serialized);
				if let Some(operand) = a {
					operand.set(a_val.into());
				}
				if let Some(operand) = b {
					operand.set(b_val.into());
				}
				if let Some(operand) = c {
					operand.set(c_val.into());
				}
			}
			Opcode::iABx(a, b) => {
				let (a_val, b_val) = Self::ABx(serialized);
				if let Some(operand) = a {
					operand.set(a_val.into());
				}
				if let Some(operand) = b {
					operand.set(b_val.into());
				}
			}
			Opcode::iAsBx(a, sbx) => {
				let (a_val, b_val) = Self::AsBx(serialized);
				if let Some(operand) = a {
					operand.set(a_val.into());
				}
				if let Some(operand) = sbx {
					operand.set_i32(b_val.into());
				}
			}
			Opcode::isBx(sbx) => {
				let (_, b_val) = Self::AsBx(serialized);
				if let Some(operand) = sbx {
					operand.set_i32(b_val);
				}
			}
			Opcode::iAC(a, c) => {
				let (a_val, b_val, _) = Self::ABC(serialized);
				if let Some(operand) = a {
					operand.set(a_val.into());
				}
				if let Some(operand) = a {
					operand.set(b_val.into());
				}
			}
		}

		(opcode, instruction)
	}

	pub fn get_a(&self) -> &Option<Value> {
		match &self {
			Opcode::iABC(a, _, _) => a,
			Opcode::iABx(a, _) => a,
			Opcode::iAsBx(a, _) => a,
			_ => unreachable!(),
		}
	}

	pub fn get_b(&self) -> &Option<Value> {
		match &self {
			Opcode::iABC(_, b, _) => b,
			_ => unreachable!(),
		}
	}

	pub fn get_bx(&self) -> &Option<Value> {
		match &self {
			Opcode::iABx(_, bx) => bx,
			_ => unreachable!(),
		}
	}

	pub fn get_sbx(&self) -> &Option<Value> {
		match &self {
			Opcode::iAsBx(_, sbx) => sbx,
			Opcode::isBx(sbx) => sbx,
			_ => unreachable!(),
		}
	}

	pub fn get_c(&self) -> &Option<Value> {
		match &self {
			Opcode::iABC(_, _, c) => c,
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Opcode, Value::*};

	#[test]
	fn test_deserilization() {
		// MOVE test
		let serialized = 64;
		// expected: opcode = Move, mode = iABC(1, 0, 0)

		let (opcode, inst) = Opcode::from_serialized(serialized);
		assert_eq!(inst.get_a().unwrap(), Reg(1));
		assert_eq!(inst.get_b().unwrap(), Reg(0));
		assert_eq!(inst.get_c(), &None);
	}
}
