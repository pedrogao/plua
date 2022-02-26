use std::collections::HashMap;

use crate::parse::*;
use crate::opcode::*;

#[derive(Debug)]
pub struct Program {
    // symbols
    pub syms: HashMap<String, Symbol>,
    // 指令
    pub instructions: Vec<OpCode>,
}

fn compile_binary_operation(
    pgrm: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    bop: BinaryOperation,
) {
    // 在栈中，op 是最后入栈的
    compile_expression(pgrm, raw, locals, *bop.left);  // 左边表达式
    compile_expression(pgrm, raw, locals, *bop.right); // 右边表达式
    match bop.operator.value.as_str() {
        "+" => {
            pgrm.instructions.push(OpCode::Add);
        }
        "-" => {
            pgrm.instructions.push(OpCode::Subtract);
        }
        "<" => {
            pgrm.instructions.push(OpCode::LessThan);
        }
        _ => panic!(
            "{}",
            bop.operator
                .loc
                .debug(raw, "Unable to compile binary operation:")
        ),
    }
}

fn compile_function_call(
    pgrm: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    fc: FunctionCall,
) {
    let len = fc.arguments.len();
    for arg in fc.arguments {
        compile_expression(pgrm, raw, locals, arg);
    }

    pgrm.instructions
        .push(OpCode::Call(fc.name.value, len)); // 函数调用
}

fn compile_literal(
    pgrm: &mut Program,
    _: &[char],
    locals: &mut HashMap<String, i32>,
    lit: Literal,
) {
    match lit {
        Literal::Number(i) => {
            let n = i.value.parse::<i32>().unwrap();
            pgrm.instructions.push(OpCode::Store(n));
        }
        Literal::Identifier(ident) => {
            pgrm.instructions
                .push(OpCode::DupPlusFP(locals[&ident.value]));
        }
    }
}

fn compile_expression(
    pgrm: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    exp: Expression,
) {
    match exp {
        Expression::BinaryOperation(bop) => {
            compile_binary_operation(pgrm, raw, locals, bop);
        }
        Expression::FunctionCall(fc) => {
            compile_function_call(pgrm, raw, locals, fc);
        }
        Expression::Literal(lit) => {
            compile_literal(pgrm, raw, locals, lit);
        }
    }
}

fn compile_function_declaration(
    pgrm: &mut Program,
    raw: &[char],
    _: &mut HashMap<String, i32>,
    fd: FunctionDeclaration,
) {
    // Jump to end of function to guard top-level
    let done_label = format!("function_done_{}", pgrm.instructions.len());
    pgrm.instructions
        .push(OpCode::Jump(done_label.clone()));

    let mut new_locals = HashMap::<String, i32>::new();

    let function_index = pgrm.instructions.len() as i32;
    let narguments = fd.parameters.len();
    for (i, param) in fd.parameters.iter().enumerate() {
        pgrm.instructions.push(OpCode::MoveMinusFP(
            i,
            narguments as i32 - (i as i32 + 1),
        ));
        new_locals.insert(param.value.clone(), i as i32);
    }

    for stmt in fd.body {
        compile_statement(pgrm, raw, &mut new_locals, stmt);
    }

    // Overwrite function lookup with total number of locals
    pgrm.syms.insert(
        fd.name.value,
        Symbol {
            location: function_index as i32,
            narguments,
            nlocals: new_locals.keys().len(),
        },
    );

    pgrm.syms.insert(
        done_label,
        Symbol {
            location: pgrm.instructions.len() as i32,
            narguments: 0,
            nlocals: 0,
        },
    );
}

fn compile_return(
    pgrm: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    ret: Return,
) {
    compile_expression(pgrm, raw, locals, ret.expression);
    pgrm.instructions.push(OpCode::Return); // so easy
}

fn compile_if(pgrm: &mut Program, raw: &[char], locals: &mut HashMap<String, i32>, if_: If) {
    compile_expression(pgrm, raw, locals, if_.test); // 编译条件语句
    let done_label = format!("if_else_{}", pgrm.instructions.len());  // 生成 label
    pgrm.instructions.push(OpCode::JumpIfNotZero(done_label.clone())); // if 跳转需要一个label
    for stmt in if_.body {
        compile_statement(pgrm, raw, locals, stmt);
    }
    pgrm.syms.insert(
        done_label,
        Symbol {
            location: pgrm.instructions.len() as i32 - 1,
            nlocals: 0,
            narguments: 0,
        },
    );
}

fn compile_local(
    pgrm: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    local: Local,
) {
    let index = locals.keys().len();
    locals.insert(local.name.value, index as i32); // 记录序号
    compile_expression(pgrm, raw, locals, local.expression); // 生成变量的表达式
    pgrm.instructions.push(OpCode::MovePlusFP(index));
}

fn compile_statement(
    pgrm: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    stmt: Statement,
) {
    match stmt {
        Statement::FunctionDeclaration(fd) => compile_function_declaration(pgrm, raw, locals, fd),
        Statement::Return(r) => compile_return(pgrm, raw, locals, r),
        Statement::If(if_) => compile_if(pgrm, raw, locals, if_),
        Statement::Local(loc) => compile_local(pgrm, raw, locals, loc),
        Statement::Expression(e) => compile_expression(pgrm, raw, locals, e),
    }
}

// 编译 ast 树，生成字节码
pub fn compile(raw: &[char], ast: Ast) -> Program {
    // TODO 暂时只支持 i32
    let mut locals: HashMap<String, i32> = HashMap::new();
    let mut pgrm = Program {
        syms: HashMap::new(),
        instructions: Vec::new(),
    };
    for stmt in ast {
        compile_statement(&mut pgrm, raw, &mut locals, stmt);
    }

    pgrm
}