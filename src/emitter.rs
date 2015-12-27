use interpreter::*;

pub enum Program {
    IntLiteral(u8),
    Binary {op: u8, left: Box<Program>, right: Box<Program>}
}

pub fn emit(program: &Program) -> Vec<u8> {
    let mut code = vec![];
    emit_into(&mut code, program, 0);
    code.push(OP_RETURN);
    code.push(0);
    code
}

fn emit_into(code: &mut Vec<u8>, program: &Program, target: u8) {
    match *program {
        Program::IntLiteral(n) => {
            load_u8(code, n, target)
        },
        Program::Binary {op, ref left, ref right} => {
            emit_into(code, &*left, 1);
            emit_into(code, &*right, 2);
            code.push(op);
            code.push(1);
            code.push(2);
            code.push(target);
        }
    }
}

fn load_u8(code: &mut Vec<u8>, value: u8, dest: u8) {
    code.push(OP_LOAD_U8);
    code.push(value);
    code.push(dest);
}

fn int_literal(n: u8) -> Box<Program> {
    Box::new(Program::IntLiteral(n))
}

fn binary_add(a: u8, b: u8) -> Box<Program> {
    let program = Program::Binary{
        op: OP_ADD,
        left: int_literal(a),
        right: int_literal(b)
    };
    Box::new(program)
}

#[test]
fn test_emit_int_literal() {
    let code1 = emit(&Program::IntLiteral(37));
    assert_eq!(37, interpret(&code1));
}

#[test]
fn test_emit_simple_binary_op() {
    let program = Program::Binary {
        op: OP_ADD,
        left: int_literal(73),
        right: int_literal(68)
    };

    let code = emit(&program);
    assert_eq!(141, interpret(&code));
}

#[test]
fn test_emit_nested_binary_op() {
    // 73 + 68 + 50 = 191
    let program = Program::Binary {
        op: OP_ADD,
        left: binary_add(73, 68),
        right: int_literal(50)
    };

    let code = emit(&program);
    assert_eq!(191, interpret(&code));
}
