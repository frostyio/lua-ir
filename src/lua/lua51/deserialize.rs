use super::{
	instructions::{Instruction, Opcode},
	Constants, Header, Local, Proto, SIGNATURE,
};
use crate::{lua::Bytecode, shared::Reader};

fn header(reader: &mut Reader) -> Header {
	assert_eq!(reader.bytes(4), SIGNATURE);
	assert_eq!(reader.byte(), b'\x51');
	assert_eq!(reader.byte(), 0);
	assert_eq!(reader.byte(), 1); // no support for big endian yet
	let int = reader.byte();
	let size_t = reader.byte();
	let instr = reader.byte();
	let lua_number = reader.byte();
	assert_eq!(reader.byte(), 0);

	println!(
		"int size: {}, size_t: {}, lua_numbr size: {}",
		int, size_t, lua_number
	);

	(int, size_t, instr, lua_number)
}

fn load_vec<V>(
	reader: &mut Reader,
	header: &Header,
	read: fn(&mut Reader, header: &Header) -> V,
) -> Vec<V> {
	let n = reader.int(header.0 as usize);
	let mut list: Vec<V> = vec![];
	for _ in 0..n {
		list.push(read(reader, header));
	}

	list
}

fn chunk(reader: &mut Reader, header: &Header) -> Proto {
	let source = reader.string(header.1);
	let line_defined = reader.int(header.0 as usize) as u32;
	let last_line_defined = reader.int(header.0 as usize) as u32;
	let nupvals = reader.byte();
	let nparams = reader.byte();
	let is_vararg_flag = reader.byte();
	let max_stack_size = reader.byte();

	// instructions
	let instructions = load_vec(reader, header, |reader, header| {
		reader.int(header.2 as usize)
	})
	.iter()
	.map(|serialized| Opcode::from_serialized(*serialized as u32))
	.collect::<Vec<Instruction>>();

	// constants
	let constants = load_vec(reader, header, |reader, header| {
		let t = reader.byte();
		match t {
			0 => Constants::Nil,
			1 => Constants::Boolean(reader.byte() != 0u8),
			3 => Constants::Number(reader.number(header.0 as usize)),
			4 => Constants::String(reader.string(header.1)),
			_ => unreachable!(),
		}
	});

	// prototypes
	let prototypes = load_vec(reader, header, chunk);

	// source lines
	let source_lines = Some(load_vec(reader, header, |reader, header| {
		reader.int(header.0 as usize)
	}));

	// local list
	let locals = Some(load_vec(reader, header, |reader, header| {
		let name = reader.string(header.1);
		let start = reader.int(header.0 as usize);
		let end = reader.int(header.0 as usize);
		Local(name, start, end)
	}));

	// upvalues list
	let upvals = Some(load_vec(reader, header, |reader, header| {
		reader.string(header.1)
	}));

	Proto {
		source,
		line_defined,
		last_line_defined,
		nupvals,
		nparams,
		is_vararg_flag,
		max_stack_size,
		instructions,
		constants,
		prototypes,
		source_lines,
		locals,
		upvals,
	}
}

pub fn deserialize_bytecode(bytecode: &Bytecode) -> (Header, Proto) {
	let mut reader = Reader::from(&bytecode.buff);

	let header_data = header(&mut reader);
	let proto = chunk(&mut reader, &header_data);

	(header_data, proto)
}
