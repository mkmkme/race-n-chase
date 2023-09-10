use std::{
    fs::File,
    io::{BufReader, Read},
};

pub(super) trait Decoder {
    fn position(&self) -> usize;
    fn next_char(&mut self) -> Result<char, std::io::Error>;
}

/// A decoder that reads from a file byte by byte and decodes the bytes into characters.
pub(super) struct FileDecoder {
    reader: BufReader<File>,
    cur_char: [u8; 1],
    cur_pos: usize,
    char_unread: bool,
}

impl FileDecoder {
    pub fn new(filename: &str) -> Result<FileDecoder, std::io::Error> {
        let file = File::open(filename)?;
        Ok(FileDecoder {
            reader: BufReader::new(file),
            cur_char: [0; 1],
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
        let mut c = self.cur_char[0] as i16;
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

impl Decoder for FileDecoder {
    fn position(&self) -> usize {
        self.cur_pos
    }

    fn next_char(&mut self) -> Result<char, std::io::Error> {
        if self.char_unread {
            self.char_unread = false;
            return Ok(self.decode_char());
        }
        match self.reader.read_exact(&mut self.cur_char) {
            Ok(_) => Ok(self.decode_char()),
            Err(e) => {
                Err(e)
            }
        }
    }
}
