// #[deny(warnings)]
mod export;
mod read;
mod setting;
mod table;
mod tablecell;
mod tablecellcore;
mod tableline;

extern crate clap;
use clap::Parser;
use setting::Args;

fn main() {
    let args = Args::parse();
    let table = read::read_from_io(args.seperation.as_str());
    println!("{:?}", args);
    println!("{:?}", table);
    println!("Hello, world!");
}
