mod mcschematic;
use mcschematic::*;

mod versions;

fn main() {
    let mut schematic = MCSchematic::new();

    schematic.set_block((0, 0, 0), "minecraft:stone");
    schematic.set_block((0, 0, 1), "minecraft:cobblestone");
    schematic.set_block((0, 0, 2), "minecraft:birch_planks");
    schematic.set_block((0, 0, 3), "minecraft:oak_planks");

    schematic
        .save("test.schem", versions::JE_1_20_1)
        .expect("Failed to save");
}
