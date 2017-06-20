enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum OpCode {
        LoadU8 = 1,   // arity: 2 - (value: u64, dest: u8)
        Add = 2,      // arity: 3 - (a: u8, b: u8, dest: u8)
        Sub = 3,      // arity: 3 - (a: u8, b: u8, dest: u8)
        Mult = 4,     // arity: 3 - (a: u8, b: u8, dest: u8)
        LessThan = 5, // arity: 2 - (a: u8, b: u8, dest: u8)
        Return = 6,   // arity: 1 - (reg: u8)
        JumpTrue = 7, // arity
        Print = 8,    // arity: 1 - (reg: u8)
    }
}
