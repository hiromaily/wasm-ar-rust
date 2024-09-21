use anyhow::Context;
use opencv::{
    core::{Mat, MatTraitConst, Vector},
    imgcodecs,
};

// load image from file path
pub fn load_image(path: &str) -> anyhow::Result<opencv::prelude::Mat> {
    let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)
        .with_context(|| format!("Failed to load image '{}'", path))?;

    if !img.empty() {
        Ok(img)
    } else {
        Err(anyhow::anyhow!("Image '{}' is empty", path))
    }
}

// load embeded template image
// e.g.
//  let template = loader::load_embedded_template(TEMPLATE_IMAGE)?;
// or
// let data = include_bytes!("../images/template.png");
// let template = loader::load_embedded_template(data)?;
pub fn load_embedded_template(data: impl AsRef<[u8]>) -> anyhow::Result<Mat> {
    let data = data.as_ref(); // convert to slice

    // treat bytes array as opencv Mat
    let buf = Vector::<u8>::from_slice(data);

    let template = imgcodecs::imdecode(&buf, imgcodecs::IMREAD_COLOR)
        .context("Failed to decode embedded template image")?;

    if !template.empty() {
        Ok(template)
    } else {
        Err(anyhow::anyhow!("Embedded template image is empty"))
    }
}
