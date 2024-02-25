use speedy::{Readable, Writable};

use hd2re::index::HD2Index;
use hd2re::sniff::{LibMagicSniff, MagikaSniff};

fn main() {
    // println!("{:x}", stringray_hash(b"packages/pre_boot"));
    // println!("{:x}", stringray_hash(b"packages/boot"));
    // println!("{:x}", stringray_hash(b"texture"));
    let index = if let Ok(hd2fs) = HD2Index::read_from_file("hd2index.bin") {
        println!("Loading saved hd2index.");
        hd2fs
    } else {
        println!("No index available, building ...");
        let hd2fs = HD2Index::create_index(r#"E:\SteamLibrary\steamapps\common\Helldivers 2\data"#);
        hd2fs.write_to_file("hd2index.bin").unwrap();
        hd2fs
    };
    println!("Loaded metadata for {} assets", index.len());
    let magika_sniff = if let Ok(sniff) = MagikaSniff::read_from_file("hd2sniff.magika.bin") {
        println!("Loading saved magika sniff.");
        sniff
    } else {
        println!("No magika sniff available.");
        let sniff = MagikaSniff::run(&index);
        sniff.write_to_file("hd2sniff.magika.bin").unwrap();
        sniff
    };
    // dbg!(magika_sniff);
    let libmagic_sniff = if let Ok(sniff) = LibMagicSniff::read_from_file("hd2sniff.libmagic.bin") {
        println!("Loading saved libmagic sniff.");
        sniff
    } else {
        println!("No libmagic sniff available.");
        let sniff = LibMagicSniff::run(&index);
        sniff.write_to_file("hd2sniff.libmagic.bin").unwrap();
        sniff
    };
    for (key, label) in libmagic_sniff.results.iter().take(10) {
        println!("{key:016x}: {label}");
    }
}
