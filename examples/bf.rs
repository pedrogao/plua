use std::io::{stdin, stdout};
use std::path::PathBuf;

use structopt::StructOpt;

extern crate plua;
use plua::bf::vm::BfVM;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "FILE")]
    file_path: PathBuf,

    #[structopt(short = "o", long = "optimize", help = "Optimize code")]
    optimize: bool,
}

fn main() {
    let opt = Opt::from_args();

    let stdin = stdin();
    let stdout = stdout();

    let ret = BfVM::new(
        &opt.file_path,
        Box::new(stdin.lock()),
        Box::new(stdout.lock()),
        opt.optimize,
    )
    .and_then(|mut vm| vm.run());

    if let Err(e) = &ret {
        eprintln!("bf: {}", e);
    }

    std::process::exit(ret.is_err() as i32)
}
