use fxt2txt::file_decoder::{Decoder, FileDecoder};
use main_error::MainResult;

fn main() -> MainResult {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err("Invalid arguments!\n\nUsage: decode-file <fxt_file>")?;
    }

    let fxt_file = &args[1];
    let mut decoder = FileDecoder::new(fxt_file)?;
    while let Ok(c) = decoder.next_char() {
        print!("{}", c);
    }
    Ok(())
}
