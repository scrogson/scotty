use byteorder::{ByteOrder, LittleEndian};
use std::result::Result;

use term::Term;
use term::Atom::{Nil, True, False};
use vm::opcode::*;

pub fn interpret(code: &[u8]) -> Result<Term, String> {
    let mut pc = 0; // "Program counter", the index of the current instruction.
    let mut regs = Vec::with_capacity(256);
    for _ in 0..256 { regs.push(Term::Atom(Nil)); }

    macro_rules! read_reg {
        () => ({
            let val = code[pc] as usize;
            pc += 1;
            val
        })
    }

    macro_rules! arith {
        ($v1: ident, $v2: ident, $expr: expr) => ({
            let a = read_reg!();
            let b = read_reg!();
            let dest = read_reg!();
            let result = match (&regs[a], &regs[b]) {
                (&Term::Integer($v1), &Term::Integer($v2)) => $expr,
                (v1, v2) => return Err(format!("invalid types for arithmetic: {:?}, {:?}", *v1, *v2))
            };
            regs[dest] = result;
        })
    }

    loop {
        let op = code[pc];
        pc += 1;
        match op {
            OP_LOAD_U8 => {
                let val = Term::Integer(code[pc] as i64);
                pc += 1;
                let dest = code[pc] as usize;
                pc += 1;
                regs[dest] = val;
            },
            OP_ADD => arith!(a, b, Term::Integer(a + b)),
            OP_SUB => arith!(a, b, Term::Integer(a - b)),
            OP_MULT => arith!(a, b, Term::Integer(a * b)),
            OP_LT => arith!(a, b, if a < b { Term::Atom(True) } else { Term::Atom(False) } ),
            OP_RETURN => {
                let r = code[pc] as usize;
                return Ok(regs[r].clone());
            },
            OP_JUMP_TRUE => {
                let r = code[pc] as usize;
                if regs[r] == Term::Atom(True) {
                    pc += 1;
                    let jump_size = LittleEndian::read_i32(&code[pc..pc + 4]);
                    pc += 4;
                    pc = pc.wrapping_add(jump_size as usize);
                } else {
                    pc += 5;
                }
            },
            OP_PRINT => {
                let r = code[pc] as usize;
                println!("{:?}", regs[r]);
            }
            _ => return Err(format!("Invalid opcode at offset {:?}", pc - 1))
        }
    }
}

fn main() {}
