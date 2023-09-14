use env_logger::Env;
use main_error::MainResult;
use thiserror::Error;

#[macro_use]
extern crate log;

#[derive(Error, Debug)]
enum FXTMainError {
    #[error("Invalid arguments!\n\nUsage: fxt2txt <fxt_file> [key]")]
    InvalidArguments,
    #[error("Key '{0}' not found in FXT file")]
    KeyNotFound(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

fn main() -> MainResult {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 || args.len() > 3 {
        return Err(FXTMainError::InvalidArguments)?;
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let fxt_file = &args[1];
    let key = if args.len() == 3 {
        Some(&args[2])
    } else {
        None
    };
    info!("fxt_file: {}", &fxt_file);
    let map = fxt2txt::parse_fxt(fxt_file)?;
    if let Some(key) = key {
        let value = map
            .get(key)
            .ok_or(FXTMainError::KeyNotFound(key.to_string()))?;
        println!("{key} = '{value}'");
    } else {
        println!("{:?}", &map);
    }

    Ok(())
}
