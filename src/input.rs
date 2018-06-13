// use quicli::prelude::*;

// use quicli::prelude::structopt::StructOpt;
// use super::*;
// use quicli::prelude::structopt::*;
// use structopt::*;

use std::path::{PathBuf};

#[derive(StructOpt, Debug)]
struct IOArg {
    #[structopt(help = "input file of hex-string/binary data (or reading from stdin if not given)")]
    input_file: Option<String>,
    // #[structopt(short = "o", long = "output", help = "output (default: print to terminal)", parse(from_os_str))]
    // output: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
struct AsmArg {
    #[structopt(long = "att", help = "use AT&T assembly instead of default Intel syntax)")]
    att_syntax: bool,

    #[structopt(short = "b", long = "base", help = "base address (default: 0x0)")]
    base_address: Option<usize>,
}

#[derive(StructOpt, Debug)]
struct SuppOpt {
    #[structopt(short = "v", long = "verbose", help = "show instruction's details", parse(from_occurrences))]
    verbose: u8,

    #[structopt(short = "a", long = "address", help = "show instruction's address")]
    show_address: bool,

    #[structopt(short = "b", long = "binary", help = "show instruction's binary data")]
    show_binary: bool
}

#[derive(StructOpt, Debug)]
#[structopt(name = "capstone-based x86/amd64 disassembler")]
struct RdisOpt {
    #[structopt(flatten)]
    io: IOArg,

    #[structopt(flatten)]
    syntax: AsmArg,

    #[structopt(flatten)]
    supp: SuppOpt,
}

pub(super) struct Config<'a> {
    pub asm_file: Option<&'a str>,  // read from stdin if None
    pub att_syntax: bool,           // false: intel, true: att
    pub base_address: usize,
    pub show_address: bool,         // true: show instruction's address
    pub show_binary: bool,          // true: show instruction's binary data
    pub show_rw_registers: bool,    // true: show instruction's read/write registers
}