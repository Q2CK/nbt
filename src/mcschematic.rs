use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::OpenOptions;

use quartz_nbt::{self, compound, io::Flavor, NbtCompound, NbtTag};

pub mod versions;
pub use versions::*;

type BlockPalette<'a> = HashMap<&'a str, i32>;
type BlockData = HashMap<(i32, i32, i32), i32>;

type Coords = (i32, i32, i32);

type Byte = i8;

// Function that executes another function on pairs of tuple entries and returns the resulting tuple
fn on_tuple<T>(f: fn(T, T) -> T, lhs: (T, T, T), rhs: (T, T, T)) -> (T, T, T) {
    (f(lhs.0, rhs.0), f(lhs.1, rhs.1), f(lhs.2, rhs.2))
}

pub trait Varint {
    /*
    Trait responsible for converting a numeric type to a vector of bytes, according to the varint format
    and vice versa.

    https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-2.md
    "Each integer is bitpacked into a single Byte with varint encoding.
    The first Byte determines the length of the integer with a maximum length
    of 5 (for a 32 bit number), and depending on the length, each proceeding Byte
    is or'ed and current value bit shifted by the length multiplied by 7. Examples can be
    found with Sponge's implementation for retreving data and storing data."
    */

    fn to_varint(&self) -> Vec<Byte>;
}

impl<'a> Varint for usize {

    fn to_varint(&self) -> Vec<Byte> {
        const MASK_7_BIT: i8 = 127;

        let mut input = *self;
        let mut output: Vec<Byte> = vec![];

        if input == 0 { return vec![0] };

        while input != 0 {
            let mut new_byte = (input & (MASK_7_BIT as usize)) as Byte;

            if input > MASK_7_BIT as usize {
                new_byte |= !MASK_7_BIT;
            }

            output.push(new_byte);
            input >>= 7;
        }
        return output;
    }
}

#[derive(Debug)]
pub struct MCSchematic<'a> {
    /*
    Structure representing the schematic being generated
    */

    block_palette: BlockPalette<'a>,
    block_data: BlockData,

    lowest_coords: Coords,
    highest_coords: Coords,

    length: i32,
    width: i32,
    height: i32
}

impl<'a> MCSchematic<'a> {
    pub fn new() -> MCSchematic<'a> {
        /*
        Method that returns a new, empty instance of the MCSchematic structure.
        The block palette, schematic boundaries and the blocks list get updated
        as new blocks are placed in the schematic
         */

        MCSchematic {
            block_palette: BlockPalette::from([("minecraft:air", 0)]),
            block_data: BlockData::new(),
            lowest_coords: (0, 0, 0),
            highest_coords: (0, 0, 0),

            length: 0,
            height: 0,
            width: 0
        }
    }

    pub fn set_block(&mut self, coords: Coords, block_data: &'a str) {
        /*
        Method that adds a new block to the schematic, updating the palette
        and blocks list as needed.

        block_data is the full in-game id of the block, such as "minecraft::stone"
         */

        // Store the current palette size
        let palette_size = self.block_palette.len() as i32;

        let palette_index: i32;

        // Check if the new block is already present in the palette, add to palette if not
        if self.block_palette.contains_key(block_data) {
            palette_index = self.block_palette[block_data];
        }
        else {
            palette_index = palette_size;
            self.block_palette.insert(block_data, palette_size);
        }

        // Add the new block to the blocks list with the given coords and its index in the palette
        self.block_data.insert(coords, palette_index);

        // Update the lowest and highest coords if needed
        if self.block_data.len() == 1 {
            self.lowest_coords = coords;
            self.highest_coords = coords;
        }
        else {
            self.lowest_coords = on_tuple(min, self.lowest_coords, coords);
            self.highest_coords = on_tuple(max, self.highest_coords, coords);
        }
    }

    pub fn save(&mut self, file_path: &'a str, version: i32) -> Result<String, String> {

        // Open the target schematic file with the provided name
        let mut file_out;
        match OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
        {
            Ok(v) => file_out = v,
            Err(e) => return Err(format!("Failed to save file: {e}"))
        }

        // Store the dimensions of the generated schematic
        (self.length, self.height, self.width) = self.get_dimensions();

        let palette_tag = self.generate_palette_tag();

        // Create the BlockData from the accumulated list of blocks
        let block_data_tag = self.generate_block_data_tag();

        // Create a new nbt root
        let nbt: NbtCompound = compound!(
            "DataVersion": NbtTag::Int(version),
            "Version": NbtTag::Int(2),

            "PalettteMax": NbtTag::Int(self.block_palette.len() as i32),
            "Palette": palette_tag,

            "BlockData": block_data_tag,

            "Metadata": {
                "MCSchematicMetadata" : {
                    "Mitochondria": "is the powerhouse of a cell"
                }
            },

            // TODO Remove unwrap
            "Length": NbtTag::Short(self.length.try_into().unwrap()),
            "Height": NbtTag::Short(self.height.try_into().unwrap()),
            "Width": NbtTag::Short(self.width.try_into().unwrap()),
        );

        quartz_nbt::io::write_nbt(
        &mut file_out,
        Some("Schematic"),
        &nbt,
        Flavor::GzCompressed)
        .expect("TODO: panic message");

        Ok(format!("Saved to {}", file_path))
    }

    fn generate_palette_tag(&self) -> NbtCompound {
        /*
        Generates the block palette nbt compound tag based on the
        list of blocks used in the generating program
        */

        let mut palette = NbtCompound::new();
        for block_type in self.block_palette.iter() {
            palette.insert(*block_type.0, NbtTag::Int(*block_type.1));
        }

        return palette;
    }

    fn generate_block_data_tag(&self) -> NbtTag {
        /*
        Generates the block data object, which is a byte array of varint-encoded numbers
        that correspond to indexes in the block palette

        The entries are indexed by: x + z * Width + y * Width * Length relative to the lowest coords
        */

        let mut bytes: Vec<Byte> = vec![];

        for z in 0..self.width {
            for y in 0..self.height {
                for x in 0..self.length {
                    bytes.append(match self.block_data.get(
                        &on_tuple(std::ops::Sub::sub,(x, y, z), self.get_dimensions())) {
                        Some(v) => (*v as usize).to_varint(),
                        None => vec![0]
                    }.as_mut()
                    );
                }
            }
        }

        return NbtTag::ByteArray(bytes);
    }

    fn get_dimensions(&self) -> (i32, i32, i32) {
        /*
        Returns a tuple containing the length, height and width of the schematic
        by subtracting the lwoest coords from the highest coords
        */
        return on_tuple(|lhs, rhs| lhs - rhs + 1, self.highest_coords, self.lowest_coords);
    }
}

