mod mcschematic;
use mcschematic::*;

fn main() {

    let mut schematic = MCSchematic::new();

    schematic.set_block((0, -1, 0), "minecraft:stone");
    schematic.set_block((1, -2, 0), "minecraft:cobblestone");
    schematic.set_block((0, -2, 1), "minecraft:cobblestone");
    schematic.set_block((1, -1, 1), "minecraft:stone");

    schematic.save("testmakro.schem", JE_1_18_2).expect("TODO: panic message");

}