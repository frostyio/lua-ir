use std::fmt::{Debug, Display};

use bytecode::lua51::Constants;

pub struct IRConstant {
	val: Constants,
}

impl Debug for IRConstant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.val)
	}
}

impl IRConstant {
	pub fn from_constant(c: Constants) -> Self {
		Self { val: c }
	}

	pub fn string(s: &str) -> Self {
		Self {
			val: Constants::String(s.to_string()),
		}
	}

	pub fn number(n: f64) -> Self {
		Self {
			val: Constants::Number(n),
		}
	}

	pub fn bool(b: bool) -> Self {
		Self {
			val: Constants::Boolean(b),
		}
	}

	pub fn nil() -> Self {
		Self {
			val: Constants::Nil,
		}
	}

	pub fn is_string(&self, s: &str) -> bool {
		match &self.val {
			Constants::String(str) => str.as_str() == s,
			_ => false,
		}
	}

	pub fn is_number(&self, n: f64) -> bool {
		match self.val {
			Constants::Number(f) => f == n,
			_ => false,
		}
	}

	pub fn is_bool(&self, bv: bool) -> bool {
		match self.val {
			Constants::Boolean(b) => b == bv,
			_ => false,
		}
	}

	pub fn is_nil(&self) -> bool {
		match self.val {
			Constants::Nil => true,
			_ => false,
		}
	}
}

#[derive(Debug)]
pub struct IRConstants {
	constants: Vec<IRConstant>,
}

impl IRConstants {
	pub fn from_constants(constants: Vec<Constants>) -> Self {
		Self {
			constants: constants
				.into_iter()
				.map(|c| IRConstant::from_constant(c))
				.collect(),
		}
	}

	pub fn get_all(&self) -> &Vec<IRConstant> {
		&self.constants
	}

	pub fn get_string(&mut self, str: &str) -> Option<usize> {
		self.constants.iter().position(|c| c.is_string(str))
	}

	pub fn get_bool(&mut self, b: bool) -> Option<usize> {
		self.constants.iter().position(|c| c.is_bool(b))
	}

	pub fn get_number(&mut self, n: f64) -> Option<usize> {
		self.constants.iter().position(|c| c.is_number(n))
	}

	pub fn get_nil(&mut self) -> Option<usize> {
		self.constants.iter().position(|c| c.is_nil())
	}

	pub fn add_string(&mut self, str: &str) -> usize {
		if let Some(pos) = self.get_string(str) {
			return pos;
		}

		self.constants
			.push(IRConstant::from_constant(Constants::String(
				str.to_string(),
			)));
		self.constants.len() - 1
	}

	pub fn add_number(&mut self, n: f64) -> usize {
		if let Some(pos) = self.get_number(n) {
			return pos;
		}

		self.constants
			.push(IRConstant::from_constant(Constants::Number(n)));
		self.constants.len() - 1
	}

	pub fn add_bool(&mut self, b: bool) -> usize {
		if let Some(pos) = self.get_bool(b) {
			return pos;
		}

		self.constants
			.push(IRConstant::from_constant(Constants::Boolean(b)));
		self.constants.len() - 1
	}

	pub fn add_nil(&mut self) -> usize {
		if let Some(pos) = self.get_nil() {
			return pos;
		}

		self.constants
			.push(IRConstant::from_constant(Constants::Nil));
		self.constants.len() - 1
	}

	pub fn set(&mut self, idx: usize, val: IRConstant) {
		self.constants[idx] = val;
	}
}
