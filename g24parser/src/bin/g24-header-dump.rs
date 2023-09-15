use env_logger::Env;
use main_error::MainResult;

use g24parser::G24Parser;

fn main() -> MainResult {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err("Invalid arguments!\n\nUsage: g24-header-dump <g24_file>")?;
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let g24_file = &args[1];
    let mut parser = G24Parser::new(g24_file)?;
    let header = parser.parse_header()?;
    println!("{}", header);
    Ok(())
}
