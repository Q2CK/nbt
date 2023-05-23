use std::collections::HashMap;

#[derive(Debug)]
struct Block {
    coords: (i32, i32, i32),
    palette_index: i32
}

#[derive(Debug)]
pub struct MCSchematic<'a> {
    block_palette: HashMap<&'a str, i32>,
    blocks: Vec<Block>,
    width: i32,
    height: i32,
    length: i32,
}

impl<'a> MCSchematic<'a> {
    pub fn new() -> MCSchematic<'a> {
        MCSchematic {
            block_palette: HashMap::new(),
            blocks: vec![],
            width: 0,
            height: 0,
            length: 0
        }
    }

    pub fn set_block(&mut self, coords: (i32, i32, i32), block_data: &'a str) {
        let palette_size = self.block_palette.len() as i32;
        let palette_index: i32;
        if self.block_palette.contains_key(block_data) {
            palette_index = self.block_palette[block_data];
        }
        else {
            palette_index = palette_size;
            self.block_palette.insert(block_data, palette_size);
        }
        self.blocks.push(Block {
            coords,
            palette_index
        });
    }

    pub fn save(folder_path: &'a str, file_name: &'a str, version: i32) {

    }

    fn calculate_dimensions(&self) {

    }
}

