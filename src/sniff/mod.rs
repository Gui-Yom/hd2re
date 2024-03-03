use ort::ExecutionProvider;
use speedy::{Readable, Writable};

use crate::index::HD2Index;

pub mod libmagic;
#[cfg(feature = "sniff-magika")]
pub mod magika;

pub(crate) fn sniff_wav(index: &HD2Index) {
    for item in index.ids() {
        if index
            .load_n_stream_bytes::<4>(item)
            .map(|b| &b == b"RIFF")
            .unwrap_or(false)
        {
            println!("{item} : WAV ({:?})", index[item].record.type_id);
        }
    }
}
