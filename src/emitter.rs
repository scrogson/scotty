use interpreter::*;

enum Program {
    IntLiteral(u8)
}

fn emit(program: Program) -> Vec<u8> {
    let mut code = vec![];
    match program {
        Program::IntLiteral(n) => {
            code.push(OP_LOAD_U8);
            code.push(n);
            code.push(0);
            code.push(OP_RETURN);
            code.push(0);
        }
    }
    code
}

#[test]
fn test_emit() {
    let code1 = emit(Program::IntLiteral(37));
    assert_eq!(37, interpret(&code1));
}
