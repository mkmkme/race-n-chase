use std::{
    fmt,
    fs::File,
    io::{BufReader, Read},
};
use thiserror::Error;

#[macro_use]
extern crate log;

const HEADER_GRAPHICS_VERSION: u32 = 336;
const EXPECTED_SPRITE_NUMBER: u32 = 42;

pub struct G24Parser(BufReader<File>);

pub struct G24Header {
    side_size: u32,
    lid_size: u32,
    aux_size: u32,
    animation_size: u32,
    clut_size: u32,
    tile_clut_size: u32,
    sprite_clut_size: u32,
    new_car_clut_size: u32,
    font_clut_size: u32,
    palette_index_size: u32,
    object_info_size: u32,
    car_info_size: u32,
    sprite_info_size: u32,
    sprite_graphics_size: u32,
    sprite_numbers_size: u32,
}

impl fmt::Display for G24Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "G24Header {{ ")?;
        writeln!(f, "  side_size:            {}, ", self.side_size)?;
        writeln!(f, "  lid_size:             {}, ", self.lid_size)?;
        writeln!(f, "  aux_size:             {}, ", self.aux_size)?;
        writeln!(f, "  animation_size:       {}, ", self.animation_size)?;
        writeln!(f, "  clut_size:            {}, ", self.clut_size)?;
        writeln!(f, "  tile_clut_size:       {}, ", self.tile_clut_size)?;
        writeln!(f, "  sprite_clut_size:     {}, ", self.sprite_clut_size)?;
        writeln!(f, "  new_car_clut_size:    {}, ", self.new_car_clut_size)?;
        writeln!(f, "  font_clut_size:       {}, ", self.font_clut_size)?;
        writeln!(f, "  palette_index_size:   {}, ", self.palette_index_size)?;
        writeln!(f, "  object_info_size:     {}, ", self.object_info_size)?;
        writeln!(f, "  car_info_size:        {}, ", self.car_info_size)?;
        writeln!(f, "  sprite_info_size:     {}, ", self.sprite_info_size)?;
        writeln!(f, "  sprite_graphics_size: {}, ", self.sprite_graphics_size)?;
        writeln!(f, "  sprite_numbers_size:  {} ", self.sprite_numbers_size)?;
        write!(f, "}}")
    }
}

#[derive(Debug, Error)]
pub enum G24ParserError {
    #[error("Invalid header magic version, expected 336, got {0}")]
    InvalidHeaderMagicVersion(u32),
    #[error("Invalid sprite number, expected 42, got {0}")]
    InvalidSpriteNumber(u32),
}

impl G24Parser {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        Ok(Self(BufReader::new(file)))
    }

    pub fn parse_header(&mut self) -> Result<G24Header, G24ParserError> {
        let mut buf_u32 = [0u8; 4];
        let mut read_u32 = || {
            self.0.read_exact(&mut buf_u32).expect("Failed to read u32");
            u32::from_le_bytes(buf_u32)
        };

        let version = read_u32();
        if version != HEADER_GRAPHICS_VERSION {
            return Err(G24ParserError::InvalidHeaderMagicVersion(version));
        }

        let side_size = read_u32();
        let lid_size = read_u32();
        let aux_size = read_u32();
        let animation_size = read_u32();
        let clut_size = read_u32();
        let tile_clut_size = read_u32();
        let sprite_clut_size = read_u32();
        let new_car_clut_size = read_u32();
        let font_clut_size = read_u32();
        let palette_index_size = read_u32();
        let object_info_size = read_u32();
        let car_info_size = read_u32();
        let sprite_info_size = read_u32();
        let sprite_graphics_size = read_u32();
        let sprite_numbers_size = read_u32();

        if side_size % 4096 != 0 {
            warn!("Side-Block texture size is not a multiple of 4096 ({side_size}), this might be a bug");
        }

        if lid_size % 4096 != 0 {
            warn!("Lid-Block texture size is not a multiple of 4096 ({lid_size}), this might be a bug");
        }

        if aux_size % 4096 != 0 {
            warn!("Aux-Block texture size is not a multiple of 4096 ({aux_size}), this might be a bug");
        }

        // TODO: OpenGTA code contained Aux-Block adjusting, check if it's needed

        if sprite_numbers_size != EXPECTED_SPRITE_NUMBER {
            return Err(G24ParserError::InvalidSpriteNumber(sprite_numbers_size));
        }

        Ok(G24Header {
            side_size,
            lid_size,
            aux_size,
            animation_size,
            clut_size,
            tile_clut_size,
            sprite_clut_size,
            new_car_clut_size,
            font_clut_size,
            palette_index_size,
            object_info_size,
            car_info_size,
            sprite_info_size,
            sprite_graphics_size,
            sprite_numbers_size,
        })
    }
}
