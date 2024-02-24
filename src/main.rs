use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;
use std::time::Instant;
use std::{fs, io};

use binrw::io::{BufReader, SeekFrom};
use binrw::BinRead;
use speedy::{Readable, Writable};

use crate::hash::NoHash;
use crate::sniff::sniff_magika;

mod hash;
mod sniff;

fn main() {
    // println!("{:x}", stringray_hash(b"packages/pre_boot"));
    // println!("{:x}", stringray_hash(b"packages/boot"));
    // println!("{:x}", stringray_hash(b"texture"));
    let index = if let Ok(hd2fs) = HD2Index::read_from_file("hd2index.bin") {
        hd2fs
    } else {
        let hd2fs = HD2Index::create_index(r#"E:\SteamLibrary\steamapps\common\Helldivers 2\data"#);
        hd2fs.write_to_file("hd2index.bin").unwrap();
        hd2fs
    };
    sniff_magika(&index);
}

#[derive(Readable, Writable)]
struct HD2Index {
    base_dir: String,
    items: HashMap<u64, Entry, NoHash>,
}

impl HD2Index {
    fn create_index(path: impl AsRef<Path>) -> Self {
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

    fn resolve_data_file(&self, id: u64) -> PathBuf {
        let mut file = PathBuf::from_str(&self.base_dir).unwrap();
        file.push(format!("{id:016x}"));
        file
    }

    fn resolve_stream_file(&self, id: u64) -> PathBuf {
        self.resolve_data_file(id).with_extension("stream")
    }

    fn load_data_bytes(&self, id: u64) -> io::Result<Vec<u8>> {
        let Entry { file_id, record } = &self.items[&id];
        let mut file = File::open(self.resolve_data_file(*file_id))?;
        let mut buf = vec![0; record.data_size as usize];
        file.seek(SeekFrom::Start(record.offset))?;
        file.read(&mut buf)?;
        Ok(buf)
    }

    fn load_n_data_bytes<const N: usize>(&self, id: u64) -> io::Result<[u8; N]> {
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

    fn load_n_stream_bytes<const N: usize>(&self, id: u64) -> io::Result<[u8; N]> {
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
}

#[derive(Readable, Writable)]
struct Entry {
    file_id: u64,
    record: DataRecord,
}

#[derive(BinRead, Debug)]
#[br(little, magic = b"\x11\x00\x00\xF0")]
struct Hd2DataFile {
    type_count: u32,
    #[br(pad_after = 0x44)]
    data_count: u32,
    #[br(count = type_count)]
    type_headers: Vec<TypeHeader>,
    #[br(count = data_count, seek_before = SeekFrom::Current(- 8))]
    data_headers: Vec<DataRecord>,
}

#[derive(BinRead, Debug)]
struct TypeHeader {
    id: DataType,
    /// Number of data records of this type
    data_count: u64,
    size: u32,
    #[br(pad_after = 8)]
    data_header_size: u32,
}

#[derive(BinRead, Debug, Readable, Writable)]
#[br(repr = u64)]
#[speedy(tag_type = u64)]
#[repr(u64)]
enum DataType {
    Unknown = 0xA8193123526FAD64,
    Skeleton = 0x18DEAD01056B72E9,
    Unknown3 = 0x931E336D7646CC26,
    Unknown4 = 0xA486D4045106165C,
    Unknown5 = 0xAB2F78E885F513C6,
    Unknown6 = 0x535A7BD3E650D799,
    Unknown7 = 0xAF32095C82F2B070,
    /// Wav audio
    Wav = 0x504B55235D21440E,
    Unknown9 = 0xE985C5F61C169997,
    Unknown10 = 0x250E0A11AC8E26F8,
    Unknown11 = 0x9831CA893B0D087D,
    Unknown12 = 0x1D59BD6687DB6B33,
    Unknown13 = 0x57A13425279979D7,
    Unknown14 = 0xAA5965F03029FA18,
    Unknown15 = 0xC4F0F4BE7FB0C8D6,
    Unknown16 = 0xD7014A50477953E0,
    Unknown17 = 0xFE73C7DCFF8A7CA5,
    Unknown18 = 0xA14E8DFA2CD117E2,
    Unknown19 = 0xF7505933166D6755,
    Unknown20 = 0x92D3EE038EEB610D,
    Unknown21 = 0x9199BB50B6896F02,
    Unknown22 = 0x27862FE24795319C,
    Unknown23 = 0x5106B81DCD58A13,
    Unknown24 = 0x9EFE0A916AAE7880,
    Unknown25 = 0x2A0A70ACFE476E1D,
    Unknown26 = 0x3B1FA9E8F6BAC374,
    Unknown27 = 0x5FDD5FE391076F9F,
    Unknown28 = 0x82645835E6B73232,
    Unknown29 = 0xAD9C6D9ED1E5E77A,
    Unknown30 = 0xB8FD4D2CEDE20ED7,
    Unknown31 = 0xD50A8B7E1C82B110,
    Unknown32 = 0xE3F2851035957AF5,
    Unknown33 = 0x6592B918E67F082C,
    Unknown34 = 0xF7A09F8BB35A1D49,
    Unknown35 = 0xFCAAF813B4D3CC1E,
    Unknown36 = 0xB277B11FE4A61D37,
    Unknown37 = 0x7910103158FC1DE9,
    Unknown38 = 0x9E5C3CC74575AEB5,
    Unknown39 = 0xE5EE32A477239A93,
    /// stringray_hash(b"texture")
    Texture = 0xCD4238C6A0C69E32,
    Model = 0xE0A48D0BE9A7453F,
    Havok = 0x5F7203C8F280DAB8,
    Material = 0xEAC0B497876ADEDF,
    Map = 0x2A690FD348FE9AC5,
    Strings = 0x0D972BAB10B40FD3,
}

#[derive(BinRead, Debug, Readable, Writable)]
struct DataRecord {
    id: u64,
    type_id: DataType,
    /// Data offset in this file
    offset: u64,
    /// Data offset in the stream file
    #[br(pad_after = 4)]
    stream_offset: u32,
    /// Data offset in the gpu file
    #[br(pad_after = 16)]
    gpu_offset: u64,
    /// Data size in this file
    data_size: u32,
    /// Data size in the stream file
    stream_size: u32,
    /// Data size in the gpu file
    #[br(pad_after = 8)]
    gpu_size: u32,
    index: u32,
    //#[br(seek_before = SeekFrom::Start(offset), count = data_size, restore_position)]
    //data: Vec<u8>,
}
