use fxt2txt::file_decoder::FileDecoder;
use main_error::MainResult;

fn main() -> MainResult {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err("Invalid arguments!\n\nUsage: decode-file <fxt_file>")?;
    }

    let fxt_file = &args[1];
    let decoder = FileDecoder::new(fxt_file)?;
    let str: String = decoder.into_iter().collect();
    println!("{}", &str);
    Ok(())
}
