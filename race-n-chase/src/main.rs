use clap::{arg, Command};
use env_logger::Env;
use font_dump::{print_bitmap, Font, FontParser};
use g24parser::{G24Parser, G24ParserError};
use main_error::MainResult;
use thiserror::Error;

#[macro_use]
extern crate log;

#[derive(Error, Debug)]
enum RNCError {
    #[error("Key {0:?} not found in FXT file")]
    KeyNotFound(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    G24ParserError(#[from] G24ParserError),
}

fn cli() -> Command {
    Command::new("race-n-chase")
        .subcommand_required(true)
        .subcommand(
            Command::new("dump-car")
                .about("Dump car info")
                .arg(arg!(<FILE> "Style file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("dump-g24-header")
                .about("Dump g24 header info")
                .arg(arg!(<FILE> "g24 file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("fxt2txt")
                .about("Decode FXT file")
                .arg(arg!(<FILE> "FXT file to decode"))
                .arg(arg!([KEY] "Key to search"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("font-dump")
                .about("Dump font info")
                .arg(arg!(<FILE> "Font file"))
                .arg_required_else_help(true),
        )
}

fn main() -> MainResult {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("dump-car", sub_matches)) => {
            let path = sub_matches.get_one::<String>("FILE").expect("required");
            println!("path is {:?}", path);
        }
        Some(("dump-g24-header", sub_matches)) => {
            let path = sub_matches.get_one::<String>("FILE").expect("required");
            return run_dump_g24_header(path);
        }
        Some(("fxt2txt", sub_matches)) => {
            let path = sub_matches.get_one::<String>("FILE").expect("required");
            let key = sub_matches.get_one::<String>("KEY");
            return run_fxt2txt(path, key.map(|x| x.as_str()));
        }
        Some(("font-dump", sub_matches)) => {
            let path = sub_matches.get_one::<String>("FILE").expect("required");
            return run_font_dump(path);
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn run_fxt2txt(filename: &str, key: Option<&str>) -> MainResult {
    let map = fxt2txt::parse_fxt(filename)?;
    if let Some(key) = key {
        let value = map.get(key).ok_or(RNCError::KeyNotFound(key.to_string()))?;
        println!("{key} = {value:?}");
    } else {
        println!("{map:?}");
    }
    Ok(())
}

fn run_dump_g24_header(filename: &str) -> MainResult {
    let mut parser = G24Parser::new(filename)?;
    let header = parser.parse_header()?;
    println!("{header}");
    Ok(())
}

fn run_font_dump(filename: &str) -> MainResult {
    info!("Parsing font file: {filename}");
    let mut parser = FontParser::new(filename)?;
    let font = Font::new(&mut parser);
    let char_bitmap = font.character_bitmap(0);
    print_bitmap(&char_bitmap);
    Ok(())
}
