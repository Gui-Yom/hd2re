use std::fs;

use hd2re::convert::convert_all_to_wav;
use speedy::{Readable, Writable};

use hd2re::index::HD2Index;
use hd2re::parse::DataType;
use hd2re::sniff::libmagic::LibMagicSniff;
use hd2re::sniff::magika::MagikaSniff;

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
    // for (&key, [data, stream, gpu]) in magika_sniff.results.iter() {
    //     if !index[key].record.type_id.is_known() {
    //         println!(
    //             "{key:016x} ({:?}): ({}, {}), ({}, {}), ({}, {})",
    //             index[key].record.type_id,
    //             data[0].0,
    //             &magika_sniff.labels[data[0].1 as usize],
    //             stream[0].0,
    //             &magika_sniff.labels[stream[0].1 as usize],
    //             gpu[0].0,
    //             &magika_sniff.labels[gpu[0].1 as usize]
    //         );
    //     }
    // }
    let libmagic_sniff = if let Ok(sniff) = LibMagicSniff::read_from_file("hd2sniff.libmagic.bin") {
        println!("Loading saved libmagic sniff.");
        sniff
    } else {
        println!("No libmagic sniff available.");
        let sniff = LibMagicSniff::run(&index);
        sniff.write_to_file("hd2sniff.libmagic.bin").unwrap();
        sniff
    };
    // for (&key, (data, stream, gpu)) in &libmagic_sniff.results {
    //     if !index[key].record.type_id.is_known()
    //         && !(LibMagicSniff::guess_is_worthless(data)
    //             && LibMagicSniff::guess_is_worthless(stream)
    //             && LibMagicSniff::guess_is_worthless(gpu))
    //     {
    //         println!(
    //             "{key:016x} ({:?}): {data}, {stream}, {gpu}",
    //             index[key].record.type_id
    //         );
    //     }
    // }

    // Dump WAV
    fs::create_dir_all("data/audio/wem").unwrap();
    for key in index.ids() {
        if index[key].record.type_id == DataType::WEM {
            fs::write(
                format!("data/audio/wem/{key:016x}.wem"),
                index.load_stream_bytes(key).unwrap(),
            )
            .unwrap();
        }
    }
    convert_all_to_wav();
}
