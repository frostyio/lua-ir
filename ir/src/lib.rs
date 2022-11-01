#![allow(dead_code, unused)]

mod lua51;
mod traits;

#[cfg(test)]
mod tests {
	use crate::{
		lua51::{IRContext, CFG},
		traits::Context,
	};
	use bytecode::lua51::{compile, deserialize_bytecode, Header, Proto};
	use std::{fs::canonicalize, path::Path};

	const TEST_FILE: &str = "../examples/test1.lua";

	fn compile_test_file() -> (Header, Proto) {
		let test_file_path = Path::new(TEST_FILE);
		let test_file = canonicalize(test_file_path).expect("Unable to find test file");

		let bc = compile(&test_file).expect("Unable to compile bytecode");
		deserialize_bytecode(&bc)
	}

	#[test]
	fn test() {
		let (header, proto) = compile_test_file();

		let cfg = CFG::new(&proto.instructions);
		let mut iter = cfg.iter().enumerate();
		while let Some((block_idx, block)) = iter.next() {
			println!("[{}]: {:?}", block_idx, block);
		}

		println!("{}", cfg);
	}

	#[test]
	fn test_ir_context() {
		let (header, proto) = compile_test_file();
		let mut context = IRContext::from_proto(proto);

		println!("current constants: {:?}", context.constants.get_all());
		let ok_idx = context.constants.get_string("ok").unwrap();
		let idx = context.constants.add_string("fake ok!");
		println!("new constants: {:?}", context.constants.get_all());

		// TODO: does not seem to remap
		context.remap_constant(ok_idx, idx);

		println!("{}", context);

		// let kst_insts = context.get_constant_instructions();
		// println!("abc: {:?}", kst_insts);
	}
}
