#[macro_use] extern crate quicli;

use quicli::prelude::*;
use std::path::{PathBuf};

#[derive(StructOpt, Debug)]
struct IOArg {
    #[structopt(short = "i", long = "input", help = "input binary code (or reading it from file")]
    input: String,

    #[structopt(short = "o", long = "output", help = "output (default: print to terminal)", parse(from_os_str))]
    output: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
struct AsmSyntax {
    #[structopt(long = "att", help = "use AT&T assembly instead of default Intel syntax)")]
    input: String,
}

#[derive(StructOpt, Debug)]
struct SuppOpt {
    #[structopt(short = "b", long = "base", help = "base address (default: 0x0)")]
    base: Option<usize>,
}

#[derive(StructOpt, Debug)]
struct RegOpt {
}

#[derive(StructOpt, Debug)]
#[structopt(name = "capstone-based disassembler")]
struct RdisOpt {
    #[structopt(flatten)]
    io: IOArg,

    #[structopt(flatten)]
    syntax: AsmSyntax,

    #[structopt(flatten)]
    verbosity: Verbosity,

    #[structopt(flatten)]
    supp: SuppOpt,
}

main!(|args: RdisOpt, log_level: verbosity| {
    println!("{}", "Hello");
});
