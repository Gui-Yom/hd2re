use binrw::io::SeekFrom;
use binrw::BinRead;
use speedy::{Readable, Writable};

#[derive(BinRead, Debug)]
#[br(little, magic = b"\x11\x00\x00\xF0")]
pub struct Hd2DataFile {
    pub type_count: u32,
    #[br(pad_after = 0x44)]
    pub data_count: u32,
    #[br(count = type_count)]
    pub type_headers: Vec<TypeHeader>,
    #[br(count = data_count, seek_before = SeekFrom::Current(- 8))]
    pub data_headers: Vec<DataRecord>,
}

#[derive(BinRead, Debug)]
pub struct TypeHeader {
    pub id: DataType,
    /// Number of data records of this type
    pub data_count: u64,
    pub size: u32,
    #[br(pad_after = 8)]
    pub data_header_size: u32,
}

#[derive(BinRead, Debug, Readable, Writable)]
#[br(repr = u64)]
#[speedy(tag_type = u64)]
#[repr(u64)]
pub enum DataType {
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
pub struct DataRecord {
    pub id: u64,
    pub type_id: DataType,
    /// Data offset in this file
    pub offset: u64,
    /// Data offset in the stream file
    #[br(pad_after = 4)]
    pub stream_offset: u32,
    /// Data offset in the gpu file
    #[br(pad_after = 16)]
    pub gpu_offset: u64,
    /// Data size in this file
    pub data_size: u32,
    /// Data size in the stream file
    pub stream_size: u32,
    /// Data size in the gpu file
    #[br(pad_after = 8)]
    pub gpu_size: u32,
    pub index: u32,
    //#[br(seek_before = SeekFrom::Start(offset), count = data_size, restore_position)]
    //data: Vec<u8>,
}
