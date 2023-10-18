use std::collections::HashMap;
use std::slice;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};

use crate::expression::Expr;
use crate::scanner::Token;
use crate::statement::Stmt;
use crate::value::Value as ValueRaw;

// jit implement by cranelift inspired by RustPython
// see: https://github.com/RustPython/RustPython/tree/main/jit

pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }
}

impl JIT {
    /// Compile a string in the toy language into machine code.
    pub fn compile(&mut self, input: &Stmt) -> Result<*const u8, String> {
        if let Stmt::FunctionStmt(name, params, body) = input {
            // TODO remove the return
            self.translate(params, "the_return".to_string(), body)?;

            let id = self
                .module
                .declare_function(name.raw.as_str(), Linkage::Export, &self.ctx.func.signature)
                .map_err(|e| e.to_string())?;
            self.module
                .define_function(id, &mut self.ctx)
                .map_err(|e| e.to_string())?;
            self.module.clear_context(&mut self.ctx);
            self.module.finalize_definitions();

            let code = self.module.get_finalized_function(id);

            Ok(code)
        } else {
            return Err(format!("stmt not support!"));
        }
    }

    pub fn create_data(&mut self, name: &str, contents: Vec<u8>) -> Result<&[u8], String> {
        self.data_ctx.define(contents.into_boxed_slice());
        let id = self
            .module
            .declare_data(name, Linkage::Export, true, false)
            .map_err(|e| e.to_string())?;

        self.module
            .define_data(id, &self.data_ctx)
            .map_err(|e| e.to_string())?;
        self.data_ctx.clear();
        self.module.finalize_definitions();
        let buffer = self.module.get_finalized_data(id);

        Ok(unsafe { slice::from_raw_parts(buffer.0, buffer.1) })
    }

    fn translate(
        &mut self,
        params: &Vec<Token>,
        the_return: String,
        stmts: &Vec<Stmt>,
    ) -> Result<(), String> {
        // 只支持一种类型的参数和一个返回值
        let int = self.module.target_config().pointer_type();

        let mut names = Vec::new();
        for p in params {
            names.push(p.raw.clone());
            self.ctx.func.signature.params.push(AbiParam::new(int));
        }

        self.ctx.func.signature.returns.push(AbiParam::new(int));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let variables =
            declare_variables(int, &mut builder, &names, &the_return, stmts, entry_block);
        // Now translate the statements of the function body.
        let mut trans = FunctionTranslator {
            int,
            builder,
            variables,
            module: &mut self.module,
        };
        for stmt in stmts {
            trans.translate_stmt(stmt)?;
        }
        // return 已经完成
        trans.builder.finalize();
        Ok(())
    }
}

/// A collection of state used for translating from toy-language AST nodes
/// into Cranelift IR.
struct FunctionTranslator<'a> {
    int: types::Type,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
}

impl<'a> FunctionTranslator<'a> {
    fn translate_stmt(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Expression(expr) => match expr {
                Expr::Literal(literal) => {
                    return if let ValueRaw::Int(imm) = literal {
                        Ok(self.builder.ins().iconst(self.int, i64::from(*imm)))
                    } else {
                        Err("value type not support".to_string())
                    };
                }

                Expr::Binary(left, op, right) => match op.raw.as_str() {
                    "+" => {
                        let lhs = self.translate_expr(left.as_ref())?;
                        let rhs = self.translate_expr(right.as_ref())?;
                        return Ok(self.builder.ins().iadd(lhs, rhs));
                    }
                    "-" => {
                        let lhs = self.translate_expr(left.as_ref())?;
                        let rhs = self.translate_expr(right.as_ref())?;
                        return Ok(self.builder.ins().isub(lhs, rhs));
                    }
                    "*" => {
                        let lhs = self.translate_expr(left.as_ref())?;
                        let rhs = self.translate_expr(right.as_ref())?;
                        return Ok(self.builder.ins().imul(lhs, rhs));
                    }
                    "/" => {
                        let lhs = self.translate_expr(left.as_ref())?;
                        let rhs = self.translate_expr(right.as_ref())?;
                        return Ok(self.builder.ins().udiv(lhs, rhs));
                    }
                    _ => {}
                },
                Expr::Assign(name, expr) => {
                    return self.translate_assign(name.raw.clone(), expr.as_ref())
                }
                _ => {}
            },
            Stmt::ReturnStmt(_token, expr) => {
                return if let Expr::Variable(ident) = expr {
                    let return_variable = self.variables.get(ident.raw.as_str()).unwrap();
                    let return_value = self.builder.use_var(*return_variable);
                    self.builder.ins().return_(&[return_value]);
                    Ok(Value::new(0))
                } else {
                    Err(format!("return type not support."))
                };
            }
            _ => {}
        }
        Err(format!("un support expr."))
    }

    fn translate_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(literal) => {
                return if let ValueRaw::Int(imm) = literal {
                    Ok(self.builder.ins().iconst(self.int, i64::from(*imm)))
                } else {
                    Err("value type not support".to_string())
                };
            }
            Expr::Binary(left, op, right) => match op.raw.as_str() {
                "+" => {
                    let lhs = self.translate_expr(left.as_ref())?;
                    let rhs = self.translate_expr(right.as_ref())?;
                    return Ok(self.builder.ins().iadd(lhs, rhs));
                }
                "-" => {
                    let lhs = self.translate_expr(left.as_ref())?;
                    let rhs = self.translate_expr(right.as_ref())?;
                    return Ok(self.builder.ins().isub(lhs, rhs));
                }
                "*" => {
                    let lhs = self.translate_expr(left.as_ref())?;
                    let rhs = self.translate_expr(right.as_ref())?;
                    return Ok(self.builder.ins().imul(lhs, rhs));
                }
                "/" => {
                    let lhs = self.translate_expr(left.as_ref())?;
                    let rhs = self.translate_expr(right.as_ref())?;
                    return Ok(self.builder.ins().udiv(lhs, rhs));
                }
                _ => Err("op not support".to_string()),
            },
            Expr::Assign(name, expr) => self.translate_assign(name.raw.clone(), expr.as_ref()),
            _ => Err("un support expr".to_string()),
        }
    }

    fn translate_assign(&mut self, name: String, expr: &Expr) -> Result<Value, String> {
        let new_value = self.translate_expr(expr)?;
        let variable = self.variables.get(&name).unwrap();
        self.builder.def_var(*variable, new_value);
        Ok(new_value)
    }

    fn translate_icmp(&mut self, cmp: IntCC, lhs: &Expr, rhs: &Expr) -> Result<Value, String> {
        let lhs = self.translate_expr(lhs)?;
        let rhs = self.translate_expr(rhs)?;
        let c = self.builder.ins().icmp(cmp, lhs, rhs);
        Ok(self.builder.ins().bint(self.int, c))
    }
}

fn declare_variables(
    int: types::Type,
    builder: &mut FunctionBuilder,
    params: &[String],
    the_return: &str,
    stmts: &Vec<Stmt>,
    entry_block: Block,
) -> HashMap<String, Variable> {
    let mut variables = HashMap::new();
    let mut index = 0;

    for (i, name) in params.iter().enumerate() {
        let val = builder.block_params(entry_block)[i];
        let var = declare_variable(int, builder, &mut variables, &mut index, name);
        builder.def_var(var, val);
    }
    let zero = builder.ins().iconst(int, 0);
    let return_variable = declare_variable(int, builder, &mut variables, &mut index, the_return);
    builder.def_var(return_variable, zero);
    for stmt in stmts {
        declare_variables_in_stmt(int, builder, &mut variables, &mut index, stmt);
    }

    variables
}

/// Recursively descend through the AST, translating all implicit
/// variable declarations.
fn declare_variables_in_stmt(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    stmt: &Stmt,
) {
    match stmt {
        Stmt::Expression(expr) => match expr {
            Expr::Assign(ref name, _) => {
                declare_variable(int, builder, variables, index, name.raw.as_str());
            }
            _ => {}
        },
        _ => (),
    }
}

fn declare_variable(
    int: types::Type,
    builder: &mut FunctionBuilder,
    variables: &mut HashMap<String, Variable>,
    index: &mut usize,
    name: &str,
) -> Variable {
    let var = Variable::new(*index);
    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, int);
        *index += 1;
    }
    var
}

#[cfg(test)]
mod tests {
    use super::JIT;
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use std::mem;

    #[test]
    fn test_jit() {
        let source = r#"
        function fib()
          n = 1 + 2 * 3;
          return n;
        end
        "#;
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();
        println!("{:#?}", tokens.as_ref().unwrap());
        assert_eq!(tokens.as_ref().unwrap().len(), 17);

        let mut parser = Parser::new(tokens.unwrap().clone());
        let result = parser.parse();
        assert_eq!(result.is_err(), false);
        println!("{:#?}", result.as_ref().unwrap());
        assert_eq!(result.as_ref().unwrap().len(), 1);

        let mut jit = JIT::default();
        let r = jit.compile(result.unwrap().get(0).unwrap());
        assert_eq!(r.is_err(), false);

        let code_ptr = r.unwrap();

        unsafe {
            let code_fn = mem::transmute::<_, fn() -> i32>(code_ptr);
            // And now we can call it!
            let i = code_fn();
            assert_eq!(i, 7);
        }
    }
}
