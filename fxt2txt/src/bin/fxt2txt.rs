use env_logger::Env;
use log::info;
use main_error::MainResult;

fn main() -> MainResult {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err("Invalid arguments!\n\nUsage: fxt2txt <fxt_file>")?;
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let fxt_file = &args[1];
    info!("fxt_file: {}", &fxt_file);
    let map = fxt2txt::parse_fxt(fxt_file)?;
    println!("map: {:?}", &map);

    Ok(())
}
