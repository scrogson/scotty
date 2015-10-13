#![allow(dead_code)]
use std::env;
use std::io::Read;
use std::io::Result;
use std::fs::File;

// Bytecode instructions
const OP_LOAD: u8 = 1; // arity: 2 - (value: u64, dest: u8)
const OP_ADD: u8 = 2; // arity: 3 - (a: u8, b: u8, dest: u8)
const OP_LT: u8 = 3; // arity: 2 - (a: u8, b: u8, dest: u8)
const OP_RETURN: u8 = 4; // arity: 1 - (reg: u8)

fn interpret(code: &[u8]) -> u64 {
    let mut pc = 0; // "Program counter", the index of the current instruction.
    let mut regs = [0u64; 256]; // Array of 256 zeros.

    loop {
        let op = code[pc];
        pc += 1;
        match op {
            OP_LOAD => {
                // Read 64 bits from the code (BUG: right now I'm only reading
                // one byte of the number!
                let val = code[pc] as u64;
                pc += 8;
                let dest = code[pc] as usize;
                pc += 1;
                regs[dest] = val;
            },
            OP_ADD => {
                let a = code[pc] as usize;
                pc += 1;
                let b = code[pc] as usize;
                pc += 1;
                let dest = code[pc] as usize;
                pc += 1;
                regs[dest] = regs[a] + regs[b];
            },
            OP_LT => {
                let a = code[pc] as usize;
                pc += 1;
                let b = code[pc] as usize;
                pc += 1;
                let dest = code[pc] as usize;
                pc += 1;
                regs[dest] = (a < b) as u64;
            },
            OP_RETURN => {
                let r = code[pc] as usize;
                return regs[r];
            },
            _ => panic!("Invalid opcode at offset {}", pc)
        }
    }
}

fn read_file(filename: &str) -> Result<Vec<u8>> {
    let mut f = try!(File::open(filename));
    let mut result = vec![];
    try!(f.read_to_end(&mut result));
    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?} args: {:?}", args.len() - 1, &args[1..]);

    let code = [
        OP_LOAD, 73, 0, 0, 0, 0, 0, 0, 0, 0, // load the number 73 into R0.
        OP_LOAD, 68, 0, 0, 0, 0, 0, 0, 0, 1, // load the number 68 into R1.
        OP_ADD, 0, 1, 2, // compute R0 + R1 and store it in R2.
        OP_RETURN, 2
    ];

    // should print 73 + 68, which is 141.
    println!("{}", interpret(&code))
}

#[test]
fn test_op_add() {
    let code = [
        OP_LOAD, 73, 0, 0, 0, 0, 0, 0, 0, 0,
        OP_LOAD, 68, 0, 0, 0, 0, 0, 0, 0, 1,
        OP_ADD, 0, 1, 2,
        OP_RETURN, 2
    ];

    assert_eq!(141, interpret(&code))
}

#[test]
fn test_op_lt() {
    let code = [
        OP_LOAD, 73, 0, 0, 0, 0, 0, 0, 0, 0,
        OP_LOAD, 68, 0, 0, 0, 0, 0, 0, 0, 1,
        OP_LT, 0, 1, 2,
        OP_RETURN, 2
    ];

    assert_eq!(1, interpret(&code))
}
