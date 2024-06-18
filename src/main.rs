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
    let mut table = read::read_from_io(args.seperation.as_str(), args.end_line.as_str());
    println!("{:#?}", args);
    table.set_color_line(1, setting::OutputColor::Red);
    table.set_color_line(2, setting::OutputColor::Yellow);
    table.set_color_column(1, setting::OutputColor::Green);
    println!("{}", table);
    println!("{:e}", 123);
    println!("Hello, world!");
}
