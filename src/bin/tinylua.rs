#![allow(dead_code)]

use std::{fs, path::PathBuf};

use plua::{intercepter::Intercepter, parser::Parser, scanner::Scanner};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tinylua", about = "A tiny language compiler <😆>.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(default_value = "stdout", short)]
    out_type: String,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let debug = opt.debug;
    let script = fs::read_to_string(opt.input).expect("could not read file");

    let mut scanner = Scanner::new(script);
    let tokens = scanner.scan_tokens().unwrap();
    if debug {
        println!("{:?}", tokens);
    }

    let mut parser = Parser::new(tokens.clone());
    let statements = parser.parse().unwrap();
    if debug {
        println!("{:?}", statements);
    }

    let mut intercepter = Intercepter::new();
    let result = intercepter.eval(&statements);

    let ret = match result {
        Ok(v) => {
            println!("{:?}", v);
            0
        }
        Err(e) => {
            eprintln!("{:?}", e);
            1
        }
    };
    std::process::exit(ret);
}
