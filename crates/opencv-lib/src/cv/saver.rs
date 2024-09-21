use opencv::{
    core::{Mat, Vector},
    imgcodecs,
};

pub fn save_image(save_path: &str, img: &Mat) -> anyhow::Result<()> {
    imgcodecs::imwrite(save_path, img, &Vector::new())?;
    Ok(())
}
