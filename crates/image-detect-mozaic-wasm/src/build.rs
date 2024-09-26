use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let source_file_path = PathBuf::from("images/poi-s.png");
    let target_file_path = PathBuf::from(out_dir).join("poi-s.png");
    fs::copy(source_file_path, target_file_path).unwrap();
}
