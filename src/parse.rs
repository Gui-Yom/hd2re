use binrw::io::SeekFrom;
use binrw::BinRead;
use speedy::{Readable, Writable};
use strum::{Display, EnumIter};

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

/// Hash comes from hashing the enum name
#[allow(non_camel_case_types)]
#[derive(BinRead, Debug, Readable, Writable, Eq, PartialEq, EnumIter, Copy, Clone, Display)]
#[br(repr = u64)]
#[speedy(tag_type = u64)]
#[repr(u64)]
pub enum DataType {
    animation = 0x931E336D7646CC26,
    /// Bink video
    bik = 0xAA5965F03029FA18,
    bones = 0x18DEAD01056B72E9,
    camera_shake = 0xFCAAF813B4D3CC1E,
    config = 0x82645835E6B73232,
    entity = 0x9831CA893B0D087D,
    flow = 0x92D3EE038EEB610D,
    font = 0x9EFE0A916AAE7880,
    geleta = 0xB8FD4D2CEDE20ED7,
    geometry_group = 0xC4F0F4BE7FB0C8D6,
    hash_lookup = 0xE3F2851035957AF5,
    havok_ai_properties = 0x6592B918E67F082C,
    havok_physics_properties = 0xF7A09F8BB35A1D49,
    level = 0x2A690FD348FE9AC5,
    lua = 0xA14E8DFA2CD117E2,
    material = 0xEAC0B497876ADEDF,
    mouse_cursor = 0xB277B11FE4A61D37,
    network_config = 0x3B1FA9E8F6BAC374,
    package = 0xAD9C6D9ED1E5E77A,
    particles = 0xA8193123526FAD64,
    physics = 0x5F7203C8F280DAB8,
    prefab = 0xAB2F78E885F513C6,
    renderable = 0x7910103158FC1DE9,
    render_config = 0x27862FE24795319C,
    runtime_font = 0x5106B81DCD58A13,
    shader_library = 0xE5EE32A477239A93,
    shader_library_group = 0x9E5C3CC74575AEB5,
    shading_environment = 0xFE73C7DCFF8A7CA5,
    shading_environment_mapping = 0x250E0A11AC8E26F8,
    speedtree = 0xE985C5F61C169997,
    state_machine = 0xA486D4045106165C,
    strings = 0x0D972BAB10B40FD3,
    texture = 0xCD4238C6A0C69E32,
    texture_atlas = 0x9199BB50B6896F02,
    unit = 0xE0A48D0BE9A7453F,
    vector_field = 0xF7505933166D6755,
    wwise_bank = 0x535A7BD3E650D799,
    wwise_dep = 0xAF32095C82F2B070,
    wwise_metadata = 0xD50A8B7E1C82B110,
    /// WWise WEM audio (modified RIFF/WAV file)
    wwise_stream = 0x504B55235D21440E,
    Unknown1 = 0x1D59BD6687DB6B33,
    Unknown2 = 0x57A13425279979D7,
    Unknown3 = 0xD7014A50477953E0,
    Unknown4 = 0x2A0A70ACFE476E1D,
    Unknown5 = 0x5FDD5FE391076F9F,
}

impl DataType {
    pub fn is_known(&self) -> bool {
        !matches!(
            self,
            DataType::Unknown1
                | DataType::Unknown2
                | DataType::Unknown3
                | DataType::Unknown4
                | DataType::Unknown5
        )
    }
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

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::hash::stingray_hash;
    use crate::parse::DataType;

    #[test]
    fn data_type_names_hash() {
        for ty in DataType::iter() {
            if ty.is_known() {
                assert_eq!(stingray_hash(ty.to_string().as_bytes()), ty as u64);
            }
        }
    }
}
