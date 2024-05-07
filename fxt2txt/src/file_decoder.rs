use std::{
    fs::File,
    io::{BufReader, Error, Read},
};

pub trait Decoder {
    fn position(&self) -> usize;
}

/// A decoder that reads from a file byte by byte and decodes the bytes into characters.
pub struct FileDecoder {
    reader: BufReader<File>,
    cur_char: u8,
    cur_pos: usize,
    char_unread: bool,
}

impl FileDecoder {
    pub fn new(filename: &str) -> Result<FileDecoder, Error> {
        let file = File::open(filename)?;
        Ok(FileDecoder {
            reader: BufReader::new(file),
            cur_char: 0,
            cur_pos: 0,
            char_unread: false,
        })
    }

    /// Decode the current byte into a character.
    ///
    /// ## Decoding algorithm
    ///
    /// The encoding is a simple one-character shift. The first 8 bytes,
    /// however, are encoded with a different algorithm. The first byte is
    /// subtracted by 1, then the result is subtracted by 0x63 shifted by the
    /// current position. The result is then converted to a character.
    fn decode_char(&mut self) -> char {
        let mut c = self.cur_char as i16;
        c -= 1;
        if self.cur_pos <= 7 {
            c -= 0x63 << self.cur_pos;
            while c < 0 {
                c += 256;
            }
        }
        self.cur_pos += 1;
        c as u8 as char
    }
}

impl Iterator for FileDecoder {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.char_unread {
            self.char_unread = false;
            return Some(self.decode_char());
        }
        self.reader
            .read_exact(std::slice::from_mut(&mut self.cur_char))
            .map(|_| self.decode_char())
            .ok()
    }
}

impl Decoder for FileDecoder {
    fn position(&self) -> usize {
        self.cur_pos
    }
}
