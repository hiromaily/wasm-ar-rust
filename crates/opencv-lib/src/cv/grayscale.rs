use anyhow::Context;
use opencv::{core::Mat, imgproc};

// transform images to grayscale
pub fn convert_to_grayscale(image: &Mat) -> anyhow::Result<Mat> {
    let mut gray_image = Mat::default();
    imgproc::cvt_color(image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0)
        .context("Failed to convert image to grayscale")?;
    Ok(gray_image)
}
