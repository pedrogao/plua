#[derive(Copy, Clone, Debug, Default)]
pub struct Location {
    col: i32,
    line: i32,
    index: usize,
}

impl Location {
    fn increment_one(&self, newline: bool) -> Location {
        self.increment(1, newline)
    }

    fn increment(&self, i: i32, newline: bool) -> Location {
        if newline {
            Location {
                index: self.index + i as usize,
                col: 0,
                line: self.line + 1,
            }
        } else {
            Location {
                index: self.index + i as usize,
                col: self.col + i,
                line: self.line,
            }
        }
    }

    pub fn debug<S: Into<String>>(&self, raw: &[char], msg: S) -> String {
        let mut line = 0;
        let mut line_str = String::new();
        // Find the whole line of original source
        for c in raw {
            if *c == '\n' {
                line += 1;

                // Done discovering line in question
                if !line_str.is_empty() {
                    break;
                }

                continue;
            }

            if self.line == line {
                line_str.push_str(&c.to_string());
            }
        }

        let space = " ".repeat(self.col as usize);
        format!("{}\n\n{}\n{}^ Near here", msg.into(), line_str, space)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Identifier,
    // 标识符
    Syntax,
    // 语法
    Keyword,
    // 关键字
    Number,
    // 数字
    Operator,   // 操作符
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    // 值
    pub kind: TokenKind,
    // 类型
    pub loc: Location,   // 位置
}

fn lex_operator(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
    // TODO: 目前只支持 + - < 三种运算符，如果支持二元运算符，需要peek
    let operators = ["+", "-", "<"];

    for possible_syntax in operators {
        let c = raw[initial_loc.index];
        let next_loc = initial_loc.increment_one(false);
        if possible_syntax == c.to_string() {
            return Some((
                Token {
                    value: possible_syntax.to_string(),
                    loc: initial_loc,
                    kind: TokenKind::Operator,
                },
                next_loc,
            ));
        }
    }

    None
}

fn lex_syntax(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
    // TODO: 目前只支持单字符
    let syntax = [";", "=", "(", ")", ","];

    for possible_syntax in syntax {
        let c = raw[initial_loc.index];
        let next_loc = initial_loc.increment_one(false);
        if possible_syntax == c.to_string() {
            return Some((
                Token {
                    value: possible_syntax.to_string(),
                    loc: initial_loc,
                    kind: TokenKind::Syntax,
                },
                next_loc,
            ));
        }
    }

    None
}

fn lex_keyword(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
    let syntax = ["function", "end", "if", "then", "local", "return"];

    let mut next_loc = initial_loc;
    let mut value = String::new();

    'outer: for possible_syntax in syntax {
        let len = possible_syntax.len();
        let op = raw.get(initial_loc.index..initial_loc.index + len);
        if op.is_none() {
            // 不满足，就下一个
            continue 'outer;
        }
        let sub: String = op.unwrap().into_iter().collect();
        if sub.as_str() != possible_syntax {
            continue 'outer;
        }
        next_loc = next_loc.increment(len as i32, false); // 不支持多行字符串
        value = sub;
        // 证明找到了，直接 break
        break;
    }

    if value.is_empty() {
        return None;
    }

    // 匹配到关键后，发现后面还有，比如 function1，那么就只是一个字符串
    let next_op = raw.get(next_loc.index);
    if next_op.is_some() {
        let next_c = next_op.unwrap();
        if next_c.is_alphanumeric() || *next_c == '_' {
            return None;
        }
    }

    Some((
        Token {
            value,
            loc: initial_loc,
            kind: TokenKind::Keyword,
        },
        next_loc,
    ))
}

fn lex_identifier(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
    let mut ident = String::new();
    let mut next_loc = initial_loc;
    let mut c = raw[initial_loc.index];
    while c.is_alphanumeric() || c == '_' { // 字母或者_
        ident.push_str(&c.to_string());
        next_loc = next_loc.increment_one(false);
        c = raw[next_loc.index];
    }

    // 标识符不能以数字开头
    if !ident.is_empty() && !ident.chars().next().unwrap().is_digit(10) {
        Some((
            Token {
                value: ident,
                loc: initial_loc,
                kind: TokenKind::Identifier,
            },
            next_loc,
        ))
    } else {
        None
    }
}

fn lex_number(raw: &[char], initial_loc: Location) -> Option<(Token, Location)> {
    // TODO 暂时不支持小数点和科学计数法
    let mut ident = String::new();
    let mut next_loc = initial_loc;
    let mut c = raw[initial_loc.index];
    while c.is_digit(10) {
        ident.push_str(&c.to_string());
        next_loc = next_loc.increment_one(false);
        c = raw[next_loc.index];
    }

    if !ident.is_empty() {
        Some((
            Token {
                value: ident,
                loc: initial_loc,
                kind: TokenKind::Number,
            },
            next_loc,
        ))
    } else {
        None
    }
}

fn skip_whitespaces(raw: &[char], initial_loc: Location) -> Location {
    let mut c = raw[initial_loc.index];
    let mut next_loc = initial_loc;
    while [' ', '\n', '\r', '\t'].contains(&c) {
        next_loc = next_loc.increment_one(c == '\n');
        if next_loc.index == raw.len() {
            break;
        }
        c = raw[next_loc.index];
    }

    next_loc
}

pub fn lex(raw: &[char]) -> Result<Vec<Token>, String> {
    // 初始位置
    let mut loc = Location::default();
    let size = raw.len(); // 源代码字符长度
    let mut tokens: Vec<Token> = vec![]; // tokens

    let lexers = [
        lex_keyword,
        lex_identifier,
        lex_number,
        lex_syntax,
        lex_operator,
    ];

    'outer: while loc.index < size {
        loc = skip_whitespaces(raw, loc); // 跳过空格
        if loc.index == size {
            // eof
            break;
        }

        for lexer in lexers { // TODO First-second优化，避免每次迭代所有lex函数
            let res = lexer(raw, loc);
            if let Some((t, next_loc)) = res {
                loc = next_loc; // 更新 location
                tokens.push(t);
                continue 'outer; // 继续
            }
        }

        return Err(loc.debug(raw, "Unrecognized character while lexing:"));
    }

    Ok(tokens)
}

mod tests {
    use super::{lex_keyword, Location};

    #[test]
    fn test_lex_keyword() {
        let raw: Vec<char> = "function".chars().collect();
        let loc = Location::default();
        let op = lex_keyword(&raw, loc);
        assert!(op.is_some());
        let token = op.unwrap();
        assert_eq!(token.0.value, "function");

        let raw: Vec<char> = "func".chars().collect();
        let loc = Location::default();
        let op = lex_keyword(&raw, loc);
        assert!(op.is_none());

        let raw: Vec<char> = "functiona".chars().collect();
        let loc = Location::default();
        let op = lex_keyword(&raw, loc);
        assert!(op.is_none());

        let raw: Vec<char> = "return ".chars().collect();
        let loc = Location::default();
        let op = lex_keyword(&raw, loc);
        assert!(op.is_some());
        let token = op.unwrap();
        assert_eq!(token.0.value, "return");
        assert_eq!(token.1.line, 0);
        assert_eq!(token.1.col, 6);
        assert_eq!(token.1.index, 6);
    }
}
