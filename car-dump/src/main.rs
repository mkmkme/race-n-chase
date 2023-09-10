use std::env;
use env_logger::Env;

#[macro_use]
extern crate log;

use main_error::MainResult;

mod error;

fn main() -> MainResult {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(error::RNCError::InvalidArguments)?;
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let car_file = &args[1];
    info!("car_file: {}", &car_file);
    Ok(())
}
