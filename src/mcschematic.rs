use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::OpenOptions;

use quartz_nbt::{self, compound, io::Flavor, NbtCompound, NbtTag};

pub mod versions;
pub use versions::*;

type BlockPalette<'a> = HashMap<&'a str, i32>;
type BlockData = HashMap<(i32, i32, i32), i32>;

type Coords = (i32, i32, i32);

type Byte = u8;

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
        const MASK_7_BIT: usize = 127;

        let mut input = *self;
        let mut output: Vec<Byte> = vec![];

        if input == 0 { return vec![0] };

        while input != 0 {
            let mut new_byte = input & MASK_7_BIT;

            if input > MASK_7_BIT {
                new_byte |= !MASK_7_BIT;
            }

            output.push(new_byte as u8);
            input >>= 7;
        }
        return output;
    }
}

// Function that executes another function on pairs of tuple entries and returns the resulting tuple
fn on_tuple<T>(f: fn(T, T) -> T, lhs: (T, T, T), rhs: (T, T, T)) -> (T, T, T) {
    (f(lhs.0, rhs.0), f(lhs.1, rhs.1), f(lhs.2, rhs.2))
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
}

impl<'a> MCSchematic<'a> {
    pub fn new() -> MCSchematic<'a> {
        /*
        Method that returns a new, empty instance of the MCSchematic structure.
        The block palette, schematic boundaries and the blocks list get updated
        as new blocks are placed in the schematic
         */

        MCSchematic {
            block_palette: BlockPalette::new(),
            block_data: BlockData::new(),
            lowest_coords: (0, 0, 0),
            highest_coords: (0, 0, 0),
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
        self.lowest_coords = on_tuple(min, self.lowest_coords, coords);
        self.highest_coords = on_tuple(max, self.highest_coords, coords);
    }

    pub fn save(&self, file_path: &'a str, version: i32) -> Result<String, String> {

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

        // Create, fill and insert the block palette tag
        let mut palette = NbtCompound::new();
        for block_type in self.block_palette.iter() {
            palette.insert(*block_type.0, NbtTag::Int(*block_type.1));
        }

        // Create the BlockData from the accumulated list of blocks
        // TODO

        // Create a new nbt root
        let nbt: NbtCompound = compound!(
            "DataVersion": NbtTag::Int(version),
            "Version": NbtTag::Int(2),
            "PalettteMax": NbtTag::Int(self.block_palette.len() as i32),
            "Palette": palette,
            "Metadata": {
                "MCSchematicMetadata" : {
                    "Mitochondria": "is the powerhouse of a cell"
                }
            }
        );

        quartz_nbt::io::write_nbt(
        &mut file_out,
        Some("Schematic"),
        &nbt,
        Flavor::GzCompressed)
        .expect("TODO: panic message");

        Ok(format!("Saved to {}", file_path))
    }
}

