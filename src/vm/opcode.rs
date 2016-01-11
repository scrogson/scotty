pub type Instruction = u8;

// arity: 2 - (value: u64, dest: u8)
pub const OP_LOAD_U8: Instruction = 1;

// arity: 3 - (a: u8, b: u8, dest: u8)
pub const OP_ADD: Instruction = 2;

// arity: 3 - (a: u8, b: u8, dest: u8)
pub const OP_SUB: Instruction = 3;

// arity: 3 - (a: u8, b: u8, dest: u8)
pub const OP_MULT: Instruction = 4;

// arity: 2 - (a: u8, b: u8, dest: u8)
pub const OP_LT: Instruction = 5;

// arity: 1 - (reg: u8)
pub const OP_RETURN: Instruction = 6;

// arity
pub const OP_JUMP_TRUE: Instruction = 7;

// arity: 1 - (reg: u8)
pub const OP_PRINT: Instruction = 8;
