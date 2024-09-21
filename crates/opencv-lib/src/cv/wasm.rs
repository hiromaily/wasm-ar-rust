use anyhow::Context;
use opencv::{
    core::{Mat, MatTraitConst, Mat_AUTO_STEP, Size, CV_8UC3, CV_8UC4},
    imgproc,
};
use wasm_bindgen::Clamped;

pub struct ClampedVec<T>(Clamped<Vec<T>>);

// convert `Clamped<Vec<u8>>, width, height` to Mat
// e.g.
//  let image_data = ClampedVec(image_data);
//  let web_image convert_to_mat(image_data, width, height)?;
pub fn convert_to_mat(image_data: ClampedVec<u8>, width: u32, height: u32) -> anyhow::Result<Mat> {
    let Clamped(data) = image_data.0;
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

// convert RGBA to Mat
pub fn convert_rgba_to_mat(rgba: &[u8], width: u32, height: u32) -> anyhow::Result<Mat> {
    let size = Size::new(width as i32, height as i32);

    // RGBA (4チャンネル) のバイトデータを Mat に変換
    let mat = Mat::from_slice(rgba).with_context(|| "Failed to create Mat from image data")?;
    let mat = unsafe {
        Mat::new_size_with_data_unsafe(
            size,
            CV_8UC4,
            mat.data() as *mut std::ffi::c_void,
            Mat_AUTO_STEP,
        )
    }?;

    // RGBA から RGB に変換 (Alpha チャンネルを無視する場合)
    let mut rgb_mat = Mat::default();
    imgproc::cvt_color(&mat, &mut rgb_mat, imgproc::COLOR_RGBA2RGB, 0)?;

    Ok(rgb_mat)
}
