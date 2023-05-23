use std::fs::{File, OpenOptions};
use quartz_nbt::{self, io::Flavor, NbtCompound, NbtList, NbtTag};

mod mcschematic;
use mcschematic::*;


fn main() {
    let mut file_out = OpenOptions::new()
        .write(true)
        .open("mcs.schem")
        .unwrap();

    let mut nbt_compound = quartz_nbt::NbtCompound::new();

    nbt_compound.insert("DataVersion", NbtTag::Int(2975));
    nbt_compound.insert("Version", NbtTag::Int(2));

    nbt_compound.insert("Height", NbtTag::Short(1));
    nbt_compound.insert("Length", NbtTag::Short(1));
    nbt_compound.insert("Width", NbtTag::Short(1));

    let mut palette = NbtCompound::new();
    palette.insert("minecraft:air", NbtTag::Int(0));
    palette.insert("minecraft:stone", NbtTag::Int(1));

    nbt_compound.insert("Palette", NbtTag::Compound(palette));
    nbt_compound.insert("PalettteMax", NbtTag::Int(2));
    nbt_compound.insert("BlockData", NbtTag::ByteArray(vec![1]));
    nbt_compound.insert("BlockEntities", NbtTag::List(NbtList::new()));

    quartz_nbt::io::write_nbt(
        &mut file_out,
        Some("Schematic"),
        &nbt_compound,
        Flavor::GzCompressed)
        .expect("TODO: panic message");

    println!("{:#?}", nbt_compound);






    let mut schematic = MCSchematic::new();
    schematic.set_block((0,0,0), "test");
    schematic.set_block((4,2,0), "test");
    schematic.set_block((0,1,0), "test2");
    schematic.set_block((3,2,0), "test");
    println!("{:#?}", schematic);
}