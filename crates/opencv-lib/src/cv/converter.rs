use anyhow::Context;
use opencv::core::{Mat, MatTraitConst, Mat_AUTO_STEP, Size, CV_8UC3};

#[allow(dead_code)]
pub struct Clamped<T>(T);

// convert `Clamped<Vec<u8>>, width, height` to Mat
pub fn convert_to_mat(
    image_data: Clamped<Vec<u8>>,
    width: u32,
    height: u32,
) -> anyhow::Result<Mat> {
    let Clamped(data) = image_data;
    let size = Size::new(width as i32, height as i32);

    // from RGB to Mat
    let mat = Mat::from_slice(&data).with_context(|| "Failed to create Mat from image data")?;
    let mat = unsafe {
        Mat::new_size_with_data_unsafe(
            size,
            CV_8UC3,
            mat.data() as *mut std::ffi::c_void,
            Mat_AUTO_STEP,
        )
    }?;

    Ok(mat)
}
