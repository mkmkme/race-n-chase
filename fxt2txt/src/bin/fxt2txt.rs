use env_logger::Env;
use log::info;
use main_error::MainResult;

fn main() -> MainResult {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 || args.len() > 3 {
        return Err("Invalid arguments!\n\nUsage: fxt2txt <fxt_file> [key]")?;
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
            .unwrap_or_else(|| panic!("Key '{}' not found", &key));
        println!("{key} = '{value}'");
    } else {
        println!("{:?}", &map);
    }

    Ok(())
}
