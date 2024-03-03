use crate::hash::NoHash;
use crate::index::{AssetMap, HD2Index};
use magika::MagikaSession;
use ort::{
    CUDAExecutionProvider, DirectMLExecutionProvider, ExecutionProvider, GraphOptimizationLevel,
    Session, TensorRTExecutionProvider,
};
use speedy::{Readable, Writable};

#[derive(Debug, Readable, Writable)]
pub struct MagikaSniff {
    pub labels: Vec<String>,
    pub results: AssetMap<[[(f32, u32); 3]; 3]>,
}

impl MagikaSniff {
    pub fn run(index: &HD2Index) -> Self {
        ort::init_from(
            r#"C:\Users\Guillaume\Desktop\onnxruntime\build\Windows\Release\onnxruntime.dll"#,
        )
        .commit()
        .unwrap();
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
        let tensorrt = TensorRTExecutionProvider::default()
            .with_timing_cache(true)
            .with_engine_cache(true);
        if tensorrt.is_available().unwrap() {
            print!("TENSORRT ");
            tensorrt.register(&session_builder).unwrap();
        }
        let cuda = CUDAExecutionProvider::default();
        if cuda.is_available().unwrap() {
            print!("CUDA ");
            cuda.register(&session_builder).unwrap();
        }
        println!();

        let mut magika = MagikaSession::from(
            session_builder,
            "../magika/python/magika/models/standard_v1",
        )
        .unwrap();
        println!("Running inference");
        let mut results = AssetMap::with_capacity_and_hasher(index.len(), NoHash);
        for (i, key) in index.ids().enumerate() {
            let data = index
                .load_data_bytes(key)
                .map(|b| magika.identify_topk::<3>(b.as_slice()).unwrap())
                .unwrap_or([(0.0, 0); 3]);
            let stream = index
                .load_stream_bytes(key)
                .map(|b| magika.identify_topk::<3>(b.as_slice()).unwrap())
                .unwrap_or([(0.0, 0); 3]);
            let gpu = index
                .load_gpu_bytes(key)
                .map(|b| magika.identify_topk::<3>(b.as_slice()).unwrap())
                .unwrap_or([(0.0, 0); 3]);
            results.insert(key, [data, stream, gpu]);
            if i % 1000 == 0 {
                println!("Processed {i}/{}", index.len());
            }
        }
        Self {
            labels: magika.labels().to_vec(),
            results,
        }
    }
}
