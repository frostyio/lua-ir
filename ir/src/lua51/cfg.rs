use bytecode::lua51::instructions::Instruction;
use graphviz::Digraph;
use std::{collections::HashMap, fmt::Display, ops::Range};

#[derive(Debug)]
pub enum Target {
	Jmp(usize),            // jump to block from sBx
	ForLoop(usize, usize), // jump to ForPrep + 1 from sBx or PC + 2
	TForLoop(usize),       // PC + 2
	ForPrep(usize),        // jump to ForLoop block from sBx
	BinCond(usize, usize), // proceeding block with jump, PC + 2 for EQ,LT,GT when skipping jump
	NOP,                   // proceed to next instruction as normal
}

#[derive(Debug)]
pub struct Block {
	target: Target,
	range: Range<usize>,
}

pub fn make_labels(insts: &Vec<Instruction>) -> Vec<Range<usize>> {
	let mut labels = vec![-1, insts.len() as i32 - 1];

	let mut iter = insts.iter().enumerate();
	while let Some((pc, (opcode, inst))) = iter.next() {
		let pc = pc as i32;
		match *opcode {
			23 | 24 | 25 => {
				labels.push(pc);
			}
			22 => {
				labels.push(pc);
				labels.push(pc + inst.get_sbx().unwrap().sbx())
			}
			31 => {
				labels.push(pc);
			}
			32 => {
				labels.push(pc);
				labels.push(pc + inst.get_sbx().unwrap().sbx());
			}
			33 => {
				labels.push(pc);
			}
			_ => {}
		}
	}
	labels.sort();

	let mut blocks = vec![];
	for (i, pc) in labels.iter().enumerate() {
		if let Some(next_pc) = labels.get(i + 1) {
			let range = (*pc + 1) as usize..(*next_pc + 1) as usize;
			if range.len() == 0 {
				continue;
			}
			blocks.push(range);
		}
	}

	blocks
}

fn get_block(labels: &Vec<Range<usize>>, target_pc: usize) -> usize {
	labels
		.iter()
		.position(|rng| rng.contains(&target_pc))
		.to_owned()
		.unwrap() // *SHOULD* never be None
}

pub fn make_blocks(insts: &Vec<Instruction>) -> Vec<Block> {
	let labels = make_labels(&insts);
	let mut blocks = vec![];

	for range in &labels {
		let last_pc = range.clone().max().unwrap();

		let instr = insts.get(last_pc);
		let target = match instr {
			Some((23 | 24 | 25, ..)) => Target::BinCond(
				// EQ,LE,GE all skip the proceeding jump instruction if true
				get_block(&labels, last_pc + 1),
				get_block(&labels, last_pc + 2),
			),
			Some((22, inst)) => Target::Jmp(get_block(
				// the compiler should never point to before 0
				&labels,
				(1 + last_pc as i32 + inst.get_sbx().unwrap().sbx()) as usize,
			)),
			Some((31, inst)) => Target::ForLoop(
				// points to corresponding FORPREP + 1 or PC + 1
				get_block(
					&labels,
					(last_pc as i32 + inst.get_sbx().unwrap().sbx()) as usize + 1,
				),
				get_block(&labels, last_pc + 1),
			),
			Some((33, inst)) => Target::TForLoop(
				// points to corresponding body + 1 or PC + 2
				get_block(&labels, last_pc + 2), // skip proceeding jump
			),
			Some((32, inst)) => Target::ForPrep(get_block(
				// points to corresponding FORLOOP
				&labels,
				(last_pc as i32 + inst.get_sbx().unwrap().sbx()) as usize + 1,
			)),
			_ => Target::NOP,
		};

		blocks.push(Block {
			target,
			range: range.clone(),
		})
	}

	blocks
}

pub fn visualize_blocks(blocks: &Vec<Block>) -> String {
	let mut graph = Digraph::new();
	let start_block = "StartBlock";
	let end_block = &format!("Block{}", blocks.len());

	graph.add_instance(start_block, "Entry");
	graph.add_instance(end_block, "End");

	for (i, block) in blocks.iter().enumerate() {
		let name = format!("Block{}", i);
		match block.target {
			Target::ForPrep(block) | Target::Jmp(block) => {
				graph.add_edge(&name, &format!("Block{}", block), None);
			}
			Target::BinCond(jmp1, jmp2) | Target::ForLoop(jmp1, jmp2) => {
				graph.add_edge(&name, &format!("Block{}", jmp1), None);
				graph.add_edge(&name, &format!("Block{}", jmp2), None);
			}
			Target::TForLoop(block) => {
				graph.add_edge(&name, &format!("Block{}", block), None);
				graph.add_edge(&name, &format!("Block{}", i + 1), None); // add edge to following jump
			}
			Target::NOP => graph.add_edge(&name, &format!("Block{}", i + 1), None),
			_ => {}
		}

		graph.add_instance(&name, &format!("Block {}: {:?}", i, block.range));
	}

	graph
		.indegrees_of(0)
		.iter()
		.filter(|name| name.as_str() != start_block)
		.for_each(|name| graph.add_edge(start_block, name, None));

	format!("{}", graph)
}

pub struct CFG {
	blocks: Vec<Block>,
}

impl CFG {
	pub fn new(insts: &Vec<Instruction>) -> Self {
		Self {
			blocks: make_blocks(insts),
		}
	}

	pub fn get_block(&self, block: usize) -> Option<&Block> {
		self.blocks.get(block)
	}

	pub fn iter(&self) -> CFGIterator {
		CFGIterator {
			cfg: &self,
			current_block: 0,
		}
	}
}

impl Display for CFG {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", visualize_blocks(&self.blocks))
	}
}

pub struct CFGIterator<'a> {
	cfg: &'a CFG,
	current_block: usize,
}

impl<'a> Iterator for CFGIterator<'a> {
	type Item = &'a Block;

	fn next(&mut self) -> Option<Self::Item> {
		self.current_block += 1;
		self.cfg.get_block(self.current_block - 1)
	}
}
