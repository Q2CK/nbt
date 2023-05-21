use std::fs::{File, OpenOptions};
use quartz_nbt;
use quartz_nbt::io::Flavor;

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

    quartz_nbt::io::write_nbt(
        &mut file_out,
        Some("Schematic"),
        &data.0,
        Flavor::GzCompressed)
        .expect("TODO: panic message");

    println!("{:#?}", data);
}