use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use quartz_nbt::{self, compound, io::Flavor, NbtCompound, NbtList, NbtTag};

type BlockPalette<'a> = HashMap<&'a str, i32>;
type BlockData<'a> = HashMap<(i32, i32, i32), &'a str>;

/*
Trait responsible for converting a numeric type to a vector of bytes, according to the varint format


*/
trait ToVarint {
    fn to_varint(&self) -> Vec<u8>;
}

impl ToVarint for usize {
    fn to_varint(&self) -> Vec<u8> {
        vec![]
    }
}

#[derive(Debug)]
pub struct MCSchematic<'a> {
    /*
    Structure representing the schematic being generated
    */

    block_palette: BlockPalette<'a>,
    block_data: BlockData<'a>,
    width: i32,
    height: i32,
    length: i32,
}

impl<'a> MCSchematic<'a> {
    pub fn new() -> MCSchematic<'a> {
        /*
        Method that returns a new, empty instance of the MCSchematic structure.
        The block palette and the blocks list get updated as new blocks are
        placed in the schematic
         */

        MCSchematic {
            block_palette: BlockPalette::new(),
            block_data: BlockData::new(),
            width: 0,
            height: 0,
            length: 0
        }
    }

    pub fn set_block(&mut self, coords: (i32, i32, i32), block_data: &'a str) {
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
        self.block_data.insert(coords, block_data);
    }

    pub fn save(&self, folder_path: &'a str, file_name: &'a str, version: i32) -> Result<String, String> {

        // Open the target schematic file with the provided name
        let mut file_out;
        match OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
        {
            Ok(v) => file_out = v,
            Err(e) => return Err("Failed to save file".to_string())
        }

        // Create, fill and insert the block palette tag
        let mut palette = NbtCompound::new();
        for block_type in self.block_palette.iter() {
            palette.insert(*block_type.0, NbtTag::Int(*block_type.1));
        }
        // Create a new nbt root
        let mut nbt: NbtCompound = compound!(
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

        Ok(format!("Saved to {}/{}", folder_path, file_name))
    }
}

