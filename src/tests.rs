use term::Term;
use term::Atom::{Nil, True, False};
use vm::opcode::OpCode::*;
use interpreter::*;

#[test]
fn test_op_add() {
    let code = [
        LoadU8 as u8, 73, 0,
        LoadU8 as u8, 68, 1,
        Add as u8, 0, 1, 2,
        Return as u8, 2
    ];

    assert_eq!(interpret(&code), Ok(Term::Integer(141)));
}

#[test]
fn test_op_sub() {
    let code1 = [
        LoadU8 as u8, 10, 0,
        LoadU8 as u8, 8, 1,
        Sub as u8, 0, 1, 2,
        Return as u8, 2
    ];

    assert_eq!(interpret(&code1), Ok(Term::Integer(2)));

    let code2 = [
        LoadU8 as u8, 8, 0,
        LoadU8 as u8, 10, 1,
        Sub as u8, 0, 1, 2,
        Return as u8, 2
    ];

    assert_eq!(interpret(&code2), Ok(Term::Integer(-2)));
}

#[test]
fn test_op_mult() {
    let code = [
        LoadU8 as u8, 2, 0,
        LoadU8 as u8, 4, 1,
        Mult as u8, 0, 1, 2,
        Return as u8, 2
    ];

    assert_eq!(interpret(&code), Ok(Term::Integer(8)));
}

#[test]
fn test_op_lt() {
    let code1 = [
        LoadU8 as u8, 73, 0,
        LoadU8 as u8, 68, 1,
        LessThan as u8, 0, 1, 2,
        Return as u8, 2
    ];

    assert_eq!(interpret(&code1), Ok(Term::Atom(False)));

    let code2 = [
        LoadU8 as u8, 68, 0,
        LoadU8 as u8, 73, 1,
        LessThan as u8, 0, 1, 2,
        Return as u8, 2
    ];

    assert_eq!(interpret(&code2), Ok(Term::Atom(True)));
}

#[test]
fn test_loop() {
    let code = [
        // Some bytecode to multiply two numbers.
        LoadU8 as u8, 6, 0,  // load the number 6 into R0 (`X`, the first number to multiply)
        LoadU8 as u8, 8, 1,  // load the number 8 into R1 (`Y`, the second number to multiply)
        LoadU8 as u8, 0, 2,  // load the number 0 into R2 (`i`, the loop counter)
        LoadU8 as u8, 0, 3,  // load the number 0 into R3 (`acc`, the accumulator)
        LoadU8 as u8, 1, 4,  // load the number 1 into R4 - it stays there forever

        // for (int i = 0; i < x; i++) {
        //   acc += y;
        // }

        Add as u8, 1, 3, 3,   // add R1 to R3, store the result in R3
        Add as u8, 2, 4, 2,   // increment R2 by 1 (because the value in R4 is 1)
        LessThan as u8, 2, 0, 5,    // is R2 < R0?
        JumpTrue as u8, 5, 0xee, 0xff, 0xff, 0xff, // if so, jump back 18 bytes

        Return as u8, 3,      // return contents of R3 (`acc`)
    ];

    assert_eq!(interpret(&code), Ok(Term::Integer(48)));
}
