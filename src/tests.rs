use interpreter::*;

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
