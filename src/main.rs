mod chunk;
mod chunk_type;
mod cli;
mod png;

use clap::Parser;
use cli::{get_args, run};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(e) = cli::get_args().and_then(cli::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
