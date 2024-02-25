use std::num::NonZeroUsize;

use magic::cookie::{DatabasePaths, Flags};
use magika::MagikaSession;
use ort::{
    CUDAExecutionProvider, DirectMLExecutionProvider, ExecutionProvider, GraphOptimizationLevel,
    Session, TensorRTExecutionProvider, XNNPACKExecutionProvider,
};
use speedy::{Readable, Writable};

use crate::hash::NoHash;
use crate::index::{AssetMap, HD2Index};

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

#[derive(Debug, Readable, Writable)]
pub struct LibMagicSniff {
    pub results: AssetMap<String>,
}

impl LibMagicSniff {
    pub fn run(index: &HD2Index) -> Self {
        println!("Using libmagic {}", magic::libmagic_version());
        let cookie = magic::Cookie::open(Flags::empty())
            .unwrap()
            .load(&[r#"C:\apps\vcpkg\packages\libmagic_x64-windows-static-md\share\libmagic\misc\magic.mgc"#].try_into().unwrap())
            .unwrap();
        let mut map = AssetMap::with_capacity_and_hasher(index.len(), NoHash);
        for (i, key) in index.ids().enumerate() {
            let bytes = index.load_data_bytes(key).unwrap();
            map.insert(key, cookie.buffer(bytes.as_slice()).unwrap());
            if i % 1000 == 0 {
                println!("Processed {i}/{}", index.len());
            }
        }
        Self { results: map }
    }
}

#[derive(Debug, Readable, Writable)]
pub struct MagikaSniff {
    pub labels: Vec<String>,
    pub results: AssetMap<[(f32, u32); 3]>,
}

impl MagikaSniff {
    pub fn run(index: &HD2Index) -> Self {
        ort::init().commit().unwrap();
        let session_builder = Session::builder()
            .unwrap()
            .with_parallel_execution(true)
            .unwrap()
            .with_inter_threads(2)
            .unwrap()
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .unwrap();

        print!("Available ONNX execution providers : ");
        let directml = DirectMLExecutionProvider::default();
        if directml.is_available().unwrap() {
            print!("DIRECTML ");
            directml.register(&session_builder);
        }
        let tensorrt = TensorRTExecutionProvider::default();
        if tensorrt.is_available().unwrap() {
            print!("TENSORRT ");
            tensorrt.register(&session_builder);
        }
        let cuda = CUDAExecutionProvider::default();
        if cuda.is_available().unwrap() {
            print!("CUDA ");
            cuda.register(&session_builder);
        }
        let xnnpack = XNNPACKExecutionProvider::default()
            .with_intra_op_num_threads(NonZeroUsize::new(4).unwrap());
        if xnnpack.is_available().unwrap() {
            print!("XNNPACK ");
            xnnpack.register(&session_builder);
        }
        println!();

        let mut magika = MagikaSession::from(
            session_builder,
            "../magika/python/magika/models/standard_v1",
        )
        .unwrap();
        println!("Running inference");
        let mut map = AssetMap::with_capacity_and_hasher(index.len(), NoHash);
        for (i, key) in index.ids().enumerate() {
            let bytes = index.load_data_bytes(key).unwrap();
            map.insert(key, magika.identify_topk::<3>(bytes.as_slice()).unwrap());
            if i % 1000 == 0 {
                println!("Processed {i}/{}", index.len());
            }
        }
        Self {
            labels: magika.labels().to_vec(),
            results: map,
        }
    }
}
