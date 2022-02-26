use std::fs;

use structopt::StructOpt;

use plua::{compile, eval, lex, parse};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "file")]
    file_path: String,

    #[structopt(short = "o", long = "optimize", help = "Optimize code")]
    optimize: bool,
}

fn main() {
    let opt = Opt::from_args();

    let contents = fs::read_to_string(opt.file_path).expect("could not read file");
    let raw: Vec<char> = contents.chars().collect();

    let tokens = match lex::lex(&raw) {
        Ok(tokens) => tokens,
        Err(msg) => panic!("{}", msg),
    };

    let ast = match parse::parse(&raw, tokens) {
        Ok(ast) => ast,
        Err(msg) => panic!("{}", msg),
    };

    let prog = compile::compile(&raw, ast);

    let ret = eval::eval(prog);

    std::process::exit(ret);
}
