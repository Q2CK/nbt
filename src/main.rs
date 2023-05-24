mod mcschematic;
use mcschematic::*;

fn main() {/*
    let mut file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open("mcs2.schem")
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
    println!("{:#?}", schematic);*/

    let mut schematic = MCSchematic::new();

    schematic.set_block((0, 0, -1), "minecraft:stone");
    schematic.set_block((-2, -5, 0), "minecraft:bone");
    schematic.set_block((-1, 0, -10), "minecraft:srone");
    schematic.set_block((1, 3, 0), "minecraft:fafone");
    schematic.set_block((4, 0, -7), "minecraft:telephone");

    schematic.save("testmakro.schem", JE_1_18_1).expect("TODO: panic message");

    println!("{:#?}", schematic);

    //println!("{:?}", (129 as usize).to_varint().iter().map(|x| format!("{:08b} ", x)).collect::<String>());
}