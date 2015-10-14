#![allow(dead_code)]
extern crate byteorder;

use std::env;
use std::io::Read;
use std::io::Result;
use std::fs::File;
use byteorder::{ByteOrder, LittleEndian};

// Bytecode instructions
pub const OP_LOAD_U8: u8 = 1; // arity: 2 - (value: u64, dest: u8)
pub const OP_ADD: u8 = 2; // arity: 3 - (a: u8, b: u8, dest: u8)
pub const OP_SUB: u8 = 3; // arity: 3 - (a: u8, b: u8, dest: u8)
pub const OP_MULT: u8 = 4; // arity: 3 - (a: u8, b: u8, dest: u8)
pub const OP_LT: u8 = 5; // arity: 2 - (a: u8, b: u8, dest: u8)
pub const OP_RETURN: u8 = 6; // arity: 1 - (reg: u8)
pub const OP_JUMP_TRUE: u8 = 7; // arity

fn interpret(code: &[u8]) -> i64 {
    let mut pc = 0; // "Program counter", the index of the current instruction.
    let mut regs = [0i64; 256]; // Array of 256 zeros.

    loop {
        let op = code[pc];
        pc += 1;
        match op {
            OP_LOAD_U8 => {
                let val = code[pc] as i64;
                pc += 1;
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
            OP_SUB => {
                let a = code[pc] as usize;
                pc += 1;
                let b = code[pc] as usize;
                pc += 1;
                let dest = code[pc] as usize;
                pc += 1;
                regs[dest] = regs[a] - regs[b];
            },
            OP_MULT => {
                let a = code[pc] as usize;
                pc += 1;
                let b = code[pc] as usize;
                pc += 1;
                let dest = code[pc] as usize;
                pc += 1;
                regs[dest] = regs[a] * regs[b];
            },
            OP_LT => {
                let a = code[pc] as usize;
                pc += 1;
                let b = code[pc] as usize;
                pc += 1;
                let dest = code[pc] as usize;
                pc += 1;
                regs[dest] = (regs[a] < regs[b]) as i64;
            },
            OP_RETURN => {
                let r = code[pc] as usize;
                return regs[r];
            },
            OP_JUMP_TRUE => {
                let r = code[pc] as usize;
                if regs[r] == 1 {
                    pc += 1;
                    let jump_size = LittleEndian::read_i32(&code[pc..pc + 4]);
                    pc += 4;
                    pc = pc.wrapping_add(jump_size as usize);
                } else {
                    pc += 5;
                }
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
        OP_LOAD_U8, 73, 0, // load the number 73 into R0.
        OP_LOAD_U8, 68, 1, // load the number 68 into R1.
        OP_ADD, 0, 1, 2, // compute R0 + R1 and store it in R2.
        OP_RETURN, 2
    ];

    // should print 73 + 68, which is 141.
    println!("{}", interpret(&code))
}

#[test]
fn test_op_add() {
    let code = [
        OP_LOAD_U8, 73, 0,
        OP_LOAD_U8, 68, 1,
        OP_ADD, 0, 1, 2,
        OP_RETURN, 2
    ];

    assert_eq!(141, interpret(&code));
}

#[test]
fn test_op_sub() {
    let code1 = [
        OP_LOAD_U8, 10, 0,
        OP_LOAD_U8, 8, 1,
        OP_SUB, 0, 1, 2,
        OP_RETURN, 2
    ];

    assert_eq!(2, interpret(&code1));

    let code2 = [
        OP_LOAD_U8, 8, 0,
        OP_LOAD_U8, 10, 1,
        OP_SUB, 0, 1, 2,
        OP_RETURN, 2
    ];

    assert_eq!(-2, interpret(&code2));
}

#[test]
fn test_op_mult() {
    let code = [
        OP_LOAD_U8, 2, 0,
        OP_LOAD_U8, 4, 1,
        OP_MULT, 0, 1, 2,
        OP_RETURN, 2
    ];

    assert_eq!(8, interpret(&code));
}

#[test]
fn test_op_lt() {
    let code1 = [
        OP_LOAD_U8, 73, 0,
        OP_LOAD_U8, 68, 1,
        OP_LT, 0, 1, 2,
        OP_RETURN, 2
    ];

    assert_eq!(0, interpret(&code1));

    let code2 = [
        OP_LOAD_U8, 68, 0,
        OP_LOAD_U8, 73, 1,
        OP_LT, 0, 1, 2,
        OP_RETURN, 2
    ];

    assert_eq!(1, interpret(&code2));
}

#[test]
fn test_loop() {
    let code = [
        // Some bytecode to multiply two numbers.
        OP_LOAD_U8, 6, 0,  // load the number 6 into R0 (`X`, the first number to multiply)
        OP_LOAD_U8, 8, 1,  // load the number 8 into R1 (`Y`, the second number to multiply)
        OP_LOAD_U8, 0, 2,  // load the number 0 into R2 (`i`, the loop counter)
        OP_LOAD_U8, 0, 3,  // load the number 0 into R3 (`acc`, the accumulator)
        OP_LOAD_U8, 1, 4,  // load the number 1 into R4 - it stays there forever

        // for (int i = 0; i < x; i++) {
        //   acc += y;
        // }

        OP_ADD, 1, 3, 3,   // add R1 to R3, store the result in R3
        OP_ADD, 2, 4, 2,   // increment R2 by 1 (because the value in R4 is 1)
        OP_LT, 2, 0, 5,    // is R2 < R0?
        OP_JUMP_TRUE, 5, 0xee, 0xff, 0xff, 0xff, // if so, jump back 18 bytes

        OP_RETURN, 3,      // return contents of R3 (`acc`)
    ];

    assert_eq!(48, interpret(&code));
}
