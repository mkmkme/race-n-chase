use std::{
    fs::File,
    io::{BufReader, Read},
};

pub struct Character {
    pub width: u8,
    pub data: Vec<u8>,
}

struct Palette([u8; 256 * 3]);

pub struct Font {
    char_height: u8,
    chars: Vec<Character>,
    palette: Palette,
}

pub struct FontParser(BufReader<File>);

fn read_byte(parser: &mut FontParser) -> u8 {
    let mut buf = [0u8; 1];
    parser.0.read_exact(&mut buf).expect("Failed to read byte");
    buf[0]
}

impl Character {
    pub fn new(parser: &mut FontParser, height: u8) -> Self {
        let width = read_byte(parser);
        let length = height as usize * width as usize;
        let mut data = vec![0; length];
        parser
            .0
            .read_exact(&mut data)
            .expect("Failed to read character data");
        Self { width, data }
    }
}

impl Palette {
    fn new(parser: &mut FontParser) -> Self {
        let mut palette = [0; 256 * 3];
        parser
            .0
            .read_exact(&mut palette)
            .expect("Failed to read palette");
        Self(palette)
    }

    fn apply(&self, data: &Vec<u8>) -> Vec<u8> {
        let mut result = Vec::with_capacity(data.len() * 4);
        for &idx in data {
            let idx = idx as usize * 3;
            result.push(self.0[idx]);
            result.push(self.0[idx + 1]);
            result.push(self.0[idx + 2]);
            if idx == 0 {
                result.push(0);
            } else {
                result.push(0xff);
            }
        }
        result
    }
}

impl Font {
    pub fn new(parser: &mut FontParser) -> Self {
        let num_chars = read_byte(parser);
        let char_height = read_byte(parser);
        info!("font contains {num_chars} characters, each {char_height} pixels high");
        let mut chars = Vec::with_capacity(num_chars as usize);
        for _ in 0..num_chars {
            chars.push(Character::new(parser, char_height));
        }
        Self {
            char_height,
            chars,
            palette: Palette::new(parser),
        }
    }

    pub fn character_height(&self) -> u8 {
        self.char_height
    }

    pub fn character_width(&self) -> u8 {
        self.chars[0].width
    }

    pub fn character_bitmap(&self, idx: usize) -> Vec<u8> {
        let char = &self.chars[idx];
        self.palette.apply(&char.data)
    }
}

impl FontParser {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        Ok(Self(BufReader::new(file)))
    }
}
