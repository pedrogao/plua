use std::result::Result;
use std::vec;

use crate::bf::error::CompileError;
use crate::bf::error::CompileErrorKind;
use crate::bf::ir::BfIR;

// 将bf代码编译为ir(opcode)
pub fn compile(src: &str) -> Result<Vec<BfIR>, CompileError> {
    let mut code: Vec<BfIR> = vec![];
    let mut stk: Vec<(u32, u32, u32)> = vec![];

    let mut line: u32 = 1;
    let mut col: u32 = 0;

    for ch in src.chars() {
        col += 1;
        match ch {
            '\n' => {
                line += 1;
                col = 0;
            }
            '+' => code.push(BfIR::AddVal(1)),
            '-' => code.push(BfIR::SubVal(1)),
            '>' => code.push(BfIR::AddPtr(1)),
            '<' => code.push(BfIR::SubPtr(1)),
            ',' => code.push(BfIR::GetByte),
            '.' => code.push(BfIR::PutByte),
            '[' => {
                let pos = code.len() as u32; // 当前字节长度
                stk.push((pos, line, col));
                code.push(BfIR::Jz);
            }
            ']' => {
                stk.pop().ok_or(CompileError {
                    line,
                    col,
                    kind: CompileErrorKind::UnexcpectedRightBracket,
                })?;
                code.push(BfIR::Jnz);
            }
            _ => {} // 其它字符，忽略
        }
    }

    // 循环结束后，如果栈不为空，说明有左括号没有匹配到右括号，弹出左括号位置，生成编译错误
    if let Some((_, line, col)) = stk.pop() {
        return Err(CompileError {
            line,
            col,
            kind: CompileErrorKind::UnclosedLeftBracket,
        });
    }

    return Ok(code);
}

pub fn optimize(code: &mut Vec<BfIR>) {
    let mut i = 0;
    let mut pc = 0;
    let len = code.len();

    macro_rules! _fold_ir {
        ($variant:ident, $x:ident) => {{
            let mut j = i + 1;
            while j < len {
                if let $variant(d) = code[j] {
                    $x = $x.wrapping_add(d);
                } else {
                    break;
                }
                j += 1;
            }
            i = j;
            code[pc] = $variant($x);
            pc += 1;
        }};
    }

    macro_rules! _normal_ir {
        () => {{
            code[pc] = code[i];
            pc += 1;
            i += 1;
        }};
    }

    use BfIR::*;
    while i < len {
        match code[i] {
            AddPtr(mut x) => _fold_ir!(AddPtr, x),
            SubPtr(mut x) => _fold_ir!(SubPtr, x),
            AddVal(mut x) => _fold_ir!(AddVal, x),
            SubVal(mut x) => _fold_ir!(SubVal, x),
            GetByte => _normal_ir!(),
            PutByte => _normal_ir!(),
            Jz => _normal_ir!(),
            Jnz => _normal_ir!(),
        }
    }
    code.truncate(pc);
    code.shrink_to_fit();
}

mod tests {
    use super::*;
    #[test]
    fn test_compile() {
        assert_eq!(
            compile("+[,.]").unwrap(),
            vec![
                BfIR::AddVal(1),
                BfIR::Jz,
                BfIR::GetByte,
                BfIR::PutByte,
                BfIR::Jnz,
            ]
        );

        match compile("[").unwrap_err().kind {
            CompileErrorKind::UnclosedLeftBracket => {}
            _ => panic!(),
        };

        match compile("]").unwrap_err().kind {
            CompileErrorKind::UnexcpectedRightBracket => {}
            _ => panic!(),
        };

        let mut code = compile("[+++++]").unwrap();
        optimize(&mut code);
        assert_eq!(code, vec![BfIR::Jz, BfIR::AddVal(5), BfIR::Jnz]);
    }
}
