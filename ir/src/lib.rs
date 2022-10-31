#![allow(dead_code, unused)]

mod lua51;

#[cfg(test)]
mod tests {
	use crate::lua51::CFG;
	use bytecode::lua51::{compile, deserialize_bytecode};
	use std::{fs::canonicalize, path::Path};

	#[test]
	fn test() {
		let test_file_path = Path::new("../examples/flow3.lua");
		let test_file = canonicalize(test_file_path).expect("Unable to find test file");

		let bc = compile(&test_file).expect("Unable to compile bytecode");
		let (header, proto) = deserialize_bytecode(&bc);

		let cfg = CFG::new(&proto.instructions);
		let mut iter = cfg.iter().enumerate();
		while let Some((block_idx, block)) = iter.next() {
			println!("[{}]: {:?}", block_idx, block);
		}

		println!("{}", cfg);
	}
}
