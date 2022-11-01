use crate::traits::{Context, IROperand};
use bytecode::lua51::{instructions::Value, Proto};

use self::instructions::IRInstruction;

mod constants;
mod instructions;

pub type Source = String;
pub type NumberOfUpvalues = u8;
pub type NumberOfParams = u8;
pub type IsVararg = u8;
pub type MaxStackSize = u8;

/**
 * IRContext - A wrapper for a Proto
 * Strips debug information
 */
pub struct IRContext {
	/* normal proto stuff */
	source: Source,

	nupvalues: NumberOfUpvalues,
	nparams: NumberOfParams,
	vararg: IsVararg,

	pub instructions: instructions::IRInstructions,
	pub constants: constants::IRConstants,
	pub closures: Vec<Box<IRContext>>,
}

impl IRContext {
	pub fn from_proto(proto: Proto) -> Self {
		Self {
			source: proto.source,
			nupvalues: proto.nupvals,
			nparams: proto.nparams,
			vararg: proto.is_vararg_flag,
			instructions: instructions::IRInstructions::from_instructions(proto.instructions),
			constants: constants::IRConstants::from_constants(proto.constants),
			closures: proto
				.prototypes
				.into_iter()
				.map(|proto| Box::new(Self::from_proto(proto)))
				.collect(),
		}
	}

	fn get_constant_references(&self, constant_idx: usize) -> Vec<(usize, IROperand<Value>)> {
		let insts = self.get_constant_instructions();
		let mut vals = vec![];

		for pc in insts {
			if let Some(inst) = self.instructions.get(pc) {
				let consts = get_constant_values(inst);
				for kst_operand in consts {
					if let Some(kst) = kst_operand.get_kst() {
						vals.push((pc, kst_operand))
					}
				}
			}
		}

		// vals
		vals
	}
}

fn get_constant_values(inst: &IRInstruction) -> Vec<IROperand<Value>> {
	let mut values = vec![];

	match inst.opcode() {
		// LOADK, GETGLOBAL, SETGLOBAL
		1 | 5 | 7 => values.push(inst.get_bx()),

		// GETTABLE, SELF
		6 | 11 => values.push(inst.get_c()),

		// SETTABLE, ADD, SUB, MUL, DIV, MOD, POW, EQ, LT, LE
		9 | 12 | 13 | 14 | 15 | 16 | 17 | 23 | 24 | 25 => {
			values.push(inst.get_b());
			values.push(inst.get_c())
		}
		_ => {}
	}

	values
		.into_iter()
		.filter(|op| match op {
			IROperand::None => false,
			_ => true,
		})
		.collect()
}

impl Context for IRContext {
	fn get_instructions(&self, opcode: usize) -> Vec<usize> {
		self.instructions.find_all(opcode)
	}

	fn get_constant_instructions(&self) -> Vec<usize> {
		let mut pcs = vec![];

		for (pc, inst) in self.instructions.iter().enumerate() {
			let constants = get_constant_values(inst);
			if constants.len() != 0 {
				pcs.push(pc);
			}
		}

		pcs
	}

	fn remap_constant(&mut self, kst1: usize, kst2: usize) {
		let references = self.get_constant_references(kst1);
		for (pc, mut operand) in references {
			// set new constant in operand
			match &mut operand {
				IROperand::Operand(operand, value) => value.set_constant(kst2),
				_ => {}
			}

			// update instruction
			if let Some(inst) = self.instructions.get_mut(pc) {
				inst.modify(operand);
			}
		}
	}
}
