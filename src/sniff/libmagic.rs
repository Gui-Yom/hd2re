use crate::hash::NoHash;
use crate::index::{AssetMap, HD2Index};
use magic::cookie::Flags;
use speedy::{Readable, Writable};
use std::convert::TryInto;

#[derive(Debug, Readable, Writable)]
pub struct LibMagicSniff {
    pub results: AssetMap<(String, String, String)>,
}

impl LibMagicSniff {
    pub fn run(index: &HD2Index) -> Self {
        println!("Using libmagic {}", magic::libmagic_version());
        let cookie = magic::Cookie::open(Flags::empty())
            .unwrap()
            .load(&[r#"C:\apps\vcpkg\packages\libmagic_x64-windows-static-md\share\libmagic\misc\magic.mgc"#].try_into().unwrap())
            .unwrap();
        let mut results = AssetMap::with_capacity_and_hasher(index.len(), NoHash);
        for (i, key) in index.ids().enumerate() {
            let data = index
                .load_data_bytes(key)
                .map(|b| cookie.buffer(b.as_slice()).unwrap())
                .unwrap_or(String::new());
            let stream = index
                .load_stream_bytes(key)
                .map(|b| cookie.buffer(b.as_slice()).unwrap())
                .unwrap_or(String::new());
            let gpu = index
                .load_gpu_bytes(key)
                .map(|b| cookie.buffer(b.as_slice()).unwrap())
                .unwrap_or(String::new());
            results.insert(key, (data, stream, gpu));
            if i % 1000 == 0 {
                println!("Processed {i}/{}", index.len());
            }
        }
        Self { results }
    }

    pub fn guess_is_worthless(guess: &str) -> bool {
        matches!(guess, "data" | "empty" | "")
    }
}
