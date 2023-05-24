use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use quartz_nbt::{self, io::Flavor, NbtCompound, NbtList, NbtTag};


#[derive(Debug)]
struct Block {
    /*
    Block structure, holds the coordinates of every block
    and its index in the block palette
    */

    coords: (i32, i32, i32),
    palette_index: i32
}

#[derive(Debug)]
pub struct MCSchematic<'a> {
    /*
    Structure representing the schematic being generated
    */

    block_palette: HashMap<&'a str, i32>,
    blocks: Vec<Block>,
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
            block_palette: HashMap::new(),
            blocks: vec![],
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
        self.blocks.push(Block {
            coords,
            palette_index
        });
    }

    pub fn save(&self, folder_path: &'a str, file_name: &'a str, version: i32) {

        // Open the target schematic file with the provided name
        let mut file_out = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();

        // Create a new nbt root
        let mut nbt = NbtCompound::new();

        // Insert version info tags. DataVersion depends on the game version.
        nbt.insert("DataVersion", NbtTag::Int(version));

        //"Mcschem only implements version 2 because its the most popular" ~ Sloimay
        nbt.insert("Version", NbtTag::Int(2));

        // Insert the block palette size tag
        nbt.insert("PalettteMax", NbtTag::Int(self.block_palette.len() as i32));

        // Create, fill and insert the block palette tag
        let mut palette = NbtCompound::new();
        for block_type in self.block_palette.iter() {
            palette.insert(*block_type.0, NbtTag::Int(*block_type.1));
        }
        nbt.insert("Palette", NbtTag::Compound(palette));
    }

    fn calculate_dimensions(&self) {

    }
}

