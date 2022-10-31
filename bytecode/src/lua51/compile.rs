use crate::{Bytecode, LuaVersion::Lua51};
use std::{error::Error, path::Path, process::Command};

pub fn compile(file_path: &Path) -> Result<Bytecode, Box<dyn Error>> {
	let script_path = Path::new("../scripts/lua51").canonicalize()?;

	let buffer = Command::new(script_path.to_str().unwrap())
		.arg(file_path.to_str().unwrap())
		.output()?
		.stdout;

	Ok(Bytecode {
		version: Lua51,
		buff: buffer,
	})
}
