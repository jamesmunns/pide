#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate serde;
extern crate toml;
extern crate uuid;

mod cli;

fn main() {
    cli::parse_cli();
}
