#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;
extern crate uuid;

mod cli;
mod docker;
mod types;

use std::process;

macro_rules! eprintln {
    ($($tt:tt)*) => {{
        use std::io::Write;
        let _ = writeln!(&mut ::std::io::stderr(), $($tt)*);
    }}
}

fn main() {
    match cli::parse_cli() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}
