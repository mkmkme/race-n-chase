#[macro_use]
extern crate log;

mod font;
pub use font::{Font, FontParser};

// XXX: This is just a test function, it should be removed
pub fn print_bitmap(bitmap: &Vec<u8>) {
    for x in (0..bitmap.len()).step_by(4) {
        println!(
            "bitmap[{}] = {}, {}, {}, {}",
            x / 4,
            bitmap[x],
            bitmap[x + 1],
            bitmap[x + 2],
            bitmap[x + 3]
        );
    }
}
