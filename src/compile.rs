use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::bytecode::*;
use crate::parse::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Int(i32),
    Nil,
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(i) => {
                return if let Value::Int(j) = rhs {
                    Value::Int(i + j)
                } else {
                    Value::Nil
                }
            }
            Value::Nil => Value::Nil,
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        if let (Value::Int(x), Value::Int(y)) = (self, rhs) {
            *x += y;
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(i) => {
                return if let Value::Int(j) = rhs {
                    Value::Int(i - j)
                } else {
                    Value::Nil
                }
            }
            Value::Nil => Value::Nil,
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        if let (Value::Int(x), Value::Int(y)) = (self, rhs) {
            *x -= y;
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(i) => {
                return if let Value::Int(j) = rhs {
                    Value::Int(i * j)
                } else {
                    Value::Nil
                }
            }
            Value::Nil => Value::Nil,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(i) => {
                return if let Value::Int(j) = rhs {
                    Value::Int(i / j)
                } else {
                    Value::Nil
                }
            }
            Value::Nil => Value::Nil,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => {
                write!(f, "{}", i)
            }
            Value::Nil => {
                write!(f, "Nil")
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Program {
    // symbols
    pub syms: HashMap<String, Symbol>,
    // 指令
    pub instructions: Vec<OpCode>,
    // bytecode
    // 指令流
    pub chunk: Vec<ByteCode>,
    // 常数
    pub constants: Vec<Value>,
    // 标签
    pub labels: Vec<String>,
}

impl Program {
    pub fn write_byte(&mut self, op: ByteCode) {
        self.chunk.push(op)
    }

    pub fn read_byte(&self, i: usize) -> &ByteCode {
        // 注意越界
        assert!(i < self.chunk.len());
        self.chunk[i].borrow()
    }

    pub fn write_constant(&mut self, v: Value) -> usize {
        let index = self.constants.len();
        self.constants.push(v);
        return index;
    }

    pub fn read_constant(&self, i: usize) -> &Value {
        assert!(i < self.constants.len());
        self.constants[i].borrow()
    }

    pub fn write_label(&mut self, label: String) -> usize {
        let index = self.labels.len();
        self.labels.push(label);
        return index;
    }

    pub fn read_label(&self, i: usize) -> &String {
        assert!(i < self.labels.len());
        self.labels[i].borrow()
    }
}

fn compile_binary_operation(
    prog: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    bop: BinaryOperation,
) {
    // 在栈中，op 是最后入栈的
    compile_expression(prog, raw, locals, *bop.left); // 左边表达式
    compile_expression(prog, raw, locals, *bop.right); // 右边表达式
    match bop.operator.value.as_str() {
        "+" => {
            prog.instructions.push(OpCode::Add);
        }
        "-" => {
            prog.instructions.push(OpCode::Subtract);
        }
        "<" => {
            prog.instructions.push(OpCode::LessThan);
        }
        ">" => {}
        _ => panic!(
            "{}",
            bop.operator
                .loc
                .debug(raw, "Unable to compile binary operation:")
        ),
    }
}

fn compile_function_call(
    prog: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    fc: FunctionCall,
) {
    let len = fc.arguments.len();
    for arg in fc.arguments {
        compile_expression(prog, raw, locals, arg);
    }

    prog.instructions.push(OpCode::Call(fc.name.value, len)); // 函数调用
}

fn compile_literal(
    prog: &mut Program,
    _: &[char],
    locals: &mut HashMap<String, i32>,
    lit: Literal,
) {
    match lit {
        Literal::Number(i) => {
            let n = i.value.parse::<i32>().unwrap();
            prog.instructions.push(OpCode::Store(n)); // 将数字入栈
        }
        Literal::Identifier(ident) => {
            prog.instructions
                .push(OpCode::DupPlusFP(locals[&ident.value])); // 将标识符入栈
        }
    }
}

fn compile_expression(
    prog: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    exp: Expression,
) {
    match exp {
        Expression::BinaryOperation(bop) => {
            compile_binary_operation(prog, raw, locals, bop);
        }
        Expression::FunctionCall(fc) => {
            compile_function_call(prog, raw, locals, fc);
        }
        Expression::Literal(lit) => {
            compile_literal(prog, raw, locals, lit);
        }
    }
}

fn compile_function_declaration(
    prog: &mut Program,
    raw: &[char],
    _: &mut HashMap<String, i32>,
    fd: FunctionDeclaration,
) {
    // Jump to end of function to guard top-level, 函数声明时，无需调用，所以直接跳到函数末尾
    let done_label = format!("function_done_{}", prog.instructions.len());
    prog.instructions.push(OpCode::Jump(done_label.clone())); // 完成指令

    // 处理参数
    let mut new_locals = HashMap::<String, i32>::new();

    let function_index = prog.instructions.len() as i32; // 当前函数字节序列
    let narguments = fd.parameters.len(); // 参数个数

    for (i, param) in fd.parameters.iter().enumerate() {
        prog.instructions.push(OpCode::GetParameter(
            // 用于参数拷贝
            i,                                  // 参数的序号
            narguments as i32 - (i as i32 + 1), // 拷贝的偏移
        ));
        new_locals.insert(param.value.clone(), i as i32); // 局部变量
    }

    // 处理语句
    for stmt in fd.body {
        compile_statement(prog, raw, &mut new_locals, stmt);
    }

    // Overwrite function lookup with total number of locals
    prog.syms.insert(
        fd.name.value, // 函数名称
        Symbol {
            location: function_index as i32, // 函数位置
            narguments,
            nlocals: new_locals.keys().len(),
        },
    );

    prog.syms.insert(
        done_label, // 完成label
        Symbol {
            location: prog.instructions.len() as i32, // 位置
            narguments: 0,
            nlocals: 0,
        },
    );
}

fn compile_return(
    prog: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    ret: Return,
) {
    compile_expression(prog, raw, locals, ret.expression);
    prog.instructions.push(OpCode::Return); // so easy
}

fn compile_if(prog: &mut Program, raw: &[char], locals: &mut HashMap<String, i32>, if_: If) {
    compile_expression(prog, raw, locals, if_.test); // 编译条件语句
    let done_label = format!("if_else_{}", prog.instructions.len()); // 生成 label
    prog.instructions
        .push(OpCode::JumpIfNotZero(done_label.clone())); // if 跳转需要一个label
    for stmt in if_.body {
        compile_statement(prog, raw, locals, stmt);
    }
    prog.syms.insert(
        done_label,
        Symbol {
            location: prog.instructions.len() as i32 - 1,
            nlocals: 0,
            narguments: 0,
        },
    );
}

fn compile_local(
    prog: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    local: Local,
) {
    let index = locals.keys().len();
    locals.insert(local.name.value, index as i32); // 记录序号
    compile_expression(prog, raw, locals, local.expression); // 生成变量的表达式
    prog.instructions.push(OpCode::MovePlusFP(index)); // 局部变量的序号
}

fn compile_statement(
    prog: &mut Program,
    raw: &[char],
    locals: &mut HashMap<String, i32>,
    stmt: Statement,
) {
    match stmt {
        Statement::FunctionDeclaration(fd) => compile_function_declaration(prog, raw, locals, fd),
        Statement::Return(r) => compile_return(prog, raw, locals, r),
        Statement::If(if_) => compile_if(prog, raw, locals, if_),
        Statement::Local(loc) => compile_local(prog, raw, locals, loc),
        Statement::Expression(e) => compile_expression(prog, raw, locals, e),
    }
}

// 编译 ast 树，生成字节码
pub fn compile(raw: &[char], ast: Ast) -> Program {
    // TODO 暂时只支持 i32
    let mut locals: HashMap<String, i32> = HashMap::new(); // 当前的局部变量
    let mut prog = Program::default();
    for stmt in ast {
        compile_statement(&mut prog, raw, &mut locals, stmt);
    }

    prog
}
