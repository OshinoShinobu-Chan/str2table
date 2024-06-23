// #[deny(warnings)]
mod export;
mod read;
mod setting;
mod table;
mod tablecell;
mod tablecellcore;
mod tableline;

extern crate clap;
use std::io::Write;

use clap::Parser;
use setting::Args;

fn main() {
    let args = Args::parse();
    let mut table;
    match &args.input {
        Some(_) => {
            table = read::read_from_file(
                args.input.as_ref().unwrap().to_str().unwrap(),
                args.seperation.as_str(),
                args.end_line.as_str(),
                &args,
            )
        }
        None => {
            table = read::read_from_io(args.seperation.as_str(), args.end_line.as_str(), &args);
        }
    }

    println!("{:#?}", args);

    //dry
    if args.dry.is_some() {
        args.to_toml(args.dry.as_ref().unwrap()).unwrap();
        return;
    }

    //set color
    match &args.output_settings.export_color {
        Some(export_color) => {
            for ((line_num, color)) in export_color.0.iter() {
                table.set_color_line(*line_num, *color);
            }
            for ((column_num, color)) in export_color.1.iter() {
                table.set_color_column(*column_num, *color);
            }
        }
        None => {}
    }

    //subtable

    //output file
    match &args.output_settings.output {
        //write to file
        Some((file_path, file_type)) => {
            use export::*;
            match file_type {
                //问题：没有去掉颜色信息
                setting::OutputFormat::Txt => {
                    table
                        .to_txt(file_path, args.seperation.chars().next().unwrap())
                        .unwrap();
                }
                setting::OutputFormat::Exls => {
                    todo!();
                    //table.to_excel(file_path, args).unwrap();
                }
                setting::OutputFormat::Csv => {
                    todo!();
                    //TODO in Export
                    //    table.to_csv().unwrap();
                }
            }
        }
        //write to stdout
        None => {
            println!("{}", table);
        }
    }

    //TODO : save config
    println!("end main");
}
