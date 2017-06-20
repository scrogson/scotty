use term::Term;
use vm::opcode::*;
use interpreter::*;

pub enum Program {
    Integer(u8),
    Binary {op: u8, left: Box<Program>, right: Box<Program>},
    Print(Box<Program>)
}

pub fn emit(program: &Program) -> Vec<u8> {
    let mut code = vec![];
    emit_into(&mut code, program, 0);
    code.push(OpCode::Return as u8);
    code.push(0);
    code
}

fn emit_into(code: &mut Vec<u8>, program: &Program, target: u8) {
    match *program {
        Program::Integer(n) => {
            load_u8(code, n, target)
        },
        Program::Binary {op, ref left, ref right} => {
            emit_into(code, &*left, target);
            emit_into(code, &*right, target + 1);
            code.push(op);
            code.push(target);
            code.push(target + 1);
            code.push(target);
        },
        Program::Print(ref program) => {
            emit_into(code, &*program, target);
            code.push(OpCode::Print as u8);
        }
    }
}

fn load_u8(code: &mut Vec<u8>, value: u8, dest: u8) {
    code.push(OpCode::LoadU8 as u8);
    code.push(value);
    code.push(dest);
}

fn int(n: u8) -> Box<Program> {
    Box::new(Program::Integer(n))
}

fn add(left: Box<Program>, right: Box<Program>) -> Box<Program> {
    let program = Program::Binary {
        op: OpCode::Add as u8,
        left: left,
        right: right
    };
    Box::new(program)
}

#[test]
fn test_emit_integer() {
    let code = emit(&Program::Integer(37));
    assert_eq!(interpret(&code), Ok(Term::Integer(37)));

    let code1 = emit(&int(56));
    assert_eq!(interpret(&code1), Ok(Term::Integer(56)));

    assert_eq!(interpret(&emit(&int(73))), Ok(Term::Integer(73)));
}

#[test]
fn test_emit_simple_binary_op() {
    let program = Program::Binary {
        op: OpCode::Add as u8,
        left: int(73),
        right: int(68)
    };

    let code = emit(&program);
    assert_eq!(interpret(&code), Ok(Term::Integer(141)));
}

#[test]
fn test_emit_nested_binary_op() {
    // (73 + 68) + 50 = 191
    let program0 = Program::Binary {
        op: OpCode::Add as u8,
        left: add(int(73), int(68)),
        right: int(50)
    };

    let code0 = emit(&program0);
    assert_eq!(interpret(&code0), Ok(Term::Integer(191)));

    // 1 + (2 + 3) = 6
    let program1 = Program::Binary {
        op: OpCode::Add as u8,
        left: int(1),
        right: add(int(2), int(3))
    };

    let code1 = emit(&program1);
    assert_eq!(interpret(&code1), Ok(Term::Integer(6)));
}

#[test]
fn test_emit_deep_nested_binary_op() {
    // (1 + (2 + (3 + (4 + 5))))
    let program = add(int(1), add(int(2), add(int(3), add(int(4), int(5)))));
    let code = emit(&program);
    assert_eq!(interpret(&code), Ok(Term::Integer(15)));
}

#[test]
fn test_emit_print() {
    assert_eq!(interpret(&emit(&Program::Print(add(int(3), int(4))))), Ok(Term::Integer(7)));
    assert_eq!(interpret(&emit(&Program::Print(int(7)))), Ok(Term::Integer(7)));
}
