// #[macro_use] extern crate quicli;
#[macro_use] extern crate failure;
#[macro_use] extern crate structopt;

mod error;
mod input;
mod output;

use error::{Error, ErrorKind, Result};

// main!(|args: input::RdisOpt, log_level: verbosity| {
//     println!("{}", "Hello");
// });

fn main() -> Result<()> {
    Ok(())
}
