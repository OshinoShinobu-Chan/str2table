// #[deny(warnings)]
mod export;
mod read;
mod setting;
mod table;
mod tablecell;
mod tableline;

extern crate clap;
use clap::Parser;
use setting::Args;

fn main() {
    let args = Args::parse();

    println!("Hello, world!");
}
