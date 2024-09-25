use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    let stl_file_path = Path::new("assets/poi.stl");
    let stl_data = std::fs::read(stl_file_path)?;

    let dest_path = Path::new(&out_dir).join("poi.rs");
    let mut f = File::create(&dest_path)?;

    write!(f, "const POI_MESH_STL: &[u8] = &{:?};", stl_data)?;

    Ok(())
}
