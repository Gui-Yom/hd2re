use onnxruntime::GraphOptimizationLevel;

use crate::HD2Index;

pub(crate) fn sniff_wav(index: &HD2Index) {
    for &item in index.items.keys() {
        if index
            .load_n_stream_bytes::<4>(item)
            .map(|b| &b == b"RIFF")
            .unwrap_or(false)
        {
            println!("{item} : WAV ({:?})", index.items[&item].record.type_id);
        }
    }
}

pub(crate) fn sniff_magika(index: &HD2Index) {
    let mut magika = magika::MagikaBuilder::new()
        .with_number_threads(4)
        .with_optimization_level(GraphOptimizationLevel::All)
        .build("../magika/python/magika/models/standard_v1")
        .unwrap();
    println!("Running inference");
    for &key in index.items.keys() {
        let bytes = index.load_data_bytes(key).unwrap();
        dbg!(magika.identify(bytes.as_slice()).unwrap());
    }
}
