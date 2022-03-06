# plua

> Simple lua interceptor implement by Rust
> 通过 Rust 实现简单的 lua 解释器

## features

- [X] interceptor
- [X] lex
- [X] parse
- [X] resolver
- [ ] interceptor
- [X] bytecode
- [X] vm(bug fix in func call todo)
- [X] jit(improve needed)
- [ ] tail recursion(尾递归)

## sytax

```
program        → declaration* EOF ;

declaration    → funDecl
               | localDecl
               | statement ;

funDecl        → "function" functionBody "end" ;
localDecl      → "local" IDENTIFIER ( "=" expression )? ";" ;

statement      → exprStmt
               | ifStmt
               | printStmt
               | returnStmt ;

exprStmt       → expression ";" ;

ifStmt         → "if" expression "then" block
                 ( "else" block )? "end" ;
                 
printStmt      → "print" expression ";" ;
returnStmt     → "return" expression? ";" ;
block          →  declaration*  ;

expression     → assignment ;
assignment     → IDENTIFIER "=" assignment | logic_or ;

logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;

unary          → ( "!" | "-" ) unary | call ;
call           → primary "(" arguments? ")" ;
primary        → "true" | "false" | "nil"
               | NUMBER | STRING | IDENTIFIER | "(" expression ")";

functionBody   → IDENTIFIER "(" parameters? ")"  block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
arguments      → expression ( "," expression )* ;

NUMBER         → DIGIT+ ( "." DIGIT+ )? ;
STRING         → "\"" <any char except "\"">* "\"" ;
IDENTIFIER     → ALPHA ( ALPHA | DIGIT )* ;
ALPHA          → "a" ... "z" | "A" ... "Z" | "_" ;
DIGIT          → "0" ... "9" ;
```

## reference

- [lust: Lua in Rust](https://github.com/eatonphil/lust)
- [Writing a minimal Lua implementation with a virtual machine from scratch in Rust](https://notes.eatonphil.com/lua-in-rust.html)
- [Brainfuck JIT 虚拟机教程](https://github.com/Nugine/bfjit)
- [cranelift-jit-demo](https://github.com/bytecodealliance/cranelift-jit-demo)
- [jit-minimal](https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/jit/examples/jit-minimal.rs)
- [cranelift-jit-demo](https://github.com/bytecodealliance/cranelift-jit-demo)
- [crafting interpreters](https://craftinginterpreters.com/contents.html)
- [So You Want to Build a Language VM](https://blog.subnetzero.io/post/building-language-vm-part-01/)
- [Building a stack-based virtual machine](https://dev.to/jimsy/building-a-stack-based-virtual-machine-5gkd)
- [Writing Interpreters in Rust: a Guide](https://rust-hosted-langs.github.io/book/introduction.html)
- [tinyvm](https://github.com/mkhan45/tinyvm)
- [RustPython](https://github.com/RustPython/RustPython)
- [语法格式描述规范BNF、EBNF、ABNF](https://www.jianshu.com/p/15efcb0c06c8)
- [The Complete Syntax of Lua](http://parrot.github.io/parrot-docs0/0.4.7/html/languages/lua/doc/lua51.bnf.html)
