use std::fs::{File, OpenOptions};
use quartz_nbt::{self, io::Flavor, NbtTag};


fn main() {
    let mut file_in = File::open("my_cool_schematic.schem").unwrap();

    let mut file_out = OpenOptions::new()
        .write(true)
        .open("mcs.schem")
        .unwrap();

    let data = quartz_nbt::io::read_nbt(
        &mut file_in,
        Flavor::GzCompressed)
        .unwrap();

    let mut nbt_compound = quartz_nbt::NbtCompound::new();

    nbt_compound.insert("BlockData", NbtTag::ByteArray(vec![0]));
    nbt_compound.insert("BlockEntities", NbtTag::ByteArray(vec![]));
    nbt_compound.insert("DataVersion", NbtTag::Short(2586));
    nbt_compound.insert("Height", NbtTag::Byte(2));
    nbt_compound.insert("Length", NbtTag::Byte(2));
    nbt_compound.insert("Width", NbtTag::Byte(2));

    quartz_nbt::io::write_nbt(
        &mut file_out,
        Some("Schematic"),
        &nbt_compound,
        Flavor::GzCompressed)
        .expect("TODO: panic message");

    println!("{:#?}", data);
}