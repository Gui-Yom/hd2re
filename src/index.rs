use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{fs, io};

use binrw::io::BufReader;
use binrw::BinRead;
use speedy::{Readable, Writable};

use crate::hash::NoHash;
use crate::parse::{DataRecord, Hd2DataFile};

pub type AssetMap<V> = HashMap<u64, V, NoHash>;

#[derive(Readable, Writable)]
pub struct HD2Index {
    base_dir: String,
    items: AssetMap<Entry>,
}

#[derive(Readable, Writable)]
pub struct Entry {
    /// File where this asset is contained
    pub file_id: u64,
    pub record: DataRecord,
}

impl HD2Index {
    pub fn create_index(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let mut self_ = Self {
            base_dir: path.to_str().unwrap().to_owned(),
            items: HashMap::with_hasher(NoHash),
        };
        let start = Instant::now();
        let mut count = 0;
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() && entry.path().extension().is_none() {
                count += 1;
                println!("file {count}");
                let mut r = BufReader::new(File::open(entry.path()).unwrap());
                let file = Hd2DataFile::read(&mut r).unwrap();
                let file_id = u64::from_str_radix(entry.file_name().to_str().unwrap(), 16).unwrap();
                for record in file.data_headers {
                    self_.items.insert(record.id, Entry { file_id, record });
                }
            }
        }
        println!("Loaded {count} files in {} ms", start.elapsed().as_millis());
        self_
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn resolve_data_file(&self, id: u64) -> PathBuf {
        Path::new(&self.base_dir).join(format!("{id:016x}"))
    }

    pub fn resolve_stream_file(&self, id: u64) -> PathBuf {
        self.resolve_data_file(id).with_extension("stream")
    }

    pub fn resolve_gpu_file(&self, id: u64) -> PathBuf {
        self.resolve_data_file(id).with_extension("gpu_resources")
    }

    pub fn load_data_bytes(&self, id: u64) -> io::Result<Vec<u8>> {
        let Entry { file_id, record } = &self.items[&id];
        let mut file = File::open(self.resolve_data_file(*file_id))?;
        let mut buf = vec![0; record.data_size as usize];
        file.seek(SeekFrom::Start(record.offset))?;
        file.read(&mut buf)?;
        Ok(buf)
    }

    pub fn load_n_data_bytes<const N: usize>(&self, id: u64) -> io::Result<[u8; N]> {
        let Entry { file_id, record } = &self.items[&id];
        if record.stream_size < N as u32 {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }
        let mut file = File::open(self.resolve_data_file(*file_id))?;
        let mut buf = [0; N];
        file.seek(SeekFrom::Start(record.offset))?;
        file.read(&mut buf)?;
        Ok(buf)
    }

    pub fn load_stream_bytes(&self, id: u64) -> io::Result<Vec<u8>> {
        let Entry { file_id, record } = &self.items[&id];
        let mut file = File::open(self.resolve_stream_file(*file_id))?;
        let mut buf = vec![0; record.stream_size as usize];
        file.seek(SeekFrom::Start(record.stream_offset as u64))?;
        file.read(&mut buf)?;
        Ok(buf)
    }

    pub fn load_n_stream_bytes<const N: usize>(&self, id: u64) -> io::Result<[u8; N]> {
        let Entry { file_id, record } = &self.items[&id];
        if record.stream_size < N as u32 {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }
        let mut file = File::open(self.resolve_stream_file(*file_id))?;
        let mut buf = [0; N];
        file.seek(SeekFrom::Start(record.stream_offset as u64))?;
        file.read(&mut buf)?;
        Ok(buf)
    }

    pub fn load_gpu_bytes(&self, id: u64) -> io::Result<Vec<u8>> {
        let Entry { file_id, record } = &self.items[&id];
        let mut file = File::open(self.resolve_gpu_file(*file_id))?;
        let mut buf = vec![0; record.gpu_size as usize];
        file.seek(SeekFrom::Start(record.gpu_offset))?;
        file.read(&mut buf)?;
        Ok(buf)
    }

    pub fn load_n_gpu_bytes<const N: usize>(&self, id: u64) -> io::Result<[u8; N]> {
        let Entry { file_id, record } = &self.items[&id];
        if record.gpu_size < N as u32 {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }
        let mut file = File::open(self.resolve_gpu_file(*file_id))?;
        let mut buf = [0; N];
        file.seek(SeekFrom::Start(record.gpu_offset))?;
        file.read(&mut buf)?;
        Ok(buf)
    }

    pub fn ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.items.keys().copied()
    }
}

impl Index<u64> for HD2Index {
    type Output = Entry;

    fn index(&self, index: u64) -> &Self::Output {
        &self.items[&index]
    }
}
