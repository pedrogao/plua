use crate::lex::*;

#[derive(Debug)]
pub enum Literal {
    Identifier(Token),
    Number(Token),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: Token,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub operator: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    // 字面量
    Literal(Literal),
    // 函数调用
    FunctionCall(FunctionCall),
    // 二元表达式
    BinaryOperation(BinaryOperation),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    // 函数名称
    pub name: Token,
    // 函数参数
    pub parameters: Vec<Token>,
    // 函数执行体
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct If {
    // 条件语句
    pub test: Expression,
    // 执行体
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Local {
    pub name: Token,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct Return {
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Statement {
    // 表达式
    Expression(Expression),
    // if语句
    If(If),
    // 函数声明
    FunctionDeclaration(FunctionDeclaration),
    // 返回
    Return(Return),
    // 局部变量
    Local(Local),
}

// AST 抽象语法树，简单定义
pub type Ast = Vec<Statement>;

// 判断是否为关键字
fn expect_keyword(tokens: &[Token], index: usize, value: &str) -> bool {
    if index >= tokens.len() {
        return false;
    }

    let t = tokens[index].clone();
    t.kind == TokenKind::Keyword && t.value == value
}

// 判断是否为语法符
fn expect_syntax(tokens: &[Token], index: usize, value: &str) -> bool {
    if index >= tokens.len() {
        return false;
    }

    let t = tokens[index].clone();
    t.kind == TokenKind::Syntax && t.value == value
}

// 判断是否为标识符
fn expect_identifier(tokens: &[Token], index: usize) -> bool {
    if index >= tokens.len() {
        return false;
    }

    let t = tokens[index].clone();
    t.kind == TokenKind::Identifier
}

// 解析表达式
fn parse_expression(raw: &[char], tokens: &[Token], index: usize) -> Option<(Expression, usize)> {
    if index >= tokens.len() {
        return None;
    }

    let t = tokens[index].clone();
    // 数字、标识符都是 literal 表达式，简单表达式
    let left = match t.kind {
        TokenKind::Number => Expression::Literal(Literal::Number(t)),
        TokenKind::Identifier => Expression::Literal(Literal::Identifier(t)),
        _ => {
            return None;
        }
    };
    let mut next_index = index + 1;
    if expect_syntax(tokens, next_index, "(") {
        next_index += 1; // Skip past open paren

        // Function call
        let mut arguments: Vec<Expression> = vec![];
        while !expect_syntax(tokens, next_index, ")") {
            if !arguments.is_empty() {
                if !expect_syntax(tokens, next_index, ",") {
                    println!(
                        "{}",
                        tokens[next_index]
                            .loc
                            .debug(raw, "Expected comma between function call arguments:")
                    );
                    return None;
                }

                next_index += 1; // Skip past comma
            }

            let res = parse_expression(raw, tokens, next_index);
            if let Some((arg, next_next_index)) = res {
                next_index = next_next_index;
                arguments.push(arg);
            } else {
                println!(
                    "{}",
                    tokens[next_index]
                        .loc
                        .debug(raw, "Expected valid expression in function call arguments:")
                );
                return None;
            }
        }

        next_index += 1; // Skip past closing paren

        return Some((
            Expression::FunctionCall(FunctionCall {
                name: tokens[index].clone(),
                arguments,
            }),
            next_index,
        ));
    }

    // Might be a literal expression
    if next_index >= tokens.len() || tokens[next_index].clone().kind != TokenKind::Operator {
        return Some((left, next_index)); // 一元表达式
    }

    // Otherwise is a binary operation
    let op = tokens[next_index].clone();
    next_index += 1; // Skip past op

    if next_index >= tokens.len() {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid right hand side binary operand:")
        );
        return None;
    }

    let rtoken = tokens[next_index].clone();
    let right = match rtoken.kind {
        TokenKind::Number => Expression::Literal(Literal::Number(rtoken)),
        TokenKind::Identifier => Expression::Literal(Literal::Identifier(rtoken)),
        _ => {
            println!(
                "{}",
                rtoken
                    .loc
                    .debug(raw, "Expected valid right hand side binary operand:")
            );
            return None;
        }
    };
    next_index += 1; // Skip past right hand operand

    Some((
        Expression::BinaryOperation(BinaryOperation {
            left: Box::new(left),
            right: Box::new(right),
            operator: op,
        }),
        next_index,
    ))
}

fn parse_function_statement(raw: &[char], tokens: &[Token], index: usize) -> Option<(Statement, usize)> {
    if !expect_keyword(tokens, index, "function") { // function关键字
        return None;
    }

    let mut next_index = index + 1;
    if !expect_identifier(tokens, next_index) {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid identifier for function name:")
        );
        return None;
    }
    let name = tokens[next_index].clone();

    next_index += 1; // Skip past name
    if !expect_syntax(tokens, next_index, "(") {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected open parenthesis in function declaration:")
        );
        return None;
    }

    next_index += 1; // Skip past open paren
    let mut parameters: Vec<Token> = vec![];
    while !expect_syntax(tokens, next_index, ")") {
        if !parameters.is_empty() {
            if !expect_syntax(tokens, next_index, ",") {
                println!("{}",
                         tokens[next_index].
                             loc.
                             debug(raw, "Expected comma or close parenthesis after parameter in function declaration:"));
                return None;
            }

            next_index += 1; // Skip past comma
        }

        parameters.push(tokens[next_index].clone());
        next_index += 1; // Skip past param
    }

    next_index += 1; // Skip past close paren

    let mut statements: Vec<Statement> = vec![];
    while !expect_keyword(tokens, next_index, "end") {
        let res = parse_statement(raw, tokens, next_index);
        if let Some((stmt, next_next_index)) = res {
            next_index = next_next_index;
            statements.push(stmt);
        } else {
            println!(
                "{}",
                tokens[next_index]
                    .loc
                    .debug(raw, "Expected valid statement in function declaration:")
            );
            return None;
        }
    }

    next_index += 1; // Skip past end

    Some((
        Statement::FunctionDeclaration(FunctionDeclaration {
            name,
            parameters,
            body: statements,
        }),
        next_index,
    ))
}

fn parse_return_statement(raw: &[char], tokens: &[Token], index: usize) -> Option<(Statement, usize)> {
    if !expect_keyword(tokens, index, "return") {
        return None;
    }

    let mut next_index = index + 1; // Skip past return
    let res = parse_expression(raw, tokens, next_index);
    if res.is_none() {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid expression in return statement:")
        );
        return None;
    }

    let (expr, next_next_index) = res.unwrap();
    next_index = next_next_index;
    if !expect_syntax(tokens, next_index, ";") {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected semicolon in return statement:")
        );
        return None;
    }

    next_index += 1; // Skip past semicolon

    Some((Statement::Return(Return { expression: expr }), next_index))
}

fn parse_local_statement(raw: &[char], tokens: &[Token], index: usize) -> Option<(Statement, usize)> {
    if !expect_keyword(tokens, index, "local") { // 关键字
        return None;
    }

    let mut next_index = index + 1; // Skip past local

    if !expect_identifier(tokens, next_index) {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid identifier for local name:")
        );
        return None;
    }

    let name = tokens[next_index].clone();
    next_index += 1; // Skip past name

    if !expect_syntax(tokens, next_index, "=") {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected = syntax after local name:")
        );
        return None;
    }

    next_index += 1; // Skip past =

    let res = parse_expression(raw, tokens, next_index);
    if res.is_none() {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid expression in local declaration:")
        );
        return None;
    }

    let (expr, next_next_index) = res.unwrap();
    next_index = next_next_index;

    if !expect_syntax(tokens, next_index, ";") {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected semicolon in return statement:")
        );
        return None;
    }

    next_index += 1; // Skip past semicolon

    Some((
        Statement::Local(Local {
            name,
            expression: expr,
        }),
        next_index,
    ))
}

fn parse_if_statement(raw: &[char], tokens: &[Token], index: usize) -> Option<(Statement, usize)> {
    if !expect_keyword(tokens, index, "if") { // 判断关键字
        return None;
    }

    let mut next_index = index + 1; // Skip past if
    let res = parse_expression(raw, tokens, next_index); // 解析 if 中的判断条件
    if res.is_none() {
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected valid expression for if test:")
        );
        return None;
    }

    let (test, next_next_index) = res.unwrap();
    next_index = next_next_index;

    if !expect_keyword(tokens, next_index, "then") { // then 关键字
        return None;
    }

    next_index += 1; // Skip past then

    let mut statements: Vec<Statement> = vec![]; // if 中的执行语句
    while !expect_keyword(tokens, next_index, "end") { // 直到遇到了end
        let res = parse_statement(raw, tokens, next_index);
        if let Some((stmt, next_next_index)) = res {
            next_index = next_next_index;
            statements.push(stmt);
        } else {
            println!(
                "{}",
                tokens[next_index]
                    .loc
                    .debug(raw, "Expected valid statement in if body:")
            );
            return None;
        }
    }

    next_index += 1; // Skip past end

    Some((
        Statement::If(If {
            test,
            body: statements,
        }),
        next_index,
    ))
}

fn parse_expression_statement(
    raw: &[char],
    tokens: &[Token],
    index: usize,
) -> Option<(Statement, usize)> {
    let mut next_index = index;
    let res = parse_expression(raw, tokens, next_index)?; // 解析表达式

    let (expr, next_next_index) = res;
    next_index = next_next_index;
    if !expect_syntax(tokens, next_index, ";") { // 语句必须以;结尾
        println!(
            "{}",
            tokens[next_index]
                .loc
                .debug(raw, "Expected semicolon after expression:")
        );
        return None;
    }

    next_index += 1; // Skip past semicolon

    Some((Statement::Expression(expr), next_index))
}

// 解析语句
fn parse_statement(raw: &[char], tokens: &[Token], index: usize) -> Option<(Statement, usize)> {
    let parsers = [
        parse_if_statement,                     // if语句
        parse_expression_statement,   // 解析语句中的表达式(多了一个;)，可以简单理解为 statement = expression;
        parse_return_statement,                 // return语句
        parse_function_statement,               // 函数语句
        parse_local_statement,                  // 变量声明
    ];
    for parser in parsers {
        let res = parser(raw, tokens, index);
        if res.is_some() {
            return res;
        }
    }

    None
}

// 解析得到AST树
pub fn parse(raw: &[char], tokens: Vec<Token>) -> Result<Ast, String> {
    let mut ast = vec![];
    let mut index = 0;
    let len = tokens.len();
    while index < len {
        let res = parse_statement(raw, &tokens, index);
        if let Some((stmt, next_index)) = res {
            index = next_index;   // 更新index
            ast.push(stmt); // push statement
            continue;             // 下一个
        }

        return Err(tokens[index].loc.debug(raw, "Invalid token while parsing:"));
    }

    Ok(ast)
}
