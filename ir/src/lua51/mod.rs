mod cfg;
pub use cfg::CFG;
mod context;
mod opcodes;

pub use context::IRContext;
pub use opcodes::get_opcode_name;
