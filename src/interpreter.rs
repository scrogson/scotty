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
        use enum_primitive::FromPrimitive;

        let op = code[pc];
        pc += 1;
        match OpCode::from_u8(op) {
            Some(OpCode::LoadU8) => {
                let val = Term::Integer(code[pc] as i64);
                pc += 1;
                let dest = code[pc] as usize;
                pc += 1;
                regs[dest] = val;
            },
            Some(OpCode::Add) => arith!(a, b, Term::Integer(a + b)),
            Some(OpCode::Sub) => arith!(a, b, Term::Integer(a - b)),
            Some(OpCode::Mult) => arith!(a, b, Term::Integer(a * b)),
            Some(OpCode::LessThan) => arith!(a, b, if a < b { Term::Atom(True) } else { Term::Atom(False) } ),
            Some(OpCode::Return) => {
                let r = code[pc] as usize;
                return Ok(regs[r].clone());
            },
            Some(OpCode::JumpTrue) => {
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
            Some(OpCode::Print) => {
                let r = code[pc] as usize;
                println!("{:?}", regs[r]);
            }
            None => return Err(format!("Invalid opcode at offset {:?}", pc - 1))
        }
    }
}

fn main() {}
