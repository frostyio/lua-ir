#![allow(dead_code, unused)]

mod graphviz;
mod lua;
mod shared;

#[cfg(test)]
mod tests {
	use super::{
		lua::lua51::{blocker, compile, deserialize_bytecode},
		shared::Reader,
	};
	use std::{fs::canonicalize, path::Path};

	#[test]
	fn test() {
		let test_file_path = Path::new("./examples/hello.lua");
		let test_file = canonicalize(test_file_path).expect("Unable to find test file");

		let bc = compile(&test_file).expect("Unable to compile bytecode");
		let (header, proto) = deserialize_bytecode(&bc);

		let blocks = blocker::make_blocks(proto.instructions);
		blocker::make_control_flow(&blocks);
	}
}
