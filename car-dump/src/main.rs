use std::env;

#[macro_use]
extern crate log;

use main_error::MainResult;

mod error;

fn main() -> MainResult {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(error::RNCError::InvalidArguments)?;
    }

    env_logger::init();

    let car_file = &args[1];
    info!("car_file: {}", &car_file);
    Ok(())
}
