pub const RET: u8 = 0x00;

pub const PUSH_I64: u8 = 0x11;
pub const PUSH_CONST: u8 = 0x12;

pub const ADD: u8 = 0x22;
pub const SUB: u8 = 0x23;
pub const MUL: u8 = 0x24;
pub const DIV: u8 = 0x25;
pub const EQ: u8 = 0x26;
pub const NE: u8 = 0x27;
pub const LT: u8 = 0x28;
pub const LE: u8 = 0x29;
pub const GT: u8 = 0x2A;
pub const GE: u8 = 0x2B;

pub const STORE: u8 = 0x30;
pub const LOAD: u8 = 0x31;

pub const JUMP: u8 = 0x40;
pub const JUMP_FALSE: u8 = 0x41;