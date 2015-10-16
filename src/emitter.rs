use interpreter::*;

pub enum Program {
    IntLiteral(u8),
    Binary {op: u8, left: u8, right: u8}
}

pub fn emit(program: Program) -> Vec<u8> {
    let mut code = vec![];
    match program {
        Program::IntLiteral(n) => {
            code.push(OP_LOAD_U8);
            code.push(n);
            code.push(0);
            code.push(OP_RETURN);
            code.push(0);
        },
        Program::Binary {op, left, right} => {
            code.push(OP_LOAD_U8);
            code.push(left);
            code.push(0);
            code.push(OP_LOAD_U8);
            code.push(right);
            code.push(1);
            code.push(op);
            code.push(0);
            code.push(1);
            code.push(2);
            code.push(OP_RETURN);
            code.push(2);
        }
    }
    code
}

#[test]
fn test_emit() {
    let code1 = emit(Program::IntLiteral(37));
    assert_eq!(37, interpret(&code1));


    let program = Program::Binary {
        op: OP_ADD,
        left: 73,
        right: 68
    };

    let code2 = emit(program);
    assert_eq!(141, interpret(&code2));
}
