use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::{fs, process};

pub fn to_wav(f: &Path) {
    let output = f.with_extension("wav");
    let output = output.file_name().unwrap();
    let output_path = PathBuf::new().join("./data/audio/wav/").join(output);
    let status = process::Command::new("C:/apps/vgmstream/vgmstream-cli")
        .arg("-o")
        .arg(output_path)
        .arg(f)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    dbg!(status);
}

pub fn convert_all_to_wav() {
    fs::create_dir_all("./data/audio/wav/").unwrap();
    for entry in fs::read_dir("data/audio/wem").unwrap() {
        let entry = entry.unwrap();
        to_wav(&entry.path());
    }
}
