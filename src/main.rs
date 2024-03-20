mod mcschematic;
use mcschematic::*;

use quartz_nbt;

mod versions;

fn main() {
    let mut schematic = MCSchematic::new();

    schematic.set_block((0, 0, 0), "minecraft:stone");
    schematic.set_block((0, 0, 1), "minecraft:cobblestone");
    schematic.set_block((0, 0, 2), "minecraft:birch_planks");
    schematic.set_block((0, 0, 3), "minecraft:oak_planks");

    let test = quartz_nbt::snbt::parse(
        r#"{CustomName:'{"italic":false,"text":"15"}',Items:[{Count:64b,Slot:0b,id:"minecraft:redstone"},{Count:64b,Slot:1b,id:"minecraft:redstone"},{Count:64b,Slot:2b,id:"minecraft:redstone"},{Count:64b,Slot:3b,id:"minecraft:redstone"},{Count:64b,Slot:4b,id:"minecraft:redstone"},{Count:64b,Slot:5b,id:"minecraft:redstone"},{Count:64b,Slot:6b,id:"minecraft:redstone"},{Count:64b,Slot:7b,id:"minecraft:redstone"},{Count:64b,Slot:8b,id:"minecraft:redstone"},{Count:64b,Slot:9b,id:"minecraft:redstone"},{Count:64b,Slot:10b,id:"minecraft:redstone"},{Count:64b,Slot:11b,id:"minecraft:redstone"},{Count:64b,Slot:12b,id:"minecraft:redstone"},{Count:64b,Slot:13b,id:"minecraft:redstone"},{Count:64b,Slot:14b,id:"minecraft:redstone"},{Count:64b,Slot:15b,id:"minecraft:redstone"},{Count:64b,Slot:16b,id:"minecraft:redstone"},{Count:64b,Slot:17b,id:"minecraft:redstone"},{Count:64b,Slot:18b,id:"minecraft:redstone"},{Count:64b,Slot:19b,id:"minecraft:redstone"},{Count:64b,Slot:20b,id:"minecraft:redstone"},{Count:64b,Slot:21b,id:"minecraft:redstone"},{Count:64b,Slot:22b,id:"minecraft:redstone"},{Count:64b,Slot:23b,id:"minecraft:redstone"},{Count:64b,Slot:24b,id:"minecraft:redstone"},{Count:64b,Slot:25b,id:"minecraft:redstone"},{Count:64b,Slot:26b,id:"minecraft:redstone"},{Count:0b,Slot:27b,id:"minecraft:redstone"}]}"#,
    );

    println!("{:#?}", test);

    schematic
        .save("test.schem", versions::JE_1_20_1)
        .expect("Failed to save");
}
